use egui::{Color32, Context, Id, Ui, Visuals};

#[allow(clippy::many_single_char_names)]
fn interpolate_color(start: Color32, end: Color32, interpolation: f32) -> Color32 {
    let r = egui::lerp(f32::from(start.r())..=f32::from(end.r()), interpolation) as u8;
    let g = egui::lerp(f32::from(start.g())..=f32::from(end.g()), interpolation) as u8;
    let b = egui::lerp(f32::from(start.b())..=f32::from(end.b()), interpolation) as u8;
    let a = egui::lerp(f32::from(start.a())..=f32::from(end.a()), interpolation) as u8;
    Color32::from_rgba_premultiplied(r, g, b, a)
}

/// A structure to manage and animate between two different themes in egui.
///
/// This allows smooth transitions (interpolations) between two sets of visuals (themes) over a
/// specified period.
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
    /// Creates a new `ThemeAnimator` without an assigned `anim_id`.
    ///
    /// This constructor initializes the animator with two themes but leaves the `anim_id` as `None`,
    /// meaning that no animation will take place until the ID is set via `set_id` or `create_id`.
    ///
    /// # Parameters:
    /// - `theme_1`: The starting theme for the animation.
    /// - `theme_2`: The ending theme for the animation.
    ///
    /// # Returns:
    /// A new `ThemeAnimator` with the provided themes and default values for other fields.
    ///
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

    /// Sets the duration of the theme animation.
    ///
    /// This method allows you to configure the total time (in seconds) that the theme transition
    /// should take. The returned `ThemeAnimator` instance will have the updated animation time.
    ///
    /// # Parameters:
    /// - `time`: The new animation duration in seconds.
    ///
    /// # Returns:
    /// A new `ThemeAnimator` instance with the updated animation time.
    #[must_use]
    pub const fn animation_time(mut self, time: f32) -> Self {
        self.animation_time = time;
        self
    }

    /// Updates the animation time.
    ///
    /// This method changes the total duration (in seconds) that the animation will take to
    /// complete. Adjust this based on how fast or slow you want the transition between themes.
    ///
    /// # Parameters:
    /// - `new_time`: The new animation time in seconds.
    ///
    pub fn update_animation_time(&mut self, new_time: f32) {
        self.animation_time = new_time;
    }

    /// Assigns a persistent ID to control the animation.
    ///
    /// This sets the `anim_id` using an existing egui `Id`, allowing egui to track and manage
    /// the animation's state.
    ///
    /// # Parameters:
    /// - `ctx`: The egui `Context` object.
    /// - `anim_id`: The persistent `Id` used for tracking animation progress.
    pub fn set_id(&mut self, ctx: &Context, anim_id: Id) {
        ctx.animate_value_with_time(anim_id, 0.0, 0.0);
        self.anim_id = Some(anim_id);
    }

    /// Creates a new persistent `Id` for the animation.
    ///
    /// This method generates a new persistent `Id` for the animator using the provided `Ui` context.
    /// It is useful when an `Id` is not manually provided and needs to be created on-the-fly.
    ///
    /// # Parameters:
    /// - `ui`: The egui `Ui` object to generate a persistent ID.
    pub fn create_id(&mut self, ui: &Ui) {
        let anim_id = ui.make_persistent_id("theme_animator");
        ui.ctx().animate_value_with_time(anim_id, 0.0, 0.0);
        self.anim_id = Some(anim_id);
    }

    /// Starts the theme animation.
    ///
    /// Sets the `animation_done` flag to `false`, allowing the interpolation to start. Does
    /// nothing if called multiple times while animation is ongoing.
    pub fn start(&mut self) {
        self.animation_done = false;
    }

    /// Performs the animation, interpolating between the two visuals.
    ///
    /// This function progresses the animation if `animation_done` is set to `false` and
    /// an `anim_id` is assigned. Once the animation reaches the end (`progress` reaches 1.0),
    /// it will automatically switch the animation direction (from `theme_1` to `theme_2` or
    /// vice versa) for the next time it is started.
    ///
    /// # Parameters:
    /// - `ctx`: The egui `Context` object used for handling the animation.
    ///
    /// # Notes:
    /// This method does nothing if the animation is already complete (`animation_done = true`)
    /// or if `anim_id` is not set.
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
