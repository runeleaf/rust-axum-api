use crate::domain::model::todo::Todo;
use crate::infrastructure::middleware::database::ConnPool;
use crate::infrastructure::request::create_todo::CreateTodo;
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};

pub async fn index_todos(Extension(db): Extension<ConnPool>) -> impl IntoResponse {
    println!("db: {:?}", db);
    let todos = sqlx::query_as!(
        Todo,
        "SELECT id, title, completed FROM todos order by id desc"
    )
    .fetch_all(&*db)
    .await
    .unwrap();

    Json(todos)
}

pub async fn create_todo(
    Extension(db): Extension<ConnPool>,
    Json(todo): Json<CreateTodo>,
) -> impl IntoResponse {
    println!("db: {:?}", db);
    let todo = sqlx::query_as!(
        Todo,
        "INSERT INTO todos (title, completed) VALUES ($1, $2) RETURNING id, title, completed",
        todo.title,
        false
    )
    .fetch_one(&*db)
    .await
    .unwrap();

    (StatusCode::CREATED, Json(todo))
}
