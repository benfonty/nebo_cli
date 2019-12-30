
pub struct Env {
    pub sso_url: &'static str,
    pub client_id: &'static str,
    pub sso_redirect_uri: &'static str
}
use std::collections::HashMap;

lazy_static! {
    pub static ref ENV: HashMap<&'static str, &'static Env> = {
        let mut m = HashMap::new();
        m.insert("local", &Env {
            sso_url: "http://localhost:8080",
            client_id: "nebowebap_cloudtest",
            sso_redirect_uri: "http://localhost:8099/ssocallback"
        });
        m.insert("prod",&Env  {
            sso_url: "https://sso.myscript.com",
            client_id: "nebowebap",
            sso_redirect_uri: "https://nebo.app/callback"
        });
        m.insert("cloudtest",&Env  {
            sso_url: "https://sso.corp.myscript.com",
            client_id: "nebowebap_cloudtest",
            sso_redirect_uri: "https://neboapp.corp.myscript.com/callback"
        });
        m.insert("cloudtest2",&Env  {
            sso_url: "https://sso.corp.myscript.com",
            client_id: "nebowebap_cloudtest",
            sso_redirect_uri: "https://neboapp2.corp.myscript.com/callback"
        });
        m
    };
}