//! Hello world server.
//!
//! A simple client that connects to a tidis server, sets key "hello" with value "world",
//! and gets it from the server after
//!
//! You can test this out by running:
//!
//!     cargo run --bin tidis-server
//!
//! And then in another terminal run:
//!
//!     cargo run --example hello_world

#![warn(rust_2018_idioms)]

use tidis::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    // Open a connection to the tidis address.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    client.set("hello", "world".into()).await?;

    // Get key "hello"
    let result = client.get("hello").await?;

    println!("got value from the server; success={:?}", result.is_some());

    Ok(())
}
