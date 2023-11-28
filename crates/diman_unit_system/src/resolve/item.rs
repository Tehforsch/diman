use std::collections::HashMap;

use syn::Ident;

use crate::{dimension_math::DimensionsAndFactor, expression::Expr, types::IntExponent};

use super::resolver::Resolvable;

pub enum IdentOrFactor {
    Factor(DimensionsAndFactor),
    Ident(Ident),
}

pub enum ValueOrExpr {
    Value(DimensionsAndFactor),
    Expr(Expr<IdentOrFactor, IntExponent>),
}

pub struct UnresolvedItem {
    pub name: Ident,
    pub val: ValueOrExpr,
}

pub struct ResolvedItem {
    pub val: DimensionsAndFactor,
}

impl Resolvable for UnresolvedItem {
    type Resolved = ResolvedItem;

    fn ident(&self) -> Ident {
        self.name.clone()
    }

    fn resolve(self, others: &HashMap<Ident, ResolvedItem>) -> Self::Resolved {
        match self.val {
            ValueOrExpr::Value(val) => ResolvedItem { val },
            ValueOrExpr::Expr(expr) => {
                let val = expr
                    .map(|val_or_expr| match val_or_expr {
                        IdentOrFactor::Factor(factor) => factor,
                        IdentOrFactor::Ident(ident) => others[&ident].val.clone(),
                    })
                    .eval();
                ResolvedItem { val }
            }
        }
    }

    fn is_resolvable(&self, others: &HashMap<Ident, ResolvedItem>) -> bool {
        match &self.val {
            ValueOrExpr::Value(_) => true,
            ValueOrExpr::Expr(expr) => expr.iter_vals().all(|val| match val {
                IdentOrFactor::Factor(_) => true,
                IdentOrFactor::Ident(ident) => others.contains_key(ident),
            }),
        }
    }
}
