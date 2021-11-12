use ita::view::Render;
use ita_codegen::*;

mod ita {
  pub use ita_view as view;
}

#[test]
fn test_html_tag() {
  let res = html_parser! { <div attr="value">Hello</div> };
  assert_eq!(res.render(), "<div attr=\"value\">Hello</div>");
}

#[test]
fn test_html_tag_with_multiple_attributes() {
  let res = html_parser! { <div attr="value" attr2="value2">Hello</div> };
  assert_eq!(
    res.render(),
    "<div attr=\"value\" attr2=\"value2\">Hello</div>"
  );
}

#[test]
fn test_html_tag_nested() {
  let res = html_parser! {
    <div class="main">
      <h1 class="heading">I am a test</h1>
      <p class="paragraph">Lorem ipsum dolor sit amet.</p>
    </div>
  };
  assert_eq!(
    res.render(),
    "<div class=\"main\"><h1 class=\"heading\">I am a test</h1><p class=\"paragraph\">Lorem ipsum dolor sit amet.</p></div>"
  );
}

#[test]
fn test_custom_element_without_macro_attr() {
  struct CustomElement {}

  impl CustomElement {
    fn view(&self) -> String {
      html_parser!(
        <div class="main">I am a custom element</div>
      )
      .render()
    }
  }

  impl Render for CustomElement {
    fn render_into<W: std::fmt::Write>(self, writer: &mut W) -> std::fmt::Result {
      write!(writer, "{}", self.view())
    }
  }

  let res = html_parser! {
    <CustomElement></CustomElement>
  };

  assert_eq!(
    res.render(),
    "<custom-element><div class=\"main\">I am a custom element</div></custom-element>"
  );
}

#[test]
fn test_custom_element_with_macro_attr() {
  #[custom_element]
  fn CustomElement() {
    html_parser!(
      <div class="main">I am a custom element</div>
    )
    .render()
  }

  let res = html_parser! {
    <CustomElement></CustomElement>
  };

  assert_eq!(
    res.render(),
    "<custom-element><div class=\"main\">I am a custom element</div></custom-element>"
  );
}

