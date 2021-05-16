use iced::{Button, Column, Container, Element, Length, Row, Sandbox, Text, TextInput, button, text_input};

#[derive(Default, Debug)]
pub struct Application {
    pass: String,
    source: String,

    pass_input: text_input::State,
    source_input: text_input::State,

    crypt_button: button::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    CryptPressed,
    PassInputChanged(String),
    SourceInputChanged(String),
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

        let crypt_button = Button::new(&mut self.crypt_button, Text::new("crypt"))
            .on_press(Message::CryptPressed);

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .max_width(600)
            .push(Text::new("qpasswd").size(60))
            .push(Row::new().spacing(10).push(pass_input).push(source_input))
            .push(crypt_button);

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
            },
            Message::PassInputChanged(value) => self.pass = value,
            Message::SourceInputChanged(value) => self.source = value,

        }
    }
}
