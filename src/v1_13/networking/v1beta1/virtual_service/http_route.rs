use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
/*
 * Configuration affecting label/content routing, sni routing, etc.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1beta1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// HttpRoute : Describes match conditions and actions for routing HTTP/1.1, HTTP2, and gRPC traffic. See VirtualService for usage examples.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct HttpRoute {
    /// The name assigned to the route for debugging purposes. The route's name will be concatenated with the match's name and will be logged in the access logs for requests matching this route/match.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Match conditions to be satisfied for the rule to be activated. All conditions inside a single match block have AND semantics, while the list of match blocks have OR semantics. The rule is matched if any one of the match blocks succeed.
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    pub _match: Option<Vec<super::HttpMatchRequest>>,
    /// A HTTP rule can either redirect or forward (default) traffic. The forwarding target can be one of several versions of a service (see glossary in beginning of document). Weights associated with the service version determine the proportion of traffic it receives.
    #[serde(rename = "route", skip_serializing_if = "Option::is_none")]
    pub route: Option<Vec<super::HttpRouteDestination>>,
    #[serde(rename = "redirect", skip_serializing_if = "Option::is_none")]
    pub redirect: Option<Box<super::HttpRedirect>>,
    #[serde(rename = "delegate", skip_serializing_if = "Option::is_none")]
    pub delegate: Option<Box<super::Delegate>>,
    #[serde(rename = "rewrite", skip_serializing_if = "Option::is_none")]
    pub rewrite: Option<Box<super::HttpRewrite>>,
    /// Timeout for HTTP requests, default is disabled.
    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[serde(rename = "retries", skip_serializing_if = "Option::is_none")]
    pub retries: Option<Box<super::HttpRetry>>,
    #[serde(rename = "fault", skip_serializing_if = "Option::is_none")]
    pub fault: Option<Box<super::HttpFaultInjection>>,
    #[serde(rename = "mirror", skip_serializing_if = "Option::is_none")]
    pub mirror: Option<Box<super::Destination>>,
    /// Percentage of the traffic to be mirrored by the `mirror` field. Use of integer `mirror_percent` value is deprecated. Use the double `mirror_percentage` field instead $hide_from_docs
    #[serde(rename = "mirrorPercent", skip_serializing_if = "Option::is_none")]
    pub mirror_percent: Option<i32>,
    #[serde(rename = "mirrorPercentage", skip_serializing_if = "Option::is_none")]
    pub mirror_percentage: Option<Box<super::Percent>>,
    #[serde(rename = "corsPolicy", skip_serializing_if = "Option::is_none")]
    pub cors_policy: Option<Box<super::CorsPolicy>>,
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none")]
    pub headers: Option<Box<super::Headers>>,
}

impl HttpRoute {
    /// Describes match conditions and actions for routing HTTP/1.1, HTTP2, and gRPC traffic. See VirtualService for usage examples.
    pub fn new() -> HttpRoute {
        HttpRoute {
            name: None,
            _match: None,
            route: None,
            redirect: None,
            delegate: None,
            rewrite: None,
            timeout: None,
            retries: None,
            fault: None,
            mirror: None,
            mirror_percent: None,
            mirror_percentage: None,
            cors_policy: None,
            headers: None,
        }
    }
}