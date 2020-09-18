use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug, Clone)]
pub struct  Channel {
   pub url:String,
   pub name:Option<String>,
   pub logo:Option<String>,
}
#[derive(Debug)]
pub struct Parser{
    path:String,
    file:File,
    channels:Vec<Channel>,
}

impl Parser {
    pub fn new(path:String) -> Self {
        let file = File::open(&path).unwrap();
        
        Self{
            path:path,
            file:file, 
            channels:Vec::new()
        }
    }

    pub fn parse(&mut self) -> Vec<Channel> {
        let reader = BufReader::new(&self.file);
        let mut infos:Vec<(Option<String>, Option<String>)> = Vec::new();
        let mut urls:Vec<String> = Vec::new();
        reader.lines().for_each(|line|{
            let l = line.unwrap();
            if l.contains("#EXTINF:"){
                let (logo, name) = Parser::parse_ext(l);
                infos.push((logo,name))
            }else if l.starts_with("http"){
                urls.push(l)
            }
        });

        
        
        let urls_iter = urls.clone();
        infos.into_iter().enumerate().for_each(|(i,info)|{
            let ustr = String::from(&urls_iter[i]);
            self.channels.push(Channel{
                url:ustr,
                name:info.0,
                logo:info.1,
            })
        });
       
        return self.channels.clone();
        
    }

    fn parse_ext(line:String) -> (Option<String>, Option<String>){
        let name:Option<String>;
        let logo:Option<String>;

        let name_split:Vec<&str> = line.split(",").collect();
        name = Some(name_split[1].to_string());
        
        let mut logo_split:Vec<&str> = line.split("tvg-logo=").collect();
        if logo_split.len() > 1{
            logo_split = logo_split[1].split(" ").collect();
            logo = Some(logo_split[0].replace("\"","").to_string());
        }else{
            logo = None;
        }

        (name, logo)
    }
}