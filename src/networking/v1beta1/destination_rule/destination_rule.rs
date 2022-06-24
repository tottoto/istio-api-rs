use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use kube::CustomResource;
/*
 * Configuration affecting load balancing, outlier detection, etc.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1beta1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DestinationRuleSpec : DestinationRule defines policies that apply to traffic intended for a service after routing has occurred.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema, CustomResource)]
#[kube(group = "networking.istio.io", version = "v1beta1", kind = "DestinationRule", namespaced)]
pub struct DestinationRuleSpec {
    /// The name of a service from the service registry. Service names are looked up from the platform's service registry (e.g., Kubernetes services, Consul services, etc.) and from the hosts declared by [ServiceEntries](https://istio.io/docs/reference/config/networking/service-entry/#ServiceEntry). Rules defined for services that do not exist in the service registry will be ignored.
    #[serde(rename = "host", skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(rename = "trafficPolicy", skip_serializing_if = "Option::is_none")]
    pub traffic_policy: Option<Box<super::TrafficPolicy>>,
    /// One or more named sets that represent individual versions of a service. Traffic policies can be overridden at subset level.
    #[serde(rename = "subsets", skip_serializing_if = "Option::is_none")]
    pub subsets: Option<Vec<super::Subset>>,
    /// A list of namespaces to which this destination rule is exported. The resolution of a destination rule to apply to a service occurs in the context of a hierarchy of namespaces. Exporting a destination rule allows it to be included in the resolution hierarchy for services in other namespaces. This feature provides a mechanism for service owners and mesh administrators to control the visibility of destination rules across namespace boundaries.
    #[serde(rename = "exportTo", skip_serializing_if = "Option::is_none")]
    pub export_to: Option<Vec<String>>,
    #[serde(rename = "workloadSelector", skip_serializing_if = "Option::is_none")]
    pub workload_selector: Option<Box<crate::r#type::v1beta1::workload_selector::WorkloadSelector>>,
}

impl DestinationRuleSpec {
    /// DestinationRule defines policies that apply to traffic intended for a service after routing has occurred.
    pub fn new() -> DestinationRuleSpec {
        DestinationRuleSpec {
            host: None,
            traffic_policy: None,
            subsets: None,
            export_to: None,
            workload_selector: None,
        }
    }
}