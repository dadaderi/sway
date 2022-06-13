use crate::{type_engine::*, Ident};

use sway_types::span::Span;

#[derive(Debug, Clone, Eq)]
pub struct TypedFunctionParameter {
    pub name: Ident,
    pub type_id: TypeId,
    pub(crate) type_span: Span,
}

// NOTE: Hash and PartialEq must uphold the invariant:
// k1 == k2 -> hash(k1) == hash(k2)
// https://doc.rust-lang.org/std/collections/struct.HashMap.html
impl PartialEq for TypedFunctionParameter {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && look_up_type_id(self.type_id) == look_up_type_id(other.type_id)
    }
}

impl CopyTypes for TypedFunctionParameter {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        self.type_id.update_type(type_mapping, &self.type_span);
    }
}
