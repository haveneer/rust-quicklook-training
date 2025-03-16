mod options;

use actix_web::{
    error, get, middleware, post, web, App, Error, HttpResponse, HttpServer, Responder,
};
use clap::Parser;
use futures::StreamExt;
use options::Options;
use rand::prelude::{IteratorRandom, StdRng};
use rand::SeedableRng;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::sync::mpsc;
use std::sync::mpsc::SyncSender;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tracing::{error, info, warn};

const MAX_PAYLOAD_SIZE: usize = 256 * 1024; // max payload size is 256k
const MAX_MSPC_MESSAGE: usize = 1024;

struct InnerData {
    addrs: HashSet<SocketAddr>,
    counter: usize,
    tx: SyncSender<MPSCMessage>,
    pending_messages: Vec<ChatMessage>,
}

type SharedData = web::Data<(Mutex<InnerData>, SocketAddr)>;

#[derive(Serialize, Deserialize, Debug)]
enum ChatMessage {
    Message(String),
    DoYouKnow(SocketAddr),
}

#[derive(Serialize, Deserialize, Debug)]
struct RegistrationRequest {
    addr: SocketAddr,
}

#[derive(Serialize, Deserialize, Debug)]
enum RegistrationAnswer {
    Registered,
    AlreadyRegistered,
}

#[derive(Debug)]
struct MPSCMessage {
    message: String,
}

/// Test it using
/// ```shell
/// curl -X GET http://localhost:port/api/hello/john
/// ```
/// This entry point cannot fail
#[get("/api/hello/{name}")]
async fn hello(name: web::Path<String>, data: SharedData) -> impl Responder {
    let mut data = data.0.lock().await;
    data.counter += 1;
    info!(
        "Hello Request received (count={count})",
        count = data.counter
    );
    data.tx
        .send(MPSCMessage {
            message: "hello".to_string(),
        })
        .expect("Cannot send message");
    format!("Hello {name}!")
}

/// Test it using
/// ```shell
/// curl -X POST -d '{"Message": "hello"}' -H "Content-type: application/json" http://localhost:8080/api/chat
/// ```
/// This entry point may fail (=> Result return)
#[post("/api/chat")]
async fn chat(mut payload: web::Payload, data: SharedData) -> Result<HttpResponse, Error> {
    {
        // minimize lock time
        let message = format!("I'm from {}", data.1);
        let data = data.0.lock().await;
        info!("JSON Request received");
        data.tx
            .send(MPSCMessage { message })
            .expect("Cannot send message");
    }

    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_PAYLOAD_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // body is loaded, now we can deserialize serde-json
    match serde_json::from_slice::<ChatMessage>(&body)? {
        ChatMessage::Message(body) => Ok(HttpResponse::Ok().json(ChatMessage::Message(body))),
        ChatMessage::DoYouKnow(addr) => {
            if data.0.lock().await.addrs.insert(addr) {
                Ok(HttpResponse::Ok().json(ChatMessage::Message("Thanks".into())))
            } else {
                Ok(HttpResponse::Ok().json(ChatMessage::Message("I already know it".into())))
            }
        }
    }
}

// curl -X POST -d '{"username": "john"}' -H "Content-type: application/json" http://localhost:8080/api/json
#[post("/api/register")]
async fn register(mut payload: web::Payload, data: SharedData) -> Result<HttpResponse, Error> {
    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_PAYLOAD_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<RegistrationRequest>(&body)?;
    let mut data = data.0.lock().await;

    if data.addrs.insert(obj.addr) {
        info!("New friend registered from {}", obj.addr);
        data.pending_messages.push(ChatMessage::DoYouKnow(obj.addr));
        Ok(HttpResponse::Ok().json(RegistrationAnswer::Registered))
    } else {
        Ok(HttpResponse::Ok().json(RegistrationAnswer::AlreadyRegistered))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let options = Options::parse();

    // Configure RUST_LOG environment variable like
    // RUST_LOG=debug
    // RUST_LOG=gossip_server=debug
    // env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    // console_subscriber::init();
    tracing_subscriber::fmt::init();

    info!("Starting server");

    // Local communication
    let (tx, rx) = mpsc::sync_channel::<MPSCMessage>(MAX_MSPC_MESSAGE);

    // Shared data across the tasks (Data <=> Arc)
    let self_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::from_str("127.0.0.1")?), options.port());
    let data = web::Data::new((
        Mutex::new(InnerData {
            addrs: options.servers().iter().copied().collect(),
            counter: 0,
            tx,
            pending_messages: vec![],
        }),
        self_addr,
    ));

    let _chat = start_chat_task(data.clone()).await;

    register_on(data.0.lock().await.addrs.iter(), data.1).await;

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::clone(&data))
            .wrap(middleware::Logger::default())
            .service(register) // alternative form: .route("/api/register", web::post().to(register));
            .service(hello) // alternative form: .route("/api/hello/{name}", web::get().to(hello));
            .service(chat) // alternative form: .route("/api/chat", web::post().to(chat));
    })
    .bind(("127.0.0.1", options.port()))?
    .run();

    let _healthcheck = tokio::spawn(async move {
        forever_wait().await;
    });

    let _intracom = tokio::spawn(async move {
        while let Ok(m) = rx.recv() {
            warn!("Message: {}", m.message);
        }
    });

    server.await?;
    // tokio::join!(server, _healthcheck, _intracom, _chat); // FIXME should be like this
    Ok(())
}

