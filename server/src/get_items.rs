// Copyright 2025 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use atklib::*;
use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};
use std::{
  collections::{HashMap, HashSet},
  fs,
  time::Duration,
};
use steampick::{App, AppId};

nestruct::flatten! {
  #[serde_as]
  #[skip_serializing_none]
  #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
  Types {
    family_apps_response: {
      apps: [{
        appid: u32,
        //name: String,
        exclude_reason: u8,
      }]
    },
    app_reviews: {
      query_summary: {
        total_positive: u32,
        total_negative: u32
      }
    },
    metadata: {
      data: {
        name: String,
        categories: [{
          description: String
        }],
        genres: [{
          description: String
        }]
      }
    },
    steam_spy: {
      tags: HashMap<String, u64>
    }
  }
}

once_cell!(async |pub data: HashMap<AppId, App>| {
  let input_apps: Response<FamilyAppsResponse> = serde_json::from_str(&fs::read_to_string("./data.json").unwrap_or("{}".into())).unwrap_or_default();
  let input_apps: HashMap<AppId, bool> = input_apps.response.apps.into_iter().filter(|a| a.exclude_reason == 0).map(|i| (i.appid, true)).collect();
  //input_apps.extend(iter);serde_json::from_str::<Vec<u32>>(&fs::read_to_string("./extra.json").unwrap_or("[]".into())).unwrap_or_default();

  let cached_apps: HashMap<AppId, App> = serde_json::from_str(&fs::read_to_string("./cache.json").unwrap_or("{}".into())).unwrap_or_default();

  let mut cached_apps: HashMap<AppId, App> = cached_apps.into_iter().filter(|(k, v)| input_apps.contains_key(k)).collect();
  let missing: HashSet<AppId> = input_apps.into_iter().filter(|(k, v)| !cached_apps.contains_key(k)).map(|(a, _)| a).collect();
  println!("missing {}", missing.len());

  for k in missing {
    println!("Getting appdetails/{k}");
    let meta = {
      let Ok(r) = Client::new().get(format!("https://store.steampowered.com/api/appdetails?appids={k}")).send().await else {
        println!("Failed to send req: {k}");
        continue;
      };
      let Ok(json) = r.text().await else {
        println!("Failed to get text: {k}");
        continue;
      };
      let Ok(mut q) = serde_json::from_str::<HashMap<u32, Metadata>>(&json) else {
        println!("Failed to parse response: {json}");
        continue;
      };
      let Some(q) = q.remove(&k) else {
        println!("Failed to get app from response: {json}");
        continue;
      };
      q
    };
    println!("Getting appreviews/{k}");
    let rev = {
      let Ok(r) = Client::new().get(format!("https://store.steampowered.com/appreviews/{k}?json=1&language=all")).send().await else {
        println!("Failed to send req: {k}");
        continue;
      };
      let Ok(json) = r.text().await else {
        println!("Failed to get text: {k}");
        continue;
      };
      let Ok(q) = serde_json::from_str::<AppReviews>(&json) else {
        println!("Failed to parse response: {json}");
        continue;
      };
      q.query_summary
    };
    println!("Getting tags/{k}");
    let tags: Vec<_> = {
      let Ok(r) = Client::new().get(format!("https://steamspy.com/api.php?request=appdetails&appid={k}")).send().await else {
        println!("Failed to send req: {k}");
        continue;
      };
      let Ok(json) = r.text().await else {
        println!("Failed to get text: {k}");
        continue;
      };
      let Ok(q) = serde_json::from_str::<SteamSpy>(&json) else {
        println!("Failed to parse response: {json}");
        continue;
      };
      q.tags.map(|i| i.0)
    };
    //let tags = {
    //  let Ok(r) = Client::new().get(format!("https://store.steampowered.com/app/{k}")).send().await else {
    //    println!("Failed to send req: {k}");
    //    continue;
    //  };
    //  let Ok(html) = r.text().await else {
    //    println!("Failed to get text: {k}");
    //    continue;
    //  };
    //  Html::parse_document(&html).select(&Selector::parse("a.app_tag").unwrap()).map(|i| html_escape::decode_html_entities(&i.inner_html()).trim().to_string()).collect::<Vec<_>>()
    //};
    tokio::time::sleep(Duration::from_secs(5)).await;
    let tags: HashSet<String> = meta.data.categories.into_iter().map(|i| i.description).chain(meta.data.genres.into_iter().map(|i| i.description)).chain(tags).collect();
    cached_apps.insert(k, App { id: k, title: meta.data.name, reviews_positive: rev.total_positive, reviews_negative: rev.total_negative, tags: tags });
    fs::write("cache.json", serde_json::to_string_pretty(&cached_apps).unwrap());
  }
  cached_apps
});

#[derive(Deserialize, Default)]
pub struct Response<T> {
  response: T,
}
