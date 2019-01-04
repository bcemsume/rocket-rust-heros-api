#[derive(Serialize, Deserialize)]
pub struct Hero {
    pub id: i32,
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32
}
