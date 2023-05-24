use std::collections::{HashMap, HashSet};

use syn::Ident;

use crate::{
    dimension_math::DimensionsAndFactor,
    expression::Expr,
    types::{
        Constant, Defs, Dimensions, Quantity, QuantityDefinition, QuantityEntry, Unit, UnitEntry,
        UnitFactor, UnresolvedDefs, ConstantEntry,
    },
};

impl UnresolvedDefs {
    pub fn resolve(self) -> Result<Defs, Error> {
        let items: Vec<UnresolvedItem> = self
            .quantities
            .iter()
            .map(|q| q.to_unresolved_item())
            .chain(self.units.iter().map(|u| u.to_unresolved_item()))
            .chain(self.constants.iter().map(|u| u.to_unresolved_item()))
            .collect();
        check_no_undefined_identifiers(&items)?;
        let mut items = Resolver::resolve(items)?;
        let quantities = as_resolved(self.quantities, &mut items);
        let units = as_resolved(self.units, &mut items);
        let constants = as_resolved(self.constants, &mut items);
        Ok(Defs {
            dimension_type: self.dimension_type,
            quantity_type: self.quantity_type,
            quantities: quantities.into_iter().map(|i| i.into()).collect(),
            units: units.into_iter().map(|i| i.into()).collect(),
            constants,
        })
    }
}

fn as_resolved<T: ItemConversion>(ts: Vec<T>, items: &mut HashMap<Ident, ResolvedItem>) -> Vec<T::Resolved> {
    ts.into_iter().map(|t| {
        let item = items.remove(t.ident()).unwrap();
        t.from_resolved_item(item)
    }).collect()
}

fn check_no_undefined_identifiers(items: &[UnresolvedItem]) -> Result<(), Error> {
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

enum IdentOrFactor {
    Factor(DimensionsAndFactor),
    Ident(Ident),
}

enum ValueOrExpr {
    Value(DimensionsAndFactor),
    Expr(Expr<IdentOrFactor>),
}

struct UnresolvedItem {
    name: Ident,
    val: ValueOrExpr,
}

struct ResolvedItem {
    val: DimensionsAndFactor,
}

trait ItemConversion {
    type Resolved;

    fn to_unresolved_item(&self) -> UnresolvedItem;
    fn from_resolved_item(self, item: ResolvedItem) -> Self::Resolved;
    fn ident(&self) -> &Ident;
}

impl ItemConversion for QuantityEntry {
    type Resolved = Quantity;

    fn to_unresolved_item(&self) -> UnresolvedItem {
        let val = match &self.rhs {
            QuantityDefinition::Dimensions(dimensions) => ValueOrExpr::Value(DimensionsAndFactor {
                dimensions: dimensions.clone(),
                factor: 1.0,
            }),
            QuantityDefinition::Expression(expr) => {
                ValueOrExpr::Expr(expr.clone().map(|x| IdentOrFactor::Ident(x)))
            }
        };
        UnresolvedItem {
            name: self.name.clone(),
            val,
        }
    }

    fn from_resolved_item(self, item: ResolvedItem) -> Self::Resolved {
        Quantity {
            name: self.name,
            dimension: item.val.dimensions,
        }
    }

    fn ident(&self) -> &Ident {
        &self.name
    }
}

impl ItemConversion for UnitEntry {
    type Resolved = Unit;

    fn to_unresolved_item(&self) -> UnresolvedItem {
        let val = ValueOrExpr::Expr(self.rhs.clone().map(|x| match x {
            UnitFactor::UnitOrQuantity(ident) => IdentOrFactor::Ident(ident),
            UnitFactor::Number(factor) => IdentOrFactor::Factor(DimensionsAndFactor {
                factor,
                dimensions: Dimensions::none(),
            }),
        }));
        UnresolvedItem {
            name: self.name.clone(),
            val,
        }
    }

    fn from_resolved_item(self, item: ResolvedItem) -> Self::Resolved {
        Unit {
            name: self.name,
            dimension: item.val.dimensions,
            factor: item.val.factor,
            symbol: self.symbol,
        }
    }

    fn ident(&self) -> &Ident {
        &self.name
    }
}

impl ItemConversion for ConstantEntry {
    type Resolved = Constant;

    fn to_unresolved_item(&self) -> UnresolvedItem {
        let val = ValueOrExpr::Expr(self.rhs.clone().map(|x| match x {
            UnitFactor::UnitOrQuantity(ident) => IdentOrFactor::Ident(ident),
            UnitFactor::Number(factor) => IdentOrFactor::Factor(DimensionsAndFactor {
                factor,
                dimensions: Dimensions::none(),
            }),
        }));
        UnresolvedItem {
            name: self.name.clone(),
            val,
        }
    }

    fn from_resolved_item(self, item: ResolvedItem) -> Self::Resolved {
        Constant {
            name: self.name,
            dimension: item.val.dimensions,
            factor: item.val.factor,
        }
    }

    fn ident(&self) -> &Ident {
        &self.name
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
    fn resolve(unresolved: Vec<U>) -> Result<HashMap<Ident, R>, Error> {
        let mut resolver = Self {
            unresolved,
            resolved: HashMap::new(),
        };
        resolver.run()?;
        Ok(resolver.resolved)
    }

    fn run(&mut self) -> Result<(), Error> {
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
                return Err(Error::unresolvable(
                    self.unresolved.drain(..).map(|x| x.ident()).collect(),
                ));
            }
        }
        Ok(())
    }
}

pub struct Error {
    idents: Vec<Ident>,
    kind: ErrorKind,
}

pub enum ErrorKind {
    Unresolvable,
    Undefined,
}

impl Error {
    fn undefined(idents: Vec<Ident>) -> Self {
        Self {
            idents,
            kind: ErrorKind::Undefined,
        }
    }

    fn unresolvable(idents: Vec<Ident>) -> Self {
        Self {
            idents,
            kind: ErrorKind::Unresolvable,
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
