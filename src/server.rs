use axum::{Router, routing::get};

// Function to start the server
pub async fn start_server() {
    // build our application with a single route
    let app = Router::new().route("/", get(hello_world));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Example route handler
async fn hello_world() -> &'static str {
    "Hello, World!"
}
