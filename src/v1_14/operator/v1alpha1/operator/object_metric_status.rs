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

/// ObjectMetricStatus : See k8s.io.autoscaling.v2beta2.ObjectMetricStatus.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct ObjectMetricStatus {
    #[serde(rename = "current", skip_serializing_if = "Option::is_none")]
    pub current: Option<Box<super::MetricValueStatus>>,
    #[serde(rename = "describedObject", skip_serializing_if = "Option::is_none")]
    pub described_object: Option<Box<super::CrossVersionObjectReference>>,
    #[serde(rename = "metric", skip_serializing_if = "Option::is_none")]
    pub metric: Option<Box<super::MetricIdentifier>>,
}

impl ObjectMetricStatus {
    /// See k8s.io.autoscaling.v2beta2.ObjectMetricStatus.
    pub fn new() -> ObjectMetricStatus {
        ObjectMetricStatus {
            current: None,
            described_object: None,
            metric: None,
        }
    }
}