#[derive(Debug)]
pub enum MultiplicativeExpr<T> {
    Factor(Factor<T>),
    FactorTimesFactor(Factor<T>, Factor<T>),
    FactorOverFactor(Factor<T>, Factor<T>),
}

#[derive(Debug)]
pub enum Factor<T> {
    Value(T),
    ParenExpr(Box<MultiplicativeExpr<T>>)
}
