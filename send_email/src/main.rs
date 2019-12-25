extern crate lettre;
extern crate lettre_email;
use lettre::smtp::authentication::IntoCredentials;
use lettre::{SmtpClient, Transport};
use lettre_email::EmailBuilder;
fn main() {
    let smtp_address = "smtp.gmail.com";
    let username = "?.com";
    let password = "?";
    let email = EmailBuilder::new()
        .to("jonc@destini.com")
        .from(username)
        .subject("Rust test send")
        .text("Bears eat beets. Bears. Beets. Battlestar Galactica.")
        .build()
        .unwrap()
        .into();
    let credentials = (username, password).into_credentials();
    let mut client = SmtpClient::new_simple(smtp_address)
        .unwrap()
        .credentials(credentials)
        .transport();
    //let _result = client.send(email);
    match client.send(email) {
        Ok(s) => println!("OK, send result={:?}", s),
        Err(e) => println!("ERR, send result={:?}", e),
    };

}