use std::process::{Command, Stdio, Child};
use std::io::{BufRead, BufReader};
// use crate::m3uparser::Channel;
use crate::lib::channel::{Channel};
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
        mpv_builder.set_option("vo","opengl").unwrap();
        mpv_builder.set_option("fs","yes").unwrap();
        mpv_builder.set_option("ytdl","yes").unwrap();
        mpv_builder.set_option("geometry","1920x1080").unwrap();
        mpv_builder.set_option("vd-lavc-threads","4").unwrap();
        mpv_builder.set_option("ontop","no").unwrap();
        mpv_builder.set_option("force-window","yes").unwrap();
        mpv_builder.set_option("osc","yes").unwrap();
        mpv_builder.set_option("hwdec", "rpi-copy").unwrap();
        mpv_builder.set_option("scale","ewa_lanczossharp").unwrap();
        mpv_builder.set_option("cscale","ewa_lanczossharp").unwrap();
        mpv_builder.set_option("dscale","mitchell").unwrap();
        mpv_builder.set_option("dither-depth","auto").unwrap();
        mpv_builder.set_option("correct-downscaling","yes").unwrap();
        // mpv_builder.set_option("linear-downscaling","yes").unwrap();
        mpv_builder.set_option("sigmoid-upscaling","yes").unwrap();
        mpv_builder.set_option("deband","yes").unwrap();
        mpv_builder.set_option("gpu-api","opengl").unwrap();
 
        match mpv_builder.set_option("load-stats-overlay","yes"){
            Ok(r) => {println!("{:?}",r)},
            Err(e) => {println!("{:?}",e)}
        }

        let mpv = mpv_builder.build().unwrap();

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

        self.mpv.command(&["loadfile",&channel.url]).unwrap();
        self.mpv.command(&["show-text",&channel.title]).unwrap();

        // {"command":["external-file",""]}
        Player::notify(channel);
    }

    pub fn play_prev(&mut self,channel:&Channel){
        self.mpv.command(&["loadfile",&channel.url]).unwrap();
        self.mpv.command(&["show-text",&channel.title]).unwrap();
       

        Player::notify(channel);
     }

    fn notify(channel:&Channel){
        Command::new("notify-send")
        .arg(&channel.title)
        .arg("-u")
        .arg("critical")
        .arg("-i")
        .arg(&channel.img_url)
        .output()
        .expect("");
     }
}