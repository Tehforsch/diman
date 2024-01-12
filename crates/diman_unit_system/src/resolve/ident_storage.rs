use std::collections::{BTreeMap, HashMap, HashSet};

use proc_macro2::Ident;

use crate::{
    dimension_math::{BaseDimensions, DimensionsAndMagnitude},
    types::{
        base_dimension::BaseDimension,
        expression::{self, Expr},
    },
    types::{
        BaseDimensionExponent, Constant, ConstantEntry, Definition, Dimension, DimensionEntry,
        Factor, Unit, UnitEntry,
    },
};

use super::error::{
    Emit, KindNotAllowedError, MultipleDefinitionsError, UndefinedAnnotationDimensionError,
    UndefinedError, UnresolvableError, ViolatedAnnotationError, WrongTypeInAnnotationError,
};

/// The kind of an identifier
#[derive(Clone, PartialEq, Copy)]
pub enum Kind {
    Dimension,
    BaseUnit,
    Unit,
    Constant,
}

impl Kind {
    fn kind_is_allowed_in_definition(&self, kind: Kind) -> bool {
        match self {
            Kind::Dimension => kind == Kind::Dimension,
            Kind::Unit => kind == Kind::Unit || kind == Kind::BaseUnit || kind == Kind::Constant,
            Kind::BaseUnit => kind == Kind::Dimension,
            Kind::Constant => {
                kind == Kind::Constant || kind == Kind::Unit || kind == Kind::BaseUnit
            }
        }
    }
}

#[derive(Clone)]
pub struct Item {
    expr: Expr<Factor<DimensionsAndMagnitude>, BaseDimensionExponent>,
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
        match &self.type_ {
            ItemType::Dimension(_) => Kind::Dimension,
            ItemType::Unit(entry) => match entry.definition {
                Definition::Base(_) => Kind::BaseUnit,
                Definition::Expression(_) => Kind::Unit,
            },
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
    pub dimensions: DimensionsAndMagnitude,
}

#[derive(Default)]
pub struct IdentStorage {
    unresolved: Vec<Item>,
    // We use BTreeMap here to make sure the generated types are deterministic
    resolved: BTreeMap<Ident, ResolvedItem>,
}

impl IdentStorage {
    fn is_resolvable(&self, item: &Item) -> bool {
        let all_factors_concrete_or_given = item.expr.iter_vals().all(|val| match val {
            Factor::Concrete(_) => true,
            Factor::Other(ident) => self.resolved.contains_key(ident),
        });
        all_factors_concrete_or_given
    }

    fn resolve_dimensions(&self, item: &Item) -> DimensionsAndMagnitude {
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

    pub fn resolve(&mut self) {
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
                UnresolvableError(
                    self.unresolved
                        .drain(..)
                        .map(|x| x.ident().clone())
                        .collect(),
                )
                .emit();
                return;
            }
        }
    }

    pub fn get_items<I: FromItem>(&self) -> Vec<I> {
        self.resolved
            .values()
            .filter(|resolved| I::is_correct_kind(resolved.item.kind()))
            .map(|resolved| {
                I::from_item_and_dimensions(resolved.item.clone(), resolved.dimensions.clone())
            })
            .collect()
    }

    fn unresolved_idents(&self) -> HashSet<Ident> {
        self.unresolved
            .iter()
            .map(|item| item.ident().clone())
            .collect()
    }

    /// Filter out all the autogenerated units that depend on a
    /// now invalid unit in order to prevent generating lots of
    /// error messages for each of the prefixed/aliased units.
    pub(crate) fn filter_autogenerated_invalid(&mut self) {
        // TODO(minor): This code clones quite a lot.
        let defined = self.unresolved_idents();
        (self.unresolved, _) = self.unresolved.drain(..).partition(|item| {
            let autogenerated_from = match &item.type_ {
                ItemType::Unit(unit) => unit.autogenerated_from.as_ref(),
                _ => None,
            };
            autogenerated_from
                .map(|ident| defined.contains(ident))
                .unwrap_or(true)
        })
    }

    pub(crate) fn filter_undefined(&mut self) {
        // TODO(minor): This code clones quite a lot.
        let defined_idents = self.unresolved_idents();
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
        if !undefined.is_empty() {
            UndefinedError(undefined_lhs).emit();
        }
    }

