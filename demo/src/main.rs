use eframe::egui;
use egui::{ThemePreference, Visuals};
use egui_theme_lerp::ThemeAnimator;

#[cfg(target_arch = "wasm32")]
use eframe::{WebOptions, WebRunner};

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    use egui::{vec2, ViewportBuilder};

    let options = eframe::NativeOptions {
        centered: true,
        persist_window: false,
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
    use eframe::wasm_bindgen::JsCast as _;
    let web_options = WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(MainWindow::new(cc)))),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
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
        cc.egui_ctx
            .options_mut(|a| a.theme_preference = ThemePreference::Light);
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
