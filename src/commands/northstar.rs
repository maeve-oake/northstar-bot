use reqwest::Response;
use serenity::all::*;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use std::collections::HashMap;

use crate::MS;

use serenity::all::{CommandDataOption, CreateCommand};

pub async fn status(_options: &[CommandDataOption]) -> String {
    let json = northstar_server_json().await;

    match json {
        Ok(res) => {
            let json = res.text().await.unwrap();
            let json: serde_json::Value = serde_json::from_str(&json).unwrap();

            let mut servers_online = 0;
            let mut password_servers = 0;
            let mut players: f32 = 0.0;
            let mut playersmax: f32 = 0.0;

            while servers_online <= json.as_array().unwrap().len() - 1 {
                let has_pwd = json
                    .get(servers_online)
                    .and_then(|value| value.get("hasPassword"));
                if has_pwd.unwrap().to_string() == "true" {
                    password_servers += 1;
                } else {
                    playersmax += json
                        .get(servers_online)
                        .and_then(|value| value.get("maxPlayers"))
                        .unwrap()
                        .to_string()
                        .parse::<f32>()
                        .unwrap();
                }
                players += json
                    .get(servers_online)
                    .and_then(|value| value.get("playerCount"))
                    .unwrap()
                    .to_string()
                    .parse::<f32>()
                    .unwrap();
                servers_online += 1;
            }
            let percentage: f32 = (players / playersmax * 100.0).ceil();

            "```Diff
## NORTHSTAR.TF STATUS: ##

+ Servers Online: "
                .to_owned()
                + &servers_online.to_string()
                + "

- Password Protected Servers: "
                + &password_servers.to_string()
                + "

+ Players in-game: "
                + &players.to_string()
                + "/"
                + &playersmax.to_string()
                + " ("
                + &percentage.to_string()
                + "%)"
                + "```"
        }

        Err(_e) => {
            panic!("oh nyo it failed")
        }
    }
}

async fn northstar_server_json() -> Result<Response, reqwest::Error> {
    let client = reqwest::Client::builder().build().unwrap();
    let url = MS.to_owned() + "/client/servers";
    let res = client
        .get(url)
        .header(
            "user-agent",
            "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.2; .NET CLR 1.0.3705;)",
        )
        .send()
        .await;

    return res;
}

