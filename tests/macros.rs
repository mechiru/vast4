#[macro_export]
macro_rules! __declare_test {
    ($ver:expr, $name:expr, $obj:expr) => {
        #[test]
        fn read_and_write() {
            pub use std::borrow::Cow;

            pub use vast4::*;

            let xml = include_str!(concat!("../data/", $ver, "/", $name, ".xml"));
            let vast = vast4::from_str::<vast4::Vast>(xml).unwrap();
            pretty_assertions::assert_eq!($obj, vast);
            let xml = vast4::to_string(&vast).unwrap();
            std::fs::create_dir_all(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/output/", $ver))
                .unwrap();
            let file =
                concat!(env!("CARGO_MANIFEST_DIR"), "/tests/output/", $ver, "/", $name, ".xml");
            std::fs::write(file, xml).unwrap();
            let file = std::fs::read_to_string(file).unwrap();
            let from_file = vast4::from_str::<vast4::Vast>(&file).unwrap();
            pretty_assertions::assert_eq!(vast, from_file);
        }
    };
}

#[macro_export]
macro_rules! declare_test_v4_2 {
    ($name:expr, $obj:expr) => {
        __declare_test!("v4_2", $name, $obj);
    };
}
