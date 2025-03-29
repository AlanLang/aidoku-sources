use aidoku::{
  error::Result,
  prelude::format,
  std::{html::Node, net::Request, Vec},
  Filter, FilterType,
};
use alloc::string::{String, ToString};
use strum_macros::Display;

#[expect(private_interfaces)]
#[derive(Display)]
#[strum(prefix = "https://nnhanman9.com")]
pub enum Url {
  #[strum(to_string = "/")]
  Domain,

  #[strum(to_string = "/catalog.php?key={keyword}")]
  Search { keyword: String },

  #[strum(to_string = "/{path}")]
  Page { path: String },
}

impl Url {
  pub fn get_html(self) -> Result<Node> {
    self.get().html()
  }
}

impl Url {
  pub fn get(&self) -> Request {
    Request::get(self.to_string()).default_headers()
  }
}

impl From<Vec<Filter>> for Url {
  fn from(filters: Vec<Filter>) -> Self {
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
    return Url::Page {
      path: "ranking/weekly".to_string(),
    };
  }
}

pub trait DefaultRequest {
  fn default_headers(self) -> Self;
}

impl DefaultRequest for Request {
  fn default_headers(self) -> Self {
    let referer = Url::Domain.to_string();
    self.header("Referer", &referer).header(
      "User-Agent",
      "Mozilla/5.0 (iPhone; CPU iPhone OS 17_6 like Mac OS X) \
			 AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.5 Mobile/15E148 Safari/604.1",
    )
  }
}
