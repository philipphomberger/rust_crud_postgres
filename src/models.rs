//Model: User struct with id, name, email
#[derive(Serialize, Deserialize)]
pub(crate) struct User {
    pub(crate) id: Option<i32>,
    pub(crate) name: String,
    pub(crate) email: String,
}
