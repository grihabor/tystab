#[macro_export]
macro_rules! table {
    ( $( $field_name:ident : $value:expr ),* $(,)* ) => {
        {
            // Abusing field_name by using it also as type's name.
            #[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
            #[allow(non_camel_case_types)]
            struct Table<$( $field_name ),*> {
                $(
                    $field_name: $field_name,
                )*
            }
            // impl Table<$( $field_name ),*> {
            //    counted_array!(const COLUMNS: [String; _   ] = [$( stringify!($field_name).into() ),*]);
            //}
            Table {
                $(
                    $field_name: Column::from($value),
                )*
            }
        }
    }
}
