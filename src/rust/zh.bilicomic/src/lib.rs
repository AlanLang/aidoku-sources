#![no_std]
mod parser;
mod url;

use aidoku::{
  error::Result,
  prelude::*,
  std::net::Request,
  std::{String, Vec},
  Chapter, Filter, Manga, MangaPageResult, Page,
};
use alloc::{string::ToString, vec};
use parser::MangaListResponse;
use url::Url;
extern crate alloc;

const BASE_URL: &str = "https://www.bilicomic.net/";

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
  let manga_list_url = Url::from((filters, page));
  let filters_page = manga_list_url.get_html()?;
  return filters_page.get_page_result();
}

#[get_manga_details]
fn get_manga_details(manga_id: String) -> Result<Manga> {
  let manga_page = Url::Manga { id: &manga_id }.get_html()?;
  return manga_page.get_details_result(&manga_id);
}

#[get_chapter_list]
fn get_chapter_list(manga_id: String) -> Result<Vec<Chapter>> {
  let manga_page = Url::Manga { id: &manga_id }.get_html()?;
  let chapter_url = manga_page
    .select(".module-merge")
    .select("a.book-status")
    .attr("href")
    .read();
  let chapter_page = Url::Manga { id: &chapter_url }.get_html()?;
  return chapter_page.get_chapter_list_result();
}

#[get_page_list]
fn get_page_list(_manga_id: String, chapter_id: String) -> Result<Vec<Page>> {
  let chapter_page = Url::Manga { id: &chapter_id }.get_html()?;
  chapter_page.get_page_list()
}

#[modify_image_request]
fn modify_image_request(request: Request) {
  request
    .header(
      "Accept",
      "image/avif,image/webp,image/apng,image/svg+xml,image/*,*/*;q=0.8",
    )
    .header("DNT", "1")
    .header("Referer", BASE_URL)
    .header("Sec-Fetch-Dest", "empty")
    .header("Sec-GPC", "1");
}
