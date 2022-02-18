use super::combiner::*;
use super::combiners::*;
use super::parser::*;
use super::parsers_core::*;
use std::fmt::Debug;

pub fn escaped_char<'a>() -> impl Parser<'a, String>
{
    then(char_single('\\'), any_char(), smcomb(|a, b| format!("{}{}", a, b)))
}

pub fn normal_string<'a>() -> impl Parser<'a, String>
{
    mod_val(
        then(
            char_single('"'),
            none_or_many_until(any_char(), char_single('"'), l_comb),
            l_comb,
        ),
        |vs| vs, //.fold(String::from(""), |str,c|str.push(c))
    )
}

pub fn digit<'a>() -> impl Parser<'a, String> { char_in_str("0123456789") }

pub fn newline<'a>() -> impl Parser<'a, String> { or(keyword("\r\n"), keyword("\n")) }

pub fn air<'a, DatT : PResData>() -> impl Parser<'a, String> { or(char_in_str(" \t"), newline()) }

pub fn comma<'a>() -> impl Parser<'a, String> { char_single(',') }

pub fn dot<'a>() -> impl Parser<'a, String> { char_single('.') }

// fn in_air<'a, DatT :Debug+Clone >(p : impl Parser<'a, DatT>) -> impl
// Parser<'a,DatT> { then(air(), then(p, air(), l_comb), r_comb) }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Infix<LeftT, ParentT, RightT>
{
    pub p : ParentT,
    pub l : LeftT,
    pub r : RightT,
}

pub fn infix<'a, DatLeft : PResData, DatParent : PResData, DatRight : PResData>(
    p_left : impl Parser<'a, DatLeft>,
    p_parent : impl Parser<'a, DatParent>,
    p_right : impl Parser<'a, DatRight>,
) -> impl Parser<'a, Infix<DatLeft, DatParent, DatRight>>
{
    mod_val(
        then(p_left, then(p_parent, p_right, lr_comb), lr_comb),
        move |(l, (p, r))| -> Infix<DatLeft, DatParent, DatRight> {
            Infix {
                l,
                p,
                r,
            }
        },
    )
}

pub fn infixp<'a, DatLeft : PResData, DatParent : PResData, DatRight : PResData>(
    p_left : impl Parser<'a, DatLeft>,
    p_parent : impl Parser<'a, DatParent>,
    p_right : impl Parser<'a, DatRight>,
) -> impl Parser<'a, DatParent>
{
    then(p_left, then(p_parent, p_right, l_comb), r_comb)
}

pub fn maybe<'a, DatT : PResData>(p : impl Parser<'a, DatT>) -> impl Parser<'a, Option<DatT>> { one_or_none(p) }
