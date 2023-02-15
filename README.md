# Engagespot Rust Library

[![Crates Badge](https://img.shields.io/crates/v/engagespot)](https://crates.io/crates/engagespot)
[![Docs.rs Badge](https://docs.rs/engagespot/badge.svg)](https://docs.rs/engagespot/)

Official rust library for communicating with Engagespot REST API. Send multi-channel notifications from your rust app.

You need Engagespot API KEY and API SECRET from your [dashboard](https://portal.engagespot.co) to get started. If you don't have one, just get one for free.

## Installation

Add to your `Cargo.toml`:

`cargo add engagespot`

## Getting Started

```rust
 use engagespot::{Engagespot, NotificationBuilder};

 // A reactor like tokio or async-std is required to run the async code.
 #[tokio::main(flavor = "current_thread")]
 async fn main() {
   // initialize engagespot client by passing the api_key and api_secret
   let client = Engagespot::new("api_key", "api_secret");

 let notification = NotificationBuilder::new("title", &vec!["foo@bar.com".to_string()]).build();
 let response = client.send(&notification).await.unwrap_or_else(|err: String| format!("Error: {}", err));
 println!("new res is {}", response);
```

## License

This code is free to use under the terms of the MIT license.
