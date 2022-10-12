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

/// K8sIoApiCoreV1VolumeProjection : Projection that may be projected along with other supported volume types



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct K8sIoApiCoreV1VolumeProjection {
    #[serde(rename = "configMap", skip_serializing_if = "Option::is_none")]
    pub config_map: Option<Box<super::K8sIoApiCoreV1ConfigMapProjection>>,
    #[serde(rename = "downwardAPI", skip_serializing_if = "Option::is_none")]
    pub downward_api: Option<Box<super::K8sIoApiCoreV1DownwardApiProjection>>,
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<Box<super::K8sIoApiCoreV1SecretProjection>>,
    #[serde(rename = "serviceAccountToken", skip_serializing_if = "Option::is_none")]
    pub service_account_token: Option<Box<super::K8sIoApiCoreV1ServiceAccountTokenProjection>>,
}

impl K8sIoApiCoreV1VolumeProjection {
    /// Projection that may be projected along with other supported volume types
    pub fn new() -> K8sIoApiCoreV1VolumeProjection {
        K8sIoApiCoreV1VolumeProjection {
            config_map: None,
            downward_api: None,
            secret: None,
            service_account_token: None,
        }
    }
}