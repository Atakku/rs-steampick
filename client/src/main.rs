// Copyright 2025 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use eframe::egui::{self, ViewportCommand};
use steampick::App;
use atklib::*;

fn main() -> eframe::Result {
  let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default()
      .with_always_on_top()
      .with_inner_size([320.0, 240.0])
      .with_min_inner_size([320.0, 240.0])
      .with_max_inner_size([320.0, 240.0])
      .with_decorations(false)
      .with_transparent(true),
    ..Default::default()
  };
  let h = reqwest::blocking::get("https://steampick.atakku.dev").unwrap().json::<Vec<App>>().unwrap();
  let mut h: Vec<App> = h.filter(|i| i.review_count >= 750);
  h.sort_by_key(|i| i.review_percent);
  h.reverse();
  eframe::run_native("SteamPick", options, Box::new(|cc| Ok(Box::new(AppState { count: 0, apps: h }))))
}

struct AppState {
  pub count: usize,
  pub apps: Vec<App>,
}

fn custom_window_frame(ctx: &egui::Context, title: &str, add_contents: impl FnOnce(&mut egui::Ui)) {
  use egui::{CentralPanel, UiBuilder};

  let panel_frame = egui::Frame::new()
    .fill(ctx.style().visuals.window_fill())
    .corner_radius(10)
    .stroke(ctx.style().visuals.widgets.noninteractive.fg_stroke)
    .outer_margin(1); // so the stroke is within the bounds

  CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
    let app_rect = ui.max_rect();

    let title_bar_height = 32.0;
    let title_bar_rect = {
      let mut rect = app_rect;
      rect.max.y = rect.min.y + title_bar_height;
      rect
    };
    title_bar_ui(ui, title_bar_rect, title);

    // Add the contents:
    let content_rect = {
      let mut rect = app_rect;
      rect.min.y = title_bar_rect.max.y;
      rect
    }
    .shrink(4.0);
    let mut content_ui = ui.new_child(UiBuilder::new().max_rect(content_rect));
    add_contents(&mut content_ui);
  });
}

impl eframe::App for AppState {
  fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
    egui::Rgba::TRANSPARENT.to_array() // Make sure we don't paint anything behind the rounded corners
  }

  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    custom_window_frame(ctx, "SteamPick", |ui| {
      ui.heading("My egui Application");
      ui.horizontal(|ui| {
        let name_label = ui.label("App: ");
        ui.text_edit_singleline(&mut self.apps[self.count].title).labelled_by(name_label.id);
      });
      ui.horizontal(|ui| {
        if ui.button("Prev").clicked() {
          self.count = (self.count + self.apps.len() - 1 ) % self.apps.len();
        }
        if ui.button("Next").clicked() {
          self.count = (self.count + self.apps.len() + 1 ) % self.apps.len();
        };
      });
    });
  }
}

fn title_bar_ui(ui: &mut egui::Ui, title_bar_rect: eframe::epaint::Rect, title: &str) {
  use egui::{Align2, FontId, Id, PointerButton, Sense, UiBuilder, vec2};

  let painter = ui.painter();

  let title_bar_response =
    ui.interact(title_bar_rect, Id::new("title_bar"), Sense::click_and_drag());

  // Paint the title:
  painter.text(
    title_bar_rect.center(),
    Align2::CENTER_CENTER,
    title,
    FontId::proportional(20.0),
    ui.style().visuals.text_color(),
  );

  // Paint the line under the title:
  painter.line_segment(
    [
      title_bar_rect.left_bottom() + vec2(1.0, 0.0),
      title_bar_rect.right_bottom() + vec2(-1.0, 0.0),
    ],
    ui.visuals().widgets.noninteractive.bg_stroke,
  );

  // Interact with the title bar (drag to move window):
  if title_bar_response.double_clicked() {
    let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
    ui.ctx().send_viewport_cmd(ViewportCommand::Maximized(!is_maximized));
  }

  if title_bar_response.drag_started_by(PointerButton::Primary) {
    ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
  }

  ui.scope_builder(
    UiBuilder::new()
      .max_rect(title_bar_rect)
      .layout(egui::Layout::right_to_left(egui::Align::Center)),
    |ui| {
      ui.spacing_mut().item_spacing.x = 4.0;
      ui.visuals_mut().button_frame = false;
      ui.add_space(8.0);
      close_maximize_minimize(ui);
    },
  );
}

/// Show some close/maximize/minimize buttons for the native window.
fn close_maximize_minimize(ui: &mut egui::Ui) {
  use egui::{Button, RichText};

  let button_height = 16.0;

  let close_response =
    ui.add(Button::new(RichText::new("🗙").size(button_height))).on_hover_text("Close the window");
  if close_response.clicked() {
    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
  }

  let minimized_response = ui
    .add(Button::new(RichText::new("🗕").size(button_height)))
    .on_hover_text("Minimize the window");
  if minimized_response.clicked() {
    ui.ctx().send_viewport_cmd(ViewportCommand::Minimized(true));
  }
}
