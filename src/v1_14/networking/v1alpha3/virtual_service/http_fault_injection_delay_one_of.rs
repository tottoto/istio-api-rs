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




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct HttpFaultInjectionDelayOneOf {
    /// Add a fixed delay before forwarding the request. Format: 1h/1m/1s/1ms. MUST be >=1ms.
    #[serde(rename = "fixedDelay")]
    pub fixed_delay: String,
}

impl HttpFaultInjectionDelayOneOf {
    pub fn new(fixed_delay: String) -> HttpFaultInjectionDelayOneOf {
        HttpFaultInjectionDelayOneOf {
            fixed_delay,
        }
    }
}