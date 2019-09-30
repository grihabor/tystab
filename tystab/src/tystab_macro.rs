#[macro_export]
macro_rules! table {
    ( $( $field_name:ident : $value:expr ),* $(,)* ) => {
        {
            // Abusing field_name by using it also as type's name.
            // #[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
            #[allow(non_camel_case_types)]
            struct Table<$( $field_name ),*> {
                $(
                    $field_name: $field_name,
                )*
            }
            impl<$( $field_name ),*> Table<$( $field_name ),*> {
                const COLUMNS: &'static str = stringify!($( $field_name ),*);
            }
            Table {
                $(
                    $field_name: Column::from($value),
                )*
            }
        }
    }
}
