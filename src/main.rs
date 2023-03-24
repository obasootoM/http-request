use actix_web::{get,web,App,HttpServer};
use std::sync::Mutex;
use serde::{Deserialize,Serialize};



struct AppState{
    todoList: Mutex<Vec<TodoList>>
}
#[derive(Deserialize,Serialize,Clone)]
struct TodoList{
    id: i32,
    date: i64,
    title: String,
}

#[get("/")]
async fn index() -> String {
    "what is your name".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState{
        todoList: Mutex::new(vec![])
    });

    HttpServer::new(move || {
        App::new()
        .app_data(app_data.clone())
        .service(index)
    })
    .bind(("127.0.0,1",8000))?
    .run()
    .await
}
