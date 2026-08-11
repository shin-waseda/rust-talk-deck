# Wio Terminal + Rust テンプレート

Wio Terminal（ATSAMD51P19A）向けの Rust 開発テンプレートです。

## 🛠 セットアップ（初回のみ）

### ツールチェーン・依存関係

```bash
cargo install cargo-hf2
sudo apt install -y pkg-config libusb-1.0-0-dev libudev-dev
```

`rust-toolchain.toml` があれば、`thumbv7em-none-eabihf` ターゲットは自動で利用されます。

うまく追加されない場合は手動で実行してください。

```bash
rustup target add thumbv7em-none-eabihf
```

### udev ルール（Ubuntu 等）

root 権限なしで USB 書き込みを行うため、`/etc/udev/rules.d/99-wio-terminal.rules` を作成します。

```text
ATTRS{idVendor}=="2886", ATTRS{idProduct}=="002d", ENV{ID_MM_DEVICE_IGNORE}="1"
SUBSYSTEM=="usb", ATTRS{idVendor}=="2886", ATTRS{idProduct}=="002d", MODE="0666"
SUBSYSTEM=="tty", ATTRS{idVendor}=="2886", ATTRS{idProduct}=="002d", MODE="0666"
```

反映します。

```bash
sudo udevadm control --reload-rules && sudo udevadm trigger
```

---

## 🚀 ビルド & 書き込み

本プロジェクトでは `.cargo/config.toml` に `cargo flash` エイリアスが定義されています。

```bash
cargo flash
```

### 書き込み手順

1. Wio Terminal を UF2 Bootloader モードにします（電源スイッチまたはリセットボタンを素早く2回操作）。
2. 青色 LED がゆっくり点滅していることを確認します。
3. `cargo flash` を実行します。

> **補足**
>
> `cargo-hf2` は Cargo の `runner`（`cargo run`）を想定した設計ではなく、ビルド済みバイナリのパスを受け取るインターフェースを提供していません。
>
> そのため `cargo run` は利用せず、`.cargo/config.toml` のエイリアス経由で `cargo flash` を使用してください。

---

## 📁 プロジェクト構成

```text
.
├── .cargo/
│   └── config.toml      # ターゲット・リンカ設定、cargo flash エイリアス
├── src/
│   ├── board.rs         # ハードウェア初期化
│   ├── controls.rs      # ボタン・5-way スイッチ
│   └── main.rs          # アプリケーション本体
├── Cargo.toml
└── README.md
```

---

## 🎮 5-way スイッチ / ボタンのマッピング（実機確認済み）

### 5-way スイッチ

| BSP フィールド  | 実際の方向 |
| ---------- | ----- |
| `switch_x` | 下     |
| `switch_y` | 右     |
| `switch_z` | 押し込み  |
| `switch_u` | 上     |
| `switch_b` | 左     |

### 上部ボタン

| BSP フィールド | 物理位置 |
| --------- | ---- |
| `button1` | 右    |
| `button2` | 真ん中  |
| `button3` | 左    |

入力は `buttons` 構造体を経由してアクセスします。

```rust
board.buttons.switch_x.is_low();
board.buttons.button1.is_low();
```

---

## ⚠️ ハマりどころ

### `cargo run` は使えない

`cargo-hf2` は `runner` 経由の実行に対応していないため、

```bash
cargo run
```

ではなく、

```bash
cargo flash
```

を使用してください。

---

### リンカ設定

`.cargo/config.toml` の

```toml
rustflags = ["-C", "link-arg=-Tlink.x"]
```

が無いとホスト向けリンクにフォールバックし、`InvalidBinary` などのエラーになります。

---

### パニックハンドラ

`Cargo.toml` の両プロファイルに

```toml
panic = "abort"
```

が必要です。

無い場合は

```text
unwinding panics are not supported without std
```

でビルドが失敗します。

---

### `cortex-m-rt`

`0.7.1` および `0.7.2` には既知のスタックアライメントバグがあります。

```bash
cargo tree | grep cortex-m-rt
```

で `0.7.3` 以降になっていることを確認してください。

---

### `embedded-graphics`

`wio_terminal` クレートが利用しているバージョンと一致させる必要があります。

現在は

```toml
embedded-graphics = "0.8"
```

で動作確認済みです。

---

## 💡 開発フロー

```bash
# ビルド確認
cargo check

# 実機へ書き込み
cargo flash
```

Bootloader モード（青色 LED 点滅）で `cargo flash` を実行するだけで、ビルドから書き込みまで完了します。

