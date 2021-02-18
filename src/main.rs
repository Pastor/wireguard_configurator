#![allow(unused_imports)]

extern crate base64;
extern crate getopts;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use std::borrow::{Borrow, Cow};
use std::borrow::BorrowMut;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::env;
use std::hash::Hash;
use std::io::Read;
use std::path::PathBuf;
use std::str::from_utf8;
use std::sync::{Arc, Mutex};

use base64::{decode, decode_config, encode, encode_config};
use getopts::Options;

use crate::config::Config;

mod config;
mod transform;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("c", "config", "configuration file");
    opts.optflag("h", "help", "usage");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let config_filename = matches.opt_str("c")
        .unwrap_or("Configuration.toml".to_owned());

    let config = Config::new(config_filename.as_str()).expect("Configuration can't load");
    println!("{}", config.to_string())
}