pub async fn search(options: &[CommandDataOption]) -> String {
    let json = northstar_server_json().await;

    match json {
        Ok(res) => {
            let mut searchtype = "name";
            let mut search = "";

            let kind = options
                .iter()
                .find(|option| option.name == "type")
                .and_then(|option| match &option.value {
                    CommandDataOptionValue::String(value) => Some(value.as_str()),
                    _ => None,
                })
                .unwrap_or("title");

            if let Some(value) = options
                .iter()
                .find(|option| option.name == "query")
                .and_then(|option| match &option.value {
                    CommandDataOptionValue::String(value) => Some(value.as_str()),
                    _ => None,
                })
            {
                search = value;
            }

            match kind {
                "mode" | "modes" | "gamemodes" => {
                    if get_playlist_name(search).unwrap() == "no gamemode?" {
                        return "```diff\n- Please specify a valid gamemode.```".to_string();
                    }

                    searchtype = "playlist";
                }

                "map" | "maps" => {
                    if get_map_name(search).unwrap() == "no map?" {
                        return "```diff\n- Please specify a valid map.```".to_string();
                    }

                    searchtype = "map";
                }

                "title" | "name" => {
                    if search == " " {
                        return "```diff\n- Please specify a search term.```".to_string();
                    }

                    searchtype = "name";
                }

                "region" => {
                    if search == " " {
                        return "```diff\n- Please specify a search term.```".to_string();
                    }

                    searchtype = "region";
                }

                _ => {
                    search = kind;
                }
            }

            let json = res.text().await.unwrap();
            let json: serde_json::Value = serde_json::from_str(&json).unwrap();

            let mut lobbies: Vec<&serde_json::Value> = [].to_vec();

            for i in 0..json.as_array().unwrap().len() {
                let jsonsearch = json.get(i).and_then(|value| value.get(searchtype));

                if jsonsearch.is_some()
                    && jsonsearch
                        .unwrap()
                        .to_string()
                        .replace('"', "")
                        .to_ascii_lowercase()
                        .contains(&search.to_ascii_lowercase())
                {
                    if searchtype == "playlist" {
                        if !jsonsearch
                            .unwrap()
                            .to_string()
                            .to_ascii_lowercase()
                            .contains("private_match")
                            || search == "private_match"
                        {
                            lobbies.push(json.get(i).unwrap())
                        }
                    } else {
                        lobbies.push(json.get(i).unwrap())
                    }
                }
            }

            if lobbies.len() == 0 {
                "```diff\n- No servers were found.```".to_string()
            } else {
                let mut searchstring = "```diff\n+ ".to_owned()
                    + &lobbies.len().to_string()
                    + &" servers were found".to_owned();

                if lobbies.len() > 10 {
                    searchstring += " - displaying first 10 results \n";
                }

                for i in 0..lobbies.len() {
                    if i < 10 {
                        let playingtext: String;

                        if lobbies[i]["hasPassword"] == true {
                            playingtext = "- PASSWORD PROTECTED!\n".to_string()
                        } else if lobbies[i]["map"].to_string().replace('"', "") == "mp_lobby" {
                            playingtext = "- Currently in the lobby\n".to_string()
                        } else {
                            playingtext = "+ ".to_owned()
                                + &"Playing "
                                + get_playlist_name(
                                    &lobbies[i]
                                        .get("playlist")
                                        .unwrap()
                                        .to_string()
                                        .replace('"', ""),
                                )
                                .unwrap()
                                + " on "
                                + &get_map_name(
                                    &lobbies[i].get("map").unwrap().to_string().replace('"', ""),
                                )
                                .unwrap()
                                + "\n";
                        }

                        if lobbies[i].get("playerCount") == lobbies[i].get("maxPlayers") {
                            searchstring += &("\n".to_owned()
                                + &(lobbies[i]
                                    .get("name")
                                    .unwrap()
                                    .to_string()
                                    .replace('"', "")
                                    .replace("`", "")
                                    + "\n"
                                    + "- "
                                    + &lobbies[i].get("playerCount").unwrap().to_string()
                                    + &" / ".to_string()
                                    + &lobbies[i].get("maxPlayers").unwrap().to_string()
                                    + " players connected"
                                    + "\n"
                                    + &playingtext));
                        } else {
                            searchstring += &("\n".to_owned()
                                + &(lobbies[i]
                                    .get("name")
                                    .unwrap()
                                    .to_string()
                                    .replace('"', "")
                                    .replace("`", "")
                                    + "\n"
                                    + "+ "
                                    + &lobbies[i].get("playerCount").unwrap().to_string()
                                    + &" / ".to_string()
                                    + &lobbies[i].get("maxPlayers").unwrap().to_string()
                                    + " players connected"
                                    + "\n"
                                    + &playingtext));
                        }
                    }
                }

                searchstring + "```"
            }
        }

        Err(_) => {
            panic!("oh nyo it failed")
        }
    }
}