async fn start_chat_task(data: SharedData) -> JoinHandle<()> {
    tokio::spawn(async move {
        let client = reqwest::Client::new(); // not necessary to instantiate a new one
        let mut rng = StdRng::from_entropy();
        loop {
            if let Some(addr) = data.0.lock().await.addrs.iter().choose(&mut rng) {
                let message = ChatMessage::Message(format!("Hello, {}!", addr));
                let url = format!("http://{}/api/chat", addr);
                match client.post(&url).json(&message).send().await {
                    Ok(response) => {
                        // info!("POST Headers: {:?}", response.headers());
                        match response.json::<ChatMessage>().await {
                            Ok(answer) => match answer {
                                ChatMessage::Message(body) => {
                                    info!("Answer: {}", body)
                                }
                                ChatMessage::DoYouKnow(_) => {
                                    todo!()
                                }
                            },
                            Err(err) => {
                                error!("{:?}", err);
                            }
                        }
                    }
                    Err(err) => error!("Failed to send POST request: {:?}", err),
                }
            }

            tokio::time::sleep(Duration::from_millis(2000)).await;
            // tokio::task::yield_now().await;
        }
    })
}

/// Do nothing, only display a healthcheck message every 10 seconds
async fn forever_wait() {
    loop {
        tokio::time::sleep(Duration::from_millis(10000)).await;
        info!("Waiting");
    }
}

async fn register_on(
    addrs: impl IntoIterator<Item = &SocketAddr>,
    self_addr: SocketAddr,
) -> JoinHandle<anyhow::Result<()>> {
    let addrs: Vec<_> = addrs.into_iter().copied().collect();
    tokio::spawn(async move {
        // Création d'un client HTTP reqwest
        let client = reqwest::Client::new();

        for other_addr in addrs {
            info!("Connecting to {:?}", other_addr);

            // Envoi d'une requête GET pour tester l'adresse
            let message = "John";
            let url = format!("http://{}/api/hello/{}", other_addr, message);
            match client.get(&url).send().await {
                Ok(response) => info!("GET Response: {:?}", response.text().await?),
                Err(err) => error!("Failed to send GET request: {:?}", err),
            }

            // Envoi d'une requête POST avec un corps JSON
            let request_body = serde_json::json!({
                "addr": self_addr,
            });
            let url = format!("http://{}/api/register", other_addr);
            match client.post(&url).json(&request_body).send().await {
                Ok(response) => {
                    info!("POST Headers: {:?}", response.headers());
                    match response.json::<RegistrationAnswer>().await? {
                        RegistrationAnswer::Registered => {
                            info!("Registered with {other_addr}");
                        }
                        RegistrationAnswer::AlreadyRegistered => {
                            error!("Already registered with {other_addr}");
                        }
                    }
                }
                Err(err) => error!("Failed to send POST request: {:?}", err),
            }
        }
        Ok(())
    })
}

// TODO: add tests
// https://actix.rs/docs/testing/

#[cfg(test)]
mod tests {
    use crate::ChatMessage;

    #[test]
    fn test_serialized_message() {
        let message = ChatMessage::Message("Hello World!".to_string());
        let json = serde_json::to_string(&message).unwrap();
        println!("{}", json);
    }
}
