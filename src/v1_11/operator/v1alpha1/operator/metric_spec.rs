use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
/*
 * Configuration affecting Istio control plane installation version and shape.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1alpha1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// MetricSpec : See k8s.io.autoscaling.v2beta1.MetricSpec.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct MetricSpec {
    #[serde(rename = "external", skip_serializing_if = "Option::is_none")]
    pub external: Option<Box<super::ExternalMetricSource>>,
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<Box<super::ObjectMetricSource>>,
    #[serde(rename = "pods", skip_serializing_if = "Option::is_none")]
    pub pods: Option<Box<super::PodsMetricSource>>,
    #[serde(rename = "resource", skip_serializing_if = "Option::is_none")]
    pub resource: Option<Box<super::ResourceMetricSource>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl MetricSpec {
    /// See k8s.io.autoscaling.v2beta1.MetricSpec.
    pub fn new() -> MetricSpec {
        MetricSpec {
            external: None,
            object: None,
            pods: None,
            resource: None,
            _type: None,
        }
    }
}