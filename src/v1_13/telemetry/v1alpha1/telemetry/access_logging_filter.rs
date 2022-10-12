use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
/*
 * Telemetry configuration for workloads.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1alpha1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AccessLoggingFilter : Allows specification of an access log filter.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct AccessLoggingFilter {
    /// CEL expression for selecting when requests/connections should be logged.
    #[serde(rename = "expression", skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
}

impl AccessLoggingFilter {
    /// Allows specification of an access log filter.
    pub fn new() -> AccessLoggingFilter {
        AccessLoggingFilter {
            expression: None,
        }
    }
}