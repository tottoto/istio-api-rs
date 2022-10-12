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

/// CorsPolicy : Describes the Cross-Origin Resource Sharing (CORS) policy, for a given service. Refer to [CORS](https://developer.mozilla.org/en-US/docs/Web/HTTP/Access_control_CORS) for further details about cross origin resource sharing. For example, the following rule restricts cross origin requests to those originating from example.com domain using HTTP POST/GET, and sets the `Access-Control-Allow-Credentials` header to false. In addition, it only exposes `X-Foo-bar` header and sets an expiry period of 1 day.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct CorsPolicy {
    /// Indicates whether the caller is allowed to send the actual request (not the preflight) using credentials. Translates to `Access-Control-Allow-Credentials` header.
    #[serde(rename = "allowCredentials", skip_serializing_if = "Option::is_none")]
    pub allow_credentials: Option<bool>,
    /// List of HTTP headers that can be used when requesting the resource. Serialized to Access-Control-Allow-Headers header.
    #[serde(rename = "allowHeaders", skip_serializing_if = "Option::is_none")]
    pub allow_headers: Option<Vec<String>>,
    /// List of HTTP methods allowed to access the resource. The content will be serialized into the Access-Control-Allow-Methods header.
    #[serde(rename = "allowMethods", skip_serializing_if = "Option::is_none")]
    pub allow_methods: Option<Vec<String>>,
    /// The list of origins that are allowed to perform CORS requests. The content will be serialized into the Access-Control-Allow-Origin header. Wildcard * will allow all origins. $hide_from_docs
    #[serde(rename = "allowOrigin", skip_serializing_if = "Option::is_none")]
    pub allow_origin: Option<Vec<String>>,
    /// String patterns that match allowed origins. An origin is allowed if any of the string matchers match. If a match is found, then the outgoing Access-Control-Allow-Origin would be set to the origin as provided by the client.
    #[serde(rename = "allowOrigins", skip_serializing_if = "Option::is_none")]
    pub allow_origins: Option<Vec<super::StringMatch>>,
    /// A list of HTTP headers that the browsers are allowed to access. Serialized into Access-Control-Expose-Headers header.
    #[serde(rename = "exposeHeaders", skip_serializing_if = "Option::is_none")]
    pub expose_headers: Option<Vec<String>>,
    /// Specifies how long the results of a preflight request can be cached. Translates to the `Access-Control-Max-Age` header.
    #[serde(rename = "maxAge", skip_serializing_if = "Option::is_none")]
    pub max_age: Option<String>,
}

impl CorsPolicy {
    /// Describes the Cross-Origin Resource Sharing (CORS) policy, for a given service. Refer to [CORS](https://developer.mozilla.org/en-US/docs/Web/HTTP/Access_control_CORS) for further details about cross origin resource sharing. For example, the following rule restricts cross origin requests to those originating from example.com domain using HTTP POST/GET, and sets the `Access-Control-Allow-Credentials` header to false. In addition, it only exposes `X-Foo-bar` header and sets an expiry period of 1 day.
    pub fn new() -> CorsPolicy {
        CorsPolicy {
            allow_credentials: None,
            allow_headers: None,
            allow_methods: None,
            allow_origin: None,
            allow_origins: None,
            expose_headers: None,
            max_age: None,
        }
    }
}