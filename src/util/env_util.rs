extern crate dotenv;

use dotenv::dotenv;
use std::{collections::HashMap, env};

pub fn get_dot_env_map() -> HashMap<String, String> {
    dotenv().ok();
    pretty_env_logger::init_timed();
    let mut dotenv_vars: HashMap<String, String> = HashMap::new();
    for (key, val) in env::vars() {
        // Use pattern bindings instead of testing .is_some() followed by .unwrap()
        dotenv_vars.insert(key, val);
    }
    dotenv_vars
}
