use std::fs;

pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let lines: Vec<&str> = contents.lines().collect();

        let host = lines.first().ok_or("Missing host in config")?.to_string();
        let port = lines.get(1).ok_or("Missing port in config")?.parse()?;

        Ok(Config { host, port })
    }
}
