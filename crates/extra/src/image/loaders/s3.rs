use std::{io::Cursor, path::Path};

use http::StatusCode;
use image::{imageops::FilterType, ImageFormat};
use s3::{creds::Credentials, Bucket, Region};

use crate::image::{ImageLoader, ImageProps};

#[derive(Clone)]
pub struct ImageLoaderS3 {
  region: Region,
  credentials: Credentials,
  bucket: String,
}

#[axum::async_trait]
impl ImageLoader for ImageLoaderS3 {
  // FIXME: Is a huge security risk, if you are saving non public files in the bucket.
  // FIXME: Is a huge security risk to let the user specify the image size, we need to find a way to predefine a range of sizes. Check https://nextjs.org/docs/api-reference/next/image
  async fn load(&self, props: ImageProps) -> Result<(ImageFormat, Vec<u8>), (StatusCode, String)> {
    let bucket = Bucket::new(&self.bucket, self.region.clone(), self.credentials.clone())
      .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match fetch_resized_image(&bucket, &props).await {
      Ok(data) => Ok(data),
      Err(_) => match resize_image(&bucket, &props).await {
        Ok(data) => Ok(data),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
      },
    }
  }
}

async fn fetch_resized_image(
  bucket: &Bucket,
  props: &ImageProps,
) -> Result<(ImageFormat, Vec<u8>), ()> {
  // TODO: find a more elegant way to add the width to the filename
  let extension = Path::new(&props.src)
    .extension()
    .unwrap()
    .to_str()
    .expect("Invalid extension");
  let filename = sized_image_filename(&props.src, props.w);
  let (data, code) = bucket.get_object(filename).await.map_err(|_| ())?;

  match code {
    200 => Ok((image_format_from_extension(extension), data)),
    _ => Err(()),
  }
}

async fn resize_image(
  bucket: &Bucket,
  props: &ImageProps,
) -> Result<(ImageFormat, Vec<u8>), String> {
  let (data, code) = bucket
    .get_object(&props.src)
    .await
    .map_err(|e| e.to_string())?;

  match code {
    200 => {
      let img = image::load_from_memory(&data).map_err(|e| e.to_string())?;
      let width = img.width() as f32;
      let height = img.height() as f32;
      let ratio = width / height;
      let new_height = (props.w as f32 * ratio).floor();
      let extension = image_format_from_extension(
        Path::new(&props.src)
          .extension()
          .unwrap()
          .to_str()
          .expect("Invalid extension"),
      );

      let resized = img.resize(props.w, new_height as u32, FilterType::Lanczos3);
      let mut bytes: Vec<u8> = Vec::new();
      resized
        .write_to(&mut Cursor::new(&mut bytes), extension)
        .map_err(|e| e.to_string())?;

      Ok((extension, bytes))
    }
    _ => Err(format!("Error loading image: {}", code)),
  }
}

fn image_format_from_extension(extension: &str) -> ImageFormat {
  match extension {
    "png" => ImageFormat::Png,
    "jpg" => ImageFormat::Jpeg,
    "jpeg" => ImageFormat::Jpeg,
    "webp" => ImageFormat::WebP,
    _ => panic!("Unsupported image format {:?}", extension),
  }
}

fn sized_image_filename(filename: &str, width: u32) -> String {
  let path = Path::new(filename);
  let extension = path
    .extension()
    .unwrap()
    .to_str()
    .expect("Invalid extension");
  let basename = path
    .file_stem()
    .expect("Filename cannot be empty")
    .to_str()
    .expect("Couldn't convert filename to string");
  format!("{}-{}{}", basename, width, extension)
}
