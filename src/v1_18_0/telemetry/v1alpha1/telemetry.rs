// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -Af resources/istio/v1_18_0/telemetry/v1alpha1/Telemetry.yaml --api-version v1alpha1
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use schemars::JsonSchema;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// Telemetry configuration for workloads. See more details at: https://istio.io/docs/reference/config/telemetry.html
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "telemetry.istio.io", version = "v1alpha1", kind = "Telemetry", plural = "telemetries")]
#[kube(namespaced)]
pub struct TelemetrySpec {
    /// Optional.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessLogging")]
    pub access_logging: Option<Vec<TelemetryAccessLogging>>,
    /// Optional.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<TelemetryMetrics>>,
    /// Optional.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<TelemetrySelector>,
    /// Optional.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracing: Option<Vec<TelemetryTracing>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TelemetryAccessLogging {
    /// Controls logging.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// Optional.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<TelemetryAccessLoggingFilter>,
    /// Allows tailoring of logging behavior to specific conditions.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "match")]
    pub r#match: Option<TelemetryAccessLoggingMatch>,
    /// Optional.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub providers: Option<Vec<TelemetryAccessLoggingProviders>>,
}

/// Optional.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TelemetryAccessLoggingFilter {
    /// CEL expression for selecting when requests/connections should be logged.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
}

/// Allows tailoring of logging behavior to specific conditions.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TelemetryAccessLoggingMatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<TelemetryAccessLoggingMatchMode>,
}

/// Allows tailoring of logging behavior to specific conditions.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum TelemetryAccessLoggingMatchMode {
    #[serde(rename = "CLIENT_AND_SERVER")]
    ClientAndServer,
    #[serde(rename = "CLIENT")]
    Client,
    #[serde(rename = "SERVER")]
    Server,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TelemetryAccessLoggingProviders {
    /// Required.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TelemetryMetrics {
    /// Optional.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overrides: Option<Vec<TelemetryMetricsOverrides>>,
    /// Optional.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub providers: Option<Vec<TelemetryMetricsProviders>>,
    /// Optional.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reportingInterval")]
    pub reporting_interval: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TelemetryMetricsOverrides {
    /// Optional.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// Match allows provides the scope of the override.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "match")]
    pub r#match: Option<TelemetryMetricsOverridesMatch>,
    /// Optional.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tagOverrides")]
    pub tag_overrides: Option<BTreeMap<String, TelemetryMetricsOverridesTagOverrides>>,
}

/// Match allows provides the scope of the override.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TelemetryMetricsOverridesMatch {
    /// Allows free-form specification of a metric.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customMetric")]
    pub custom_metric: Option<String>,
    /// One of the well-known Istio Standard Metrics.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric: Option<TelemetryMetricsOverridesMatchMetric>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<TelemetryMetricsOverridesMatchMode>,
}

/// Match allows provides the scope of the override.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum TelemetryMetricsOverridesMatchMetric {
    #[serde(rename = "ALL_METRICS")]
    AllMetrics,
    #[serde(rename = "REQUEST_COUNT")]
    RequestCount,
    #[serde(rename = "REQUEST_DURATION")]
    RequestDuration,
    #[serde(rename = "REQUEST_SIZE")]
    RequestSize,
    #[serde(rename = "RESPONSE_SIZE")]
    ResponseSize,
    #[serde(rename = "TCP_OPENED_CONNECTIONS")]
    TcpOpenedConnections,
    #[serde(rename = "TCP_CLOSED_CONNECTIONS")]
    TcpClosedConnections,
    #[serde(rename = "TCP_SENT_BYTES")]
    TcpSentBytes,
    #[serde(rename = "TCP_RECEIVED_BYTES")]
    TcpReceivedBytes,
    #[serde(rename = "GRPC_REQUEST_MESSAGES")]
    GrpcRequestMessages,
    #[serde(rename = "GRPC_RESPONSE_MESSAGES")]
    GrpcResponseMessages,
}

/// Match allows provides the scope of the override.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum TelemetryMetricsOverridesMatchMode {
    #[serde(rename = "CLIENT_AND_SERVER")]
    ClientAndServer,
    #[serde(rename = "CLIENT")]
    Client,
    #[serde(rename = "SERVER")]
    Server,
}

/// Optional.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TelemetryMetricsOverridesTagOverrides {
    /// Operation controls whether or not to update/add a tag, or to remove it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<TelemetryMetricsOverridesTagOverridesOperation>,
    /// Value is only considered if the operation is `UPSERT`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Optional.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum TelemetryMetricsOverridesTagOverridesOperation {
    #[serde(rename = "UPSERT")]
    Upsert,
    #[serde(rename = "REMOVE")]
    Remove,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TelemetryMetricsProviders {
    /// Required.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Optional.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TelemetrySelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TelemetryTracing {
    /// Optional.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customTags")]
    pub custom_tags: Option<BTreeMap<String, TelemetryTracingCustomTags>>,
    /// Controls span reporting.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "disableSpanReporting")]
    pub disable_span_reporting: Option<bool>,
    /// Allows tailoring of behavior to specific conditions.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "match")]
    pub r#match: Option<TelemetryTracingMatch>,
    /// Optional.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub providers: Option<Vec<TelemetryTracingProviders>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "randomSamplingPercentage")]
    pub random_sampling_percentage: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useRequestIdForTraceSampling")]
    pub use_request_id_for_trace_sampling: Option<bool>,
}

/// Optional.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TelemetryTracingCustomTags {
    /// Environment adds the value of an environment variable to each span.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<TelemetryTracingCustomTagsEnvironment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header: Option<TelemetryTracingCustomTagsHeader>,
    /// Literal adds the same, hard-coded value to each span.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub literal: Option<TelemetryTracingCustomTagsLiteral>,
}

/// Environment adds the value of an environment variable to each span.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TelemetryTracingCustomTagsEnvironment {
    /// Optional.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultValue")]
    pub default_value: Option<String>,
    /// Name of the environment variable from which to extract the tag value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TelemetryTracingCustomTagsHeader {
    /// Optional.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultValue")]
    pub default_value: Option<String>,
    /// Name of the header from which to extract the tag value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Literal adds the same, hard-coded value to each span.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TelemetryTracingCustomTagsLiteral {
    /// The tag value to use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Allows tailoring of behavior to specific conditions.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TelemetryTracingMatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<TelemetryTracingMatchMode>,
}

/// Allows tailoring of behavior to specific conditions.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum TelemetryTracingMatchMode {
    #[serde(rename = "CLIENT_AND_SERVER")]
    ClientAndServer,
    #[serde(rename = "CLIENT")]
    Client,
    #[serde(rename = "SERVER")]
    Server,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TelemetryTracingProviders {
    /// Required.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

