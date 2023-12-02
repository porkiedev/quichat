/// This is the translation layer between the API and the Database
pub mod api;

/// This contains the datatypes used by the server/client to create API requests
pub mod api_datatypes;

/// This contains the datatypes that are ultimately stored in the database
pub mod db_datatypes;

/// This contains the datatypes that are ultimately used by the client
/// 
/// They are typically very similar to their `db_datatypes` counterparts, but (sometimes) with some confidental information stripped
pub mod client_datatypes;
