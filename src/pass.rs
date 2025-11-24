//! Serialized definition for TKET passes.
//!
//! Based on the `compiler_pass_v1` schema.
//! <https://github.com/CQCL/tket/blob/main/schemas/compiler_pass_v1.json>

pub mod standard;

#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use standard::StandardPass;

/// Stub for a serialized architecture blob following `architecture_v1.json`.
//
// TODO: Replace with the actual schema.
// <https://github.com/CQCL/tket/blob/main/schemas/architecture_v1.json>
pub type Architecture = serde_json::Value;

/// Stub for a serialized placement blob following `placement_v1.json`.
//
// TODO: Replace with the actual schema.
// <https://github.com/CQCL/tket/blob/main/schemas/placement_v1.json>
pub type Placement = serde_json::Value;

/// Stub for a serialized predicate blob following `predicate_v1.json`.
//
// TODO: Replace with the actual schema.
// <https://github.com/CQCL/tket/blob/main/schemas/predicate_v1.json>
pub type Predicate = serde_json::Value;

/// A pass in a TKET circuit.
//
// This struct is both tagged adjacently (with a `pass_class` string field) and
// externally (the pass definition itself is contained inside a field with the
// same name as the pass class).
//
// Hence the non-standard structure of the enum.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, derive_more::From)]
#[serde(tag = "pass_class")]
pub enum BasePass {
    /// A standard pass.
    StandardPass {
        /// The pass data.
        #[serde(rename = "StandardPass")]
        pass: StandardPass,
    },
    /// A sequence of passes.
    SequencePass {
        /// The pass data.
        #[serde(rename = "SequencePass")]
        pass: SequencePass,
    },
    /// A pass that iterates an internal pass until no further change.
    RepeatPass {
        /// The pass data.
        #[serde(rename = "RepeatPass")]
        pass: RepeatPass,
    },
    /// A pass that iterates an internal pass whilst some metric decreases.
    RepeatWithMetricPass {
        /// The pass data.
        #[serde(rename = "RepeatWithMetricPass")]
        pass: RepeatWithMetricPass,
    },
    /// A pass that iterates an internal pass until some predicate is satisfied.
    RepeatUntilSatisfiedPass {
        /// The pass data.
        #[serde(rename = "RepeatUntilSatisfiedPass")]
        pass: RepeatUntilSatisfiedPass,
    },
}

/// A pass that executes a sequence of passes in order.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct SequencePass {
    /// The passes to be executed in order.
    pub sequence: Vec<BasePass>,
}

/// A pass that iterates an internal pass until no further change.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepeatPass {
    /// The body of the loop, to be iterated until no further change.
    pub body: Box<BasePass>,
}

/// A pass that iterates an internal pass whilst some metric decreases.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepeatWithMetricPass {
    /// The body of the loop.
    pub body: Box<BasePass>,
    /// The metric that conditions the loop,
    /// stored as a dill string of the python function.
    pub metric: String,
}

/// A pass that iterates an internal pass until some predicate is satisfied.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepeatUntilSatisfiedPass {
    /// The body of the loop.
    pub body: Box<BasePass>,
    /// The predicate that conditions the loop.
    /// The loop is terminated when this predicate returns True.
    pub predicate: Predicate,
}
