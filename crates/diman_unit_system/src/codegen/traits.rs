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

    fn has_output_type(&self) -> bool {
        match self {
            Add | Sub | Mul | Div => true,
            _ => false,
        }
    }
}

#[derive(Default)]
enum StorageType {
    #[default]
    Generic,
    Concrete(Type),
}

impl std::fmt::Debug for StorageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageType::Generic => write!(f, "Generic"),
            StorageType::Concrete(ty) => {
                let s: String = quote! { #ty }.to_string();
                write!(f, "Concrete({})", s)
            }
        }
    }
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
impl Operand {
    fn ref_sign(&self) -> TokenStream {
        match self.reference {
            ReferenceType::Value => quote! {},
            ReferenceType::Reference => quote! {&'a },
        }
    }
}

enum OutputQuantityDimension {
    Existing(TokenStream),
    New(TokenStream),
}

impl OutputQuantityDimension {
    fn unwrap(&self) -> &TokenStream {
        match self {
            OutputQuantityDimension::Existing(t) => &t,
            OutputQuantityDimension::New(t) => &t,
        }
    }
}

struct OutputQuantity {
    storage: TokenStream,
    dimension: OutputQuantityDimension,
}

impl OutputQuantity {
    fn output_type_def(&self, quantity_type: &Ident) -> TokenStream {
        let OutputQuantity { storage, dimension } = self;
        let dimension = dimension.unwrap();
        let out = quote! { type Output = #quantity_type < #storage, #dimension >; };
        out
    }

    fn generic_const_bound(&self, quantity_type: &Ident) -> TokenStream {
        if let OutputQuantityDimension::New(dim) = &self.dimension {
            //
            // TODO(minor): This compiles?
            // A very weird sequence of events has led me to this
            // code. For context: this 'trait' bound is needed whenever
            // generic_const_exprs are used in return types of functions (as
            // far as I understand it it has to do with making sure the const
            // expr evaluates without panicking.). Now it seemed to me that
            // you would write `Quantity< STORAGE, DIM >:` where STORAGE is the
            // storage type of the return type and DIM is the dimension of the
            // return type.
            // This works for almost all of the trait impls, but the one for
            // concrete storage types (such as f32) run into
            // error[E0275]: overflow evaluating the requirement `f32: Mul<Quantity<_, _>>`
            //
            // It seems that this problem is somewhat similar to this one:
            // https://github.com/rust-lang/rust/issues/79807
            // I've reproduced a minimal example of this here:
            // https://gist.github.com/rust-play/df60936a9a6bc0f7c29b190545fb7d34
            // Note that this happens on stable rust and doesn't require adt_const_params
            // or generic_const_exprs.
            //
            // Now I realized that the same code had previously compiled with a different
            // trait bound for the const generic and that made me realize I could literally
            // put whatever storage type that I wanted here. I am guessing that this trait
            // bound is really only used to evaluate the const generic expression and
            // doesn't care about the storage type. Still, this seems very confusing.
            // However, since this helps with getting the code to compile, I put the
            // most innocent possible storage type here: `()`.
            // (Note that _ is not allowed)
            quote! { #quantity_type < (), #dim >: }
        } else {
            quote! {}
        }
    }
}

impl std::fmt::Debug for OutputQuantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { storage, dimension } = self;
        let dimension = dimension.unwrap();
        write!(f, "StorageType {}", "{")?;
        write!(f, "  storage: {}", quote! { #storage }.to_string())?;
        write!(f, "  dimension: {}", quote! { #dimension }.to_string())?;
        write!(f, "{}", "}")
    }
}

#[derive(Default)]
struct NumericTrait {
    name: Trait,
    fn_return_expr: TokenStream,
    lhs_operand: Operand,
    rhs_operand: Operand,
}

impl std::fmt::Debug for NumericTrait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Trait {}\n", "{")?;
        write!(f, "  name: {:?}\n", self.name)?;
        write!(f, "  fn_return_expr: {}\n", self.fn_return_expr)?;
        write!(f, "  lhs_operand: {:?}\n", self.lhs_operand)?;
        write!(f, "  rhs_operand: {:?}\n", self.rhs_operand)?;
        write!(f, "{}", "}")
    }
}

impl std::fmt::Display for NumericTrait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} ({:?} {:?} {:?}) ({:?} {:?} {:?})",
            self.name,
            self.lhs_operand.type_,
            self.lhs_operand.storage,
            self.lhs_operand.reference,
            self.rhs_operand.type_,
            self.rhs_operand.storage,
            self.rhs_operand.reference
        )
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
        // This restriction could be restricted in principle, in practice however,
        // if I am too lose here, I run into ICEs which are very hard to track down
        // and very hard to reproduce. See https://github.com/Tehforsch/diman/issues/2
        matches!(
            self.lhs_operand.type_,
            QuantityType::Quantity | QuantityType::DimensionlessQuantity
        ) && matches!(
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
        let storage = self.storage_types().1;
        let dimension = self.dimension_types().1;
        self.type_for_operand(
            &self.rhs_operand,
            quantity_type,
            dimension_type,
            storage,
            dimension,
        )
    }

