// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -Af resources/istio/v1_15_0/networking/v1beta1/VirtualService.yaml --api-version v1beta1
// kopium version: 0.15.0

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// Configuration affecting label/content routing, sni routing, etc. See more details at: https://istio.io/docs/reference/config/networking/virtual-service.html
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "networking.istio.io", version = "v1beta1", kind = "VirtualService", plural = "virtualservices")]
#[kube(namespaced)]
pub struct VirtualServiceSpec {
    /// A list of namespaces to which this virtual service is exported.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "exportTo")]
    pub export_to: Option<Vec<String>>,
    /// The names of gateways and sidecars that should apply these routes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateways: Option<Vec<String>>,
    /// The destination hosts to which traffic is being sent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hosts: Option<Vec<String>>,
    /// An ordered list of route rules for HTTP traffic.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<Vec<VirtualServiceHttp>>,
    /// An ordered list of route rules for opaque TCP traffic.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcp: Option<Vec<VirtualServiceTcp>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<Vec<VirtualServiceTls>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceHttp {
    /// Cross-Origin Resource Sharing policy (CORS).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "corsPolicy")]
    pub cors_policy: Option<VirtualServiceHttpCorsPolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delegate: Option<VirtualServiceHttpDelegate>,
    /// A HTTP rule can either return a direct_response, redirect or forward (default) traffic.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "directResponse")]
    pub direct_response: Option<VirtualServiceHttpDirectResponse>,
    /// Fault injection policy to apply on HTTP traffic at the client side.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fault: Option<VirtualServiceHttpFault>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<VirtualServiceHttpHeaders>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "match")]
    pub r#match: Option<Vec<VirtualServiceHttpMatch>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mirror: Option<VirtualServiceHttpMirror>,
    /// Percentage of the traffic to be mirrored by the `mirror` field.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mirrorPercent")]
    pub mirror_percent: Option<i64>,
    /// Percentage of the traffic to be mirrored by the `mirror` field.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mirrorPercentage")]
    pub mirror_percentage: Option<VirtualServiceHttpMirrorPercentage>,
    /// Percentage of the traffic to be mirrored by the `mirror` field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mirror_percent_1: Option<i64>,
    /// The name assigned to the route for debugging purposes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A HTTP rule can either return a direct_response, redirect or forward (default) traffic.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redirect: Option<VirtualServiceHttpRedirect>,
    /// Retry policy for HTTP requests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retries: Option<VirtualServiceHttpRetries>,
    /// Rewrite HTTP URIs and Authority headers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rewrite: Option<VirtualServiceHttpRewrite>,
    /// A HTTP rule can either return a direct_response, redirect or forward (default) traffic.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route: Option<Vec<VirtualServiceHttpRoute>>,
    /// Timeout for HTTP requests, default is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

/// Cross-Origin Resource Sharing policy (CORS).
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceHttpCorsPolicy {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowCredentials")]
    pub allow_credentials: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowHeaders")]
    pub allow_headers: Option<Vec<String>>,
    /// List of HTTP methods allowed to access the resource.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowMethods")]
    pub allow_methods: Option<Vec<String>>,
    /// The list of origins that are allowed to perform CORS requests.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowOrigin")]
    pub allow_origin: Option<Vec<String>>,
    /// String patterns that match allowed origins.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowOrigins")]
    pub allow_origins: Option<Vec<VirtualServiceHttpCorsPolicyAllowOrigins>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "exposeHeaders")]
    pub expose_headers: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxAge")]
    pub max_age: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceHttpCorsPolicyAllowOrigins {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// RE2 style regex-based match (https://github.com/google/re2/wiki/Syntax).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceHttpDelegate {
    /// Name specifies the name of the delegate VirtualService.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace specifies the namespace where the delegate VirtualService resides.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// A HTTP rule can either return a direct_response, redirect or forward (default) traffic.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceHttpDirectResponse {
    /// Specifies the content of the response body.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<VirtualServiceHttpDirectResponseBody>,
    /// Specifies the HTTP response status to be returned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

/// Specifies the content of the response body.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceHttpDirectResponseBody {
    /// response body as base64 encoded bytes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub string: Option<String>,
}

