extern crate dotenv;

use dotenv::dotenv;
use std::{env, collections::HashMap};

pub const APP_NAME: &str = "LEAGUE_APP";

pub fn get_dot_env_map() -> HashMap<String, String>{
    dotenv().ok();
    let mut dotenv_vars: HashMap<String, String> = HashMap::new();
    for (key, val) in env::vars() {
        // Use pattern bindings instead of testing .is_some() followed by .unwrap()
        dotenv_vars.insert(key, val);
    }
    dotenv_vars
}