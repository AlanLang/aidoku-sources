use aidoku::{error::Result, Manga, MangaPageResult};
use alloc::{string::ToString, vec::Vec};

use crate::url::Url;

pub fn parser_mange_list(path: &str) -> Result<MangaPageResult> {
  let html = Url::Page {
    path: path.to_string(),
  }
  .get_html()?;
  let manga_list: Vec<Manga> = html
    .select("div.itemBox")
    .array()
    .map(|item| {
      let node = item.as_node().unwrap();
      let cover = node
        .select("div.itemImg")
        .select("img")
        .attr("src")
        .to_string();
      let url = node.select("div.itemTxt").select("a").attr("href");
      let title = node.select("div.itemTxt").select("a").text().to_string();
      Manga {
        id: url.to_string(),
        cover: cover,
        title: title,
        url: url.to_string(),
        ..Default::default()
      }
    })
    .collect();
  Ok(MangaPageResult {
    manga: manga_list,
    has_more: false,
  })
}
