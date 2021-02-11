use radiko_rs::auth::*;
use subprocess::{self};

fn main() {
    let args:Vec<String> = std::env::args().skip(1).collect();
    let station = &args[0];


    if let Ok(s) = auth1() {
        let token = partial_key(s);
        token.auth2();

        if let Some(url) = token.gen_temp_chunk_m3u8_url(station) {
            println!("{}", url);

            let cmd = format!(
                "ffplay -nodisp -loglevel quiet -headers 'X-Radiko-Authtoken: {}' -i '{}'",
                token.authtoken, url
            );

            println!("{}", cmd);
            let shell = subprocess::Exec::shell(cmd);

            let _ = shell.popen();
        }
    }
}
