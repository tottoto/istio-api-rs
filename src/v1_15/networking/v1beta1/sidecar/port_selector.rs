use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
/*
 * Configuration affecting network reachability of a sidecar.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1beta1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PortSelector : PortSelector specifies the number of a port to be used for matching or selection for final routing.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct PortSelector {
    /// Valid port number
    #[serde(rename = "number", skip_serializing_if = "Option::is_none")]
    pub number: Option<i32>,
}

impl PortSelector {
    /// PortSelector specifies the number of a port to be used for matching or selection for final routing.
    pub fn new() -> PortSelector {
        PortSelector {
            number: None,
        }
    }
}