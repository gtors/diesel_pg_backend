use diesel::expression::SelectableExpression;
use diesel::query_source::Table;


/// The `distinct_on` method
///
/// This trait should not be relied on directly by most apps. Its behavior is
/// provided by [`QueryDsl`]. However, you may need a where clause on this trait
/// to call `distinct_on` from generic code.
///
/// [`QueryDsl`]: ../trait.QueryDsl.html
pub trait DistinctOnDsl<Selection> {
    /// The type returned by `.distinct_on`
    type Output;

    /// See the trait documentation
    fn distinct_on(self, selection: Selection) -> Self::Output;
}

impl<T, Selection> DistinctOnDsl<Selection> for T
where
    Selection: SelectableExpression<T>,
    T: Table,
    T::Query: DistinctOnDsl<Selection>,
{
    type Output = <T::Query as DistinctOnDsl<Selection>>::Output;

    fn distinct_on(self, selection: Selection) -> Self::Output {
        self.as_query().distinct_on(selection)
    }
}
