pub use mk_file_config_core::FileConfig;
#[cfg(feature = "derive")]
pub use mk_file_config_derive::FileConfig;

#[cfg(feature = "derive")]
#[test]
fn test_derive_simple() {
    use std::fs::File;

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    enum TestEnum {
        TestVariant1,
        TestVariant2,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, FileConfig)]
    struct TestStruct {
        test_int: i32,
        test_str: String,
        test_enum: TestEnum,
    }

    impl Default for TestStruct {
        fn default() -> Self {
            TestStruct {
                test_int: 0,
                test_str: "".to_owned(),
                test_enum: TestEnum::TestVariant1,
            }
        }
    }

    let file_config = TestStruct::init().unwrap();
    let file_config_default = TestStruct::default();
    assert_eq!(file_config, file_config_default);

    std::fs::remove_file("config.yml").unwrap()
}
