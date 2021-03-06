use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}


pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>, // Renamed!
) -> HttpResponse {
    let _result = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    // There is a bit of ceremony here to get our hands on a &PgConnection.
    // web::Data<Arc<PgConnection>> is equivalent to Arc<Arc<PgConnection>>
    // Therefore connection.get_ref() returns a &Arc<PgConnection>
    // which we can then deref to a &PgConnection.
    // We could have avoided the double Arc wrapping using .app_data()
    // instead of .data() in src/startup.rs - we'll get to it later!
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        return HttpResponse::InternalServerError().finish()
    });
    
    HttpResponse::Ok().finish()
}
