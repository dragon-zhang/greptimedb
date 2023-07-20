//! for declare dataflow description that is the last step before build dataflow

mod func;
mod id;
mod linear;
mod relation;

use datatypes::prelude::ConcreteDataType;
use datatypes::value::Value;
pub use id::{GlobalId, Id, LocalId};
pub use linear::MapFilterProject;
pub use relation::{AggregateExpr, TableFunc};
use serde::{Deserialize, Serialize};

use crate::storage::errors::DataflowError;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum ScalarExpr {
    /// A column of the input row
    Column(usize),
    /// A literal value.
    Literal(Result<Value, DataflowError>, ConcreteDataType),
    CallFunc {
        func: String,
        exprs: Vec<ScalarExpr>,
    },
}
