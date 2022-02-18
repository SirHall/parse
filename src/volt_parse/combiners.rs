use super::{
    combiner::gen_comb,
    parser::{POut, PResData},
};
use std::fmt::Debug;

pub fn l_comb<'a, DatA : PResData, DatB : PResData>(a : POut<'a, DatA>, b : POut<'a, DatB>) -> POut<'a, DatA>
{
    gen_comb(a, b, |l, _| l)
}

pub fn r_comb<'a, DatA : PResData, DatB : PResData>(a : POut<'a, DatA>, b : POut<'a, DatB>) -> POut<'a, DatB>
{
    gen_comb(a, b, |_, r| r)
}

pub fn lr_comb<'a, DatA : PResData, DatB : PResData>(a : POut<'a, DatA>, b : POut<'a, DatB>) -> POut<'a, (DatA, DatB)>
{
    gen_comb(a, b, |l, r| (l, r))
}

pub fn rl_comb<'a, DatA : PResData, DatB : PResData>(a : POut<'a, DatA>, b : POut<'a, DatB>) -> POut<'a, (DatB, DatA)>
{
    gen_comb(a, b, |l, r| (r, l))
}
