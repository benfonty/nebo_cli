use rusoto_s3::S3Client;

use rusoto_cognito_identity::GetCredentialsForIdentityInput;
use rusoto_cognito_identity::CognitoIdentityClient;
use rusoto_cognito_identity::CognitoIdentity;
use rusoto_core::Region;
use rusoto_core::credential::StaticProvider;
use rusoto_core::credential::AwsCredentials;

use crate::configuration::Credentials;
use crate::configuration::S3Configuration;

use std::error::Error;

use std::str::FromStr;

use std::collections::HashMap;

use log::debug;

mod transfer;

const LOCAL_TEST: &str = "LOCAL_TEST";

pub fn upload_file(filename: &str, 
    uuid: &str, 
    signature: &str,
    user_credentials: Credentials,
    s3_config: &S3Configuration,
    share_with_myscript: Option<&str>,
    collect_login: Option<&str>) -> Result<(), Box<dyn Error>>{
    debug!("Begin Uploading file to S3");
    let local_region;
    
    if user_credentials.identity_pool_id == LOCAL_TEST {
        debug!("Local test, do not use cognito");
        local_region = Region::Custom {
            name: s3_config.region.clone(),
            endpoint: s3_config.service_endpoint.clone().ok_or("missing service endpoint for local test")?.into(),
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

    let mut manager = transfer::TransferManager::init(client, filename, uuid, signature, &s3_config, share_with_myscript, collect_login)?;
    manager.transfer()?;
    
    debug!("End Uploading file to S3 OK");
    Ok(())
}

fn get_cognito_credentials(token: &str, identity_id: &str,identity_pool_id: &str,  provider: &str, region: &str)-> Result<AwsCredentials, Box<dyn Error>> {
    if identity_pool_id != LOCAL_TEST {
        debug!("Begin calling cognito for credentials");
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
        debug!("End calling cognito for credentials ok");
        return Ok(AwsCredentials::new(
            credentials.access_key_id.ok_or("No access key id returned by cognito")?, 
            credentials.secret_key.ok_or("No secret key returned by cognito")?, 
            credentials.session_token, 
            None));
    }

    Ok(AwsCredentials::default())
}