// Copyright 2025 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use atklib::{R, once_cell};
use derivative::Derivative;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};
use std::{collections::HashMap, fs};

use crate::AppId;

nestruct::flatten! {
  #[serde_as]
  #[skip_serializing_none]
  #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Default)]
  Types {
    store_item: {
      //item_type
      id: AppId,
      //success
      //visible
      //unvailable_for_country_restriction: bool?,
      name: String,
      //store_url_path
      //appid: u32
      //type
      //included_types
      //included_appids
      //is_free
      //is_early_access
      #[serde(default, skip_serializing_if = "Vec::is_empty")]
      content_descriptorids: [i32],
      #[serde(default, skip_serializing_if = "Vec::is_empty")]
      tagids: [u32],
      related_items: {
        parent_appid: u32?,
      }?,
      included_items: {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        included_apps: [StoreItem],
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        included_packages: [StoreItem],
      }?,
      categories: {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        supported_player_categoryids: [u32],
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        feature_categoryids: [u32],
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        controller_categoryids: [u32],
      }?,
      reviews: {
        summary_filtered: StoreReviewSummary?,
        summary_unfiltered: StoreReviewSummary?
      }?,
      basic_info: {
        short_description: String?,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        publishers: [Publisher],
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        developers: [Publisher],
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        franchises: [Publisher],
        capsule_headline: String?,
      }?,
      #[serde(default, skip_serializing_if = "Vec::is_empty")]
      tags: [{
        tagid: u32?,
        weight: u32?
      }],
      ////assets
      release: {
        steam_release_date: u32?,
        original_release_date: u32?,
        original_steam_release_date: u32?,
        is_coming_soon: bool?,
        is_preload: bool?,
        custom_release_date_message: String?,
        is_abridged_release_date: bool?,
        coming_soon_display: String?,
        is_early_access: bool?,
        mac_release_date: u32?,
        linux_release_date: u32?,
      }?,
      platforms: {
        // todo:
      }?,
      game_rating: {
        // todo:
      }?,
      best_purchase_option: PurchaseOption?,
      #[serde(default, skip_serializing_if = "Vec::is_empty")]
      purchase_options: [PurchaseOption],
      #[serde(default, skip_serializing_if = "Vec::is_empty")]
      accessories: [PurchaseOption],
      self_purchase_option: PurchaseOption?,
      screenshots: {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        all_ages_screenshots: [Screenshot],
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        mature_content_screenshots: [Screenshot]
      }?,
      trailers: {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        highlights: [Trailer],
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        other_trailers: [Trailer],
      }?,
      #[serde(default, skip_serializing_if = "Vec::is_empty")]
      supported_languages: [{
        //todo:
      }],
    },
    video_source: {
      filename: String?,
      #[serde(alias = "type")]
      video_type: String?,
    }?,
    trailer: {
      trailer_name: String?,
      trailer_url_format: String?,
      #[serde(default, skip_serializing_if = "Vec::is_empty")]
      trailer_480p: [VideoSource],
      #[serde(default, skip_serializing_if = "Vec::is_empty")]
      trailer_max: [VideoSource],
      #[serde(default, skip_serializing_if = "Vec::is_empty")]
      microtrailer: [VideoSource],
      screenshot_medium: String?,
      screenshot_full: String?,
      trailer_base_id: i32?,
    },
    publisher: {
      name: String?,
      creator_clan_account_id: u32?,
    },
    screenshot: {
      filename: String?,
      ordinal: i32?,
    },
    store_review_summary: {
      review_count: u32?,
      percent_positive: i32?,
      review_score: i32?,
      review_score_label: String?
    },
    purchase_option: {
      packageid: i32?,
      bundleid: i32?,
      purchase_option_name: String?,
      //final_price_in_cents: i64?,
      //original_price_in_cents: i64?,
      //user_final_price_in_cents: i64?,
      formatted_final_price: String?,
      formatted_original_price: String?,
      //discount_pct: i32?,
      user_discount_pct: i32?,
      bundle_discount_pct: i32?,
      //#[serde(default, skip_serializing_if = "Vec::is_empty")]
      //active_discounts: [Discount],
      //#[serde(default, skip_serializing_if = "Vec::is_empty")]
      //user_active_discounts: [Discount],
      //#[serde(default, skip_serializing_if = "Vec::is_empty")]
      //inactive_discounts: [Discount],
      user_can_purchase: bool?,
      user_can_purchase_as_gift: bool?,
      //is_commercial_license: bool?,
      //should_suppress_discount_pct: bool?,
      //hide_discount_pct_for_compliance: bool?,
      //included_game_count: i32?,
      //lowest_recent_price_in_cents: i64?,
      requires_shipping: bool?,
      recurrence_info: {
        packageid: i32?,
        billing_agreement_type: i32?,
        renewal_time_unit: i32?,
        renewal_time_period: i32?,
      }?,
    },
    discount: {
      discount_amount: i64?,
      discount_description: String?,
      discount_end_date: u32?,
    },
    store_item_response: {
      store_items: [StoreItem]?
    },
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
  #[derive(Serialize)]
  GetItemsParams {
    ids: [#[serde(rename_all = "lowercase")] {
      AppId(AppId),
      PackageId(AppId),
      BundleId(AppId),
      TagId(AppId),
      CreatorId(AppId),
      HubCategoryId(AppId)
    }],
    context: #[derive(Derivative)] #[derivative(Default)] {
      #[derivative(Default(value = "\"US\".to_string()"))]
      country_code: String,
    },
    data_request: #[derive(Derivative)] #[derivative(Default)] {
      #[derivative(Default(value = "true"))]
      include_assets: bool,
      #[derivative(Default(value = "true"))]
      include_release: bool,
      #[derivative(Default(value = "true"))]
      include_platforms: bool,
      #[derivative(Default(value = "true"))]
      include_all_purchase_options: bool,
      #[derivative(Default(value = "true"))]
      include_screenshots: bool,
      #[derivative(Default(value = "true"))]
      include_trailers: bool,
      #[derivative(Default(value = "true"))]
      include_ratings: bool,
      #[derivative(Default(value = "20"))]
      include_tag_count: i32,
      #[derivative(Default(value = "true"))]
      include_reviews: bool,
      #[derivative(Default(value = "true"))]
      include_basic_info: bool,
      #[derivative(Default(value = "true"))]
      include_supported_languages: bool,
      #[derivative(Default(value = "true"))]
      include_full_description: bool,
      #[derivative(Default(value = "true"))]
      include_included_items: bool,
      #[derivative(Default(value = "true"))]
      include_assets_without_overrides: bool,
      #[derivative(Default(value = "true"))]
      include_links: bool,
    }
  }
}

