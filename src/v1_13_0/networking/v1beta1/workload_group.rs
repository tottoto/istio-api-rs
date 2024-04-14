// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -Af resources/istio/v1_13_0/networking/v1beta1/WorkloadGroup.yaml --api-version v1beta1 -D Default
// kopium version: 0.17.2

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
#[kube(group = "networking.istio.io", version = "v1beta1", kind = "WorkloadGroup", plural = "workloadgroups")]
#[kube(namespaced)]
pub struct WorkloadGroupSpec {
    /// Metadata that will be used for all corresponding `WorkloadEntries`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<WorkloadGroupMetadata>,
    /// `ReadinessProbe` describes the configuration the user must provide for healthchecking on their workload.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probe: Option<WorkloadGroupProbe>,
    /// Template to be used for the generation of `WorkloadEntry` resources that belong to this `WorkloadGroup`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<WorkloadGroupTemplate>,
}

/// Metadata that will be used for all corresponding `WorkloadEntries`.
#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
pub struct WorkloadGroupMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

/// `ReadinessProbe` describes the configuration the user must provide for healthchecking on their workload.
#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
pub struct WorkloadGroupProbe {
    /// Health is determined by how the command that is executed exited.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exec: Option<WorkloadGroupProbeExec>,
    /// Minimum consecutive failures for the probe to be considered failed after having succeeded.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureThreshold")]
    pub failure_threshold: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpGet")]
    pub http_get: Option<WorkloadGroupProbeHttpGet>,
    /// Number of seconds after the container has started before readiness probes are initiated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "initialDelaySeconds")]
    pub initial_delay_seconds: Option<i32>,
    /// How often (in seconds) to perform the probe.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "periodSeconds")]
    pub period_seconds: Option<i32>,
    /// Minimum consecutive successes for the probe to be considered successful after having failed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "successThreshold")]
    pub success_threshold: Option<i32>,
    /// Health is determined by if the proxy is able to connect.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tcpSocket")]
    pub tcp_socket: Option<WorkloadGroupProbeTcpSocket>,
    /// Number of seconds after which the probe times out.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeoutSeconds")]
    pub timeout_seconds: Option<i32>,
}

/// Health is determined by how the command that is executed exited.
#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
pub struct WorkloadGroupProbeExec {
    /// Command to run.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
pub struct WorkloadGroupProbeHttpGet {
    /// Host name to connect to, defaults to the pod IP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Headers the proxy will pass on to make the request.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpHeaders")]
    pub http_headers: Option<Vec<WorkloadGroupProbeHttpGetHttpHeaders>>,
    /// Path to access on the HTTP server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Port on which the endpoint lives.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
pub struct WorkloadGroupProbeHttpGetHttpHeaders {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Health is determined by if the proxy is able to connect.
#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
pub struct WorkloadGroupProbeTcpSocket {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
}

/// Template to be used for the generation of `WorkloadEntry` resources that belong to this `WorkloadGroup`.
#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
pub struct WorkloadGroupTemplate {
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

