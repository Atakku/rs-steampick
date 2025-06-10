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
    dioxus::launch(|| rsx! {div{}});
  }

  #[cfg(feature = "desktop")]
  {
    server_fn::client::set_server_url("https://steampick.atakku.dev");
    use dioxus::desktop::{wry::dpi::PhysicalSize, Config, WindowBuilder};
    dioxus::LaunchBuilder::desktop()
      .with_cfg(Config::default().with_window(WindowBuilder::new().with_inner_size(PhysicalSize::new(300, 300)).with_always_on_top(true).with_decorations(false)))
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
        let game = use_resource(move || {open::that(format!("steam://advertise/{id}")).unwrap(); api::get_game(id())});
        rsx! {
          match &*game.read_unchecked() {
            Some(Ok(Some(game))) => rsx! {
              //if let Some(t) = &game.trailer {
              //  video {
              //    autoplay: true,
              //    r#loop: true,
              //    muted: false,
              //    class: "h-[480px] w-[854px] bg-black",
              //    src: "https://shared.cloudflare.steamstatic.com/store_item_assets/{t}"
              //  }
              //},
              h1 { "Games: {count}/{len} {game.name}" }
            },
            _ => rsx! {h1 { "Games: {count}/{len} some fuckin shit went wrong meow" }},
          }
          button { onclick: move |_| count.set((len() as i32 + count() - 1) % len()), "Previous game!!!!" }
          br {  }
          button { onclick: move |_| count.set((len() as i32 + count() + 1) % len()), "Next game!!!!" }
          br {  }
          button { onclick: move |_| {count.set(0); order.set(Order::Score(1000))}, "{order}" }
          br {  }
          button { onclick: move |_| {count.set(0); order.set(Order::Random)}, "Order randomly" }
          br {}
          button { onclick: move |_| {count.set(0); order.set(Order::ScoreAndFilterTag(1000, 3859))}, "Order by score (>1000 reviews) and Multiplayer only" }
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
