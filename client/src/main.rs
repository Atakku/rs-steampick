// Copyright 2025 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

#![windows_subsystem = "windows"]

use eframe::egui::{self, Button, CentralPanel, Context, RichText, TextEdit, Ui, ViewportCommand, Visuals};
use steampick::App;

const DIMS: [f32; 2] = [320.0, 210.0];

fn main() -> eframe::Result {
  let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default()
      .with_always_on_top()
      .with_inner_size(DIMS)
      .with_min_inner_size(DIMS)
      .with_max_inner_size(DIMS)
      .with_decorations(false)
      .with_transparent(true),
    ..Default::default()
  };
  eframe::run_native("SteamPick", options, Box::new(|_| Ok(Box::new(AppState::new()))))
}

#[derive(Default)]
struct AppState {
  pub index: usize,
  pub raw: Vec<App>,
  pub apps: Vec<App>,
  pub min_rev: usize,
  pub whitelist: String,
  pub blacklist: String,
}

impl AppState {
  fn new() -> Self {
    let raw =
      reqwest::blocking::get("https://steampick.atakku.dev").unwrap().json::<Vec<App>>().unwrap();
    let mut app = Self { raw, min_rev: 600, ..Default::default() };
    app.update_apps();
    app
  }

  fn app<'a>(&'a self) -> &'a App {
    &self.apps[self.index]
  }

  fn link(&self, link: &str) {
    open::that(format!("steam://{link}/{}", self.app().id)).unwrap();
  }

  fn scroll_store(&mut self, change: i32) {
    scroll(&mut self.index, self.apps.len(), change);
    self.link("store");
  }

  fn scroll_rev(&mut self, change: i32) {
    let raw_min_rev = if self.min_rev <= 1 { 0 } else { self.min_rev };
    self.min_rev = (raw_min_rev as i32 + change).max(1) as usize;
    self.update_apps();
  }

  fn update_apps(&mut self) {
    let blacklist = self
      .blacklist
      .split(",")
      .map(|i| i.trim().to_lowercase().to_string())
      .filter(|i| !i.is_empty())
      .collect::<Vec<String>>();
    let whitelist = self
      .whitelist
      .split(",")
      .map(|i| i.trim().to_lowercase().to_string())
      .filter(|i| !i.is_empty())
      .collect::<Vec<String>>();

    self.apps = self
      .raw
      .clone()
      .into_iter()
      .filter(|i| i.reviews() as usize >= self.min_rev)
      .filter(|i| {
        blacklist.is_empty() || !i.tags.iter().any(|l| blacklist.contains(&l.to_lowercase()))
      })
      .filter(|i| {
        whitelist.is_empty() || i.tags.iter().any(|l| whitelist.contains(&l.to_lowercase()))
      })
      .collect();
    self.apps.sort_by(|a, b| a.score().partial_cmp(&b.score()).unwrap());
    self.apps.reverse();

    if self.apps.is_empty() {
      self.apps = vec![App {
        id: 0,
        title: "No results".to_string(),
        reviews_positive: 0,
        reviews_negative: 0,
        tags: Default::default(),
      }]
    }

    self.scroll_store(0);
  }

  fn link_button(&self, ui: &mut Ui, name: &str, path: &str) {
    if ui.add(Button::new(RichText::new(name).size(16.0))).clicked() {
      self.link(path);
    }
  }

  fn arrow(&self, ui: &mut Ui, name: &str, desc: &str) -> bool {
    ui.add(Button::new(RichText::new(name).size(20.0))).on_hover_text_at_pointer(desc).clicked()
  }
}

fn scroll(index: &mut usize, count: usize, change: i32) {
  *index = ((*index + count) as i32 + change) as usize % count;
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
            .on_hover_text_at_pointer("Close the window");
          if close_response.clicked() {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
          }

          let minimized_response = ui
            .add(Button::new(RichText::new("🗕").size(button_height)))
            .on_hover_text_at_pointer("Minimize the window");
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

      ui.heading(format!("Reviews: {} ({:.2}%)", &self.app().reviews(), &self.app().score()));
      ui.horizontal(|ui| {
        if self.arrow(ui, "<<", "Rewind x100") {
          self.scroll_store(-100);
        }
        if self.arrow(ui, "<-", "Rewind x10") {
          self.scroll_store(-10);
        }
        if self.arrow(ui, "<", "Rewind 1") {
          self.scroll_store(-1);
        }
        ui.heading(format!("[{:0>4}/{:0>4}]", self.index + 1, self.apps.len(),));
        if self.arrow(ui, ">", "Forward x1") {
          self.scroll_store(1);
        };
        if self.arrow(ui, "->", "Forward x10") {
          self.scroll_store(10);
        };
        if self.arrow(ui, ">>", "Forward x100") {
          self.scroll_store(100);
        };
      });

      ui.horizontal(|ui| {
        if self.arrow(ui, "---", "Decrease minimum reviews filter by 50") {
          self.scroll_rev(-500);
        }
        if self.arrow(ui, "--", "Decrease minimum reviews filter by 50") {
          self.scroll_rev(-50);
        }
        if self.arrow(ui, "-", "Decrease minimum reviews filter by 5") {
          self.scroll_rev(-5);
        }
        ui.heading(format!("Min: {}", self.min_rev,));
        if self.arrow(ui, "+", "Increase minimum reviews filter by 5") {
          self.scroll_rev(5);
        };
        if self.arrow(ui, "++", "Increase minimum reviews filter by 50") {
          self.scroll_rev(50);
        };
        if self.arrow(ui, "+++", "Increase minimum reviews filter by 50") {
          self.scroll_rev(500);
        };
      });

      ui.horizontal_wrapped(|ui| {
        self.link_button(ui, "Install", "install");
        self.link_button(ui, "Launch", "launch");
        self.link_button(ui, "Library", "open/library/details");
        self.link_button(ui, "Friends", "url/CommunityFriendsThatPlay");
        self.link_button(ui, "News", "updatenews");
      });

      let mut tags = self.app().tags.iter().fold(String::new(), |a, b| a + b + ", ");

      ui.horizontal(|ui| {
        ui.text_edit_singleline(&mut tags).on_hover_text_at_pointer("This game's tags")
      });

      ui.horizontal(|ui| {
        if ui.text_edit_singleline(&mut self.whitelist).on_hover_text_at_pointer("Whitelist (Any)").changed() {
          self.update_apps();
        }
      });

      ui.horizontal(|ui| {
        if ui.text_edit_singleline(&mut self.blacklist).on_hover_text_at_pointer("Blacklist (Any)").changed() {
          self.update_apps();
        }
      });
    });
  }
}
