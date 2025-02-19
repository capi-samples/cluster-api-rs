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

use super::types::Path;

// BootstrapContract encodes information about the Cluster API contract for bootstrap objects.
pub struct BootstrapContract;

// Bootstrap provide access to the information about the Cluster API contract for bootstrap objects.
pub fn bootstrap() -> BootstrapContract {
    BootstrapContract
}

impl BootstrapContract {
    // Ready provide access to status.ready field in a bootstrap object.
    pub fn ready(&self) -> Path<bool> {
        Path::from_tokens(["status", "ready"].into_iter())
    }

    // ReadyConditionType returns the type of the ready condition.
    pub fn ready_condition_type(&self) -> String {
        "Ready".to_string()
    }

    // DataSecretName provide access to status.dataSecretName field in a bootstrap object.
    pub fn data_secret_name(&self) -> Path<String> {
        Path::from_tokens(["status", "dataSecretName"].into_iter())
    }

    // FailureReason provides access to the status.failureReason field in an bootstrap object. Note that this field is optional.
    //
    // Deprecated: This function is deprecated and is going to be removed. Please see https://github.com/kubernetes-sigs/cluster-api/blob/main/docs/proposals/20240916-improve-status-in-CAPI-resources.md for more details.
    pub fn failure_reason(&self) -> Path<String> {
        Path::from_tokens(["status", "failureReason"].into_iter())
    }

    // FailureMessage provides access to the status.failureMessage field in an bootstrap object. Note that this field is optional.
    //
    // Deprecated: This function is deprecated and is going to be removed. Please see https://github.com/kubernetes-sigs/cluster-api/blob/main/docs/proposals/20240916-improve-status-in-CAPI-resources.md for more details.
    pub fn failure_message(&self) -> Path<String> {
        Path::from_tokens(["status", "failureMessage"].into_iter())
    }
}
