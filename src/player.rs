use std::process::{Command, Stdio, Child};
use std::io::{BufRead, BufReader};
use crate::m3uparser::Channel;
use std::env::*;
use mpv::*;
// #[derive(Debug)]
pub struct Player{
    // process: Child,
    mpv:MpvHandler
    // process: Option<std::process::Child>
}

impl Player {
    pub fn new() -> Self{
        set_var("DISPLAY",":0");
        let mut mpv_builder = mpv::MpvHandlerBuilder::new().expect("Failed to init MPV builder");
        mpv_builder.try_hardware_decoding().unwrap();
        mpv_builder.set_option("vo","gpu").unwrap();
        mpv_builder.set_option("fs","yes").unwrap();
        // mpv_builder.set_option("force-window","yes").unwrap();
        mpv_builder.set_option("ytdl","yes").unwrap();
        mpv_builder.set_option("ytdl-format","best").unwrap();
        mpv_builder.set_option("ontop","no").unwrap();
        mpv_builder.set_option("player-operation-mode","pseudo-gui").unwrap();
        
        
        match mpv_builder.set_option("load-stats-overlay","yes"){
            Ok(r) => {println!("{:?}",r)},
            Err(e) => {println!("{:?}",e)}
        }

        let mpv = mpv_builder.build().unwrap();

       
        // let com = Command::new("mpv")
        // .env("DISPLAY",":0")
        // .arg("-fs")
        // .arg("-vo=gpu")
        // .arg("--prefetch-playlist")
        // .arg("--playlist=res/playlist.m3u")
        // // .arg("--no-terminal")
        // // .arg("--pause")
        // .stdin(Stdio::piped())
        // // .stdout(Stdio::piped())
        // .spawn()
        // .unwrap();


        Command::new("notify-send")
            .arg("Starting TV")
            .spawn()
            .expect("");

        Self{
           mpv
        }
    }

    pub fn quit(&mut self){
        // self.process.kill().expect("");
    }

    pub fn play_next(&mut self, channel:&Channel){
    //    Command::new("xdotool")
    //     .env("DISPLAY",":0")
    //     .arg("key")
    //     .arg("greater")
    //     .spawn()
    //     .expect("");

        let c1 = self.mpv.command(&["loadfile",&channel.url]).unwrap();
        self.mpv.command(&["show-text",&channel.name.as_ref().unwrap()]);
        

       
        Player::notify(channel);
    }

    pub fn play_prev(&mut self,channel:&Channel){
        let c1 = self.mpv.command(&["loadfile",&channel.url]).unwrap();
        self.mpv.command(&["show-text",&channel.name.as_ref().unwrap()]);
       

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