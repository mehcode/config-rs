use config::{Config, FileFormat};
use serde::Deserialize;
use std::fmt::Debug;

const ALL_CFG_FILE: &str = "./tests/all.env";

#[derive(Debug, Deserialize)]
struct TestConfig {
    #[serde(alias = "TEST__NUMBER")]
    pub number: i32,
    #[serde(alias = "TEST__STRING")]
    pub string: String,
    #[serde(alias = "TEST__FLOAT")]
    pub float: f32,
    #[serde(alias = "TEST__BOOLEAN")]
    pub boolean: bool,
    #[serde(flatten)]
    pub segment: TestSegmentConfig,
}

#[derive(Debug, Deserialize)]
struct TestSegmentConfig {
    #[serde(alias = "TEST__SEGMENT_NUMBER")]
    pub segment_number: i32,
    #[serde(alias = "TEST__SEGMENT_STRING")]
    pub segment_string: String,
    #[serde(alias = "TEST__SEGMENT_FLOAT")]
    pub segment_float: f32,
    #[serde(alias = "TEST__SEGMENT_BOOLEAN")]
    pub segment_boolean: bool,
    #[serde(flatten)]
    pub segment_child: TestSegmentChildConfig,
    #[serde(flatten)]
    pub segment_child2: TestSegmentChildConfig2,
}

#[derive(Debug, Deserialize)]
struct TestSegmentChildConfig {
    #[serde(alias = "TEST__SEGMENT_CHILD_BOOLEAN")]
    pub segment_child_bool: bool,
    #[serde(alias = "TEST__SEGMENT_CHILD_FLOAT")]
    pub segment_child_float: f64,
}

#[derive(Debug, Deserialize)]
struct TestSegmentChildConfig2 {
    #[serde(alias = "TEST__SEGMENT_CHILD2_BOOLEAN")]
    pub segment_child_number: u64,
    #[serde(alias = "TEST__SEGMENT_CHILD2_FLOAT")]
    pub segment_child_float: f64,
    #[serde(flatten)]
    pub segment_child2_child: TestSegmentChildConfig2Child,
}

#[derive(Debug, Deserialize)]
struct TestSegmentChildConfig2Child {
    #[serde(alias = "TEST__SEGMENT_CHILD2_CHILD_FLOAT")]
    pub segment_child2_child: f32,
}

#[test]
fn test_file_config_success() {
    let cfg = Config::builder()
        .add_source(config::File::new(ALL_CFG_FILE, FileFormat::Ini))
        .build()
        .unwrap()
        .try_deserialize::<TestConfig>();
    dbg!(&cfg);
    assert!(cfg.is_ok());
}
