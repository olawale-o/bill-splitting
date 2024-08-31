#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
}
#[derive(Debug, Clone)]
pub struct Contributor {
    pub user: User,
    pub percentage: f32,
    pub amount_contributed: f64,
}
