pub mod bill {
    use crate::structs::Contributor;

    pub fn handle_split(contributors: Vec<Contributor>, bill_to_split: &f64) -> Vec<Contributor> {
        let results: Vec<Contributor> = contributors
            .into_iter()
            .map(|mut c| {
                let amount_to_pay = (c.percentage as f64 / 100.0) * bill_to_split;
                c.amount_contributed = amount_to_pay;
                c
            })
            .collect();
        results
    }
}

pub mod prompt {
    use crate::structs::Contributor;
    use crate::structs::User;
    use std::io;

    pub fn prompt_user(s: &mut String, err_msg: &str) {
        io::stdin().read_line(s).expect(err_msg);
    }

    pub fn enter_user_mode(contributors: &mut Vec<Contributor>) {
        println!("Kindly add atleast 2 users to split bill");
        println!("Kindly enter 2 to exit adding users");
        loop {
            let mut name = String::new();
            let mut percentage = String::new();
            let mut keep_mode = String::new();
            let user = User {
                name: String::new(),
            };
            let mut contributor = Contributor {
                user,
                percentage: 0.0,
                amount_contributed: 0.0,
            };

            println!("Kindly enter name of the user");
            prompt_user(&mut name, "Kindly enter a name");

            contributor.user.name = name.trim().into();

            println!("Kindly enter percentage contribution");
            prompt_user(&mut percentage, "Kindly enter a percentage");

            let value: f32 = percentage.trim().parse().unwrap();
            contributor.percentage = value;

            self::add_contributor(contributor, contributors);

            if contributors.len() >= 2 {
                println!("Do you want to continue");
                prompt_user(&mut keep_mode, "Keep editing");

                let trim_keep_mode = keep_mode.trim();

                if trim_keep_mode == "y" || trim_keep_mode == "yes" {
                    continue;
                } else {
                    break;
                }
            }
        }
    }

    pub fn enter_amount_mode() {}

    fn add_contributor(contributor: Contributor, contributors: &mut Vec<Contributor>) {
        contributors.push(contributor)
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::{Contributor, User};

    use super::bill;

    #[test]
    fn handle_split() {
        let contributors = vec![
            Contributor {
                user: User {
                    name: "wale".into(),
                },
                percentage: 20f32,
                amount_contributed: 0.0,
            },
            Contributor {
                user: User {
                    name: "sola".into(),
                },
                percentage: 80f32,
                amount_contributed: 0.0,
            },
        ];
        let amount_to_split = 2000f64;

        let result = bill::handle_split(contributors, &amount_to_split);

        let expect = vec![
            Contributor {
                user: User {
                    name: "wale".into(),
                },
                percentage: 20f32,
                amount_contributed: 400f64,
            },
            Contributor {
                user: User {
                    name: "sola".into(),
                },
                percentage: 80f32,
                amount_contributed: 1600f64,
            },
        ];

        assert_eq!(result[0].amount_contributed, expect[0].amount_contributed);
        assert_eq!(result[1].amount_contributed, expect[1].amount_contributed);
    }
}
