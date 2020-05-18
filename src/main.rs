extern crate lettre;

#[macro_use]
extern crate log;
#[macro_use]
extern crate structopt;

use structopt::StructOpt;
use lettre::{Transport, SmtpClient};

use lettre_email::*;

use std::io;
use std::io::prelude::*;

/// Send a simple mail from stdin
///
/// To obtain logging information, redirect stdout to file.
///
/// INFO is the default.
/// e.g.
/// RUST_LOG=mailer=info ./mailer
#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(short = "f", default_value = "")]
    /// Specify the Return-Path address
    return_path: String,
}

fn main() {
    env_logger::init();

    trace!("Parsing arguments.");
    let opt = Cli::from_args();
    debug!("{:?}", opt);

    let mut from = "user@localhost".to_string();
    let mut cc = "".to_string();
    let mut bcc = "".to_string();
    let to = "devin@localhost"; //FIXME

    let mut stdin = io::stdin();
    trace!("Parsing header.");
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line == "" { // Signalling separator between Headers and Body.
            break;
        }

        let tokens = line.split(':').collect::<Vec<_>>();
        let field = tokens[0].trim().to_lowercase();
        let value = tokens[1].trim().to_lowercase();

        debug!("field: {}, value: {}", field, value);
        match field.as_str() {
            "from" => { from = value; }
            "cc" => {cc = value;}
            "bcc" => {bcc = value;}
            _ => {} // This includes `subject`, `from`, `to`, etc..
        }
    }

    trace!("Parsing body.");
    let mut msg = "".to_string();
    let _ = stdin.read_to_string(&mut msg);
    let msg = msg.trim();
    debug!("msg body: \n{}", msg);

    trace!("Setting Return-Path");
    let ret_path = match opt.return_path.as_str() {
        "" => { from.to_string() }
        _ => {opt.return_path}
    };

    let email = EmailBuilder::new()
        .to(to.to_string())
        .cc(cc)
        .bcc(bcc)
        .from(ret_path.to_string()) // Sets Return-Path as a side effect. // TODO, file bug.
        .text(msg.to_string())
        .header(("from", from.to_string()))
        .build()
        .expect("Can't compose mail")
        .into();

    let mut mailer = SmtpClient::new_unencrypted_localhost().unwrap().transport();

    match mailer.send(email) {
        Ok(_) => { info!("succesfully sent. from: {}; to: {}; len: {}", from, to, msg.len()); }
        _ => { error!("sending failed. from: {}; to: {}; len: {}", from, to, msg.len()); }
    }
}
