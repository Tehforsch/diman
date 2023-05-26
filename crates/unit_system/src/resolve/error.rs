use syn::Ident;

pub struct Error {
    idents: Vec<Ident>,
    kind: ErrorKind,
}

pub type Result<T> = std::result::Result<T, Error>;

pub enum ErrorKind {
    Unresolvable,
    Undefined,
}

impl Error {
    pub fn undefined(idents: Vec<Ident>) -> Self {
        Self {
            idents,
            kind: ErrorKind::Undefined,
        }
    }

    pub fn unresolvable(idents: Vec<Ident>) -> Self {
        Self {
            idents,
            kind: ErrorKind::Unresolvable,
        }
    }

    pub fn emit(self) {
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
    }
}
