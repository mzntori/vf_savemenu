mod errors;
mod saves;

use std::path::PathBuf;
use errors::Error;

use saves::*;

use iced::{Application, Color, Command, Element, Length, Renderer, Settings};
use iced::alignment::{Horizontal, Vertical};
use iced::theme::Text::Color as TextColor;
use iced::widget::{button, container, row, text, Button, Row, Column, Space};
use iced::theme::Theme;
use iced::window;

use open;

struct SaveManager;

impl SaveManager {
    // fn update_button_colors(&mut self) {
    //     let mut colors: Vec<theme::Button> = vec![];
    //
    //     for slot in 0..=8 {
    //         let save = Save::at_slot(slot);
    //         if save.exists() {
    //             colors.push(theme::Button::Primary);
    //         } else {
    //             colors.push(theme::Button::Secondary);
    //         }
    //     }
    //
    //     self.button_colors = colors;
    // }
}

impl Application for SaveManager {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();
    
    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let (mng, cmd) = (
            SaveManager {},
            Command::none()
        );
        
        (mng, cmd)
    }
    
    fn title(&self) -> String {
        String::from("VF Save Menu")
    }
    
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Delete(slot) => {
                let save = Save::at_slot(slot);
                dbg!(&save);
                save.create_recovery_save().unwrap_or_default();
                save.remove_files().unwrap_or_default();
            }
            Message::RemoveAll => {
                for slot in 0..=8 {
                    let save = Save::at_slot(slot);
                    dbg!(&save);
                    if let Ok(()) = save.create_recovery_save() {
                        save.remove_files().unwrap_or_default();
                    }
                }
            }
            Message::OpenPath(buf) => {
                open::that(buf).unwrap_or_default();
            }
            #[allow(unreachable_patterns)]
            _ => {}
        }
        
        Command::none()
    }
    
    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        fn create_button(slot: u8) -> Button<'static, Message, Renderer> {
            button(
                text(format!("{}", slot))
                    .horizontal_alignment(Horizontal::Center)
                    .vertical_alignment(Vertical::Center)
            )
                .height(Length::FillPortion(1))
                .width(Length::FillPortion(1))
                .on_press(Message::Delete(slot))
        }
        
        fn create_row(row: u8) -> Row<'static ,Message, Renderer> {
            row![
                create_button(row*3),
                create_button(row*3 + 1),
                create_button(row*3 + 2),
            ]
                .height(Length::FillPortion(2))
                .width(Length::FillPortion(2))
                .spacing(15)
        }
        
        let content = Column::new()
            .push(create_row(0))
            .push(Space::with_height(15))
            .push(create_row(1))
            .push(Space::with_height(15))
            .push(create_row(2))
            .push(Space::with_height(15))
            .push(
                row![
                    button(text("Remove all")
                        .horizontal_alignment(Horizontal::Center)
                        .vertical_alignment(Vertical::Center)
                    )
                        .width(Length::FillPortion(1))
                        .height(Length::FillPortion(1))
                        .on_press(Message::RemoveAll),
                    button(text("Folder")
                        .horizontal_alignment(Horizontal::Center)
                        .vertical_alignment(Vertical::Center)
                    )
                        .width(Length::FillPortion(1))
                        .height(Length::FillPortion(1))
                        .on_press(Message::OpenPath(vf_root_dir().unwrap_or_default())),
                    
                ]
                    .height(Length::FillPortion(1))
                    .width(Length::FillPortion(1))
                    .spacing(15)
            )
            .push(Space::with_height(7))
            .push(
                text("For questions ping @mzntori on discord.")
                    .size(14)
                    .style(TextColor(Color::from_rgb8(140, 140, 140)))
                    .horizontal_alignment(Horizontal::Center)
                    .width(Length::Fill)
            )
            .spacing(0);
            
        container(content)
            .center_x()
            .center_y()
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
    
    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}

#[derive(Debug, Clone)]
enum Message {
    Delete(u8),
    RemoveAll,
    OpenPath(PathBuf),
}

fn main() -> Result<(), Error> {
    create_vf_menu_dir()?;

    SaveManager::run(Settings {
        window: window::Settings {
            size: (300, 350),
            ..window::Settings::default()
        },
        ..Settings::default()
    })?;
    
    Ok(())
}