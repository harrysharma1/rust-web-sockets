use std::{collections::HashMap};
use std::convert::Infallible;
use std::sync::Arc;
use handler::TopicActionRequest;
use tokio::sync::{mpsc, RwLock};
use warp::{ws::Message, Filter, Rejection};
use crate::handler::{add_topic, remove_topic};

mod handler;
mod websocket;


type Result<T> = std::result::Result<T, Rejection>;
type Clients = Arc<RwLock<HashMap<String,Client>>>;

#[derive(Debug, Clone)]
pub struct Client{
    pub uid: usize, 
    pub topics: Vec<String>,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

async fn main() {
    let clients:Clients = Arc::new(RwLock::new(HashMap::new()));
    
    // Generating routes for web sockets
    let health_route = warp::path!("health").and_then(handler::health_handler);
    
    let register = warp::path("register");
    let register_route = register
        .and(warp::post())
        .and(warp::body::json())
        .and(with_clients(clients.clone()))
        .and_then(handler::register_handler)
        .or(register
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_clients(clients.clone()))
            .and_then(handler::)

        );
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}
