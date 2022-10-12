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

/// HorizontalPodAutoscalerSpec : See k8s.io.api.autoscaling.v2beta1.HorizontalPodAutoscalerSpec.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct HorizontalPodAutoscalerSpec {
    #[serde(rename = "maxReplicas", skip_serializing_if = "Option::is_none")]
    pub max_replicas: Option<i32>,
    #[serde(rename = "metrics", skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<super::MetricSpec>>,
    #[serde(rename = "minReplicas", skip_serializing_if = "Option::is_none")]
    pub min_replicas: Option<i32>,
    #[serde(rename = "scaleTargetRef", skip_serializing_if = "Option::is_none")]
    pub scale_target_ref: Option<Box<super::CrossVersionObjectReference>>,
}

impl HorizontalPodAutoscalerSpec {
    /// See k8s.io.api.autoscaling.v2beta1.HorizontalPodAutoscalerSpec.
    pub fn new() -> HorizontalPodAutoscalerSpec {
        HorizontalPodAutoscalerSpec {
            max_replicas: None,
            metrics: None,
            min_replicas: None,
            scale_target_ref: None,
        }
    }
}