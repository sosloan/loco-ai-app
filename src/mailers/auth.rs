#![allow(non_upper_case_globals)]

use loco_rs::prelude::*;
use serde_json::json;
use tokio_retry::strategy::{ExponentialBackoff, jitter};
use tokio_retry::Retry;

use crate::models::users;

static welcome: Dir<'_> = include_dir!("src/mailers/auth/welcome");
static forgot: Dir<'_> = include_dir!("src/mailers/auth/forgot");

#[allow(clippy::module_name_repetitions)]
pub struct AuthMailer {}
impl Mailer for AuthMailer {}
impl AuthMailer {
    /// Sending welcome email the the given user
    ///
    /// # Errors
    ///
    /// When email sending is failed
    pub async fn send_welcome(ctx: &AppContext, user: &users::Model) -> Result<()> {
        let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

        Retry::spawn(retry_strategy, || async {
            Self::mail_template(
                ctx,
                &welcome,
                mailer::Args {
                    to: user.email.to_string(),
                    locals: json!({
                      "name": user.name,
                      "verifyToken": user.email_verification_token,
                      "domain": ctx.config.server.full_url()
                    }),
                    ..Default::default()
                },
            )
            .await
        })
        .await?;

        Ok(())
    }

    /// Sending forgot password email
    ///
    /// # Errors
    ///
    /// When email sending is failed
    pub async fn forgot_password(ctx: &AppContext, user: &users::Model) -> Result<()> {
        let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

        Retry::spawn(retry_strategy, || async {
            Self::mail_template(
                ctx,
                &forgot,
                mailer::Args {
                    to: user.email.to_string(),
                    locals: json!({
                      "name": user.name,
                      "resetToken": user.reset_token,
                      "domain": ctx.config.server.full_url()
                    }),
                    ..Default::default()
                },
            )
            .await
        })
        .await?;

        Ok(())
    }
}
