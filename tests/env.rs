extern crate config;

use config::*;
use std::env;

#[test]
fn test_default() {
    env::set_var("A_B_C", "abc");

    let environment = Environment::new();

    assert!(environment.collect().unwrap().contains_key("a_b_c"));

    env::remove_var("A_B_C");
}

#[test]
fn test_prefix_is_removed_from_key() {
    env::set_var("B_A_C", "abc");

    let environment = Environment::with_prefix("B_");

    assert!(environment.collect().unwrap().contains_key("a_c"));

    env::remove_var("B_A_C");
}

#[test]
fn test_separator_behavior() {
    env::set_var("C_B_A", "abc");

    let mut environment = Environment::with_prefix("C");
    environment.separator("_");

    assert!(environment.collect().unwrap().contains_key("b.a"));

    env::remove_var("C_B_A");
}
