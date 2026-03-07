use serde::Serialize;

#[derive(Serialize)]
pub struct MenuItem {
    pub id: i32,
    pub nombre: String,
    pub hijos: Vec<MenuHijo>
}

#[derive(Serialize)]
pub struct MenuHijo {
    pub id: i32,
    pub nombre: String
}