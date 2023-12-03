use serde::{Serialize, Deserialize};
use tokio::{io::AsyncReadExt, pin};
use wtransport::{ClientConfig, Endpoint, endpoint::ConnectOptions};
use log::debug;

use quichat_datatypes::api::api_response;


#[tokio::main]
async fn main() {
    
    // Initialize logger
    env_logger::builder().filter(Some(module_path!()), log::LevelFilter::Debug).init();

    // Initialize the client
    let client_config = ClientConfig::builder().with_bind_default().with_native_certs().build();
    let connection_options = ConnectOptions::builder("https://127.0.0.1:8080/wt")
    .add_header("api-version", "V1").build();
    let connection = Endpoint::client(client_config).unwrap().connect(connection_options).await.unwrap();

    loop {
        tokio::select! {
            Ok((tx, rx)) = connection.accept_bi() => {
                tokio::spawn(handle_bi(tx, rx));
            },
            Ok(_rx) = connection.accept_uni() => {
                continue;
            }
            else => {
                break;
            }
        }
    }

    debug!("Connection closed");

}

async fn handle_bi(tx: impl tokio::io::AsyncWrite, rx: impl tokio::io::AsyncRead) {
    pin!(tx, rx);

    // Create a buffer that we can read data into
    let mut data = vec![0; 1024];
    
    let mut start_time = std::time::Instant::now();

    // Read data from the stream into the buffer
    while let Ok(num_read) = rx.read(&mut data).await {

        start_time = std::time::Instant::now();

        // The first 8 bytes of a packet is a usize that specifies how long the total packet is
        let packet_size = usize::from_ne_bytes(data[..8].try_into().unwrap());

        // Receive all of the data and serialize it into an api response type
        let packet = rmp_serde::decode::from_slice::<api_response::ResponseTypes>(&data[8..8+packet_size as usize]).unwrap();
        debug!("Received packet:\nTotal packet size: {}\nData size: {}\n{:?}", num_read, packet_size, packet);

    }

    let stop_time = std::time::Instant::now();
    debug!("Channel closed {}ms after read", stop_time.duration_since(start_time).as_millis());

}

#[derive(Debug, Serialize, Deserialize)]
struct TestStructV1 {
    name: String
}
