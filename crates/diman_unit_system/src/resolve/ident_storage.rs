use std::collections::{BTreeMap, HashMap, HashSet};

use proc_macro2::Ident;

use crate::{
    dimension_math::{BaseDimensions, DimensionsAndFactor},
    expression::{self, Expr},
    types::{
        Constant, ConstantEntry, Dimension, DimensionDefinition, DimensionEntry, DimensionFactor,
        IntExponent, Unit, UnitDefinition, UnitEntry, UnitFactor,
    },
};

use super::error::{
    Emit, KindNotAllowedError, MultipleDefinitionsError, UndefinedAnnotationDimensionError,
    UndefinedError, UnresolvableError, ViolatedAnnotationError,
};

/// The kind of an identifier
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum Kind {
    Dimension,
    Unit,
    Constant,
}

impl Kind {
    fn kind_is_allowed_in_definition(&self, kind: Kind) -> bool {
        match self {
            Kind::Dimension => kind == Kind::Dimension,
            // TODO(major): This needs to be Kind::Unit only once the syntax for
            // base units is there.
            Kind::Unit => kind == Kind::Unit || kind == Kind::Dimension || kind == Kind::Constant,
            Kind::Constant => kind == Kind::Constant || kind == Kind::Unit,
        }
    }
}

#[derive(Clone)]
pub struct Item {
    expr: Expr<Factor<DimensionsAndFactor>, IntExponent>,
    type_: ItemType,
}

#[derive(Clone)]
pub enum ItemType {
    Dimension(DimensionEntry),
    Unit(UnitEntry),
    Constant(ConstantEntry),
}

impl Item {
    fn kind(&self) -> Kind {
        match self.type_ {
            ItemType::Dimension(_) => Kind::Dimension,
            ItemType::Unit(_) => Kind::Unit,
            ItemType::Constant(_) => Kind::Constant,
        }
    }

    fn ident(&self) -> &Ident {
        match &self.type_ {
            ItemType::Dimension(dim) => &dim.name,
            ItemType::Unit(unit) => &unit.name,
            ItemType::Constant(constant) => &constant.name,
        }
    }

    fn annotation(&self) -> Option<&Ident> {
        match &self.type_ {
            ItemType::Dimension(_) => None,
            ItemType::Unit(entry) => entry.dimension_annotation.as_ref(),
            ItemType::Constant(entry) => entry.dimension_annotation.as_ref(),
        }
    }
}

impl ItemType {
    fn unwrap_dimension(self) -> DimensionEntry {
        match self {
            Self::Dimension(entry) => entry,
            _ => panic!("unwrap_dimension called on non-dimension entry"),
        }
    }

    fn unwrap_unit(self) -> UnitEntry {
        match self {
            Self::Unit(entry) => entry,
            _ => panic!("unwrap_unit called on non-unit entry"),
        }
    }

    fn unwrap_constant(self) -> ConstantEntry {
        match self {
            Self::Constant(entry) => entry,
            _ => panic!("unwrap_constant called on non-constant entry"),
        }
    }
}

pub struct ResolvedItem {
    pub item: Item,
    pub dimensions: DimensionsAndFactor,
}

#[derive(Clone)]
pub enum Factor<D> {
    Concrete(D),
    Other(Ident),
}

#[derive(Default)]
pub struct IdentStorage {
    unresolved: Vec<Item>,
    // We use BTreeMap here to make sure the generated types are deterministic
    resolved: BTreeMap<Ident, ResolvedItem>,
}

impl IdentStorage {
    fn is_resolvable(&self, item: &Item) -> bool {
        let all_factors_concrete_or_given = (&item.expr).iter_vals().all(|val| match val {
            Factor::Concrete(_) => true,
            Factor::Other(ident) => self.resolved.contains_key(&ident),
        });
        all_factors_concrete_or_given
    }

    fn resolve_dimensions(&self, item: &Item) -> DimensionsAndFactor {
        item.expr
            .clone()
            .map(|val_or_expr| match val_or_expr {
                Factor::Concrete(factor) => factor,
                Factor::Other(ident) => self.resolved[&ident].dimensions.clone(),
            })
            .eval()
    }

