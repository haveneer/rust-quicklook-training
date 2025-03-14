mod options;

use actix_web::web::Data;
use actix_web::{
    error, get, middleware, post, web, App, Error, HttpResponse, HttpServer, Responder,
};
use clap::Parser;
use futures::StreamExt;
use options::Options;
use serde::{Deserialize, Serialize};
use std::sync::mpsc::SyncSender;
use std::sync::{mpsc, RwLock};
use tracing::{error, info, warn};

const MAX_PAYLOAD_SIZE: usize = 256 * 1024; // max payload size is 256k
const MAX_MSPC_MESSAGE: usize = 1024;

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    username: String,
}

struct CentralData {
    counter: usize,
}

#[derive(Debug)]
struct MPSCMessage {
    message: String,
}

// curl -X GET http://localhost:port/api/hello/john

#[get("/api/hello/{name}")]
async fn hello(
    name: web::Path<String>,
    data: Data<RwLock<(CentralData, SyncSender<MPSCMessage>)>>,
) -> impl Responder {
    // Never fail
    let mut data = data.write().unwrap();
    data.0.counter += 1;
    info!(
        "Hello Request received (count={count})",
        count = data.0.counter
    );
    data.1
        .send(MPSCMessage {
            message: "hello".to_string(),
        })
        .expect("Cannot send message");
    format!("Hello {name}!")
}

#[post("/api/json")]
async fn echo(
    mut payload: web::Payload,
    data: Data<RwLock<(CentralData, SyncSender<MPSCMessage>)>>,
) -> Result<HttpResponse, Error> {
    // May fail
    {
        // minimize lock timetime
        let data = data.read().unwrap();
        info!("JSON Request received");
        data.1
            .send(MPSCMessage {
                message: "echo".to_string(),
            })
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
    let mut obj = serde_json::from_slice::<Message>(&body)?;
    obj.username = obj.username.to_uppercase();
    Ok(HttpResponse::Ok().json(obj)) // <- send response
}

// curl -X POST -d '{"username": "john"}' -H "Content-type: application/json" http://localhost:7878/api/json

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let options = Options::parse();

    // Configure RUST_LOG environment variable like
    // RUST_LOG=debug
    // RUST_LOG=gossip_server=debug
    // env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    // console_subscriber::init();
    tracing_subscriber::fmt::init();
    let entry_points = options.entry_points().clone();
    // tokio::spawn(async move {
    println!("entry_points: {entry_points:?}");
    register(&entry_points).await?;
    // });

    info!("Server started");

    let (tx, rx) = mpsc::sync_channel::<MPSCMessage>(MAX_MSPC_MESSAGE);

    let data = Data::new(RwLock::new((CentralData { counter: 0 }, tx)));

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&data))
            .wrap(middleware::Logger::default())
            .service(hello) // alternative form: .route("/api/hello/{name}", web::get().to(echo));
            .service(echo) // alternative form: .route("/api/json", web::post().to(echo));
    })
    .bind(("127.0.0.1", options.port()))?
    .run();

    tokio::spawn(async move {
        forever_wait().await;
    });

    tokio::spawn(async move {
        while let Ok(m) = rx.recv() {
            warn!("Message from {}", m.message);
        }
    });

    server.await?;
    Ok(())
}

async fn forever_wait() {
    loop {
        tokio::time::sleep(tokio::time::Duration::from_millis(10000)).await;
        info!("Waiting");
    }
}

async fn register(addrs: &Vec<String>) -> anyhow::Result<()> {
    // Création d'un client HTTP reqwest
    let client = reqwest::Client::new();
    for addr in addrs {
        // Conversion de la chaîne en SocketAddr
        let addr: std::net::SocketAddr = addr.parse()?;
        info!("Connecting to {:?}", addr);

        // Envoi d'une requête GET
        let message = "John";
        let url = format!("http://{}/api/hello/{}", addr, message);
        match client.get(&url).send().await {
            Ok(response) => info!("GET Response: {:?}", response),
            Err(err) => error!("Failed to send GET request: {:?}", err),
        }

        // Envoi d'une requête POST avec un corps JSON
        let request_body = serde_json::json!({
            "username": "john",
        });
        let url = format!("http://{}/api/json", addr);
        match client.post(&url)
            .json(&request_body)
            .send().await {
            Ok(mut response) => {
                info!("POST Headers: {:?}", response.headers());
                let val = response.json::<Message>().await?;
                info!("POST Response: {:?}", val);
            }
            Err(err) => error!("Failed to send POST request: {:?}", err),
        }
    }
    Ok(())
}

// TODO: add tests
// https://actix.rs/docs/testing/
