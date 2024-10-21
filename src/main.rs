use axum::{routing::get, Extension, Router};
use std::env;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let server_msg: String = env::var("SERVER_MSG").expect("SERVER_MSG is not defined");
    //Router
    let app = Router::new()
        .route("/", get(top_handler))
        .layer(Extension(server_msg));

    //Server.layer(Extension(home))
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

//*ハンドラ関数 */
async fn top_handler(Extension(server_msg): Extension<String>) -> String {
    server_msg.to_string()
}
