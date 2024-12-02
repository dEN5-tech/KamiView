use iced::widget::{container, image::{Handle, Image as IcedImage}};
use iced::{ContentFit, Element, Length, theme, Command};
use crate::Message;
use crate::theme::Theme;
use crate::styles::CustomContainer;
use crate::resources;

pub struct ImageCard;

#[derive(Debug, Clone)]
pub struct ImageCardProps {
    pub url: String,
    pub width: Length,
    pub height: Length,
    pub content_fit: ContentFit,
}

impl Default for ImageCardProps {
    fn default() -> Self {
        Self {
            url: String::new(),
            width: Length::Fill,
            height: Length::Fill,
            content_fit: ContentFit::Contain,
        }
    }
}

fn get_loading_handle() -> Handle {
    Handle::from_memory(
        resources::get_svg("loading.svg")
            .unwrap_or_default()
            .as_bytes()
            .to_vec()
    )
}

fn get_no_image_handle() -> Handle {
    Handle::from_memory(
        resources::get_svg("no_image.svg")
            .unwrap_or_default()
            .as_bytes()
            .to_vec()
    )
}

impl ImageCard {
    pub fn load_remote_image(url: String) -> Command<Message> {
        Command::perform(
            async move {
                match resources::fetch_image(&url).await {
                    Ok(bytes) => Message::ImageLoaded(url, bytes),
                    Err(_) => Message::ImageLoadFailed(url),
                }
            },
            |msg| msg
        )
    }

    pub fn view<'a>(props: ImageCardProps, theme: &Theme) -> (Element<'a, Message>, Option<Command<Message>>) {
        let ImageCardProps { url, width, height, content_fit } = props;

        let (image_handle, command) = if url.is_empty() {
            (get_no_image_handle(), None)
        } else if url.starts_with("http") {
            (get_loading_handle(), Some(Self::load_remote_image(url.clone())))
        } else {
            (Handle::from_path(url), None)
        };

        let element = container(
            IcedImage::new(image_handle)
                .width(width)
                .height(height)
                .content_fit(content_fit)
        )
        .style(theme::Container::Custom(Box::new(CustomContainer {
            color: theme.surface,
            text_color: None,
            hover_color: None,
        })))
        .padding(10)
        .into();

        (element, command)
    }
}