once_cell!(|pub data: HashMap<AppId, StoreItem>| {
  let input_apps: Response<FamilyAppsResponse> = serde_json::from_str(&fs::read_to_string("./data.json").unwrap_or("{}".into())).unwrap_or_default();
  let initial_apps: Vec<AppId> = input_apps.response.apps.into_iter().filter(|a| a.exclude_reason == 0).map(|a| a.appid).collect();
  let cached_apps: HashMap<AppId, StoreItem> = serde_json::from_str(&fs::read_to_string("./cache.json").unwrap_or("{}".into())).unwrap_or_default();
  let mut cached_apps: HashMap<AppId, StoreItem> = cached_apps.into_iter().filter(|(k, v)| initial_apps.contains(k)).collect();
  let missing: Vec<AppId> = initial_apps.into_iter().filter(|i| !cached_apps.contains_key(i)).collect();
  let mut items: Vec<StoreItem> = Vec::new();
  for ids in missing.chunks(100) {
    let json = serde_json::to_string(&GetItemsParams {
      ids: ids.into_iter().map(|id| Ids::AppId(*id)).collect(),
      context: Default::default(),
      data_request: Default::default(),
    }).unwrap();
    let url =
      format!("https://api.steampowered.com/IStoreBrowseService/GetItems/v1?input_json={json}");
    let json = Client::new().get(url).send().unwrap().text().unwrap();
    let Ok(res) = serde_json::from_str::<Response<StoreItemResponse>>(&json) else {
      println!("Failed to parse response: {json}");
      continue;
    };
    cached_apps.extend(res.response.store_items.into_iter().flatten().map(|a| (a.id, a)));
    fs::write("cache.json", serde_json::to_string(&cached_apps).unwrap());
  }
  cached_apps
});

#[derive(Deserialize, Default)]
pub struct Response<T> {
  response: T,
}
