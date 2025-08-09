macro_rules! gen_write {
    ($dst:expr, $($args:tt)*) => {
        write!($dst, $($args)*).map_err(|err| {
            let loc = $crate::codegen::error::Location::new(file!(), line!());
            Into::<$crate::codegen::error::Error>::into((err, loc))
        })
    };
}
