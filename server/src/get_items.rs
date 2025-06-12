// Copyright 2025 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use atklib::*;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};
use std::{collections::HashMap, fs, thread::sleep, time::Duration};
use steampick::{App, AppId};

nestruct::flatten! {
  #[serde_as]
  #[skip_serializing_none]
  #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Default)]
  Types {
    family_apps_response: {
      apps: [{
        appid: u32,
        name: String,
        exclude_reason: u8
      }]
    }
  }
}

nestruct::flatten! {
  #[serde_as]
  #[skip_serializing_none]
  #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Default)]
  AppReviews {
    query_summary: {
      total_positive: u32,
      total_negative: u32
    }
  }
}

once_cell!(|pub data: HashMap<AppId, App>| {
  let input_apps: Response<FamilyAppsResponse> = serde_json::from_str(&fs::read_to_string("./data.json").unwrap_or("{}".into())).unwrap_or_default();
  let input_apps: HashMap<AppId, Apps> = input_apps.response.apps.into_iter().filter(|a| a.exclude_reason == 0).map(|i| (i.appid, i)).collect();
  let cached_apps: HashMap<AppId, App> = serde_json::from_str(&fs::read_to_string("./cache.json").unwrap_or("{}".into())).unwrap_or_default();
  let mut cached_apps: HashMap<AppId, App> = cached_apps.into_iter().filter(|(k, v)| input_apps.contains_key(k)).collect();
  let missing: HashMap<AppId, Apps> = input_apps.into_iter().filter(|(k, v)| !cached_apps.contains_key(k)).collect();
  for (k, v) in missing {
    let Ok(r) = Client::new().get(format!("https://store.steampowered.com/appreviews/{k}?json=1")).send() else {
      println!("Failed to send req: {k}");
      continue;
    };
    let Ok(json) = r.text() else {
      println!("Failed to get text: {k}");
      continue;
    };
    let Ok(q) = serde_json::from_str::<AppReviews>(&json) else {
      println!("Failed to parse response: {json}");
      continue;
    };
    let q = q.query_summary;
    cached_apps.insert(k, App { id: k, title: v.name, reviews_positive: q.total_positive, reviews_negative: q.total_negative });
    fs::write("cache.json", serde_json::to_string(&cached_apps).unwrap());
    sleep(Duration::from_secs(1));
  }
  cached_apps
});

#[derive(Deserialize, Default)]
pub struct Response<T> {
  response: T,
}