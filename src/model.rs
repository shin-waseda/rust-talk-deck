#[derive(Clone, Copy, PartialEq)]
pub enum Screen {
    Menu,
    MenuConfirm,
    Accel,
}

#[derive(Clone, Copy)]
pub struct AccelReading {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy)]
pub struct Model {
    pub screen: Screen,
    pub cursor: usize,
    pub confirm_cursor: usize,
    pub accel: Option<AccelReading>,
    pub accel_offset: (f32, f32, f32),
}

pub enum Event {
    NavigateUp,
    NavigateDown,
    NavigateLeft,
    NavigateRight,
    Select,
    Back,
    AccelSampled(Result<AccelReading, ()>),
}

impl Model {
    pub fn new() -> Self {
        Self {
            screen: Screen::Menu,
            cursor: 0,
            confirm_cursor: 0,
            accel: None,
            accel_offset: (0.0, 0.0, 0.0),
        }
    }

    pub fn update(self, event: Event) -> Self {
        match (self.screen, event) {
            (Screen::Menu, Event::NavigateUp) => Self {
                cursor: if self.cursor == 0 { 4 } else { self.cursor - 1 },
                ..self
            },
            (Screen::Menu, Event::NavigateDown) => Self {
                cursor: (self.cursor + 1) % 5,
                ..self
            },
            (Screen::Menu, Event::Select) => Self {
                screen: Screen::MenuConfirm,
                confirm_cursor: 0,
                ..self
            },

            (Screen::MenuConfirm, Event::NavigateLeft) | (Screen::MenuConfirm, Event::NavigateUp) => Self {
                confirm_cursor: 0,
                ..self
            },
            (Screen::MenuConfirm, Event::NavigateRight) | (Screen::MenuConfirm, Event::NavigateDown) => Self {
                confirm_cursor: 1,
                ..self
            },
            (Screen::MenuConfirm, Event::Select) => {
                if self.confirm_cursor == 0 {
                    if self.cursor == 0 {
                        Self { screen: Screen::Accel, ..self }
                    } else {
                        Self { screen: Screen::Menu, ..self }
                    }
                } else {
                    Self { screen: Screen::Menu, ..self }
                }
            },
            (Screen::MenuConfirm, Event::Back) => Self {
                screen: Screen::Menu,
                ..self
            },

            (Screen::Accel, Event::Back) => Self { screen: Screen::Menu, ..self },
            (Screen::Accel, Event::AccelSampled(Ok(a))) => Self {
                accel: Some(a),
                ..self
            },
            _ => self,
        }
    }
}