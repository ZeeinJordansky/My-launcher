use eframe::egui::{self, Color32, Rounding, Stroke, Vec2};

pub const ACCENT: Color32 = Color32::from_rgb(0, 200, 150);
pub const ACCENT_HOVER: Color32 = Color32::from_rgb(0, 230, 170);
pub const BG_DARK: Color32 = Color32::from_rgb(18, 18, 24);
pub const BG_CARD: Color32 = Color32::from_rgb(28, 28, 36);
pub const BG_SIDEBAR: Color32 = Color32::from_rgb(22, 22, 30);
pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(240, 240, 245);
pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(160, 160, 175);
pub const SUCCESS: Color32 = Color32::from_rgb(80, 200, 120);
pub const WARNING: Color32 = Color32::from_rgb(240, 180, 50);
pub const ERROR: Color32 = Color32::from_rgb(230, 70, 70);

pub fn setup_custom_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.visuals.window_rounding = Rounding::same(12.0);
    style.visuals.menu_rounding = Rounding::same(8.0);
    style.visuals.widgets.noninteractive.rounding = Rounding::same(8.0);
    style.visuals.widgets.inactive.rounding = Rounding::same(8.0);
    style.visuals.widgets.hovered.rounding = Rounding::same(8.0);
    style.visuals.widgets.active.rounding = Rounding::same(8.0);
    style.visuals.widgets.open.rounding = Rounding::same(8.0);
    style.visuals.dark_mode = true;
    style.visuals.panel_fill = BG_DARK;
    style.visuals.window_fill = BG_CARD;
    style.visuals.extreme_bg_color = BG_SIDEBAR;
    style.visuals.faint_bg_color = Color32::from_rgb(35, 35, 45);
    style.visuals.widgets.noninteractive.bg_fill = BG_CARD;
    style.visuals.widgets.inactive.bg_fill = Color32::from_rgb(40, 40, 52);
    style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(50, 50, 65);
    style.visuals.widgets.active.bg_fill = ACCENT;
    style.visuals.selection.bg_fill = ACCENT.linear_multiply(0.35);
    style.visuals.hyperlink_color = ACCENT;
    style.visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, TEXT_SECONDARY);
    style.visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, TEXT_PRIMARY);
    style.visuals.widgets.hovered.fg_stroke = Stroke::new(1.5, TEXT_PRIMARY);
    style.visuals.widgets.active.fg_stroke = Stroke::new(1.5, Color32::BLACK);
    style.spacing.item_spacing = Vec2::new(10.0, 8.0);
    style.spacing.button_padding = Vec2::new(14.0, 8.0);
    ctx.set_style(style);
}

pub fn card_frame() -> egui::Frame {
    egui::Frame::none()
        .fill(BG_CARD)
        .rounding(Rounding::same(12.0))
        .inner_margin(egui::Margin::same(16.0))
        .stroke(Stroke::new(1.0, Color32::from_rgb(45, 45, 60)))
}

pub fn accent_button(ui: &mut egui::Ui, text: &str) -> egui::Response {
    ui.add(
        egui::Button::new(egui::RichText::new(text).color(Color32::BLACK).strong())
            .fill(ACCENT)
            .rounding(Rounding::same(8.0))
            .min_size(Vec2::new(120.0, 36.0)),
    )
}
