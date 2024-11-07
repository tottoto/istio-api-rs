// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -Af resources/istio/v1_23_0/networking/v1alpha3/Gateway.yaml --api-version v1alpha3
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use schemars::JsonSchema;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// Configuration affecting edge load balancer. See more details at: https://istio.io/docs/reference/config/networking/gateway.html
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "networking.istio.io", version = "v1alpha3", kind = "Gateway", plural = "gateways")]
#[kube(namespaced)]
pub struct GatewaySpec {
    /// One or more labels that indicate a specific set of pods/VMs on which this gateway configuration should be applied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<BTreeMap<String, String>>,
    /// A list of server specifications.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<GatewayServers>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GatewayServers {
    /// The ip or the Unix domain socket to which the listener should be bound to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultEndpoint")]
    pub default_endpoint: Option<String>,
    /// One or more hosts exposed by this gateway.
    pub hosts: Vec<String>,
    /// An optional name of the server, when set must be unique across all servers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The Port on which the proxy should listen for incoming connections.
    pub port: GatewayServersPort,
    /// Set of TLS related options that govern the server's behavior.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<GatewayServersTls>,
}

/// The Port on which the proxy should listen for incoming connections.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GatewayServersPort {
    /// Label assigned to the port.
    pub name: String,
    /// A valid non-negative integer port number.
    pub number: i64,
    /// The protocol exposed on the port.
    pub protocol: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetPort")]
    pub target_port: Option<i64>,
}

/// Set of TLS related options that govern the server's behavior.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GatewayServersTls {
    /// REQUIRED if mode is `MUTUAL` or `OPTIONAL_MUTUAL`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "caCertificates")]
    pub ca_certificates: Option<String>,
    /// OPTIONAL: The path to the file containing the certificate revocation list (CRL) to use in verifying a presented client side certificate.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "caCrl")]
    pub ca_crl: Option<String>,
    /// Optional: If specified, only support the specified cipher list.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cipherSuites")]
    pub cipher_suites: Option<Vec<String>>,
    /// For gateways running on Kubernetes, the name of the secret that holds the TLS certs including the CA certificates.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "credentialName")]
    pub credential_name: Option<String>,
    /// If set to true, the load balancer will send a 301 redirect for all http connections, asking the clients to use HTTPS.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpsRedirect")]
    pub https_redirect: Option<bool>,
    /// Optional: Maximum TLS protocol version.
    /// 
    /// Valid Options: TLS_AUTO, TLSV1_0, TLSV1_1, TLSV1_2, TLSV1_3
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxProtocolVersion")]
    pub max_protocol_version: Option<GatewayServersTlsMaxProtocolVersion>,
    /// Optional: Minimum TLS protocol version.
    /// 
    /// Valid Options: TLS_AUTO, TLSV1_0, TLSV1_1, TLSV1_2, TLSV1_3
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minProtocolVersion")]
    pub min_protocol_version: Option<GatewayServersTlsMinProtocolVersion>,
    /// Optional: Indicates whether connections to this port should be secured using TLS.
    /// 
    /// Valid Options: PASSTHROUGH, SIMPLE, MUTUAL, AUTO_PASSTHROUGH, ISTIO_MUTUAL, OPTIONAL_MUTUAL
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<GatewayServersTlsMode>,
    /// REQUIRED if mode is `SIMPLE` or `MUTUAL`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "privateKey")]
    pub private_key: Option<String>,
    /// REQUIRED if mode is `SIMPLE` or `MUTUAL`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serverCertificate")]
    pub server_certificate: Option<String>,
    /// A list of alternate names to verify the subject identity in the certificate presented by the client.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subjectAltNames")]
    pub subject_alt_names: Option<Vec<String>>,
    /// An optional list of hex-encoded SHA-256 hashes of the authorized client certificates.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "verifyCertificateHash")]
    pub verify_certificate_hash: Option<Vec<String>>,
    /// An optional list of base64-encoded SHA-256 hashes of the SPKIs of authorized client certificates.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "verifyCertificateSpki")]
    pub verify_certificate_spki: Option<Vec<String>>,
}

/// Set of TLS related options that govern the server's behavior.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum GatewayServersTlsMaxProtocolVersion {
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

/// Set of TLS related options that govern the server's behavior.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum GatewayServersTlsMinProtocolVersion {
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

/// Set of TLS related options that govern the server's behavior.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum GatewayServersTlsMode {
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
    #[serde(rename = "OPTIONAL_MUTUAL")]
    OptionalMutual,
}

