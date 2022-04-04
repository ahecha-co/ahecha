pub mod loaders;

use ahecha_html::{Attributes, Component, Element, Node};
use axum::{body::HttpBody, extract::Query, Router};
use axum_extra::routing::RouterExt;
use axum_macros::TypedPath;
use http::{HeaderMap, StatusCode};
use image::ImageFormat;
use serde::Deserialize;

pub struct Image<'a> {
  pub src: &'a str,
  pub alt: Option<&'a str>,
  pub class: Option<&'a str>,
  pub width: Option<&'a str>,
  pub height: Option<&'a str>,
  pub quality: u8,
}

impl<'a> Component for Image<'a> {
  fn view(&self) -> ahecha_html::Node {
    Node::Element(Element {
      attributes: Attributes::default()
        .set(Some(("src", self.src)))
        .set(Some(("alt", self.alt)))
        .set(Some(("class", self.class)))
        .set(Some(("width", self.width)))
        .set(Some(("height", self.height))),
      children: Default::default(),
      name: "img",
    })
  }
}

impl<'a> Default for Image<'a> {
  fn default() -> Self {
    Self {
      src: Default::default(),
      alt: Default::default(),
      class: Default::default(),
      width: Default::default(),
      height: Default::default(),
      quality: 75,
    }
  }
}

#[axum::async_trait]
pub trait ImageLoader: Clone {
  async fn load(&self, props: ImageProps) -> Result<(ImageFormat, Vec<u8>), (StatusCode, String)>;
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/_ahecha/image")]
pub struct ImagePagePath;

#[derive(Deserialize)]
pub struct ImageProps {
  pub src: String,
  #[serde(default = "default_width")]
  pub w: u32,
  pub q: Option<u8>,
}

pub struct AhechaImagePage;

impl AhechaImagePage {
  pub fn mount<I>(image_loader: I) -> Router<I>
  where
    I: ImageLoader + HttpBody + Send + Sync + 'static,
  {
    Router::new().typed_get(
      |_: ImagePagePath, Query(props): Query<ImageProps>| async move {
        match image_loader.load(props).await {
          Ok((format, buffer)) => (
            StatusCode::OK,
            {
              let mut headers = HeaderMap::new();
              headers.insert(
                "Content-Type",
                match format {
                  ImageFormat::Png => "image/png",
                  ImageFormat::Jpeg => "image/jpeg",
                  ImageFormat::WebP => "image/webp",
                  _ => panic!("Unsupported image format {:?}", format),
                }
                .parse()
                .unwrap(),
              );

              headers
            },
            buffer,
          ),
          Err(e) => {
            panic!("Error loading image: {:?}", e);
          }
        }
      },
    )
  }
}

fn default_width() -> u32 {
  1200
}
