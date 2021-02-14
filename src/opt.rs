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
use clap::{App, Arg};
pub struct Opts {
    station: String,
}

impl Opts {
    pub fn get() -> Self {
        let matches = App::new("radiko-rs")
                                    .version("0.1.3")
                                    .author("h49k")
                                    .arg(Arg::with_name("station")
                                        .short("s")
                                        .long("station")
                                        .value_name("STATION")
                                        .takes_value(true)
                                        .required(true))
                                    .get_matches();

        let station = matches.value_of("station").unwrap().to_string();
        
        Self {
            station: station,
        }
    }

    pub fn station(&self) -> &str {
        &self.station
    }

}