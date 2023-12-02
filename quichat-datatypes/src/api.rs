use serde::{Serialize, Deserialize};

//
// This file contains all of the datatypes that are used by the server and/or client for generating requests
// Basically, these structs are equivalent to the JSON objects you would sent to a regular HTTP API
//

/// The server will use the structs inside of the response module to reply to client requests
pub mod api_response {
    use serde::{Serialize, Deserialize};
    // use super::client_datatypes;
    use super::super::client;

    /// Version 1 API Response types
    #[derive(Debug, Serialize, Deserialize)]
    pub enum ResponseTypes {
        /// Was the message sent?
        SendMessage(SendMessageResponse),
        /// Was the message edited?
        EditMessage(EditMessageResponse),
        /// Was the message deleted?
        DeleteMessage(DeleteMessageResponse),
        /// Did we get the messages?
        GetMessages(GetMessagesResponse),
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
        pub message: client::Message,
    }

    /// EditMessageResponse struct
    /// 
    /// Send this to the client to indicate if the message was edited successfully
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EditMessageResponse {
        /// The response status
        pub status: ResponseStatus,
        /// The full message that was edited
        pub message: client::Message,
    }

    /// DeleteMessageResponse struct
    /// 
    /// Send this to the client to indicate if the message was deleted successfully
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DeleteMessageResponse {
        /// The response status
        pub status: ResponseStatus
    }

    /// GetMessagesResponse struct
    /// 
    /// Send this to the client to indicate if we successfully fetched all of the messages they requested
    /// - Note: There are no guarantees that the server will return exactly the number of messages that you requested. It can't return things that don't exist!
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetMessagesResponse {
        /// The response status
        pub status: ResponseStatus,
        /// A vec containing the messages that the client requested
        pub messages: Vec<client::Message>
    }

}

/// The client will use the structs inside of the request module to send requests to the server
pub mod api_request {
    use serde::{Serialize, Deserialize};

    /// Version 1 API Request types
    #[derive(Debug, Serialize, Deserialize)]
    pub enum RequestType {
        /// Try to send a message
        SendMessage(SendMessageRequest),
        /// Try to edit a message
        EditMessage(EditMessageRequest),
        /// Try to delete a message (all we really need is the message ID)
        DeleteMessage(DeleteMessageRequest),
        GetMessages(GetMessagesRequest),
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

    /// GetMessagesRequest struct
    /// 
    /// Send this to the server if you want to get some messages from a channel
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetMessagesRequest {
        /// The ID of the channel that you want to get messages from
        channel_id: u32,
        /// The number of messages that you want to get
        /// - Note: you can't get more than 255 messages per request, because that should never be required (always load messages in chunks on an as-needed basis)
        num_messages: u8,
        /// Optionally only get messages older than the provided message id
        /// - This is useful if need to load message history. Just create a request with the `message_id` of the oldest message you have.
        before_message_id: Option<u32>
    }

}

/// All existing API Versions
#[derive(Debug, Serialize, Deserialize)]
pub enum ApiVersion {
    V1,
    V2
}
