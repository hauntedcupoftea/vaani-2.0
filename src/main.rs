use std::env;
use std::error::Error;
mod embedder;
mod model;
mod server;
mod train;

// Define available modes as an enum
#[derive(Debug, Clone, PartialEq)]
enum Mode {
    Server,
    Train,
    // Easy to add more modes here
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Server
    }
}

fn parse_args() -> Mode {
    let args: Vec<String> = env::args().collect();
    // Loop through arguments to find "--mode"
    if let Some(pos) = args.iter().position(|arg| arg == "--mode") {
        // Check if there's a value after "--mode"
        if pos + 1 < args.len() {
            match args[pos + 1].to_lowercase().as_str() {
                "server" => return Mode::Server,
                "train" => return Mode::Train,
                // Add more modes here as needed
                _ => eprintln!("Unknown mode: {}, using default", args[pos + 1]),
            }
        }
    }

    Mode::default()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line arguments
    let mode = parse_args();

    println!("Running in {:?} mode", mode);

    // Handle different modes
    match mode {
        Mode::Server => server::start_server().await,
        Mode::Train => train::train(),
        // Easy to add more modes here
    }

    Ok(())
}
