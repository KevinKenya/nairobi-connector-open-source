// Copyright The TOON Authors
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

//! TOON-to-MCP Bridge — wraps TOON-formatted UI snapshots into MCP Content.

use rmcp::model::Content;

/// Wrap a TOON string into MCP tool result content.
pub fn wrap_toon(toon_str: &str) -> Vec<Content> {
    vec![Content::text(toon_str.to_string())]
}

/// Wrap a plain text string into MCP content.
pub fn wrap_text(text: &str) -> Vec<Content> {
    vec![Content::text(text.to_string())]
}

/// Create an error content for MCP tool failure.
pub fn wrap_error(message: &str) -> Vec<Content> {
    vec![Content::text(format!("ERROR: {}", message))]
}