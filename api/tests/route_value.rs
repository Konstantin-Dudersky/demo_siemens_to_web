use axum_test::TestServer;
use serde_json::to_string as serialize;

use redis_client::RedisHashAsync;

use api::app;

async fn create_test_server() -> TestServer {
    let redis_hash = RedisHashAsync::new("redis://127.0.0.1/", "test_api")
        .await
        .unwrap();
    let app = app::App::new(redis_hash);
    TestServer::new(app.app.into_make_service()).unwrap()
}

#[tokio::test]
async fn test1() {
    let msg1 = messages::Messages::IntValueFromOpcUa(
        messages::types::SimpleValue::new(15),
    );

    let test_server = create_test_server().await;

    let put_response = test_server.put("/value/test_msg").json(&msg1).await;
    put_response.assert_status_ok();
    put_response.assert_text(serialize(&msg1).unwrap());
}
