use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned};
use syn::Type;

use crate::types::Defs;

#[derive(Clone, Copy)]
enum Trait {
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

enum StorageType {
    Generic,
    Concrete(Type),
}

enum QuantityType {
    Quantity,
    Dimensionless,
    Storage,
}

enum ReferenceType {
    Value,
    Reference,
    MutableReference,
}

impl ReferenceType {
    fn is_ref(&self) -> bool {
        match self {
            ReferenceType::Value => false,
            ReferenceType::Reference => true,
            ReferenceType::MutableReference => true,
        }
    }
}

struct Operand {
    type_: QuantityType,
    storage: StorageType,
    reference: ReferenceType,
}
impl Operand {
    fn ref_sign(&self, span: proc_macro2::Span) -> TokenStream {
        match self.reference {
            ReferenceType::Value => quote_spanned! {span=>},
            ReferenceType::Reference => quote_spanned! {span=>&'a },
            ReferenceType::MutableReference => quote_spanned! {span=>&'a mut },
        }
    }

    fn is_storage(&self) -> bool {
        !matches!(
            self.type_,
            QuantityType::Quantity | QuantityType::Dimensionless
        )
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
            //
            // Not sure how much of a bug this is but I filed
            // https://github.com/rust-lang/rust/issues/119690
            quote! { #quantity_type < (), #dim >: }
        } else {
            quote! {}
        }
    }
}

struct NumericTrait {
    name: Trait,
    lhs: Operand,
    rhs: Operand,
}

impl NumericTrait {
    /// Whether the trait allows for different dimensions on the
    /// left-hand and right-hand sides. Only Mul and Div allow this,
    /// every other trait requires the same dimension on both sides
    /// (or, in the case of MulAssign/DivAssign, a dimensionless
    /// expression on the rhs)
    fn different_dimensions_allowed(&self) -> bool {
        use Trait::*;
        match self.name {
            Add | Sub | AddAssign | SubAssign | MulAssign | DivAssign | PartialEq | PartialOrd => {
                false
            }
            Mul | Div => true,
        }
    }

    /// Whether the trait allows different generic storage types on the left-hand and
    /// right-hand sides as opposed to the same generic storage types on both sides.
    /// This is finicky because relaxing the requirements too much will result in duplicate
    /// trait impls.
    fn different_storage_types_allowed(&self) -> bool {
        // This restriction could be restricted in principle, in practice however,
        // if I am too lose here, I run into duplicate trait impls
        matches!(
            self.lhs.type_,
            QuantityType::Quantity | QuantityType::Dimensionless
        ) && matches!(
            self.rhs.type_,
            QuantityType::Quantity | QuantityType::Dimensionless
        )
    }

    /// The two names of the generic dimension types on LHS and RHS
    /// respectively.  Each return value is an Option because if the
    /// operand on a side is either a storage type or a dimensionless
    /// quantity, it will not have a named dimension.
    fn dimension_types(&self) -> (Option<TokenStream>, Option<TokenStream>) {
        use QuantityType::*;
        match (&self.lhs.type_, &self.rhs.type_) {
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

    /// The two names of the storage types on LHS and RHS respectively.
    /// Each type is either a concrete storage type or a generic storage type.
    /// The two generic types can be different if the trait allows it.
    fn storage_types(&self) -> (TokenStream, TokenStream) {
        use StorageType::*;
        let different_storage_types_allowed = self.different_storage_types_allowed();
        match (&self.lhs.storage, &self.rhs.storage) {
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
        if self.lhs.reference.is_ref() {
            num_lifetimes += 1
        }
        if self.rhs.reference.is_ref() {
            num_lifetimes += 1
        }
        let mut types = vec![];
        match num_lifetimes {
            0 => {}
            1 | 2 => types.push(quote! { 'a }),
            _ => unreachable!(),
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
        if matches!(self.lhs.storage, StorageType::Generic) {
            types.push(lhs_storage);
        }
        // Make sure we don't declare the storage type twice if it is the same
        if matches!(self.rhs.storage, StorageType::Generic)
            && (!matches!(self.lhs.storage, StorageType::Generic)
                || self.different_storage_types_allowed())
        {
            types.push(rhs_storage);
        }
        types
    }

    /// Generates all the generics, i.e. the <...> in `impl<...> Trait for`.
    /// Adds the appropriate amount of required lifetimes, const generics and
    /// generic storage types for the impl.
    fn generics_gen(&self, dimension_type: &Ident) -> TokenStream {
        let types = self.generics(dimension_type);
        quote! {
            < #(#types),* >
        }
    }

    /// Returns the expression for the left-hand side type
    fn lhs_type(&self, quantity_type: &Ident, dimension_type: &Ident) -> TokenStream {
        let storage = self.storage_types().0;
        let dimension = self.dimension_types().0;
        self.type_for_operand(&self.lhs, quantity_type, dimension_type, storage, dimension)
    }

    /// Returns the expression for the right-hand side type
    fn rhs_type(&self, quantity_type: &Ident, dimension_type: &Ident) -> TokenStream {
        let storage = self.storage_types().1;
        let dimension = self.dimension_types().1;
        self.type_for_operand(&self.rhs, quantity_type, dimension_type, storage, dimension)
    }

    fn type_for_operand(
        &self,
        operand: &Operand,
        quantity_type: &Ident,
        dimension_type: &Ident,
        storage: TokenStream,
        dimension: Option<TokenStream>,
    ) -> TokenStream {
        let span = dimension_type.span();
        let ref_sign = operand.ref_sign(span);
        let type_name = match operand.type_ {
            QuantityType::Quantity => {
                quote_spanned! {span=> #quantity_type < #storage, #dimension > }
            }
            QuantityType::Dimensionless => {
                quote_spanned! {span=>#quantity_type < #storage, { #dimension_type :: none() } >}
            }
            QuantityType::Storage => quote_spanned! {span=>#storage},
        };
        quote_spanned! {span=>#ref_sign #type_name}
    }

    /// Generates the trait bounds for a concrete implementation.
    /// This consists of
    /// 1. A trait bound for the same trait but for the underlying storage types
    /// 2. If necessary, a `Copy` trait bound for the LHS/RHS storage type, if we only
    ///    receive a &Quantity on the LHS/RHS respectively.
    /// 3. If necessary, a bound on the const generic expression for
    /// mul/div-type traits, where a new dimension is created.
    fn trait_bounds(
        &self,
        quantity_type: &Ident,
        output_type: &Option<OutputQuantity>,
    ) -> TokenStream {
        if matches!(self.lhs.storage, StorageType::Generic)
            || matches!(self.rhs.storage, StorageType::Generic)
        {
            let (lhs_storage, rhs_storage) = self.storage_types();
            let trait_name = self.name.name();
            let output_bound = quote! {};
            let lhs_copy_bound = if self.lhs.reference.is_ref() {
                quote! { #lhs_storage: Copy, }
            } else {
                quote! {}
            };
            let rhs_copy_bound = if self.rhs.reference.is_ref() {
                quote! { #rhs_storage: Copy, }
            } else {
                quote! {}
            };
            let generic_const_bound = output_type
                .as_ref()
                .map(|output_type| output_type.generic_const_bound(&quantity_type))
                .unwrap_or(quote! {});
            quote! {
                #lhs_storage: #trait_name :: < #rhs_storage, #output_bound >,
                #lhs_copy_bound
                #rhs_copy_bound
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
        quote! { < #lhs as #trait_name<#rhs> >::Output }
    }

    fn output_quantity_dimension(&self, dimension_type: &Ident) -> OutputQuantityDimension {
        assert!(self.name.has_output_type());
        let span = dimension_type.span();
        use OutputQuantityDimension::*;
        use QuantityType::*;
        let existing = Existing(quote_spanned! { span=> D });
        match (&self.lhs.type_, &self.rhs.type_) {
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
            (Dimensionless, Storage) | (Storage, Dimensionless) => {
                New(quote_spanned! {span=> { #dimension_type :: none() } })
            }
            _ => unreachable!(),
        }
    }

    /// A representation of the output type of the trait function.
    /// If an output type exists (for Add, Sub, Mul and Div), it is
    /// always a quantity and is defined by its storage type and its dimension.
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

    /// Whether the trait function takes its argument by reference.
    fn rhs_takes_ref(&self) -> bool {
        matches!(self.name, PartialOrd | PartialEq)
    }

    /// The returned expression from the trait function.
    /// If an output type exists, the returned object is
    /// a quantity, so we wrap the underlying storage-type-level
    /// expression in Quantity(...).
    fn fn_return_expr(
        &self,
        quantity_type: &Ident,
        output_type: &Option<OutputQuantity>,
    ) -> TokenStream {
        let lhs = match self.lhs.type_ {
            QuantityType::Quantity | QuantityType::Dimensionless => quote! { self.0 },
            QuantityType::Storage => quote! { self },
        };
        let rhs = match self.rhs.type_ {
            QuantityType::Quantity | QuantityType::Dimensionless => quote! { rhs.0 },
            QuantityType::Storage => quote! { rhs },
        };
        let fn_name = self.name.fn_name();
        let deref_or_ref = if self.rhs_takes_ref() {
            quote! { & }
        } else {
            if self.rhs.reference.is_ref() && self.rhs.is_storage() {
                quote! {*}
            } else {
                quote! {}
            }
        };
        let result = quote! { #lhs.#fn_name(#deref_or_ref #rhs) };
        if output_type.is_some() {
            quote! { #quantity_type ( #result ) }
        } else {
            result
        }
    }
}

macro_rules! def_operand {
    (&mut $quantity: ident, $storage: expr) => {
        Operand {
            reference: ReferenceType::MutableReference,
            type_: QuantityType::$quantity,
            storage: $storage,
        }
    };
    (& $quantity: ident, $storage: expr) => {
        Operand {
            reference: ReferenceType::Reference,
            type_: QuantityType::$quantity,
            storage: $storage,
        }
    };
    ($quantity: ident, $storage: expr) => {
        Operand {
            reference: ReferenceType::Value,
            type_: QuantityType::$quantity,
            storage: $storage,
        }
    };
}

macro_rules! add_trait {
    (
        $traits: ident,
        $name: path,
     ($($lhs:tt)*), ($($rhs:tt)*)) => {
        $traits.push(NumericTrait {
            name: $name,
            lhs: def_operand!($($lhs)*),
            rhs: def_operand!($($rhs)*),
        })
    }
}

impl Defs {
    #[rustfmt::skip]
    fn iter_numeric_traits(&self) -> impl Iterator<Item = NumericTrait> + '_ {
        let mut traits = vec![];
        use StorageType::*;
        for t in [Add, Sub, Mul, Div] {
            add_trait!(traits, t, (Quantity, Generic), (Quantity, Generic));
            add_trait!(traits, t, (Quantity, Generic), (&Quantity, Generic));
            add_trait!(traits, t, (&Quantity, Generic), (Quantity, Generic));
            add_trait!(traits, t, (&Quantity, Generic), (&Quantity, Generic));
        }
        for t in [AddAssign, SubAssign] {
            add_trait!(traits, t, (Quantity, Generic), (Quantity, Generic));
            add_trait!(traits, t, (Quantity, Generic), (&Quantity, Generic));
            add_trait!(traits, t, (&mut Quantity, Generic), (Quantity, Generic));
            add_trait!(traits, t, (&mut Quantity, Generic), (&Quantity, Generic));
        }
        for t in [MulAssign, DivAssign] {
            add_trait!(traits, t, (Quantity, Generic), (Dimensionless, Generic));
            add_trait!(traits, t, (Quantity, Generic), (&Dimensionless, Generic));
            add_trait!(traits, t, (&mut Quantity, Generic), (Dimensionless, Generic));
            add_trait!(traits, t, (&mut Quantity, Generic), (&Dimensionless, Generic));
        }
        for t in [PartialOrd, PartialEq] {
            add_trait!(traits, t, (Quantity, Generic), (Quantity, Generic));
            add_trait!(traits, t, (Quantity, Generic), (&Quantity, Generic));
            add_trait!(traits, t, (&Quantity, Generic), (Quantity, Generic));
            // Note that core already implements PartialEq(/Ord) for &A <-> &B
            // for all A and B that implement it automatically, so we add no
            // &Quantity / &Quantity impl here.
        }
        for t in [Add, Sub] {
            add_trait!(traits, t, (Dimensionless, Generic), (Storage, Generic));
            add_trait!(traits, t, (Dimensionless, Generic), (&Storage, Generic));
            add_trait!(traits, t, (&Dimensionless, Generic), (Storage, Generic));
            add_trait!(traits, t, (&Dimensionless, Generic), (&Storage, Generic));
        }
        for t in [AddAssign, SubAssign] {
            add_trait!(traits, t, (Dimensionless, Generic), (Storage, Generic));
            add_trait!(traits, t, (Dimensionless, Generic), (&Storage, Generic));
            add_trait!(traits, t, (&mut Dimensionless, Generic), (Storage, Generic));
            add_trait!(traits, t, (&mut Dimensionless, Generic), (&Storage, Generic));
        }
        for ty in self.storage_type_names() {
            for t in [Add, Sub] {
                add_trait!(traits, t, (Storage, Concrete(ty.clone())), (Dimensionless, Concrete(ty.clone())));
                add_trait!(traits, t, (Storage, Concrete(ty.clone())), (&Dimensionless, Concrete(ty.clone())));
                add_trait!(traits, t, (&Storage, Concrete(ty.clone())), (Dimensionless, Concrete(ty.clone())));
                add_trait!(traits, t, (&Storage, Concrete(ty.clone())), (&Dimensionless, Concrete(ty.clone())));
            }
            for t in [AddAssign, SubAssign] {
                add_trait!(traits, t, (Storage, Concrete(ty.clone())), (Dimensionless, Concrete(ty.clone())));
                add_trait!(traits, t, (Storage, Concrete(ty.clone())), (&Dimensionless, Concrete(ty.clone())));
                // Primitive storage types like f32 dont implement &mut f32: AddAssign<f32>, so
                // we won't either.
            }
            for t in [Mul, Div] {
                add_trait!(traits, t, (Quantity, Concrete(ty.clone())), (Storage, Concrete(ty.clone())));
                add_trait!(traits, t, (&Quantity, Concrete(ty.clone())), (Storage, Concrete(ty.clone())));
                add_trait!(traits, t, (Quantity, Concrete(ty.clone())), (&Storage, Concrete(ty.clone())));
                add_trait!(traits, t, (&Quantity, Concrete(ty.clone())), (&Storage, Concrete(ty.clone())));
                add_trait!(traits, t, (Storage, Concrete(ty.clone())), (Quantity, Generic));
            }
            for t in [MulAssign, DivAssign] {
                add_trait!(traits, t, (Quantity, Generic), (Storage, Concrete(ty.clone())));
                add_trait!(traits, t, (Storage, Concrete(ty.clone())), (Quantity, Generic));
            }
            for t in [PartialEq, PartialOrd] {
                add_trait!(traits, t, (Dimensionless, Generic), (Storage, Concrete(ty.clone())));
                add_trait!(traits, t, (Storage, Concrete(ty.clone())), (Dimensionless, Generic));
            }
        }
        traits.into_iter()
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

    fn generic_numeric_trait_impl(&self, numeric_trait: NumericTrait) -> TokenStream {
        let name = numeric_trait.name;
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

        let trait_bounds = numeric_trait.trait_bounds(&self.quantity_type, &output_type);
        let fn_return_expr = numeric_trait.fn_return_expr(&self.quantity_type, &output_type);
        quote! {
            impl #impl_generics #trait_name::<#rhs> for #lhs
            where
                #trait_bounds
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

    pub fn impl_numeric_traits(&self) -> TokenStream {
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
}
