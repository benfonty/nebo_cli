use std::collections::HashMap;

use reqwest::blocking::Client;
use reqwest::blocking::ClientBuilder;
use reqwest::header::HeaderMap;

use std::error::Error;

use log::debug;

pub struct Env {
    pub sso_url: &'static str,
    pub neboapp_url: &'static str,
    pub client_id: &'static str,
    pub sso_redirect_uri: &'static str
}

lazy_static! {
    pub static ref ENV: HashMap<&'static str, &'static Env> = {
        let mut m = HashMap::new();
        m.insert("local", &Env {
            sso_url: "http://localhost:8080",
            neboapp_url: "http://localhost:8899",
            client_id: "nebowebap_cloudtest",
            sso_redirect_uri: "http://localhost:8099/ssocallback"
        });
        m.insert("prod",&Env  {
            sso_url: "https://sso.myscript.com",
            neboapp_url: "https://www.nebo.app",
            client_id: "nebowebap",
            sso_redirect_uri: "https://nebo.app/callback"
        });
        m.insert("cloudtest",&Env  {
            sso_url: "https://sso.corp.myscript.com",
            neboapp_url: "https://neboapp.corp.myscript.com",
            client_id: "nebowebap_cloudtest",
            sso_redirect_uri: "https://neboapp.corp.myscript.com/callback"
        });
        m.insert("cloudtest2",&Env  {
            sso_url: "https://sso.corp.myscript.com",
            neboapp_url: "https://neboapp2.corp.myscript.com",
            client_id: "nebowebap_cloudtest",
            sso_redirect_uri: "https://neboapp2.corp.myscript.com/callback"
        });
        m
    };
}

pub fn get_default_client(token: &str) -> Client {
    let mut headers = HeaderMap::new();
    headers.insert(http::header::AUTHORIZATION, token.parse().unwrap());
    headers.insert(http::header::CONTENT_TYPE, "application/json".parse().unwrap());
    
    ClientBuilder::new()
        .default_headers(headers)   
        .build()
        .unwrap()
}

pub const NEBO_API_URI_PAGES: &str = "/api/v2.0/nebo/pages";

pub fn scan_dir(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    debug!("Scanning directory {}", path);
    let mut result = Vec::new();
    for dir_entry in std::fs::read_dir(path)? {
        if let Ok(dir) = dir_entry {
            if let Some(s) = dir.path().to_str() {
                let mut is_dir = false;
                if let Ok(file_type) = dir.file_type() {
                    if file_type.is_dir() {
                        is_dir = true;
                        if let Ok(mut content) = scan_dir(s) {
                            result.append(&mut content);
                        }
                    }
                }
                if !is_dir {
                    if let Ok(string) = dir.file_name().into_string() {
                        if string.ends_with(".nebo") {
                            result.push(s.into());
                        }
                    }
                }
            }
        }
    }
    debug!("{} nebo files in {}", result.len(), path);
    return Ok(result);
}