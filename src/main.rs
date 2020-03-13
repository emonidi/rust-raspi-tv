use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
mod m3uparser;
use m3uparser::Channel;
mod player;
use player::Player;

fn main() {
    
    let mut is_playing:bool = false;
    let mut index = 0;
    let mut  player:Option<Player> = None;
    let output = Command::new("cec-client")
    .stdout(Stdio::piped())
    .spawn()
    .unwrap()
    .stdout
    .ok_or_else(|| "Could not capture standard output.")
    .unwrap();

    let reader = BufReader::new(output);

    let mut parser = m3uparser::Parser::new(String::from("res/playlist.m3u"));
    let channels = parser.parse();
    let mut lines = reader.lines();
    
    loop {
        let line = lines.next().unwrap().unwrap();
       
        if(has_keypress(&line)){
            let mut split:Vec<&str> = line.rsplit_terminator("key pressed:").collect();
            split = split[0].split("(").collect();
            let command = split[0].trim();
            println!("IS PLAYING:{}",is_playing);
            println!("COMMAND:{}",command);
            
            match command {
                "data" => {
                    if !is_playing{
                        player = Some(Player::new());
                        match &mut player{
                            Some(player) => {
                                println!("{:#?}", &channels[index]);
                                println!("PLAY_NEXT");
                                player.play_next(&channels[index]);
                                index +=1;
                            },
                            None => {}
                        }
                        is_playing = true;
                        index+=1;
                    }else{
                       match &mut player{
                           Some(player) => {
                                player.quit();
                                index = 0;
                           }
                           None => {

                           }
                       }
                       player = None;
                       is_playing = false;
                    }
                }

                "channel up" => {
                    if index < channels.len() - 1 {
                        match &mut player{
                            Some(player) => {
                                println!("{:#?}", &channels[index]);
                                println!("PLAY_NEXT");
                                player.play_next(&channels[index]);
                                index +=1;
                            },
                            None => {}
                        }
                    }
                }

                "channel down" => {
                    if index > 0 {
                        match &mut player{
                            Some(player) => {
                                println!("PLAY_NEXT");
                                player.play_prev(&channels[index]);
                                index -=1;
                            },
                            None => {}
                        }
                    }
                }
                _ => {

                }
            }
        }
    }    
}

fn has_keypress(line:&String)-> bool {
    line.contains("key pressed:") &&
    line.contains("duration")
}