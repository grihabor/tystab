extern crate tystab_macro;

use std::ops::Add;
pub use tystab_macro::table;

pub struct Table0;
pub struct Table1<A>(Column<A>);
pub struct Table2<A, B>(Column<A>, Column<B>);
pub struct Table3<A, B, C>(Column<A>, Column<B>, Column<C>);

trait WithColumn<C, R> {
    fn column(self, _: Column<C>) -> R;
}

impl<A> WithColumn<A, Table1<A>> for Table0 {
    fn column(self, col: Column<A>) -> Table1<A> {
        Table1(col)
    }
}

impl<A, B> WithColumn<B, Table2<A, B>> for Table1<A> {
    fn column(self, col: Column<B>) -> Table2<A, B> {
        Table2(self.0, col)
    }
}

impl<A, B, C> WithColumn<C, Table3<A, B, C>> for Table2<A, B> {
    fn column(self, col: Column<C>) -> Table3<A, B, C> {
        Table3(self.0, self.1, col)
    }
}

pub type Table = Table0;

impl Table {
    fn new() -> Self {
        Table {}
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Column<T>(T);

impl<T> From<Vec<T>> for Column<Vec<T>> {
    fn from(value: Vec<T>) -> Self {
        Column(value)
    }
}

impl<T: Add<Output = T> + Copy> Add for &Column<Vec<T>> {
    type Output = Column<Vec<T>>;

    fn add(self, rhs: Self) -> Self::Output {
        Column(
            self.0
                .iter()
                .zip(rhs.0.iter())
                .map(|(a, b)| *a + *b)
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{table, Column};

    #[test]
    fn add_columns() {
        let df = table! {
            x: vec![1, 2, 3],
            y: vec![4, 5, 6],
        };
        let result = &df.x + &df.y;
        let df = table! {
            x: df.x,
            y: df.y,
            s: result,
        };
        assert_eq!(Column::from(vec![5, 7, 9]), df.s)
    }
}
