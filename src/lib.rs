//! # Engagespot
//!
//! Rust library for communicating with Engagespot REST API. Send multi-channel notifications from your rust app.
//!
//! ## Pre-requisites
//! You need Engagespot API KEY and API SECRET from your engagespot [dashboard](https://portal.engagespot.co/feed) to get started.
//! If you don't have one, just get one for free.
//!
//! ## Installation
//! `cargo add engagespot`
//!
//!
//! ## Usage
//!
//! ```
//!
//! use engagespot::{Engagespot, NotificationBuilder};
//!
//! // A reactor like tokio or async-std is required to run the async code.
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() {
//!   // initialize engagespot client by passing the api_key and api_secret
//!   let client = Engagespot::new("api_key", "api_secret");
//!
//! let notification = NotificationBuilder::new("title", &vec!["foo@bar.com".to_string()]).build();
//! let response = client.send(&notification).await.unwrap_or_else(|err: String| format!("Error: {}", err));
//! println!("new res is {}", response);
//! }
//! ```

mod channels;
mod client;
mod notification;
mod notification_item;

pub use channels::Channels;
pub use client::{Engagespot, EngagespotBuilder};
pub use notification::{Notification, NotificationBuilder};
pub use notification_item::NotificationItem;
