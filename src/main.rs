//! src/main.rs

use std::net::TcpListener;
use website::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let address = TcpListener::bind("127.0.0.1:8000")?;
    run(address)?.await
}
