use std::{fs, io, result};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

use serde_derive::{Deserialize, Serialize};

pub struct Config {
    pub address: String,
    pub port: u16,
    pub pool: String,
    pub private_key: String,
    pub security_groups: HashMap<String, SecurityGroup>,
    pub users: HashMap<String, User>,
}

pub struct User {
    pub name: String,
    pub ip: String,
    pub public_key: String,
    pub security_group: Vec<String>,
}

pub struct SecurityGroup {
    pub name: String,
    pub allowed_ip: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone)]
struct _Config {
    #[serde(rename = "Gate")]
    pub gate: _Gate,
    #[serde(rename = "SecurityGroup")]
    pub security_group: Vec<_SecurityGroup>,
    #[serde(rename = "User")]
    pub user: Vec<_User>,
}

#[derive(Deserialize, Serialize, Clone)]
struct _User {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "ip")]
    pub ip: String,
    #[serde(rename = "public_key")]
    pub public_key: String,
    #[serde(rename = "security_group")]
    pub security_group: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone)]
struct _SecurityGroup {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "allowed_ip")]
    pub allowed_ip: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone)]
struct _Gate {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "port")]
    pub port: u16,
    #[serde(rename = "pool")]
    pub pool: String,
    #[serde(rename = "private_key")]
    pub private_key: String,
}

impl Config {
    #[inline(always)]
    pub fn new(filename: &str) -> Option<Config> {
        let text: String = fs::read_to_string(filename).unwrap();
        let config: _Config = toml::from_str(text.as_str()).unwrap();
        let mut security_groups = HashMap::new();
        config.security_group.iter().for_each(|g| {
            security_groups.insert(g.name.clone(), SecurityGroup {
                name: g.name.clone(),
                allowed_ip: g.allowed_ip.clone(),
            });
        });
        let mut users = HashMap::new();
        config.user.iter().for_each(|u| {
            users.insert(u.name.clone(), User {
                name: u.name.clone(),
                security_group: u.security_group.clone(),
                ip: u.ip.clone(),
                public_key: u.public_key.clone(),
            });
        });
        Some(Config {
            address: config.gate.address,
            port: config.gate.port,
            pool: config.gate.pool,
            private_key: config.gate.private_key,
            security_groups,
            users,
        })
    }
}

impl ToString for Config {
    fn to_string(&self) -> String {
        let mut buf = String::new();
        buf += "[Interface]\n";
        buf.push_str(format!("Address = {}\n", self.address.clone()).as_str());
        buf.push_str(format!("ListenPort = {}\n", self.port).as_str());
        buf.push_str(format!("PrivateKey = {}\n", self.private_key).as_str());
        buf
    }
}

