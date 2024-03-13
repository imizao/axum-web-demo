use axum::routing::get;
use serde_json::{json, Value};
use std::collections::HashMap;

mod book;
use crate::book::Book;

mod data;
use crate::data::DATA;

use std::thread;

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .route("/demo.json", get(get_demo_json))
        .route("/demo-form", get(get_demo_form).post(post_demo_form))
        .route("/books", get(get_books).put(put_books))
        .route("/books/:id", get(get_books_id).delete(delete_books_id))
        .route(
            "/books/:id/form",
            get(get_books_id_form).post(post_books_id_form),
        )
        .route("/demo-query", get(get_demo_query))
        .route("/foo", get(get_foo).put(put_foo))
        .route("/demo-http-status-code", get(demo_http_status_code))
        .route("/demo-path/:id", get(get_demo_path_id))
        .fallback(fallback);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub async fn get_demo_json() -> axum::extract::Json<Value> {
    json!({"a":"b"}).into()
}

pub async fn put_demo_json(
    axum::extract::Json(data): axum::extract::Json<serde_json::Value>,
) -> String {
    format!("Put demo JSON data: {:?}", data)
}

pub async fn get_demo_path_id(axum::extract::Path(id): axum::extract::Path<String>) -> String {
    format!("Get demo path id: {:?}", id)
}

pub async fn get_demo_query(
    axum::extract::Query(params): axum::extract::Query<HashMap<String, String>>,
) -> String {
    format!("Demo query params: {:?}", params)
}

pub async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("No route {}", uri),
    )
}
/// axum handler for "GET /demo-form" which responds with a form.
/// This demo shows how to write a typical HTML form with input fields.
pub async fn get_demo_form() -> axum::response::Html<&'static str> {
    r#"
    <!doctype html>
    <html>
        <head>
            <title>Book</title>
        </head>
        <body>
            <h1>Book</h1>
            <form method="post" action="/demo-form">
                <p>
                    <label for="title">
                        Title:
                        <br>
                        <input name="title">
                    </label>
                </p>
                <p>
                    <label for="author">
                        Author:
                        <br>
                        <input name="author">
                    </label>
                </p>
                <p>
                    <input type="submit">
                </p?
            </form>
        </body>
    </html>
    "#
    .into()
}
pub async fn post_demo_form(form: axum::extract::Form<Book>) -> axum::response::Html<String> {
    let book: Book = form.0;
    format!(
        r#"
            <!doctype html>
            <html>
                <head>
                    <title>Book</title>
                </head>
                <body>
                    <h1>Book</h1>
                    {:?}
                </body>
            </html>
        "#,
        &book
    )
    .into()
}

pub async fn demo_http_status_code() -> (axum::http::StatusCode, String) {
    (axum::http::StatusCode::OK, "Everything is Ok".to_string())
}

pub async fn get_foo() -> String {
    "GET foo".to_string()
}

pub async fn put_foo() -> String {
    "PUT foo".to_string()
}

#[allow(dead_code)]
async fn print_data() {
    thread::spawn(move || {
        let data = DATA.lock().unwrap();
        println!("{:?}", data);
    })
    .join()
    .unwrap();
}

pub async fn get_books() -> axum::response::Html<String> {
    thread::spawn(move || {
        let data = DATA.lock().unwrap();
        let mut books = data.values().collect::<Vec<_>>().clone();
        books.sort_by(|a, b| a.title.cmp(&b.title));
        books
            .iter()
            .map(|book| format!("<p>{}</p>\n", &book))
            .collect::<String>()
    })
    .join()
    .unwrap()
    .into()
}

pub async fn put_books(
    axum::extract::Json(book): axum::extract::Json<Book>,
) -> axum::response::Html<String> {
    thread::spawn(move || {
        let mut data = DATA.lock().unwrap();
        data.insert(book.id, book.clone());
        format!("put book: {}", &book)
    })
    .join()
    .unwrap()
    .into()
}

pub async fn get_books_id(
    axum::extract::Path(id): axum::extract::Path<u32>,
) -> axum::response::Html<String> {
    thread::spawn(move || {
        let data = DATA.lock().unwrap();
        match data.get(&id) {
            Some(book) => format!("<p>{}</p>\n", &book),
            None => format!("<p>Book id {} not found</p>", id),
        }
    })
    .join()
    .unwrap()
    .into()
}

pub async fn delete_books_id(
    axum::extract::Path(id): axum::extract::Path<u32>,
) -> axum::response::Html<String> {
    thread::spawn(move || {
        let mut data = DATA.lock().unwrap();
        if data.contains_key(&id) {
            data.remove(&id);
            format!("delete book id: {}", &id)
        } else {
            format!("book id: {} not found", &id)
        }
    })
    .join()
    .unwrap()
    .into()
}

pub async fn get_books_id_form(
    axum::extract::Path(id): axum::extract::Path<u32>,
) -> axum::response::Html<String> {
    thread::spawn(move || {
        let data = DATA.lock().unwrap();
        match data.get(&id) {
            Some(book) => format!(
                concat!(
                    "<form method=\"post\" action=\"/books/{}/form\">\n",
                    "<input type=\"hidden\" name=\"id\" value=\"{}\">\n",
                    "<p><input type=\"text\" name=\"title\" value=\"{}\"></p>\n",
                    "<p><input type=\"text\" name=\"author\" value=\"{}\"></p>\n",
                    "<input type=\"submit\" value=\"Save\" >\n",
                    "</form>\n"
                ),
                &book.id, &book.id, &book.title, &book.author
            ),
            None => format!("<p>Book id {} not found</p>", id),
        }
    })
    .join()
    .unwrap()
    .into()
}

pub async fn post_books_id_form(form: axum::extract::Form<Book>) -> axum::response::Html<String> {
    let new_book: Book = form.0;
    thread::spawn(move || {
        let mut data = DATA.lock().unwrap();
        if data.contains_key(&new_book.id) {
            data.insert(new_book.id, new_book.clone());
            format!("update book: {}", &new_book)
        } else {
            format!("Book id not found: {} ", &new_book.id)
        }
    })
    .join()
    .unwrap()
    .into()
}
