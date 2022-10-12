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

/// K8sIoApiCoreV1VolumeSource : Represents the source of a volume to mount. Only one of its members may be specified.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct K8sIoApiCoreV1VolumeSource {
    #[serde(rename = "awsElasticBlockStore", skip_serializing_if = "Option::is_none")]
    pub aws_elastic_block_store: Option<Box<super::K8sIoApiCoreV1AwsElasticBlockStoreVolumeSource>>,
    #[serde(rename = "azureDisk", skip_serializing_if = "Option::is_none")]
    pub azure_disk: Option<Box<super::K8sIoApiCoreV1AzureDiskVolumeSource>>,
    #[serde(rename = "azureFile", skip_serializing_if = "Option::is_none")]
    pub azure_file: Option<Box<super::K8sIoApiCoreV1AzureFileVolumeSource>>,
    #[serde(rename = "cephfs", skip_serializing_if = "Option::is_none")]
    pub cephfs: Option<Box<super::K8sIoApiCoreV1CephFsVolumeSource>>,
    #[serde(rename = "cinder", skip_serializing_if = "Option::is_none")]
    pub cinder: Option<Box<super::K8sIoApiCoreV1CinderVolumeSource>>,
    #[serde(rename = "configMap", skip_serializing_if = "Option::is_none")]
    pub config_map: Option<Box<super::K8sIoApiCoreV1ConfigMapVolumeSource>>,
    #[serde(rename = "csi", skip_serializing_if = "Option::is_none")]
    pub csi: Option<Box<super::K8sIoApiCoreV1CsiVolumeSource>>,
    #[serde(rename = "downwardAPI", skip_serializing_if = "Option::is_none")]
    pub downward_api: Option<Box<super::K8sIoApiCoreV1DownwardApiVolumeSource>>,
    #[serde(rename = "emptyDir", skip_serializing_if = "Option::is_none")]
    pub empty_dir: Option<Box<super::K8sIoApiCoreV1EmptyDirVolumeSource>>,
    #[serde(rename = "ephemeral", skip_serializing_if = "Option::is_none")]
    pub ephemeral: Option<Box<super::K8sIoApiCoreV1EphemeralVolumeSource>>,
    #[serde(rename = "fc", skip_serializing_if = "Option::is_none")]
    pub fc: Option<Box<super::K8sIoApiCoreV1FcVolumeSource>>,
    #[serde(rename = "flexVolume", skip_serializing_if = "Option::is_none")]
    pub flex_volume: Option<Box<super::K8sIoApiCoreV1FlexVolumeSource>>,
    #[serde(rename = "flocker", skip_serializing_if = "Option::is_none")]
    pub flocker: Option<Box<super::K8sIoApiCoreV1FlockerVolumeSource>>,
    #[serde(rename = "gcePersistentDisk", skip_serializing_if = "Option::is_none")]
    pub gce_persistent_disk: Option<Box<super::K8sIoApiCoreV1GcePersistentDiskVolumeSource>>,
    #[serde(rename = "gitRepo", skip_serializing_if = "Option::is_none")]
    pub git_repo: Option<Box<super::K8sIoApiCoreV1GitRepoVolumeSource>>,
    #[serde(rename = "glusterfs", skip_serializing_if = "Option::is_none")]
    pub glusterfs: Option<Box<super::K8sIoApiCoreV1GlusterfsVolumeSource>>,
    #[serde(rename = "hostPath", skip_serializing_if = "Option::is_none")]
    pub host_path: Option<Box<super::K8sIoApiCoreV1HostPathVolumeSource>>,
    #[serde(rename = "iscsi", skip_serializing_if = "Option::is_none")]
    pub iscsi: Option<Box<super::K8sIoApiCoreV1IscsiVolumeSource>>,
    #[serde(rename = "nfs", skip_serializing_if = "Option::is_none")]
    pub nfs: Option<Box<super::K8sIoApiCoreV1NfsVolumeSource>>,
    #[serde(rename = "persistentVolumeClaim", skip_serializing_if = "Option::is_none")]
    pub persistent_volume_claim: Option<Box<super::K8sIoApiCoreV1PersistentVolumeClaimVolumeSource>>,
    #[serde(rename = "photonPersistentDisk", skip_serializing_if = "Option::is_none")]
    pub photon_persistent_disk: Option<Box<super::K8sIoApiCoreV1PhotonPersistentDiskVolumeSource>>,
    #[serde(rename = "portworxVolume", skip_serializing_if = "Option::is_none")]
    pub portworx_volume: Option<Box<super::K8sIoApiCoreV1PortworxVolumeSource>>,
    #[serde(rename = "projected", skip_serializing_if = "Option::is_none")]
    pub projected: Option<Box<super::K8sIoApiCoreV1ProjectedVolumeSource>>,
    #[serde(rename = "quobyte", skip_serializing_if = "Option::is_none")]
    pub quobyte: Option<Box<super::K8sIoApiCoreV1QuobyteVolumeSource>>,
    #[serde(rename = "rbd", skip_serializing_if = "Option::is_none")]
    pub rbd: Option<Box<super::K8sIoApiCoreV1RbdVolumeSource>>,
    #[serde(rename = "scaleIO", skip_serializing_if = "Option::is_none")]
    pub scale_io: Option<Box<super::K8sIoApiCoreV1ScaleIoVolumeSource>>,
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<Box<super::K8sIoApiCoreV1SecretVolumeSource>>,
    #[serde(rename = "storageos", skip_serializing_if = "Option::is_none")]
    pub storageos: Option<Box<super::K8sIoApiCoreV1StorageOsVolumeSource>>,
    #[serde(rename = "vsphereVolume", skip_serializing_if = "Option::is_none")]
    pub vsphere_volume: Option<Box<super::K8sIoApiCoreV1VsphereVirtualDiskVolumeSource>>,
}

impl K8sIoApiCoreV1VolumeSource {
    /// Represents the source of a volume to mount. Only one of its members may be specified.
    pub fn new() -> K8sIoApiCoreV1VolumeSource {
        K8sIoApiCoreV1VolumeSource {
            aws_elastic_block_store: None,
            azure_disk: None,
            azure_file: None,
            cephfs: None,
            cinder: None,
            config_map: None,
            csi: None,
            downward_api: None,
            empty_dir: None,
            ephemeral: None,
            fc: None,
            flex_volume: None,
            flocker: None,
            gce_persistent_disk: None,
            git_repo: None,
            glusterfs: None,
            host_path: None,
            iscsi: None,
            nfs: None,
            persistent_volume_claim: None,
            photon_persistent_disk: None,
            portworx_volume: None,
            projected: None,
            quobyte: None,
            rbd: None,
            scale_io: None,
            secret: None,
            storageos: None,
            vsphere_volume: None,
        }
    }
}