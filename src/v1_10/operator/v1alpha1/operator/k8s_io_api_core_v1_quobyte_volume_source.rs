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

/// K8sIoApiCoreV1QuobyteVolumeSource : Represents a Quobyte mount that lasts the lifetime of a pod. Quobyte volumes do not support ownership management or SELinux relabeling.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct K8sIoApiCoreV1QuobyteVolumeSource {
    /// Group to map volume access to Default is no group +optional
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// ReadOnly here will force the Quobyte volume to be mounted with read-only permissions. Defaults to false. +optional
    #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// Registry represents a single or multiple Quobyte Registry services specified as a string as host:port pair (multiple entries are separated with commas) which acts as the central registry for volumes
    #[serde(rename = "registry", skip_serializing_if = "Option::is_none")]
    pub registry: Option<String>,
    /// Tenant owning the given Quobyte volume in the Backend Used with dynamically provisioned Quobyte volumes, value is set by the plugin +optional
    #[serde(rename = "tenant", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
    /// User to map volume access to Defaults to serivceaccount user +optional
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Volume is a string that references an already created Quobyte volume by name.
    #[serde(rename = "volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,
}

impl K8sIoApiCoreV1QuobyteVolumeSource {
    /// Represents a Quobyte mount that lasts the lifetime of a pod. Quobyte volumes do not support ownership management or SELinux relabeling.
    pub fn new() -> K8sIoApiCoreV1QuobyteVolumeSource {
        K8sIoApiCoreV1QuobyteVolumeSource {
            group: None,
            read_only: None,
            registry: None,
            tenant: None,
            user: None,
            volume: None,
        }
    }
}