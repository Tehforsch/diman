mod error;
mod resolver;

use std::collections::{HashMap, HashSet};

use proc_macro2::Span;
use syn::Ident;

use crate::{
    derive_dimension::to_snakecase,
    dimension_math::DimensionsAndFactor,
    types::{
        BaseDimensionEntry, BaseDimensions, Constant, ConstantEntry, Defs, Dimension,
        DimensionDefinition, DimensionEntry, DimensionIdent, Unit, UnitEntry, UnitFactor,
        UnresolvedDefs,
    },
};

use self::{
    error::{Emit, Error, MultipleTypeDefinitionsError, Result},
    resolver::{Factor, Named, Resolvable, Resolved, Resolver},
};

fn default_dimension_type() -> Ident {
    Ident::new("Dimension", Span::call_site())
}

fn default_quantity_type() -> Ident {
    Ident::new("quantity", Span::call_site())
}

fn get_single_ident(
    mut dimension_types: Vec<Ident>,
    type_name: &'static str,
    default: impl Fn() -> Ident,
) -> (Ident, std::result::Result<(), MultipleTypeDefinitionsError>) {
    if dimension_types.len() == 1 {
        (dimension_types.remove(0), Ok(()))
    } else if dimension_types.is_empty() {
        (default(), Ok(()))
    } else {
        let dimension_type = dimension_types[0].clone();
        (
            dimension_type,
            Err(MultipleTypeDefinitionsError {
                idents: dimension_types,
                type_name,
            }),
        )
    }
}

/// A helper function for emitting all the errors contained in the
/// result types but continuing with a partial result anyways This is
/// done so that we at least define all the quantities that can be
/// partially resolved in order to keep the amount of error messages
/// manageable.
fn emit_errors<T, E: Emit>((input, result): (T, std::result::Result<(), E>)) -> T {
    if let Err(err) = result {
        err.emit();
    }
    input
}

fn filter_multiply_defined_identifiers<R: Resolvable>(items: Vec<R>) -> (Vec<R>, Result<()>) {
    let mut counter: HashMap<_, usize> = items.iter().map(|item| (item.ident(), 0)).collect();
    for item in items.iter() {
        *counter.get_mut(item.ident()).unwrap() += 1;
    }
    let err = if items.iter().any(|item| counter[item.ident()] > 1) {
        Err(Error::Multiple(
            counter
                .iter()
                .filter(|(_, count)| **count > 1)
                .map(|(multiply_defined_name, _)| {
                    items
                        .iter()
                        .map(|item| item.ident().clone())
                        .filter(|name| &name == multiply_defined_name)
                        .collect()
                })
                .collect(),
        ))
    } else {
        Ok(())
    };
    (items, err)
}

fn get_rhs_idents(item: &impl Resolvable) -> Vec<Ident> {
    let mut rhs_idents = vec![];
    let expr = item.expr();
    for val in expr.iter_vals() {
        match val {
            Factor::Concrete(_) => {}
            Factor::Other(ident) => {
                rhs_idents.push(ident.clone());
            }
        }
    }
    rhs_idents
}

fn filter_undefined_identifiers<R: Resolvable>(
    items: Vec<R>,
    known: &HashSet<Ident>,
) -> (Vec<R>, Result<()>) {
    let (defined, undefined): (Vec<_>, Vec<_>) = items.into_iter().partition(|item| {
        get_rhs_idents(item)
            .iter()
            .all(|rhs_ident| known.contains(rhs_ident))
    });

    let err = if undefined.is_empty() {
        Ok(())
    } else {
        let mut undefined_rhs = vec![];
        for item in undefined {
            let rhs_idents = get_rhs_idents(&item);
            for rhs_ident in rhs_idents {
                if !known.contains(&rhs_ident) {
                    undefined_rhs.push(rhs_ident.clone());
                }
            }
        }
        Err(Error::Undefined(undefined_rhs))
    };
    (defined, err)
}

