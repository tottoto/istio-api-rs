use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
/*
 * Customizing Envoy configuration generated by Istio.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1alpha3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EnvoyFilterRouteConfigurationMatch : Conditions specified in RouteConfigurationMatch must be met for the patch to be applied to a route configuration object or a specific virtual host within the route configuration.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct EnvoyFilterRouteConfigurationMatch {
    /// The service port number or gateway server port number for which this route configuration was generated. If omitted, applies to route configurations for all ports.
    #[serde(rename = "portNumber", skip_serializing_if = "Option::is_none")]
    pub port_number: Option<i32>,
    /// Applicable only for GATEWAY context. The gateway server port name for which this route configuration was generated.
    #[serde(rename = "portName", skip_serializing_if = "Option::is_none")]
    pub port_name: Option<String>,
    /// The Istio gateway config's namespace/name for which this route configuration was generated. Applies only if the context is GATEWAY. Should be in the namespace/name format. Use this field in conjunction with the `portNumber` and `portName` to accurately select the Envoy route configuration for a specific HTTPS server within a gateway config object.
    #[serde(rename = "gateway", skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    #[serde(rename = "vhost", skip_serializing_if = "Option::is_none")]
    pub vhost: Option<Box<super::EnvoyFilterRouteConfigurationMatchVirtualHostMatch>>,
    /// Route configuration name to match on. Can be used to match a specific route configuration by name, such as the internally generated `http_proxy` route configuration for all sidecars.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl EnvoyFilterRouteConfigurationMatch {
    /// Conditions specified in RouteConfigurationMatch must be met for the patch to be applied to a route configuration object or a specific virtual host within the route configuration.
    pub fn new() -> EnvoyFilterRouteConfigurationMatch {
        EnvoyFilterRouteConfigurationMatch {
            port_number: None,
            port_name: None,
            gateway: None,
            vhost: None,
            name: None,
        }
    }
}