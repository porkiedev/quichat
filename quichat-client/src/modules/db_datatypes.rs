use serde::{Deserialize, Serialize};

///
/// This contains datatypes that are stored in the database itself
/// 

/// A user struct. This contains information about a user
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    /// The user id, this should be the same as the database table entry
    pub id: u32,
    /// The user's username
    pub username: String,
    /// The date and time that the user registered their account
    pub datetime: Option<chrono::NaiveDateTime>
}

/// A message struct. This contains information about a user message (including the message itself)
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    /// The user who sent the message
    pub user: User,
    /// The ID of the channel that the message was sent in
    pub channel_id: u32,
    /// The ID of the message object itself
    pub message_id: u32,
    /// The actual text content of the message
    pub content: String,
    /// The **server-assigned** date and time of the message
    /// - Note: The server will generate the date and time when it receives a message. This prevents abuse (clients getting to 'send messages' at unrealistic times, etc)
    pub datetime: chrono::NaiveDateTime
}

///
/// A channel struct
/// 
/// Every channel has a unique ID. Direct messages with other users is also considered a channel
/// 
#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    /// The unique id of the channel
    pub id: u32,
    /// The name of the channel
    pub name: String,
    /// The date and time that the channel was created
    pub datetime: Option<chrono::NaiveDateTime>
}
