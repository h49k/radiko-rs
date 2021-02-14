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

use m3u8_rs::playlist::Playlist;
use radiko_rs::{token::Token,
                opt::Opts,
            };

fn main() {
    let opts = Opts::get();
    let station = opts.station();

    let auth1_response = match Token::auth1() {
        Ok(r) => r,
        _ => return,
    };

    let token = Token::gen_token(auth1_response);
    token.auth2();

    let h_resp = match token.get_playlist_text(station) {
        Ok(r) => r,
        _ => return,
    };

    let ux = match h_resp.into_string() {
        Ok(s) => s,
        _ => return,
    };

    let m3u8 = m3u8_rs::parse_playlist_res(ux.as_bytes());

    let pl = match m3u8 {
        Ok(Playlist::MasterPlaylist(p)) => p,
        Ok(Playlist::MediaPlaylist(_)) => return,
        Err(_) => return,
    };

    let mut vs = pl.variants;

    let st = match vs.pop() {
        Some(s) => s,
        None => return,
    };

    token.exec_ffplay(st.uri);
}
