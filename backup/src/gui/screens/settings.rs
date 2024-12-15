use iced::{
    widget::{container, Column, Row, text, pick_list},
    Element, Length,
};
use crate::gui::types::{Message, SettingsArgs, SettingsTab};
use crate::storage::ThemeType;

pub struct SettingsScreen {
    args: SettingsArgs,
}

impl SettingsScreen {
    pub fn new(args: SettingsArgs) -> Self {
        Self { args }
    }

    fn tab_button<'a>(&self, tab: SettingsTab, label: &str) -> Element<'a, Message> {
        let style = if self.args.active_tab == tab {
            iced::theme::Button::Primary
        } else {
            iced::theme::Button::Secondary
        };

        iced::widget::button(text(label))
            .style(style)
            .on_press(Message::SettingsTabChanged(tab))
            .into()
    }

    pub fn view<'a>(&self) -> Element<'a, Message> {
        let tabs: Element<'a, Message> = Row::new()
            .spacing(20)
            .push(self.tab_button(SettingsTab::General, "General"))
            .push(self.tab_button(SettingsTab::Appearance, "Appearance"))
            .push(self.tab_button(SettingsTab::Playback, "Playback"))
            .into();

        let content: Element<'a, Message> = match self.args.active_tab {
            SettingsTab::Appearance => {
                let theme_options = vec![ThemeType::Light, ThemeType::Dark];
                let theme_picker = pick_list(
                    theme_options,
                    Some(self.args.current_theme),
                    Message::ThemeChanged,
                );

                Column::new()
                    .spacing(20)
                    .push(text("Theme:"))
                    .push(theme_picker)
                    .into()
            }
            _ => Column::new()
                .push(text("Other settings coming soon..."))
                .into()
        };

        let main_content: Element<'a, Message> = Column::new()
            .spacing(20)
            .padding(20)
            .push(tabs)
            .push(content)
            .into();

        container(main_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
} 