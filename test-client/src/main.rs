use json::object;
use rand::Rng;
use std::{thread, time};
use tungstenite::{connect, Message};
use url::Url;

fn main() {
    let (mut socket, _response) = connect(Url::parse("ws://localhost:27017").unwrap())
        .expect("[test-client] cannot connect to socket");
    let rand_player_number = rand::thread_rng().gen_range(1..20);
    loop {
        let mut _x = json::Array::new();
        let mut _y = json::Array::new();
        let mut _health = json::Array::new();
        let mut _team = json::Array::new();
        let mut _dormant = json::Array::new();
        let mut _rotation = json::Array::new();
        let mut _scoped = json::Array::new();
        let mut rng = rand::thread_rng();

        for _i in 0..rand_player_number {
            _x.push(json::from(rng.gen_range(0.0..1000.0)));
            _y.push(json::from(rng.gen_range(0.0..1000.0)));
            _health.push(json::from(rng.gen_range(0.0..100.0)));
            _team.push(json::from(rng.gen_range(0..2)));
            _dormant.push(json::from(0));
            _rotation.push(json::from(rng.gen_range(0.0..360.0)));
            _scoped.push(json::from(rng.gen_range(0..2)));
        }

        let data = object! {
            "x": _x,
            "y": _y,
            "health": _health,
            "team": _team,
            "dormant": _dormant,
            "rotation": _rotation,
            "scoped": _scoped,
        };

        socket.write_message(Message::from(data.dump())).unwrap();

        thread::sleep(time::Duration::from_millis(1000));
    }
}
