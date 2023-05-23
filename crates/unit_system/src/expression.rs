#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Expr<T> {
    Value(Factor<T>),
    Times(Factor<T>, Box<Expr<T>>),
    Over(Factor<T>, Box<Expr<T>>),
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Factor<T> {
    Value(T),
    ParenExpr(Box<Expr<T>>),
}
