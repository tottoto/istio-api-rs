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

/// ConnectionPoolSettingsHttpSettings : Settings applicable to HTTP1.1/HTTP2/GRPC connections.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct ConnectionPoolSettingsHttpSettings {
    #[serde(rename = "h2UpgradePolicy", skip_serializing_if = "Option::is_none")]
    pub h2_upgrade_policy: Option<super::ConnectionPoolSettingsHttpSettingsH2UpgradePolicy>,
    /// Maximum number of pending HTTP requests to a destination. Default 2^32-1.
    #[serde(rename = "http1MaxPendingRequests", skip_serializing_if = "Option::is_none")]
    pub http1_max_pending_requests: Option<i32>,
    /// Maximum number of requests to a backend. Default 2^32-1.
    #[serde(rename = "http2MaxRequests", skip_serializing_if = "Option::is_none")]
    pub http2_max_requests: Option<i32>,
    /// The idle timeout for upstream connection pool connections. The idle timeout is defined as the period in which there are no active requests. If not set, the default is 1 hour. When the idle timeout is reached, the connection will be closed. If the connection is an HTTP/2 connection a drain sequence will occur prior to closing the connection. Note that request based timeouts mean that HTTP/2 PINGs will not keep the connection alive. Applies to both HTTP1.1 and HTTP2 connections.
    #[serde(rename = "idleTimeout", skip_serializing_if = "Option::is_none")]
    pub idle_timeout: Option<String>,
    /// Maximum number of requests per connection to a backend. Setting this parameter to 1 disables keep alive. Default 0, meaning \"unlimited\", up to 2^29.
    #[serde(rename = "maxRequestsPerConnection", skip_serializing_if = "Option::is_none")]
    pub max_requests_per_connection: Option<i32>,
    /// Maximum number of retries that can be outstanding to all hosts in a cluster at a given time. Defaults to 2^32-1.
    #[serde(rename = "maxRetries", skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i32>,
    /// If set to true, client protocol will be preserved while initiating connection to backend. Note that when this is set to true, h2_upgrade_policy will be ineffective i.e. the client connections will not be upgraded to http2.
    #[serde(rename = "useClientProtocol", skip_serializing_if = "Option::is_none")]
    pub use_client_protocol: Option<bool>,
}

impl ConnectionPoolSettingsHttpSettings {
    /// Settings applicable to HTTP1.1/HTTP2/GRPC connections.
    pub fn new() -> ConnectionPoolSettingsHttpSettings {
        ConnectionPoolSettingsHttpSettings {
            h2_upgrade_policy: None,
            http1_max_pending_requests: None,
            http2_max_requests: None,
            idle_timeout: None,
            max_requests_per_connection: None,
            max_retries: None,
            use_client_protocol: None,
        }
    }
}