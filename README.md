# egui-theme-lerp

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
    pub fn new( -> Self {
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

# Contributing

Contributions, issues, and feature requests are welcome! If you'd like to contribute, please open a pull request.

# License

This project is licensed under the MIT License.
