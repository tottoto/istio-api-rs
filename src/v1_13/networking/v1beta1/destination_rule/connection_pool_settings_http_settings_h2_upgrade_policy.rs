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

/// ConnectionPoolSettingsHttpSettingsH2UpgradePolicy : Policy for upgrading http1.1 connections to http2.

/// Policy for upgrading http1.1 connections to http2.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, JsonSchema)]
pub enum ConnectionPoolSettingsHttpSettingsH2UpgradePolicy {
    #[serde(rename = "DEFAULT")]
    DEFAULT,
    #[serde(rename = "DO_NOT_UPGRADE")]
    DONOTUPGRADE,
    #[serde(rename = "UPGRADE")]
    UPGRADE,

}

impl ToString for ConnectionPoolSettingsHttpSettingsH2UpgradePolicy {
    fn to_string(&self) -> String {
        match self {
            Self::DEFAULT => String::from("DEFAULT"),
            Self::DONOTUPGRADE => String::from("DO_NOT_UPGRADE"),
            Self::UPGRADE => String::from("UPGRADE"),
        }
    }
}

impl Default for ConnectionPoolSettingsHttpSettingsH2UpgradePolicy {
    fn default() -> ConnectionPoolSettingsHttpSettingsH2UpgradePolicy {
        Self::DEFAULT
    }
}