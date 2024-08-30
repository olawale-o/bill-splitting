pub struct User {
    pub name: String,
}

pub struct Contributor {
    pub user: User,
    pub percentage: usize,
    pub amount_contributed: f64,
}
