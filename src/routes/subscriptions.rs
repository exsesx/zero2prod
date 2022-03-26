use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::{Error, PgPool};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(error) => {
            println!("Failed to execute query: {}", error);

            if let Error::Database(e) = error {
                if e.code().unwrap() == "23505" {
                    return HttpResponse::Conflict().finish();
                }
            }

            HttpResponse::InternalServerError().finish()
        }
    }
}
