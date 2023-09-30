use json::object;
use rand::Rng;
use std::{thread, time};
use tungstenite::{connect, Message};

fn main() {
    loop {
        match connect("ws://localhost:27017") {
            Ok((mut socket, _response)) => loop {
                let rand_player_number = rand::thread_rng().gen_range(1..=10);
                let mut _id = json::Array::new();
                let mut _x = json::Array::new();
                let mut _y = json::Array::new();
                let mut _health = json::Array::new();
                let mut _team = json::Array::new();
                let mut _dormant = json::Array::new();
                let mut _rotation = json::Array::new();
                let mut _scoped = json::Array::new();
                let mut _weapon = json::Array::new();
                let mut _kill = json::Array::new();
                let mut _death = json::Array::new();
                let mut _assist = json::Array::new();
                let mut _acs = json::Array::new();
                let mut _shield = json::Array::new();
                let mut _credits = json::Array::new();

                let mut rng = rand::thread_rng();

                for _i in 0..rand_player_number {
                    _id.push(json::from(rng.gen_range(0..22)));
                    _x.push(json::from(rng.gen_range(0.0..1000.0)));
                    _y.push(json::from(rng.gen_range(0.0..1000.0)));
                    _health.push(json::from(rng.gen_range(0.0..100.0)));
                    _team.push(json::from(rng.gen_range(0..2)));
                    _dormant.push(json::from(rng.gen_range(0..2)));
                    _rotation.push(json::from(rng.gen_range(0.0..360.0)));
                    _scoped.push(json::from(rng.gen_range(0..2)));
                    _weapon.push(json::from(rng.gen_range(0..100)));
                    _kill.push(json::from(rng.gen_range(0..30)));
                    _death.push(json::from(rng.gen_range(0..30)));
                    _assist.push(json::from(rng.gen_range(0..30)));
                    _acs.push(json::from(rng.gen_range(0..400)));
                    _shield.push(json::from(rng.gen_range(0..50)));
                    _credits.push(json::from(rng.gen_range(0..16000)));
                }

                let players = object! {
                "id": _id,
                "x": _x,
                "y": _y,
                "health": _health,
                "team": _team,
                "dormant": _dormant,
                "rotation": _rotation,
                "scoped": _scoped,
                "weapon": _weapon,
                "kill": _kill,
                "death": _death,
                "assist": _assist,
                "acs": _acs,
                "shield": _shield,
                "credits": _credits,
                };

                let _max_rounds = rng.gen_range(0..25);

                let mut _t_score = json::Array::new();
                let mut _ct_score = json::Array::new();
                let mut _spike_x = json::Array::new();
                let mut _spike_y = json::Array::new();
                let mut _spike_timer = json::Array::new();
                let mut _defusal_time = json::Array::new();
                let mut _round_win_status = json::Array::new();
                let mut _round_timer = json::Array::new();

                _t_score.push(json::from(rng.gen_range(0..16)));
                _ct_score.push(json::from(rng.gen_range(0..16)));
                _spike_x.push(json::from(rng.gen_range(0.0..1000.0)));
                _spike_y.push(json::from(rng.gen_range(0.0..1000.0)));
                _spike_timer.push(json::from(rng.gen_range(0.0..35.0)));
                _defusal_time.push(json::from(rng.gen_range(0.0..8.0)));
                _round_timer.push(json::from(rng.gen_range(0.0..150.0)));

                for _i in 0.._max_rounds {
                    _round_win_status.push(json::from(rng.gen_range(0..2)));
                }

                let game_info = object! {
                "t_score": _t_score,
                "ct_score": _ct_score,
                "spike_x": _spike_x,
                "spike_y": _spike_y,
                "spike_timer": _spike_timer,
                "defusal_time": _defusal_time,
                "round_win_status": _round_win_status,
                "max_rounds": _max_rounds,
                "round_timer" : _round_timer,
                };

                let data = object! {
                "players": players,
                "game_info": game_info,
                };

                match socket.write_message(Message::from(data.dump())) {
                    Ok(_) => {
                        thread::sleep(time::Duration::from_millis(1000));
                    }
                    Err(_) => {
                        eprintln!("Error sending message. Attempting to reconnect...");
                        break;
                    }
                }
            },
            Err(e) => {
                eprintln!("Error connecting to server: {}", e);
                thread::sleep(time::Duration::from_millis(5000));
            }
        }
    }
}
