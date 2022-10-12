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

/// K8sIoApiCoreV1AwsElasticBlockStoreVolumeSource : Represents a Persistent Disk resource in AWS.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct K8sIoApiCoreV1AwsElasticBlockStoreVolumeSource {
    /// fsType is the filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: \"ext4\", \"xfs\", \"ntfs\". Implicitly inferred to be \"ext4\" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore TODO: how do we prevent errors in the filesystem from compromising the machine +optional
    #[serde(rename = "fsType", skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,
    /// partition is the partition in the volume that you want to mount. If omitted, the default is to mount by volume name. Examples: For volume /dev/sda1, you specify the partition as \"1\". Similarly, the volume partition for /dev/sda is \"0\" (or you can leave the property empty). +optional
    #[serde(rename = "partition", skip_serializing_if = "Option::is_none")]
    pub partition: Option<i32>,
    /// readOnly value true will force the readOnly setting in VolumeMounts. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore +optional
    #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// volumeID is unique ID of the persistent disk resource in AWS (Amazon EBS volume). More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
    #[serde(rename = "volumeID", skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<String>,
}

impl K8sIoApiCoreV1AwsElasticBlockStoreVolumeSource {
    /// Represents a Persistent Disk resource in AWS.
    pub fn new() -> K8sIoApiCoreV1AwsElasticBlockStoreVolumeSource {
        K8sIoApiCoreV1AwsElasticBlockStoreVolumeSource {
            fs_type: None,
            partition: None,
            read_only: None,
            volume_id: None,
        }
    }
}