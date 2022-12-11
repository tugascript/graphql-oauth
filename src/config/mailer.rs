use async_graphql::{Error, Result};
use entities::user::Model;
use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message,
    Tokio1Executor,
};
use std::env;

#[derive(Debug, Clone)]
pub struct Mailer {
    pub email: String,
    pub front_end_url: String,
    pub mailer: AsyncSmtpTransport<Tokio1Executor>,
}

impl Mailer {
    pub fn new() -> Self {
        let host = env::var("EMAIL_HOST").unwrap();
        let email = env::var("EMAIL_USER").unwrap();
        let password = env::var("EMAIL_PASSWORD").unwrap();
        let port = env::var("EMAIL_PORT").unwrap().parse::<u16>().unwrap();
        let front_end_url = env::var("FRONT_END_URL").unwrap();
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&host)
            .unwrap()
            .port(port)
            .credentials(Credentials::new(email.to_owned(), password))
            .build();

        Self {
            email,
            front_end_url,
            mailer,
        }
    }

    async fn send_email(&self, to: String, subject: String, body: String) -> Result<()> {
        let message = Message::builder()
            .from(self.email.parse().unwrap())
            .to(to.parse().unwrap())
            .subject(subject)
            .body(body);

        if let Ok(msg) = message {
            match self.mailer.send(msg).await {
                Err(_) => Err(Error::from("Error sending the email")),
                _ => Ok(()),
            }
        } else {
            Err(Error::from("Invalid subject or body"))
        }
    }

    pub async fn send_confirmation_email(&self, model: Model, jwt: &str) -> Result<()> {
        let link = format!("{}/confirmation/{}", self.front_end_url, &jwt);
        let full_name = model.get_full_name();

        self.send_email(
            model.email,
            format!("Email confirmation, {}", &full_name),
            format!(
                r#"
            <body>
              <p>Hello {},</p>
              <br />
              <p>Welcome to Your Company,</p>
              <p>
                Click
                <b>
                  <a href='{}' target='_blank'>here</a>
                </b>
                to activate your acount or go to this link:
                {}
              </p>
              <p><small>This link will expire in an hour.</small></p>
              <br />
              <p>Best regards,</p>
              <p>Your Company Team</p>
            </body>
          "#,
                &full_name, &link, &link,
            ),
        )
        .await
    }

    pub async fn send_access_email(&self, email: &str, full_name: &str, code: &str) -> Result<()> {
        self.send_email(
            email.to_owned(),
            format!("Your access code, {}", full_name),
            format!(
                r#"
                <body>
                    <p>Hello {},</p>
                    <br />
                    <p>Welcome to Your Company,</p>
                    <p>
                        Your access code is
                        <b>{}</b>
                    </p>
                    <p><small>This code will expire in 15 minutes.</small></p>
                    <br />
                    <p>Best regards,</p>
                    <p>Your Company Team</p>
                </body> 
            "#,
                full_name, code
            ),
        )
        .await
    }

    pub async fn send_password_reset_email(
        &self,
        email: &str,
        full_name: &str,
        token: &str,
    ) -> Result<()> {
        let link = format!("{}/confirmation/{}", self.front_end_url, &token);

        self.send_email(
            email.to_owned(),
            format!("Email confirmation, {}", full_name),
            format!(
                r#"
                <body>
                    <p>Hello {},</p>
                    <br />
                    <p>Your password reset link:
                    <b><a href='{}' target='_blank'>here</a></b></p>
                    <p>Or go to this link: {}</p>
                    <p><small>This link will expire in 30 minutes.</small></p>
                    <br />
                    <p>Best regards,</p>
                    <p>Your Company Team</p>
                </body>
                "#,
                &full_name, &link, &link,
            ),
        )
        .await
    }
}
