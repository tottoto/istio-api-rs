use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
/*
 * Provides configuration for individual workloads.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1beta1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ProxyImage : The following values are used to construct proxy image url. $hub/$image_name/$tag-$image_type example: docker.io/istio/proxyv2:1.11.1 or docker.io/istio/proxyv2:1.11.1-distroless This information was previously part of the Values API.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct ProxyImage {
    /// The image type of the image. Istio publishes default, debug, and distroless images. Other values are allowed if those image types (example: centos) are published to the specified hub. supported values: default, debug, distroless.
    #[serde(rename = "imageType", skip_serializing_if = "Option::is_none")]
    pub image_type: Option<String>,
}

impl ProxyImage {
    /// The following values are used to construct proxy image url. $hub/$image_name/$tag-$image_type example: docker.io/istio/proxyv2:1.11.1 or docker.io/istio/proxyv2:1.11.1-distroless This information was previously part of the Values API.
    pub fn new() -> ProxyImage {
        ProxyImage {
            image_type: None,
        }
    }
}