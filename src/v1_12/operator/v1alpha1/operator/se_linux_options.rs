use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
/*
 * Configuration affecting Istio control plane installation version and shape.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1alpha1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SeLinuxOptions : See k8s.io.api.core.v1.SELinuxOptions.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct SeLinuxOptions {
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl SeLinuxOptions {
    /// See k8s.io.api.core.v1.SELinuxOptions.
    pub fn new() -> SeLinuxOptions {
        SeLinuxOptions {
            level: None,
            role: None,
            _type: None,
            user: None,
        }
    }
}