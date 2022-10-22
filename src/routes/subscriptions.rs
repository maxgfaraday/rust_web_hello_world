// The collection of subscription related functionality
use crate::domain::{NewSubscriber,SubscriberEmail,SubscriberName};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use std::convert::{TryFrom};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}

impl TryFrom<FormData> for NewSubscriber {
    type Error = String;

    fn try_from(form: FormData) -> Result<Self, Self::Error> {
        let email = SubscriberEmail::parse(form.email)?;
        let name  = SubscriberName::parse(form.name)?;
        Ok(Self {email, name})
    }
}

#[tracing::instrument (
    name = "Adding a new subscriber",
    skip(form, connection_pool),
    fields (
        subscriber_name = %form.name,
        subscriber_email = %form.email
    )
)]
//Question: Why does this function have to be async?
pub async fn subscribe(form: web::Form<FormData>, connection_pool: web::Data<PgPool>) -> HttpResponse {
    match insert_subscriber(&form, &connection_pool).await
    {
        Ok(_) => {
            tracing::info!("New subscriber details have been saved");
            HttpResponse::Ok().finish()
        }
        Err(_) => {
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details to database",
    skip (form, connection_pool),
)]
async fn insert_subscriber(form: &FormData, connection_pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
INSERT INTO subscriptions (id, email, name, subscribed_at)
VALUES ($1, $2, $3, $4)
"#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
        .execute(connection_pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;
    Ok(())
}
