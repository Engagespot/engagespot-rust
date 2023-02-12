use crate::NotificationItem;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Notification<T: Serialize = Option<String>> {
    /// Notification item. Required.
    notification: NotificationItem,
    /// Recipients of the notification. Required.
    /// Can be a list of emails or a list of user ids.
    recipients: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional data to be sent with the notification.
    /// Should be a serde serializable json object.
    data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Category of the notification. If not provided, it will be sent to everyone.
    category: Option<String>,
}

pub struct NotificationBuilder<'a, T: Serialize> {
    pub notification: NotificationItem,
    pub recipients: &'a Vec<String>,
    pub data: Option<T>,
    pub category: Option<String>,
}

impl<'a, T: Serialize> NotificationBuilder<'a, T> {

    /// Create a new notification builder with the title and recipients.
    /// Other fields can be set by chaining the methods.
    /// **Example:**
    /// ```
    /// use engagespot::{NotificationBuilder, NotificationItem};
    /// let notification = NotificationBuilder::new("Title", vec!["email1".to_string(), "email2".to_string()]).build();
    pub fn new(title: &str, recipients: &'a Vec<String>) -> Self {
        NotificationBuilder {
            notification: NotificationItem::new(title),
            recipients,
            data: None,
            category: None,
        }
    }

    /// Set the notification item
    pub fn notification_item(mut self, notification: NotificationItem) -> Self {
        self.notification = notification;
        self
    }

    /// Set the title of the notification item.
    pub fn title(mut self, title: &str) -> Self {
        self.notification = self.notification.title(title);
        self
    }

    /// Set the message of the notification item.
    pub fn message(mut self, message: &str) -> Self {
        self.notification = self.notification.message(message);
        self
    }

    /// Set the url of the notification item.
    pub fn url(mut self, url: &str) -> Self {
        self.notification = self.notification.url(url);
        self
    }

    /// Set the icon of the notification item.
    pub fn icon(mut self, icon: &str) -> Self {
        self.notification = self.notification.icon(icon);
        self
    }

    /// Set the recipients of the notification.
    pub fn recipients(mut self, recipients: &'a Vec<String>) -> Self {
        self.recipients = recipients;
        self
    }

    /// Set the additional data of the notification.
    /// Should be a serde serializable json object.
    pub fn data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }

    /// Set the category of the notification.
    pub fn category(mut self, category: String) -> Self {
        self.category = Some(category);
        self
    }

    /// Build the notification. Returns the Notification struct.
    pub fn build(self) -> Notification<T> {
        Notification {
            notification: self.notification,
            recipients: self.recipients.clone(),
            data: self.data,
            category: self.category,
        }
    }
}
