use iced::{scrollable, Column, Container, Element, Image, Length, Scrollable, Text};

#[derive(Default)]
pub struct StorePage {
    scroll: scrollable::State,
}

impl StorePage {
    pub fn new() -> Self {
        Self {
            scroll: scrollable::State::new(),
        }
    }

    pub fn view(&mut self) -> Element<crate::login::Message> {
        let logo = Container::new(
            Image::new("images/logo.png")
                .width(Length::Units(40))
                .height(Length::Units(40)),
        )
        .padding(10);

        let content = Column::new()
            .push(logo)
            .push(Text::new("Welcome to the store page!"));

        let scrollable_content = Scrollable::new(&mut self.scroll)
            .push(content)
            .width(Length::Fill)
            .height(Length::Fill);

        Container::new(scrollable_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
