use std::{fs, io, result};
use std::error::Error;
use std::fs::File;

use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    #[serde(rename = "Gate")]
    pub gate: Gate,
    #[serde(rename = "SecurityGroup")]
    pub security_group: Vec<SecurityGroup>,
    #[serde(rename = "User")]
    pub user: Vec<User>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct User {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "ip")]
    pub ip: String,
    #[serde(rename = "public_key")]
    pub public_key: String,
    #[serde(rename = "security_group")]
    pub security_group: Vec<String>
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SecurityGroup {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "allowed_ip")]
    pub allowed_ip: Vec<String>
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Gate {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "port")]
    pub port: u16,
    #[serde(rename = "pool")]
    pub pool: String,
    #[serde(rename = "private_key")]
    pub private_key: String
}

impl Config {
    #[inline(always)]
    pub fn new(filename: &str) -> Option<Config> {
        let text: String = fs::read_to_string(filename).unwrap();
        let config = toml::from_str(text.as_str()).unwrap();
        Some(config)
    }
}
