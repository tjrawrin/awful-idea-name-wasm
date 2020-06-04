//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate awful_idea_name;
extern crate regex;
extern crate wasm_bindgen_test;

use awful_idea_name::AwfulIdeaName;
use regex::Regex;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn it_returns_a_random_name() {
	let a = AwfulIdeaName::new();
	let name1 = a.generate();
	let name2 = a.generate();

	assert!(!(&name1 == &name2))
}

#[wasm_bindgen_test]
fn it_contains_four_digits_at_the_end() {
	let a = AwfulIdeaName::new();
	let name = a.generate();
	let re = Regex::new(r"^\w+-\w+-[0-9]{4}$").unwrap();

	assert!(re.is_match(&name));
}
