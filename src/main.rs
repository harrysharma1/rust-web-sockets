use std::usize;
use tokio::sync::mpsc;
use warp::filters::ws::Message;
mod websocket;
mod handler;

pub struct Client{
    pub uid: usize,
    pub topics: Vec<String>,
    pub sender: Option<tokio::sync::mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

fn main() {
    println!("Hello, world!");
}
