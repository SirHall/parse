use super::{
    combiner::gen_comb,
    parser::{POut, PResData},
};

pub fn take_left<'a, DatA : PResData, DatB : PResData>(a : POut<'a, DatA>, b : POut<'a, DatB>) -> POut<'a, DatA>
{
    gen_comb(a, b, |l, _| l)
}

pub fn take_right<'a, DatA : PResData, DatB : PResData>(a : POut<'a, DatA>, b : POut<'a, DatB>) -> POut<'a, DatB>
{
    gen_comb(a, b, |_, r| r)
}

pub fn left_right<'a, DatA : PResData, DatB : PResData>(
    a : POut<'a, DatA>,
    b : POut<'a, DatB>,
) -> POut<'a, (DatA, DatB)>
{
    gen_comb(a, b, |l, r| (l, r))
}

pub fn right_left<'a, DatA : PResData, DatB : PResData>(
    a : POut<'a, DatA>,
    b : POut<'a, DatB>,
) -> POut<'a, (DatB, DatA)>
{
    gen_comb(a, b, |l, r| (r, l))
}
