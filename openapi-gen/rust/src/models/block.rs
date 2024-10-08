/*
 * Rust Playground API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Block {
    #[serde(rename = "block_id")]
    pub block_id: String,
    #[serde(rename = "height")]
    pub height: i32,
}

impl Block {
    pub fn new(block_id: String, height: i32) -> Block {
        Block { block_id, height }
    }
}
