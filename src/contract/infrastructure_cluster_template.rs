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

use super::{metadata::Metadata, types::Path};

// InfrastructureClusterTemplateContract encodes information about the Cluster API contract for InfrastructureClusterTemplate objects
// like DockerClusterTemplates, AWSClusterTemplates, etc.
pub struct InfrastructureClusterTemplateContract;

// InfrastructureClusterTemplate provides access to the information about the Cluster API contract for InfrastructureClusterTemplate objects.
pub fn infrastructure_cluster_template() -> InfrastructureClusterTemplateContract {
    InfrastructureClusterTemplateContract
}

impl InfrastructureClusterTemplateContract {
    // Template provides access to the template.
    pub fn template(&self) -> InfrastructureClusterTemplateTemplate {
        InfrastructureClusterTemplateTemplate
    }
}

// InfrastructureClusterTemplateTemplate provides a helper struct for working with the template in an InfrastructureClusterTemplate..
pub struct InfrastructureClusterTemplateTemplate;

impl InfrastructureClusterTemplateTemplate {
    // Metadata provides access to the metadata of a template.
    pub fn metadata(&self) -> Metadata {
        Metadata::new(Path::from_tokens(
            ["spec", "template", "metadata"].into_iter(),
        ))
    }
}
