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

/// K8sIoApiCoreV1ConfigMapVolumeSource : Adapts a ConfigMap into a volume.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct K8sIoApiCoreV1ConfigMapVolumeSource {
    /// defaultMode is optional: mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Defaults to 0644. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set. +optional
    #[serde(rename = "defaultMode", skip_serializing_if = "Option::is_none")]
    pub default_mode: Option<i32>,
    /// items if unspecified, each key-value pair in the Data field of the referenced ConfigMap will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the ConfigMap, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'. +optional
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<super::K8sIoApiCoreV1KeyToPath>>,
    #[serde(rename = "localObjectReference", skip_serializing_if = "Option::is_none")]
    pub local_object_reference: Option<Box<super::K8sIoApiCoreV1LocalObjectReference>>,
    /// optional specify whether the ConfigMap or its keys must be defined +optional
    #[serde(rename = "optional", skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

impl K8sIoApiCoreV1ConfigMapVolumeSource {
    /// Adapts a ConfigMap into a volume.
    pub fn new() -> K8sIoApiCoreV1ConfigMapVolumeSource {
        K8sIoApiCoreV1ConfigMapVolumeSource {
            default_mode: None,
            items: None,
            local_object_reference: None,
            optional: None,
        }
    }
}