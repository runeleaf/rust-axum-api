use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateTodo {
    pub title: String,
}
