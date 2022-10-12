use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
/*
 * Telemetry configuration for workloads.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1alpha1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct TracingCustomTagOneOf1 {
    #[serde(rename = "environment")]
    pub environment: Box<super::TracingEnvironment>,
}

impl TracingCustomTagOneOf1 {
    pub fn new(environment: super::TracingEnvironment) -> TracingCustomTagOneOf1 {
        TracingCustomTagOneOf1 {
            environment: Box::new(environment),
        }
    }
}