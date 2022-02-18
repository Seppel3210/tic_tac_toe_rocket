use clap::Parser;
use url::Url;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    server_address: String,
}

#[tokio::main]
async fn main() {
    let server_address = Cli::parse().server_address;
    let server = match Url::parse(&server_address) {
        Ok(url) => url,
        Err(reason) => {
            eprintln!("Failed to parse server address: {reason}");
            return;
        }
    };
}
