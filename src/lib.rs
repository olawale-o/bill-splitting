use std::io;
mod structs;
mod utils;

pub fn run() {
    let mut contributors = vec![];
    let mut amount_parse: f64 = 0.0;

    println!("This is a bill splitting app");
    println!("Enter q, quit or exit to exit the application or CTRL+C, ctrl+c");

    'l: loop {
        let mut mode = String::new();

        println!("Kindly enter number 1 to add users or bill");
        io::stdin()
            .read_line(&mut mode)
            .expect("Kindly enter a mode");

        let trim_mode = mode.trim();

        if trim_mode == "q" || trim_mode == "quit" || trim_mode == "exit" {
            break 'l;
        }
        amount_parse = utils::prompt::enter_user_mode(&mut contributors);

        if amount_parse > 0f64 {
            break;
        }
    }
    let contributors = utils::bill::handle_split(contributors, &amount_parse);
    contributors.iter().for_each(|c| {
        println!(
            "{} will contribute {} out of {}",
            c.user.name, c.amount_contributed, amount_parse
        )
    });
}
