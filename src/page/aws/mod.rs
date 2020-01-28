use rusoto_s3::S3Client;

use rusoto_core::Region;
use rusoto_cognito_identity::CognitoProvider;

use crate::configuration::Configuration;

use std::error::Error;

use std::str::FromStr;

use log::debug;

mod transfer;

const LOCAL_TEST: &str = "LOCAL_TEST";

pub fn upload_file(filename: &str, 
    uuid: &str, 
    signature: &str,
    configuration: &Configuration,
    credential_provider: CognitoProvider,
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