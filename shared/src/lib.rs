// Copyright 2025 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use serde::{Deserialize, Serialize};

pub type AppId = u32;

#[derive(Deserialize, Serialize, Clone)]
pub struct App {
  pub id: AppId,
  pub title: String,
  pub reviews_positive: u32,
  pub reviews_negative: u32,
  //pub tags: Vec<u32>,
}

impl App {
  pub fn reviews(&self) -> u32 {
    self.reviews_negative + self.reviews_positive
  }

  pub fn score(&self) -> f32 {
    self.reviews_positive as f32 * 100.0 / self.reviews() as f32
  }
}
