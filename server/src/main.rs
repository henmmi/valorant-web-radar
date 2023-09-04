use std::net::TcpListener;
use tungstenite::accept;
use std::thread::spawn;

fn main() {
    let server = TcpListener::bind("localhost:27017").unwrap();
    
    let mut client_2: Option<tungstenite::WebSocket<std::net::TcpStream>> = None;
    
    for stream in server.incoming() {
        if client_2.is_none() {
            client_2 = Some(accept(stream.unwrap()).unwrap());
            continue;
        }
        
        //Handle client #1
        let mut client_1 = accept(stream.unwrap()).unwrap();
        let client_2_clone = client_2.as_ref().unwrap().try_clone().unwrap();
        
        spawn(move || {
            loop {
                let msg = client_1.read().unwrap();
                
                if msg.is_binary() || msg.is_text() {
                    client_2_clone.write(msg).unwrap();
                    println!("Sent");
                }
            }
        });
    }
}