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

use serde::{Deserialize, Serialize};

use super::types::{Error, Path};

#[derive(Serialize, Deserialize)]
pub struct ObjectMeta {
    // labels is a map of string keys and values that can be used to organize and categorize
    // (scope and select) objects. May match selectors of replication controllers
    // and services.
    // More info: http://kubernetes.io/docs/user-guide/labels
    // +optional
    labels: std::collections::HashMap<String, String>,

    // annotations is an unstructured key value map stored with a resource that may be
    // set by external tools to store and retrieve arbitrary metadata. They are not
    // queryable and should be preserved when modifying objects.
    // More info: http://kubernetes.io/docs/user-guide/annotations
    // +optional
    annotations: std::collections::HashMap<String, String>,
}

// Metadata provides a helper struct for working with Metadata.
pub struct Metadata {
    pub path: Path<ObjectMeta>,
}

impl Metadata {
    pub fn new(path: Path<ObjectMeta>) -> Self {
        Self { path }
    }

    // Path returns the path of the metadata.
    pub fn path(&self) -> &jsonptr::PointerBuf {
        &self.path.0
    }

    // Get gets the metadata object.
    pub fn get(&self, obj: &kube::api::DynamicObject) -> Result<ObjectMeta, Error> {
        self.path.get(obj)
    }

    // Set sets the metadata value.
    // Note: We are blanking out empty label annotations, thus avoiding triggering infinite reconcile
    // given that at json level label: {} or annotation: {} is different from no field, which is the
    // corresponding value stored in etcd given that those fields are defined as omitempty.
    pub fn set(&self, obj: &mut kube::api::DynamicObject, metadata: ObjectMeta) -> Result<(), Error> {
        if !metadata.labels.is_empty() {
            let mut labels_path = self.path().clone();
            labels_path.push_front("labels");
            Path::new(labels_path).set(obj, metadata.labels)?;
        }

        if !metadata.annotations.is_empty() {
            let mut annotations_path = self.path().clone();
            annotations_path.push_front("annotations");
            Path::new(annotations_path).set(obj, metadata.annotations)?;
        }

        Ok(())
    }
}
