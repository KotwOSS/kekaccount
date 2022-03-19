use lettre::{
    AsyncTransport, Message, AsyncSmtpTransport, Tokio1Executor, 
    transport::smtp::{authentication::Credentials, client::{Tls, TlsParametersBuilder}}, message::MultiPart
};

use tokio::sync::OnceCell;

use crate::colors;

static MAIL_OPTIONS: OnceCell<MailOptions> = OnceCell::const_new();

#[derive(Debug)]
pub struct MailOptions {
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    from: String,
    verification_base: String
}

pub fn init(host: String, port: u16, user: String, password: String, from: String, verification_base: String) {
    println!(
        "{}INIT{} smtp.rs",
        colors::GREEN,
        colors::RESET
    );

    let credentials = Credentials::new(user, password);

    let tls_params = TlsParametersBuilder::new(host.clone());

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(host.as_str())
        .expect("ERROR")
        .credentials(credentials)
        .port(port)
        .tls(Tls::Required(tls_params.build().unwrap()))
        .build();

    MAIL_OPTIONS.set(MailOptions {
        mailer,
        from,
        verification_base
    }).unwrap();
}

pub async fn send<'a,'b,'c>(subject: &'a str, body: String, to: &'b str, reply_to: &'c str) -> Result<(), Box<dyn std::error::Error>> {
    let mail_opts = MAIL_OPTIONS.get().unwrap();

    let mailer = &mail_opts.mailer;

    let email = Message::builder()
        .from(mail_opts.from.parse()?)
        .to(to.parse()?)
        .reply_to(reply_to.parse()?)
        .subject(subject)
        .body(body)?;

    mailer.send(email).await?;

    Ok(())
}

pub async fn send_multipart<'a,'b,'c>(subject: &'a str, multipart: MultiPart, to: &'b str, reply_to: &'c str) -> Result<(), Box<dyn std::error::Error>> {
    let mail_opts = MAIL_OPTIONS.get().unwrap();

    let mailer = &mail_opts.mailer;

    let email = Message::builder()
        .from(mail_opts.from.parse()?)
        .to(to.parse()?)
        .reply_to(reply_to.parse()?)
        .subject(subject)
        .multipart(multipart)?;

    mailer.send(email).await?;

    Ok(())
}

pub async fn send_verification<'a,'b>(username: &'a str, email: &'b str, verification_id: String) -> Result<(), Box<dyn std::error::Error>> {
    let mail_opts = MAIL_OPTIONS.get().unwrap();

    let html = format!(
        r#"<h3>Hello, {}</h3><p>Please verify your email <a href="{}{}">here</a>!"#, 
        username, mail_opts.verification_base, verification_id
    );

    let plain = format!(
r#"Hello, {}

Please verify your email at {}{}"#, 
        username, mail_opts.verification_base, verification_id
    );

    send_multipart("Email verification", 
        MultiPart::alternative_plain_html(plain, html), format!("{} <{}>", username, email).as_str(), "noreply@noreply.com"
    ).await
}