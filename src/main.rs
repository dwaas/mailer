extern crate lettre;

#[macro_use]
extern crate log;

use log::Level;

use structopt::StructOpt;
use lettre::{SendableEmail, EmailAddress, Transport, Envelope, SmtpClient};

/*
#[derive(StructOpt)]
struct Cli {
}
*/


fn main() {
    env_logger::init();

    let from = "user@localhost";
    let to = "devin@localhost";
    let msg = "Hello world!";

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
