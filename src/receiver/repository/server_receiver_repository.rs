use std::sync::Arc;
use async_trait::async_trait;
use tokio::net::TcpStream;
use crate::utility::initializer::AcceptorChannel;

#[async_trait]
pub trait ServerReceiverRepository {
    async fn receive(&mut self);
    async fn inject_accept_channel(&mut self, acceptor_channel_arc: Arc<AcceptorChannel>);
}