pub use impls::*;

#[macro_use]
mod impls {
    #[doc(hidden)]
    #[macro_export]
    macro_rules! declare_test {
        ($name:ident, $ty:ty, $xml:expr, $obj:expr) => {
            #[cfg(test)]
            #[test]
            fn $name() {
                const XML: &str = $xml;
                let obj = $obj;
                pretty_assertions::assert_eq!($crate::from_str::<$ty>(XML).unwrap(), obj);
                pretty_assertions::assert_eq!($crate::to_string(&obj).unwrap(), XML);
            }
        };
    }
}
