use lettre::{
    Message, SmtpTransport, Transport,
    transport::smtp::authentication::Credentials
};
use std::env;

pub fn enviar_credenciales(
    destino: &str,
    usuario: &str,
    password: &str
) -> Result<(), Box<dyn std::error::Error>> {

    let smtp_user = env::var("SMTP_USER")?;
    let smtp_pass = env::var("SMTP_PASS")?;
    let smtp_host = env::var("SMTP_HOST")?;

    let email = Message::builder()
        .from(smtp_user.parse()?)
        .to(destino.parse()?)
        .subject("Credenciales de acceso al sistema")
        .body(format!(
            "
Bienvenido al sistema

Usuario: {}
Contraseña: {}

Por seguridad cambia tu contraseña después de iniciar sesión.
            ",
            usuario,
            password
        ))?;

    let creds = Credentials::new(smtp_user.clone(), smtp_pass);

    let mailer = SmtpTransport::relay(&smtp_host)?
        .credentials(creds)
        .build();

    mailer.send(&email)?;

    Ok(())
}