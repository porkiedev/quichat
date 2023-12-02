use serde::{Serialize, Deserialize};

//
// This file contains all of the datatypes that are used by the server and/or client for generating requests
// Basically, these structs are equivalent to the JSON objects you would sent to a regular HTTP API
//

/// The server will use the structs inside of the response module to reply to client requests
pub mod api_response {
    use serde::{Serialize, Deserialize};
    // use super::client_datatypes;
    use super::super::client_datatypes;

    /// Version 1 API Response types
    #[derive(Debug, Serialize, Deserialize)]
    pub enum ApiResponseType {
        /// Try to send a message
        SendMessage(SendMessageResponse),
        /// Try to edit a message
        EditMessage(EditMessageResponse),
        /// Try to delete a message (all we really need is the message ID)
        DeleteMessage(DeleteMessageResponse),
        Ping
    }

    /// The response status of a request
    #[derive(Debug, Serialize, Deserialize)]
    pub enum ResponseStatus {
        Success,
        AuthenticationError,
        Error(String)
    }

    /// SendMessageResponse struct
    /// 
    /// Send this to the client to indicate if the message was sent successfully
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SendMessageResponse {
        /// The response status
        pub status: ResponseStatus,
        /// The full message that was sent
        pub message: client_datatypes::Message,
    }

    /// EditMessageResponse struct
    /// 
    /// Send this to the client to indicate if the message was edited successfully
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EditMessageResponse {
        /// The response status
        pub status: ResponseStatus,
        /// The full message that was edited
        pub message: client_datatypes::Message,
    }

    /// DeleteMessageResponse struct
    /// 
    /// Send this to the client to indicate if the message was deleted successfully
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DeleteMessageResponse {
        /// The response status
        pub status: ResponseStatus
    }

}

/// The client will use the structs inside of the request module to send requests to the server
pub mod api_request {
    use serde::{Serialize, Deserialize};

    /// Version 1 API Request types
    #[derive(Debug, Serialize, Deserialize)]
    pub enum ApiRequestType {
        /// Try to send a message
        SendMessage(SendMessageRequest),
        /// Try to edit a message
        EditMessage(EditMessageRequest),
        /// Try to delete a message (all we really need is the message ID)
        DeleteMessage(DeleteMessageRequest),
        Ping
    }

    /// SendMessageRequest struct
    /// 
    /// Send this to the server if you want to send a message
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SendMessageRequest {
        /// The ID of the channel that you want to send the message in
        pub channel_id: u32,
        /// The actual text content of the message
        pub content: String
    }

    /// EditMessageRequest struct
    /// 
    /// Send this to the server if you want to edit an existing message
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EditMessageRequest {
        /// The ID of the message that you wish to edit
        pub message_id: u32,
        /// The text content that will replace the old message content
        pub content: String
    }

    /// DeleteMessageRequest struct
    /// 
    /// Send this to the server if you want to delete a message
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DeleteMessageRequest {
        /// The ID of the message that you wish to delete
        pub message_id: u32
    }

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

}

/// All existing API Versions
#[derive(Debug, Serialize, Deserialize)]
pub enum ApiVersion {
    V1,
    V2
}

