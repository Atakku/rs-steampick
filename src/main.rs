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

#[cfg(feature = "desktop")]
static CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
  #[cfg(feature = "server")]
  {
    server::get_data();
    dioxus::launch(|| rsx!{});
  }

  #[cfg(feature = "desktop")]
  {
    server_fn::client::set_server_url("https://steampick.atakku.dev");
    use dioxus::desktop::{Config, WindowBuilder};
    dioxus::LaunchBuilder::desktop()
      .with_cfg(Config::default().with_window(WindowBuilder::new().with_decorations(false)))
      .launch(app);
  }
}

#[cfg(feature = "desktop")]
pub fn app() -> Element {
  let mut order = use_signal(|| Order::Score(1000));
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
  rsx! {
    document::Stylesheet { href: CSS }
    TitleBar {}
    match &*list.read_unchecked() {
      Some(Ok(games)) => {
        let mut count = use_signal(|| 0);
        let mut len = use_memo(move || list.unwrap().unwrap().len() as i32 - 1);
        let mut id = use_memo(move || list.unwrap().unwrap()[count() as usize]);
        let game = use_resource(move || api::get_game(id()));
        rsx! {
          match &*game.read_unchecked() {
          Some(Ok(Some(game))) => rsx! {
            if let Some(t) = &game.trailer {
              h1 { "Games: {t}" }
            },
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
}

#[component]
#[cfg(feature = "desktop")]
pub fn TitleBar() -> Element {
  use dioxus::desktop::use_window;

  rsx! {
    div {
      class: "pr-1 h-6 w-full bg-gray-900 flex justify-end text-white",
      div {
        class: "w-full",
        onmousedown: move |_| use_window().drag(),
      }
      button {
        onclick: move |_| use_window().set_minimized(true),
        "🗕"
      }
      button {
        onclick: move |_| use_window().set_maximized(true),
        "🗖"
      }
      button {
        onclick: move |_| use_window().close(),
        "🗙"
      }
    }
  }
}
