use super::parser::{POut, PRes, PResData};

pub trait Combiner<'a, DatA : PResData, DatB : PResData, DatOut : PResData> =
    Fn(POut<'a, DatA>, POut<'a, DatB>) -> POut<'a, DatOut> + Clone;
pub trait CombinerOk<'a, DatA : PResData, DatB : PResData, DatOut : PResData> = Fn(DatA, DatB) -> DatOut + Clone;

pub fn gen_comb<'a, DatA : PResData, DatB : PResData, DatOut : PResData>(
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
        (Ok(_pa), Err(pb)) => Err(pb),
        (Err(pa), _) => Err(pa),
    }
}

// Simple Combine, generates a combiner that is *simple*
pub fn smcomb<'a, DatA : PResData, DatB : PResData, DatOut : PResData>(
    comb : impl Fn(DatA, DatB) -> DatOut + Clone,
) -> impl Combiner<'a, DatA, DatB, DatOut>
{
    move |a, b| gen_comb(a, b, comb.clone())
}
