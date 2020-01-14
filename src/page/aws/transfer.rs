use rusoto_s3::S3;
use rusoto_s3::PutObjectRequest;
use rusoto_s3::CreateMultipartUploadRequest;
use rusoto_s3::CompleteMultipartUploadRequest;
use rusoto_s3::CompletedMultipartUpload;
use rusoto_s3::UploadPartRequest;
use rusoto_s3::UploadPartError;
use rusoto_core::RusotoError;
use rusoto_s3::UploadPartOutput;
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

use threadpool::ThreadPool;
use std::sync::mpsc::channel;

const NB_THREADS_MULTIPART: usize = 3;
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
    upload_id: Option<String>
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
            upload_id: None
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
        let create_request = CreateMultipartUploadRequest {
            bucket: self.bucket.clone(),
            content_disposition: self.content_disposition.clone(),
            content_type: self.content_type.clone(),
            key: self.key.clone(),
            metadata: self.metadata.clone(),
            server_side_encryption: self.server_side_encryption.clone(),
            ssekms_key_id: self.ssekms_key_id.clone(),
            ..Default::default()
        };
        let creation_result = self.client.create_multipart_upload(create_request).sync()?;
        self.upload_id = creation_result.upload_id;
        debug!("end init multipart upload for {}", self.key);
        let pool = ThreadPool::with_name("multipart".into(), NB_THREADS_MULTIPART);
        let (tx, rx) = channel::<Result<(UploadPartOutput, i64), RusotoError<UploadPartError>>>();
        let mut nb_parts = 0;
        for (i, chunk) in FileChunkIterator::new(&self.file)?.enumerate() {
            let (tx, part_number, chunk_size, upload_id, client, data, bucket, key) = (
                tx.clone(),
                i as i64 + 1,
                chunk.size,
                self.upload_id.clone().ok_or("missing upload id")?,
                self.client.clone(),
                ByteStream::from(chunk.chunk),
                self.bucket.clone(),
                self.key.clone()
            );
            pool.execute(move || {
                let log = format!("send part {} of size {} for {}", part_number, chunk_size, key);
                debug!("{}", log);
                let part = client.upload_part(UploadPartRequest {
                    body: Some(data),
                    bucket: bucket,
                    content_length: Some(chunk_size as i64),
                    key: key,
                    part_number: part_number,
                    upload_id: upload_id,
                    ..Default::default()
                })
                .sync();
                debug!("end {}", log);
                if part.is_err() {
                    tx.send(Err(part.err().unwrap())).unwrap();
                }
                else {
                    tx.send(Ok((part.ok().unwrap(), part_number))).unwrap();
                }
            });
            nb_parts = part_number;
        }
        pool.join();
        if pool.panic_count() > 0 {
            return Err(Box::from("a multipart thread panicked"));
        }
        let results: Vec<Result<(UploadPartOutput, i64), RusotoError<UploadPartError>>> = rx.iter().take(nb_parts as usize).collect();
        let mut parts = Vec::new();
        for result in results {
            match result {
                Ok(value) => parts.push(CompletedPart{e_tag: value.0.e_tag.clone(), part_number: Some(value.1)}),
                Err(e) => return Err(Box::new(e))
            }
        }

        debug!("complete multipart upload for {}", self.key);
        parts.sort_by(|a,b| a.part_number.unwrap().partial_cmp(&b.part_number.unwrap()).unwrap());
        let completed_parts = CompletedMultipartUpload{ parts: Some(parts)};
        let request = CompleteMultipartUploadRequest{
            bucket: self.bucket.clone(),
            key: self.key.clone(),
            multipart_upload: Some(completed_parts),
            upload_id: self.upload_id.clone().unwrap(),
            ..Default::default()
        };
        self.client.complete_multipart_upload(request).sync()?;
        self.transfert_done = true;
        debug!("end complete multipart upload for {}", self.key);
        Ok(())
    }

    fn transfer_monopart(& mut self) -> Result<(), Box<dyn Error>> {
        debug!("monopart upload for {}", self.key);
        let mut request = PutObjectRequest {
            bucket: self.bucket.clone(),
            content_disposition: self.content_disposition.clone(),
            content_type: self.content_type.clone(),
            key: self.key.clone(),
            server_side_encryption: self.server_side_encryption.clone(),
            ssekms_key_id: self.ssekms_key_id.clone(),
            metadata: self.metadata.clone(),
            ..Default::default()
        };
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
                    ..Default::default()
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
    chunk: Vec<u8>
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
        Some(FileChunk{size: size_read, chunk: buffer})
    }
}