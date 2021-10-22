// /// The `ApiQuery` defines an API route that is intended to be used to retrieve data.
// #[proc_macro_derive(ApiQuery, attributes())]
// pub fn derive_api_query(_item: TokenStream) -> TokenStream {
//     TokenStream::new()
// }

// We will have here macros to help us to get the strong typed query path
// query!(api::query::get)
// This macro should return a function where you can set the params of the path if any and send the
// request to the server.
