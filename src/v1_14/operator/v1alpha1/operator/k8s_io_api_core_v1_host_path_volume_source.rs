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

/// K8sIoApiCoreV1HostPathVolumeSource : Represents a host path mapped into a pod. Host path volumes do not support ownership management or SELinux relabeling.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct K8sIoApiCoreV1HostPathVolumeSource {
    /// path of the directory on the host. If the path is a symlink, it will follow the link to the real path. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// type for HostPath Volume Defaults to \"\" More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath +optional
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl K8sIoApiCoreV1HostPathVolumeSource {
    /// Represents a host path mapped into a pod. Host path volumes do not support ownership management or SELinux relabeling.
    pub fn new() -> K8sIoApiCoreV1HostPathVolumeSource {
        K8sIoApiCoreV1HostPathVolumeSource {
            path: None,
            _type: None,
        }
    }
}