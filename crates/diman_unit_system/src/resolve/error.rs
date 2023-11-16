use proc_macro::{Diagnostic, Level};
use syn::Ident;

pub enum Error {
    Unresolvable(Vec<Ident>),
    Undefined(Vec<Ident>),
    Multiple(Vec<Vec<Ident>>),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub fn emit(self) {
        match self {
            Error::Unresolvable(idents) => emit_unresolvable(idents),
            Error::Undefined(idents) => emit_undefined(idents),
            Error::Multiple(idents) => emit_multiple(idents),
        }
    }
}

fn emit_undefined(idents: Vec<Ident>) {
    for ident in idents {
        ident
            .span()
            .unwrap()
            .error(format!("Undefined identifier \"{}\".", ident))
            .emit();
    }
}

fn emit_unresolvable(idents: Vec<Ident>) {
    for ident in idents {
        ident
            .span()
            .unwrap()
            .error(format!("Unresolvable definition \"{}\".", ident))
            .help("Remove recursive definitions.")
            .emit();
    }
}

fn emit_multiple(idents: Vec<Vec<Ident>>) {
    for idents in idents {
        let name = &idents[0];
        Diagnostic::spanned(
            idents
                .iter()
                .map(|ident| ident.span().unwrap())
                .collect::<Vec<_>>(),
            Level::Error,
            format!("Identifier \"{}\" defined multiple times.", name),
        )
        .emit();
    }
}
