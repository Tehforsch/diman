mod error;
mod resolver;

use std::collections::{HashMap, HashSet};

use proc_macro2::Span;
use syn::Ident;

use crate::{
    derive_dimension::to_snakecase,
    dimension_math::DimensionsAndFactor,
    expression::{self, Expr},
    types::{
        BaseDimensions, Constant, ConstantEntry, Defs, Dimension, DimensionDefinition,
        DimensionEntry, DimensionIdent, IntExponent, Unit, UnitDefinition, UnitEntry, UnitFactor,
        UnresolvedDefs,
    },
};

use self::{
    error::{
        Emit, MultipleDefinitionsError, MultipleTypeDefinitionsError,
        UndefinedAnnotationDimensionError, UndefinedError, ViolatedAnnotationError,
    },
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
/// result types but continuing with a partial result anyways. This is
/// done so that we at least define all the quantities that can be
/// partially resolved in order to keep the amount of error messages
/// manageable.
fn emit_errors<T, E: Emit>((input, result): (T, std::result::Result<(), E>)) -> T {
    if let Err(err) = result {
        err.emit();
    }
    input
}

impl Resolvable for DimensionEntry {
    type Dim = BaseDimensions;

    type Resolved = Dimension;

    fn expr(&self) -> Expr<Factor<Self::Dim>, IntExponent> {
        match &self.rhs {
            DimensionDefinition::Expression(expr) => expr.clone().map(|e| match e {
                DimensionIdent::One => Factor::Concrete(BaseDimensions::none()),
                DimensionIdent::Dimension(ident) => Factor::Other(ident),
            }),
            DimensionDefinition::Base => {
                let mut fields = HashMap::default();
                fields.insert(self.dimension_entry_name(), 1);
                Expr::Value(expression::Factor::Value(Factor::Concrete(
                    BaseDimensions { fields },
                )))
            }
        }
    }

    fn into_resolved(self, d: Self::Dim) -> Self::Resolved {
        Dimension {
            name: self.name,
            dimensions: d,
        }
    }
}

impl Resolvable for UnitEntry {
    type Dim = DimensionsAndFactor;

    type Resolved = Unit;

    fn expr(&self) -> Expr<Factor<Self::Dim>, IntExponent> {
        match &self.definition {
            UnitDefinition::Expression(rhs) => rhs.clone().map(|e| match e {
                UnitFactor::Unit(ident) => Factor::Other(ident),
                UnitFactor::Number(num) => Factor::Concrete(DimensionsAndFactor::factor(num)),
            }),
            UnitDefinition::Base(dimension) => {
                Expr::Value(expression::Factor::Value(Factor::Other(dimension.clone())))
            }
        }
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

impl Resolvable for ConstantEntry {
    type Dim = DimensionsAndFactor;

    type Resolved = Constant;

    fn expr(&self) -> Expr<Factor<Self::Dim>, IntExponent> {
        self.rhs.clone().map(|e| match e {
            UnitFactor::Unit(ident) => Factor::Other(ident),
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

impl Resolved<DimensionsAndFactor> for Dimension {
    fn dims(&self) -> DimensionsAndFactor {
        DimensionsAndFactor {
            factor: 1.0,
            dimensions: self.dimensions.clone(),
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

fn filter_multiply_defined_identifiers<R: Resolvable>(
    items: Vec<R>,
) -> (Vec<R>, Result<(), MultipleDefinitionsError>) {
    let mut counter: HashMap<_, usize> = items.iter().map(|item| (item.ident(), 0)).collect();
    for item in items.iter() {
        *counter.get_mut(item.ident()).unwrap() += 1;
    }
    let err = if items.iter().any(|item| counter[item.ident()] > 1) {
        Err(MultipleDefinitionsError(
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
) -> (Vec<R>, Result<(), UndefinedError>) {
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
        Err(UndefinedError(undefined_rhs))
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

fn resolve_and_check_annotation<
    R: Resolvable<Dim = DimensionsAndFactor> + Annotated + Clone,
    G: Resolved<R::Dim>,
>(
    items: Vec<R>,
    given: &[G],
    dimensions: &HashMap<Ident, BaseDimensions>,
) -> Vec<R::Resolved> {
    let given = given
        .iter()
        .map(|g| (g.ident().clone(), g.dims()))
        .collect();
    let mut resolved = emit_errors(Resolver::resolve(items.clone(), given));
    check_annotations(&items, &resolved, dimensions);
    convert_vec_to_resolved(items, &mut resolved)
}

fn check_annotations<R: Resolvable<Dim = DimensionsAndFactor> + Annotated + Clone>(
    items: &[R],
    resolved: &HashMap<Ident, <R as Resolvable>::Dim>,
    dimensions: &HashMap<Ident, BaseDimensions>,
) {
    for item in items.iter() {
        let annotation = item.get_annotation();
        if let Some(annotation) = annotation {
            let rhs_dims = &resolved[item.ident()].dimensions;
            if let Some(lhs_dims) = &dimensions.get(annotation) {
                if *lhs_dims != rhs_dims {
                    ViolatedAnnotationError {
                        annotation: annotation,
                        lhs_dims,
                        rhs_dims,
                    }
                    .emit();
                }
            } else {
                UndefinedAnnotationDimensionError(annotation).emit()
            }
        }
    }
}

fn resolve(items: Vec<DimensionEntry>) -> (Vec<Dimension>, HashMap<Ident, BaseDimensions>) {
    let resolved = emit_errors(Resolver::resolve(items.clone(), HashMap::new()));
    (
        convert_vec_to_resolved(items, &mut resolved.clone()),
        resolved,
    )
}

fn filter<R: Resolvable + Clone>(items: Vec<R>, all_known: &HashSet<Ident>) -> Vec<R> {
    let items = emit_errors(filter_undefined_identifiers(items, all_known));
    let items = emit_errors(filter_multiply_defined_identifiers(items));
    items
}

impl UnresolvedDefs {
    pub fn resolve(mut self) -> Defs {
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
        extend_names(&mut known_names, &self.units);
        extend_names(&mut known_names, &self.constants);
        self.dimensions = filter(self.dimensions, &known_names);
        self.units = filter(self.units, &known_names);
        self.constants = filter(self.constants, &known_names);
        let base_dimensions = self
            .dimensions
            .iter()
            .filter(|d| d.is_base_dimension())
            .map(|x| to_snakecase(&x.name))
            .collect();
        let (dimensions, ident_dimension_map) = resolve(self.dimensions);
        let units = resolve_and_check_annotation(self.units, &dimensions, &ident_dimension_map);
        let constants = resolve_and_check_annotation(self.constants, &units, &ident_dimension_map);
        Defs {
            dimension_type,
            quantity_type,
            dimensions,
            units,
            constants,
            base_dimensions,
        }
    }
}

fn extend_names<N: Named>(known_names: &mut HashSet<Ident>, names: &[N]) {
    known_names.extend(names.iter().map(|n| n.ident().clone()))
}

macro_rules! impl_named {
    ($t: ty) => {
        impl Named for $t {
            fn ident(&self) -> &Ident {
                &self.name
            }
        }
    };
}

impl_named!(DimensionEntry);
impl_named!(UnitEntry);
impl_named!(Unit);
impl_named!(Dimension);
impl_named!(ConstantEntry);

trait Annotated: Resolvable {
    fn get_annotation(&self) -> Option<&Ident>;
}

impl Annotated for UnitEntry {
    fn get_annotation(&self) -> Option<&Ident> {
        self.dimension_annotation.as_ref()
    }
}

impl Annotated for ConstantEntry {
    fn get_annotation(&self) -> Option<&Ident> {
        self.dimension_annotation.as_ref()
    }
}