/// Fault injection policy to apply on HTTP traffic at the client side.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceHttpFault {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abort: Option<VirtualServiceHttpFaultAbort>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delay: Option<VirtualServiceHttpFaultDelay>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceHttpFaultAbort {
    /// GRPC status code to use to abort the request.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "grpcStatus")]
    pub grpc_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "http2Error")]
    pub http2_error: Option<String>,
    /// HTTP status code to use to abort the Http request.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpStatus")]
    pub http_status: Option<i32>,
    /// Percentage of requests to be aborted with the error code provided.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percentage: Option<VirtualServiceHttpFaultAbortPercentage>,
}

/// Percentage of requests to be aborted with the error code provided.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceHttpFaultAbortPercentage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceHttpFaultDelay {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "exponentialDelay")]
    pub exponential_delay: Option<String>,
    /// Add a fixed delay before forwarding the request.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fixedDelay")]
    pub fixed_delay: Option<String>,
    /// Percentage of requests on which the delay will be injected (0-100).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percent: Option<i32>,
    /// Percentage of requests on which the delay will be injected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percentage: Option<VirtualServiceHttpFaultDelayPercentage>,
}

/// Percentage of requests on which the delay will be injected.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceHttpFaultDelayPercentage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceHttpHeaders {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<VirtualServiceHttpHeadersRequest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<VirtualServiceHttpHeadersResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceHttpHeadersRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceHttpHeadersResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceHttpMatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authority: Option<VirtualServiceHttpMatchAuthority>,
    /// Names of gateways where the rule should be applied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateways: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<BTreeMap<String, VirtualServiceHttpMatchHeaders>>,
    /// Flag to specify whether the URI matching should be case-insensitive.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ignoreUriCase")]
    pub ignore_uri_case: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<VirtualServiceHttpMatchMethod>,
    /// The name assigned to a match.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specifies the ports on the host that is being addressed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// Query parameters for matching.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "queryParams")]
    pub query_params: Option<BTreeMap<String, VirtualServiceHttpMatchQueryParams>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<VirtualServiceHttpMatchScheme>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceLabels")]
    pub source_labels: Option<BTreeMap<String, String>>,
    /// Source namespace constraining the applicability of a rule to workloads in that namespace.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceNamespace")]
    pub source_namespace: Option<String>,
    /// The human readable prefix to use when emitting statistics for this route.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statPrefix")]
    pub stat_prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<VirtualServiceHttpMatchUri>,
    /// withoutHeader has the same syntax with the header, but has opposite meaning.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "withoutHeaders")]
    pub without_headers: Option<BTreeMap<String, VirtualServiceHttpMatchWithoutHeaders>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceHttpMatchAuthority {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// RE2 style regex-based match (https://github.com/google/re2/wiki/Syntax).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceHttpMatchHeaders {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// RE2 style regex-based match (https://github.com/google/re2/wiki/Syntax).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceHttpMatchMethod {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// RE2 style regex-based match (https://github.com/google/re2/wiki/Syntax).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
}

/// Query parameters for matching.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceHttpMatchQueryParams {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// RE2 style regex-based match (https://github.com/google/re2/wiki/Syntax).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceHttpMatchScheme {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// RE2 style regex-based match (https://github.com/google/re2/wiki/Syntax).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceHttpMatchUri {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// RE2 style regex-based match (https://github.com/google/re2/wiki/Syntax).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
}

/// withoutHeader has the same syntax with the header, but has opposite meaning.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceHttpMatchWithoutHeaders {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// RE2 style regex-based match (https://github.com/google/re2/wiki/Syntax).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceHttpMirror {
    /// The name of a service from the service registry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Specifies the port on the host that is being addressed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<VirtualServiceHttpMirrorPort>,
    /// The name of a subset within the service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subset: Option<String>,
}

/// Specifies the port on the host that is being addressed.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceHttpMirrorPort {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
}

/// Percentage of the traffic to be mirrored by the `mirror` field.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceHttpMirrorPercentage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

/// A HTTP rule can either return a direct_response, redirect or forward (default) traffic.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceHttpRedirect {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authority: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "derivePort")]
    pub derive_port: Option<VirtualServiceHttpRedirectDerivePort>,
    /// On a redirect, overwrite the port portion of the URL with this value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "redirectCode")]
    pub redirect_code: Option<i64>,
    /// On a redirect, overwrite the scheme portion of the URL with this value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

