#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Screen {
    Menu,
    MenuConfirm,
    Talk(TalkState),
    Status,
    Magic,
    Item,
    Escape,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TalkState {
    WazaMenu,
    Miru,
    TonaeruResult,
    TonaeruConfirm,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AccelReading {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Model {
    pub screen: Screen,
    pub cursor: usize,
    pub confirm_cursor: usize,
    pub waza_cursor: usize,
    pub result_confirm_cursor: usize,
    pub accel: Option<AccelReading>,
    pub accel_offset: (f32, f32, f32),
    pub tonaeru_result: (i32, i32, i32),
    pub message: Option<&'static str>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Event {
    NavigateUp,
    NavigateDown,
    NavigateLeft,
    NavigateRight,
    Select,
    Back,
    AccelSampled(Result<AccelReading, ()>),
    ZeroHadouExecuted((f32, f32, f32)),
    TonaeruCompleted((i32, i32, i32)),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Effect {
    /// ぜろのはどう実行：加速度基準値を更新し、メッセージを一時表示して遅延
    ExecuteZeroHadou,
    /// となえる実行：サンプリング開始
    ExecuteTonaeru,
    /// にげる実行：エスケープ画面表示して遅延
    ExecuteEscape,
    /// システムリセット（メインメニューの「にげる」）
    SystemReset,
    /// メッセージクリア
    ClearMessage,
}

pub struct UpdateResult {
    pub model: Model,
    pub effect: Option<Effect>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            screen: Screen::Menu,
            cursor: 0,
            confirm_cursor: 0,
            waza_cursor: 0,
            result_confirm_cursor: 0,
            accel: None,
            accel_offset: (0.0, 0.0, 0.0),
            tonaeru_result: (0, 0, 0),
            message: None,
        }
    }

    pub fn update(self, event: Event) -> UpdateResult {
        let (model, effect) = match (self.screen, event) {
            // --- メインメニュー (Screen::Menu) ---
            (Screen::Menu, Event::NavigateUp) => (Self {
                cursor: if self.cursor == 0 { 4 } else { self.cursor - 1 },
                ..self
            }, None),
            (Screen::Menu, Event::NavigateDown) => (Self {
                cursor: (self.cursor + 1) % 5,
                ..self
            }, None),
            (Screen::Menu, Event::Select) => (Self {
                screen: Screen::MenuConfirm,
                confirm_cursor: 0,
                ..self
            }, None),

            // --- 確認ダイアログ (Screen::MenuConfirm) ---
            (Screen::MenuConfirm, Event::NavigateLeft) | (Screen::MenuConfirm, Event::NavigateUp) => (Self {
                confirm_cursor: 0,
                ..self
            }, None),
            (Screen::MenuConfirm, Event::NavigateRight) | (Screen::MenuConfirm, Event::NavigateDown) => (Self {
                confirm_cursor: 1,
                ..self
            }, None),
            (Screen::MenuConfirm, Event::Select) => {
                if self.confirm_cursor == 0 {
                    match self.cursor {
                        0 => (Self { screen: Screen::Talk(TalkState::WazaMenu), waza_cursor: 0, ..self }, None),
                        1 => (Self { screen: Screen::Status, ..self }, None),
                        2 => (Self { screen: Screen::Magic, ..self }, None),
                        3 => (Self { screen: Screen::Item, ..self }, None),
                        _ => (
                            Self { screen: Screen::Escape, ..self },
                            Some(Effect::SystemReset), // ★ メインメニューからの逃走はシステムリセット
                        ),
                    }
                } else {
                    (Self { screen: Screen::Menu, ..self }, None)
                }
            },
            (Screen::MenuConfirm, Event::Back) => (Self {
                screen: Screen::Menu,
                ..self
            }, None),

            // --- わざメニュー (Screen::Talk(TalkState::WazaMenu)) ---
            (Screen::Talk(TalkState::WazaMenu), Event::NavigateUp) | (Screen::Talk(TalkState::WazaMenu), Event::NavigateLeft) => (Self {
                waza_cursor: if self.waza_cursor == 0 { 3 } else { self.waza_cursor - 1 },
                ..self
            }, None),
            (Screen::Talk(TalkState::WazaMenu), Event::NavigateDown) | (Screen::Talk(TalkState::WazaMenu), Event::NavigateRight) => (Self {
                waza_cursor: (self.waza_cursor + 1) % 4,
                ..self
            }, None),
            (Screen::Talk(TalkState::WazaMenu), Event::Select) => match self.waza_cursor {
                0 => (self, Some(Effect::ExecuteZeroHadou)),
                1 => (self, Some(Effect::ExecuteTonaeru)),
                2 => (Self {
                    screen: Screen::Talk(TalkState::Miru),
                    ..self
                }, None),
                3 => (self, Some(Effect::ExecuteEscape)), // ★ わざメニューからの逃走は ExecuteEscape
                _ => (self, None),
            },
            (Screen::Talk(TalkState::WazaMenu), Event::ZeroHadouExecuted(offset)) => (Self {
                accel_offset: offset,
                message: Some("きじゅんち こうしん！"),
                ..self
            }, None), // ★ Effect::ClearMessage を削除
            (Screen::Talk(TalkState::WazaMenu), Event::TonaeruCompleted(result)) => (Self {
                screen: Screen::Talk(TalkState::TonaeruConfirm),
                tonaeru_result: result,
                result_confirm_cursor: 0,
                ..self
            }, None),
            (Screen::Talk(TalkState::TonaeruResult), Event::Select) => (Self {
                screen: Screen::Talk(TalkState::TonaeruConfirm),
                result_confirm_cursor: 0,
                ..self
            }, None),

            // --- となえる確認 (Screen::Talk(TalkState::TonaeruConfirm)) ---
            (Screen::Talk(TalkState::TonaeruConfirm), Event::NavigateLeft) | (Screen::Talk(TalkState::TonaeruConfirm), Event::NavigateUp) => (Self {
                result_confirm_cursor: 0,
                ..self
            }, None),
            (Screen::Talk(TalkState::TonaeruConfirm), Event::NavigateRight) | (Screen::Talk(TalkState::TonaeruConfirm), Event::NavigateDown) => (Self {
                result_confirm_cursor: 1,
                ..self
            }, None),
            (Screen::Talk(TalkState::TonaeruConfirm), Event::Select) => {
                if self.result_confirm_cursor == 0 {
                    // もどる：技メニューへ戻る際にメッセージを消去
                    (Self { screen: Screen::Talk(TalkState::WazaMenu), message: None, ..self }, None)
                } else {
                    // とどまる
                    (Self { screen: Screen::Talk(TalkState::TonaeruResult), ..self }, None)
                }
            },
            (Screen::Talk(talk_state), Event::Back) => match talk_state {
                // わざメニューにいる時はメインメニューに戻る
                TalkState::WazaMenu => (Self {
                    screen: Screen::Menu,
                    message: None,
                    ..self
                }, None),
                // 「みる」や「結果表示」などにいる時はわざメニューに戻る
                TalkState::Miru | TalkState::TonaeruResult | TalkState::TonaeruConfirm => (Self {
                    screen: Screen::Talk(TalkState::WazaMenu),
                    message: None,
                    ..self
                }, None),
            },

            // --- キャンセル (Back ボタン) による共通の戻り遷移 ---
            (Screen::Status, Event::Back)
            | (Screen::Magic, Event::Back)
            | (Screen::Item, Event::Back)
            | (Screen::Escape, Event::Back) => (Self {
                screen: Screen::Menu,
                message: None,
                ..self
            }, None),

            // センサー値の更新など
            (_, Event::AccelSampled(Ok(a))) => (Self {
                accel: Some(a),
                ..self
            }, None),

            _ => (self, None),
        };

        UpdateResult { model, effect }
    }
}