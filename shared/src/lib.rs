// Copyright 2025 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use serde::{Deserialize, Serialize};

pub type AppId = u32;

#[derive(Deserialize, Serialize)]
pub struct App {
  pub id: AppId,
  pub title: String,
  pub review_count: u32,
  pub review_percent: i32,
  pub tags: Vec<u32>,
}
