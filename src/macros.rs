#[macro_export]
macro_rules! enum_with_order {
    (
        #[derive($($traits:tt)*)]
        enum $name:ident in $order_name:ident {
            $($items:tt)*
        }
    ) => {
        #[derive($($traits)*)]
        pub enum $name {
            $($items)*
        }

        pub const $order_name: &[$name] = {
            use $name::*;
            &[$($items)*]
        };
    };
}
