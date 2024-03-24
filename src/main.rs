use actix_web::{get, post, put, delete, web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};

// Define your data structure
#[derive(Serialize, Deserialize, Debug)]
struct Book {
    id: u32,
    title: String,
    author: String,
}

// In-memory database (for simplicity)
struct AppState {
    books: Mutex<Vec<Book>>,
}

#[get("/books")]
async fn get_books(state: web::Data<Arc<AppState>>) -> impl Responder {
    let books = state.books.lock().unwrap();
    HttpResponse::Ok().json(&*books)
}

#[post("/books")]
async fn add_book(new_book: web::Json<Book>, state: web::Data<Arc<AppState>>) -> impl Responder {
    let mut books = state.books.lock().unwrap();
    books.push(new_book.into_inner());
    HttpResponse::Created()
}

#[put("/books/{id}")]
async fn update_book(
    path: web::Path<u32>,
    updated_book: web::Json<Book>,
    state: web::Data<Arc<AppState>>,
) -> impl Responder {
    let id = path.into_inner();
    let mut books = state.books.lock().unwrap();
    if let Some(book) = books.iter_mut().find(|b| b.id == id) {
        *book = updated_book.into_inner();
        HttpResponse::Ok()
    } else {
        HttpResponse::NotFound()
    }
}

#[delete("/books/{id}")]
async fn delete_book(
    path: web::Path<u32>,
    state: web::Data<Arc<AppState>>,
) -> impl Responder {
    let id = path.into_inner();
    let mut books = state.books.lock().unwrap();
    if let Some(index) = books.iter().position(|b| b.id == id) {
        books.remove(index);
        HttpResponse::NoContent()
    } else {
        HttpResponse::NotFound()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = Arc::new(AppState {
        books: Mutex::new(vec![
            Book { id: 1, title: "The Hobbit".to_string(), author: "J.R.R. Tolkien".to_string() },
            Book { id: 2, title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string() },
            Book { id: 3, title: "1984".to_string(), author: "George Orwell".to_string() },
        ]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(get_books)
            .service(add_book)
            .service(update_book)
            .service(delete_book)
    })
    .bind("127.0.0.1:9090")?
    .run()
    .await
}
