use std::net::TcpListener;
use tungstenite::{Message, accept};
use std::thread::spawn;
use std::sync::{Arc, Mutex};

type PeerMap = Arc<Mutex<Vec<tungstenite::WebSocket<std::net::TcpStream>>>>;
pub type Broadcaster = dyn Fn(String) + Send;
fn main() {
    let _x = open_websocket();
    loop {
        // Keep the main thread alive.
        // Maybe handle some main thread tasks here, if there are any.
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
fn open_websocket() -> Box<Broadcaster> {
    let server = TcpListener::bind("localhost:27017")
        .expect("[server] cannot bind to port");
    // Create list to hold peers
    let peers: PeerMap = Arc::new(Mutex::new(Vec::new()));
    
    println!("collecting peers");
    
    // Spawn thread to collect peers (connected clients)
    let peers_clone = peers.clone();
    // Closure to handle new clients
    spawn(move || {
        for stream in server.incoming() {
            //Handle client
            let client = accept(stream.unwrap()).unwrap();
            println!("Registered new client");
            
            // Add a lock to peer
            let mut peers_guard = peers_clone.lock().unwrap();
            
            if peers_guard.len() < 11 {
                peers_guard.push(client);
            } else {
                println!("Too many clients connected");
                continue;
            }
        }
    });
    
    println!("create closure");
    // Closure to handle messages from clients
    return Box::new(move |msg| {
        let mut peers_guard = peers.lock().unwrap();
        for sock in peers_guard.iter_mut() {
            let out = Message::from(msg.clone());
            if let Err(e) = sock.send(out) {
                println!("Error sending message: {:?}", e);
            } else {
                println!("Sent message to client");
            }
        }
    });
}