    pub fn add<I: Into<Item>>(&mut self, items: Vec<I>) {
        self.unresolved.extend(items.into_iter().map(|t| t.into()));
    }

    pub fn resolve(&mut self) -> Result<(), UnresolvableError> {
        // TODO(minor): This is a very inefficient topological sort.
        while !self.unresolved.is_empty() {
            let next_resolvable_index = self
                .unresolved
                .iter()
                .enumerate()
                .find(|(_, x)| self.is_resolvable(x))
                .map(|(i, _)| i);
            if let Some(index) = next_resolvable_index {
                let next_resolvable = self.unresolved.remove(index);
                let name = next_resolvable.ident().clone();
                let dimensions = self.resolve_dimensions(&next_resolvable);
                self.resolved.insert(
                    name,
                    ResolvedItem {
                        dimensions,
                        item: next_resolvable,
                    },
                );
            } else {
                return Err(UnresolvableError(
                    self.unresolved
                        .drain(..)
                        .map(|x| x.ident().clone())
                        .collect(),
                ));
            }
        }
        Ok(())
    }

    pub fn get_items<I: FromItem>(&self) -> Vec<I> {
        self.resolved
            .values()
            .filter(|resolved| resolved.item.kind() == I::kind())
            .map(|resolved| {
                I::from_item_and_dimensions(resolved.item.clone(), resolved.dimensions.clone())
            })
            .collect()
    }

    pub(crate) fn filter_undefined(&mut self) -> Result<(), UndefinedError> {
        // TODO(minor): This code clones quite a lot.
        let defined_idents: HashSet<_> = self
            .unresolved
            .iter()
            .map(|item| item.ident().clone())
            .collect();
        let mut undefined_lhs = vec![];
        let (defined, undefined): (Vec<_>, Vec<_>) = self.unresolved.drain(..).partition(|item| {
            item.expr.iter_vals().all(|x| match x {
                Factor::Concrete(_) => true,
                Factor::Other(ident) => {
                    let defined = defined_idents.contains(ident);
                    if !defined {
                        undefined_lhs.push(ident.clone());
                    }
                    defined
                }
            })
        });
        self.unresolved = defined;
        if undefined.is_empty() {
            Ok(())
        } else {
            Err(UndefinedError(undefined_lhs))
        }
    }

    pub(crate) fn filter_multiply_defined(&mut self) -> Result<(), MultipleDefinitionsError> {
        let num_definitions: HashMap<_, usize> =
            self.unresolved
                .iter()
                .fold(HashMap::default(), |mut acc, item| {
                    *acc.entry(item.ident()).or_insert(0) += 1;
                    acc
                });
        let mut v: Vec<Vec<_>> = vec![];
        for (ident, count) in num_definitions {
            if count > 1 {
                v.push(
                    self.unresolved
                        .iter()
                        .filter(|item| item.ident() == ident)
                        .map(|x| x.ident().clone())
                        .collect(),
                );
            }
        }
        if v.is_empty() {
            Ok(())
        } else {
            Err(MultipleDefinitionsError(v))
        }
    }

    pub(crate) fn check_type_annotations(&self) -> Result<(), ViolatedAnnotationError> {
        for item in self.resolved.values() {
            if let Some(annotation) = item.item.annotation() {
                match self.resolved.get(annotation) {
                    Some(annotation_dimension) => {
                        if annotation_dimension.dimensions.dimensions != item.dimensions.dimensions
                        {
                            ViolatedAnnotationError {
                                annotation,
                                annotation_dims: &annotation_dimension.dimensions.dimensions,
                                expr_dims: &item.dimensions.dimensions,
                            }
                            .emit()
                        }
                    }
                    None => UndefinedAnnotationDimensionError(annotation).emit(),
                }
            }
        }
        Ok(())
    }

