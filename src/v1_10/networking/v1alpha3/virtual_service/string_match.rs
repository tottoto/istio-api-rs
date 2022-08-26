use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
/*
 * Configuration affecting label/content routing, sni routing, etc.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1alpha3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// StringMatch : Describes how to match a given string in HTTP headers. Match is case-sensitive.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct StringMatch {
    /// exact string match
    #[serde(rename = "exact")]
    pub exact: String,
    /// prefix-based match
    #[serde(rename = "prefix")]
    pub prefix: String,
    /// RE2 style regex-based match (https://github.com/google/re2/wiki/Syntax).
    #[serde(rename = "regex")]
    pub regex: String,
}

impl StringMatch {
    /// Describes how to match a given string in HTTP headers. Match is case-sensitive.
    pub fn new(exact: String, prefix: String, regex: String) -> StringMatch {
        StringMatch {
            exact,
            prefix,
            regex,
        }
    }
}