mod helpers;

use helpers::spawn_app;
use sqlx;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;

    let body = "name=nick%20ccc&email=notreal%40mydomain.com";
    let response = app.post_subscriptions(body.into()).await;
        
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "notreal@mydomain.com");
    assert_eq!(saved.name, "nick-ccc")
}

#[tokio::test]
async fn subscribe_returns_a_400_for_invalid_form_data() {
    let app = spawn_app().await;

    let test_cases = vec![
        ("name=nikc%20c", "missing email"),
        ("email=notreal%40mydomain.com&", "missing name"),
        ("", "missing both"),
    ];
    for (invalid_body, error_message) in test_cases {
        let response = app.post_subscriptions(invalid_body.into()).await;
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did no fail with 400 bad request when the payload was {}!",
            error_message
        );
    }
}
