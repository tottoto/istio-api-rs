use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
/*
 * Configuration affecting load balancing, outlier detection, etc.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1alpha3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ClientTlsSettings : SSL/TLS related settings for upstream connections. See Envoy's [TLS context](https://www.envoyproxy.io/docs/envoy/latest/api-v3/extensions/transport_sockets/tls/v3/common.proto.html#common-tls-configuration) for more details. These settings are common to both HTTP and TCP upstreams.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct ClientTlsSettings {
    /// OPTIONAL: The path to the file containing certificate authority certificates to use in verifying a presented server certificate. If omitted, the proxy will not verify the server's certificate. Should be empty if mode is `ISTIO_MUTUAL`.
    #[serde(rename = "caCertificates", skip_serializing_if = "Option::is_none")]
    pub ca_certificates: Option<String>,
    /// REQUIRED if mode is `MUTUAL`. The path to the file holding the client-side TLS certificate to use. Should be empty if mode is `ISTIO_MUTUAL`.
    #[serde(rename = "clientCertificate", skip_serializing_if = "Option::is_none")]
    pub client_certificate: Option<String>,
    /// The name of the secret that holds the TLS certs for the client including the CA certificates. Secret must exist in the same namespace with the proxy using the certificates. The secret (of type `generic`)should contain the following keys and values: `key: <privateKey>`, `cert: <clientCert>`, `cacert: <CACertificate>`. Here CACertificate is used to verify the server certificate. Secret of type tls for client certificates along with ca.crt key for CA certificates is also supported. Only one of client certificates and CA certificate or credentialName can be specified.
    #[serde(rename = "credentialName", skip_serializing_if = "Option::is_none")]
    pub credential_name: Option<String>,
    /// InsecureSkipVerify specifies whether the proxy should skip verifying the CA signature and SAN for the server certificate corresponding to the host. This flag should only be set if global CA signature verifcation is enabled, `VerifyCertAtClient` environmental variable is set to `true`, but no verification is desired for a specific host. If enabled with or without `VerifyCertAtClient` enabled, verification of the CA signature and SAN will be skipped.
    #[serde(rename = "insecureSkipVerify", skip_serializing_if = "Option::is_none")]
    pub insecure_skip_verify: Option<bool>,
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<super::ClientTlsSettingsTlSmode>,
    /// REQUIRED if mode is `MUTUAL`. The path to the file holding the client's private key. Should be empty if mode is `ISTIO_MUTUAL`.
    #[serde(rename = "privateKey", skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    /// SNI string to present to the server during TLS handshake. If unspecified, SNI will be automatically set based on downstream HTTP host/authority header for SIMPLE and MUTUAL TLS modes, provided `ENABLE_AUTO_SNI` environmental variable is set to `true`.
    #[serde(rename = "sni", skip_serializing_if = "Option::is_none")]
    pub sni: Option<String>,
    /// A list of alternate names to verify the subject identity in the certificate. If specified, the proxy will verify that the server certificate's subject alt name matches one of the specified values. If specified, this list overrides the value of subject_alt_names from the ServiceEntry. If unspecified, automatic validation of upstream presented certificate for new upstream connections will be done based on the downstream HTTP host/authority header, provided `VERIFY_CERT_AT_CLIENT` and `ENABLE_AUTO_SNI` environmental variables are set to `true`.
    #[serde(rename = "subjectAltNames", skip_serializing_if = "Option::is_none")]
    pub subject_alt_names: Option<Vec<String>>,
}

impl ClientTlsSettings {
    /// SSL/TLS related settings for upstream connections. See Envoy's [TLS context](https://www.envoyproxy.io/docs/envoy/latest/api-v3/extensions/transport_sockets/tls/v3/common.proto.html#common-tls-configuration) for more details. These settings are common to both HTTP and TCP upstreams.
    pub fn new() -> ClientTlsSettings {
        ClientTlsSettings {
            ca_certificates: None,
            client_certificate: None,
            credential_name: None,
            insecure_skip_verify: None,
            mode: None,
            private_key: None,
            sni: None,
            subject_alt_names: None,
        }
    }
}