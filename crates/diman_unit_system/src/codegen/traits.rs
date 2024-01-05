use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned};
use syn::Type;

use crate::types::Defs;

// Add the default impl for the convenient update syntax on `NumericTrait`,
// this will never actually be used
#[derive(Default, Debug)]
enum Trait {
    #[default]
    Add,
    Sub,
    Mul,
    Div,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    PartialEq,
    PartialOrd,
}

use Trait::*;

impl Trait {
    fn name(&self) -> TokenStream {
        match self {
            Add => quote! { std::ops::Add },
            Sub => quote! { std::ops::Sub },
            Mul => quote! { std::ops::Mul },
            Div => quote! { std::ops::Div },
            AddAssign => quote! { std::ops::AddAssign },
            SubAssign => quote! { std::ops::SubAssign },
            MulAssign => quote! { std::ops::MulAssign },
            DivAssign => quote! { std::ops::DivAssign },
            PartialEq => quote! { std::cmp::PartialEq },
            PartialOrd => quote! { std::cmp::PartialOrd },
        }
    }

    fn fn_name(&self) -> TokenStream {
        match self {
            Add => quote! { add },
            Sub => quote! { sub },
            Mul => quote! { mul },
            Div => quote! { div },
            AddAssign => quote! { add_assign },
            SubAssign => quote! { sub_assign },
            MulAssign => quote! { mul_assign },
            DivAssign => quote! { div_assign },
            PartialEq => quote! { eq },
            PartialOrd => quote! { partial_cmp },
        }
    }

    fn fn_return_type(&self) -> TokenStream {
        match self {
            Add | Sub | Mul | Div => quote! { Self::Output },
            AddAssign | SubAssign | MulAssign | DivAssign => {
                quote! { () }
            }
            PartialEq => quote! { bool },
            PartialOrd => quote! { Option<std::cmp::Ordering> },
        }
    }

    fn lhs_arg(&self) -> TokenStream {
        match self {
            Add | Sub | Mul | Div => quote! { self },
            AddAssign | SubAssign | MulAssign | DivAssign => {
                quote! { &mut self }
            }
            PartialEq | PartialOrd => quote! { &self },
        }
    }

    fn rhs_arg_type(&self, rhs: &TokenStream) -> TokenStream {
        match self {
            Add | Sub | Mul | Div | AddAssign | SubAssign | MulAssign | DivAssign => rhs.clone(),
            PartialEq | PartialOrd => {
                let rhs = rhs.clone();
                quote! { &#rhs }
            }
        }
    }
}

#[derive(Default, Debug)]
enum StorageType {
    #[default]
    Generic,
    Concrete(Type),
}

#[derive(Default, Debug)]
enum QuantityType {
    #[default]
    Quantity,
    DimensionlessQuantity,
    Storage,
}

#[derive(Default, Debug)]
enum ReferenceType {
    #[default]
    Value,
    Reference,
}

#[derive(Default, Debug)]
struct Operand {
    type_: QuantityType,
    storage: StorageType,
    reference: ReferenceType,
}

#[derive(Default)]
struct NumericTrait {
    name: Trait,
    fn_return_expr: TokenStream,
    trait_bound_impl: TokenStream,
    output_type: Option<TokenStream>,
    lhs_type: TokenStream,
    rhs_type: TokenStream,
    lhs_operand: Operand,
    rhs_operand: Operand,
}

impl std::fmt::Debug for NumericTrait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Trait {}\n", "{")?;
        write!(f, "  name: {:?}\n", self.name)?;
        write!(f, "  fn_return_expr: {}\n", self.fn_return_expr)?;
        write!(f, "  trait_bound_impl: {}\n", self.trait_bound_impl)?;
        match &self.output_type {
            None => write!(f, "  output_type_def: None\n")?,
            Some(x) => write!(f, "  output_type_def: {}\n", x)?,
        }
        write!(f, "  lhs: {}\n", self.lhs_type)?;
        write!(f, "  rhs: {}\n", self.rhs_type)?;
        write!(f, "  lhs_operand: {:?}\n", self.lhs_operand)?;
        write!(f, "  rhs_operand: {:?}\n", self.rhs_operand)?;
        write!(f, "{}", "}")
    }
}

