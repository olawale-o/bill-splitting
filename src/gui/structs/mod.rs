#[derive(Debug, Clone)]
pub enum ContributorState {
    IDLE,
    EDIT,
}

#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum ContributorMessage {
    OnSliderChanged(f32),
    OnRemove,
    Edit,
    ContributorNameEdit(String),
    FinishEdit,
}

#[derive(Debug, Clone)]
pub enum Message {
    AddUser,
    OnAmountChanged(String),
    ContributorManaged(usize, ContributorMessage),
}

#[derive(Debug, Clone)]
pub struct Contributor {
    pub user: User,
    pub percentage: f32,
    pub amount_contributed: f64,
    pub state: ContributorState,
}

pub struct BillSplit {
    pub amount: String,
    pub contributors: Vec<Contributor>,
}

impl BillSplit {
    pub fn generate_contributors() -> Self {
        let c: Vec<Contributor> = vec![1, 2]
            .iter()
            .map(|i| Contributor::new(format!("user {i}")))
            .collect();
        Self {
            amount: String::from("0.0"),
            contributors: c,
        }
    }
}
