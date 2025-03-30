use aidoku::{
  error::Result,
  helpers::uri::QueryParameters,
  prelude::format,
  std::{html::Node, net::Request, Vec},
  Filter, FilterType,
};
use alloc::string::{String, ToString};
use strum_macros::Display;

#[expect(private_interfaces)]
#[derive(Display)]
#[strum(prefix = "https://www.bilicomic.net")]
pub enum Url<'a> {
  #[strum(to_string = "/")]
  Domain,

  // status={status}&mainCategoryId={kind}&orderBy={sort_by}&page={page}
  #[strum(to_string = "/filter/lastupdate_0_0_0_0_0_0_0_{page}_0.html")]
  Filters { page: i32 },

  #[strum(to_string = "/search.html?searchkey={keyword}")]
  Search { keyword: String },

  #[strum(to_string = "{id}")]
  Manga { id: &'a str },
}

impl<'a> Url<'a> {
  pub fn get_html(self) -> Result<Node> {
    self.get().html()
  }
}

impl Url<'_> {
  pub fn get(&self) -> Request {
    Request::get(self.to_string()).default_headers()
  }
}
impl<'a> From<(Vec<Filter>, i32)> for Url<'a> {
  fn from((filters, page): (Vec<Filter>, i32)) -> Self {
    let mut query = QueryParameters::new();

    for filter in filters {
      match filter.kind {
        FilterType::Title => {
          let keyword = match filter.value.as_string() {
            Ok(str_ref) => str_ref.read(),
            Err(_) => continue,
          };
          return Url::Search { keyword };
        }
        _ => continue,
      }
    }
    query.push_encoded("page", Some(page.to_string().as_str()));
    Url::Filters { page }
  }
}

pub trait DefaultRequest {
  fn default_headers(self) -> Self;
}

impl DefaultRequest for Request {
  fn default_headers(self) -> Self {
    let referer = Url::Domain.to_string();
    self
      .header("Referer", &referer)
      .header(
        "User-Agent",
        "Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Mobile Safari/537.36",
      )
      .header("Cookie", "night=1; jieqiVisitTime=jieqiCartoonsearchTime%3D1743339324; jieqiVisitId=cartoon_cartoonviews%3D1%7C733; jieqiRecentRead=626.48152.0.1.1739634073.0-650.49472.0.1.1739698723.0-643.48896.0.1.1739698735.0-28.1691.0.1.1739698769.0-467.36951.0.1.1739713582.0-26.1646.0.1.1739714424.0-1.2.0.1.1743338889.0-733.56893.0.1.1743339402.0; cf_clearance=QtY9jBXWASUPhsTtKyZzxcsCR9598Er2IfTpdFGrJNM-1743339405-1.2.1.1-5Ud6P3ICY5vNPoczwWJ6gy1k5VFRba9I7tNSVIDpUbl0bOlHWDtAPqmRgFxrHLtql03SbyRUMnxLhHt51ommhiCJCLxfGRH6g3gTdIU5IDv3sOEOpbEtSm2QGE5ARK2LWxhN8jCHcuC6h2rhcAbPKZcEp6rtGeDo3jPpaCPcwjijqA3PH2jT0LHSekmBg2.UdM2uFIP6kaTMj.sghcoUsBuv5IydHybFaQFkZPvO0YnCSZezHTfLGGAmoQf6skwBIbHlVo2bq_p9XLXnJ0y3AmZ5bijj3TdbB1NiiavwpqwWDezg0fO3HIT7gq5O_Fx13h7Ecujm.fNBlCfMFF.kW8aCq6yUdp4cNXHVV85zO.4")
      .header("Accept-Language", "zh-CN,zh;q=0.9")
      .header("Accept", "*/*")
  }
}
