use lettre::{transport::smtp::authentication::Credentials,header::ContentType, SmtpTransport,Message, Transport};
use crate::utils::{
    error::ApiError
};
use std::env::var;

pub fn send(recipient:String,subject:String,body:String)->Result<(),ApiError>{
    let email = Message::builder()
    .from(format!("Do Not Reply <{}>",var("MAILER_USERNAME")?).parse().unwrap())
    .to(recipient.parse().unwrap())
    .subject(subject).header(ContentType::html())
    .body(body)?;

    let creds = Credentials::new(
        var("MAILER_USERNAME")?,
        var("MAILER_PASSWORD")?
    );

    let mailer = SmtpTransport::relay("smtp.gmail.com")?.credentials(creds).build();

    match mailer.send(&email){
        Ok(_)=> Ok(()),
        Err(e)=> Err(ApiError::InternalServerErrorWithMessage(e.to_string()))
    }
}