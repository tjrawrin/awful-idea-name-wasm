extern crate awful_idea_name;
extern crate rand;
extern crate wasm_bindgen;

use awful_idea_name::AwfulIdeaName;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn generate() {
    AwfulIdeaName::new().generate();
}
