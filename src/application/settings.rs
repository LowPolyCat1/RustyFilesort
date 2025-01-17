use iced::{
    self,
    window::{
        settings::PlatformSpecific,
        // Icon
    },
};
// use image;

pub fn settings() -> iced::Settings {
    iced::Settings::default()
}

pub fn window_settings() -> iced::window::Settings {
    // static ICON: &[u8] = include_bytes!("../.././images/icon.ico");
    // const ICON_HEIGHT: u32 = 64;
    // const ICON_WIDTH: u32 = 64;

    // let image = image::load_from_memory(ICON).unwrap();
    // let icon =
    //     iced::window::Icon::from_file(image.as_bytes().to_vec(), ICON_HEIGHT, ICON_WIDTH).unwrap();

    iced::window::Settings {
        size: iced::Size {
            width: 1080.0,
            height: 720.0,
        },
        position: iced::window::Position::Centered,
        min_size: Some(iced::Size {
            width: 640.0,
            height: 640.0,
        }),
        max_size: None,
        visible: true,
        resizable: true,
        decorations: true,
        transparent: false,
        level: iced::window::Level::Normal,
        icon: None,
        platform_specific: PlatformSpecific::default(),
        exit_on_close_request: true,
    }
}
