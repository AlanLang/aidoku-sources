use crate::url::Url;
use aidoku::{
  error::Result,
  prelude::*,
  std::{html::Node, ArrayRef, ValueRef, Vec},
  Chapter, Manga, MangaPageResult, MangaStatus, Page,
};
use alloc::{
  fmt::format,
  string::{String, ToString},
};

pub trait MangaListResponse {
  fn get_page_result(self) -> Result<MangaPageResult>;
  fn get_details_result(self, manga_id: &str) -> Result<Manga>;
  fn get_chapter_list_result(self) -> Result<Vec<Chapter>>;
  fn get_page_list(self) -> Result<Vec<Page>>;
}

impl MangaListResponse for Node {
  fn get_page_result(self) -> Result<MangaPageResult> {
    let manga = self
      .select("li.book-li")
      .array()
      .get_manga_list(get_home_page_mange)?;
    let page_count = self
      .select("a.last")
      .text()
      .read()
      .parse::<usize>()
      .unwrap_or(0);
    let page = self
      .select("#pagelink")
      .select("strong")
      .text()
      .read()
      .parse::<usize>()
      .unwrap_or(1);

    let has_more = page < page_count;
    Ok(MangaPageResult { manga, has_more })
  }

  fn get_details_result(self, manga_id: &str) -> Result<Manga> {
    let cover = self
      .select("div.module-item-cover")
      .select("img")
      .attr("src")
      .read();
    let title = self.select("h1.book-title").text().read();
    let author = self.select("div.book-rand-a").text().read();
    let description = self.select("#bookSummary").select("content").text().read();
    let categories = self
      .select("span.tag-small-group")
      .select(".tag-small")
      .array()
      .map(|item| {
        let node = item.as_node();
        match node {
          Ok(node) => node.text().read(),
          Err(_) => String::new(),
        }
      })
      .collect();
    let status_str = self.select("p.book-meta").text().read().to_string();
    let status_str = status_str.split("|").next().unwrap_or("连载");
    let status = match status_str {
      "连载" => MangaStatus::Ongoing,
      "完结" => MangaStatus::Completed,
      _ => MangaStatus::Unknown,
    };
    let manga_url = self
      .select(".module-merge")
      .select("a.book-status")
      .attr("href")
      .read();

    Ok(Manga {
      id: manga_id.to_string(),
      cover,
      title,
      artist: author.clone(),
      author,
      description,
      url: Url::Manga { id: &manga_url }.to_string(),
      categories,
      status,
      ..Default::default()
    })
  }

  fn get_chapter_list_result(self) -> Result<Vec<Chapter>> {
    let chapters = self
      .select("#volumes")
      .select("div.catalog-volume")
      .array()
      .filter_map(get_some_chapter)
      .flatten()
      .collect::<Vec<_>>();
    Ok(chapters)
  }

  fn get_page_list(self) -> Result<Vec<Page>> {
    let pages = self
      .select("div#acontentz")
      .select("img")
      .array()
      .filter_map(|item| {
        let node = item.as_node();
        if let Ok(node) = node {
          let src = node.attr("data-src").read();
          Some(Page {
            url: src,
            ..Default::default()
          })
        } else {
          None
        }
      })
      .collect::<Vec<_>>();
    Ok(pages)
  }
}

trait MangaArr {
  fn get_manga_list<F>(self, parser: F) -> Result<Vec<Manga>>
  where
    F: Fn(ValueRef) -> Result<Manga>;
}
impl MangaArr for ArrayRef {
  fn get_manga_list<F>(self, parser: F) -> Result<Vec<Manga>>
  where
    F: Fn(ValueRef) -> Result<Manga>,
  {
    let mut manga = Vec::<Manga>::new();
    for item in self {
      manga.push(parser(item)?);
    }
    Ok(manga)
  }
}

fn get_home_page_mange(item: ValueRef) -> Result<Manga> {
  let node = item.as_node()?;

  let cover = node.select("img").attr("data-src").read();
  let title = node.select("h4.book-title").text().read();
  let id = node.select("a.book-layout").attr("href").read();
  let url = Url::Manga { id: &id }.to_string();

  Ok(Manga {
    id,
    cover,
    title,
    url,
    ..Default::default()
  })
}

fn get_some_chapter(node: ValueRef) -> Option<Vec<Chapter>> {
  get_chapter(node).ok()
}

fn get_chapter_item(item: ValueRef) -> Option<Chapter> {
  let node = item.as_node();
  if let Ok(node) = node {
    let id = node.select("a.chapter-li-a").attr("href").read();
    if id.starts_with("javascript:") {
      return None;
    }
    let title = node.select(".chapter-index").text().read();
    Some(Chapter {
      id,
      title,
      ..Default::default()
    })
  } else {
    None
  }
}

fn get_chapter(item: ValueRef) -> Result<Vec<Chapter>> {
  let node = item.as_node()?;

  let capter_name = node.select(".chapter-bar").select("h3").text().read();
  let chapters = node
    .select("li.jsChapter")
    .array()
    .filter_map(get_chapter_item)
    .map(|item| Chapter {
      id: item.id.clone(),
      title: format!("{} {}", capter_name, item.title),
      url: Url::Manga { id: &item.id }.to_string(),
      ..Default::default()
    })
    .collect::<Vec<_>>();
  Ok(chapters)
}
