use std::{
    collections::{HashMap, HashSet},
};

use syn::Ident;

use crate::{
    dimension_math::DimensionsAndFactor,
    expression::Expr,
    types::{
        Constant, Defs, Dimensions, Quantity, QuantityDefinition, QuantityEntry, Unit, UnitEntry,
        UnitFactor, UnresolvedDefs,
    },
};

impl UnresolvedDefs {
    pub fn resolve(self) -> Result<Defs, Error> {
        let items: Vec<UnresolvedItem> = self
            .quantities
            .into_iter()
            .map(|q| q.into())
            .chain(self.units.into_iter().map(|u| u.into()))
            .collect();
        check_no_undefined_identifiers(&items)?;
        let items = Resolver::resolve(items)?;
        let (quantities, units): (Vec<_>, Vec<_>) = items
            .into_iter()
            .partition(|x| matches!(x.type_, Type::Quantity));
        let constants = self
            .constants
            .into_iter()
            .map(|constant| {
                // Very inefficient, but hopefully irrelevant for now
                let unit = units
                    .iter()
                    .find(|unit| unit.name == constant.unit)
                    .ok_or_else(|| Error::unresolvable(vec![constant.unit]))?;
                Ok(Constant {
                    name: constant.name,
                    dimension: unit.val.dimensions.clone(),
                    factor: unit.val.factor,
                })
            })
            .collect::<Result<_, _>>()?;
        Ok(Defs {
            dimension_type: self.dimension_type,
            quantity_type: self.quantity_type,
            quantities: quantities.into_iter().map(|i| i.into()).collect(),
            units: units.into_iter().map(|i| i.into()).collect(),
            constants,
        })
    }
}

fn check_no_undefined_identifiers(
    items: &[UnresolvedItem],
) -> Result<(), Error> {
    let lhs_idents: HashSet<Ident> = items.iter().map(|item| item.name.clone()).collect();
    let mut undefined_rhs_idents = vec![];
    for item in items.iter() {
        match &item.val {
            ValueOrExpr::Value(_) => {}
            ValueOrExpr::Expr(expr) => {
                for val in expr.iter_vals() {
                    match val {
                        IdentOrFactor::Factor(_) => {}
                        IdentOrFactor::Ident(ident) => {
                            if !lhs_idents.contains(ident) {
                                undefined_rhs_idents.push(ident.clone());
                            }
                        }
                    }
                }
            }
        }
    }
    if undefined_rhs_idents.is_empty() {
        Ok(())
    } else {
        Err(Error::undefined(undefined_rhs_idents))
    }
}

enum Type {
    Unit(Option<String>),
    Quantity,
}

enum IdentOrFactor {
    Factor(DimensionsAndFactor),
    Ident(Ident),
}

enum ValueOrExpr {
    Value(DimensionsAndFactor),
    Expr(Expr<IdentOrFactor>),
}

struct UnresolvedItem {
    type_: Type,
    name: Ident,
    val: ValueOrExpr,
}

struct ResolvedItem {
    type_: Type,
    name: Ident,
    val: DimensionsAndFactor,
}

impl From<QuantityEntry> for UnresolvedItem {
    fn from(q: QuantityEntry) -> Self {
        let val = match q.rhs {
            QuantityDefinition::Dimensions(dimensions) => ValueOrExpr::Value(DimensionsAndFactor {
                dimensions,
                factor: 1.0,
            }),
            QuantityDefinition::Expression(expr) => {
                ValueOrExpr::Expr(expr.map(|x| IdentOrFactor::Ident(x)))
            }
        };
        Self {
            type_: Type::Quantity,
            name: q.name,
            val,
        }
    }
}

impl From<UnitEntry> for UnresolvedItem {
    fn from(q: UnitEntry) -> Self {
        let val = ValueOrExpr::Expr(q.rhs.map(|x| match x {
            UnitFactor::UnitOrQuantity(ident) => IdentOrFactor::Ident(ident),
            UnitFactor::Number(factor) => IdentOrFactor::Factor(DimensionsAndFactor {
                factor,
                dimensions: Dimensions::none(),
            }),
        }));
        Self {
            type_: Type::Unit(q.symbol),
            name: q.name,
            val,
        }
    }
}

impl From<ResolvedItem> for Quantity {
    fn from(q: ResolvedItem) -> Self {
        Quantity {
            name: q.name,
            dimension: q.val.dimensions,
        }
    }
}