fn convert_vec_to_resolved<R: Resolvable>(
    ts: Vec<R>,
    items: &mut HashMap<Ident, R::Dim>,
) -> Vec<R::Resolved> {
    ts.into_iter()
        .filter_map(|t| {
            let item = items.remove(t.ident());
            item.map(|item| t.into_resolved(item))
        })
        .collect()
}

fn resolve<R: Resolvable + Clone>(
    items: Vec<R>,
    given: &[R::Given],
    all_known: &HashSet<Ident>,
) -> Vec<R::Resolved> {
    let items = emit_errors(filter_undefined_identifiers(items, all_known));
    let items = emit_errors(filter_multiply_defined_identifiers(items));
    let mut resolved = emit_errors(Resolver::resolve(items.clone(), given));
    convert_vec_to_resolved(items, &mut resolved)
}

impl UnresolvedDefs {
    pub fn resolve(self) -> Defs {
        let quantity_type = emit_errors(get_single_ident(
            self.quantity_types,
            "quantity type",
            default_quantity_type,
        ));
        let dimension_type = emit_errors(get_single_ident(
            self.dimension_types,
            "dimension type",
            default_dimension_type,
        ));
        let mut known_names = HashSet::default();
        extend_names(&mut known_names, &self.dimensions);
        let (base_dimensions, dimensions) = split_base_dimensions(self.dimensions);
        let base_dimensions: Vec<_> = base_dimensions
            .into_iter()
            .map(|d| BaseDimension(d))
            .collect();
        let mut dimensions = resolve(dimensions, &base_dimensions, &known_names);
        dimensions.extend(base_dimensions.iter().map(|d| Dimension {
            name: d.0.name.clone(),
            dimension: d.dims(),
        }));
        extend_names(&mut known_names, &self.units);
        let (base_units, units) = split_base_units(self.units);
        let base_units: Vec<_> = base_units
            .into_iter()
            .map(|u| BaseUnit::new(u, &dimensions))
            .collect();
        let mut units = resolve(units, &base_units, &known_names);
        units.extend(base_units.iter().map(|u| Unit {
            name: u.unit.name.clone(),
            factor: 1.0,
            dimensions: u.dimensions.clone(),
            symbol: u.unit.symbol.clone(),
        }));
        extend_names(&mut known_names, &self.constants);
        let constants = resolve(self.constants, &units, &known_names);
        Defs {
            dimension_type,
            quantity_type,
            dimensions,
            units,
            constants,
            base_dimensions: base_dimensions
                .into_iter()
                .map(|x| to_snakecase(&x.0.name))
                .collect(),
        }
    }
}

fn extend_names<N: Named>(known_names: &mut HashSet<Ident>, names: &[N]) {
    known_names.extend(names.iter().map(|n| n.ident().clone()))
}

fn split_base_dimensions(
    dimensions: Vec<DimensionEntry>,
) -> (Vec<DimensionEntry>, Vec<DimensionEntry>) {
    dimensions
        .into_iter()
        .partition(|dim| dim.is_base_dimension())
}

fn split_base_units(units: Vec<UnitEntry>) -> (Vec<UnitEntry>, Vec<UnitEntry>) {
    units.into_iter().partition(|unit| unit.is_base_unit())
}

pub struct BaseDimension(DimensionEntry);

impl Resolved<BaseDimensions> for BaseDimension {
    fn dims(&self) -> BaseDimensions {
        BaseDimensions {
            fields: vec![BaseDimensionEntry {
                ident: self.0.dimension_entry_name(),
                value: 1,
            }],
        }
    }
}

impl Named for BaseDimension {
    fn ident(&self) -> &Ident {
        &self.0.name
    }
}

impl Resolvable for DimensionEntry {
    type Dim = BaseDimensions;

    type Given = BaseDimension;

    type Resolved = Dimension;

