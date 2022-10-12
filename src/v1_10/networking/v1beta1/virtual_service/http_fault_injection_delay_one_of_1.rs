use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
/*
 * Configuration affecting label/content routing, sni routing, etc.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1beta1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct HttpFaultInjectionDelayOneOf1 {
    #[serde(rename = "exponentialDelay")]
    pub exponential_delay: String,
}

impl HttpFaultInjectionDelayOneOf1 {
    pub fn new(exponential_delay: String) -> HttpFaultInjectionDelayOneOf1 {
        HttpFaultInjectionDelayOneOf1 {
            exponential_delay,
        }
    }
}