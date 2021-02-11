use radiko_rs::auth::*;
use subprocess::{self};


fn main() {
    if let Ok(s) = auth1() {
        let token = partial_key(s);
        token.auth2();

        if let Some(url) =  token.gen_temp_chunk_m3u8_url("FMT") {
            println!("{}", url);

            let cmd = format!("ffplay -nodisp -loglevel quiet -headers 'X-Radiko-Authtoken: {}' -i '{}'",
                                     token.authtoken ,url);
                                    
            println!("{}", cmd);
            let shell = subprocess::Exec::shell(cmd);

            let _ = shell.popen();
        }
    }
}
