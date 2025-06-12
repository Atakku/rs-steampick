// Copyright 2025 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

#![windows_subsystem = "windows"]

use atklib::*;
use eframe::egui::{self, Button, CentralPanel, Context, RichText, ViewportCommand, Visuals};
use steampick::App;

fn main() -> eframe::Result {
  let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default()
      .with_always_on_top()
      .with_inner_size([250.0, 150.0])
      .with_min_inner_size([250.0, 150.0])
      .with_max_inner_size([250.0, 150.0])
      .with_decorations(false)
      .with_transparent(true),
    ..Default::default()
  };
  let h =
    reqwest::blocking::get("https://steampick.atakku.dev").unwrap().json::<Vec<App>>().unwrap();
  let mut state = AppState { count: 0, apps: h.filter(|i| i.reviews() >= 600) };
  state.apps.sort_by(|a, b| a.score().partial_cmp(&b.score()).unwrap());
  state.apps.reverse();
  state.scroll(0);
  eframe::run_native("SteamPick", options, Box::new(|cc| Ok(Box::new(state))))
}

struct AppState {
  pub count: usize,
  pub apps: Vec<App>,
}

impl AppState {
  fn app<'a>(&'a self) -> &'a App {
    &self.apps[self.count]
  }

  fn link(&self, link: &str) {
    open::that(format!("steam://{link}/{}", self.app().id)).unwrap();
  }

  fn scroll(&mut self, change: i32) {
    self.count = ((self.count + self.apps.len()) as i32 + change) as usize % self.apps.len();
    self.link("store");
  }
}

impl eframe::App for AppState {
  fn clear_color(&self, _visuals: &Visuals) -> [f32; 4] {
    egui::Rgba::TRANSPARENT.to_array() // Make sure we don't paint anything behind the rounded corners
  }

  fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
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

      use egui::{Align2, FontId, Id, PointerButton, Sense, UiBuilder, vec2};

      let painter = ui.painter();

      let title_bar_response =
        ui.interact(title_bar_rect, Id::new("title_bar"), Sense::click_and_drag());

      // Paint the title:
      painter.text(
        title_bar_rect.center(),
        Align2::CENTER_CENTER,
        &self.app().title,
        FontId::proportional(14.0),
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
          ui.spacing_mut().item_spacing.x = 2.0;
          ui.visuals_mut().button_frame = false;
          ui.add_space(7.0);

          let button_height = 14.0;

          let close_response = ui
            .add(Button::new(RichText::new("🗙").size(button_height)))
            .on_hover_text("Close the window");
          if close_response.clicked() {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
          }

          let minimized_response = ui
            .add(Button::new(RichText::new("🗕").size(button_height)))
            .on_hover_text("Minimize the window");
          if minimized_response.clicked() {
            ui.ctx().send_viewport_cmd(ViewportCommand::Minimized(true));
          }
        },
      );

      // Add the contents:
      let content_rect = {
        let mut rect = app_rect;
        rect.min.y = title_bar_rect.max.y;
        rect
      }
      .shrink(4.0);
      let mut ui = ui.new_child(UiBuilder::new().max_rect(content_rect));

      ui.heading(format!(
        "Score: {:.2}% ({})",

        &self.app().score(),
        &self.app().reviews()
      ));
      ui.horizontal(|ui| {
        if ui.add(Button::new(RichText::new("<<").size(20.0))).on_hover_text("Rewind x10").clicked() {
          self.scroll(-10);
        }
        if ui.add(Button::new(RichText::new("<-").size(20.0))).on_hover_text("Rewind x1").clicked() {
          self.scroll(-1);
        }
        ui.heading(format!("[{:0>4}/{:0>4}]", self.count + 1, self.apps.len(),));
        if ui.add(Button::new(RichText::new("->").size(20.0))).on_hover_text("Forward x1").clicked() {
          self.scroll(1);
        };
        if ui.add(Button::new(RichText::new(">>").size(20.0))).on_hover_text("Forward x10").clicked() {
          self.scroll(10);
        };
      });
      
      ui.horizontal(|ui| {
        if ui.add(Button::new(RichText::new("Download").size(16.0))).clicked() {
          self.link("install");
        }
        if ui.add(Button::new(RichText::new("Launch").size(16.0))).clicked() {
          self.link("launch");
        }
        if ui.add(Button::new(RichText::new("Library").size(16.0))).clicked() {
          self.link("open/library/details");
        }
      });
      ui.horizontal(|ui| {
        if ui.add(Button::new(RichText::new("Friends").size(16.0))).clicked() {
          self.link("url/CommunityFriendsThatPlay");
        }
        if ui.add(Button::new(RichText::new("News").size(16.0))).clicked() {
          self.link("updatenews");
        }
      });
    });
  }
}
