use std::str::FromStr;

use axum_test::TestServer;
use serde_json::to_string as serialize;
use url::Url;

use redis_client_lib::RedisPubAsync;

use api::app;

async fn create_test_server() -> TestServer {
    let url = Url::from_str("redis://127.0.0.1").unwrap();
    let redis_hash = RedisPubAsync::new(&url, "test_api").await.unwrap();
    let app = app::App::new(redis_hash);
    TestServer::new(app.app.into_make_service()).unwrap()
}

#[tokio::test]
async fn test1() {
    let msg1 = messages::Messages::MotorState(
        messages::types::SingleValue::new(15, None),
    );

    let test_server = create_test_server().await;

    let put_response = test_server.put("/value/test_msg").json(&msg1).await;
    put_response.assert_status_ok();
    put_response.assert_text(serialize(&msg1).unwrap());
}
