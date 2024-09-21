# egui-theme-lerp
<a href="https://crates.io/crates/egui-theme-lerp"><img src="https://img.shields.io/crates/v/egui-theme-lerp.svg?style=flat-square&logo=rust&color=orange" alt="Crates version"/></a>
<a href="https://crates.io/crates/egui-theme-lerp"><img src="https://img.shields.io/crates/d/egui-theme-lerp?style=flat-square" alt="Downloads"/></a>

A simple library for [egui](https://github.com/emilk/egui) to smoothly animate theme transitions by interpolating between any two visuals/themes. It works with any set of visuals, allowing transitions between light/dark modes or custom themes.

[](https://github.com/user-attachments/assets/5ea94394-f60b-4b62-bd3f-38497d89b984)

# Installation

Add the following to your `Cargo.toml`:
```toml
[dependencies]
egui-theme-lerp = "0.1.0"
```

# Usage

```rust
use egui_theme_lerp::ThemeAnimator;
use egui::Visuals;

pub struct MainWindow {
    theme_animator: ThemeAnimator,
}

impl MainWindow {
    pub fn new() -> Self {
        Self {
            theme_animator: ThemeAnimator::new(Visuals::light(), Visuals::dark()),
        }
    }
}


impl eframe::App for MainWindow {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.theme_animator.anim_id.is_none() {
                self.theme_animator.create_id(ui);
            } else {
                self.theme_animator.animate(ctx);
            }

            ui.vertical_centered(|ui| {
                if ui.button("Switch Theme".to_string()).clicked() {
                    self.theme_animator.start();
                }
            });
        });
    }
}

```

# Run Demo

The demo is accessible online via [this link](https://therustypickle.github.io/egui-theme-lerp/)

- Clone the repository `git clone https://github.com/TheRustyPickle/egui-theme-lerp`
- Move into the demo folder `cd egui-theme-lerp/demo`
    - To run natively `cargo run --release` 

    or

    - To run in wasm locally install the required target with `rustup target add wasm32-unknown-unknown`
    - Install Trunk with `cargo install --locked trunk`
    - `trunk serve` to run and visit `http://127.0.0.1:8080/`

# Contributing

Contributions, issues, and feature requests are welcome! If you'd like to contribute, please open a pull request.

# License

This project is licensed under the MIT License.
