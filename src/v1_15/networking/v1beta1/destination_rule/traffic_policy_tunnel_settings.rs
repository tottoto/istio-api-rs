use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
/*
 * Configuration affecting load balancing, outlier detection, etc.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1beta1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct TrafficPolicyTunnelSettings {
    /// Specifies which protocol to use for tunneling the downstream connection. Supported protocols are: CONNECT - uses HTTP CONNECT; POST - uses HTTP POST. CONNECT is used by default if not specified. HTTP version for upstream requests is determined by the service protocol defined for the proxy.
    #[serde(rename = "protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// Specifies a host to which the downstream connection is tunneled. Target host must be an FQDN or IP address.
    #[serde(rename = "targetHost", skip_serializing_if = "Option::is_none")]
    pub target_host: Option<String>,
    /// Specifies a port to which the downstream connection is tunneled.
    #[serde(rename = "targetPort", skip_serializing_if = "Option::is_none")]
    pub target_port: Option<i32>,
}

impl TrafficPolicyTunnelSettings {
    pub fn new() -> TrafficPolicyTunnelSettings {
        TrafficPolicyTunnelSettings {
            protocol: None,
            target_host: None,
            target_port: None,
        }
    }
}