#![no_std]

mod parser;
mod url;

use aidoku::{
  error::Result,
  prelude::*,
  std::{net::Request, String, Vec},
  Chapter, Filter, Listing, Manga, MangaPageResult, MangaStatus, Page,
};
use alloc::string::ToString;
use parser::parser_mange_list;
use url::Url;
extern crate alloc;

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
  let manga_list_url = Url::from(filters);
  if let Url::Search { .. } = manga_list_url {
    let html = manga_list_url.get_html()?;
    let manga_list: Vec<Manga> = html
      .select("div.imgBox")
      .select("ul")
      .select("li")
      .array()
      .map(|item| {
        let node = item.as_node().unwrap();
        let cover = node.select("a").select("img").attr("src").to_string();
        let url = node.select("a").attr("href");
        let title = node.select("a").text().to_string();
        Manga {
          id: url.to_string(),
          cover: cover,
          title: title,
          url: url.to_string(),
          ..Default::default()
        }
      })
      .collect();
    return Ok(MangaPageResult {
      manga: manga_list,
      has_more: false,
    });
  }
  parser_mange_list("ranking/weekly")
}

#[get_manga_listing]
fn get_manga_listing(listing: Listing, _: i32) -> Result<MangaPageResult> {
  let path = match listing.name.as_str() {
    "日榜" => "ranking/daily",
    "周榜" => "ranking/weekly",
    "月榜" => "ranking/monthly",
    "总榜" => "ranking/all",
    "最近更新" => "update",
    "新书发布" => "update/newbook",
    "推荐漫画" => "update/recommend",
    _ => "ranking/weekly",
  };
  parser_mange_list(path)
}

#[get_manga_details]
fn get_manga_details(manga_id: String) -> Result<Manga> {
  let html = Url::Page {
    path: manga_id.clone(),
  }
  .get_html()?;

  let introduction = html.select("div.Introduct_Sub");
  let cover = introduction
    .select("div#Cover")
    .select("img")
    .attr("src")
    .to_string();
  let title = introduction.select("h1").text().to_string();
  let author = introduction
    .select("div.sub_r")
    .select("p.txtItme")
    .first()
    .text()
    .to_string();
  let categories = introduction
    .select("div.sub_r")
    .select("p.txtItme")
    .array()
    .get(1)
    .as_node()?
    .text()
    .to_string()
    .split(",")
    .filter(|s| !s.is_empty())
    .map(|s| s.trim().to_string())
    .collect();
  let last_update = introduction
    .select("p.txtItme")
    .last()
    .select("span.date")
    .text()
    .to_string();
  let description = html.select("p.txtDesc").text().to_string();
  let status = if last_update.contains("连载中") {
    MangaStatus::Ongoing
  } else {
    MangaStatus::Completed
  };
  Ok(Manga {
    id: manga_id,
    cover: cover,
    title: title,
    author: author,
    description: description,
    status: status,
    categories: categories,
    ..Default::default()
  })
}

#[get_chapter_list]
fn get_chapter_list(manga_id: String) -> Result<Vec<Chapter>> {
  let html = Url::Page { path: manga_id }.get_html()?;
  let chapter_list: Vec<Chapter> = html
    .select("ul#mh-chapter-list-ol-0")
    .select("li")
    .array()
    .map(|li| {
      let node = li.as_node().unwrap();
      let title = node.select("span").text().to_string();
      let url = node.select("a").attr("href");
      Chapter {
        id: url.to_string(),
        title: title.clone(),
        url: url.to_string(),
        volume: 1.0,
        chapter: extract_first_number(&title)
          .unwrap_or("0".to_string())
          .parse::<f32>()
          .unwrap_or(0.0),
        ..Default::default()
      }
    })
    .collect();
  Ok(chapter_list)
}

#[get_page_list]
fn get_page_list(manga_id: String, chapter_id: String) -> Result<Vec<Page>> {
  let html = Url::Page { path: chapter_id }.get_html()?;
  let page_list = html
    .select("img.lazy")
    .array()
    .map(|img| {
      let node = img.as_node().unwrap();
      let src = node.attr("data-original");
      Page {
        url: src.to_string(),
        ..Default::default()
      }
    })
    .collect();
  Ok(page_list)
}

#[modify_image_request]
fn modify_image_request(request: Request) {
  request.header("Referer", &Url::Domain.to_string()).header(
    "User-Agent",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 17_6 like Mac OS X) \
			 AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.5 Mobile/15E148 Safari/604.1",
  );
}

fn extract_first_number(text: &str) -> Option<String> {
  let mut num = String::new();

  for c in text.chars() {
    if c.is_digit(10) {
      num.push(c);
    } else if !num.is_empty() {
      return Some(num);
    }
  }

  if !num.is_empty() {
    Some(num)
  } else {
    None
  }
}
