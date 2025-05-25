// Copyright 2025 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

#![allow(non_snake_case)]

use api::Order;
use dioxus::prelude::*;

pub type AppId = u32;

mod api;
#[cfg(feature = "server")]
mod server;
mod ui;

static CSS: Asset = asset!("/assets/paper.min.css");

fn main() {
  // Set the url of the server where server functions are hosted..
  #[cfg(not(feature = "server"))]
  server_fn::client::set_server_url("https://steampick.atakku.dev");

  #[cfg(feature = "server")]
  server::get_data();

  dioxus::launch(app);
}

pub fn app() -> Element {
  let mut order = use_signal(|| Order::Score(300));
  let mut list = use_resource(move || api::get_games(order()));
  //let mut len = use_memo(move || {
  //  list.read().unwrap().map(|i| i.len()).unwrap_or(0);
  //});
  //let mut count = use_signal(|| 0);
  //let mut list = use_resource(move || api::get_game(list[count.read()]));
  //let mut game = use_signal(|| None);
  //let mut get_games = use_resource(move || async move {
  //  let data = api::get_games(order()).await.unwrap();
  //  let game = api::get_game(data[count() as usize]).await;
  //});

  match &*list.read_unchecked() {
    Some(Ok(games)) => {
      let mut count = use_signal(|| 0);
      let mut len = use_memo(move || list.unwrap().unwrap().len() as i32 - 1);
      let mut id = use_memo(move || list.unwrap().unwrap()[count() as usize]);
      let game = use_resource(move || api::get_game(id()));
      rsx! {
        match &*game.read_unchecked() {
        Some(Ok(Some(game))) => rsx! {
          h1 { "Games: {count}/{len} {game.name}" }
        },
        _ => rsx! {div { "Meow" }},
      }
      button { onclick: move |_| count.set((count + 1) % len()), "Click me!" }
      }
    }
    Some(Err(_)) => rsx! {
      div { "Loading games failed" }
    },
    None => rsx! {
      div { "Loading games..." }
    },
  }
}
