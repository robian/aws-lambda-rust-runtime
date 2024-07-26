use http::{HeaderMap, Method};
use query_map::QueryMap;
use serde::{Deserialize, Serialize};

use crate::{
    custom_serde::{deserialize_headers, deserialize_nullish_boolean, http_method, serialize_headers},
    encodings::Body,
};

/// `VpcLatticeEventV2` contains data originating from the VPC Lattice Lambda target integration
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VpcLatticeEventV2 {
    #[serde(deserialize_with = "deserialize_headers", default)]
    #[serde(serialize_with = "serialize_headers")]
    pub headers: HeaderMap,
    #[serde(with = "http_method")]
    pub method: Method,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub query_string_parameters: QueryMap,
    pub request_context: VpcLatticeEventContext,
    pub version: String,
    pub body: Option<String>,
    #[serde(default, deserialize_with = "deserialize_nullish_boolean")]
    pub is_base64_encoded: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
    pub principal: Option<String>,
    #[serde(rename = "principalOrgID")]
    pub principal_org_id: Option<String>,
    pub session_name: Option<String>,
    pub source_vpc_arn: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VpcLatticeEventContext {
    pub identity: Identity,
    pub region: String,
    pub service_arn: String,
    pub service_network_arn: String,
    pub target_group_arn: String,
    pub time_epoch: String,
}

/// `VpcLatticeResponse` configures the response to be returned by the VPC Lattice Lambda target for the request
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VpcLatticeResponse {
    pub status_code: i64,
    #[serde(default)]
    pub status_description: Option<String>,
    #[serde(deserialize_with = "http_serde::header_map::deserialize", default)]
    #[serde(serialize_with = "serialize_headers")]
    pub headers: HeaderMap,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Body>,
    #[serde(default, deserialize_with = "deserialize_nullish_boolean")]
    pub is_base64_encoded: bool,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[cfg(feature = "vpc_lattice")]
    fn example_vpc_lattice_lambda_target_request_get() {
        let data = include_bytes!("../../fixtures/example-vpc_lattice-lambda-target-request-get.json");
        let parsed: VpcLatticeEventV2 = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: VpcLatticeEventV2 = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }

    #[test]
    #[cfg(feature = "vpc_lattice")]
    fn example_vpc_lattice_lambda_target_request_post() {
        let data = include_bytes!("../../fixtures/example-vpc_lattice-lambda-target-request-post.json");
        let parsed: VpcLatticeEventV2 = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: VpcLatticeEventV2 = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }

    #[test]
    #[cfg(feature = "vpc_lattice")]
    fn example_vpc_lattice_lambda_target_response() {
        let data = include_bytes!("../../fixtures/example-vpc_lattice-lambda-target-response.json");
        let parsed: VpcLatticeResponse = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: VpcLatticeResponse = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