pub fn get_map_name(name: &str) -> Option<&str> {
    let maps = HashMap::from([
        //mp maps
        ("mp_angel_city", "Angel City"),
        ("mp_black_water_canal", "Black Water Canal"),
        ("mp_grave", "Boomtown"),
        ("mp_colony02", "Colony"),
        ("mp_complex3", "Complex"),
        ("mp_crashsite3", "Crashsite"),
        ("mp_drydock", "DryDock"),
        ("mp_eden", "Eden"),
        ("mp_thaw", "Exoplanet"),
        ("mp_forwardbase_kodai", "Forward Base Kodai"),
        ("mp_glitch", "Glitch"),
        ("mp_homestead", "Homestead"),
        ("mp_relic02", "Relic"),
        ("mp_rise", "Rise"),
        ("mp_wargames", "Wargames"),
        ("mp_lobby", "Lobby"),
        ("mp_box", "Box"),
        //lf maps
        ("mp_lf_deck", "Deck"),
        ("mp_lf_meadow", "Meadow"),
        ("mp_lf_stacks", "Stacks"),
        ("mp_lf_township", "Township"),
        ("mp_lf_traffic", "Traffic"),
        ("mp_lf_uma", "UMA"),
        //coliseum
        ("mp_coliseum", "The Coliseum"),
        ("mp_coliseum_column", "Pillars"),
        //campaign
        ("sp_training", "The Pilot's Gauntlet"),
        ("sp_crashsite", "BT-7274"),
        ("sp_sewers1", "Blood and Rust"),
        ("sp_boomtown_start", "Into the Abyss - Part 1"),
        ("sp_boomtown", "Into the Abyss - Part 2"),
        ("sp_boomtown_end", "Into the Abyss - Part 2"),
        ("sp_hub_timeshift", "Effect and Cause - Part 1 or 3"),
        ("sp_timeshift_spoke02", "Effect and Cause - Part 2"),
        ("sp_beacon", "The Beacon - Part 1 or 3"),
        ("sp_beacon_spoke0", "The Beacon - Part 2"),
        ("sp_tday", "Trial by Fire"),
        ("sp_s2s", "The Ark"),
        ("sp_skyway_v1", "The Fold Weapon"),
    ]);
    return Some(maps.get(name).unwrap_or(&"no map?"));
}

pub fn get_playlist_name(name: &str) -> Option<&str> {
    let modes = HashMap::from([
        ("private_match", "Private Match"),
        //vanilla
        ("aitdm", "Attrition"),
        ("at", "Bounty Hunt"),
        ("coliseum", "Coliseum"),
        ("cp", "Amped Hardpoint"),
        ("ctf", "Capture the Flag"),
        ("fd", "Frontier Defense"),
        ("fd_easy", "Frontier Defense (Easy)"),
        ("fd_normal", "Frontier Defense (Regular)"),
        ("fd_hard", "Frontier Defense (Hard)"),
        ("fd_insane", "Frontier Defense (Insane)"),
        ("fd_master", "Frontier Defense (Master)"),
        ("lts", "Last Titan Standing"),
        ("mfd", "Marked For Death"),
        ("ps", "Pilots vs. Pilots"),
        ("solo", "Campaign"),
        ("tdm", "Skirmish"),
        ("ttdm", "Titan Brawl"),
        ("lf", "Live Fire"),
        //vanilla featured
        ("alts", "Aegis Last Titan Standing"),
        ("attdm", "Aegis Titan Brawl"),
        ("ffa", "Free For All"),
        ("fra", "Free Agents"),
        ("holopilot_lf", "The Great Bamboozle"),
        ("rocket_lf", "Rocket Arena"),
        ("turbo_lts", "Turbo Last Titan Standing"),
        ("turbo_ttdm", "Turbo Titan Brawl"),
        //northstar custom
        ("chamber", "One in the Chamber"),
        ("ctf_comp", "Competitive CTF"),
        ("fastball", "Fastball"),
        ("gg", "Gun Game"),
        ("hidden", "The Hidden"),
        ("hs", "Hide and Seek"),
        ("inf", "Infection"),
        ("kr", "Amped Killrace"),
        ("sns", "Sticks and Stones"),
        ("tffa", "Titan FFA"),
        ("tt", "Titan Tag"),
        ("fw", "Frontier War"),
        //northstar coop
        ("sp_coop", "Singleplayer Coop"),
    ]);
    return Some(modes.get(name).unwrap_or(&"no gamemode?"));
}

pub fn register() -> CreateCommand {
    CreateCommand::new("status").description("a general overview of northstar.tf");
    CreateCommand::new("search")
        .description("search Northstar servers")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "type", "what to search")
                .required(true)
                .add_string_choice("title", "title")
                .add_string_choice("map", "map")
                .add_string_choice("mode", "mode")
                .add_string_choice("region", "region"),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "query", "search query")
                .required(true),
        )
}
