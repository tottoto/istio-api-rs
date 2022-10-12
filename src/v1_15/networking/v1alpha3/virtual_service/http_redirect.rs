use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
/*
 * Configuration affecting label/content routing, sni routing, etc.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1alpha3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// HttpRedirect : HTTPRedirect can be used to send a 301 redirect response to the caller, where the Authority/Host and the URI in the response can be swapped with the specified values. For example, the following rule redirects requests for /v1/getProductRatings API on the ratings service to /v1/bookRatings provided by the bookratings service.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct HttpRedirect {
    /// On a redirect, overwrite the Authority/Host portion of the URL with this value.
    #[serde(rename = "authority", skip_serializing_if = "Option::is_none")]
    pub authority: Option<String>,
    /// On a redirect, Specifies the HTTP status code to use in the redirect response. The default response code is MOVED_PERMANENTLY (301).
    #[serde(rename = "redirectCode", skip_serializing_if = "Option::is_none")]
    pub redirect_code: Option<i32>,
    /// On a redirect, overwrite the scheme portion of the URL with this value. For example, `http` or `https`. If unset, the original scheme will be used. If `derivePort` is set to `FROM_PROTOCOL_DEFAULT`, this will impact the port used as well
    #[serde(rename = "scheme", skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    /// On a redirect, overwrite the Path portion of the URL with this value. Note that the entire path will be replaced, irrespective of the request URI being matched as an exact path or prefix.
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    /// On a redirect, overwrite the port portion of the URL with this value.
    #[serde(rename = "port")]
    pub port: i32,
    #[serde(rename = "derivePort")]
    pub derive_port: super::HttpRedirectRedirectPortSelection,
}

impl HttpRedirect {
    /// HTTPRedirect can be used to send a 301 redirect response to the caller, where the Authority/Host and the URI in the response can be swapped with the specified values. For example, the following rule redirects requests for /v1/getProductRatings API on the ratings service to /v1/bookRatings provided by the bookratings service.
    pub fn new(port: i32, derive_port: super::HttpRedirectRedirectPortSelection) -> HttpRedirect {
        HttpRedirect {
            authority: None,
            redirect_code: None,
            scheme: None,
            uri: None,
            port,
            derive_port,
        }
    }
}