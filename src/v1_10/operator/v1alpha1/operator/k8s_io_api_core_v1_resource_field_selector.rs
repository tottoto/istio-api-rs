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

/// K8sIoApiCoreV1ResourceFieldSelector : ResourceFieldSelector represents container resources (cpu, memory) and their output format



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct K8sIoApiCoreV1ResourceFieldSelector {
    /// Container name: required for volumes, optional for env vars +optional
    #[serde(rename = "containerName", skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[serde(rename = "divisor", skip_serializing_if = "Option::is_none")]
    pub divisor: Option<Box<super::K8sIoApimachineryPkgApiResourceQuantity>>,
    /// Required: resource to select
    #[serde(rename = "resource", skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}

impl K8sIoApiCoreV1ResourceFieldSelector {
    /// ResourceFieldSelector represents container resources (cpu, memory) and their output format
    pub fn new() -> K8sIoApiCoreV1ResourceFieldSelector {
        K8sIoApiCoreV1ResourceFieldSelector {
            container_name: None,
            divisor: None,
            resource: None,
        }
    }
}