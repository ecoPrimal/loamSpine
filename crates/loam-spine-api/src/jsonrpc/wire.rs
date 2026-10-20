// SPDX-License-Identifier: AGPL-3.0-or-later

//! JSON-RPC 2.0 wire types and error codes.
//!
//! These types define the protocol-level structures for JSON-RPC 2.0
//! communication. They are transport-agnostic and used by both TCP
//! and UDS server implementations.
//!
//! `JsonRpcError` and `JsonRpcResponse` are the canonical types from
//! `cellmembrane-types`. `JsonRpcRequest` is kept local because its
//! non-optional `params`/`id` fields (with `#[serde(default)]`) are
//! deeply embedded throughout the dispatch and test code — the canonical
//! type uses `Option<Value>` which would require 50+ call-site changes
//! for no wire-level benefit.

use serde::{Deserialize, Serialize};

// Standard JSON-RPC 2.0 error codes
pub(crate) const PARSE_ERROR: i32 = -32700;
pub(crate) const INVALID_REQUEST: i32 = -32600;
pub(crate) const METHOD_NOT_FOUND: i32 = -32601;
pub(crate) const INVALID_PARAMS: i32 = -32602;
pub(crate) const LOAMSPINE_ERROR: i32 = -32000;

/// A JSON-RPC 2.0 request — kept local.
///
/// Uses non-optional `params`/`id` with `#[serde(default)]` (defaulting to
/// `Value::Null`). The canonical type wraps these in `Option`, which would
/// require unwrapping at every dispatch call site.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    /// Protocol version (must be "2.0").
    pub jsonrpc: String,
    /// Method name.
    pub method: String,
    /// Method parameters.
    #[serde(default)]
    pub params: serde_json::Value,
    /// Request ID (null for notifications).
    #[serde(default)]
    pub id: serde_json::Value,
}

/// A JSON-RPC 2.0 response — canonical type from cellmembrane-types.
pub use cellmembrane_types::JsonRpcResponse;

/// A JSON-RPC 2.0 error — canonical type from cellmembrane-types.
pub use cellmembrane_types::JsonRpcError;
