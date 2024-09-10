mod structs;
use std::usize;
use structs::{BillSplit, Contributor, ContributorMessage, ContributorState, Message, User};

use iced::{
    alignment::Horizontal,
    widget::{button, column, container, row, scrollable, slider, text, text_input, TextInput},
    Alignment, Element, Length, Sandbox, Settings,
};

impl Contributor {
    fn new(name: String) -> Self {
        Self {
            user: User { name },
            percentage: 0.0,
            amount_contributed: 0.0,
            state: ContributorState::IDLE,
        }
    }
    fn text_input_id(i: usize) -> text_input::Id {
        text_input::Id::new(format!("contributor {i}"))
    }

    fn view(&self, i: usize) -> Element<ContributorMessage> {
        let percentage_slider = slider(
            0.0..=100.0,
            self.percentage,
            ContributorMessage::OnSliderChanged,
        );
        let contributor_detail = row![
            text(self.user.name.clone()),
            text(self.percentage),
            text(self.amount_contributed),
        ]
        .spacing(20.0);

        let action_buttons = row![
            button("Remove").on_press(ContributorMessage::OnRemove),
            button("Edit").on_press(ContributorMessage::Edit),
        ]
        .align_items(Alignment::End)
        .spacing(10.0);

        let username_input: TextInput<_> =
            text_input("Hello", &self.user.name).on_input(ContributorMessage::ContributorNameEdit);

        let save_actions = row![
            button("Remove").on_press(ContributorMessage::OnRemove),
            button("Save").on_press(ContributorMessage::FinishEdit)
        ]
        .align_items(Alignment::End)
        .spacing(10.0);
        let w = match &self.state {
            ContributorState::IDLE => {
                column!(percentage_slider, contributor_detail, action_buttons).spacing(20.0)
            }
            ContributorState::EDIT => {
                column!(percentage_slider, username_input, save_actions).spacing(20.0)
            }
        };

        container(w).width(Length::Fill).into()
    }

    fn update(&mut self, message: ContributorMessage) {
        match message {
            ContributorMessage::OnSliderChanged(value) => {}
            ContributorMessage::OnRemove => {}
            ContributorMessage::Edit => self.state = ContributorState::EDIT,
            ContributorMessage::ContributorNameEdit(value) => self.user.name = value,
            ContributorMessage::FinishEdit => {
                if !self.user.name.is_empty() {
                    self.state = ContributorState::IDLE;
                }
            }
        }
    }
}

impl Sandbox for BillSplit {
    type Message = Message;

    fn new() -> Self {
        Self::generate_contributors()
    }

    fn title(&self) -> String {
        String::from("App - Bill Splitting")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::AddUser => {
                let mut name = String::from("user ");
                let t: String = (self.contributors.len() + 1).to_string();
                name.insert_str(5, &t);
                self.contributors.push(Contributor::new(name));
            }
            Message::OnAmountChanged(value) => {
                self.amount = value.trim().to_string();
            }

            Message::ContributorManaged(i, ContributorMessage::OnRemove) => {
                self.contributors.remove(i);
            }
            Message::ContributorManaged(i, ContributorMessage::OnSliderChanged(value)) => {
                let c = self.contributors.get_mut(i).unwrap();
                if matches!(c.state, ContributorState::IDLE) {
                    c.percentage = value
                }

                self.contributors
                    .iter_mut()
                    .map(|c| {
                        let amount_to_pay =
                            (c.percentage as f64 / 100.0) * self.amount.parse::<f64>().unwrap();
                        c.amount_contributed = amount_to_pay
                    })
                    .collect()
            }
            Message::ContributorManaged(i, contributor_message) => {
                if let Some(c) = self.contributors.get_mut(i) {
                    let should_focus = matches!(contributor_message, ContributorMessage::Edit);
                    c.update(contributor_message);
                    // if should_focus {
                    //     let id = Contributor::text_input_id(i);
                    // } else {
                    // }
                } else {
                    println!("Issues")
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let bills = column(self.contributors.iter().enumerate().map(|(i, f)| {
            f.view(i)
                .map(move |message| Message::ContributorManaged(i, message))
        }))
        .spacing(15.0);

        let columns = column![
            container(button("Add person").on_press(Message::AddUser))
                .width(Length::Fill)
                .align_x(Horizontal::Right),
            text_input("Please enter an amount", &self.amount).on_input(Message::OnAmountChanged),
            bills,
        ]
        .spacing(20.0);

        scrollable(
            container(
                container(columns)
                    .padding(20.0)
                    .max_width(500.0)
                    .center_x()
                    .center_y(),
            )
            .width(Length::Fill)
            .align_x(Horizontal::Center),
        )
        .into()
    }
}

pub fn run() -> iced::Result {
    BillSplit::run(Settings::default())
}
