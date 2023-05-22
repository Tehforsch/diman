use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    *,
};

use crate::types::{Prefixes, Symbol, Factor, DimensionInt, DimensionEntry, DimensionsEntry, QuantityEntry, Defs, UnitsEntry, UnitEntry};

pub enum UnitDefinitionEntry {
    Name(Ident),
    Factor(Factor),
    Symbol(Symbol),
    Prefixes(Prefixes),
}

pub enum QuantityDefinitionEntry {
    Dimensions(DimensionsEntry),
    Units(UnitsEntry),
}


impl Parse for Prefixes {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _: token::Bracket = bracketed!(content in input);
        let prefixes: Punctuated<Ident, Token![,]> = content.parse_terminated(Ident::parse)?;
        Ok(Prefixes {
            prefixes: prefixes.into_iter().collect(),
        })
    }
}

impl Parse for Symbol {
    fn parse(input: ParseStream) -> Result<Self> {
        let lit: Lit = input.parse()?;
        let symbol = match lit {
            Lit::Str(x) => Ok(x.value()),
            _ => Err(Error::new(input.span(), "Expected string literal.")),
        }?;
        Ok(Self { symbol })
    }
}

impl Parse for Factor {
    fn parse(input: ParseStream) -> Result<Self> {
        let lit: Lit = input.parse()?;
        let factor = match lit {
            Lit::Int(x) => x.base10_parse(),

            Lit::Float(x) => x.base10_parse(),
            _ => Err(Error::new(input.span(), "Expected float literal.")),
        }?;
        Ok(Self { factor })
    }
}

impl Parse for DimensionInt {
    fn parse(input: ParseStream) -> Result<Self> {
        let lit: Lit = input.parse()?;
        let val = match lit {
            Lit::Int(x) => x.base10_parse(),

            _ => Err(Error::new(input.span(), "Expected int literal.")),
        }?;
        Ok(Self { val })
    }
}

impl Parse for UnitDefinitionEntry {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let _: Token![:] = input.parse()?;
        match name.to_string().as_str() {
            "name" => Ok(Self::Name(input.parse()?)),
            "factor" => Ok(Self::Factor(input.parse()?)),
            "symbol" => Ok(Self::Symbol(input.parse()?)),
            "prefixes" => Ok(Self::Prefixes(input.parse()?)),
            ident => Err(Error::new(
                ident.span(),
                format!("Unexpected identifier: {}", ident),
            )),
        }
    }
}

impl Parse for DimensionEntry {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident: Ident = input.parse()?;
        let _: Token![:] = input.parse()?;
        let value: DimensionInt = input.parse()?;
        Ok(Self { ident, value })
    }
}

fn set_option_and_throw_error_if_is_some<T>(
    item: &mut Option<T>,
    new_item: T,
    name: &str,
    span: Span,
) -> Result<()> {
    if item.is_some() {
        Err(Error::new(
            span,
            format!("More than one definition of {} given.", name),
        ))
    } else {
        *item = Some(new_item);
        Ok(())
    }
}

impl Parse for UnitEntry {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut name: Option<Ident> = None;
        let mut factor: Option<Factor> = None;
        let mut symbol: Option<Symbol> = None;
        let mut prefixes: Option<Prefixes> = None;
        let content;
        let _: token::Brace = braced!(content in input);
        let entries: Punctuated<UnitDefinitionEntry, Token![,]> =
            content.parse_terminated(UnitDefinitionEntry::parse)?;
        for entry in entries.into_iter() {
            match entry {
                UnitDefinitionEntry::Name(def_name) => set_option_and_throw_error_if_is_some(
                    &mut name,
                    def_name,
                    "name",
                    input.span(),
                )?,
                UnitDefinitionEntry::Factor(def_factor) => set_option_and_throw_error_if_is_some(
                    &mut factor,
                    def_factor,
                    "factor",
                    input.span(),
                )?,
                UnitDefinitionEntry::Symbol(def_symbol) => set_option_and_throw_error_if_is_some(
                    &mut symbol,
                    def_symbol,
                    "symbol",
                    input.span(),
                )?,
                UnitDefinitionEntry::Prefixes(def_prefixes) => {
                    set_option_and_throw_error_if_is_some(
                        &mut prefixes,
                        def_prefixes,
                        "prefixes",
                        input.span(),
                    )?
                }
            }
        }
        let name = name.unwrap();
        let factor = factor.unwrap().factor;
        let symbol = symbol.map(|symbol| symbol.symbol);
        let prefixes = prefixes.map(|prefixes| prefixes.prefixes).unwrap_or(vec![]);
        Ok(Self {
            name,
            factor,
            symbol,
            prefixes,
        })
    }
}

