#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Either2<LeftT, RightT>
{
    Left(LeftT),
    Right(RightT),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Either3<TA, TB, TC>
{
    A(TA),
    B(TB),
    C(TC),
}
