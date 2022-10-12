use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
/*
 * Request authentication configuration for workloads.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1beta1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RequestAuthenticationSpec : RequestAuthenticationSpec defines what request authentication methods are supported by a workload. It will reject a request if the request contains invalid authentication information, based on the configured authentication rules. A request that does not contain any authentication credentials will be accepted but will not have any authenticated identity. To restrict access to authenticated requests only, this should be accompanied by an authorization rule. Examples: - Require JWT for all request for workloads that have label `app:httpbin`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema, CustomResource)]
#[kube(group = "security.istio.io", version = "v1beta1", kind = "RequestAuthentication", namespaced)]
pub struct RequestAuthenticationSpec {
    /// Define the list of JWTs that can be validated at the selected workloads' proxy. A valid token will be used to extract the authenticated identity. Each rule will be activated only when a token is presented at the location recognized by the rule. The token will be validated based on the JWT rule config. If validation fails, the request will be rejected. Note: if more than one token is presented (at different locations), the output principal is nondeterministic.
    #[serde(rename = "jwtRules", skip_serializing_if = "Option::is_none")]
    pub jwt_rules: Option<Vec<super::JwtRule>>,
    #[serde(rename = "selector", skip_serializing_if = "Option::is_none")]
    pub selector: Option<Box<crate::r#type::v1beta1::selector::workload_selector::WorkloadSelector>>,
}

impl RequestAuthenticationSpec {
    /// RequestAuthenticationSpec defines what request authentication methods are supported by a workload. It will reject a request if the request contains invalid authentication information, based on the configured authentication rules. A request that does not contain any authentication credentials will be accepted but will not have any authenticated identity. To restrict access to authenticated requests only, this should be accompanied by an authorization rule. Examples: - Require JWT for all request for workloads that have label `app:httpbin`
    pub fn new() -> RequestAuthenticationSpec {
        RequestAuthenticationSpec {
            jwt_rules: None,
            selector: None,
        }
    }
}