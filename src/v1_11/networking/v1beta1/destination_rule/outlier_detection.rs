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

/// OutlierDetection : A Circuit breaker implementation that tracks the status of each individual host in the upstream service. Applicable to both HTTP and TCP services. For HTTP services, hosts that continually return 5xx errors for API calls are ejected from the pool for a pre-defined period of time. For TCP services, connection timeouts or connection failures to a given host counts as an error when measuring the consecutive errors metric. See Envoy's [outlier detection](https://www.envoyproxy.io/docs/envoy/latest/intro/arch_overview/upstream/outlier) for more details.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct OutlierDetection {
    /// Minimum ejection duration. A host will remain ejected for a period equal to the product of minimum ejection duration and the number of times the host has been ejected. This technique allows the system to automatically increase the ejection period for unhealthy upstream servers. format: 1h/1m/1s/1ms. MUST BE >=1ms. Default is 30s.
    #[serde(rename = "baseEjectionTime", skip_serializing_if = "Option::is_none")]
    pub base_ejection_time: Option<String>,
    /// Number of 5xx errors before a host is ejected from the connection pool. When the upstream host is accessed over an opaque TCP connection, connect timeouts, connection error/failure and request failure events qualify as a 5xx error. This feature defaults to 5 but can be disabled by setting the value to 0.
    #[serde(rename = "consecutive5xxErrors", skip_serializing_if = "Option::is_none")]
    pub consecutive5xx_errors: Option<i32>,
    /// Number of errors before a host is ejected from the connection pool. Defaults to 5. When the upstream host is accessed over HTTP, a 502, 503, or 504 return code qualifies as an error. When the upstream host is accessed over an opaque TCP connection, connect timeouts and connection error/failure events qualify as an error. $hide_from_docs
    #[serde(rename = "consecutiveErrors", skip_serializing_if = "Option::is_none")]
    pub consecutive_errors: Option<i32>,
    /// Number of gateway errors before a host is ejected from the connection pool. When the upstream host is accessed over HTTP, a 502, 503, or 504 return code qualifies as a gateway error. When the upstream host is accessed over an opaque TCP connection, connect timeouts and connection error/failure events qualify as a gateway error. This feature is disabled by default or when set to the value 0.
    #[serde(rename = "consecutiveGatewayErrors", skip_serializing_if = "Option::is_none")]
    pub consecutive_gateway_errors: Option<i32>,
    /// The number of consecutive locally originated failures before ejection occurs. Defaults to 5. Parameter takes effect only when split_external_local_origin_errors is set to true.
    #[serde(rename = "consecutiveLocalOriginFailures", skip_serializing_if = "Option::is_none")]
    pub consecutive_local_origin_failures: Option<i32>,
    /// Time interval between ejection sweep analysis. format: 1h/1m/1s/1ms. MUST BE >=1ms. Default is 10s.
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// Maximum % of hosts in the load balancing pool for the upstream service that can be ejected. Defaults to 10%.
    #[serde(rename = "maxEjectionPercent", skip_serializing_if = "Option::is_none")]
    pub max_ejection_percent: Option<i32>,
    /// Outlier detection will be enabled as long as the associated load balancing pool has at least min_health_percent hosts in healthy mode. When the percentage of healthy hosts in the load balancing pool drops below this threshold, outlier detection will be disabled and the proxy will load balance across all hosts in the pool (healthy and unhealthy). The threshold can be disabled by setting it to 0%. The default is 0% as it's not typically applicable in k8s environments with few pods per service.
    #[serde(rename = "minHealthPercent", skip_serializing_if = "Option::is_none")]
    pub min_health_percent: Option<i32>,
    /// Determines whether to distinguish local origin failures from external errors. If set to true consecutive_local_origin_failure is taken into account for outlier detection calculations. This should be used when you want to derive the outlier detection status based on the errors seen locally such as failure to connect, timeout while connecting etc. rather than the status code retuned by upstream service. This is especially useful when the upstream service explicitly returns a 5xx for some requests and you want to ignore those responses from upstream service while determining the outlier detection status of a host. Defaults to false.
    #[serde(rename = "splitExternalLocalOriginErrors", skip_serializing_if = "Option::is_none")]
    pub split_external_local_origin_errors: Option<bool>,
}

impl OutlierDetection {
    /// A Circuit breaker implementation that tracks the status of each individual host in the upstream service. Applicable to both HTTP and TCP services. For HTTP services, hosts that continually return 5xx errors for API calls are ejected from the pool for a pre-defined period of time. For TCP services, connection timeouts or connection failures to a given host counts as an error when measuring the consecutive errors metric. See Envoy's [outlier detection](https://www.envoyproxy.io/docs/envoy/latest/intro/arch_overview/upstream/outlier) for more details.
    pub fn new() -> OutlierDetection {
        OutlierDetection {
            base_ejection_time: None,
            consecutive5xx_errors: None,
            consecutive_errors: None,
            consecutive_gateway_errors: None,
            consecutive_local_origin_failures: None,
            interval: None,
            max_ejection_percent: None,
            min_health_percent: None,
            split_external_local_origin_errors: None,
        }
    }
}