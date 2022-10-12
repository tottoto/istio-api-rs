use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
/*
 * Configuration affecting network reachability of a sidecar.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1alpha3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ServerTlsSettingsTlSmode : TLS modes enforced by the proxy

/// TLS modes enforced by the proxy
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, JsonSchema)]
pub enum ServerTlsSettingsTlSmode {
    #[serde(rename = "PASSTHROUGH")]
    PASSTHROUGH,
    #[serde(rename = "SIMPLE")]
    SIMPLE,
    #[serde(rename = "MUTUAL")]
    MUTUAL,
    #[serde(rename = "AUTO_PASSTHROUGH")]
    AUTOPASSTHROUGH,
    #[serde(rename = "ISTIO_MUTUAL")]
    ISTIOMUTUAL,

}

impl ToString for ServerTlsSettingsTlSmode {
    fn to_string(&self) -> String {
        match self {
            Self::PASSTHROUGH => String::from("PASSTHROUGH"),
            Self::SIMPLE => String::from("SIMPLE"),
            Self::MUTUAL => String::from("MUTUAL"),
            Self::AUTOPASSTHROUGH => String::from("AUTO_PASSTHROUGH"),
            Self::ISTIOMUTUAL => String::from("ISTIO_MUTUAL"),
        }
    }
}

impl Default for ServerTlsSettingsTlSmode {
    fn default() -> ServerTlsSettingsTlSmode {
        Self::PASSTHROUGH
    }
}