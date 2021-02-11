use ureq::{self, Response};
use substring::Substring;
use regex::Regex;

#[derive(Debug)]
pub struct Token {
    partialkey: String,
    pub authtoken: String,
}

impl Token {
    pub fn new(partialkey: String, authtoken: String) -> Self { Self { partialkey, authtoken } }
}

impl Token {
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

    pub fn gen_temp_chunk_m3u8_url(&self, station: &str) -> Option<String>{
        let re = Regex::new(r"https?://.+m3u8").unwrap();
        let url = geturl(String::from(station));
        let resp = ureq::get(&url)
            .set("X-Radiko-AuthToken", &self.authtoken)
            .call();
        
        if let Ok(r) = resp {
            let txt = r.into_string().unwrap();
            let m3u8_url = re.captures(&txt);

            if let Some(u) = m3u8_url {
                let text = u.get(0).map_or("", |m| m.as_str());

                return Some(String::from(text))
            }
        }
        None
    }
}

fn geturl(station: String) -> String {
    format!("http://f-radiko.smartstream.ne.jp/{}/_definst_/simul-stream.stream/playlist.m3u8", station)
}


pub fn auth1() -> Result<ureq::Response, ureq::Error> {
    let url = "https://radiko.jp/v2/api/auth1";

    let resp = ureq::get(url)
        .set("User-Agent", "curl/7.56.1")
        .set("Accept", "*/*")
        .set("X-Radiko-App","pc_html5")
        .set("X-Radiko-App-Version", "0.0.1")
        .set("X-Radiko-User", "dummy_user")
        .set("X-Radiko-Device", "pc")
        .call();

    resp
}

pub fn partial_key(resp: Response) -> Token {
    let auth_key = "bcd151073c03b352e1ef2fd66c32209da9ca0afa";

    let auth_token = resp.header("x-radiko-authtoken").unwrap().to_string();
    let keylength: usize = resp.header("x-radiko-keylength").unwrap().to_string().parse().unwrap();
    let keyoffset: usize = resp.header("x-radiko-keyoffset").unwrap().to_string().parse().unwrap();

    let slice = auth_key.substring(keyoffset, keyoffset + keylength,);

    let b = base64::encode(slice);

    Token::new(b, auth_token)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth1() {
    }

}