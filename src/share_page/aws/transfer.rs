use rusoto_s3::S3;
use rusoto_s3::PutObjectRequest;
use rusoto_s3::CreateMultipartUploadRequest;
use rusoto_s3::CompleteMultipartUploadRequest;
use rusoto_s3::CompletedMultipartUpload;
use rusoto_s3::UploadPartRequest;
use rusoto_s3::CompletedPart;
use rusoto_core::ByteStream;
use rusoto_s3::AbortMultipartUploadRequest;
use std::fs::File;
use std::io::Read;

use std::collections::HashMap;

use crate::configuration::S3Configuration;

use log::{debug, error};

use rusoto_s3::S3Client;

use std::error::Error;

const READ_BUFFER_SIZE: usize = 5 * 1024 * 1024;
const NO_KMS: &str = "NO_KMS_KEY";

pub struct TransferManager {
    client: S3Client,
    file: File,
    key: String,
    bucket: String,
    content_disposition: Option<String>,
    content_type: Option<String>,
    server_side_encryption: Option<String>,
    ssekms_key_id: Option<String>,
    metadata: Option<HashMap<String, String>>,
    transfert_done: bool,
    upload_id: Option<String>,
    parts: Vec<(i64, String)>
}

impl TransferManager {
    pub fn init(client: S3Client, path: &str, uuid: &str, signature: &str, s3_config: &S3Configuration, share_with_myscript: Option<&str>, collect_login: Option<&str>) -> Result<Self , Box<dyn Error>> {
        debug!("Init Transfert Manager for {}", uuid);
        let mut transfert_manager = TransferManager {
            client: client,
            file: File::open(path)?,
            key: format!("{}{}_{}.nebo", s3_config.client_directory_prefix, uuid, signature),
            bucket: s3_config.bucket.clone(),
            content_disposition: Some("attachment; filename=page.nebo; filename*=UTF-8''page.nebo".into()),
            content_type: Some("application/vnd.myscript.nebo".into()),
            server_side_encryption: None,
            ssekms_key_id: None,
            metadata: Some(generate_metadata(collect_login, signature, share_with_myscript)),
            transfert_done: false,
            upload_id: None,
            parts: vec!()
        };
        debug!("S3 file key is {}", transfert_manager.key);
        if s3_config.kms_key != NO_KMS {
            debug!("KMS activated");
            transfert_manager.server_side_encryption = Some("aws:kms".into());
            transfert_manager.ssekms_key_id = Some(s3_config.kms_key.clone());
        }
        Ok(transfert_manager)
    }

    fn transfer_multipart(& mut self) -> Result<(), Box<dyn Error>> {
        debug!("init multipart upload for {}", self.key);
        let mut create_request = CreateMultipartUploadRequest::default();
        create_request.bucket = self.bucket.clone();
        create_request.content_disposition = self.content_disposition.clone();
        create_request.content_type = self.content_type.clone();
        create_request.key = self.key.clone();
        create_request.metadata = self.metadata.clone();
        create_request.server_side_encryption = self.server_side_encryption.clone();
        create_request.ssekms_key_id = self.ssekms_key_id.clone();
        let creation_result = self.client.create_multipart_upload(create_request).sync()?;
        self.upload_id = creation_result.upload_id;
        debug!("end init multipart upload for {}", self.key);
        
        for (i, chunk) in FileChunkIterator::new(&self.file)?.enumerate() {
            let part_number = i as i64 + 1;
            debug!("send part {} of size {} for {}", part_number, chunk.size, self.key);
            let part = self.client.upload_part(UploadPartRequest {
                body: Some(chunk.chunk),
                bucket: self.bucket.clone(),
                content_length: Some(chunk.size as i64),
                content_md5: None,
                key: self.key.clone(),
                part_number: part_number,
                request_payer: None,
                sse_customer_algorithm: None,
                sse_customer_key: None,
                sse_customer_key_md5: None,
                upload_id: self.upload_id.clone().ok_or("missing upload id")?
            })
            .sync()?;
            
            self.parts.push((part_number, part.e_tag.ok_or("missing etag")?));
            debug!("end send part {} of size {} for {}", i, chunk.size, self.key);
        }

        debug!("complete multipart upload for {}", self.key);
        let completed_parts = CompletedMultipartUpload{ parts: Some(self.parts.iter().map(|x| CompletedPart{e_tag: Some(x.1.clone()), part_number: Some(x.0)}).collect())};
        let request = CompleteMultipartUploadRequest{
            bucket: self.bucket.clone(),
            key: self.key.clone(),
            multipart_upload: Some(completed_parts),
            request_payer: None,
            upload_id: self.upload_id.clone().unwrap()
        };
        self.client.complete_multipart_upload(request).sync()?;
        self.transfert_done = true;
        debug!("end complete multipart upload for {}", self.key);
        Ok(())
    }

