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

/// RollingUpdateDeployment : See k8s.io.api.apps.v1.RollingUpdateDeployment.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct RollingUpdateDeployment {
    /// Synthetic type for generating Go structs. GOTYPE: *IntOrStringForPB
    #[serde(rename = "maxSurge", skip_serializing_if = "Option::is_none")]
    pub max_surge: Option<serde_json::Value>,
    /// Synthetic type for generating Go structs. GOTYPE: *IntOrStringForPB
    #[serde(rename = "maxUnavailable", skip_serializing_if = "Option::is_none")]
    pub max_unavailable: Option<serde_json::Value>,
}

impl RollingUpdateDeployment {
    /// See k8s.io.api.apps.v1.RollingUpdateDeployment.
    pub fn new() -> RollingUpdateDeployment {
        RollingUpdateDeployment {
            max_surge: None,
            max_unavailable: None,
        }
    }
}