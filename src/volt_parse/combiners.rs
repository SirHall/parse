use super::{
    combiner::gen_comb,
    parser::{POut, PResData},
};
use std::fmt::Display;

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

pub fn disp_comb<'a, DatA : PResData + Display, DatB : PResData + Display>(
    a : POut<'a, DatA>,
    b : POut<'a, DatB>,
) -> POut<'a, String>
{
    gen_comb(a, b, |l, r| format!("{l}{r}"))
}

pub(crate) fn tuple_left_char_vec_to_str<'a, DatB : PResData>(
    a : POut<'a, Vec<char>>,
    b : POut<'a, DatB>,
) -> POut<'a, (String, DatB)>
{
    gen_comb(a, b, |l : Vec<char>, r| (l.into_iter().collect(), r))
}
