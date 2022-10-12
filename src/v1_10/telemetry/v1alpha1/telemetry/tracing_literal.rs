use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1alpha1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct TracingLiteral {
    /// The tag value to use.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl TracingLiteral {
    pub fn new() -> TracingLiteral {
        TracingLiteral {
            value: None,
        }
    }
}