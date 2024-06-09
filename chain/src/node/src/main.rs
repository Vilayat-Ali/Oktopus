// pub mod error;
// pub mod handler;

// use std::{collections::HashMap, env::args, net::SocketAddr, sync::{Arc, Mutex}};
// use tokio::net::TcpListener;
// use futures_channel::mpsc::UnboundedSender;
// use tokio_tungstenite::tungstenite::protocol::Message;
// use tracing::{info, Level};
// use tracing_subscriber::{self, FmtSubscriber};

// use handler::handler_connection;
// use utils::ascii_art;

// pub(crate) type Tx = UnboundedSender<Message>;
// pub(crate) type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;
// pub(crate) type State = Arc<Mutex<HashMap<SocketAddr, UnboundedSender<Message>>>>;

// #[tokio::main]
// async fn main() -> Result<(), std::io::Error> {
//     println!("{}", ascii_art::VILCHAIN_ASCII_LOGO_ART);
    
//     let subscriber = FmtSubscriber::builder()
//         .with_max_level(Level::INFO)
//         .finish();
//     tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

//     let port = args().nth(1).unwrap_or("8000".to_string());
//     let addr = format!("127.0.0.1:{port}");

//     let state = PeerMap::new(Mutex::new(HashMap::new()));

//     let try_socket = TcpListener::bind(&addr).await;
//     let listener = try_socket.expect("Failed to bind");

//     info!("Listening on: {}", addr);

//     // Let's spawn the handling of each connection in a separate task.
//     while let Ok((stream, addr)) = listener.accept().await {
//         tokio::spawn(handler_connection(state.clone(), stream, addr));
//     }

//     Ok(())
// }

use structure::BigNum;

fn main() {
    let mut big_num_1 = BigNum::from("12345.6789");
        let big_num_2 = BigNum::from("12345.6789");
        big_num_1.substract(&big_num_2);

        println!("{}", big_num_1.to_string());
}