    fn lhs_type(&self, quantity_type: &Ident, dimension_type: &Ident) -> TokenStream {
        let storage = self.storage_types().0;
        let dimension = self.dimension_types().0;
        self.type_for_operand(
            &self.lhs_operand,
            quantity_type,
            dimension_type,
            storage,
            dimension,
        )
    }

    fn type_for_operand(
        &self,
        operand: &Operand,
        quantity_type: &Ident,
        dimension_type: &Ident,
        storage: TokenStream,
        dimension: Option<TokenStream>,
    ) -> TokenStream {
        let ref_sign = operand.ref_sign();
        let type_name = match operand.type_ {
            QuantityType::Quantity => quote! { #quantity_type < #storage, #dimension > },
            QuantityType::DimensionlessQuantity => {
                quote! {#quantity_type < #storage, { #dimension_type :: none() } >}
            }
            QuantityType::Storage => quote! {#storage},
        };
        quote! {#ref_sign #type_name}
    }

    fn trait_bound_impl(
        &self,
        quantity_type: &Ident,
        output_type: &Option<OutputQuantity>,
    ) -> TokenStream {
        if matches!(self.lhs_operand.storage, StorageType::Generic)
            || matches!(self.rhs_operand.storage, StorageType::Generic)
        {
            let (lhs_storage, rhs_storage) = self.storage_types();
            let ref_sign = self.rhs_operand.ref_sign();
            let trait_name = self.name.name();
            let output_bound = quote! {};
            let generic_const_bound = output_type
                .as_ref()
                .map(|output_type| output_type.generic_const_bound(&quantity_type))
                .unwrap_or(quote! {});
            quote! {
                #lhs_storage: #trait_name :: < #ref_sign #rhs_storage, #output_bound >,
                #generic_const_bound
            }
        } else {
            quote! {}
        }
    }

    fn output_quantity_storage(&self) -> TokenStream {
        assert!(self.name.has_output_type());
        let trait_name = self.name.name();
        let (lhs, rhs) = self.storage_types();
        let lhs = match self.lhs_operand.reference {
            ReferenceType::Value => lhs,
            ReferenceType::Reference => quote! { &'a #lhs },
        };
        let rhs = match self.rhs_operand.reference {
            ReferenceType::Value => rhs,
            ReferenceType::Reference => quote! { &'a #rhs },
        };
        quote! { < #lhs as #trait_name<#rhs> >::Output }
    }

    fn output_quantity_dimension(&self, dimension_type: &Ident) -> OutputQuantityDimension {
        assert!(self.name.has_output_type());
        let span = dimension_type.span();
        use OutputQuantityDimension::*;
        use QuantityType::*;
        let existing = Existing(quote_spanned! { span=> D });
        match (&self.lhs_operand.type_, &self.rhs_operand.type_) {
            (Quantity, Quantity) => match self.name {
                Mul => New(quote_spanned! {span=> { DL.dimension_mul(DR) } }),
                Div => New(quote_spanned! {span=> { DL.dimension_div(DR) } }),
                _ => existing,
            },
            (Quantity, Storage) => existing,
            (Storage, Quantity) => match self.name {
                Mul => existing,
                Div => New(quote_spanned! {span=> { D.dimension_inv() } }),
                _ => unreachable!(),
            },
            (DimensionlessQuantity, Storage) | (Storage, DimensionlessQuantity) => {
                New(quote_spanned! {span=> { #dimension_type :: none() } })
            }
            _ => unreachable!(),
        }
    }

    fn output_type(&self, dimension_type: &Ident) -> Option<OutputQuantity> {
        if !self.name.has_output_type() {
            None
        } else {
            Some(OutputQuantity {
                storage: self.output_quantity_storage(),
                dimension: self.output_quantity_dimension(dimension_type),
            })
        }
    }

    /// For an impl of Add or Sub between two quantities
    fn add_or_sub_quantity_quantity(
        _defs: &Defs,
        name: Trait,
        fn_return_expr: TokenStream,
    ) -> Self {
        Self {
            name,
            fn_return_expr,
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
            ..Default::default()
        }
    }

    /// For an impl of Add or Sub between a quantity and a reference to a quantity
    fn add_or_sub_quantity_refquantity(
        _defs: &Defs,
        name: Trait,
        fn_return_expr: TokenStream,
    ) -> Self {
        Self {
            name,
            fn_return_expr,
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
            ..Default::default()
        }
    }

    /// For an impl of AddAssign or SubAssign between two quantities
    fn add_or_sub_assign_quantity_quantity(
        _defs: &Defs,
        name: Trait,
        fn_return_expr: TokenStream,
    ) -> Self {
        Self {
            name,
            fn_return_expr,
            ..Default::default()
        }
    }

    /// For an impl of AddAssign or SubAssign between a quantity and a reference to a quantity
    fn add_or_sub_assign_quantity_refquantity(
        _defs: &Defs,
        name: Trait,
        fn_return_expr: TokenStream,
    ) -> Self {
        Self {
            name,
            fn_return_expr,
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
            ..Default::default()
        }
    }

    /// For an impl of Add or Sub between a dimensionless quantity and a storage type
    fn add_or_sub_quantity_type(defs: &Defs, name: Trait, fn_return_expr: TokenStream) -> Self {
        Self {
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
        Self {
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
            name,
            fn_return_expr,
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
        _defs: &Defs,
        name: Trait,
        fn_return_expr: TokenStream,
        storage_type: &Type,
    ) -> Self {
        Self {
            name,
            fn_return_expr,
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
        _defs: &Defs,
        name: Trait,
        fn_return_expr: TokenStream,
    ) -> Self {
        Self {
            name,
            fn_return_expr,
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
        _defs: &Defs,
        name: Trait,
        fn_return_expr: TokenStream,
        storage_type: &Type,
    ) -> NumericTrait {
        Self {
            name,
            fn_return_expr,
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
    fn mul_type_quantity(
        _defs: &Defs,
        name: Trait,
        fn_return_expr: TokenStream,
        storage_type: &Type,
    ) -> NumericTrait {
        Self {
            name,
            fn_return_expr,
            lhs_operand: Operand {
                type_: QuantityType::Storage,
                storage: StorageType::Concrete(storage_type.clone()),
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
        _defs: &Defs,
        name: Trait,
        fn_return_expr: TokenStream,
    ) -> Self {
        Self {
            name,
            fn_return_expr,
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
        _defs: &Defs,
        name: Trait,
        fn_return_expr: TokenStream,
        rhs: &Type,
    ) -> Self {
        Self {
            name,
            fn_return_expr,
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
        _defs: &Defs,
        name: Trait,
        fn_return_expr: TokenStream,
        lhs: &Type,
    ) -> Self {
        Self {
            name,
            fn_return_expr,
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

    fn cmp_trait_quantity_type(_defs: &Defs, rhs: &Type, name: Trait) -> Self {
        let fn_name = name.fn_name();
        Self {
            name,
            fn_return_expr: quote! { self.0.#fn_name(rhs) },
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

    fn cmp_trait_type_quantity(_defs: &Defs, lhs: &Type, name: Trait) -> Self {
        let fn_name = name.fn_name();
        Self {
            name,
            fn_return_expr: quote! { self.#fn_name(&rhs.0) },
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
            NumericTrait::add_or_sub_quantity_quantity(
                self,
                Add,
                quote! { Quantity(self.0 + rhs.0) },
            ),
            NumericTrait::add_or_sub_quantity_quantity(
                self,
                Sub,
                quote! { Quantity(self.0 - rhs.0) },
            ),
            NumericTrait::add_or_sub_quantity_refquantity(
                self,
                Add,
                quote! { Quantity(self.0 + &rhs.0) },
            ),
            NumericTrait::add_or_sub_quantity_refquantity(
                self,
                Sub,
                quote! { Quantity(self.0 - &rhs.0) },
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
            NumericTrait::add_or_sub_quantity_type(self, Add, quote! { Quantity(self.0 + rhs) }),
            NumericTrait::add_or_sub_quantity_type(self, Sub, quote! { Quantity(self.0 - rhs) }),
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
            ),
            NumericTrait::mul_or_div_quantity_quantity(
                self,
                Div,
                quote! { #quantity_type(self.0 / rhs.0) },
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
                        NumericTrait::mul_type_quantity(
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
            fn_return_expr,
            rhs_operand: _,
            lhs_operand: _,
        } = &numeric_trait;
        let fn_name = name.fn_name();
        let trait_name = name.name();
        let fn_return_type = name.fn_return_type();
        let lhs = numeric_trait.lhs_type(&self.quantity_type, &self.dimension_type);
        let rhs = numeric_trait.rhs_type(&self.quantity_type, &self.dimension_type);
        let lhs_arg = name.lhs_arg();
        let rhs_arg = name.rhs_arg_type(&rhs);
        let fn_args = quote! { #lhs_arg, rhs: #rhs_arg };
        let impl_generics = numeric_trait.generics_gen(&self.dimension_type);

        let output_type = numeric_trait.output_type(&self.dimension_type);
        let output_type_def = output_type
            .as_ref()
            .map(|output_type| output_type.output_type_def(&self.quantity_type));

        let derived_trait_bound_impl =
            numeric_trait.trait_bound_impl(&self.quantity_type, &output_type);

        let trait_bound_impl = derived_trait_bound_impl;
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
