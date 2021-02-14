// Copyright (c) h49k 2021
//
// Permission to use, copy, modify, and distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

// use regex::Regex;
use substring::Substring;
use ureq::{self, Response};

#[derive(Debug)]
pub struct Token {
    partialkey: String,
    pub authtoken: String,
}

impl Token {
    /// Constructs a new Token.
    /// Called by partial_key method.
    fn new(partialkey: String, authtoken: String) -> Self {
        Self {
            partialkey,
            authtoken,
        }
    }

    /// Constructs a new Token
    pub fn gen_token(auth1_response: Response) -> Self {
        // auth_key from https://radiko.jp/apps/js/playerCommon.js
        let auth_key = "bcd151073c03b352e1ef2fd66c32209da9ca0afa";
    
        // Headers got by auth1(). 
        let auth_token = auth1_response.header("x-radiko-authtoken").unwrap().to_string();
        let keylength: usize = auth1_response
            .header("x-radiko-keylength")
            .unwrap()
            .to_string()
            .parse()
            .unwrap();
        let keyoffset: usize = auth1_response
            .header("x-radiko-keyoffset")
            .unwrap()
            .to_string()
            .parse()
            .unwrap();
    
        let sliced_key = auth_key.substring(keyoffset, keyoffset + keylength);
    
        let sliced_key_b64 = base64::encode(sliced_key);
    
        Token::new(sliced_key_b64, auth_token)
    }

    /// This is second auth process.
    /// Area 
    pub fn auth2(&self) {
        let url = "https://radiko.jp/v2/api/auth2";
        let resp = ureq::get(url)
            .set("X-Radiko-AuthToken", &self.authtoken)
            .set("X-Radiko-Partialkey", &self.partialkey)
            .set("X-Radiko-User", "dummy_user")
            .set("X-Radiko-Device", "pc")
            .call();

        if let Ok(r) = resp {
            print!("AREA:{}", r.into_string().unwrap());
        }
    }

    // pub fn gen_temp_chunk_m3u8_url(&self, station: &str) -> Option<String> {
    //     let re = Regex::new(r"https?://.+m3u8").unwrap();
    //     let url = geturl(String::from(station));
    //     let resp = ureq::get(&url)
    //         .set("X-Radiko-AuthToken", &self.authtoken)
    //         .call();

    //     if let Ok(r) = resp {
    //         let txt = r.into_string().unwrap();
    //         println!("****{:?}", &txt);

    //         let m3u8_url = re.captures(&txt);

    //         if let Some(u) = m3u8_url {
    //             let text = u.get(0).map_or("", |m| m.as_str());

    //             return Some(String::from(text));
    //         }
    //     }
    //     None
    // }

    pub fn get_playlist_text(&self, station: &str) -> Result<Response, ureq::Error>{
        let url = geturl(String::from(station));
        let resp = ureq::get(&url)
            .set("X-Radiko-AuthToken", &self.authtoken)
            .call();
        resp
    }

    pub fn exec_ffplay(&self, uri: String) {
        let cmd = format!(
            "ffplay -nodisp -loglevel quiet -headers 'X-Radiko-Authtoken: {}' -i '{}'",
            self.authtoken, uri
        );

        println!("{}", cmd);
        let shell = subprocess::Exec::shell(cmd);
        let _ = shell.popen();
    }

}


fn geturl(station: String) -> String {
    format!(
        "http://f-radiko.smartstream.ne.jp/{}/_definst_/simul-stream.stream/playlist.m3u8",
        station
    )
}


pub fn auth1() -> Result<Response, ureq::Error> {
    let url = "https://radiko.jp/v2/api/auth1";

    let resp = ureq::get(url)
        .set("User-Agent", "curl/7.56.1")
        .set("Accept", "*/*")
        .set("X-Radiko-App", "pc_html5")
        .set("X-Radiko-App-Version", "0.0.1")
        .set("X-Radiko-User", "dummy_user")
        .set("X-Radiko-Device", "pc")
        .call();

    resp
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geturl_001() {
        assert_eq!(geturl(String::from("FMT")),
            "http://f-radiko.smartstream.ne.jp/FMT/_definst_/simul-stream.stream/playlist.m3u8".to_string())
    }

}