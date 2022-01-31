#[derive(Debug, PartialEq, Clone, Default)]
pub struct Select<'a> {
  pub(crate) distinct: bool,
  pub(crate) tables: Vec<Table<'a>>,
  pub(crate) columns: Vec<Expression<'a>>,
  pub(crate) conditions: Option<ConditionTree<'a>>,
  pub(crate) ordering: Ordering<'a>,
  pub(crate) grouping: Grouping<'a>,
  pub(crate) having: Option<ConditionTree<'a>>,
  pub(crate) limit: Option<Value<'a>>,
  pub(crate) offset: Option<Value<'a>>,
  pub(crate) joins: Vec<Join<'a>>,
  pub(crate) ctes: Vec<CommonTableExpression<'a>>,
  pub(crate) comment: Option<Cow<'a, str>>,
}
