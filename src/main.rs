use std::net::TcpListener;

use email_newsletter::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind por");
    run(listener)?.await
}
