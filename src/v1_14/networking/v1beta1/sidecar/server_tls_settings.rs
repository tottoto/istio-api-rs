use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
/*
 * Configuration affecting network reachability of a sidecar.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1beta1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct ServerTlsSettings {
    /// REQUIRED if mode is `MUTUAL`. The path to a file containing certificate authority certificates to use in verifying a presented client side certificate.
    #[serde(rename = "caCertificates", skip_serializing_if = "Option::is_none")]
    pub ca_certificates: Option<String>,
    /// Optional: If specified, only support the specified cipher list. Otherwise default to the default cipher list supported by Envoy.
    #[serde(rename = "cipherSuites", skip_serializing_if = "Option::is_none")]
    pub cipher_suites: Option<Vec<String>>,
    /// For gateways running on Kubernetes, the name of the secret that holds the TLS certs including the CA certificates. Applicable only on Kubernetes. The secret (of type `generic`) should contain the following keys and values: `key: <privateKey>` and `cert: <serverCert>`. For mutual TLS, `cacert: <CACertificate>` can be provided in the same secret or a separate secret named `<secret>-cacert`. Secret of type tls for server certificates along with ca.crt key for CA certificates is also supported. Only one of server certificates and CA certificate or credentialName can be specified.
    #[serde(rename = "credentialName", skip_serializing_if = "Option::is_none")]
    pub credential_name: Option<String>,
    /// If set to true, the load balancer will send a 301 redirect for all http connections, asking the clients to use HTTPS.
    #[serde(rename = "httpsRedirect", skip_serializing_if = "Option::is_none")]
    pub https_redirect: Option<bool>,
    #[serde(rename = "maxProtocolVersion", skip_serializing_if = "Option::is_none")]
    pub max_protocol_version: Option<super::ServerTlsSettingsTlsProtocol>,
    #[serde(rename = "minProtocolVersion", skip_serializing_if = "Option::is_none")]
    pub min_protocol_version: Option<super::ServerTlsSettingsTlsProtocol>,
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<super::ServerTlsSettingsTlSmode>,
    /// REQUIRED if mode is `SIMPLE` or `MUTUAL`. The path to the file holding the server's private key.
    #[serde(rename = "privateKey", skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    /// REQUIRED if mode is `SIMPLE` or `MUTUAL`. The path to the file holding the server-side TLS certificate to use.
    #[serde(rename = "serverCertificate", skip_serializing_if = "Option::is_none")]
    pub server_certificate: Option<String>,
    /// A list of alternate names to verify the subject identity in the certificate presented by the client.
    #[serde(rename = "subjectAltNames", skip_serializing_if = "Option::is_none")]
    pub subject_alt_names: Option<Vec<String>>,
    /// An optional list of hex-encoded SHA-256 hashes of the authorized client certificates. Both simple and colon separated formats are acceptable. Note: When both verify_certificate_hash and verify_certificate_spki are specified, a hash matching either value will result in the certificate being accepted.
    #[serde(rename = "verifyCertificateHash", skip_serializing_if = "Option::is_none")]
    pub verify_certificate_hash: Option<Vec<String>>,
    /// An optional list of base64-encoded SHA-256 hashes of the SKPIs of authorized client certificates. Note: When both verify_certificate_hash and verify_certificate_spki are specified, a hash matching either value will result in the certificate being accepted.
    #[serde(rename = "verifyCertificateSpki", skip_serializing_if = "Option::is_none")]
    pub verify_certificate_spki: Option<Vec<String>>,
}

impl ServerTlsSettings {
    pub fn new() -> ServerTlsSettings {
        ServerTlsSettings {
            ca_certificates: None,
            cipher_suites: None,
            credential_name: None,
            https_redirect: None,
            max_protocol_version: None,
            min_protocol_version: None,
            mode: None,
            private_key: None,
            server_certificate: None,
            subject_alt_names: None,
            verify_certificate_hash: None,
            verify_certificate_spki: None,
        }
    }
}