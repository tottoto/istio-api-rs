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




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct HttpFaultInjectionAbortOneOf2 {
    #[serde(rename = "http2Error")]
    pub http2_error: String,
}

impl HttpFaultInjectionAbortOneOf2 {
    pub fn new(http2_error: String) -> HttpFaultInjectionAbortOneOf2 {
        HttpFaultInjectionAbortOneOf2 {
            http2_error,
        }
    }
}