impl From<ResolvedItem> for Unit {
    fn from(q: ResolvedItem) -> Self {
        let symbol = match q.type_ {
            Type::Unit(symbol) => symbol,
            Type::Quantity => unreachable!(),
        };
        Unit {
            name: q.name,
            dimension: q.val.dimensions,
            factor: q.val.factor,
            symbol: symbol,
        }
    }
}

trait Resolvable {
    type Resolved;

    fn resolve(self, others: &HashMap<Ident, Self::Resolved>) -> Self::Resolved;
    fn is_resolvable(&self, others: &HashMap<Ident, Self::Resolved>) -> bool;
    fn ident(&self) -> Ident;
}

impl Resolvable for UnresolvedItem {
    type Resolved = ResolvedItem;

    fn ident(&self) -> Ident {
        self.name.clone()
    }

    fn resolve(self, others: &HashMap<Ident, ResolvedItem>) -> Self::Resolved {
        match self.val {
            ValueOrExpr::Value(val) => ResolvedItem {
                type_: self.type_,
                name: self.name,
                val,
            },
            ValueOrExpr::Expr(expr) => {
                let val = expr
                    .map(|val_or_expr| match val_or_expr {
                        IdentOrFactor::Factor(factor) => factor,
                        IdentOrFactor::Ident(ident) => others[&ident].val.clone(),
                    })
                    .eval();
                ResolvedItem {
                    type_: self.type_,
                    name: self.name,
                    val,
                }
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

struct Resolver<U, R> {
    unresolved: Vec<U>,
    resolved: HashMap<Ident, R>,
}

impl<U: Resolvable<Resolved = R>, R> Resolver<U, R> {
    fn resolve_with(
        unresolved: Vec<U>,
        resolved: HashMap<Ident, R>,
    ) -> Result<Vec<R>, Error> {
        let mut resolver = Self {
            unresolved,
            resolved,
        };
        resolver.run()?;
        Ok(resolver.resolved.into_iter().map(|(_, x)| x).collect())
    }

    fn resolve(unresolved: Vec<U>) -> Result<Vec<R>, Error> {
        Self::resolve_with(unresolved, HashMap::new())
    }

    fn run(&mut self) -> Result<(), Error> {
        // This is a very inefficient topological sort.
        while !self.unresolved.is_empty() {
            let next_resolvable = self
                .unresolved
                .iter()
                .enumerate()
                .find(|(_, x)| self.resolvable(x));
            if let Some((index, _)) = next_resolvable {
                let next_resolvable = self.unresolved.remove(index);
                let name = next_resolvable.ident();
                let resolved = next_resolvable.resolve(&self.resolved);
                self.resolved.insert(name, resolved);
            } else {
                return Err(Error::unresolvable(
                    self.unresolved.drain(..).map(|x| x.ident()).collect(),
                ));
            }
        }
        Ok(())
    }

    fn resolvable(&self, u: &U) -> bool {
        u.is_resolvable(&self.resolved)
    }
}

pub struct Error {
    idents: Vec<Ident>,
    kind: ErrorKind,
}

pub enum ErrorKind{
    Unresolvable,
    Undefined,
}

impl Error {
    fn undefined(idents: Vec<Ident>) -> Self {
        Self {
            idents,
            kind: ErrorKind::Undefined
        }
    }

    fn unresolvable(idents: Vec<Ident>) -> Self {
        Self {
            idents,
            kind: ErrorKind::Unresolvable
        }
    }

    pub fn emit(self) -> () {
        let error_msg = match self.kind {
            ErrorKind::Unresolvable => "Unresolvable definition:",
            ErrorKind::Undefined => "Undefined identifier:",
        };
        let help = match self.kind {
            ErrorKind::Unresolvable => "Possible cause: recursive definitions?",
            ErrorKind::Undefined => "This identifier only appears on the right hand side.",
        };
        for ident in self.idents.iter() {
            ident
                .span()
                .unwrap()
                .error(format!("{} \"{}\"", error_msg, ident))
                .help(help)
                .emit();
        }
        // write!(
        //     f,
        //     "Unable to resolve. Unresolvable definitions: {}",
        //     self.0
        //         .iter()
        //         .map(|x| format!("\"{}\"", x.to_string()))
        //         .collect::<Vec<_>>()
        //         .join(", ")
        // )
    }
}
