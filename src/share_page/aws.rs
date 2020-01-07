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
use std::env;

use std::error::Error;

use std::str::FromStr;

const LOCAL_TEST: &str = "LOCAL_TEST";
const NO_KMS: &str = "NO_KMS_KEY";

pub fn upload_file(filename: &str, 
    bucket: &str, 
    prefix: &str, 
    identity_pool_id: &str, 
    region: &str, 
    service_endpoint: Option<&str>, 
    uuid: &str, 
    signature: &str,
    kms_key: &str,
    share_with_myscript: Option<&str>,
    collect_login: Option<&str>) -> Result<(), Box<dyn Error>>{
    print!("uploading file to S3... ");
    let local_region;
    
    if identity_pool_id == LOCAL_TEST {
        local_region = Region::Custom {
            name: region.into(),
            endpoint: service_endpoint.ok_or("missing service endpoint for local test")?.into(),
        };
    }
    else {
        local_region = Region::from_str(region)?;
    }
    let client = S3Client::new(local_region);
    let mut request = PutObjectRequest::default();
    request.bucket = bucket.into();
    request.content_disposition = Some("attachment; filename=page.nebo; filename*=UTF-8''page.nebo".into());
    request.content_type = Some("application/vnd.myscript.nebo".into());
    request.key = format!("{}{}_{}.nebo", prefix, uuid, signature);
    if kms_key != NO_KMS {
        request.server_side_encryption = Some("aws:kms".into());
        request.ssekms_key_id = Some(kms_key.into());
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

pub fn get_cognito_credentials(token: &str, identity_id: &str,identity_pool_id: &str,  provider: &str, region: &str)-> Result<(), Box<dyn Error>> {
    // It turns out that, event if the get_credentials_for_identity doesn't need credentials,
    // It doesn't work if there is really no credentials given. So let's give some credentials
    env::set_var("AWS_SECRET_ACCESS_KEY", "dummy2");
    env::set_var("AWS_ACCESS_KEY_ID", "dummy2");
    if identity_pool_id != LOCAL_TEST {
        let client = CognitoIdentityClient::new(Region::from_str(region)?);
        let mut input = GetCredentialsForIdentityInput::default();
        input.identity_id = identity_id.into();
        let mut logins = HashMap::new();
        logins.insert(provider.into(), token.into());
        input.logins = Some(logins);
        let response = client.get_credentials_for_identity(input).sync()?;

        // Let's set the new env vars to the credentials given by the cognito identity
        let credentials = response.credentials.ok_or("No credentials given by cognito identity")?;
        env::set_var("AWS_ACCESS_KEY_ID", credentials.access_key_id.ok_or("No access key id given by cognito identity")?);
        env::set_var("AWS_SECRET_ACCESS_KEY", credentials.secret_key.ok_or("No secret key given by cognito identity")?);
        env::set_var("AWS_SESSION_TOKEN", credentials.session_token.ok_or("No token given by cognito identity")?);
    }

    Ok(())
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