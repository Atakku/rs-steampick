// Copyright 2025 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use std::{ffi::os_str::Display, fmt};

use crate::AppId;
#[cfg(feature = "server")]
use crate::server::*;
use atklib::VecExt;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[server(endpoint = "get_game")]
pub async fn get_game(id: AppId) -> Result<Option<App>, ServerFnError> {
  Ok(get_data().get(&id).map(|a| a.into()))
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Order {
  Random,
  Score(u32),
  ScoreAndFilterTag(u32, u32),
}

impl fmt::Display for Order {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Order::Random => write!(f, "Random order"),
      Order::Score(min) => write!(f, "Random order"),
      Order::ScoreAndFilterTag(min, tag) => write!(f, "Random order"),
    }
  }
}

#[server(endpoint = "get_games")]
pub async fn get_games(order: Order) -> Result<Vec<AppId>, ServerFnError> {
  let mut values: Vec<(&AppId, &StoreItem)> = get_data().iter().collect();

  use rand::seq::SliceRandom;
  match order {
    Order::Random => values.shuffle(&mut rand::rng()),
    Order::Score(min) => {
      values = values.filter(|(_, i)| {
        i.reviews.clone().is_some_and(|i| {
          i.summary_filtered.is_some_and(|i| i.review_count.is_some_and(|i| i >= min))
        })
      });
      values.sort_by_key(|(_, i)| {
        i.reviews
          .clone()
          .unwrap_or_default()
          .summary_filtered
          .unwrap_or_default()
          .percent_positive
          .unwrap_or(0)
      });
      values.reverse();
    },
    Order::ScoreAndFilterTag(min, tag) => {
      values = values.filter(|(_, i)| {
        i.tagids.contains(&tag) &&
        i.reviews.clone().is_some_and(|i| {
          i.summary_filtered.is_some_and(|i| i.review_count.is_some_and(|i| i >= min))
        })
      });
      values.sort_by_key(|(_, i)| {
        i.reviews
          .clone()
          .unwrap_or_default()
          .summary_filtered
          .unwrap_or_default()
          .percent_positive
          .unwrap_or(0)
      });
      values.reverse();
    }
  }
  Ok(values.into_iter().map(|(k, _)| *k).collect())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct App {
  pub id: AppId,
  pub name: String,
  pub trailer: Option<String>,
  //pub full_description: Option<String>
}

#[cfg(feature = "server")]
impl From<&StoreItem> for App {
  fn from(value: &StoreItem) -> Self {
    Self {
      id: value.id,
      name: value.name.clone(),
      trailer: value
        .trailers
        .clone()
        .map(|i| {
          i.highlights
            .first()
            .map(|i| {
              let Some(format) = &i.trailer_url_format else {
                return None;
              };
              let Some(trailer) = i.trailer_480p.first() else {
                return None;
              };
              trailer.filename.clone().map(|i| format.replace("${FILENAME}", &i))
            })
            .flatten()
        })
        .flatten(),
        //full_description: value.full_description
    }
  }
}