    fn expr(&self) -> crate::expression::Expr<Factor<Self::Dim>, crate::types::IntExponent> {
        match &self.rhs {
            DimensionDefinition::BaseDimensions(_) => todo!("what"),
            DimensionDefinition::Expression(expr) => expr.clone().map(|e| match e {
                DimensionIdent::One => Factor::Concrete(BaseDimensions::none()),
                DimensionIdent::Dimension(ident) => Factor::Other(ident),
            }),
            DimensionDefinition::Base => unreachable!(), // We filtered out all base dimensions at this point
        }
    }

    fn into_resolved(self, d: Self::Dim) -> Self::Resolved {
        Dimension {
            name: self.name,
            dimension: d,
        }
    }
}

impl Named for DimensionEntry {
    fn ident(&self) -> &Ident {
        &self.name
    }
}

impl Named for UnitEntry {
    fn ident(&self) -> &Ident {
        &self.name
    }
}

pub struct BaseUnit {
    unit: UnitEntry,
    dimensions: BaseDimensions,
}

impl BaseUnit {
    fn new(unit: UnitEntry, dimensions: &[Dimension]) -> BaseUnit {
        let annotated_dimension = unit.dimension_annotation.as_ref().unwrap().to_string();
        // Compare names directly here, because the spans are different between annotation and
        // the definition of the dimension
        let dimensions = dimensions
            .iter()
            .find(|dim| dim.name.to_string() == annotated_dimension)
            .unwrap()
            .dimension
            .clone();
        Self { unit, dimensions }
    }
}

impl Resolvable for UnitEntry {
    type Dim = DimensionsAndFactor;

    type Given = BaseUnit;

    type Resolved = Unit;

    fn expr(&self) -> crate::expression::Expr<Factor<Self::Dim>, crate::types::IntExponent> {
        let expr = self.rhs.as_ref().unwrap(); // We filtered out all base units at this point
        expr.clone().map(|e| match e {
            UnitFactor::UnitOrDimension(ident) => Factor::Other(ident),
            UnitFactor::Number(num) => Factor::Concrete(DimensionsAndFactor::factor(num)),
        })
    }

    fn into_resolved(self, dim_and_factor: Self::Dim) -> Self::Resolved {
        Unit {
            name: self.name,
            dimensions: dim_and_factor.dimensions,
            symbol: self.symbol,
            factor: dim_and_factor.factor,
        }
    }
}

impl Resolved<DimensionsAndFactor> for BaseUnit {
    fn dims(&self) -> DimensionsAndFactor {
        DimensionsAndFactor {
            factor: 1.0,
            dimensions: self.dimensions.clone(),
        }
    }
}

impl Named for BaseUnit {
    fn ident(&self) -> &Ident {
        &self.unit.name
    }
}

impl Named for ConstantEntry {
    fn ident(&self) -> &Ident {
        &self.name
    }
}

impl Resolvable for ConstantEntry {
    type Dim = DimensionsAndFactor;

    type Given = Unit;

    type Resolved = Constant;

    fn expr(&self) -> crate::expression::Expr<Factor<Self::Dim>, crate::types::IntExponent> {
        self.rhs.clone().map(|e| match e {
            UnitFactor::UnitOrDimension(ident) => Factor::Other(ident),
            UnitFactor::Number(num) => Factor::Concrete(DimensionsAndFactor::factor(num)),
        })
    }

    fn into_resolved(self, dim_and_factor: Self::Dim) -> Self::Resolved {
        Constant {
            name: self.name,
            dimension: dim_and_factor.dimensions,
            factor: dim_and_factor.factor,
        }
    }
}

impl Resolved<DimensionsAndFactor> for Unit {
    fn dims(&self) -> DimensionsAndFactor {
        DimensionsAndFactor {
            factor: self.factor,
            dimensions: self.dimensions.clone(),
        }
    }
}

impl Named for Unit {
    fn ident(&self) -> &Ident {
        &self.name
    }
}
