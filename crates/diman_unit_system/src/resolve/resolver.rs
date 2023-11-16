use std::collections::HashMap;

use syn::Ident;

use super::{error::Error, error::Result};

pub trait Resolvable {
    type Resolved;

    fn resolve(self, others: &HashMap<Ident, Self::Resolved>) -> Self::Resolved;
    fn is_resolvable(&self, others: &HashMap<Ident, Self::Resolved>) -> bool;
    fn ident(&self) -> Ident;
}

pub struct Resolver<U, R> {
    unresolved: Vec<U>,
    resolved: HashMap<Ident, R>,
}

impl<U: Resolvable<Resolved = R>, R> Resolver<U, R> {
    pub fn resolve(unresolved: Vec<U>) -> (HashMap<Ident, R>, Result<()>) {
        let mut resolver = Self {
            unresolved,
            resolved: HashMap::new(),
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
                .find(|(_, x)| x.is_resolvable(&self.resolved));
            if let Some((index, _)) = next_resolvable {
                let next_resolvable = self.unresolved.remove(index);
                let name = next_resolvable.ident();
                let resolved = next_resolvable.resolve(&self.resolved);
                self.resolved.insert(name, resolved);
            } else {
                return Err(Error::Unresolvable(
                    self.unresolved.drain(..).map(|x| x.ident()).collect(),
                ));
            }
        }
        Ok(())
    }
}
