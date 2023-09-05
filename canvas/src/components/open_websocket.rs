use std::net::TcpListener;
use tungstenite::{Message, accept};
use std::thread::spawn;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type PeerMap = Arc<Mutex<HashMap<u8, tungstenite::WebSocket<std::net::TcpStream>>>>;
pub type Broadcaster = dyn Fn(String) + Send;

pub fn open_websocket() -> Box<Broadcaster> {
    let server = TcpListener::bind("localhost:27017").unwrap();
    // Create list to hold peers
    let peers: PeerMap = Arc::new(Mutex::new(HashMap::new()));
    
    println!("collecting peers");
    
    // Spawn thread to collect peers (connected clients)
    let peers_clone = peers.clone();
    // Closure to handle new clients
    let mut next_id: u8 = 0;
    spawn(move || {
        for stream in server.incoming() {
            //Handle client
            let client = accept(stream.unwrap()).unwrap();
            println!("Registered new client");
            peers_clone.lock().unwrap().insert(next_id, client);
            next_id += 1;
        }
    });
    println!("create closure");
    // Closure to handle messages from clients
    // Boxing the closure allows it to be moved on the heap
    // Removing need to know size at compile time
    return Box::new(move |msg| {
        for sock in peers.lock().unwrap().iter_mut() {
            let msg1 = msg.clone();
            let out = Message::from(msg1);
            sock.1.send(out).unwrap();
            println!("Sent message to client");
        }
    });
}