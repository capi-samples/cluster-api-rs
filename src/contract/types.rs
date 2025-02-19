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

// Path defines a how to access a field in an Unstructured object.

use std::{marker::PhantomData, path};

use jsonptr::{PointerBuf, Token};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unable to resolve path: {0}")]
    ResolveErr(#[from] jsonptr::resolve::Error),

    #[error("Unable to assign to path: {0}")]
    AssignErr(#[from] jsonptr::assign::Error),

    #[error("Unable to decode reference: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("No available references to resolve")]
    NoRefError,
}

pub type Result<R> = std::result::Result<R, Error>;

#[derive(PartialEq, Eq)]
pub struct Path<R>(pub jsonptr::PointerBuf, PhantomData<R>);

impl<R> Path<R> {
    pub fn new(path: jsonptr::PointerBuf) -> Self {
        Self(path, PhantomData::default())
    }

    pub fn from_tokens<'t>(tokens: impl IntoIterator<Item: Into<Token<'t>>>) -> Self {
        Self::new(PointerBuf::from_tokens(tokens))
    }

    pub fn path(&self) -> jsonptr::PointerBuf {
        self.0.clone()
    }
}

impl<R: Serialize + DeserializeOwned> Path<R> {
    pub fn get(&self, obj: &kube::api::DynamicObject) -> Result<R> {
        Ok(serde_json::from_value(self.0.resolve(&obj.data)?.clone())?)
    }

    pub fn set(&self, obj: &mut kube::api::DynamicObject, data: R) -> Result<()> {
        self.0.assign(&mut obj.data, serde_json::to_value(data)?)?;
        Ok(())
    }
}

impl<R> ToString for Path<R> {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

pub struct Paths<R>(Vec<Path<R>>);

impl<R> Paths<R> {
    pub fn new(paths: Vec<Path<R>>) -> Self {
        Self(paths)
    }
}

impl<R: Serialize + DeserializeOwned> Paths<R> {
    pub fn get(&self, obj: &kube::api::DynamicObject) -> Result<R> {
        let p = self.0.first().ok_or(Error::NoRefError)?;
        for path in &self.0 {
            if let Ok(data) = path.get(obj) {
                return Ok(data);
            };
        }

        p.get(obj)
    }

    pub fn set(&self, obj: &mut kube::api::DynamicObject, data: R) -> Result<()> {
        let p = self.0.first().ok_or(Error::NoRefError)?;
        p.set(obj, data)
    }
}
