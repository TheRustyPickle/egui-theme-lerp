use egui::{Color32, Context, Id, Ui, Visuals};

#[allow(clippy::many_single_char_names)]
fn interpolate_color(start: Color32, end: Color32, interpolation: f32) -> Color32 {
    let r = egui::lerp(f32::from(start.r())..=f32::from(end.r()), interpolation) as u8;
    let g = egui::lerp(f32::from(start.g())..=f32::from(end.g()), interpolation) as u8;
    let b = egui::lerp(f32::from(start.b())..=f32::from(end.b()), interpolation) as u8;
    let a = egui::lerp(f32::from(start.a())..=f32::from(end.a()), interpolation) as u8;
    Color32::from_rgba_premultiplied(r, g, b, a)
}

pub struct ThemeAnimator {
    /// egui persistent ID used for animation
    pub anim_id: Option<Id>,
    /// Theme 1 visuals for interpolation
    pub theme_1: Visuals,
    /// Theme 2 visuals for interpolation
    pub theme_2: Visuals,
    /// Value between 0.0 and 1.0 where 1.0 means the animation is completed
    pub progress: f32,
    /// How long the interpolation should take in seconds. Default is 1.0
    pub animation_time: f32,
    /// Whether we are interpolating from Theme 1 to Theme 2
    pub theme_1_to_2: bool,
    /// Whether the interpolation is complete. Setting to `false` will start the interpolation
    pub animation_done: bool,
}

impl ThemeAnimator {
    /// Create a new `ThemeAnimator` without an `anim_id`. This will do nothing if `anim_id` is not set
    /// later
    #[must_use]
    pub const fn new(theme_1: Visuals, theme_2: Visuals) -> Self {
        Self {
            anim_id: None,
            theme_1,
            theme_2,
            progress: 0.0,
            animation_time: 1.0,
            theme_1_to_2: true,
            animation_done: true,
        }
    }

    /// Update the amount of time the animation should take
    pub fn update_animation_time(&mut self, new_time: f32) {
        self.animation_time = new_time;
    }

    /// Use the passed `Id` for `anim_id`
    pub fn set_id(&mut self, ctx: &Context, anim_id: Id) {
        ctx.animate_value_with_time(anim_id, 0.0, 0.0);
        self.anim_id = Some(anim_id);
    }

    /// Set `anim_id` by creating a new `Id` and use that for animation
    pub fn create_id(&mut self, ui: &Ui) {
        let anim_id = ui.make_persistent_id("theme_animator");
        ui.ctx().animate_value_with_time(anim_id, 0.0, 0.0);
        self.anim_id = Some(anim_id);
    }

    /// Start animation
    pub fn start(&mut self) {
        self.animation_done = false;
    }

