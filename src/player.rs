use std::process::{Command, Stdio, Child};
use std::io::{BufRead, BufReader};
use crate::m3uparser::Channel;

#[derive(Debug)]
pub struct Player{
    process: Child,
    // process: Option<std::process::Child>
}

impl Player {
    pub fn new() -> Self{
        
        let com = Command::new("mpv")
        .env("DISPLAY",":0")
        .arg("-fs")
        .arg("-vo=gpu")
        .arg("--prefetch-playlist")
        .arg("--playlist=res/playlist.m3u")
        // .arg("--no-terminal")
        // .arg("--pause")
        .stdin(Stdio::piped())
        // .stdout(Stdio::piped())
        .spawn()
        .unwrap();


        Command::new("notify-send")
            .arg("Starting TV")
            .spawn()
            .expect("");

        Self{
           process:com
        }
    }

    pub fn quit(&mut self){
        self.process.kill().expect("");
    }

    pub fn play_next(&mut self, channel:&Channel){
       Command::new("xdotool")
        .env("DISPLAY",":0")
        .arg("key")
        .arg("greater")
        .spawn()
        .expect("");

        Player::notify(channel);
    }

    pub fn play_prev(&mut self,channel:&Channel){
        Command::new("xdotool")
         .env("DISPLAY",":0")
         .arg("key")
         .arg("less")
         .spawn()
         .expect("");

         Player::notify(channel);
     }

    fn notify(channel:&Channel){
        let name = match &channel.name {
            Some(name) => String::from(name), 
            None => String::from("")
        };

        let icon = match &channel.logo {
            Some(logo) => String::from(logo),
            None => String::from("")
        };

        Command::new("notify-send")
        .arg(name)
        .arg("-u")
        .arg("critical")
        .arg("-i")
        .arg(icon)
        .output()
        .expect("");
     }
}