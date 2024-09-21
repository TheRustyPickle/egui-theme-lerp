use egui::Visuals;
use egui_theme_lerp::ThemeAnimator;

#[cfg(target_arch = "wasm32")]
use eframe::{WebLogger, WebOptions, WebRunner};

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    use egui::{vec2, ViewportBuilder};

    let options = eframe::NativeOptions {
        centered: true,
        persist_window: false,
        default_theme: eframe::Theme::Light,
        viewport: ViewportBuilder {
            inner_size: Some(vec2(400.0, 400.0)),

            ..Default::default()
        },
        ..Default::default()
    };

    eframe::run_native(
        "Theme Lerp Demo",
        options,
        Box::new(|cc| Ok(Box::new(MainWindow::new(cc)))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    let web_options = WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let start_result = WebRunner::new()
            .start(
                "the_canvas_id",
                web_options,
                Box::new(|cc| Ok(Box::new(MainWindow::new(cc)))),
            )
            .await;

        // Remove the loading text and spinner:
        let loading_text = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("loading_text"));
        if let Some(loading_text) = loading_text {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}

pub struct MainWindow {
    theme_animator: ThemeAnimator,
}

impl MainWindow {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_pixels_per_point(1.2);
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

            let theme_emoji = if !self.theme_animator.animation_done {
                if self.theme_animator.theme_1_to_2 {
                    "☀"
                } else {
                    "🌙"
                }
            } else if self.theme_animator.theme_1_to_2 {
                "🌙"
            } else {
                "☀"
            };

            ui.vertical_centered(|ui| {
                if ui.button(format!("Switch Theme {theme_emoji}")).clicked() {
                    self.theme_animator.start();
                }
            });
        });
    }
}
