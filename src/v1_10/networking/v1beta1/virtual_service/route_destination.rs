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

/// RouteDestination : L4 routing rule weighted destination.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct RouteDestination {
    #[serde(rename = "destination", skip_serializing_if = "Option::is_none")]
    pub destination: Option<Box<super::Destination>>,
    /// The proportion of traffic to be forwarded to the service version. If there is only one destination in a rule, all traffic will be routed to it irrespective of the weight.
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}

impl RouteDestination {
    /// L4 routing rule weighted destination.
    pub fn new() -> RouteDestination {
        RouteDestination {
            destination: None,
            weight: None,
        }
    }
}