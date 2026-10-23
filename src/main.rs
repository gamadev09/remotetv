use std::fs;
use std::io::{Read, Write};
use std::net::{IpAddr, TcpListener, TcpStream};
use std::net::Command;

const PORT: u16 = 8000;
const CONF_FILE: &str = "config.conf";

#[derive(Clone, Default)]
struct Config {
    adb: String,
    tv: String,
}

impl Config{
    fn is_set(&self) -> bool {
        !self.adb.is_empty() && !self.tv.is_empty()
    }
}


fn load_config() -> Config {
    let mut cfg = Config::default();
    if let Ok(text) = fs::read_to_string(CONF_FILE) {
        for line in text.lines() {
            if let Some((k, v)) = line.split_once('=') {
                match k.trim() {
                    "adb" => cfg.adb = v.trim().to_string(),
                    "tv" => cfg.tv = v.trim().to_string(),
                    _ => {}
                }
            }
        }
    }
    cfg
}

fn save_config(cfg: &Config) -> std::io::Result<()> {
    fs::write(CONF_FILE, format!("adb={}\ntv={}\n", cfg.adb, cfg.tv))
}


