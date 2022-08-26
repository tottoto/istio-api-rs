use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
/*
 * Configuration affecting load balancing, outlier detection, etc.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1alpha3
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct LoadBalancerSettingsConsistentHashLbOneOf {
    /// Hash based on a specific HTTP header.
    #[serde(rename = "httpHeaderName")]
    pub http_header_name: String,
}

impl LoadBalancerSettingsConsistentHashLbOneOf {
    pub fn new(http_header_name: String) -> LoadBalancerSettingsConsistentHashLbOneOf {
        LoadBalancerSettingsConsistentHashLbOneOf {
            http_header_name,
        }
    }
}