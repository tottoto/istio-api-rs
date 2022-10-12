use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
/*
 * Configuration affecting label/content routing, sni routing, etc.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1beta1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// HeadersHeaderOperations : HeaderOperations Describes the header manipulations to apply



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct HeadersHeaderOperations {
    /// Append the given values to the headers specified by keys (will create a comma-separated list of values)
    #[serde(rename = "add", skip_serializing_if = "Option::is_none")]
    pub add: Option<::std::collections::HashMap<String, String>>,
    /// Remove the specified headers
    #[serde(rename = "remove", skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<String>>,
    /// Overwrite the headers specified by key with the given values
    #[serde(rename = "set", skip_serializing_if = "Option::is_none")]
    pub set: Option<::std::collections::HashMap<String, String>>,
}

impl HeadersHeaderOperations {
    /// HeaderOperations Describes the header manipulations to apply
    pub fn new() -> HeadersHeaderOperations {
        HeadersHeaderOperations {
            add: None,
            remove: None,
            set: None,
        }
    }
}