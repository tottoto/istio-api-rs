use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
/*
 * Configuration affecting edge load balancer.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1beta1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ServerTlsSettingsTlsProtocol : TLS protocol versions.

/// TLS protocol versions.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, JsonSchema)]
pub enum ServerTlsSettingsTlsProtocol {
    #[serde(rename = "TLS_AUTO")]
    TLSAUTO,
    #[serde(rename = "TLSV1_0")]
    TLSV10,
    #[serde(rename = "TLSV1_1")]
    TLSV11,
    #[serde(rename = "TLSV1_2")]
    TLSV12,
    #[serde(rename = "TLSV1_3")]
    TLSV13,

}

impl ToString for ServerTlsSettingsTlsProtocol {
    fn to_string(&self) -> String {
        match self {
            Self::TLSAUTO => String::from("TLS_AUTO"),
            Self::TLSV10 => String::from("TLSV1_0"),
            Self::TLSV11 => String::from("TLSV1_1"),
            Self::TLSV12 => String::from("TLSV1_2"),
            Self::TLSV13 => String::from("TLSV1_3"),
        }
    }
}

impl Default for ServerTlsSettingsTlsProtocol {
    fn default() -> ServerTlsSettingsTlsProtocol {
        Self::TLSAUTO
    }
}