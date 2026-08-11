use embedded_hal::digital::v2::InputPin;

/// 5-wayスイッチの方向
#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Press,
}

/// 上部ボタン (A=右, B=真ん中, C=左)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonId {
    A,
    B,
    C,
}

/// 5-wayスイッチのラッパー
pub struct FiveWaySwitch<X, Y, Z, U, B> {
    down: X,
    right: Y,
    press: Z,
    up: U,
    left: B,
}

impl<X, Y, Z, U, B> FiveWaySwitch<X, Y, Z, U, B>
where
    X: InputPin,
    X::Error: core::fmt::Debug,
    Y: InputPin,
    Y::Error: core::fmt::Debug,
    Z: InputPin,
    Z::Error: core::fmt::Debug,
    U: InputPin,
    U::Error: core::fmt::Debug,
    B: InputPin,
    B::Error: core::fmt::Debug,
{
    pub fn new(switch_x: X, switch_y: Y, switch_z: Z, switch_u: U, switch_b: B) -> Self {
        Self {
            down: switch_x,
            right: switch_y,
            press: switch_z,
            up: switch_u,
            left: switch_b,
        }
    }

    pub fn read(&self) -> Option<Direction> {
        if self.up.is_low().unwrap() {
            Some(Direction::Up)
        } else if self.down.is_low().unwrap() {
            Some(Direction::Down)
        } else if self.left.is_low().unwrap() {
            Some(Direction::Left)
        } else if self.right.is_low().unwrap() {
            Some(Direction::Right)
        } else if self.press.is_low().unwrap() {
            Some(Direction::Press)
        } else {
            None
        }
    }

    pub fn is_pressed(&self, last_direction: &mut Option<Direction>) -> Option<Direction> {
        let current_direction = self.read();
        let pressed_direction = if current_direction.is_some() && last_direction.is_none() {
            current_direction
        } else {
            None
        };
        *last_direction = current_direction;
        pressed_direction
    }
}

/// 上部ボタンA/B/Cのラッパー
pub struct TopButtons<A, Bt, C> {
    a: A,
    b: Bt,
    c: C,
}

impl<A, Bt, C> TopButtons<A, Bt, C>
where
    A: InputPin,
    A::Error: core::fmt::Debug,
    Bt: InputPin,
    Bt::Error: core::fmt::Debug,
    C: InputPin,
    C::Error: core::fmt::Debug,
{
    pub fn new(button1: A, button2: Bt, button3: C) -> Self {
        Self {
            a: button1,
            b: button2,
            c: button3,
        }
    }

    pub fn read(&self) -> Option<ButtonId> {
        if self.a.is_low().unwrap() {
            Some(ButtonId::A)
        } else if self.b.is_low().unwrap() {
            Some(ButtonId::B)
        } else if self.c.is_low().unwrap() {
            Some(ButtonId::C)
        } else {
            None
        }
    }

    pub fn is_pressed(&self, last_button: &mut Option<ButtonId>) -> Option<ButtonId> {
        let current_button = self.read();
        let pressed_button = if current_button.is_some() && last_button.is_none() {
            current_button
        } else {
            None
        };
        *last_button = current_button;
        pressed_button
    }
}