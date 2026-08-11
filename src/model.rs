#[derive(Clone, Copy, PartialEq)]
pub enum Screen {
    Menu,
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
    pub accel: Option<AccelReading>,
    pub accel_offset: (f32, f32, f32),
}

/// ハードから来た生の入力ではなく、意味のある「出来事」
pub enum Event {
    NavigateUp,
    NavigateDown,
    Select,
    Back,
    AccelSampled(Result<AccelReading, ()>),
}

impl Model {
    pub fn new() -> Self {
        Self { screen: Screen::Menu, cursor: 0, accel: None, accel_offset: (0.0, 0.0, 0.0) }
    }

    /// 副作用ゼロ。ピンもディスプレイも一切知らない、ただの状態遷移
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
            (Screen::Menu, Event::Select) if self.cursor == 0 => Self {
                screen: Screen::Accel,
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