impl NumericTrait {
    fn different_dimensions_allowed(&self) -> bool {
        use Trait::*;
        match self.name {
            Add | Sub | AddAssign | SubAssign | MulAssign | DivAssign | PartialEq | PartialOrd => {
                false
            }
            Mul | Div => true,
        }
    }

    fn different_storage_types_allowed(&self) -> bool {
        let trait_allows = match self.name {
            Add | Sub | AddAssign | SubAssign | PartialEq | PartialOrd => false,
            Mul | Div | MulAssign | DivAssign => true,
        };
        trait_allows
            && matches!(
                self.lhs_operand.type_,
                QuantityType::Quantity | QuantityType::DimensionlessQuantity
            )
            && matches!(
                self.rhs_operand.type_,
                QuantityType::Quantity | QuantityType::DimensionlessQuantity
            )
    }

    fn dimension_types(&self) -> (Option<TokenStream>, Option<TokenStream>) {
        use QuantityType::*;
        match (&self.lhs_operand.type_, &self.rhs_operand.type_) {
            (Quantity, Quantity) => {
                if self.different_dimensions_allowed() {
                    (Some(quote! { DL }), Some(quote! { DR }))
                } else {
                    (Some(quote! { D }), Some(quote! { D }))
                }
            }
            (Quantity, _) => (Some(quote! { D }), None),
            (_, Quantity) => (None, Some(quote! { D })),
            _ => (None, None),
        }
    }

