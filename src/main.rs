use std::net::TcpListener;

use trsws::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:800")
        .expect("Failed to bind to port 8000");
    run(listener)?.await
}
