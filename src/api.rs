// Copyright 2025 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use crate::AppId;
#[cfg(feature = "server")]
use crate::server::*;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[server(endpoint = "get_game")]
pub async fn get_game(id: AppId) -> Result<Option<App>, ServerFnError> {
  Ok(get_data().get(&id).map(|a| a.into()))
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Order {
  Random,
}

#[server(endpoint = "get_games")]
pub async fn get_games(order: Order) -> Result<Vec<AppId>, ServerFnError> {
  let mut values: Vec<(&AppId, &StoreItem)> = get_data().iter().collect();

  use rand::seq::SliceRandom;
  match order {
    Order::Random => values.shuffle(&mut rand::rng()),
  }
  Ok(values.into_iter().map(|(k, _)| *k).collect())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct App {
  pub id: AppId,
  pub name: String,
}

#[cfg(feature = "server")]
impl From<&StoreItem> for App {
  fn from(value: &StoreItem) -> Self {
    Self { id: value.id, name: value.name.clone() }
  }
}
