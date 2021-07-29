/// Widget Style
pub mod widget {
    use iced::{container, Background, Color};

    pub const SURFACE: Color = Color::from_rgb(
        0x54 as f32 / 255.0,
        0x49 as f32 / 255.0,
        0x4B as f32 / 255.0,
    );

    pub const FRONT: Color = Color::from_rgb(
        126.0 as f32 / 255.0,
        130.0 as f32 / 255.0,
        135.0 as f32 / 255.0,
    );

    pub struct Pane {
        pub is_focused: bool,
    }

    pub struct Container {
        
    }

    impl container::StyleSheet for Pane {
        fn style(&self) -> container::Style {
            container::Style {
                background: Some(Background::Color(SURFACE)),
                border_width: 2.0,
                border_color: if self.is_focused {
                    Color::BLACK
                } else {
                    Color::from_rgb(0.7, 0.7, 0.7)
                },
                ..Default::default()
            }
        }
    }

    impl container::StyleSheet for Container {
        fn style(&self) -> container::Style {
            container::Style {
                background: Some(Background::Color(FRONT)),
                ..Default::default()
            }
        }
    }
}
