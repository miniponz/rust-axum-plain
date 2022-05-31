use axum::routing::get;

mod book;
mod data;
use crate::data::DATA;
use std::thread;

#[tokio::main]

pub async fn main() {
    // create router
    let app = axum::Router::new()
        // .fallback(fallback.into_service())
        .route("/", get(hello))
        .route("/books", get(get_books));

    // run app
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(signal_shutdown())
        .await
        .unwrap();

    print_data().await;
}

pub async fn hello() -> String {
    "hello".into()
}

pub async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("No route {}", uri),
    )
}

async fn signal_shutdown() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal crtl-c");
    println!("signal shutdown")
}

async fn print_data() {
    thread::spawn(move || {
        let data = DATA.lock().unwrap();
        println!("data: {:?}", data)
    })
    .join()
    .unwrap()
}

// a handler that uses external data source
pub async fn get_books() -> axum::response::Html<String> {
    thread::spawn(move || {
        let data = DATA.lock().unwrap();
        let mut books = data.values().collect::<Vec<_>>().clone();
        books.sort_by(|a, b| a.title.cmp(&b.title));
        books
            .iter()
            .map(|&book| format!("<p>{}<p>\n", &book))
            .collect::<String>()
    })
    .join()
    .unwrap()
    .into()
}
