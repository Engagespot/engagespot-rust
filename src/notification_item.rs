use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NotificationItem {
    /// Title of the notification item. Required.
    title: String,
    /// Message of the notification item.
    message: Option<String>,
    /// Url of the notification item.
    url: Option<String>,
    /// Icon of the notification item.
    icon: Option<String>,
}

impl NotificationItem {
    /// Create a new notification item with the title only.
    ///  Other fields can be set by chaining the methods.
    /// **Example:**
    /// ```
    /// use engagespot::NotificationItem;
    /// let notification_item = NotificationItem::new("Title")
    /// .message("Message")
    /// .url("https://google.com")
    /// .icon("favicon.png");
    /// ```
    pub fn new(title: &str) -> Self {
        NotificationItem {
            title: title.to_string(),
            message: None,
            url: None,
            icon: None,
        }
    }

    /// Create a new notification item title, message, url and icon.
    /// **Example:**
    /// ```
    /// use engagespot::NotificationItem;
    /// let notification_item = NotificationItem::with_args("Title", "Message", "https://google.com", "favicon.png");
    /// ```
    pub fn with_args(title: &str, message: &str, url: &str, icon: &str) -> Self {
        NotificationItem {
            title: title.to_string(),
            message: Some(message.to_string()),
            url: Some(url.to_string()),
            icon: Some(icon.to_string()),
        }
    }

    /// Set the title of the notification item.
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Set the message of the notification item.
    pub fn message(mut self, message: &str) -> Self {
        self.message = Some(message.to_string());
        self
    }

    /// Set the url of the notification item.
    pub fn url(mut self, url: &str) -> Self {
        self.url = Some(url.to_string());
        self
    }

    /// Set the icon of the notification item.
    pub fn icon(mut self, icon: &str) -> Self {
        self.icon = Some(icon.to_string());
        self
    }
}
