use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
/*
 * Configuration affecting label/content routing, sni routing, etc.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1alpha3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// HttpRetry : Describes the retry policy to use when a HTTP request fails. For example, the following rule sets the maximum number of retries to 3 when calling ratings:v1 service, with a 2s timeout per retry attempt. A retry will be attempted if there is a connect-failure, refused_stream or when the upstream server responds with Service Unavailable(503).



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct HttpRetry {
    /// Number of retries to be allowed for a given request. The interval between retries will be determined automatically (25ms+). When request `timeout` of the [HTTP route](https://istio.io/docs/reference/config/networking/virtual-service/#HTTPRoute) or `per_try_timeout` is configured, the actual number of retries attempted also depends on the specified request `timeout` and `per_try_timeout` values.
    #[serde(rename = "attempts", skip_serializing_if = "Option::is_none")]
    pub attempts: Option<i32>,
    /// Timeout per attempt for a given request, including the initial call and any retries. Format: 1h/1m/1s/1ms. MUST BE >=1ms. Default is same value as request `timeout` of the [HTTP route](https://istio.io/docs/reference/config/networking/virtual-service/#HTTPRoute), which means no timeout.
    #[serde(rename = "perTryTimeout", skip_serializing_if = "Option::is_none")]
    pub per_try_timeout: Option<String>,
    /// Specifies the conditions under which retry takes place. One or more policies can be specified using a ‘,’ delimited list. If retry_on specifies a valid HTTP status, it will be added to retriable_status_codes retry policy. See the [retry policies](https://www.envoyproxy.io/docs/envoy/latest/configuration/http/http_filters/router_filter#x-envoy-retry-on) and [gRPC retry policies](https://www.envoyproxy.io/docs/envoy/latest/configuration/http/http_filters/router_filter#x-envoy-retry-grpc-on) for more details.
    #[serde(rename = "retryOn", skip_serializing_if = "Option::is_none")]
    pub retry_on: Option<String>,
    /// Flag to specify whether the retries should retry to other localities. See the [retry plugin configuration](https://www.envoyproxy.io/docs/envoy/latest/intro/arch_overview/http/http_connection_management#retry-plugin-configuration) for more details.
    #[serde(rename = "retryRemoteLocalities", skip_serializing_if = "Option::is_none")]
    pub retry_remote_localities: Option<bool>,
}

impl HttpRetry {
    /// Describes the retry policy to use when a HTTP request fails. For example, the following rule sets the maximum number of retries to 3 when calling ratings:v1 service, with a 2s timeout per retry attempt. A retry will be attempted if there is a connect-failure, refused_stream or when the upstream server responds with Service Unavailable(503).
    pub fn new() -> HttpRetry {
        HttpRetry {
            attempts: None,
            per_try_timeout: None,
            retry_on: None,
            retry_remote_localities: None,
        }
    }
}