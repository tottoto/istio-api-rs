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

/// ObjectMetricSource : See k8s.io.autoscaling.v2beta2.ObjectMetricSource.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct ObjectMetricSource {
    #[serde(rename = "averageValue", skip_serializing_if = "Option::is_none")]
    pub average_value: Option<Box<super::IntOrString>>,
    #[serde(rename = "describedObject", skip_serializing_if = "Option::is_none")]
    pub described_object: Option<Box<super::CrossVersionObjectReference>>,
    #[serde(rename = "metric", skip_serializing_if = "Option::is_none")]
    pub metric: Option<Box<super::MetricIdentifier>>,
    #[serde(rename = "metricName", skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "selector", skip_serializing_if = "Option::is_none")]
    pub selector: Option<Box<super::K8sIoApimachineryPkgApisMetaV1LabelSelector>>,
    /// Type changes from CrossVersionObjectReference to ResourceMetricTarget in autoscaling v2beta2/v2 compared with v2beta1 Change it to dynamic type to keep backward compatible
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: Option<serde_json::Value>,
    #[serde(rename = "targetValue", skip_serializing_if = "Option::is_none")]
    pub target_value: Option<Box<super::IntOrString>>,
}

impl ObjectMetricSource {
    /// See k8s.io.autoscaling.v2beta2.ObjectMetricSource.
    pub fn new() -> ObjectMetricSource {
        ObjectMetricSource {
            average_value: None,
            described_object: None,
            metric: None,
            metric_name: None,
            selector: None,
            target: None,
            target_value: None,
        }
    }
}