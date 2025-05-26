// Copyright 2025 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

pub type AppId = u32;

#[derive(Serialize, Deserialize)]
pub struct StoreItem {
  // item_type
  pub id: AppId,
  // success
  // visible
  pub name: String,
  pub store_url_path: String,
  // appid
  // type
  // is_free
  // content_descriptorids
  pub categories: Categories,
  pub assets: Assets
}

#[derive(Serialize, Deserialize)]
pub struct Categories {
  pub supported_player_categoryids: [u32],
  pub feature_categoryids: [u32]
}

pub struct Assets {
  asset_url_format: String,
  main_capsule: String,
  small_capsule: String,
  header: String,
  page_background: String,
  hero_capsule: String,
  library_capsule: String,
  library_capsule_2x: String,
  library_hero: String,
  library_hero_2x: String,
  community_icon: String,
  page_background_path: String,
  raw_page_background: String
}

pub struct BasicInfo {
  short_description: String
}