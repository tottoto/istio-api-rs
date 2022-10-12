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

/// KubernetesResourcesSpec : KubernetesResourcesConfig is a common set of k8s resource configs for components.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct KubernetesResourcesSpec {
    #[serde(rename = "affinity", skip_serializing_if = "Option::is_none")]
    pub affinity: Option<Box<super::Affinity>>,
    /// Deployment environment variables. [https://kubernetes.io/docs/tasks/inject-data-application/define-environment-variable-container/](https://kubernetes.io/docs/tasks/inject-data-application/define-environment-variable-container/)
    #[serde(rename = "env", skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<super::EnvVar>>,
    #[serde(rename = "hpaSpec", skip_serializing_if = "Option::is_none")]
    pub hpa_spec: Option<Box<super::HorizontalPodAutoscalerSpec>>,
    /// k8s imagePullPolicy. [https://kubernetes.io/docs/concepts/containers/images/](https://kubernetes.io/docs/concepts/containers/images/)
    #[serde(rename = "imagePullPolicy", skip_serializing_if = "Option::is_none")]
    pub image_pull_policy: Option<String>,
    /// k8s nodeSelector. [https://kubernetes.io/docs/concepts/configuration/assign-pod-node/#nodeselector](https://kubernetes.io/docs/concepts/configuration/assign-pod-node/#nodeselector)
    #[serde(rename = "nodeSelector", skip_serializing_if = "Option::is_none")]
    pub node_selector: Option<::std::collections::HashMap<String, String>>,
    /// Overlays for k8s resources in rendered manifests.
    #[serde(rename = "overlays", skip_serializing_if = "Option::is_none")]
    pub overlays: Option<Vec<super::K8sObjectOverlay>>,
    /// k8s pod annotations. [https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations/](https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations/)
    #[serde(rename = "podAnnotations", skip_serializing_if = "Option::is_none")]
    pub pod_annotations: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "podDisruptionBudget", skip_serializing_if = "Option::is_none")]
    pub pod_disruption_budget: Option<Box<super::PodDisruptionBudgetSpec>>,
    /// k8s priority_class_name. Default for all resources unless overridden. [https://kubernetes.io/docs/concepts/configuration/pod-priority-preemption/#priorityclass](https://kubernetes.io/docs/concepts/configuration/pod-priority-preemption/#priorityclass)
    #[serde(rename = "priorityClassName", skip_serializing_if = "Option::is_none")]
    pub priority_class_name: Option<String>,
    #[serde(rename = "readinessProbe", skip_serializing_if = "Option::is_none")]
    pub readiness_probe: Option<Box<super::ReadinessProbe>>,
    /// k8s Deployment replicas setting. [https://kubernetes.io/docs/concepts/workloads/controllers/deployment/](https://kubernetes.io/docs/concepts/workloads/controllers/deployment/)
    #[serde(rename = "replicaCount", skip_serializing_if = "Option::is_none")]
    pub replica_count: Option<i32>,
    #[serde(rename = "resources", skip_serializing_if = "Option::is_none")]
    pub resources: Option<Box<super::Resources>>,
    #[serde(rename = "securityContext", skip_serializing_if = "Option::is_none")]
    pub security_context: Option<Box<super::PodSecurityContext>>,
    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: Option<Box<super::ServiceSpec>>,
    /// k8s service annotations. [https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations/](https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations/)
    #[serde(rename = "serviceAnnotations", skip_serializing_if = "Option::is_none")]
    pub service_annotations: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "strategy", skip_serializing_if = "Option::is_none")]
    pub strategy: Option<Box<super::DeploymentStrategy>>,
    /// k8s toleration [https://kubernetes.io/docs/concepts/configuration/taint-and-toleration/](https://kubernetes.io/docs/concepts/configuration/taint-and-toleration/)
    #[serde(rename = "tolerations", skip_serializing_if = "Option::is_none")]
    pub tolerations: Option<Vec<super::Toleration>>,
    /// k8s volumeMounts VolumeMounts defines the collection of VolumeMount to inject into containers.
    #[serde(rename = "volumeMounts", skip_serializing_if = "Option::is_none")]
    pub volume_mounts: Option<Vec<super::K8sIoApiCoreV1VolumeMount>>,
    /// k8s volume [https://kubernetes.io/docs/concepts/storage/volumes/](https://kubernetes.io/docs/concepts/storage/volumes/) Volumes defines the collection of Volume to inject into the pod.
    #[serde(rename = "volumes", skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<super::K8sIoApiCoreV1Volume>>,
}

impl KubernetesResourcesSpec {
    /// KubernetesResourcesConfig is a common set of k8s resource configs for components.
    pub fn new() -> KubernetesResourcesSpec {
        KubernetesResourcesSpec {
            affinity: None,
            env: None,
            hpa_spec: None,
            image_pull_policy: None,
            node_selector: None,
            overlays: None,
            pod_annotations: None,
            pod_disruption_budget: None,
            priority_class_name: None,
            readiness_probe: None,
            replica_count: None,
            resources: None,
            security_context: None,
            service: None,
            service_annotations: None,
            strategy: None,
            tolerations: None,
            volume_mounts: None,
            volumes: None,
        }
    }
}