use json::object;
use rand::Rng;
use std::{thread, time};
use tungstenite::{connect, Message};
use url::Url;

fn main() {
    let (mut socket, _response) = connect(Url::parse("ws://localhost:27017").unwrap())
        .expect("[test-client] cannot connect to socket");

    loop {
        let mut _x = json::Array::new();
        let mut _y = json::Array::new();
        let mut _health = json::Array::new();
        let mut _team = json::Array::new();
        let mut _dormant = json::Array::new();
        let mut rng = rand::thread_rng();

        for _i in 0..10 {
            _x.push(json::from(rng.gen_range(0.0..1000.0)));
            _y.push(json::from(rng.gen_range(0.0..1000.0)));
            _health.push(json::from(rng.gen_range(0.0..100.0)));
            _team.push(json::from(rng.gen_range(0..2)));
            _dormant.push(json::from(0));
        }

        let data = object! {
            "x": _x,
            "y": _y,
            "health": _health,
            "team": _team,
            "dormant": _dormant,
        };

        socket.write_message(Message::from(data.dump())).unwrap();

        thread::sleep(time::Duration::from_millis(1000));
    }
}
