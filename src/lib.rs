use std::io;
mod structs;
mod utils;

pub fn run() {
    let mut contributors = vec![];
    let mut amount_parse: f64 = 0.0;

    println!("This is bill splitting app");
    println!("Enter 1 to add users");
    println!("Enter 2 to add amout to be splitted by users");
    println!("Enter q, quit or exit to exit the application or CTRL+C, ctrl+c");

    'l: loop {
        let mut mode = String::new();

        println!("Kindly enter number 1 or 2 to add users or bill");
        io::stdin()
            .read_line(&mut mode)
            .expect("Kindly enter a mode");

        let trim_mode = mode.trim();

        if trim_mode == "q" || trim_mode == "quit" || trim_mode == "exit" {
            break 'l;
        }

        let value: i8 = match trim_mode.parse() {
            Ok(num) => num,
            Err(_) => continue 'l,
        };

        if value == 1 {
            utils::prompt::enter_user_mode(&mut contributors);
        } else if value == 2 {
            println!("Add amount to be splitted");
            let mut amount = String::new();
            utils::prompt::prompt_user(&mut amount, "Kindly enter a amount");

            amount_parse = amount.trim().parse().unwrap();

            if amount_parse > 0.0 {
                break;
            } else {
                println!("Enter amount greater than 0 ")
            }
        } else {
            continue 'l;
        }
    }
    let contributors = utils::helpers::handle_split(contributors, &amount_parse);
    for c in contributors {
        println!(
            "{} will contribute {} out of {}",
            c.user.name, c.amount_contributed, amount_parse
        )
    }
}
