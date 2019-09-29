extern crate tystab;
use tystab::{table, Column};

#[test]
fn add_columns() {
    let table = table! {
        x: vec![1, 2, 3],
        y: vec![4, 5, 6],
    };
    let result = &table.x + &table.y;
    assert_eq!(Column::from(vec![5, 7, 9]), result)
}