impl Parse for DimensionsEntry {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _: token::Brace = braced!(content in input);
        let fields: Punctuated<DimensionEntry, Token![,]> =
            content.parse_terminated(DimensionEntry::parse)?;
        Ok(Self { fields: fields.into_iter().collect() })
    }
}

impl Parse for UnitsEntry {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _: token::Bracket = bracketed!(content in input);
        let units: Punctuated<UnitEntry, Token![,]> = content.parse_terminated(UnitEntry::parse)?;
        Ok(UnitsEntry {
            units: units.into_iter().collect(),
        })
    }
}

impl UnitsEntry {
    fn empty() -> Self {
        Self { units: vec![] }
    }
}

fn get_unique_entry<T>(
    mut defs: Vec<QuantityDefinitionEntry>,
    span: proc_macro2::Span,
    name: &str,
    unwrap: fn(QuantityDefinitionEntry) -> T,
    allow_none: bool,
) -> Result<Option<T>> {
    if defs.len() == 0 {
        if allow_none {
            return Ok(None);
        }
        return Err(Error::new(
            span,
            format!("No definition of {} given for quantity.", name),
        ));
    }
    if defs.len() > 1 {
        return Err(Error::new(
            span,
            format!("More than one definition of {} given for quantity.", name),
        ));
    }
    Ok(Some(unwrap(defs.remove(0))))
}


impl QuantityEntry {
    pub fn parse_named(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let _: token::Eq = input.parse()?;
        let content;
        let _: token::Brace = braced!(content in input);
        let quantities: Punctuated<_, Token![,]> =
            content.parse_terminated(QuantityDefinitionEntry::parse_named)?;
        let (dimensions_def, units_def): (Vec<_>, Vec<_>) = quantities
            .into_iter()
            .partition(|entry| matches!(entry, QuantityDefinitionEntry::Dimensions(..)));
        let dimensions_def = get_unique_entry(
            dimensions_def,
            input.span(),
            "dimensions",
            QuantityDefinitionEntry::unwrap_dimensions,
            false,
        )?
        .unwrap();
        let units_def = get_unique_entry(
            units_def,
            input.span(),
            "units",
            QuantityDefinitionEntry::unwrap_units,
            true,
        )?
        .unwrap_or(UnitsEntry::empty());
        Ok(Self {
            name,
            dimensions_def,
            units_def,
        })
    }
}

impl QuantityDefinitionEntry {
    fn parse_named(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let _: Token![:] = input.parse()?;
        match name.to_string().as_str() {
            "dimension" => Ok(Self::Dimensions(input.parse()?)),
            "units" => Ok(Self::Units(input.parse()?)),
            ident => Err(Error::new(
                ident.span(),
                format!("Unexpected identifier: {}", ident),
            )),
        }
    }

    fn unwrap_dimensions(self) -> DimensionsEntry {
        match self {
            QuantityDefinitionEntry::Dimensions(dims) => dims,
            QuantityDefinitionEntry::Units(_) => {
                panic!("unwrap_dimensions called on Units variant")
            }
        }
    }

    fn unwrap_units(self) -> UnitsEntry {
        match self {
            QuantityDefinitionEntry::Units(units) => units,
            QuantityDefinitionEntry::Dimensions(_) => {
                panic!("unwrap_units called on Dimensions variant")
            }
        }
    }
}


impl Parse for Defs {
    fn parse(input: ParseStream) -> Result<Self> {
        let unit_names_type: Type = input.parse()?;
        let _: Token![,] = input.parse()?;
        let dimension_type: Type = input.parse()?;
        let _: Token![,] = input.parse()?;
        let quantity_type: Type = input.parse()?;
        let _: Token![,] = input.parse()?;
        let content;
        let _: token::Bracket = bracketed!(content in input);
        let quantities: Punctuated<_, Token![,]> =
            content.parse_terminated(QuantityEntry::parse_named)?;
        Ok(Self {
            unit_names_type,
            dimension_type,
            quantity_type,
            quantities: quantities.into_iter().collect(),
        })
    }
}
