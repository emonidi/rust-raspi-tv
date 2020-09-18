use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
// mod m3uparser;
// use m3uparser::Channel;
mod lib;
mod player;
use lib::scraper::NScraper;
use player::Player;
use std::env::vars;

fn main() {
    let mut user = String::new();
    let mut pass = String::new();

    for (key, value) in vars() {
        if key == "N_USER" {
            user = String::from(&value)
        }
        if key == "N_PASS" {
            pass = String::from(&value)
        };
    }

    Command::new("notify-send")
        .arg("DOWNLOADING CHANNELS")
        .spawn()
        .expect("");

    let mut is_playing: bool = false;
    let mut index = 0;
    let mut player: Option<Player> = None;
    let output = Command::new("cec-client")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .stdout
        .ok_or_else(|| "Could not capture standard output.")
        .unwrap();

    let reader = BufReader::new(output);
    let mut lines = reader.lines();

    let scraper = NScraper::new();
    scraper.sign_in(user, pass);
    let channels = scraper.get_live_channels();

    Command::new("notify-send")
        .arg("CHANNELS DOWNLOADED")
        .spawn()
        .expect("");

    // let mut parser = m3uparser::Parser::new(String::from("res/playlist.m3u"));
    // let channels = parser.parse();

    loop {
        let line = lines.next().unwrap().unwrap();
        if (has_keypress(&line)) {
            let mut split: Vec<&str> = line.rsplit_terminator("key pressed:").collect();
            split = split[0].split("(").collect();
            let command = split[0].trim();
            println!("IS PLAYING:{}", is_playing);
            println!("COMMAND:{}", command);
            match command {
                "data" => {
                    if !is_playing {
                        player = Some(Player::new());
                        match &mut player {
                            Some(player) => {
                                println!("{:#?}", &channels[index]);
                                println!("PLAY_NEXT");
                                player.play_next(&channels[index]);
                                index = 0;
                                is_playing = true;
                            }
                            None => {}
                        }
                    } else {
                        match &mut player {
                            Some(player) => {
                                player.quit();
                                index = 0;
                            }
                            None => {}
                        }
                        player = None;
                        is_playing = false;
                    }
                }

                "channel up" => {
                    if index < channels.len() - 1 {
                        match &mut player {
                            Some(player) => {
                                println!("{:#?}", &channels[index]);
                                println!("PLAY_NEXT");
                                player.play_next(&channels[index]);
                                index += 1;
                            }
                            None => {}
                        }
                    }
                }

                "channel down" => {
                    if index > 0 {
                        match &mut player {
                            Some(player) => {
                                println!("PLAY_NEXT");
                                player.play_prev(&channels[index]);
                                index -= 1;
                            }
                            None => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

fn has_keypress(line: &String) -> bool {
    line.contains("key pressed:") && line.contains("duration")
}
