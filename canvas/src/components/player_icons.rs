pub enum Agent {
    Brimstone,
    Viper,
    Omen,
    Killjoy,
    Cypher,
    Sova,
    Sage,
    Phoenix,
    Jett,
    Reyna,
    Raze,
    Breach,
    Skye,
    Yoru,
    Astra,
    Kayo,
    Chamber,
    Neon,
    Fade,
    Harbor,
    Gekko,
    Deadlock,
}

impl Agent {
    pub fn get_agent_name(id: i32) -> &'static str {
        match id {
            0 => "Brimstone",
            1 => "Viper",
            2 => "Omen",
            3 => "Killjoy",
            4 => "Cypher",
            5 => "Sova",
            6 => "Sage",
            7 => "Phoenix",
            8 => "Jett",
            9 => "Reyna",
            10 => "Raze",
            11 => "Breach",
            12 => "Skye",
            13 => "Yoru",
            14 => "Astra",
            15 => "Kayo",
            16 => "Chamber",
            17 => "Neon",
            18 => "Fade",
            19 => "Harbor",
            20 => "Gekko",
            21 => "Deadlock",
            _ => "Unknown",
        }
    }
    pub fn agent_player_icon_url(id: i32) -> String {
        "http://127.0.0.1:8080/images/".to_owned() + Agent::get_agent_name(id) + ".png"
    }
}
