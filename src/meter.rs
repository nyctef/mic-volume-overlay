use eframe::egui;

fn meter_ui(ui: &mut egui::Ui, amount: f32) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(1.0, 4.0);
    let (rect, response) = ui.allocate_at_least(desired_size, egui::Sense::click_and_drag());

    let amount = amount.clamp(0.0, 1.0);
    // TODO: this probably isn't the exact right calculation,
    // but let's try to make the meter behave more linearly?
    let adjusted_amount = (amount.log10() + 3.0) / 4.0;

    if ui.is_rect_visible(rect) {
        let visuals = ui.style().interact(&response);
        ui.painter().rect_filled(rect, 0.0, visuals.bg_fill);

        let mut inner_rect = rect.shrink(3.0);
        let level = egui::lerp(rect.bottom()..=rect.top(), adjusted_amount);
        inner_rect.set_top(level);
        ui.painter()
            .rect_filled(inner_rect, 0.0, visuals.text_color());

        // dbg!(rect.top(), inner_rect.top());
    }

    response
}

pub fn meter(amount: f32) -> impl egui::Widget {
    move |ui: &mut egui::Ui| meter_ui(ui, amount)
}
