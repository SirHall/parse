// TODO: Re-do this with procedural macros?
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Or2<TA, TB>
{
    A(TA),
    B(TB),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Or3<TA, TB, TC>
{
    A(TA),
    B(TB),
    C(TC),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Or4<TA, TB, TC, TD>
{
    A(TA),
    B(TB),
    C(TC),
    D(TD),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Or5<TA, TB, TC, TD, TE>
{
    A(TA),
    B(TB),
    C(TC),
    D(TD),
    E(TE),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Or6<TA, TB, TC, TD, TE, TF>
{
    A(TA),
    B(TB),
    C(TC),
    D(TD),
    E(TE),
    F(TF),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Or7<TA, TB, TC, TD, TE, TF, TG>
{
    A(TA),
    B(TB),
    C(TC),
    D(TD),
    E(TE),
    F(TF),
    G(TG),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Or8<TA, TB, TC, TD, TE, TF, TG, TH>
{
    A(TA),
    B(TB),
    C(TC),
    D(TD),
    E(TE),
    F(TF),
    G(TG),
    H(TH),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Or9<TA, TB, TC, TD, TE, TF, TG, TH, TI>
{
    A(TA),
    B(TB),
    C(TC),
    D(TD),
    E(TE),
    F(TF),
    G(TG),
    H(TH),
    I(TI),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Or10<TA, TB, TC, TD, TE, TF, TG, TH, TI, TJ>
{
    A(TA),
    B(TB),
    C(TC),
    D(TD),
    E(TE),
    F(TF),
    G(TG),
    H(TH),
    I(TI),
    J(TJ),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Or11<TA, TB, TC, TD, TE, TF, TG, TH, TI, TJ, TK>
{
    A(TA),
    B(TB),
    C(TC),
    D(TD),
    E(TE),
    F(TF),
    G(TG),
    H(TH),
    I(TI),
    J(TJ),
    K(TK),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Or12<TA, TB, TC, TD, TE, TF, TG, TH, TI, TJ, TK, TL>
{
    A(TA),
    B(TB),
    C(TC),
    D(TD),
    E(TE),
    F(TF),
    G(TG),
    H(TH),
    I(TI),
    J(TJ),
    K(TK),
    L(TL),
}
