//! sphered — the SpherePOP ASCII DSL.
//!
//! Every symbol is a distinct evaluator motion; every `<(...)>` is an
//! admissible, witnessed transition. The evaluator FOLLOWs inward and
//! RETURNs outward, so the visual nesting is the execution structure.

pub mod ast;
pub mod eval;
pub mod lexer;
pub mod parser;

pub use eval::{eval, Output};
