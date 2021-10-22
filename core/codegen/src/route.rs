use regex::Regex;

#[derive(Debug)]
pub(crate) struct PathRoute {
  pub path: String,
  pub path_params: Vec<String>,
}

pub(crate) fn path_route_builder(file_path: &str) -> Option<PathRoute> {
  let mut parts: Vec<String> = file_path[..(file_path.len() - 3)]
    .split('/')
    .map(|p| p.to_string())
    .collect();
  let mut path_params: Vec<String> = vec![];

  if let Some(route_index) = parts.iter().position(|p| p == "src") {
    let re = Regex::new(r"^__.*__$").unwrap();
    parts = parts[(route_index + 1)..]
      .to_vec()
      .iter()
      .map(|p| {
        if re.is_match(&p) {
          let param = &p[2..(p.len() - 2)];
          let res = format!("<{}>", param);
          path_params.push(format!("{}", param));
          res
        } else {
          p.clone()
        }
      })
      .collect::<Vec<_>>();

    // if parts.first().unwrap() != &"api" && parts.first().unwrap() != &"pages" {
    //   return None;
    // }

    if parts.contains(&"pages".to_string()) {
      parts = parts[1..].to_vec();
    }

    if let Some(file_name) = parts.last() {
      if file_name.ends_with(".rs") {
        let index = parts.len() - 1;
        parts[index] = file_name[..(file_name.len() - 3)].to_string();
      }
    }

    let re_path = Regex::new(r"<[^<>]*>").unwrap();
    let mut path = format!("/{}", re_path.replace_all(&parts.join("/"), "{}"));

    if path.ends_with("index") {
      path = path[0..path.len() - 5].to_string();
    }

    return Some(PathRoute { path, path_params });
  } else {
    None
  }
}

#[cfg(test)]
mod test {
  use crate::route::path_route_builder;

  #[test]
  fn simple_path_route_test() {
    let route = path_route_builder("src/api/query.rs").unwrap();
    assert_eq!("/api/query", route.path);
  }

  #[test]
  fn path_route_with_params_test() {
    let route = path_route_builder("src/api/query/__id__.rs").unwrap();
    assert_eq!("/api/query/{}", route.path);
    assert_eq!(vec!["id"], route.path_params);
  }

  #[test]
  fn path_route_with_multiple_params_test() {
    let route = path_route_builder("src/api/posts/__post_id__/comments/__comment_id__.rs").unwrap();
    assert_eq!("/api/posts/{}/comments/{}", route.path);
    assert_eq!(vec!["post_id", "comment_id"], route.path_params);
  }
}
