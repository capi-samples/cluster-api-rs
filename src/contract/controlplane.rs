/*
Copyright 2021 The Kubernetes Authors.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use k8s_openapi::api::core::v1::ObjectReference;
use kube::{api::DynamicObject, core::Duration};
use semver::Version;

use super::{
    metadata::Metadata,
    types::{Error, Path, Paths, Result},
};

// ControlPlaneContract encodes information about the Cluster API contract for ControlPlane objects
// like e.g the KubeadmControlPlane etc.
pub struct ControlPlaneContract;

// ControlPlane provide access to the information about the Cluster API contract for ControlPlane objects.
pub fn control_plane() -> ControlPlaneContract {
    ControlPlaneContract
}

impl ControlPlaneContract {
    // MachineTemplate provides access to MachineTemplate in a ControlPlane object, if any.
    // NOTE: When working with unstructured there is no way to understand if the ControlPlane provider
    // do support a field in the type definition from the fact that a field is not set in a given instance.
    // This is why in we are deriving if MachineTemplate is required from the ClusterClass in the topology reconciler code.
    pub fn machine_template(&self) -> ControlPlaneMachineTemplate {
        ControlPlaneMachineTemplate
    }

    // Version provide access to version field in a ControlPlane object, if any.
    // NOTE: When working with unstructured there is no way to understand if the ControlPlane provider
    // do support a field in the type definition from the fact that a field is not set in a given instance.
    // This is why in we are deriving if version is required from the ClusterClass in the topology reconciler code.
    pub fn version(&self) -> Path<Version> {
        Path::from_tokens(["spec", "version"].into_iter())
    }

    // StatusVersion provide access to the version field in a ControlPlane object status, if any.
    pub fn status_version(&self) -> Path<Version> {
        Path::from_tokens(["status", "version"].into_iter())
    }

    // Ready provide access to the status.ready field in a ControlPlane object.
    pub fn ready(&self) -> Path<bool> {
        Path::from_tokens(["status", "ready"].into_iter())
    }

    // Initialized provide access to status.initialized field in a ControlPlane object.
    pub fn initialized(&self) -> Path<bool> {
        Path::from_tokens(["status", "initialized"].into_iter())
    }

    // Replicas provide access to replicas field in a ControlPlane object, if any.
    // NOTE: When working with unstructured there is no way to understand if the ControlPlane provider
    // do support a field in the type definition from the fact that a field is not set in a given instance.
    // This is why in we are deriving if replicas is required from the ClusterClass in the topology reconciler code.
    pub fn replicas(&self) -> Path<i64> {
        Path::from_tokens(["spec", "replicas"].into_iter())
    }

    // StatusReplicas provide access to the status.replicas field in a ControlPlane object, if any. Applies to implementations using replicas.
    pub fn status_replicas(&self) -> Path<i64> {
        Path::from_tokens(["status", "replicas"].into_iter())
    }

    // UpdatedReplicas provide access to the status.updatedReplicas field in a ControlPlane object, if any. Applies to implementations using replicas.
    pub fn updated_replicas(&self) -> Path<i64> {
        Path::from_tokens(["status", "updatedReplicas"].into_iter())
    }

    // ReadyReplicas provide access to the status.readyReplicas field in a ControlPlane object, if any. Applies to implementations using replicas.
    pub fn ready_replicas(&self) -> Path<i64> {
        Path::from_tokens(["status", "readyReplicas"].into_iter())
    }

    // UnavailableReplicas provide access to the status.unavailableReplicas field in a ControlPlane object, if any. Applies to implementations using replicas.
    pub fn unavailable_replicas(&self) -> Path<i64> {
        Path::from_tokens(["status", "unavailableReplicas"].into_iter())
    }

    // V1Beta2ReadyReplicas provide access to readyReplicas field in a ControlPlane object, if any. Applies to implementations using replicas.
    pub fn v1_beta2_ready_replicas(&self) -> Paths<i32> {
        Paths::new(vec![
            Path::from_tokens(["status", "v1beta2", "readyReplicas"].into_iter()),
            Path::from_tokens(["status", "readyReplicas"].into_iter()),
        ])
    }

    // V1Beta2AvailableReplicas provide access to the availableReplicas field in a ControlPlane object, if any. Applies to implementations using replicas.
    pub fn v1_beta2_available_replicas(&self) -> Paths<i32> {
        Paths::new(vec![
            Path::from_tokens(["status", "v1beta2", "availableReplicas"].into_iter()),
            Path::from_tokens(["status", "availableReplicas"].into_iter()),
        ])
    }

    // V1Beta2UpToDateReplicas provide access to the upToDateReplicas field in a ControlPlane object, if any. Applies to implementations using replicas.
    pub fn v1_beta2_up_to_date_replicas(&self) -> Paths<i32> {
        Paths::new(vec![
            Path::from_tokens(["status", "v1beta2", "upToDateReplicas"].into_iter()),
            Path::from_tokens(["status", "upToDateReplicas"].into_iter()),
        ])
    }

    // AvailableConditionType returns the type of the available condition.
    pub fn available_condition_type(&self) -> String {
        "Available".to_string()
    }

    // Selector provide access to the status.selector field in a ControlPlane object, if any. Applies to implementations using replicas.
    pub fn selector(&self) -> Path<String> {
        Path::from_tokens(["status", "selector"].into_iter())
    }

    // FailureReason provides access to the status.failureReason field in an ControlPlane object. Note that this field is optional.
    //
    // Deprecated: This function is deprecated and is going to be removed. Please see https://github.com/kubernetes-sigs/cluster-api/blob/main/docs/proposals/20240916-improve-status-in-CAPI-resources.md for more details.
    pub fn failure_reason(&self) -> Path<String> {
        Path::from_tokens(["status", "failureReason"].into_iter())
    }

    // FailureMessage provides access to the status.failureMessage field in an ControlPlane object. Note that this field is optional.
    //
    // Deprecated: This function is deprecated and is going to be removed. Please see https://github.com/kubernetes-sigs/cluster-api/blob/main/docs/proposals/20240916-improve-status-in-CAPI-resources.md for more details.
    pub fn failure_message(&self) -> Path<String> {
        Path::from_tokens(["status", "failureMessage"].into_iter())
    }

    // ExternalManagedControlPlane provides access to the status.externalManagedControlPlane field in an ControlPlane object.
    // Note that this field is optional.
    pub fn external_managed_control_plane(&self) -> Path<bool> {
        Path::from_tokens(["status", "externalManagedControlPlane"].into_iter())
    }

    // IsProvisioning returns true if the control plane is being created for the first time.
    // Returns false, if the control plane was already previously provisioned.
    pub fn is_provisioning(&self, obj: &DynamicObject) -> Result<bool> {
        // We can know if the control plane was previously created or is being cretaed for the first
        // time by looking at controlplane.status.version. If the version in status is set to a valid
        // value then the control plane was already provisioned at a previous time. If not, we can
        // assume that the control plane is being created for the first time.
        match self.status_version().get(obj) {
            Ok(_) => Ok(false),
            // Empty version
            Err(Error::Serde(_)) => Ok(true),
            // Missing version
            Err(Error::ResolveErr(_)) => Ok(true),
            Err(e) => Err(e)?,
        }
    }

    // IsUpgrading returns true if the control plane is in the middle of an upgrade, false otherwise.
    // A control plane is considered upgrading if:
    // - if spec.version is greater than status.version.
    // Note: A control plane is considered not upgrading if the status or status.version is not set.
    pub fn is_upgrading(&self, obj: &DynamicObject) -> Result<bool> {
        let spec_version = self.version().get(obj)?;
        match self.status_version().get(obj) {
            Ok(version) => Ok(spec_version > version),
            // status version is not yet set
            // If the status.version is not yet present in the object, it implies the
            // first machine of the control plane is provisioning. We can reasonably assume
            // that the control plane is not upgrading at this stage.
            Err(Error::ResolveErr(_)) => Ok(false),
            Err(e) => Err(e)?,
        }
    }

    // IsScaling returns true if the control plane is in the middle of a scale operation, false otherwise.
    // A control plane is considered scaling if:
    // - status.replicas is not yet set.
    // - spec.replicas != status.replicas.
    // - spec.replicas != status.updatedReplicas.
    // - spec.replicas != status.readyReplicas.
    // - status.unavailableReplicas > 0.
    pub fn is_scaling(&self, obj: &DynamicObject) -> Result<bool> {
        let desired_replicas = self.replicas().get(obj)?;

        let status_replicas = match self.status_replicas().get(obj) {
            Ok(replicas) => replicas,
            // status is probably not yet set on the control plane
            // if status is missing we can consider the control plane to be scaling
            // so that we can block any operations that expect control plane to be stable.
            Err(Error::ResolveErr(_)) => return Ok(true),
            e => e?,
        };

        let updated_replicas = match self.updated_replicas().get(obj) {
            Ok(replicas) => replicas,
            // If updatedReplicas is not set on the control plane
            // we should consider the control plane to be scaling so that
            // we block any operation that expect the control plane to be stable.
            Err(Error::ResolveErr(_)) => return Ok(true),
            e => e?,
        };

        let ready_replicas = match self.ready_replicas().get(obj) {
            Ok(replicas) => replicas,
            // If readyReplicas is not set on the control plane
            // we should consider the control plane to be scaling so that
            // we block any operation that expect the control plane to be stable.
            Err(Error::ResolveErr(_)) => return Ok(true),
            e => e?,
        };

        let unavailable_replicas = match self.unavailable_replicas().get(obj) {
            Ok(replicas) => replicas,
            // If unavailableReplicas is not set on the control plane we assume it is 0.
            // We have to do this as the following happens after clusterctl move with KCP:
            // * clusterctl move creates the KCP object without status
            // * the KCP controller won't patch the field to 0 if it doesn't exist
            //   * This is because the patchHelper marshals before/after object to JSON to calculate a diff
            //     and as the unavailableReplicas field is not a pointer, not set and 0 are both rendered as 0.
            //     If before/after of the field is the same (i.e. 0), there is no diff and thus also no patch to set it to 0.
            Err(Error::ResolveErr(_)) => 0,
            e => e?,
        };

        // Control plane is still scaling if:
        // * .spec.replicas, .status.replicas, .status.updatedReplicas,
        //   .status.readyReplicas are not equal and
        // * unavailableReplicas > 0
        if status_replicas != desired_replicas
            || updated_replicas != desired_replicas
            || ready_replicas != desired_replicas
            || unavailable_replicas > 0
        {
            return Ok(true);
        }

        return Ok(false);
    }
}

// ControlPlaneMachineTemplate provides a helper struct for working with MachineTemplate in ClusterClass.
pub struct ControlPlaneMachineTemplate;

impl ControlPlaneMachineTemplate {
    // InfrastructureRef provides access to the infrastructureRef of a MachineTemplate.
    pub fn infrastructure_ref(&self) -> Path<ObjectReference> {
        Path::from_tokens(["spec", "machineTemplate", "infrastructureRef"].into_iter())
    }

    // Metadata provides access to the metadata of a MachineTemplate.
    pub fn metadata(&self) -> Metadata {
        Metadata::new(Path::from_tokens(
            ["spec", "machineTemplate", "metadata"].into_iter(),
        ))
    }

    // NodeDrainTimeout provides access to the nodeDrainTimeout of a MachineTemplate.
    pub fn node_drain_timeout(&self) -> Path<Duration> {
        Path::from_tokens(["spec", "machineTemplate", "nodeDrainTimeout"].into_iter())
    }

    // NodeVolumeDetachTimeout provides access to the nodeVolumeDetachTimeout of a MachineTemplate.
    pub fn node_volume_detach_timeout(&self) -> Path<Duration> {
        Path::from_tokens(["spec", "machineTemplate", "nodeVolumeDetachTimeout"].into_iter())
    }

    // NodeDeletionTimeout provides access to the nodeDeletionTimeout of a MachineTemplate.
    pub fn node_deletion_timeout(&self) -> Path<Duration> {
        Path::from_tokens(["spec", "machineTemplate", "nodeDeletionTimeout"].into_iter())
    }
}
