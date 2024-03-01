use crate::{Client, Clients};
use futures::{FutureExt, StreamExt};
use serde::Deserialize;
use serde_json::from_str;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::UnboundedReceiverStream, StreamExt};
use warp::ws::{Message, WebSocket};

#[derive(Deserialize, Debug)]
pub struct TopicsRequest {
    topics: Vec<String>,
}

pub async fn client_connection(websocket: WebSocket, id: String, clients: Clients, mut client: Client){
    let (client_websocket_sender, mut client_websocket_rcv) = websocket.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();
    
    let client_rcv = UnboundedReceiverStream::new(client_rcv);
    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            eprintln!("error sending websocket msg: {}", e);
        }
    }));

    client.sender = Some(client_sender);
    clients.write().await.insert(id.clone(), client);

    println!("Connected: id-{}",id);

    while let Some(result) = client_websocket_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("Error receiving ws message for id-{}): {}", id.clone(), e);
                break;
            }
        };
        client_msg(&id, msg, &clients).await; 
        
    }
    
    clients.write().await.remove(&id);
    println!("Diconnected: id-{}",id);
}

async fn client_msg(id: &str, msg: Message, clients: &Clients){
    println!("Received message from id-{}: {:?}", id, msg);
    let message = match msg.to_str(){
        Ok(v) => v,
        None => return,
    };

    if message == "ping" || message == "ping\n"{
        return;
    }

    let topics_req: TopicsRequest = match from_str(&message) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error while parsing message to topics request: {}", e);
            return;
        }
    };

    let mut locked = clients.write().await;
    if let Some(v) = locked.get_mut(id) {
        v.topics = topics_req.topics;
    }
}
