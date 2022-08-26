use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
/*
 * Configuration affecting load balancing, outlier detection, etc.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1beta1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LoadBalancerSettingsConsistentHashLb : Consistent Hash-based load balancing can be used to provide soft session affinity based on HTTP headers, cookies or other properties. The affinity to a particular destination host will be lost when one or more hosts are added/removed from the destination service.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct LoadBalancerSettingsConsistentHashLb {
    /// The minimum number of virtual nodes to use for the hash ring. Defaults to 1024. Larger ring sizes result in more granular load distributions. If the number of hosts in the load balancing pool is larger than the ring size, each host will be assigned a single virtual node.
    #[serde(rename = "minimumRingSize", skip_serializing_if = "Option::is_none")]
    pub minimum_ring_size: Option<i32>,
    /// Hash based on a specific HTTP header.
    #[serde(rename = "httpHeaderName")]
    pub http_header_name: String,
    #[serde(rename = "httpCookie")]
    pub http_cookie: Box<super::LoadBalancerSettingsConsistentHashLbHttpCookie>,
    /// Hash based on the source IP address. This is applicable for both TCP and HTTP connections.
    #[serde(rename = "useSourceIp")]
    pub use_source_ip: bool,
    /// Hash based on a specific HTTP query parameter.
    #[serde(rename = "httpQueryParameterName")]
    pub http_query_parameter_name: String,
}

impl LoadBalancerSettingsConsistentHashLb {
    /// Consistent Hash-based load balancing can be used to provide soft session affinity based on HTTP headers, cookies or other properties. The affinity to a particular destination host will be lost when one or more hosts are added/removed from the destination service.
    pub fn new(http_header_name: String, http_cookie: super::LoadBalancerSettingsConsistentHashLbHttpCookie, use_source_ip: bool, http_query_parameter_name: String) -> LoadBalancerSettingsConsistentHashLb {
        LoadBalancerSettingsConsistentHashLb {
            minimum_ring_size: None,
            http_header_name,
            http_cookie: Box::new(http_cookie),
            use_source_ip,
            http_query_parameter_name,
        }
    }
}