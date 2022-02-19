use std::fmt::Debug;

use super::file_pos::FilePos;

// Input text to be parsed
#[derive(Debug, Clone, Copy)]
pub struct ParserInput<'a>
{
    pub text : &'a str,
    pub pos :  FilePos,
}

impl ParserInput<'a>
{
    pub fn new(to_parse : &'a str) -> ParserInput<'a>
    {
        ParserInput {
            text : to_parse,
            pos :  FilePos {
                line : 1, column : 0
            },
        }
    }
}

pub trait PResData = Debug + Clone + PartialEq + Eq;

#[derive(Debug, Clone, PartialEq)]
pub struct PRes<'a, DatT : PResData>
{
    pub val :       DatT,
    pub pos :       FilePos,
    pub remainder : &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PErr
{
    pub pos : FilePos,
}

pub type POut<'a, DatT> = Result<PRes<'a, DatT>, PErr>;

pub fn get_p_out_pos<'a, DatT : PResData>(pout : &POut<'a, DatT>) -> FilePos
{
    match pout
    {
        Ok(p_succ) => p_succ.pos,
        Err(p_err) => p_err.pos,
    }
}

pub trait Parser<'a, DatT : PResData> = Fn(&ParserInput<'a>) -> POut<'a, DatT> + Clone;

pub trait Predicate<'a> = Fn(&'a str) -> bool + Clone;

impl<DatT : PResData> PRes<'a, DatT>
{
    pub fn to_in(&self) -> ParserInput<'a>
    {
        ParserInput {
            pos :  self.pos,
            text : self.remainder,
        }
    }
}
