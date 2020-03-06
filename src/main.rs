use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::io::{copy, Read, Write};
use std::collections::HashMap;
use m3u::{Reader, Entries, Entry};
mod m3uparser;
use m3uparser::Channel;
fn main() {
//   start_remote();
    
  
    let mut is_playing:bool = false;
    let mut index = 0;
   
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
  

    reader
    .lines()
    .filter_map(|line| line.ok())
    .for_each(|line| {
        if(has_keypress(&line)){
            let mut split:Vec<&str> = line.rsplit_terminator("key pressed:").collect();
            split = split[0].split("(").collect();
            let command = split[0].trim();
            println!("IS PLAYING:{}",is_playing);
            println!("COMMAND:{}",command);
 
            match command {
                "data" => {
                    if is_playing {
                        stop_play();
                        // index = 0;
                        is_playing = false;
                    }else{
                        play_channel(&channels[0]);
                        is_playing = true;
                    }
                }
                "channel up" => {
                     index += 1;
                     play_channel(&channels[index]);
                     
                 },
                 "channel down"=>{
                    index -= 1;
                    play_channel(&channels[index]);
                }
                 _ => {
 
                 }
            }
        }
    });

    
}

fn play_channel(channel:&Channel){

    let name = &channel.name;
    let logo = match &channel.logo {
        Some(url) => url,
        None => ""
    };
    
    let not = Command::new("kodi-send")
    .arg("-a")
    .arg(format!("Notification(,{},10000,{})", name.as_ref().unwrap(),logo))
    .output()
    .unwrap();

    let output = Command::new("kodi-send")
    .arg("-a")
    .arg(format!("PlayMedia({})", channel.url))
    .output()
    .unwrap();

    println!("{:?}", output);
}

fn start_play(url:String){
    let output = Command::new("kodi-send")
    .arg("-a")
    .arg(format!("PlayMedia({})", url))
    .output()
    .unwrap();

    println!("{:?}", output);
}

fn stop_play(){
    let output = Command::new("kodi-send")
    .arg("-a")
    .arg("PlayerControl(Stop)")
    .output()
    .unwrap();

    println!("{:?}", output);
}


fn has_keypress(line:&String)-> bool {
    line.contains("key pressed:") &&
    line.contains("duration")
}