    pub(crate) fn filter_multiply_defined(&mut self) {
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
        if !v.is_empty() {
            MultipleDefinitionsError(v).emit();
        }
    }

    pub(crate) fn check_type_annotations(&self) {
        for item in self.resolved.values() {
            if let Some(annotation_ident) = item.item.annotation() {
                match self.resolved.get(annotation_ident) {
                    Some(annotation) => {
                        if !matches!(annotation.item.kind(), Kind::Dimension) {
                            WrongTypeInAnnotationError {
                                annotation_ident,
                                annotation_kind: annotation.item.kind(),
                            }
                            .emit()
                        } else if annotation.dimensions.dimensions != item.dimensions.dimensions {
                            ViolatedAnnotationError {
                                annotation: annotation_ident,
                                annotation_dims: &annotation.dimensions.dimensions,
                                expr_dims: &item.dimensions.dimensions,
                            }
                            .emit()
                        }
                    }
                    None => UndefinedAnnotationDimensionError(annotation_ident).emit(),
                }
            }
        }
    }

    pub(crate) fn check_kinds_in_definitions(&self) {
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
                    let rhs_kind = kinds.get(rhs_ident);
                    // If we cannot not find the rhs_ident, it means that
                    // it was previously filtered out due to be undefined / multiply defined.
                    // In that case, we'll skip this check for this identifier.
                    if let Some(rhs_kind) = rhs_kind {
                        if !lhs_kind.kind_is_allowed_in_definition(*rhs_kind) {
                            KindNotAllowedError {
                                lhs_ident: lhs.ident(),
                                rhs_ident,
                                lhs_kind,
                                rhs_kind: *rhs_kind,
                            }
                            .emit();
                        }
                    }
                }
            }
        }
    }
}

impl From<DimensionEntry> for Item {
    fn from(entry: DimensionEntry) -> Self {
        let expr = match &entry.rhs {
            Definition::Expression(expr) => expr.clone().map(|f| {
                f.map_concrete(|_| DimensionsAndMagnitude::dimensions(BaseDimensions::none()))
            }),
            Definition::Base(()) => Expr::Value(expression::Factor::Value(Factor::Concrete(
                DimensionsAndMagnitude::dimensions(BaseDimensions::for_base_dimension(
                    BaseDimension::from_dimension(&entry.name),
                )),
            ))),
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
            Definition::Expression(rhs) => rhs
                .clone()
                .map(|f| f.map_concrete(DimensionsAndMagnitude::magnitude)),
            Definition::Base(dimension) => {
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
        let expr = entry
            .rhs
            .clone()
            .map(|f| f.map_concrete(DimensionsAndMagnitude::magnitude));
        Item {
            type_: ItemType::Constant(entry),
            expr,
        }
    }
}

pub trait FromItem {
    fn from_item_and_dimensions(item: Item, dimensions: DimensionsAndMagnitude) -> Self;
    fn is_correct_kind(kind: Kind) -> bool;
}

impl FromItem for Dimension {
    fn is_correct_kind(kind: Kind) -> bool {
        kind == Kind::Dimension
    }

    fn from_item_and_dimensions(item: Item, dimensions: DimensionsAndMagnitude) -> Self {
        let dimension_entry = item.type_.unwrap_dimension();
        Dimension {
            dimensions: dimensions.dimensions,
            name: dimension_entry.name,
        }
    }
}

impl FromItem for Unit {
    fn is_correct_kind(kind: Kind) -> bool {
        kind == Kind::Unit || kind == Kind::BaseUnit
    }

    fn from_item_and_dimensions(item: Item, dimensions: DimensionsAndMagnitude) -> Self {
        let unit_entry = item.type_.unwrap_unit();
        Unit {
            dimensions: dimensions.dimensions,
            name: unit_entry.name,
            magnitude: dimensions.magnitude,
            symbol: unit_entry.symbol,
            is_base_unit: matches!(unit_entry.definition, Definition::Base(_)),
        }
    }
}

impl FromItem for Constant {
    fn is_correct_kind(kind: Kind) -> bool {
        kind == Kind::Constant
    }

    fn from_item_and_dimensions(item: Item, dimensions: DimensionsAndMagnitude) -> Self {
        let constant_entry = item.type_.unwrap_constant();
        Constant {
            dimensions: dimensions.dimensions,
            name: constant_entry.name,
            magnitude: dimensions.magnitude,
        }
    }
}
