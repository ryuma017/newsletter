use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    // `info_span!` は info-level の span を作る
    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name,
    );
    let _request_span_guard = request_span.enter();
    // `_request_span_guard` は `subscribe` の最後に drop され、span を抜ける

    // `query_span` に対して `.enter()` は呼ばない
    // `.instrument()` がクエリの future のライフタイムにおいて、適切なタイミングで処理してくれる
    let query_span = tracing::info_span!("Saving new subscriber details in thd database.");
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
    .instrument(query_span) // まず instrumentation をアタッチし、
    .await                  // `.await` する
    {
        Ok(_) => {
            tracing::info!(
                "request_id {} - New subscriber details have been saved.",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            // この error log は `query_span` の外
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