/// A HTTP rule can either return a direct_response, redirect or forward (default) traffic.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum VirtualServiceHttpRedirectDerivePort {
    #[serde(rename = "FROM_PROTOCOL_DEFAULT")]
    FromProtocolDefault,
    #[serde(rename = "FROM_REQUEST_PORT")]
    FromRequestPort,
}

/// Retry policy for HTTP requests.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceHttpRetries {
    /// Number of retries to be allowed for a given request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attempts: Option<i32>,
    /// Timeout per attempt for a given request, including the initial call and any retries.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "perTryTimeout")]
    pub per_try_timeout: Option<String>,
    /// Specifies the conditions under which retry takes place.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "retryOn")]
    pub retry_on: Option<String>,
    /// Flag to specify whether the retries should retry to other localities.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "retryRemoteLocalities")]
    pub retry_remote_localities: Option<bool>,
}

/// Rewrite HTTP URIs and Authority headers.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceHttpRewrite {
    /// rewrite the Authority/Host header with this value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authority: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceHttpRoute {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<VirtualServiceHttpRouteDestination>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<VirtualServiceHttpRouteHeaders>,
    /// Weight specifies the relative proportion of traffic to be forwarded to the destination.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceHttpRouteDestination {
    /// The name of a service from the service registry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Specifies the port on the host that is being addressed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<VirtualServiceHttpRouteDestinationPort>,
    /// The name of a subset within the service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subset: Option<String>,
}

/// Specifies the port on the host that is being addressed.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceHttpRouteDestinationPort {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceHttpRouteHeaders {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<VirtualServiceHttpRouteHeadersRequest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<VirtualServiceHttpRouteHeadersResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceHttpRouteHeadersRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceHttpRouteHeadersResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceTcp {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "match")]
    pub r#match: Option<Vec<VirtualServiceTcpMatch>>,
    /// The destination to which the connection should be forwarded to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route: Option<Vec<VirtualServiceTcpRoute>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceTcpMatch {
    /// IPv4 or IPv6 ip addresses of destination with optional subnet.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "destinationSubnets")]
    pub destination_subnets: Option<Vec<String>>,
    /// Names of gateways where the rule should be applied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateways: Option<Vec<String>>,
    /// Specifies the port on the host that is being addressed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceLabels")]
    pub source_labels: Option<BTreeMap<String, String>>,
    /// Source namespace constraining the applicability of a rule to workloads in that namespace.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceNamespace")]
    pub source_namespace: Option<String>,
    /// IPv4 or IPv6 ip address of source with optional subnet.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceSubnet")]
    pub source_subnet: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceTcpRoute {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<VirtualServiceTcpRouteDestination>,
    /// Weight specifies the relative proportion of traffic to be forwarded to the destination.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceTcpRouteDestination {
    /// The name of a service from the service registry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Specifies the port on the host that is being addressed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<VirtualServiceTcpRouteDestinationPort>,
    /// The name of a subset within the service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subset: Option<String>,
}

/// Specifies the port on the host that is being addressed.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceTcpRouteDestinationPort {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceTls {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "match")]
    pub r#match: Option<Vec<VirtualServiceTlsMatch>>,
    /// The destination to which the connection should be forwarded to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route: Option<Vec<VirtualServiceTlsRoute>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceTlsMatch {
    /// IPv4 or IPv6 ip addresses of destination with optional subnet.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "destinationSubnets")]
    pub destination_subnets: Option<Vec<String>>,
    /// Names of gateways where the rule should be applied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateways: Option<Vec<String>>,
    /// Specifies the port on the host that is being addressed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// SNI (server name indicator) to match on.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sniHosts")]
    pub sni_hosts: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceLabels")]
    pub source_labels: Option<BTreeMap<String, String>>,
    /// Source namespace constraining the applicability of a rule to workloads in that namespace.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceNamespace")]
    pub source_namespace: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceTlsRoute {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<VirtualServiceTlsRouteDestination>,
    /// Weight specifies the relative proportion of traffic to be forwarded to the destination.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceTlsRouteDestination {
    /// The name of a service from the service registry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Specifies the port on the host that is being addressed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<VirtualServiceTlsRouteDestinationPort>,
    /// The name of a subset within the service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subset: Option<String>,
}

/// Specifies the port on the host that is being addressed.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VirtualServiceTlsRouteDestinationPort {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
}

