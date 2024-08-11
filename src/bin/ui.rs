use encrypt_static_html::encrypt_html;
use iced::widget::{button, column, text, text_input, vertical_space};
use iced::{window, Sandbox, Settings};
use std::path::Path;

#[derive(Default)]
struct MyApp {
    src: String,
    dst: String,
    password: String,
    message: String,
    title: String,
    message_box: Option<String>,
}

#[derive(Debug, Clone)]
enum UIMessage {
    ChangeSrc(String),
    ChangeDst(String),
    ChangePassword(String),
    ChangeMessage(String),
    ChangeTitle(String),
    ClickEncrypt,
    CloseMessageBox,
}

impl MyApp {
    fn encrypt(&mut self) {
        let status = encrypt_html(
            Path::new(&self.src),
            &self.password,
            Path::new(&self.dst),
            true,
            true,
            &self.message,
            &self.title,
        );
        if let Err(e) = status {
            self.message_box = Some(format!("Error: {}", e));
        } else {
            self.message_box = Some("HTML file encrypted successfully!".into())
        }
    }
}

impl Sandbox for MyApp {
    type Message = UIMessage;

    fn new() -> Self {
        Self {
            src: String::from("index.html"),
            dst: String::from("index_encrypted.html"),
            password: String::from("1234"),
            message: String::from("Enter the password to view the page"),
            title: String::from("My webpage"),
            message_box: None,
        }
    }

    fn title(&self) -> String {
        String::from("Encrypt Static HTML UI")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            UIMessage::ChangeSrc(v) => self.src = v,
            UIMessage::ChangeDst(v) => self.dst = v,
            UIMessage::ChangePassword(v) => self.password = v,
            UIMessage::ChangeMessage(v) => self.message = v,
            UIMessage::ChangeTitle(v) => self.title = v,
            UIMessage::ClickEncrypt => self.encrypt(),
            UIMessage::CloseMessageBox => self.message_box = None,
        }
    }

    fn view(&self) -> iced::Element<Self::Message> {
        if self.message_box.is_some() {
            column![
                text(self.message_box.as_ref().unwrap()),
                button("Ok").on_press(UIMessage::CloseMessageBox),
            ]
            .into()
        } else {
            column![
                text("Path to input html file to encrypt"),
                text_input("", self.src.as_str()).on_input(UIMessage::ChangeSrc),
                vertical_space().height(10),
                text("Path to output encrypted html file"),
                text_input("", self.dst.as_str()).on_input(UIMessage::ChangeDst),
                vertical_space().height(10),
                text("Encryption password"),
                text_input("", self.password.as_str()).on_input(UIMessage::ChangePassword),
                vertical_space().height(10),
                text("Password request text"),
                text_input("", self.message.as_str()).on_input(UIMessage::ChangeMessage),
                vertical_space().height(10),
                text("Title of the password request page"),
                text_input("", self.title.as_str()).on_input(UIMessage::ChangeTitle),
                vertical_space().height(10),
                button("Encrypt")
                    .on_press(UIMessage::ClickEncrypt)
                    .padding(5),
            ]
            .padding(20)
            .into()
        }
    }
}

fn main() {
    MyApp::run(Settings {
        window: window::Settings {
            size: iced::Size::new(600., 400.),
            resizable: true,
            decorations: true,
            ..Default::default()
        },
        ..Default::default()
    })
    .unwrap();
}
