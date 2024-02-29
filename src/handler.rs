se futures::Future;
use uuid::Uuid;
use warp::reply::Reply;

use crate::{Clients, RegisterRequest};

pub async fn register_handler(body: RegisterRequest, clients: Clients) -> Result<impl Reply>{
    let uid = body.uid;

    let uuid = Uuid::new_v4().simple().to_string();

    register_client(uuid.clone(), uid, clients).await;

    Ok(json(&RegisterResponse {
        url: format!("ws://127.0.0.1:8000/ws/{}", uuid),
    }))
    
}

async fn register_client(id: String, uid: usize, clients: Clients){
    clients.lock().await.insert(
        id,
        Client{
            uid,
            topics: vec![String::from("cats")],
            sender: None,
        },
    );
}

pub fn health_handler() -> impl Future<Output = Result<impl Reply>> {
    futures::future::ready(Ok(StatusCode::OK))
}
