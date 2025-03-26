pub struct CliArgs {
    pub verbose: bool,
    pub config_path: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
}

impl CliArgs {
    pub fn parse() -> Result<Self, String> {
        let args: Vec<String> = std::env::args().collect();
        let mut verbose = false;
        let mut config_path = None;
        let mut host = None;
        let mut port = None;

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "-v" | "--verbose" => verbose = true,
                "-c" | "--config" => {
                    i += 1;
                    if i < args.len() {
                        config_path = Some(args[i].clone());
                    } else {
                        return Err("Missing value for config path".to_string());
                    }
                }
                "-h" | "--host" => {
                    i += 1;
                    if i < args.len() {
                        host = Some(args[i].clone());
                    } else {
                        return Err("Missing value for host".to_string());
                    }
                }
                "-p" | "--port" => {
                    i += 1;
                    if i < args.len() {
                        port = Some(
                            args[i]
                                .parse()
                                .map_err(|_| "Invalid port number".to_string())?,
                        );
                    } else {
                        return Err("Missing value for port".to_string());
                    }
                }
                _ => return Err(format!("Unknown argument: {}", args[i])),
            }
            i += 1;
        }

        Ok(CliArgs {
            verbose,
            config_path,
            host,
            port,
        })
    }
}
