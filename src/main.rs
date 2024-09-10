use std::str::FromStr;
mod gui;
fn main() -> iced::Result {
    gui::run()
}
// fn main() {
//     if let Err(e) = f64::from_str("0.12") {
//         println!("Failed conversion to f64: {e}");
//     } else {
//         println!("Correct value")
//     }
// }
