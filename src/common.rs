
pub struct Env {
    pub sso_url: &'static str
}
use std::collections::HashMap;

lazy_static! {
    pub static ref ENV: HashMap<&'static str, &'static Env> = {
        let mut m = HashMap::new();
        m.insert("local", &Env {
            sso_url: "http://localhost:8080"
        });
        m.insert("prod",&Env  {
            sso_url: "https://sso.myscript.com"
        });
        m
    };
}