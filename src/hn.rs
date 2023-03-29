use reqwest::Error;
use serde::Deserialize;
use urlencoding::encode;

#[derive(Deserialize)]
struct AlgoliaSearchHit {
    #[serde(rename(deserialize = "objectID"))]
    object_id: String,
}

#[derive(Deserialize)]
struct AlgoliaSearchResponse {
    hits: Vec<AlgoliaSearchHit>,
}

pub async fn fetch_hn_discussion_url(url: &String) -> Result<Option<String>, Error> {
    let resp = reqwest::get(fmt_search_url(url))
        .await?
        .json::<AlgoliaSearchResponse>()
        .await?;

    match resp.hits.first() {
        Some(hit) => Ok(Some(fmt_hn_item_url(&hit.object_id))),
        _ => Ok(None),
    }
}

fn fmt_search_url(url: &String) -> String {
    format!(
        "http://hn.algolia.com/api/v1/search?query={}&restrictSearchableAttributes=url&tags=story",
        encode(url)
    )
}

fn fmt_hn_item_url(object_id: &String) -> String {
    format!("https://news.ycombinator.com/item?id={}", object_id)
}
