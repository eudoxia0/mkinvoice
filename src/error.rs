// Copyright 2026 Fernando Borretti
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

use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct ScriptError {
    message: String,
}

impl ScriptError {
    pub fn new(message: impl ToString) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

pub type Fallible<T> = Result<T, ScriptError>;

impl Display for ScriptError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ScriptError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl From<std::io::Error> for ScriptError {
    fn from(value: std::io::Error) -> Self {
        ScriptError {
            message: format!("I/O error: {value}"),
        }
    }
}

impl From<toml::de::Error> for ScriptError {
    fn from(value: toml::de::Error) -> Self {
        ScriptError {
            message: format!("TOML parse error: {value}"),
        }
    }
}
