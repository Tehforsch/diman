use std::collections::HashMap;

use syn::Ident;

use crate::{
    expression::{Expr, MulDiv},
    types::IntExponent,
};

use super::{error::Error, error::Result};

pub enum Factor<D> {
    Concrete(D),
    Other(Ident),
}

pub trait Named {
    fn ident(&self) -> &Ident;
}

pub trait Resolvable: Named {
    type Dim: MulDiv;
    type Resolved;

    fn expr(&self) -> Expr<Factor<Self::Dim>, IntExponent>;
    fn into_resolved(self, d: Self::Dim) -> Self::Resolved;
}

pub trait Resolved<Dim: MulDiv>: Named {
    fn dims(&self) -> Dim;
}

fn is_resolvable<R: Resolvable>(r: &R, given: &HashMap<Ident, R::Dim>) -> bool {
    let expr = r.expr();
    let all_factors_concrete_or_given = expr.iter_vals().all(|val| match val {
        Factor::Concrete(_) => true,
        Factor::Other(ident) => given.contains_key(&ident),
    });
    all_factors_concrete_or_given
}

fn resolve<R: Resolvable>(r: R, given: &HashMap<Ident, R::Dim>) -> R::Dim {
    let expr = r.expr();
    expr.map(|val_or_expr| match val_or_expr {
        Factor::Concrete(factor) => factor,
        Factor::Other(ident) => given[&ident].clone(),
    })
    .eval()
}

pub struct Resolver<R: Resolvable> {
    unresolved: Vec<R>,
    resolved: HashMap<Ident, R::Dim>,
}

impl<R: Resolvable> Resolver<R> {
    pub fn resolve(
        unresolved: Vec<R>,
        given: HashMap<Ident, R::Dim>,
    ) -> (HashMap<Ident, R::Dim>, Result<()>) {
        let mut resolver = Self {
            unresolved,
            resolved: given,
        };
        let result = resolver.run();
        (resolver.resolved, result)
    }

    fn run(&mut self) -> Result<()> {
        // This is a very inefficient topological sort.
        while !self.unresolved.is_empty() {
            let next_resolvable = self
                .unresolved
                .iter()
                .enumerate()
                .find(|(_, x)| is_resolvable(*x, &self.resolved));
            if let Some((index, _)) = next_resolvable {
                let next_resolvable = self.unresolved.remove(index);
                let name = next_resolvable.ident().clone();
                let resolved = resolve(next_resolvable, &self.resolved);
                self.resolved.insert(name, resolved);
            } else {
                return Err(Error::Unresolvable(
                    self.unresolved
                        .drain(..)
                        .map(|x| x.ident().clone())
                        .collect(),
                ));
            }
        }
        Ok(())
    }
}
