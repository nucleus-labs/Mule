
mod widgets;

use peacock::api::{Application, MessageGeneric, MessageButton};

#[derive(Default)]
struct Mule;

fn button_press(_state: &mut Mule, message: MessageGeneric) -> iced::Task<MessageGeneric> {
    match message {
        MessageGeneric::Button(id, _) => println!("Button with id '{id}' was pressed!"),
    }
    ().into()
}

fn main() -> iced::Result {
    let title: &'static str = concat!("Mule V", env!("CARGO_PKG_VERSION"));
    let mut app = Application::<Mule>::new(title, widgets::gen_index);
    
    app.add_callback(MessageGeneric::Button("hwllo".into(), MessageButton::Pressed), button_press);
    app.run()
}
