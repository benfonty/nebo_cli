use rusoto_s3::S3Client;

use rusoto_cognito_identity::GetCredentialsForIdentityInput;
use rusoto_cognito_identity::CognitoIdentityClient;
use rusoto_cognito_identity::CognitoIdentity;
use rusoto_core::Region;
use rusoto_core::credential::StaticProvider;
use rusoto_core::credential::AwsCredentials;

use crate::configuration::Configuration;

use std::error::Error;

use std::str::FromStr;

use std::collections::HashMap;

use log::debug;

mod transfer;

const LOCAL_TEST: &str = "LOCAL_TEST";

pub fn upload_file(filename: &str, 
    uuid: &str, 
    signature: &str,
    configuration: &Configuration,
    credential_provider: StaticProvider,
    share_with_myscript: Option<&str>,
    collect_login: Option<&str>) -> Result<(), Box<dyn Error>>{
    debug!("Begin Uploading file to S3");
    let local_region;
    
    if configuration.credentials.identity_pool_id == LOCAL_TEST {
        debug!("Local test, do not use cognito");
        local_region = Region::Custom {
            name: configuration.s3.region.clone(),
            endpoint: configuration.s3.service_endpoint.clone().ok_or("missing service endpoint for local test")?.into(),
        };
    }
    else {
        local_region = Region::from_str(&configuration.s3.region)?;
    }
    let client = S3Client::new_with(
        rusoto_core::request::HttpClient::new()?,
        credential_provider,
        local_region
    );

    let mut manager = transfer::TransferManager::init(client, filename, uuid, signature, &configuration.s3, share_with_myscript, collect_login)?;
    manager.transfer()?;
    
    debug!("End Uploading file to S3 OK");
    Ok(())
}

pub fn get_cognito_credentials(token: &str, identity_id: &str,identity_pool_id: &str,  provider: &str, region: &str)-> Result<AwsCredentials, Box<dyn Error>> {
    if identity_pool_id != LOCAL_TEST {
        debug!("Begin calling cognito for credentials");
        let client = CognitoIdentityClient::new(Region::from_str(region)?);
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