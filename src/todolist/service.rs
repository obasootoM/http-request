use actix_web::{get,post,put,delete,web,Responder,HttpResponse};
use crate::{AppState,TodoList};
use super::model::{CreateEntry,UpdateEntry};

#[get("todolist/entry/{id}")]
async fn get_entry(data: web::Data<AppState>) -> impl Responder  {
    HttpResponse::Ok().json(data.todoList.lock().unwrap().to_vec())
}

#[post("todolist/entry")]
async fn create_entry(data: web::Data<AppState>, param_object: web::Json<CreateEntry>) -> impl Responder {
    let mut todolist_entry = data.todoList.lock().unwrap();
    let mut max_id: i32 = 0;

    for i in 0..todolist_entry.len() {
        if todolist_entry[i].id > max_id {
            max_id = todolist_entry[i].id
        }
    }
    todolist_entry.push(TodoList{
        id: max_id + 1,
        date: param_object.date,
        title: param_object.title.clone(),

    });

    HttpResponse::Ok().json(todolist_entry.to_vec())
}

#[put("todolist/entry/{id}")]

async fn update_entry(data: web::Data<AppState>, param_object: web::Json<UpdateEntry>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let mut todolist_entry = data.todoList.lock().unwrap();

    for i in 0..todolist_entry.len() {
        if todolist_entry[i].id == id {
            todolist_entry[i].title = param_object.title.clone()
        }
        break
    }

    HttpResponse::Ok().json(todolist_entry.to_vec())
}

#[delete("todolist/entry/{id}")]


async fn delete_entry(data: web::Data<AppState>, path: web::Path<i32>)-> impl Responder {
    let id = path.into_inner();
    let mut todolist_entry = data.todoList.lock().unwrap();

    *todolist_entry = todolist_entry.to_vec().into_iter().filter(|x| x.id != id ).collect();
    HttpResponse::Ok().json(todolist_entry.to_vec())
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_entry)
    .service(create_entry)
    .service(update_entry)
    .service(delete_entry);
}