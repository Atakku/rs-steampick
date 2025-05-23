// Copyright 2025 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

#![allow(non_snake_case)]

use api::Order;
use dioxus::prelude::*;
use rand::seq::IndexedRandom;

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
  let mut order = use_signal(|| Order::Random);
  let mut get_games = use_resource(move || api::get_games(order()));
  let mut count = use_signal(|| 0);

  match &*get_games.read_unchecked() {
    Some(Ok(games)) => {
      let len = games.len();
      rsx! {
        //document::Stylesheet { href: CSS }
        h1 { "High-Five counter: {count}" }
        button { onclick: move |_| count.set((count + 1) % len), "Up high!" }
        button { onclick: move |_| count.set((count - 1) % len), "Down low!" }
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
