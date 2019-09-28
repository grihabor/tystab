use std::ops::Add;

struct Table0;
struct Table1<A>(Column<A>);
struct Table2<A, B>(Column<A>, Column<B>);

trait WithColumn<C, R> {
    fn push(self, _: Column<C>) -> R;
}

impl<A> WithColumn<A, Table1<A>> for Table0 {
    fn push(self, col: Column<A>) -> Table1<A> {
        Table1(col)
    }
}

impl<A, B> WithColumn<B, Table2<A, B>> for Table1<A> {
    fn push(self, col: Column<B>) -> Table2<A, B> {
        Table2(self.0, col)
    }
}

type Table = Table0;

impl Table {
    fn new() -> Self {
        Table {}
    }
}

#[derive(Debug)]
struct Column<T>(T);

impl<T> From<T> for Column<T> {
    fn from(values: T) -> Self {
        Column(values)
    }
}

impl<'a, T: Add<Output=T> + Copy> Add for &Column<Vec<T>> {
    type Output = Column<Vec<T>>;

    fn add(self, rhs: Self) -> Self::Output {
        Column(
            self.0.iter()
            .zip(rhs.0.iter())
            .map(|(a, b)| *a + *b)
            .collect()
        )
    }
}

fn main() {
    let table = Table::new()
        .push(vec![1, 2, 3].into())
        .push(vec![2, 3, 4].into());
    println!("{:?}", &table.0 + &table.1)
}
