use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
/*
 * Configuration for access control on workloads.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1beta1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct AuthorizationPolicyExtensionProvider {
    /// Specifies the name of the extension provider. The list of available providers is defined in the MeshConfig. Note, currently at most 1 extension provider is allowed per workload. Different workloads can use different extension provider.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl AuthorizationPolicyExtensionProvider {
    pub fn new() -> AuthorizationPolicyExtensionProvider {
        AuthorizationPolicyExtensionProvider {
            name: None,
        }
    }
}