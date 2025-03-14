/*
Copyright 2022 The Kubernetes Authors.

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

use crate::capi_machine::MachineStatusAddresses;

use super::types::Path;

pub type MachineAddresses = Vec<MachineStatusAddresses>;

// InfrastructureMachineContract encodes information about the Cluster API contract for InfrastructureMachine objects
// like DockerMachines, AWS Machines, etc.
pub struct InfrastructureMachineContract;

// InfrastructureMachine provide access to the information about the Cluster API contract for InfrastructureMachine objects.
pub fn infrastructure_machine() -> InfrastructureMachineContract {
    InfrastructureMachineContract
}

impl InfrastructureMachineContract {
    // Ready provides access to status.ready field in an InfrastructureMachine object.
    pub fn ready(&self) -> Path<bool> {
        Path::from_tokens(["status", "ready"].into_iter())
    }

    // ReadyConditionType returns the type of the ready condition.
    pub fn ready_condition_type(&self) -> String {
        "Ready".to_string()
    }

    // FailureReason provides access to the status.failureReason field in an InfrastructureMachine object. Note that this field is optional.
    //
    // Deprecated: This function is deprecated and is going to be removed. Please see https://github.com/kubernetes-sigs/cluster-api/blob/main/docs/proposals/20240916-improve-status-in-CAPI-resources.md for more details.
    pub fn failure_reason(&self) -> Path<String> {
        Path::from_tokens(["status", "failureReason"].into_iter())
    }

    // FailureMessage provides access to the status.failureMessage field in an InfrastructureMachine object. Note that this field is optional.
    //
    // Deprecated: This function is deprecated and is going to be removed. Please see https://github.com/kubernetes-sigs/cluster-api/blob/main/docs/proposals/20240916-improve-status-in-CAPI-resources.md for more details.
    pub fn failure_message(&self) -> Path<String> {
        Path::from_tokens(["status", "failureMessage"].into_iter())
    }

    // Addresses provides access to the status.addresses field in an InfrastructureMachine object. Note that this field is optional.
    pub fn addresses(&self) -> Path<MachineAddresses> {
        Path::from_tokens(["status", "addresses"].into_iter())
    }

    // ProviderID provides access to the spec.providerID field in an InfrastructureMachine object.
    pub fn provider_id(&self) -> Path<String> {
        Path::from_tokens(["spec", "providerID"].into_iter())
    }

    // FailureDomain provides access to the spec.failureDomain field in an InfrastructureMachine object. Note that this field is optional.
    pub fn failure_domain(&self) -> Path<String> {
        Path::from_tokens(["spec", "failureDomain"].into_iter())
    }
}
