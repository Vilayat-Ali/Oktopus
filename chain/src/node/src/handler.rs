use super::State;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use futures_channel::mpsc::unbounded;
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};

pub(crate) async fn handler_connection(state: State, raw_stream: TcpStream, addr: SocketAddr) {
    todo!()
}