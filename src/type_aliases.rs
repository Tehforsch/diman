pub trait QProduct {
    type Output;
}

pub type Product<Q1> = <Q1 as QProduct>::Output;
pub type Quotient<Q1, Q2> =
    <<Q1 as QProduct>::Output as std::ops::Div<<Q2 as QProduct>::Output>>::Output;

impl<Q1: QProduct, Q2: QProduct> QProduct for (Q1, Q2)
where
    <Q1 as QProduct>::Output: std::ops::Mul<<Q2 as QProduct>::Output>,
{
    type Output = <<Q1 as QProduct>::Output as std::ops::Mul<<Q2 as QProduct>::Output>>::Output;
}

impl<Q1, Q2, Q3: QProduct> QProduct for (Q1, Q2, Q3)
where
    (Q1, Q2): QProduct,
    <(Q1, Q2) as QProduct>::Output: std::ops::Mul<<Q3 as QProduct>::Output>,
{
    type Output =
        <<(Q1, Q2) as QProduct>::Output as std::ops::Mul<<Q3 as QProduct>::Output>>::Output;
}
