use reqwest::{header, Client, Error};
use serde::Serialize;

use crate::Notification;

const DEFAULT_BASE_URL: &str = "https://api.engagespot.co/v3";

/// Builder for creating the Engagespot client.
/// Additional configurations like base_url, client, etc. can be set using the builder methods.`
pub struct EngagespotBuilder {
    base_url: String,
    client: Client,
}

/// Engagespot client to communicate with Engagespot APIs for sending notifications, triggering templates,
/// creating categories, etc.
pub struct Engagespot {
    /// Engagespot API Base URL (default: https://api.engagespot.co/v3). No need to change unless you are using a self hosted Engagespot instance.
    base_url: String,
    /// The HTTP client used to make requests to Engagespot API. Default is reqwest::Client.
    client: Client,
}

fn create_default_client(api_key: &str, api_secret: &str) -> Result<Client, Error> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "Content-Type",
        header::HeaderValue::from_static("application/json"),
    );
    headers.insert(
        "X-ENGAGESPOT-API-KEY",
        header::HeaderValue::from_str(api_key).unwrap(),
    );
    headers.insert(
        "X-ENGAGESPOT-API-SECRET",
        header::HeaderValue::from_str(api_secret).unwrap(),
    );

    let client = Client::builder()
        .user_agent("engagespot-rust")
        .default_headers(headers)
        .build();
    client
}

impl EngagespotBuilder {
    /// Creates a new EngagespotBuilder with the given API key and API secret.
    /// API key and secret can be obtained from the Engagespot [dashboard](https://portal.engagespot.co/feed).
    /// Engagespot API Secret. Get it from your Engagespot [dashboard](https://portal.engagespot.co/feed).
    /// Additional configurations like base_url, client etc... can be chained using the builder methods.
    /// Once all configurations are set, call the build method to create the Engagespot client.
    /// **Example:**
    ///
    /// ```
    /// # use engagespot::EngagespotBuilder;
    /// let engagespot = EngagespotBuilder::new("key", "secret").base_url("base_url/v2").build();
    /// ```
    pub fn new(api_key: &str, api_secret: &str) -> Self {
        let client = create_default_client(api_key, api_secret)
            .unwrap_or_else(|error| panic!("Error creating client {:?}", error));

        EngagespotBuilder {
            base_url: DEFAULT_BASE_URL.to_string(),
            client,
        }
    }

    /// Sets the base_url for Engagespot API. Default is https://api.engagespot.co/v3.
    /// No need to change unless you are using a self hosted Engagespot instance.
    pub fn base_url(mut self, base_url: &str) -> Self {
        self.base_url = base_url.to_string();
        self
    }

    /// Builds and returns the Engagespot client.
    pub fn build(self) -> Engagespot {
        Engagespot {
            base_url: self.base_url,
            client: self.client,
        }
    }
}

impl Engagespot {
    /// Creates a new Engagespot client with the given API key and API secret.
    /// Use this method if you want to use engagespot with default configurations.
    /// **Example:**
    ///
    /// ```
    /// # use engagespot::Engagespot;
    /// let engagespot = Engagespot::new("key", "secret");
    /// ```
    pub fn new(api_key: &str, api_secret: &str) -> Engagespot {
        EngagespotBuilder::new(api_key, api_secret).build()
    }

    /// Sends a notification with required configuration
    /// send method takes a Notification struct as input.
    /// Notification struct can be created using the NotificationBuilder.
    /// **Example:**
    /// ```
    /// use engagespot::{Engagespot, NotificationBuilder};
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() {
    ///   let client = Engagespot::new("api_key", "api_secret");
    ///   let notification = NotificationBuilder::new("title", &vec!["hello@foo.com".to_string()]).build();
    ///   let response = client.send(&notification).await;
    /// }
    /// ```
    pub async fn send<T: Serialize>(
        &self,
        notification: &Notification<T>,
    ) -> Result<String, String> {
        let url = self.get_url("notifications");
        let response = self.client.post(&url).json(&notification).send().await;
        match response {
            Ok(response) => self.handle_response(response).await,
            Err(error) => Err(error.to_string()),
        }
    }

    /// Creates or updates a user with the given attributes.
    /// The attributes are passed as a generic type T which implements the Serialize trait.
    /// **Example:**
    /// 
    /// ```
    /// 
    /// use serde::{Deserialize, Serialize};
    /// use engagespot::Engagespot;
    ///  #[derive(Serialize, Deserialize)]
    ///  struct Data {
    ///     foo: String,
    /// }
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() {
    ///   let client = Engagespot::new("api_key", "api_secret");
    ///   client.create_or_update_user_attrs("identifier", &Data { foo: "bar".to_string() }).await;
    /// }
    /// ```
    /// 
    pub async fn create_or_update_user_attrs<T: Serialize>(&self, identifier: &str, attrs: &T) -> Result<String, String> {
        let url = self.get_url(format!("users/{identifier}").as_str());
        let response = self.client.put(&url).json(&attrs).send().await;
        match response {
            Ok(response) => self.handle_response(response).await,
            Err(error) => Err(error.to_string()),
        }
    }

    async fn handle_response(&self, response: reqwest::Response) -> Result<String, String> {
        let status = response.status();
        let response_text = response.text().await.unwrap();
        if !status.is_success() {
            return Err(response_text);
        }
        Ok(response_text)
    }

    fn get_url(&self, path: &str) -> String {
        format!("{}/{}", self.base_url, path)
    }
}


#[cfg(test)]
mod tests {
    use crate::{Engagespot, EngagespotBuilder};

    #[test]
    fn builder_init() {
        let client = EngagespotBuilder::new("api_key", "api_secret").build();
        assert_eq!(client.base_url, "https://api.engagespot.co/v3");
    }

    #[test]
    fn builder_methods() {
        let builder = EngagespotBuilder::new("api_key", "api_secret")
            .base_url("https://api.engagespot.co/v5");
        let client = builder.build();
        assert_eq!(client.base_url, "https://api.engagespot.co/v5");
    }

    #[test]
    fn engagespot_init() {
        let client = Engagespot::new("api_key", "api_secret");
        assert_eq!(client.base_url, "https://api.engagespot.co/v3");
    }
}
