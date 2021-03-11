// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You
// can obtain one at http://mozilla.org/MPL/2.0/.

use std::fs::OpenOptions;
use std::io::{stdin, stdout, Write};
use rusqlite::Connection;

mod writer;
mod error;

fn main()
{
    let conn = Connection::open("./password.db3");
    println!("{}", conn.is_ok());
    let mut file = match OpenOptions::new().write(true).open(r"/dev/hidg0")
    {
        Ok(f) => f,
        Err(e) => { println!("File open failed: {}\n Try opening with sudo.", e); panic!() } ,
    };
    writer::write_password("abcdefASDAS0[SADUIA{Sd][][;ad", &mut file).expect("works");
}

// Create macro
// Use macro
// Change Macro
// Delete Macro?