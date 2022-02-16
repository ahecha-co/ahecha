use serde::Deserialize;

use crate::Validate;

pub struct FormUrlEncoded<T>(T)
where
  T: Validate;

impl<'de, T> FormUrlEncoded<T>
where
  T: Validate + Deserialize<'de>,
{
  pub fn from_bytes(input: &'de [u8]) -> anyhow::Result<Self> {
    let values = serde_urlencoded::from_bytes(input)?;
    T::validate(values)?;
    let res = serde_urlencoded::from_bytes(input)?;

    Ok(Self(res))
  }

  pub fn from_str(input: &'de str) -> anyhow::Result<Self> {
    let values = serde_urlencoded::from_str(input)?;
    T::validate(values)?;
    let res = serde_urlencoded::from_str(input)?;

    Ok(Self(res))
  }
}

pub struct FormData<T>(T)
where
  T: Validate;

impl<T> FormData<T> where T: Validate {}