    pub(crate) fn check_types(&self) {
        // TODO(minor): Having to collect into a HashMap here is annoying.
        let kinds: HashMap<_, _> = self
            .unresolved
            .iter()
            .map(|item| (item.ident(), item.kind()))
            .collect();
        for lhs in self.unresolved.iter() {
            let lhs_kind = lhs.kind();
            for rhs_factor in lhs.expr.iter_vals() {
                if let Factor::Other(rhs_ident) = rhs_factor {
                    let rhs_kind = kinds[rhs_ident];
                    if !lhs_kind.kind_is_allowed_in_definition(rhs_kind) {
                        KindNotAllowedError {
                            lhs_ident: lhs.ident(),
                            rhs_ident: rhs_ident,
                            lhs_kind,
                            rhs_kind,
                        }
                        .emit();
                    }
                }
            }
        }
    }
}

impl From<DimensionEntry> for Item {
    fn from(entry: DimensionEntry) -> Self {
        let expr = match &entry.rhs {
            DimensionDefinition::Expression(expr) => expr.clone().map(|e| match e {
                DimensionFactor::One => {
                    Factor::Concrete(DimensionsAndFactor::dimensions(BaseDimensions::none()))
                }
                DimensionFactor::Dimension(ident) => Factor::Other(ident),
            }),
            DimensionDefinition::Base => {
                let mut fields = HashMap::default();
                fields.insert(entry.dimension_entry_name(), 1);
                Expr::Value(expression::Factor::Value(Factor::Concrete(
                    DimensionsAndFactor::dimensions(BaseDimensions { fields }),
                )))
            }
        };
        Item {
            type_: ItemType::Dimension(entry),
            expr,
        }
    }
}

impl From<UnitEntry> for Item {
    fn from(entry: UnitEntry) -> Self {
        let expr = match &entry.definition {
            UnitDefinition::Expression(rhs) => rhs.clone().map(|e| match e {
                UnitFactor::Unit(ident) => Factor::Other(ident),
                UnitFactor::Number(num) => Factor::Concrete(DimensionsAndFactor::factor(num)),
            }),
            UnitDefinition::Base(dimension) => {
                Expr::Value(expression::Factor::Value(Factor::Other(dimension.clone())))
            }
        };
        Item {
            type_: ItemType::Unit(entry),
            expr,
        }
    }
}

impl From<ConstantEntry> for Item {
    fn from(entry: ConstantEntry) -> Self {
        let expr = entry.rhs.clone().map(|e| match e {
            UnitFactor::Unit(ident) => Factor::Other(ident),
            UnitFactor::Number(num) => Factor::Concrete(DimensionsAndFactor::factor(num)),
        });
        Item {
            type_: ItemType::Constant(entry),
            expr,
        }
    }
}

pub trait FromItem {
    fn kind() -> Kind;
    fn from_item_and_dimensions(item: Item, dimensions: DimensionsAndFactor) -> Self;
}

impl FromItem for Dimension {
    fn kind() -> Kind {
        Kind::Dimension
    }

    fn from_item_and_dimensions(item: Item, dimensions: DimensionsAndFactor) -> Self {
        let dimension_entry = item.type_.unwrap_dimension();
        Dimension {
            dimensions: dimensions.dimensions,
            name: dimension_entry.name,
        }
    }
}

impl FromItem for Unit {
    fn kind() -> Kind {
        Kind::Unit
    }

    fn from_item_and_dimensions(item: Item, dimensions: DimensionsAndFactor) -> Self {
        let unit_entry = item.type_.unwrap_unit();
        Unit {
            dimensions: dimensions.dimensions,
            name: unit_entry.name,
            factor: dimensions.factor,
            symbol: unit_entry.symbol,
        }
    }
}

impl FromItem for Constant {
    fn kind() -> Kind {
        Kind::Constant
    }

    fn from_item_and_dimensions(item: Item, dimensions: DimensionsAndFactor) -> Self {
        let constant_entry = item.type_.unwrap_constant();
        Constant {
            dimensions: dimensions.dimensions,
            name: constant_entry.name,
            factor: dimensions.factor,
        }
    }
}
