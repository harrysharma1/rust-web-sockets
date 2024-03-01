use crate::{websocket, Client, Clients, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::{http::StatusCode, reply::json, ws::Message, Reply};


#[derive(Deserialize, Debug)]
pub struct RegisterRequest {
    uid: usize,
    topic: String,
}

#[derive(Deserialize)]
pub struct TopicActionRequest {
    topic: String,
    client_id: String,
}


#[derive(Serialize, Debug)]
pub struct RegisterResponse {
    url: String,
}

#[derive(Deserialize, Debug)]
pub struct Event {
    topic: String,
    uid: Option<usize>,
    message: String,
}


pub async fn publish_handler(body: Event, clients: Clients) -> Result<impl Reply> {
    clients
        .read()
        .await
        .iter()
        .filter(|(_, client)| match body.uid {
            Some(v) => client.uid == v,
            None => true,
        })
        .filter(|(_, client)| client.topics.contains(&body.topic))
        .for_each(|(_, client)| {
            if let Some(sender) = &client.sender {
                let _ = sender.send(Ok(Message::text(body.message.clone())));
            }
        });

    Ok(StatusCode::OK)
}

pub async fn register_handler(body: RegisterRequest, clients: Clients) -> Result<impl Reply> {
    let uid = body.uid;
    let topic = body.topic; // Capture the entry topic
    let uuid = Uuid::new_v4().as_simple().to_string();

    register_client(uuid.clone(), uid, topic, clients).await; // Pass the entry topic
    Ok(json(&RegisterResponse {
        url: format!("ws://127.0.0.1:8000/ws/{}", uuid),
    }))
}

async fn register_client(id: String, uid: usize, topic: String, clients: Clients){
    clients.write().await.insert(
        id, 
        Client{
            uid,
            topics: vec![topic],
            sender:  None,
            
        }
    );
}

pub async  fn unregister_handler(id: String, clients: Clients) -> Result<impl Reply> {
    clients.write().await.remove(&id);
    Ok(StatusCode::OK)
}

pub async fn ws_handler(ws: warp::ws::Ws, id: String, clients: Clients) -> Result<impl Reply> {
    let client = clients.read().await.get(&id);
    match client{
        Some(c) => Ok(ws.on_upgrade(move |socket| websocket::client_connection(socket, id, clients, c))),
        None => Err(warp::reject::not_found()),
    }
}

pub async fn health_handler() -> Result<impl Reply>{
    Ok(StatusCode::OK)
}

pub async fn add_topic(body: TopicActionRequest, clients: Clients) -> Result<impl Reply> {
    let mut client_write = clients.write().await;
    if let Some(client) = client_write.get_mut(&body.client_id){
        client.topics.push(body.topic);
    }
    Ok(warp::reply::with_status("Added topic successfully", StatusCode::OK))
}

pub async fn remove_topic(body: TopicActionRequest, clients: Clients) -> Result<impl Reply> {
    let mut client_write = clients.write().await;
    if let Some(client) = client_write.get_mut(&body.client_id){
        client.topics.retain(|t| t!= &body.topic);
    }
    Ok(warp::reply::with_status("Removed topic successfully", StatusCode::OK))
}






