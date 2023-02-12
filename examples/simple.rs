use dotenv::dotenv;
use engagespot::{Engagespot, NotificationBuilder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Data<'a> {
    foo: &'a str,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenv()
        .ok()
        .expect("Error occurred while reading .env file");
    let api_key = std::env::var("API_KEY").expect("API_KEY must be set.");
    let api_secret = std::env::var("API_SECRET").expect("API_SECRET must be set.");

    let client = Engagespot::new(&api_key, &api_secret);

    let notification = NotificationBuilder::new("Test", &vec!["hemanditwiz@gmail.com".to_string()])
        .title("Message received")
        .message("New message received")
        .icon("favicon.png")
        .url("https://google.com")
        .data(&Data { foo: "bar" })
        .build();
    let res = client
        .send(&notification)
        .await
        .unwrap_or_else(|err: String| format!("Error: {}", err));
    println!("Response is {res}");

    let res = client
        .create_or_update_user_attrs("hemanditwiz@gmail.com", &Data { foo: "bar" })
        .await
        .unwrap_or_else(|err: String| format!("Error: {}", err));

    println!("Response is {res}");
}
