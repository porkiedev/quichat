//
// These are the datatypes that are ultimately used by the client.
// They are similar to their `db_datatypes` counterparts, but with some confidental information stripped out.
//

use serde::{Serialize, Deserialize};


/// A user struct.
/// 
/// This contains information about a user
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    /// The user's id
    pub id: u32,
    /// The user's username
    pub username: String,
    /// The date and time that the user registered their account
    pub datetime: Option<chrono::NaiveDateTime>
}


/// A message struct.
/// 
/// This contains information about a message
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    /// The ID of the user who sent the message
    pub user_id: u32,
    /// The ID of the channel that the message was sent in
    pub channel_id: u32,
    /// The ID of the message object itself
    pub message_id: u32,
    /// The actual text content of the message
    pub content: String,
    /// The date and time that the message was sent
    pub datetime: chrono::NaiveDateTime
}
