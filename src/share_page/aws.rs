use rusoto_s3::S3Client;
use rusoto_s3::S3;
use rusoto_s3::PutObjectRequest;
use std::fs::File;
use std::io::Read;

use rusoto_cognito_identity::GetCredentialsForIdentityInput;
use rusoto_cognito_identity::CognitoIdentityClient;
use rusoto_cognito_identity::CognitoIdentity;
use rusoto_core::Region;
use std::collections::HashMap;
use rusoto_core::credential::StaticProvider;
use rusoto_core::credential::AwsCredentials;

use crate::configuration::Credentials;
use crate::configuration::S3Configuration;

use std::error::Error;

use std::str::FromStr;

const LOCAL_TEST: &str = "LOCAL_TEST";
const NO_KMS: &str = "NO_KMS_KEY";

pub fn upload_file(filename: &str, 
    uuid: &str, 
    signature: &str,
    user_credentials: Credentials,
    s3_config: S3Configuration,
    share_with_myscript: Option<&str>,
    collect_login: Option<&str>) -> Result<(), Box<dyn Error>>{
    print!("uploading file to S3... ");
    let local_region;
    
    if user_credentials.identity_pool_id == LOCAL_TEST {
        local_region = Region::Custom {
            name: s3_config.region,
            endpoint: s3_config.service_endpoint.ok_or("missing service endpoint for local test")?.into(),
        };
    }
    else {
        local_region = Region::from_str(&s3_config.region)?;
    }
    let client = S3Client::new_with(
        rusoto_core::request::HttpClient::new()?,
        StaticProvider::from(
            get_cognito_credentials(
                &user_credentials.access_token, 
                &user_credentials.identity_id,
                &user_credentials.identity_pool_id,
                &user_credentials.identity_provider,
                &user_credentials.region
            )?
        ),
        local_region
    );
    let mut request = PutObjectRequest::default();
    request.bucket = s3_config.bucket;
    request.content_disposition = Some("attachment; filename=page.nebo; filename*=UTF-8''page.nebo".into());
    request.content_type = Some("application/vnd.myscript.nebo".into());
    request.key = format!("{}{}_{}.nebo", s3_config.client_directory_prefix, uuid, signature);
    if s3_config.kms_key != NO_KMS {
        request.server_side_encryption = Some("aws:kms".into());
        request.ssekms_key_id = Some(s3_config.kms_key);
    }
    request.metadata = Some(generate_metadata(collect_login, signature, share_with_myscript));
    let mut content = Vec::new();
    let mut source = File::open(filename)?;
    source.read_to_end(&mut content)?;
    request.body = Some(content.into());
    // https://github.com/localstack/localstack/issues/1647
    let result = client.put_object(request).sync();
    if let Err(error) = result {
        if !error.to_string().contains("Expected EndElement PutObjectResponse") {
            return Err(Box::new(error));
        }
    }

    println!("ok");
    Ok(())
}

fn get_cognito_credentials(token: &str, identity_id: &str,identity_pool_id: &str,  provider: &str, region: &str)-> Result<AwsCredentials, Box<dyn Error>> {
    if identity_pool_id != LOCAL_TEST {
        // It turns out that, event if the get_credentials_for_identity doesn't need credentials,
        // It doesn't work if there is really no credentials given. So let's give some "dummy" credentials
        let client = CognitoIdentityClient::new_with(
            rusoto_core::request::HttpClient::new()?,
            StaticProvider::from(AwsCredentials::default()),
            Region::from_str(region)?
        );
        let mut input = GetCredentialsForIdentityInput::default();
        input.identity_id = identity_id.into();
        let mut logins = HashMap::new();
        logins.insert(provider.into(), token.into());
        input.logins = Some(logins);
        let response = client.get_credentials_for_identity(input).sync()?;

        let credentials = response.credentials.ok_or("No credentials given by cognito identity")?;
        return Ok(AwsCredentials::new(
            credentials.access_key_id.ok_or("No access key id returned by cognito")?, 
            credentials.secret_key.ok_or("No secret key returned by cognito")?, 
            credentials.session_token, 
            None));
    }

    Ok(AwsCredentials::default())
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