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




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct LoadBalancerSettingsOneOf {
    #[serde(rename = "simple")]
    pub simple: super::LoadBalancerSettingsSimpleLb,
}

impl LoadBalancerSettingsOneOf {
    pub fn new(simple: super::LoadBalancerSettingsSimpleLb) -> LoadBalancerSettingsOneOf {
        LoadBalancerSettingsOneOf {
            simple,
        }
    }
}