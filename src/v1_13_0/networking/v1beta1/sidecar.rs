// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -Af resources/istio/v1_13_0/networking/v1beta1/Sidecar.yaml --api-version v1beta1 -D Default
// kopium version: 0.15.0

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// Configuration affecting network reachability of a sidecar. See more details at: https://istio.io/docs/reference/config/networking/sidecar.html
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
#[kube(group = "networking.istio.io", version = "v1beta1", kind = "Sidecar", plural = "sidecars")]
#[kube(namespaced)]
pub struct SidecarSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub egress: Option<Vec<SidecarEgress>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ingress: Option<Vec<SidecarIngress>>,
    /// Configuration for the outbound traffic policy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "outboundTrafficPolicy")]
    pub outbound_traffic_policy: Option<SidecarOutboundTrafficPolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "workloadSelector")]
    pub workload_selector: Option<SidecarWorkloadSelector>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
pub struct SidecarEgress {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "captureMode")]
    pub capture_mode: Option<SidecarEgressCaptureMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hosts: Option<Vec<String>>,
    /// The port associated with the listener.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<SidecarEgressPort>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum SidecarEgressCaptureMode {
    #[serde(rename = "DEFAULT")]
    Default,
    #[serde(rename = "IPTABLES")]
    Iptables,
    #[serde(rename = "NONE")]
    None,
}

/// The port associated with the listener.
#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
pub struct SidecarEgressPort {
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

#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
pub struct SidecarIngress {
    /// The IP to which the listener should be bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "captureMode")]
    pub capture_mode: Option<SidecarIngressCaptureMode>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultEndpoint")]
    pub default_endpoint: Option<String>,
    /// The port associated with the listener.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<SidecarIngressPort>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<SidecarIngressTls>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum SidecarIngressCaptureMode {
    #[serde(rename = "DEFAULT")]
    Default,
    #[serde(rename = "IPTABLES")]
    Iptables,
    #[serde(rename = "NONE")]
    None,
}

/// The port associated with the listener.
#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
pub struct SidecarIngressPort {
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

#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
pub struct SidecarIngressTls {
    /// REQUIRED if mode is `MUTUAL`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "caCertificates")]
    pub ca_certificates: Option<String>,
    /// Optional: If specified, only support the specified cipher list.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cipherSuites")]
    pub cipher_suites: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "credentialName")]
    pub credential_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpsRedirect")]
    pub https_redirect: Option<bool>,
    /// Optional: Maximum TLS protocol version.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxProtocolVersion")]
    pub max_protocol_version: Option<SidecarIngressTlsMaxProtocolVersion>,
    /// Optional: Minimum TLS protocol version.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minProtocolVersion")]
    pub min_protocol_version: Option<SidecarIngressTlsMinProtocolVersion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<SidecarIngressTlsMode>,
    /// REQUIRED if mode is `SIMPLE` or `MUTUAL`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "privateKey")]
    pub private_key: Option<String>,
    /// REQUIRED if mode is `SIMPLE` or `MUTUAL`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serverCertificate")]
    pub server_certificate: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subjectAltNames")]
    pub subject_alt_names: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "verifyCertificateHash")]
    pub verify_certificate_hash: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "verifyCertificateSpki")]
    pub verify_certificate_spki: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum SidecarIngressTlsMaxProtocolVersion {
    #[serde(rename = "TLS_AUTO")]
    TlsAuto,
    #[serde(rename = "TLSV1_0")]
    Tlsv10,
    #[serde(rename = "TLSV1_1")]
    Tlsv11,
    #[serde(rename = "TLSV1_2")]
    Tlsv12,
    #[serde(rename = "TLSV1_3")]
    Tlsv13,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum SidecarIngressTlsMinProtocolVersion {
    #[serde(rename = "TLS_AUTO")]
    TlsAuto,
    #[serde(rename = "TLSV1_0")]
    Tlsv10,
    #[serde(rename = "TLSV1_1")]
    Tlsv11,
    #[serde(rename = "TLSV1_2")]
    Tlsv12,
    #[serde(rename = "TLSV1_3")]
    Tlsv13,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum SidecarIngressTlsMode {
    #[serde(rename = "PASSTHROUGH")]
    Passthrough,
    #[serde(rename = "SIMPLE")]
    Simple,
    #[serde(rename = "MUTUAL")]
    Mutual,
    #[serde(rename = "AUTO_PASSTHROUGH")]
    AutoPassthrough,
    #[serde(rename = "ISTIO_MUTUAL")]
    IstioMutual,
}

/// Configuration for the outbound traffic policy.
#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
pub struct SidecarOutboundTrafficPolicy {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "egressProxy")]
    pub egress_proxy: Option<SidecarOutboundTrafficPolicyEgressProxy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<SidecarOutboundTrafficPolicyMode>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
pub struct SidecarOutboundTrafficPolicyEgressProxy {
    /// The name of a service from the service registry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Specifies the port on the host that is being addressed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<SidecarOutboundTrafficPolicyEgressProxyPort>,
    /// The name of a subset within the service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subset: Option<String>,
}

/// Specifies the port on the host that is being addressed.
#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
pub struct SidecarOutboundTrafficPolicyEgressProxyPort {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
}

/// Configuration for the outbound traffic policy.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum SidecarOutboundTrafficPolicyMode {
    #[serde(rename = "REGISTRY_ONLY")]
    RegistryOnly,
    #[serde(rename = "ALLOW_ANY")]
    AllowAny,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
pub struct SidecarWorkloadSelector {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