    fn storage_types(&self) -> (TokenStream, TokenStream) {
        use StorageType::*;
        let different_storage_types_allowed = self.different_storage_types_allowed();
        match (&self.lhs_operand.storage, &self.rhs_operand.storage) {
            (Generic, Generic) => {
                if different_storage_types_allowed {
                    (quote! { LHS }, quote! { RHS })
                } else {
                    (quote! { S }, quote! { S })
                }
            }
            (Concrete(ty), Generic) => (quote! {#ty}, quote! {S}),
            (Generic, Concrete(ty)) => (quote! {S}, quote! {#ty}),
            (Concrete(tyl), Concrete(tyr)) => (quote! {#tyl}, quote! {#tyr}),
        }
    }

    fn generics(&self, dimension_type: &Ident) -> Vec<TokenStream> {
        let mut num_lifetimes = 0;
        if let ReferenceType::Reference = self.lhs_operand.reference {
            num_lifetimes += 1
        }
        if let ReferenceType::Reference = self.rhs_operand.reference {
            num_lifetimes += 1
        }
        let mut types = vec![];
        match num_lifetimes {
            0 => {}
            1 => types.push(quote! { 'a }),
            _ => todo!(),
        }
        let make_dim_expr_from_name = |name| quote! { const #name: #dimension_type };
        let (lhs_dimension, rhs_dimension) = self.dimension_types();
        let has_lhs_dimension = lhs_dimension.is_some();
        types.extend(lhs_dimension.into_iter().map(make_dim_expr_from_name));
        // Make sure we don't declare the dimension type twice if it is the same
        if self.different_dimensions_allowed() || !has_lhs_dimension {
            types.extend(rhs_dimension.into_iter().map(make_dim_expr_from_name));
        }
        let (lhs_storage, rhs_storage) = self.storage_types();
        if matches!(self.lhs_operand.storage, StorageType::Generic) {
            types.push(lhs_storage);
        }
        // Make sure we don't declare the storage type twice if it is the same
        if matches!(self.rhs_operand.storage, StorageType::Generic)
            && (!matches!(self.lhs_operand.storage, StorageType::Generic)
                || self.different_storage_types_allowed())
        {
            types.push(rhs_storage);
        }
        types
    }

    fn generics_gen(&self, dimension_type: &Ident) -> TokenStream {
        let types = self.generics(dimension_type);
        quote! {
            < #(#types),* >
        }
    }

    fn rhs_type(&self, quantity_type: &Ident, dimension_type: &Ident) -> TokenStream {
        let ref_sign = match self.rhs_operand.reference {
            ReferenceType::Value => quote! {},
            ReferenceType::Reference => quote! {&'a },
        };
        let storage = self.storage_types().1;
        let dimension = self.dimension_types().1;
        let type_name = match self.rhs_operand.type_ {
            QuantityType::Quantity => quote! { #quantity_type < #storage, #dimension > },
            QuantityType::DimensionlessQuantity => {
                quote! {#quantity_type < #storage, { #dimension_type :: none() } >}
            }
            QuantityType::Storage => quote! {#storage},
        };
        quote! {#ref_sign #type_name}
    }

    fn additive_quantity_quantity_defaults(defs: &Defs) -> Self {
        let Defs { quantity_type, .. } = defs;
        Self {
            rhs_type: quote! { #quantity_type<S, D> },
            lhs_type: quote! { #quantity_type<S, D> },
            ..Default::default()
        }
    }

    fn additive_ref_quantity_quantity_defaults(defs: &Defs) -> Self {
        let Defs { quantity_type, .. } = defs;
        Self {
            rhs_type: quote! { &'a #quantity_type<S, D> },
            ..Self::additive_quantity_quantity_defaults(defs)
        }
    }

    /// For an impl of Add or Sub between two quantities
    fn add_or_sub_quantity_quantity(defs: &Defs, name: Trait, fn_return_expr: TokenStream) -> Self {
        let trait_name = name.name();
        Self {
            name,
            fn_return_expr,
            trait_bound_impl: quote! {S: #trait_name<Output = S>},
            output_type: Some(quote! { Self }),
            lhs_operand: Operand {
                type_: QuantityType::Quantity,
                storage: StorageType::Generic,
                reference: ReferenceType::Value,
            },
            rhs_operand: Operand {
                type_: QuantityType::Quantity,
                storage: StorageType::Generic,
                reference: ReferenceType::Value,
            },
            ..Self::additive_quantity_quantity_defaults(defs)
        }
    }

    /// For an impl of Add or Sub between a quantity and a reference to a quantity
    fn add_or_sub_quantity_refquantity(
        defs: &Defs,
        name: Trait,
        fn_return_expr: TokenStream,
    ) -> Self {
        let trait_name = name.name();
        Self {
            name,
            fn_return_expr,
            trait_bound_impl: quote! {S: #trait_name<&'a S, Output = S>},
            output_type: Some(quote! { Self }),
            lhs_operand: Operand {
                type_: QuantityType::Quantity,
                storage: StorageType::Generic,
                reference: ReferenceType::Value,
            },
            rhs_operand: Operand {
                type_: QuantityType::Quantity,
                storage: StorageType::Generic,
                reference: ReferenceType::Reference,
            },
            ..Self::additive_ref_quantity_quantity_defaults(defs)
        }
    }

    /// For an impl of AddAssign or SubAssign between two quantities
    fn add_or_sub_assign_quantity_quantity(
        defs: &Defs,
        name: Trait,
        fn_return_expr: TokenStream,
    ) -> Self {
        let trait_name = name.name();
        Self {
            name,
            fn_return_expr,
            output_type: None,
            trait_bound_impl: quote! {S: #trait_name<S>},
            ..Self::additive_quantity_quantity_defaults(defs)
        }
    }

    /// For an impl of AddAssign or SubAssign between a quantity and a reference to a quantity
    fn add_or_sub_assign_quantity_refquantity(
        defs: &Defs,
        name: Trait,
        fn_return_expr: TokenStream,
    ) -> Self {
        let trait_name = name.name();
        Self {
            name,
            fn_return_expr,
            output_type: None,
            trait_bound_impl: quote! {S: #trait_name<&'a S>},
            lhs_operand: Operand {
                type_: QuantityType::Quantity,
                storage: StorageType::Generic,
                reference: ReferenceType::Value,
            },
            rhs_operand: Operand {
                type_: QuantityType::Quantity,
                storage: StorageType::Generic,
                reference: ReferenceType::Reference,
            },
            ..Self::additive_ref_quantity_quantity_defaults(defs)
        }
    }

    /// For an impl of Add or Sub between a dimensionless quantity and a storage type
    fn add_or_sub_quantity_type(defs: &Defs, name: Trait, fn_return_expr: TokenStream) -> Self {
        let Defs {
            quantity_type,
            dimension_type,
            ..
        } = defs;
        Self {
            rhs_type: quote! { S },
            lhs_type: quote! { #quantity_type<S, { #dimension_type::none() }> },
            lhs_operand: Operand {
                type_: QuantityType::DimensionlessQuantity,
                storage: StorageType::Generic,
                reference: ReferenceType::Value,
            },
            rhs_operand: Operand {
                type_: QuantityType::Storage,
                storage: StorageType::Generic,
                reference: ReferenceType::Value,
            },
            ..Self::add_or_sub_quantity_quantity(defs, name, fn_return_expr)
        }
    }

    /// For an impl of AddAssign or SubAssign between a dimensionless quantity and a storage type
    fn add_or_sub_assign_quantity_type(
        defs: &Defs,
        name: Trait,
        fn_return_expr: TokenStream,
    ) -> Self {
        let Defs {
            quantity_type,
            dimension_type,
            ..
        } = defs;
        Self {
            rhs_type: quote! { S },
            lhs_type: quote! { #quantity_type<S, { #dimension_type::none() }> },
            lhs_operand: Operand {
                type_: QuantityType::DimensionlessQuantity,
                storage: StorageType::Generic,
                reference: ReferenceType::Value,
            },
            rhs_operand: Operand {
                type_: QuantityType::Storage,
                storage: StorageType::Generic,
                reference: ReferenceType::Value,
            },
            ..Self::add_or_sub_assign_quantity_quantity(defs, name, fn_return_expr)
        }
    }

    /// For an impl of Add or Sub between a storage type and a dimensionless quantity
    fn add_or_sub_type_quantity(
        defs: &Defs,
        name: Trait,
        fn_inner_return_expr: TokenStream,
        storage_type: &Type,
    ) -> Self {
        let Defs {
            quantity_type,
            dimension_type,
            ..
        } = defs;
        let span = defs.span();
        let quantity =
            quote_spanned! {span=> #quantity_type::<#storage_type, { #dimension_type::none() }> };
        let fn_return_expr = quote! { #quantity( #fn_inner_return_expr ) };
        Self {
            lhs_type: quote! { #storage_type },
            rhs_type: quantity.clone(),
            output_type: Some(quantity),
            name,
            fn_return_expr,
            trait_bound_impl: quote! {},
            lhs_operand: Operand {
                type_: QuantityType::Storage,
                storage: StorageType::Concrete(storage_type.clone()),
                reference: ReferenceType::Value,
            },
            rhs_operand: Operand {
                type_: QuantityType::DimensionlessQuantity,
                storage: StorageType::Concrete(storage_type.clone()),
                reference: ReferenceType::Value,
            },
        }
    }

    /// For an impl of AddAssign or SubAssign between a storage type and a dimensionless quantity
    fn add_or_sub_assign_type_quantity(
        defs: &Defs,
        name: Trait,
        fn_return_expr: TokenStream,
        storage_type: &Type,
    ) -> Self {
        let Defs {
            quantity_type,
            dimension_type,
            ..
        } = defs;
        let quantity = quote! { #quantity_type::<#storage_type, { #dimension_type::none() }> };
        Self {
            lhs_type: quote! { #storage_type },
            rhs_type: quantity.clone(),
            output_type: None,
            name,
            fn_return_expr,
            trait_bound_impl: quote! {},
            lhs_operand: Operand {
                type_: QuantityType::Storage,
                storage: StorageType::Concrete(storage_type.clone()),
                reference: ReferenceType::Value,
            },
            rhs_operand: Operand {
                type_: QuantityType::DimensionlessQuantity,
                storage: StorageType::Concrete(storage_type.clone()),
                reference: ReferenceType::Value,
            },
        }
    }

    /// For an impl of Mul or Div between two quantities
    fn mul_or_div_quantity_quantity(
        defs: &Defs,
        name: Trait,
        fn_return_expr: TokenStream,
        dimension_fn: TokenStream,
    ) -> Self {
        let Defs { quantity_type, .. } = defs;
        let span = defs.span();
        let lhs = quote! { #quantity_type<LHS, DL> };
        let rhs = quote! { #quantity_type<RHS, DR> };
        let trait_name = name.name();
        Self {
            name,
            fn_return_expr,
            trait_bound_impl: quote! {
                LHS: #trait_name<RHS>,
                #quantity_type<LHS, { DL.#dimension_fn(DR) }>:,
            },
            output_type: Some(quote_spanned! {
                span=>
                #quantity_type<
                    <LHS as #trait_name<RHS>>::Output,
                    { DL.#dimension_fn(DR) },
                >
            }),
            rhs_type: rhs,
            lhs_type: lhs,
            lhs_operand: Operand {
                type_: QuantityType::Quantity,
                storage: StorageType::Generic,
                reference: ReferenceType::Value,
            },
            rhs_operand: Operand {
                type_: QuantityType::Quantity,
                storage: StorageType::Generic,
                reference: ReferenceType::Value,
            },
        }
    }

    /// For an impl of Mul or Div between a quantity and a concrete storage type
    fn mul_or_div_quantity_type(
        defs: &Defs,
        name: Trait,
        fn_return_expr: TokenStream,
        storage_type: &Type,
    ) -> NumericTrait {
        let Defs { quantity_type, .. } = defs;
        let lhs = quote! { #quantity_type<S, D> };
        let rhs = quote! { #storage_type };
        let trait_name = name.name();
        Self {
            name,
            fn_return_expr,
            lhs_type: lhs,
            rhs_type: rhs.clone(),
            trait_bound_impl: quote! {
                S: #trait_name<#storage_type>,
            },
            output_type: Some(quote! {
                #quantity_type<
                    <S as #trait_name<#storage_type>>::Output,
                    D,
                >
            }),
            lhs_operand: Operand {
                type_: QuantityType::Quantity,
                storage: StorageType::Generic,
                reference: ReferenceType::Value,
            },
            rhs_operand: Operand {
                type_: QuantityType::Storage,
                storage: StorageType::Concrete(storage_type.clone()),
                reference: ReferenceType::Value,
            },
        }
    }

    /// For an impl of Mul or Div between a concrete storage type and a quantity
    fn div_type_quantity(
        defs: &Defs,
        name: Trait,
        fn_return_expr: TokenStream,
        storage_type: &Type,
    ) -> NumericTrait {
        let Defs { quantity_type, .. } = defs;
        let span = defs.span();
        let trait_name = name.name();
        Self {
            trait_bound_impl: quote! {
                #storage_type: #trait_name<S>,
                #quantity_type<#storage_type, { D.dimension_inv() }>:,
            },
            output_type: Some(quote_spanned! {span=>
                #quantity_type<
                    <#storage_type as #trait_name<S>>::Output,
                    { D.dimension_inv() },
                >
            }),
            ..Self::mul_type_quantity(defs, name, fn_return_expr, storage_type)
        }
    }

    /// For an impl of Mul or Div between a concrete storage type and a quantity
    fn mul_type_quantity(
        defs: &Defs,
        name: Trait,
        fn_return_expr: TokenStream,
        storage_type: &Type,
    ) -> NumericTrait {
        let Defs { quantity_type, .. } = defs;
        let rhs = quote! { #quantity_type<S, D> };
        let lhs = quote! { #storage_type };
        let trait_name = name.name();
        Self {
            name,
            fn_return_expr,
            lhs_type: lhs,
            rhs_type: rhs.clone(),
            trait_bound_impl: quote! {
                #storage_type: #trait_name<S>,
            },
            output_type: Some(quote! {
                #quantity_type<
                    <#storage_type as #trait_name<S>>::Output,
                    D,
                >
            }),
            lhs_operand: Operand {
                type_: QuantityType::Storage,
                storage: StorageType::Generic,
                reference: ReferenceType::Value,
            },
            rhs_operand: Operand {
                type_: QuantityType::Quantity,
                storage: StorageType::Generic,
                reference: ReferenceType::Value,
            },
        }
    }

    /// For an impl of MulAssign or DivAssign between two quantities (only for
    /// dimensionless right hand side)
    fn mul_or_div_assign_quantity_quantity(
        defs: &Defs,
        name: Trait,
        fn_return_expr: TokenStream,
    ) -> Self {
        let Defs {
            quantity_type,
            dimension_type,
            ..
        } = defs;
        let lhs = quote! { #quantity_type<LHS, D> };
        let rhs = quote! { #quantity_type<RHS, { #dimension_type::none() }> };
        let trait_name = name.name();
        Self {
            name,
            fn_return_expr,
            trait_bound_impl: quote! {
                LHS: #trait_name<RHS>,
            },
            output_type: None,
            rhs_type: rhs,
            lhs_type: lhs,
            lhs_operand: Operand {
                type_: QuantityType::Quantity,
                storage: StorageType::Generic,
                reference: ReferenceType::Value,
            },
            rhs_operand: Operand {
                type_: QuantityType::DimensionlessQuantity,
                storage: StorageType::Generic,
                reference: ReferenceType::Value,
            },
        }
    }

    /// For an impl of MulAssign or DivAssign between a quantity and a storage type
    fn mul_or_div_assign_quantity_type(
        defs: &Defs,
        name: Trait,
        fn_return_expr: TokenStream,
        rhs: &Type,
    ) -> Self {
        let Defs { quantity_type, .. } = defs;
        let lhs = quote! { #quantity_type<S, D> };
        let trait_name = name.name();
        Self {
            name,
            fn_return_expr,
            trait_bound_impl: quote! {
                S: #trait_name<#rhs>,
            },
            output_type: None,
            rhs_type: quote! { #rhs },
            lhs_type: lhs,
            lhs_operand: Operand {
                type_: QuantityType::Quantity,
                storage: StorageType::Generic,
                reference: ReferenceType::Value,
            },
            rhs_operand: Operand {
                type_: QuantityType::Storage,
                storage: StorageType::Concrete(rhs.clone()),
                reference: ReferenceType::Value,
            },
        }
    }

    /// For an impl of MulAssign or DivAssign between a quantity and a storage type
    fn mul_or_div_assign_type_quantity(
        defs: &Defs,
        name: Trait,
        fn_return_expr: TokenStream,
        lhs: &Type,
    ) -> Self {
        let Defs { quantity_type, .. } = defs;
        let rhs = quote! { #quantity_type<S, D> };
        let trait_name = name.name();
        Self {
            name,
            fn_return_expr,
            trait_bound_impl: quote! {
                #lhs: #trait_name<S>,
            },
            output_type: None,
            rhs_type: rhs,
            lhs_type: quote! { #lhs },
            lhs_operand: Operand {
                type_: QuantityType::Storage,
                storage: StorageType::Concrete(lhs.clone()),
                reference: ReferenceType::Value,
            },
            rhs_operand: Operand {
                type_: QuantityType::Quantity,
                storage: StorageType::Generic,
                reference: ReferenceType::Value,
            },
        }
    }

    fn cmp_trait_quantity_type(defs: &Defs, rhs: &Type, name: Trait) -> Self {
        let Defs {
            quantity_type,
            dimension_type,
            ..
        } = defs;
        let fn_name = name.fn_name();
        let trait_name = name.name();
        Self {
            name,
            fn_return_expr: quote! { self.0.#fn_name(rhs) },
            trait_bound_impl: quote! { S: #trait_name<#rhs> },
            output_type: None,
            rhs_type: quote! { #rhs },
            lhs_type: quote! { #quantity_type<S, {#dimension_type::none()} > },
            lhs_operand: Operand {
                type_: QuantityType::DimensionlessQuantity,
                storage: StorageType::Generic,
                reference: ReferenceType::Value,
            },
            rhs_operand: Operand {
                type_: QuantityType::Storage,
                storage: StorageType::Concrete(rhs.clone()),
                reference: ReferenceType::Value,
            },
        }
    }

    fn cmp_trait_type_quantity(defs: &Defs, lhs: &Type, name: Trait) -> Self {
        let Defs {
            quantity_type,
            dimension_type,
            ..
        } = defs;
        let rhs = quote! { #quantity_type<S, {#dimension_type::none()} > };
        let fn_name = name.fn_name();
        let trait_name = name.name();
        Self {
            name,
            fn_return_expr: quote! { self.#fn_name(&rhs.0) },
            trait_bound_impl: quote! { #lhs: #trait_name<S> },
            output_type: None,
            rhs_type: rhs,
            lhs_type: quote! { #lhs },
            lhs_operand: Operand {
                type_: QuantityType::Storage,
                storage: StorageType::Concrete(lhs.clone()),
                reference: ReferenceType::Value,
            },
            rhs_operand: Operand {
                type_: QuantityType::DimensionlessQuantity,
                storage: StorageType::Generic,
                reference: ReferenceType::Value,
            },
        }
    }
}

impl Defs {
    pub fn span(&self) -> proc_macro2::Span {
        self.dimension_type.span()
    }

    pub(crate) fn qproduct_trait(&self) -> TokenStream {
        let Self {
            quantity_type,
            dimension_type,
            ..
        } = &self;
        quote! {
            impl<S, const D: #dimension_type> diman::QProduct for #quantity_type<S, D> {
                type Output = #quantity_type<S, D>;
            }
        }
    }

    fn iter_numeric_traits(&self) -> impl Iterator<Item = NumericTrait> + '_ {
        let Self { quantity_type, .. } = self;
        vec![
            NumericTrait::add_or_sub_quantity_quantity(self, Add, quote! { Self(self.0 + rhs.0) }),
            NumericTrait::add_or_sub_quantity_quantity(self, Sub, quote! { Self(self.0 - rhs.0) }),
            NumericTrait::add_or_sub_quantity_refquantity(
                self,
                Add,
                quote! { Self(self.0 + &rhs.0) },
            ),
            NumericTrait::add_or_sub_quantity_refquantity(
                self,
                Sub,
                quote! { Self(self.0 - &rhs.0) },
            ),
            NumericTrait::add_or_sub_assign_quantity_quantity(
                self,
                AddAssign,
                quote! { self.0 += rhs.0; },
            ),
            NumericTrait::add_or_sub_assign_quantity_quantity(
                self,
                SubAssign,
                quote! { self.0 -= rhs.0; },
            ),
            NumericTrait::add_or_sub_assign_quantity_refquantity(
                self,
                AddAssign,
                quote! { self.0 += &rhs.0; },
            ),
            NumericTrait::add_or_sub_assign_quantity_refquantity(
                self,
                SubAssign,
                quote! { self.0 -= &rhs.0; },
            ),
            NumericTrait::add_or_sub_quantity_type(self, Add, quote! { Self(self.0 + rhs) }),
            NumericTrait::add_or_sub_quantity_type(self, Sub, quote! { Self(self.0 - rhs) }),
            NumericTrait::add_or_sub_assign_quantity_type(
                self,
                AddAssign,
                quote! { self.0 += rhs; },
            ),
            NumericTrait::add_or_sub_assign_quantity_type(
                self,
                SubAssign,
                quote! { self.0 -= rhs; },
            ),
            NumericTrait::mul_or_div_quantity_quantity(
                self,
                Mul,
                quote! { #quantity_type(self.0 * rhs.0) },
                quote! { dimension_mul },
            ),
            NumericTrait::mul_or_div_quantity_quantity(
                self,
                Div,
                quote! { #quantity_type(self.0 / rhs.0) },
                quote! { dimension_div },
            ),
            NumericTrait::mul_or_div_assign_quantity_quantity(
                self,
                MulAssign,
                quote! { self.0 *= rhs.0; },
            ),
            NumericTrait::mul_or_div_assign_quantity_quantity(
                self,
                DivAssign,
                quote! { self.0 /= rhs.0; },
            ),
        ]
        .into_iter()
        .chain(
            self.storage_type_names()
                .into_iter()
                .flat_map(move |storage_type| {
                    [
                        NumericTrait::mul_or_div_quantity_type(
                            self,
                            Mul,
                            quote! { #quantity_type(self.0 * rhs) },
                            &storage_type,
                        ),
                        NumericTrait::mul_or_div_quantity_type(
                            self,
                            Div,
                            quote! { #quantity_type(self.0 / rhs) },
                            &storage_type,
                        ),
                        NumericTrait::mul_or_div_assign_quantity_type(
                            self,
                            MulAssign,
                            quote! { self.0 *= rhs; },
                            &storage_type,
                        ),
                        NumericTrait::mul_or_div_assign_quantity_type(
                            self,
                            DivAssign,
                            quote! { self.0 /= rhs; },
                            &storage_type,
                        ),
                        NumericTrait::mul_or_div_assign_type_quantity(
                            self,
                            MulAssign,
                            quote! { *self *= rhs.0; },
                            &storage_type,
                        ),
                        NumericTrait::mul_or_div_assign_type_quantity(
                            self,
                            DivAssign,
                            quote! { *self /= rhs.0; },
                            &storage_type,
                        ),
                        NumericTrait::mul_type_quantity(
                            self,
                            Mul,
                            quote! { #quantity_type(self * rhs.0) },
                            &storage_type,
                        ),
                        NumericTrait::div_type_quantity(
                            self,
                            Div,
                            quote! { #quantity_type(self / rhs.0) },
                            &storage_type,
                        ),
                        NumericTrait::add_or_sub_type_quantity(
                            self,
                            Add,
                            quote! { self + rhs.0 },
                            &storage_type,
                        ),
                        NumericTrait::add_or_sub_type_quantity(
                            self,
                            Sub,
                            quote! { self - rhs.0 },
                            &storage_type,
                        ),
                        NumericTrait::add_or_sub_assign_type_quantity(
                            self,
                            AddAssign,
                            quote! { *self += rhs.0; },
                            &storage_type,
                        ),
                        NumericTrait::add_or_sub_assign_type_quantity(
                            self,
                            SubAssign,
                            quote! { *self -= rhs.0; },
                            &storage_type,
                        ),
                        NumericTrait::cmp_trait_quantity_type(self, &storage_type, PartialEq),
                        NumericTrait::cmp_trait_type_quantity(self, &storage_type, PartialEq),
                        NumericTrait::cmp_trait_quantity_type(self, &storage_type, PartialOrd),
                        NumericTrait::cmp_trait_type_quantity(self, &storage_type, PartialOrd),
                    ]
                    .into_iter()
                }),
        )
    }

    pub fn numeric_traits(&self) -> TokenStream {
        let ops: TokenStream = self
            .iter_numeric_traits()
            .map(|num_trait| self.generic_numeric_trait_impl(num_trait))
            .collect();
        let sum = self.impl_sum();
        let neg = self.impl_neg();
        let from = self.impl_from();
        quote! {
            #ops
            #sum
            #neg
            #from
        }
    }

    fn generic_numeric_trait_impl(&self, numeric_trait: NumericTrait) -> TokenStream {
        let NumericTrait {
            name,
            trait_bound_impl,
            fn_return_expr,
            output_type,
            rhs_type: rhs,
            lhs_type: lhs,
            rhs_operand: _,
            lhs_operand: _,
        } = &numeric_trait;
        let fn_name = name.fn_name();
        let trait_name = name.name();
        let fn_return_type = name.fn_return_type();
        let lhs_arg = name.lhs_arg();
        let rhs_arg = name.rhs_arg_type(rhs);
        let fn_args = quote! { #lhs_arg, rhs: #rhs_arg };
        let output_type_def = match output_type {
            Some(output_type) => quote! { type Output = #output_type; },
            None => quote! {},
        };
        let impl_generics = numeric_trait.generics_gen(&self.dimension_type);
        let rhs = numeric_trait.rhs_type(&self.quantity_type, &self.dimension_type);
        quote! {
            impl #impl_generics #trait_name::<#rhs> for #lhs
            where
                #trait_bound_impl
            {
                #output_type_def
                fn #fn_name(#fn_args) -> #fn_return_type {
                    #fn_return_expr
                }
            }
        }
    }

    fn impl_sum(&self) -> TokenStream {
        let Self {
            quantity_type,
            dimension_type,
            ..
        } = self;
        quote! {
            impl<const D: #dimension_type, S: Default + std::ops::AddAssign<S>> std::iter::Sum
                for #quantity_type<S, D>
            {
                fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                    let mut total = Self::default();
                    for item in iter {
                        total += item;
                    }
                    total
                }
            }

        }
    }

    fn impl_neg(&self) -> TokenStream {
        let Self {
            quantity_type,
            dimension_type,
            ..
        } = self;
        quote! {
            impl<const D: #dimension_type, S: std::ops::Neg<Output=S>> std::ops::Neg for #quantity_type<S, D> {
                type Output = Self;

                fn neg(self) -> Self::Output {
                    Self(-self.0)
                }
            }
        }
    }

    fn impl_from(&self) -> TokenStream {
        let Self {
            quantity_type,
            dimension_type,
            ..
        } = self;
        quote! {
            impl<S> From<S>
                for #quantity_type<S, { #dimension_type::none() }>
            {
                fn from(rhs: S) -> Self {
                    Self(rhs)
                }
            }

        }
    }
}
