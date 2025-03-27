use ghost_gst::{CliArgs, Config, Connection};

// Define a default connection type with compile-time parameters
type DefaultConnection = Connection<100, 30000, 200, 10>;
// 100ms base, 30s max, 2.0 factor, 0.1 jitter

fn main() {
    let args = match CliArgs::parse() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Error parsing arguments: {}", e);
            return;
        }
    };

    if args.verbose {
        println!("Verbose mode enabled");
    }

    let config = if let Some(config_path) = args.config_path {
        match Config::load(&config_path) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("Error loading config: {}", e);
                return;
            }
        }
    } else {
        Config {
            host: args.host.unwrap_or_else(|| "localhost".to_string()),
            port: args.port.unwrap_or(8080),
        }
    };

    println!("Attempting to connect to {}:{}", config.host, config.port);

    match DefaultConnection::new(&config.host, config.port, 5) {
        Ok(mut connection) => {
            println!("Connected successfully!");
            if let Err(e) = connection.send("Hello from Ghost (gst)!") {
                eprintln!("Error sending message: {}", e);
                return;
            }
            match connection.receive() {
                Ok(response) => println!("Received response: {}", response),
                Err(e) => eprintln!("Error receiving response: {}", e),
            }
        }
        Err(e) => {
            println!("Could not connect to server: {}", e);
            println!(
                "This is expected if no server is running on {}:{}",
                config.host, config.port
            );
        }
    }

    println!("Ghost (gst) terminated successfully.");
    println!("Exiting Ghost (gst)...");
}
