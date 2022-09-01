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

/// LoadBalancerSettings : Load balancing policies to apply for a specific destination. See Envoy's load balancing [documentation](https://www.envoyproxy.io/docs/envoy/latest/intro/arch_overview/upstream/load_balancing/load_balancing) for more details.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct LoadBalancerSettings {
    #[serde(rename = "localityLbSetting", skip_serializing_if = "Option::is_none")]
    pub locality_lb_setting: Option<Box<super::LocalityLoadBalancerSetting>>,
    /// Represents the warmup duration of Service. If set, the newly created endpoint of service remains in warmup mode starting from its creation time for the duration of this window and Istio progressively increases amount of traffic for that endpoint instead of sending proportional amount of traffic. This should be enabled for services that require warm up time to serve full production load with reasonable latency. Currently this is only supported for ROUND_ROBIN and LEAST_REQUEST load balancers.
    #[serde(rename = "warmupDurationSecs", skip_serializing_if = "Option::is_none")]
    pub warmup_duration_secs: Option<String>,
    #[serde(rename = "simple")]
    pub simple: super::LoadBalancerSettingsSimpleLb,
    #[serde(rename = "consistentHash")]
    pub consistent_hash: Box<super::LoadBalancerSettingsConsistentHashLb>,
}

impl LoadBalancerSettings {
    /// Load balancing policies to apply for a specific destination. See Envoy's load balancing [documentation](https://www.envoyproxy.io/docs/envoy/latest/intro/arch_overview/upstream/load_balancing/load_balancing) for more details.
    pub fn new(simple: super::LoadBalancerSettingsSimpleLb, consistent_hash: super::LoadBalancerSettingsConsistentHashLb) -> LoadBalancerSettings {
        LoadBalancerSettings {
            locality_lb_setting: None,
            warmup_duration_secs: None,
            simple,
            consistent_hash: Box::new(consistent_hash),
        }
    }
}