use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
/*
 * Configuration affecting label/content routing, sni routing, etc.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1beta1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TlsMatchAttributes : TLS connection match attributes.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct TlsMatchAttributes {
    /// Specifies the port on the host that is being addressed. Many services only expose a single port or label ports with the protocols they support, in these cases it is not required to explicitly select the port.
    #[serde(rename = "port", skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// Names of gateways where the rule should be applied. Gateway names in the top-level `gateways` field of the VirtualService (if any) are overridden. The gateway match is independent of sourceLabels.
    #[serde(rename = "gateways", skip_serializing_if = "Option::is_none")]
    pub gateways: Option<Vec<String>>,
    /// One or more labels that constrain the applicability of a rule to workloads with the given labels. If the VirtualService has a list of gateways specified in the top-level `gateways` field, it should include the reserved gateway `mesh` in order for this field to be applicable.
    #[serde(rename = "sourceLabels", skip_serializing_if = "Option::is_none")]
    pub source_labels: Option<::std::collections::HashMap<String, String>>,
    /// Source namespace constraining the applicability of a rule to workloads in that namespace. If the VirtualService has a list of gateways specified in the top-level `gateways` field, it must include the reserved gateway `mesh` for this field to be applicable.
    #[serde(rename = "sourceNamespace", skip_serializing_if = "Option::is_none")]
    pub source_namespace: Option<String>,
    /// IPv4 or IPv6 ip addresses of destination with optional subnet. E.g., a.b.c.d/xx form or just a.b.c.d.
    #[serde(rename = "destinationSubnets", skip_serializing_if = "Option::is_none")]
    pub destination_subnets: Option<Vec<String>>,
    /// SNI (server name indicator) to match on. Wildcard prefixes can be used in the SNI value, e.g., *.com will match foo.example.com as well as example.com. An SNI value must be a subset (i.e., fall within the domain) of the corresponding virtual serivce's hosts.
    #[serde(rename = "sniHosts", skip_serializing_if = "Option::is_none")]
    pub sni_hosts: Option<Vec<String>>,
}

impl TlsMatchAttributes {
    /// TLS connection match attributes.
    pub fn new() -> TlsMatchAttributes {
        TlsMatchAttributes {
            port: None,
            gateways: None,
            source_labels: None,
            source_namespace: None,
            destination_subnets: None,
            sni_hosts: None,
        }
    }
}