    /// Linear interpolate the theme animation. Automatically switches animation direction for the
    /// next time once completed. Will do nothing unless `animation_done` is `false` and
    /// `anim_id` is set.
    #[allow(clippy::too_many_lines)] // TODO: Refactor into multiple functions
    pub fn animate(&mut self, ctx: &Context) {
        if self.animation_done {
            return;
        }
        let Some(anim_id) = self.anim_id else { return };

        self.progress = ctx.animate_value_with_time(anim_id, 1.0, self.animation_time);

        let (mut new_visual, start_visual, end_visual) = if self.theme_1_to_2 {
            (self.theme_2.clone(), &self.theme_1, &self.theme_2)
        } else {
            (self.theme_1.clone(), &self.theme_2, &self.theme_1)
        };

        {
            new_visual.widgets.noninteractive.bg_fill = interpolate_color(
                start_visual.widgets.noninteractive.bg_fill,
                end_visual.widgets.noninteractive.bg_fill,
                self.progress,
            );
            new_visual.widgets.noninteractive.weak_bg_fill = interpolate_color(
                start_visual.widgets.noninteractive.weak_bg_fill,
                end_visual.widgets.noninteractive.weak_bg_fill,
                self.progress,
            );
            new_visual.widgets.noninteractive.bg_stroke.color = interpolate_color(
                start_visual.widgets.noninteractive.bg_stroke.color,
                end_visual.widgets.noninteractive.bg_stroke.color,
                self.progress,
            );

            new_visual.widgets.noninteractive.fg_stroke.color = interpolate_color(
                start_visual.widgets.noninteractive.fg_stroke.color,
                end_visual.widgets.noninteractive.fg_stroke.color,
                self.progress,
            );
        }

        {
            new_visual.widgets.inactive.bg_fill = interpolate_color(
                start_visual.widgets.inactive.bg_fill,
                end_visual.widgets.inactive.bg_fill,
                self.progress,
            );
            new_visual.widgets.inactive.weak_bg_fill = interpolate_color(
                start_visual.widgets.inactive.weak_bg_fill,
                end_visual.widgets.inactive.weak_bg_fill,
                self.progress,
            );
            new_visual.widgets.inactive.bg_stroke.color = interpolate_color(
                start_visual.widgets.inactive.bg_stroke.color,
                end_visual.widgets.inactive.bg_stroke.color,
                self.progress,
            );
            new_visual.widgets.inactive.fg_stroke.color = interpolate_color(
                start_visual.widgets.inactive.fg_stroke.color,
                end_visual.widgets.inactive.fg_stroke.color,
                self.progress,
            );
        }

        {
            new_visual.widgets.hovered.bg_fill = interpolate_color(
                start_visual.widgets.hovered.bg_fill,
                end_visual.widgets.hovered.bg_fill,
                self.progress,
            );
            new_visual.widgets.hovered.weak_bg_fill = interpolate_color(
                start_visual.widgets.hovered.weak_bg_fill,
                end_visual.widgets.hovered.weak_bg_fill,
                self.progress,
            );
            new_visual.widgets.hovered.bg_stroke.color = interpolate_color(
                start_visual.widgets.hovered.bg_stroke.color,
                end_visual.widgets.hovered.bg_stroke.color,
                self.progress,
            );
            new_visual.widgets.hovered.fg_stroke.color = interpolate_color(
                start_visual.widgets.hovered.fg_stroke.color,
                end_visual.widgets.hovered.fg_stroke.color,
                self.progress,
            );
        }

        {
            new_visual.widgets.active.bg_fill = interpolate_color(
                start_visual.widgets.active.bg_fill,
                end_visual.widgets.active.bg_fill,
                self.progress,
            );
            new_visual.widgets.active.weak_bg_fill = interpolate_color(
                start_visual.widgets.active.weak_bg_fill,
                end_visual.widgets.active.weak_bg_fill,
                self.progress,
            );
            new_visual.widgets.active.bg_stroke.color = interpolate_color(
                start_visual.widgets.active.bg_stroke.color,
                end_visual.widgets.active.bg_stroke.color,
                self.progress,
            );
            new_visual.widgets.active.fg_stroke.color = interpolate_color(
                start_visual.widgets.active.fg_stroke.color,
                end_visual.widgets.active.fg_stroke.color,
                self.progress,
            );
        }

        {
            new_visual.widgets.open.bg_stroke.color = interpolate_color(
                start_visual.widgets.open.bg_stroke.color,
                end_visual.widgets.open.bg_stroke.color,
                self.progress,
            );
            new_visual.widgets.open.fg_stroke.color = interpolate_color(
                start_visual.widgets.open.fg_stroke.color,
                end_visual.widgets.open.fg_stroke.color,
                self.progress,
            );
            new_visual.widgets.open.bg_fill = interpolate_color(
                start_visual.widgets.open.bg_fill,
                end_visual.widgets.open.bg_fill,
                self.progress,
            );
            new_visual.widgets.open.weak_bg_fill = interpolate_color(
                start_visual.widgets.open.weak_bg_fill,
                end_visual.widgets.open.weak_bg_fill,
                self.progress,
            );
        }

        {
            new_visual.selection.bg_fill = interpolate_color(
                start_visual.selection.bg_fill,
                end_visual.selection.bg_fill,
                self.progress,
            );
            new_visual.selection.stroke.color = interpolate_color(
                start_visual.selection.stroke.color,
                end_visual.selection.stroke.color,
                self.progress,
            );
        }

        new_visual.hyperlink_color = interpolate_color(
            start_visual.hyperlink_color,
            end_visual.hyperlink_color,
            self.progress,
        );

        {
            new_visual.faint_bg_color = interpolate_color(
                start_visual.faint_bg_color,
                end_visual.faint_bg_color,
                self.progress,
            );
            new_visual.extreme_bg_color = interpolate_color(
                start_visual.extreme_bg_color,
                end_visual.extreme_bg_color,
                self.progress,
            );
        }

        new_visual.code_bg_color = interpolate_color(
            start_visual.code_bg_color,
            end_visual.code_bg_color,
            self.progress,
        );

        {
            new_visual.warn_fg_color = interpolate_color(
                start_visual.warn_fg_color,
                end_visual.warn_fg_color,
                self.progress,
            );
            new_visual.error_fg_color = interpolate_color(
                start_visual.error_fg_color,
                end_visual.error_fg_color,
                self.progress,
            );
        }

        {
            new_visual.window_shadow.color = interpolate_color(
                start_visual.window_shadow.color,
                end_visual.window_shadow.color,
                self.progress,
            );

            new_visual.window_fill = interpolate_color(
                start_visual.window_fill,
                end_visual.window_fill,
                self.progress,
            );
            new_visual.window_stroke.color = interpolate_color(
                start_visual.window_stroke.color,
                end_visual.window_stroke.color,
                self.progress,
            );
        }

        new_visual.panel_fill = interpolate_color(
            start_visual.panel_fill,
            end_visual.panel_fill,
            self.progress,
        );

        new_visual.popup_shadow.color = interpolate_color(
            start_visual.popup_shadow.color,
            end_visual.popup_shadow.color,
            self.progress,
        );
        new_visual.text_cursor.stroke.color = interpolate_color(
            start_visual.text_cursor.stroke.color,
            end_visual.text_cursor.stroke.color,
            self.progress,
        );

        ctx.set_visuals(new_visual);

        if self.progress == 1.0 {
            self.animation_done = true;
            self.theme_1_to_2 = !self.theme_1_to_2;
            self.progress = 0.0;
            ctx.animate_value_with_time(anim_id, 0.0, 0.0);
        }
    }
}
