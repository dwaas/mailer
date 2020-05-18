extern crate lettre;

#[macro_use]
extern crate log;

use log::Level;

use structopt::StructOpt;
use lettre::{SendableEmail, EmailAddress, Transport, Envelope, SmtpClient};

use std::io;
use std::io::prelude::*;

/*
#[derive(StructOpt)]
struct Cli {
}
*/


fn main() {
    env_logger::init();

    let mut from = "user@localhost".to_string();
    let mut cc = "".to_string();
    let mut bcc = "".to_string();
    let to = "devin@localhost";

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
    stdin.read_to_string(&mut msg);
    let msg = msg.trim();
    debug!("msg body: \n{}", msg);


    let email = SendableEmail::new(
        Envelope::new(
            Some(EmailAddress::new(from.to_string()).unwrap()),
            vec![EmailAddress::new(to.to_string()).unwrap()]
            ).unwrap(),
            "id".to_string(), //TODO?
            msg.to_string().into_bytes(),
        );

    let mut mailer = SmtpClient::new_unencrypted_localhost().unwrap().transport();

    match mailer.send(email) {
        Ok(_) => { info!("succesfully sent. from: {}; to: {}; len: {}", from, to, msg.len()); }
        _ => { error!("sending failed. from: {}; to: {}; len: {}", from, to, msg.len()); }
    }
}
