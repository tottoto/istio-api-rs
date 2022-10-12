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

/// IstioComponentSetSpec : IstioComponentSpec defines the desired installed state of Istio components.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct IstioComponentSetSpec {
    #[serde(rename = "base", skip_serializing_if = "Option::is_none")]
    pub base: Option<Box<super::BaseComponentSpec>>,
    #[serde(rename = "cni", skip_serializing_if = "Option::is_none")]
    pub cni: Option<Box<super::ComponentSpec>>,
    #[serde(rename = "egressGateways", skip_serializing_if = "Option::is_none")]
    pub egress_gateways: Option<Vec<super::GatewaySpec>>,
    #[serde(rename = "ingressGateways", skip_serializing_if = "Option::is_none")]
    pub ingress_gateways: Option<Vec<super::GatewaySpec>>,
    #[serde(rename = "istiodRemote", skip_serializing_if = "Option::is_none")]
    pub istiod_remote: Option<Box<super::ComponentSpec>>,
    #[serde(rename = "pilot", skip_serializing_if = "Option::is_none")]
    pub pilot: Option<Box<super::ComponentSpec>>,
}

impl IstioComponentSetSpec {
    /// IstioComponentSpec defines the desired installed state of Istio components.
    pub fn new() -> IstioComponentSetSpec {
        IstioComponentSetSpec {
            base: None,
            cni: None,
            egress_gateways: None,
            ingress_gateways: None,
            istiod_remote: None,
            pilot: None,
        }
    }
}