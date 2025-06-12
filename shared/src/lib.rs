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
