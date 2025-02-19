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
use kube::core::Duration;

use super::{metadata::Metadata, types::Path};

// ControlPlaneTemplateContract encodes information about the Cluster API contract for ControlPlaneTemplate objects
// like e.g. the KubeadmControlPlane etc.
pub struct ControlPlaneTemplateContract;

// ControlPlaneTemplate provide access to the information about the Cluster API contract for ControlPlaneTemplate objects.
pub fn control_plane_template() -> ControlPlaneTemplateContract {
    ControlPlaneTemplateContract
}

impl ControlPlaneTemplateContract {
    // InfrastructureMachineTemplate provide access to InfrastructureMachineTemplate reference, if any.
    // NOTE: When working with unstructured there is no way to understand if the ControlPlane provider
    // do support a field in the type definition from the fact that a field is not set in a given instance.
    // This is why in we are deriving if this field is required from the ClusterClass in the topology reconciler code.
    pub fn infrastructure_machine_template(&self) -> Path<ObjectReference> {
        Path::from_tokens(
            [
                "spec",
                "template",
                "spec",
                "machineTemplate",
                "infrastructureRef",
            ]
            .into_iter(),
        )
    }

    // Template provides access to the template.
    pub fn template(&self) -> ControlPlaneTemplateTemplate {
        ControlPlaneTemplateTemplate
    }
}

// ControlPlaneTemplateTemplate provides a helper struct for working with the template in an ControlPlaneTemplate.
pub struct ControlPlaneTemplateTemplate;

impl ControlPlaneTemplateTemplate {
    // Metadata provides access to the metadata of a template.
    pub fn metadata(&self) -> Metadata {
        Metadata::new(Path::from_tokens(
            ["spec", "template", "metadata"].into_iter(),
        ))
    }

    // MachineTemplate provides access to MachineTemplate in a ControlPlaneTemplate object, if any.
    pub fn machine_template(&self) -> ControlPlaneTemplateMachineTemplate {
        ControlPlaneTemplateMachineTemplate
    }
}

// ControlPlaneTemplateMachineTemplate provides a helper struct for working with MachineTemplate.
pub struct ControlPlaneTemplateMachineTemplate;

impl ControlPlaneTemplateMachineTemplate {
    // Metadata provides access to the metadata of the MachineTemplate of a ControlPlaneTemplate.
    pub fn metadata(&self) -> Metadata {
        Metadata::new(Path::from_tokens(
            ["spec", "template", "spec", "machineTemplate", "metadata"].into_iter(),
        ))
    }

    // NodeDrainTimeout provides access to the nodeDrainTimeout of a MachineTemplate.
    pub fn node_drain_timeout(&self) -> Path<Duration> {
        Path::from_tokens(
            [
                "spec",
                "template",
                "spec",
                "machineTemplate",
                "nodeDrainTimeout",
            ]
            .into_iter(),
        )
    }

    // NodeVolumeDetachTimeout provides access to the nodeVolumeDetachTimeout of a MachineTemplate.
    pub fn node_volume_detach_timeout(&self) -> Path<Duration> {
        Path::from_tokens(
            [
                "spec",
                "template",
                "spec",
                "machineTemplate",
                "nodeVolumeDetachTimeout",
            ]
            .into_iter(),
        )
    }

    // NodeDeletionTimeout provides access to the nodeDeletionTimeout of a MachineTemplate.
    pub fn node_deletion_timeout(&self) -> Path<Duration> {
        Path::from_tokens(
            [
                "spec",
                "template",
                "spec",
                "machineTemplate",
                "nodeDeletionTimeout",
            ]
            .into_iter(),
        )
    }
}
