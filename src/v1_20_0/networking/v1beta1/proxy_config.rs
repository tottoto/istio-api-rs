// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -Af resources/istio/v1_20_0/networking/v1beta1/ProxyConfig.yaml --api-version v1beta1 -D Default
// kopium version: 0.16.2

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// Provides configuration for individual workloads. See more details at: https://istio.io/docs/reference/config/networking/proxy-config.html
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
#[kube(group = "networking.istio.io", version = "v1beta1", kind = "ProxyConfig", plural = "proxyconfigs")]
#[kube(namespaced)]
pub struct ProxyConfigSpec {
    /// The number of worker threads to run.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<i64>,
    /// Additional environment variables for the proxy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "environmentVariables")]
    pub environment_variables: Option<BTreeMap<String, String>>,
    /// Specifies the details of the proxy image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<ProxyConfigImage>,
    /// Optional.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<ProxyConfigSelector>,
}

/// Specifies the details of the proxy image.
#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
pub struct ProxyConfigImage {
    /// The image type of the image.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageType")]
    pub image_type: Option<String>,
}

/// Optional.
#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
pub struct ProxyConfigSelector {
    /// One or more labels that indicate a specific set of pods/VMs on which a policy should be applied.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

