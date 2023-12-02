use serde::{Serialize, Deserialize};
use tokio::{io::AsyncReadExt, pin};
use wtransport::{ClientConfig, Endpoint};
use log::debug;

use quichat_datatypes::api::api_response;


#[tokio::main]
async fn main() {
    
    // Initialize logger
    env_logger::builder().filter(Some(module_path!()), log::LevelFilter::Debug).init();

    // Initialize the client
    // let config = ClientConfig::default();

    // let tls_config = rustls::ClientConfig::builder().with_safe_defaults().with_root_certificates(rustls::RootCertStore::empty()).with_no_client_auth();
    let config = ClientConfig::builder().with_bind_default().with_native_certs().build();

    let connection = Endpoint::client(config).unwrap().connect("https://127.0.0.1:8080/wt?api_version=V1").await.unwrap();
    
    // let (tx, rx) = connection.accept_bi().await.unwrap();
    // let handler1 = handle_bi(tx, rx);

    // let (tx, rx) = connection.accept_bi().await.unwrap();
    // let handler2 = handle_bi(tx, rx);

    // tokio::join!(handler1, handler2);
    // tokio::spawn(future);

    loop {
        tokio::select! {
            Ok((tx, rx)) = connection.accept_bi() => {
                // let (tx, rx) = stream.split();
                tokio::spawn(handle_bi(tx, rx));
            }
            else => {
                break
            }
        }
    }

    debug!("Connection closed");

    // let mut data = vec![0; 1024];
    // while let Some(num_read) = rx.read(&mut data).await.unwrap() {
    //     let packet_size = usize::from_ne_bytes(data[..8].try_into().unwrap());
    //     debug!("data is: {:?}", packet_size);
    //     let packet = rmp_serde::decode::from_slice::<TestStructV1>(&data[8..8+packet_size as usize]).unwrap();
    //     debug!("Received packet:\n{:?}", packet);
    // }

}

async fn handle_bi(tx: impl tokio::io::AsyncWrite, rx: impl tokio::io::AsyncRead) {
    pin!(tx, rx);

    let mut data = vec![0; 1024];
    
    while let Ok(num_read) = rx.read(&mut data).await {
        let packet_size = usize::from_ne_bytes(data[..8].try_into().unwrap());
        let packet = rmp_serde::decode::from_slice::<api_response::ResponseTypes>(&data[8..8+packet_size as usize]).unwrap();
        debug!("Received packet:\nTotal packet size: {}\nData size: {}\n{:?}", num_read, packet_size, packet);
    }

}

#[derive(Debug, Serialize, Deserialize)]
struct TestStructV1 {
    name: String
}
