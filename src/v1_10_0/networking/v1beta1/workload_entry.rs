// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -Af resources/istio/v1_10_0/networking/v1beta1/WorkloadEntry.yaml --api-version v1beta1
// kopium version: 0.20.0

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use schemars::JsonSchema;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// Configuration affecting VMs onboarded into the mesh. See more details at: https://istio.io/docs/reference/config/networking/workload-entry.html
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "networking.istio.io", version = "v1beta1", kind = "WorkloadEntry", plural = "workloadentries")]
#[kube(namespaced)]
pub struct WorkloadEntrySpec {
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

