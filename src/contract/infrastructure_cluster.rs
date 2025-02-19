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

use std::collections::BTreeMap;

use jsonptr::PointerBuf;
use kube::api::DynamicObject;

use crate::capi_cluster::ClusterStatusFailureDomains;

use super::types::{Path, Result};

type FailureDomains = BTreeMap<String, ClusterStatusFailureDomains>;

// InfrastructureClusterContract encodes information about the Cluster API contract for InfrastructureCluster objects
// like DockerClusters, AWS Clusters, etc.
pub struct InfrastructureClusterContract;

pub fn infrastructure_cluster() -> InfrastructureClusterContract {
    InfrastructureClusterContract
}

impl InfrastructureClusterContract {
    // ControlPlaneEndpoint provides access to ControlPlaneEndpoint in an InfrastructureCluster object.
    pub fn control_plane_endpoint(&self) -> InfrastructureClusterControlPlaneEndpoint {
        InfrastructureClusterControlPlaneEndpoint
    }
}

// InfrastructureClusterControlPlaneEndpoint provides a helper struct for working with ControlPlaneEndpoint
// in an InfrastructureCluster object.
pub struct InfrastructureClusterControlPlaneEndpoint;

impl InfrastructureClusterControlPlaneEndpoint {
    // Host provides access to the host field in the ControlPlaneEndpoint in an InfrastructureCluster object.
    pub fn host(&self) -> Path<String> {
        Path::from_tokens(["spec", "controlPlaneEndpoint", "host"].into_iter())
    }

    // Port provides access to the port field in the ControlPlaneEndpoint in an InfrastructureCluster object.
    pub fn port(&self) -> Path<i64> {
        Path::from_tokens(["spec", "controlPlaneEndpoint", "port"].into_iter())
    }
}

impl InfrastructureClusterContract {
    // Ready provides access to the status.ready field in an InfrastructureCluster object.
    pub fn ready(&self) -> Path<bool> {
        Path::from_tokens(["status", "ready"].into_iter())
    }

    // ReadyConditionType returns the type of the ready condition.
    pub fn ready_condition_type(&self) -> String {
        "Ready".to_string()
    }

    // FailureReason provides access to the status.failureReason field in an InfrastructureCluster object. Note that this field is optional.
    //
    // Deprecated: This function is deprecated and is going to be removed. Please see https://github.com/kubernetes-sigs/cluster-api/blob/main/docs/proposals/20240916-improve-status-in-CAPI-resources.md for more details.
    pub fn failure_reason(&self) -> Path<String> {
        Path::from_tokens(["status", "failureReason"].into_iter())
    }

    // FailureMessage provides access to the status.failureMessage field in an InfrastructureCluster object. Note that this field is optional.
    //
    // Deprecated: This function is deprecated and is going to be removed. Please see https://github.com/kubernetes-sigs/cluster-api/blob/main/docs/proposals/20240916-improve-status-in-CAPI-resources.md for more details.
    pub fn failure_message(&self) -> Path<String> {
        Path::from_tokens(["status", "failureMessage"].into_iter())
    }

    // FailureDomains provides access to the status.failureDomains field in an InfrastructureCluster object. Note that this field is optional.
    pub fn failure_domains(&self) -> Path<FailureDomains> {
        Path::from_tokens(["status", "failureDomains"].into_iter())
    }

    // IgnorePaths returns a list of paths to be ignored when reconciling an InfrastructureCluster.
    // NOTE: The controlPlaneEndpoint struct currently contains two mandatory fields (host and port).
    // As the host and port fields are not using omitempty, they are automatically set to their zero values
    // if they are not set by the user. We don't want to reconcile the zero values as we would then overwrite
    // changes applied by the infrastructure provider controller.
    pub fn ignore_paths(infrastructure_cl: &DynamicObject) -> Result<Vec<PointerBuf>> {
        let mut ignore_paths = vec![];
        let host = infrastructure_cluster()
            .control_plane_endpoint()
            .host()
            .get(infrastructure_cl)?;
        if host.is_empty() {
            ignore_paths.push(
                infrastructure_cluster()
                    .control_plane_endpoint()
                    .host()
                    .path(),
            )
        }

        let port = infrastructure_cluster()
            .control_plane_endpoint()
            .port()
            .get(infrastructure_cl)?;
        if port == 0 {
            ignore_paths.push(
                infrastructure_cluster()
                    .control_plane_endpoint()
                    .port()
                    .path(),
            )
        }

        Ok(ignore_paths)
    }
}
