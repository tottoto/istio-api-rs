use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
/*
 * Describes a collection of workload instances.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1beta1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct HttpHealthCheckConfig {
    /// Host name to connect to, defaults to the pod IP. You probably want to set \"Host\" in httpHeaders instead.
    #[serde(rename = "host", skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Headers the proxy will pass on to make the request. Allows repeated headers.
    #[serde(rename = "httpHeaders", skip_serializing_if = "Option::is_none")]
    pub http_headers: Option<Vec<super::HttpHeader>>,
    /// Path to access on the HTTP server.
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Port on which the endpoint lives.
    #[serde(rename = "port", skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// HTTP or HTTPS, defaults to HTTP
    #[serde(rename = "scheme", skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
}

impl HttpHealthCheckConfig {
    pub fn new() -> HttpHealthCheckConfig {
        HttpHealthCheckConfig {
            host: None,
            http_headers: None,
            path: None,
            port: None,
            scheme: None,
        }
    }
}