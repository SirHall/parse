use std::fmt::Debug;

use super::parser::{POut, PRes};

pub trait Combiner<'a, DatA : Debug + Clone, DatB : Debug + Clone, DatOut : Debug + Clone> =
    Fn(POut<'a, DatA>, POut<'a, DatB>) -> POut<'a, DatOut> + Clone;
pub trait CombinerOk<'a, DatA : Debug + Clone, DatB : Debug + Clone, DatOut : Debug + Clone> =
    Fn(DatA, DatB) -> DatOut + Clone;

pub fn gen_comb<'a, DatA : Debug + Clone, DatB : Debug + Clone, DatOut : Debug + Clone>(
    a : POut<'a, DatA>,
    b : POut<'a, DatB>,
    comb : impl CombinerOk<'a, DatA, DatB, DatOut>,
) -> POut<'a, DatOut>
{
    match (a, b)
    {
        (Ok(pa), Ok(pb)) => Ok(PRes {
            val :       comb(pa.val, pb.val),
            pos :       pb.pos,
            remainder : pb.remainder,
        }),
        (Ok(pa), Err(pb)) => Err(pb),
        (Err(pa), _) => Err(pa),
    }
}

// Simple Combine, generates a combiner that is *simple*
pub fn smcomb<'a, DatA : Debug + Clone, DatB : Debug + Clone, DatOut : Debug + Clone>(
    comb : impl Fn(DatA, DatB) -> DatOut + Clone,
) -> impl Combiner<'a, DatA, DatB, DatOut>
{
    move |a, b| gen_comb(a, b, comb.clone())
}
