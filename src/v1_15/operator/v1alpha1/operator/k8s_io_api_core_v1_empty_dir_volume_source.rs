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

/// K8sIoApiCoreV1EmptyDirVolumeSource : Represents an empty directory for a pod. Empty directory volumes support ownership management and SELinux relabeling.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct K8sIoApiCoreV1EmptyDirVolumeSource {
    /// medium represents what type of storage medium should back this directory. The default is \"\" which means to use the node's default medium. Must be an empty string (default) or Memory. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir +optional
    #[serde(rename = "medium", skip_serializing_if = "Option::is_none")]
    pub medium: Option<String>,
    #[serde(rename = "sizeLimit", skip_serializing_if = "Option::is_none")]
    pub size_limit: Option<Box<super::K8sIoApimachineryPkgApiResourceQuantity>>,
}

impl K8sIoApiCoreV1EmptyDirVolumeSource {
    /// Represents an empty directory for a pod. Empty directory volumes support ownership management and SELinux relabeling.
    pub fn new() -> K8sIoApiCoreV1EmptyDirVolumeSource {
        K8sIoApiCoreV1EmptyDirVolumeSource {
            medium: None,
            size_limit: None,
        }
    }
}