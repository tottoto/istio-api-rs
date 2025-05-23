// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f resources/istio/v1_24_0/networking/v1beta1/Gateway.yaml --api-version v1beta1
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "networking.istio.io", version = "v1beta1", kind = "Gateway", plural = "gateways")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct GatewaySpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<GatewayServers>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GatewayServers {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultEndpoint")]
    pub default_endpoint: Option<String>,
    pub hosts: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub port: GatewayServersPort,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<GatewayServersTls>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GatewayServersPort {
    pub name: String,
    pub number: i64,
    pub protocol: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetPort")]
    pub target_port: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GatewayServersTls {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "caCertificates")]
    pub ca_certificates: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "caCrl")]
    pub ca_crl: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cipherSuites")]
    pub cipher_suites: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "credentialName")]
    pub credential_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpsRedirect")]
    pub https_redirect: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxProtocolVersion")]
    pub max_protocol_version: Option<GatewayServersTlsMaxProtocolVersion>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minProtocolVersion")]
    pub min_protocol_version: Option<GatewayServersTlsMinProtocolVersion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<GatewayServersTlsMode>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "privateKey")]
    pub private_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serverCertificate")]
    pub server_certificate: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subjectAltNames")]
    pub subject_alt_names: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "verifyCertificateHash")]
    pub verify_certificate_hash: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "verifyCertificateSpki")]
    pub verify_certificate_spki: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GatewayStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<IntOrString>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "validationMessages")]
    pub validation_messages: Option<Vec<GatewayStatusValidationMessages>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GatewayStatusValidationMessages {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "documentationUrl")]
    pub documentation_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<GatewayStatusValidationMessagesLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<GatewayStatusValidationMessagesType>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum GatewayStatusValidationMessagesLevel {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "ERROR")]
    Error,
    #[serde(rename = "WARNING")]
    Warning,
    #[serde(rename = "INFO")]
    Info,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GatewayStatusValidationMessagesType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

