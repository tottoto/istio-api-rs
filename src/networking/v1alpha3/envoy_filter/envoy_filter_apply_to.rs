use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
/*
 * Customizing Envoy configuration generated by Istio.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1alpha3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EnvoyFilterApplyTo : `ApplyTo` specifies where in the Envoy configuration, the given patch should be applied.

/// `ApplyTo` specifies where in the Envoy configuration, the given patch should be applied.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, JsonSchema)]
pub enum EnvoyFilterApplyTo {
    #[serde(rename = "INVALID")]
    INVALID,
    #[serde(rename = "LISTENER")]
    LISTENER,
    #[serde(rename = "FILTER_CHAIN")]
    FILTERCHAIN,
    #[serde(rename = "NETWORK_FILTER")]
    NETWORKFILTER,
    #[serde(rename = "HTTP_FILTER")]
    HTTPFILTER,
    #[serde(rename = "ROUTE_CONFIGURATION")]
    ROUTECONFIGURATION,
    #[serde(rename = "VIRTUAL_HOST")]
    VIRTUALHOST,
    #[serde(rename = "HTTP_ROUTE")]
    HTTPROUTE,
    #[serde(rename = "CLUSTER")]
    CLUSTER,
    #[serde(rename = "EXTENSION_CONFIG")]
    EXTENSIONCONFIG,
    #[serde(rename = "BOOTSTRAP")]
    BOOTSTRAP,

}

impl ToString for EnvoyFilterApplyTo {
    fn to_string(&self) -> String {
        match self {
            Self::INVALID => String::from("INVALID"),
            Self::LISTENER => String::from("LISTENER"),
            Self::FILTERCHAIN => String::from("FILTER_CHAIN"),
            Self::NETWORKFILTER => String::from("NETWORK_FILTER"),
            Self::HTTPFILTER => String::from("HTTP_FILTER"),
            Self::ROUTECONFIGURATION => String::from("ROUTE_CONFIGURATION"),
            Self::VIRTUALHOST => String::from("VIRTUAL_HOST"),
            Self::HTTPROUTE => String::from("HTTP_ROUTE"),
            Self::CLUSTER => String::from("CLUSTER"),
            Self::EXTENSIONCONFIG => String::from("EXTENSION_CONFIG"),
            Self::BOOTSTRAP => String::from("BOOTSTRAP"),
        }
    }
}

impl Default for EnvoyFilterApplyTo {
    fn default() -> EnvoyFilterApplyTo {
        Self::INVALID
    }
}