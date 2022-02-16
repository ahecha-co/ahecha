pub enum Doctype {
  Html5,
}

impl ToString for Doctype {
  fn to_string(&self) -> String {
    match self {
      Doctype::Html5 => "<!DOCTYPE HTML>".to_owned(),
    }
  }
}
