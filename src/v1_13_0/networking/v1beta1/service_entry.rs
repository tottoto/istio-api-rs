// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -Af resources/istio/v1_13_0/networking/v1beta1/ServiceEntry.yaml --api-version v1beta1
// kopium version: 0.15.0

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// Configuration affecting service registry. See more details at: https://istio.io/docs/reference/config/networking/service-entry.html
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "networking.istio.io", version = "v1beta1", kind = "ServiceEntry", plural = "serviceentries")]
#[kube(namespaced)]
pub struct ServiceEntrySpec {
    /// The virtual IP addresses associated with the service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<String>>,
    /// One or more endpoints associated with the service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<ServiceEntryEndpoints>>,
    /// A list of namespaces to which this service is exported.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "exportTo")]
    pub export_to: Option<Vec<String>>,
    /// The hosts associated with the ServiceEntry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hosts: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<ServiceEntryLocation>,
    /// The ports associated with the external service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<ServiceEntryPorts>>,
    /// Service discovery mode for the hosts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<ServiceEntryResolution>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subjectAltNames")]
    pub subject_alt_names: Option<Vec<String>>,
    /// Applicable only for MESH_INTERNAL services.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "workloadSelector")]
    pub workload_selector: Option<ServiceEntryWorkloadSelector>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ServiceEntryEndpoints {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// One or more labels associated with the endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// The locality associated with the endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    /// Set of ports associated with the endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ports: Option<BTreeMap<String, i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceAccount")]
    pub service_account: Option<String>,
    /// The load balancing weight associated with the endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
}

/// Configuration affecting service registry. See more details at: https://istio.io/docs/reference/config/networking/service-entry.html
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum ServiceEntryLocation {
    #[serde(rename = "MESH_EXTERNAL")]
    MeshExternal,
    #[serde(rename = "MESH_INTERNAL")]
    MeshInternal,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ServiceEntryPorts {
    /// Label assigned to the port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A valid non-negative integer port number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
    /// The protocol exposed on the port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetPort")]
    pub target_port: Option<i64>,
}

/// Configuration affecting service registry. See more details at: https://istio.io/docs/reference/config/networking/service-entry.html
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum ServiceEntryResolution {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "STATIC")]
    Static,
    #[serde(rename = "DNS")]
    Dns,
    #[serde(rename = "DNS_ROUND_ROBIN")]
    DnsRoundRobin,
}

/// Applicable only for MESH_INTERNAL services.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ServiceEntryWorkloadSelector {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

