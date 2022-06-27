use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
/*
 * Configuration for access control on workloads.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1beta1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RuleTo : To includes a list or operations.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct RuleTo {
    #[serde(rename = "operation", skip_serializing_if = "Option::is_none")]
    pub operation: Option<Box<super::Operation>>,
}

impl RuleTo {
    /// To includes a list or operations.
    pub fn new() -> RuleTo {
        RuleTo {
            operation: None,
        }
    }
}