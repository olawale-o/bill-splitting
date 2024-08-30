mod structs;
mod utils;
use structs::Contributor;
use structs::User;

fn main() {
    let user_1 = User {
        name: "wale".into(),
    };
    let user_2 = User {
        name: "sola".into(),
    };

    let contributor_1 = Contributor {
        user: user_1,
        percentage: 32,
        amount_contributed: 0.0,
    };
    let contributor_2 = Contributor {
        user: user_2,
        percentage: 68,
        amount_contributed: 0.0,
    };

    let contributors = vec![contributor_1, contributor_2];

    let bill_to_split: f64 = 1200.00;

    let contributors = utils::helpers::handle_split(contributors, &bill_to_split);

    for c in contributors {
        println!(
            "{} will contribute {} out of {}",
            c.user.name, c.amount_contributed, bill_to_split
        )
    }
}
