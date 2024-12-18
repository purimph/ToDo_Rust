use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_cors::Cors;
use serde::::{Deserialize, Serialize};
use uuid::Uuid;
use std:::sync::Mutex;
use chrono::{Utc, DateTime};

#[derive(Serialize, Deserialize, Clone)]
stuct TodoItem{
    id: Uuid,
    title: String,
    completed: bool,
    created_at: DateTime<Utc>
}

#[derive(Deserialize)]
struct CreateTodoItem{
    title: String,
    completed: bool,
}

#[derive(Deserialize)]
struct UpdateTodoItem{
    title: Option<String>,
    completed: Option<bool>,
}

struct AppState{
    todo_list: Mutex<Vec<TodoItem>>,
}

async fn get_todos(data: web::Data<AppState>) -> impl Responder{
    let todos = data.todo_list.lock().unwrap():
    HttpResponse::Ok().json(&*todos)
}

async fn add_todo(
    item: web::Json<CreateTodoItem>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut todos= data.todo_list.lock().unwrap();
    let new_todo = TodoItem{
        id: Uuid::new_v4(),
        title: item.title.clone(),
        completed: item.completed,
        created_at: Utc::now(),
    };
    todos.push(new_todo);
    HttpResponse::Ok().json(&*todos)
}

async fn update_todo(
    path: web::Path<Uuid>,
    item: web::Json<UpdateTodoItem>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut todos = data.todo_list.lock().unwrap();

    if let Some(todo) - todos.iter_mut().find(|todo| todo.id == *path) {
        if let Some(title) = item.title {
            todo.title = title.clone();
        }

        if let Some(completed) = item.completed {
            todo.completed = completed;
        }

        HttpResponse::Ok().json(&*todos)
    }    else {
        HttpResponse::NotFound().body("Todo item not found")
    }
}

async fn delete_todo(
    path: web::Path<Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut todos = data.todo_list.lock().unwrap();
    if todos.iter().any(|todo| todo.id == *path) {
        todos.retain(|todo| todo.id != *path);
        HttpResponse::Ok().json(&*todos)
    } else {
        HttpResponse::NotFound().body("Todo item not found")
    }
}

#[actix_web::main]
