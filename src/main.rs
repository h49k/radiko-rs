// use ureq;
use radiko_rs::auth::*;
// use std::{convert::TryInto};
// use substring::Substring;
// use base64;
use subprocess;


fn main() {
    // let auth_key = "bcd151073c03b352e1ef2fd66c32209da9ca0afa";
    // println!("Hello, world!");
    if let Ok(s) = auth1() {
        let token = partial_key(s);
        token.auth2();
        // let url = token.gen_temp_chunk_m3u8_url("FMT");
        if let Some(url) =  token.gen_temp_chunk_m3u8_url("FMT") {
            println!("{}", url);

            let cmd = format!("ffplay -nodisp -loglevel quiet -headers 'X-Radiko-Authtoken: {}' -i '{}'",
                                     token.authtoken ,url);
                                    
            println!("{}", cmd);
            subprocess::Exec::shell("ps");

        }
        // let url = geturl(String::from("FMT"));
        // println!("{}", url);

        // let auth_token = s.header("x-radiko-authtoken").unwrap().to_string();
        // let keylength: usize = s.header("x-radiko-keylength").unwrap().to_string().parse().unwrap();
        // let keyoffset: usize = s.header("x-radiko-keyoffset").unwrap().to_string().parse().unwrap();
    
        // println!("{}\n{}\n{}", auth_token, keylength, keyoffset);
        // // let slice = &auth_token[keyoffset..keylength];
        // let slice = auth_key.substring(keyoffset, keyoffset + keylength,);
        // println!("{}", slice);
        // let b = base64::encode(slice);
        // println!("{}",b);

    }

    // println!("{}", s);


    // let token = Token {
    //     auth_token: String::from(s.header("x-radiko-authtoken")),
    //     keylength: s.header("x-radiko-keylength"),
    //     keyoffset: s.header("x-radiko-keyoffset"),
    // }
    // let h = (s.header("x-radiko-authtoken"),
    // s.header("x-radiko-keylength"),
    // s.header("x-radiko-keyoffset"));

    // for n in h.into_iter() {
    //     match n {
    //         Some(n) => println!("{}", n),
    //         None => println!("Sorry!"),
    //     }
    // }
}
