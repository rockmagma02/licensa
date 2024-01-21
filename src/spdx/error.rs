// Copyright 2024 Nelson Dominguez
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{error::Error, fmt, io};

use clap::error::ErrorKind;
use derive_more::Display;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SpdxError {
  #[error("SPDX license ID \"{0}\" not found")]
  NotFound(String),

  #[error("Failed to read licenses metadata file")]
  MetadataFileNotFound,

  #[error(transparent)]
  HttpError(#[from] reqwest::Error),

  #[error(transparent)]
  DataError(#[from] serde_json::Error),

  #[error(transparent)]
  Io(#[from] io::Error),
}

// impl Error for SpdxError {}

pub fn to_clap_error(err: impl ToString) -> String {
  let val = err.to_string();
  println!("{val}");
  val
}
