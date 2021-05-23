use iced::{
    button, pick_list, text_input, Button, Column, Container, Element, Length, PickList, Radio,
    Row, Sandbox, Text, TextInput,
};

#[derive(Default, Debug)]
pub struct Application {
    pass: String,
    source: String,

    mode_list: pick_list::State<Mode>,
    selected_mode: Mode,

    pass_input: text_input::State,
    source_input: text_input::State,

    crypt_button: button::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    CryptPressed,
    PassInputChanged(String),
    SourceInputChanged(String),
    ModeSelected(Mode),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Encrypt,
    Decrypt,
}

impl Mode {
    const ALL: [Mode; 2] = [Mode::Encrypt, Mode::Decrypt];
}

impl Default for Mode {
    fn default() -> Self {
        Self::Encrypt
    }
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Mode::Encrypt => "encrypt",
                Mode::Decrypt => "decrypt",
            }
        )
    }
}

impl Sandbox for Application {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("test")
    }

    fn view(&mut self) -> Element<Message> {
        let pass_input = TextInput::new(
            &mut self.pass_input,
            "Enter pass...",
            &self.pass,
            Message::PassInputChanged,
        )
        .padding(10)
        .size(20);

        let source_input = TextInput::new(
            &mut self.source_input,
            "Enter source...",
            &self.source,
            Message::SourceInputChanged,
        )
        .padding(10)
        .size(20);

        let list = PickList::new(
            &mut self.mode_list,
            &Mode::ALL[..],
            Some(self.selected_mode),
            Message::ModeSelected,
        );

        let crypt_button =
            Button::new(&mut self.crypt_button, Text::new("crypt")).on_press(Message::CryptPressed);

        let radio = Column::new()
            .spacing(10)
            .push(
                Radio::new(
                    Mode::Encrypt,
                    "encrypt",
                    Some(self.selected_mode),
                    Message::ModeSelected,
                )
                .size(20),
            )
            .push(
                Radio::new(
                    Mode::Decrypt,
                    "decrypt",
                    Some(self.selected_mode),
                    Message::ModeSelected,
                )
                .size(20),
            );

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .max_width(600)
            .push(
                Row::new()
                    .spacing(10)
                    .push(Text::new("qpasswd").size(60))
                    .push(radio),
            )
            .push(Row::new().spacing(10).push(pass_input).push(source_input))
            .push(
                Row::new()
                    .spacing(10)
                    .push(crypt_button)
                    .push(Text::new(self.result)),
            );

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::CryptPressed => {
                println!("{:?}", self);
            }
            Message::PassInputChanged(value) => self.pass = value,
            Message::SourceInputChanged(value) => self.source = value,
            Message::ModeSelected(value) => self.selected_mode = value,
        }
    }
}