#[test]
fn test_parse_bootstrap_sign_up_example() {
  let res = html_parser!(
    <!doctype html>
    <html lang="en">
      <head>
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <meta name="description" content="">
        <meta name="author" content="Mark Otto, Jacob Thornton, and Bootstrap contributors">
        <meta name="generator" content="Hugo 0.88.1">
        <title>Signin Template - Bootstrap v5.1</title>

        <link rel="canonical" href="https://getbootstrap.com/docs/5.1/examples/sign-in/">

        <!-- Bootstrap core CSS -->
        <link href="/docs/5.1/dist/css/bootstrap.min.css" rel="stylesheet" integrity="sha384-1BmE4kWBq78iYhFldvKuhfTAU6auU8tT94WrHftjDbrCEXSU1oBoqyl2QvZ6jIW3" crossorigin="anonymous">
          <!-- Favicons -->
        <link rel="apple-touch-icon" href="/docs/5.1/assets/img/favicons/apple-touch-icon.png" sizes="180x180">
        <link rel="icon" href="/docs/5.1/assets/img/favicons/favicon-32x32.png" sizes="32x32" type="image/png">
        <link rel="icon" href="/docs/5.1/assets/img/favicons/favicon-16x16.png" sizes="16x16" type="image/png">
        <link rel="manifest" href="/docs/5.1/assets/img/favicons/manifest.json">
        <link rel="mask-icon" href="/docs/5.1/assets/img/favicons/safari-pinned-tab.svg" color="#7952b3">
        <link rel="icon" href="/docs/5.1/assets/img/favicons/favicon.ico">
        <meta name="theme-color" content="#7952b3">

        <style>
          .bd-placeholder-img {
            font-size: 1.125rem;
            text-anchor: middle;
            -webkit-user-select: none;
            -moz-user-select: none;
            user-select: none;
          }

          @media (min-width: 768px) {
            .bd-placeholder-img-lg {
              font-size: 3.5rem;
            }
          }
        </style>

        <!-- Custom styles for this template -->
        <link href="signin.css" rel="stylesheet">
      </head>
      <body class="text-center">
        <main class="form-signin">
          <form>
            <img class="mb-4" src="/docs/5.1/assets/brand/bootstrap-logo.svg" alt="" width="72" height="57">
            <h1 class="h3 mb-3 fw-normal">Please sign in</h1>

            <div class="form-floating">
              <input type="email" class="form-control" id="floatingInput" placeholder="name@example.com">
              <label for="floatingInput">Email address</label>
            </div>
            <div class="form-floating">
              <input type="password" class="form-control" id="floatingPassword" placeholder="Password">
              <label for="floatingPassword">Password</label>
            </div>

            <div class="checkbox mb-3">
              <label>
                <input type="checkbox" value="remember-me"> Remember me
              </label>
            </div>
            <button class="w-100 btn btn-lg btn-primary" type="submit">Sign in</button>
            <p class="mt-5 mb-3 text-muted">&copy; 2017-2021</p>
          </form>
        </main>
      </body>
    </html>
  );
  assert_eq!(res.render(), "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"></meta><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"></meta><meta name=\"description\" content=\"\"></meta><meta name=\"author\" content=\"Mark Otto, Jacob Thornton, and Bootstrap contributors\"></meta><meta name=\"generator\" content=\"Hugo 0.88.1\"></meta><title>Signin Template - Bootstrapv5.1</title><meta rel=\"canonical\" href=\"https://getbootstrap.com/docs/5.1/examples/sign-in/\"></meta>&lt;!--  Bootstrap coreCSS  --&gt;--&gt;<meta href=\"/docs/5.1/dist/css/bootstrap.min.css\" rel=\"stylesheet\" integrity=\"sha384-1BmE4kWBq78iYhFldvKuhfTAU6auU8tT94WrHftjDbrCEXSU1oBoqyl2QvZ6jIW3\" crossorigin=\"anonymous\"></meta>&lt;!--  Favicons  --&gt;--&gt;<meta rel=\"apple-touch-icon\" href=\"/docs/5.1/assets/img/favicons/apple-touch-icon.png\" sizes=\"180x180\"></meta><meta rel=\"icon\" href=\"/docs/5.1/assets/img/favicons/favicon-32x32.png\" sizes=\"32x32\" type=\"image/png\"></meta><meta rel=\"icon\" href=\"/docs/5.1/assets/img/favicons/favicon-16x16.png\" sizes=\"16x16\" type=\"image/png\"></meta><meta rel=\"manifest\" href=\"/docs/5.1/assets/img/favicons/manifest.json\"></meta><meta rel=\"mask-icon\" href=\"/docs/5.1/assets/img/favicons/safari-pinned-tab.svg\" color=\"#7952b3\"></meta><meta rel=\"icon\" href=\"/docs/5.1/assets/img/favicons/favicon.ico\"></meta><meta name=\"theme-color\" content=\"#7952b3\"></meta><style>.bd - placeholder - img    font - size : 1.125rem ; text - anchor : middle ; - webkit - user - select    : none ; - moz - user - select : none ; user - select : none ;@ media(min - width : 768px) .bd - placeholder - img - lg { font - size : 3.5rem ; }</style>&lt;!-- Custom styles for this template  --&gt;--&gt;<meta href=\"signin.css\" rel=\"stylesheet\"></meta></head><body class=\"text-center\"><main class=\"form-signin\"><form><meta class=\"mb-4\" src=\"/docs/5.1/assets/brand/bootstrap-logo.svg\" alt=\"\" width=\"72\" height=\"57\"></meta><h1 class=\"h3 mb-3 fw-normal\">Please sign in</h1><div class=\"form-floating\"><meta type=\"email\" class=\"form-control\" id=\"floatingInput\" placeholder=\"name@example.com\"></meta><label for=\"floatingInput\">Email address</label></div><div class=\"form-floating\"><meta type=\"password\" class=\"form-control\" id=\"floatingPassword\" placeholder=\"Password\"></meta><label for=\"floatingPassword\">Password</label></div><div class=\"checkbox mb-3\"><label><meta type=\"checkbox\" value=\"remember-me\"></meta>Remember me</label></div><button class=\"w-100 btn btn-lg btn-primary\" type=\"submit\">Signin</button><p class=\"mt-5 mb-3 text-muted\">&amp; copy ; 2017 - 2021</p></form></main></body></html>");
}
