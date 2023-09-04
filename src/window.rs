
use std::path::PathBuf;

use iced::{Application, executor, Theme, Command, Length};
use iced::widget::{column, text, Image};
use iced::widget::image::Handle;

pub struct State {
    image_path: PathBuf
}

#[derive(Debug)]
pub enum Messages {}

impl Application for State {
    type Message = Messages;
    type Executor = executor::Default;
    type Flags = PathBuf;
    type Theme = Theme;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            State {
                image_path: flags
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
            let image_path_as_string = &self.image_path.to_str();
            match image_path_as_string {
                Some(path_string) => String::from(*path_string),
                None => String::from("something went wrong!")
            }
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let image = Image::new(Handle::from_path(&self.image_path))
        .content_fit(iced::ContentFit::Fill);
       
        image.height(Length::Fill)
        .width(Length::Fill).into()
    }
}