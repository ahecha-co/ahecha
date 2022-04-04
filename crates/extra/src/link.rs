// TODO: Figure out how to integrate with the html! macro.
// TODO: Figure out how to check that the href path is available in the routes.
pub struct Link<T>
where
  T: TypedPath,
{
  href: T,
  children: Children,
}
