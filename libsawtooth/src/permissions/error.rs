/*
 * Copyright 2018 Bitwise IO
 * Copyright 2020 Cargill Incorporated
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * ------------------------------------------------------------------------------
 */

//! Permission-related errors.

use std::error::Error;
use std::fmt;

/// An error that may occur while interacting with an IdentitySource.
#[derive(Debug)]
pub enum IdentityError {
    ReadError(String),
}

impl Error for IdentityError {}

impl fmt::Display for IdentityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IdentityError::ReadError(s) => write!(f, "Unable to read: {}", s),
        }
    }
}
