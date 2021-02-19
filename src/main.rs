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
use std::io::{Read, Write};
use std::path::{PathBuf, Path};
use std::str::from_utf8;
use std::sync::{Arc, Mutex};

use base64::{decode, decode_config, encode, encode_config};
use getopts::Options;

use crate::config::Config;
use std::fs::File;
use std::process::Command;

mod config;
mod transform;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

#[inline(always)]
fn home_dir(path: &str) -> &Path {
    let dir = env::home_dir().unwrap();
    let _file_path = format!("{}\\{}", dir.display(), path);
    return Path::new(".");
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("c", "config", "configuration file");
    opts.optflag("r", "reload", "reload configuration file");
    opts.optflag("h", "help", "usage");
    opts.optopt("f", "file_name", "", "FILENAME");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return Ok(());
    }
    let config_filename = matches.opt_str("c")
        .unwrap_or("Configuration.toml".to_owned());

    let config = Config::from_file(config_filename.as_str()).expect("Configuration can't load");
    if matches.opt_present("r") {
        let output = matches.opt_str("f");
        let input = match output {
            Some(f) => f,
            _ => String::from("wg0.conf")
        };

        //Write to /etc/wireguard/wg0.conf
        let path = Path::new(&input);
        let mut file = match File::open(&path) {
            Err(_) => File::create(path)?,
            Ok(file) => file,
        };
        file.write_all(config.to_string().as_bytes())?;
        file.flush()?;
        //Reload //sudo systemctl start wg-quick@wg0.service
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", "echo hello"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg("systemctl")
                .arg("restart")
                .arg("wg-quick@wg0.service")
                .output()
                .expect("failed to execute process")
        };
        let process_output = output.stdout;
        println!("{}", String::from_utf8(process_output).unwrap());
        return Ok(());
    }
    println!("{}", config.to_string());
    Ok(())
}