    fn transfer_monopart(& mut self) -> Result<(), Box<dyn Error>> {
        debug!("monopart upload for {}", self.key);
        let mut request = PutObjectRequest::default();
        request.bucket = self.bucket.clone();
        request.content_disposition = self.content_disposition.clone();
        request.content_type = self.content_type.clone();
        request.key = self.key.clone();
        request.server_side_encryption = self.server_side_encryption.clone();
        request.ssekms_key_id = self.ssekms_key_id.clone();
        request.metadata = self.metadata.clone();
        let mut content = Vec::new();
        self.file.read_to_end(&mut content)?;
        request.body = Some(content.into());
        // https://github.com/localstack/localstack/issues/1647
        let result = self.client.put_object(request).sync();
        if let Err(error) = result {
            if !error.to_string().contains("Expected EndElement PutObjectResponse") {
                return Err(Box::new(error));
            }
        }
        debug!("end monopart upload for {}", self.key);
        Ok(())
    }

    pub fn transfer(& mut self) -> Result<(), Box<dyn Error>> {
        if self.file.metadata()?.len() > READ_BUFFER_SIZE as u64 {
            return self.transfer_multipart();
        }
        else {
            return self.transfer_monopart();
        }
    }
}

fn generate_metadata(login: Option<&str>, signature: &str, share_with_myscript: Option<&str>) -> HashMap<String, String> {
    let mut metadata = HashMap::new();
    metadata.insert("x-amz-meta-content-sha256".into(), signature.into());
    if let Some(s) = share_with_myscript {
        metadata.insert("x-amz-meta-share-with-myscript".into(), s.into());
    }
    if let Some(l) = login {
        metadata.insert("x-amz-meta-login".into(), l.into());
    }
    metadata
}

impl Drop for TransferManager {
    fn drop(&mut self) {
        if let Some(id) = &self.upload_id {
            if !self.transfert_done {
                debug!("abort multipart upload for {}", self.key);
                let request = AbortMultipartUploadRequest {
                    bucket: self.bucket.clone(),
                    key: self.key.clone(),
                    upload_id: id.into(),
                    request_payer: None
                };
                if let Err(e) = self.client.abort_multipart_upload(request).sync() {
                    error!("unable to abort multipart upload for {}: {}", self.key, e);
                }
                else {
                    debug!("end abort multipart upload for {}", self.key);
                }
            }

        }
    }
}

struct FileChunkIterator<'a> {
    file: &'a File
}

struct FileChunk {
    size: usize,
    chunk: ByteStream
}

impl<'a> FileChunkIterator<'a> {
    pub fn new(file: &'a File) -> Result<Self, Box<dyn Error>> {
        Ok(FileChunkIterator {
            file: file
        })
    }
}

impl<'a> Iterator for FileChunkIterator<'a> {
    type Item = FileChunk;
    fn next(&mut self) -> Option<FileChunk>{
        let mut buffer = vec![0; READ_BUFFER_SIZE];
        let size_read = self.file.read(&mut buffer).expect("Cannot read file");
        if size_read == 0 {
            return None;
        }
        buffer.truncate(size_read);
        Some(FileChunk{size: size_read, chunk: buffer.into()})
    }
}