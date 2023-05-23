#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum MultiplicativeExpr<T> {
    Factor(Factor<T>),
    Times(Factor<T>, Factor<T>),
    Over(Factor<T>, Factor<T>),
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Factor<T> {
    Value(T),
    ParenExpr(Box<MultiplicativeExpr<T>>),
}
