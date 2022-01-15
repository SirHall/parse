#![feature(in_band_lifetimes, trait_alias, type_alias_impl_trait)]
use std::{char, fmt::Debug};

// use rayon::prelude::*;

// Text position in the original file
#[derive(Debug, Default, Clone, Copy)]
pub struct FilePos
{
    line :   usize,
    column : usize,
}

impl FilePos
{
    pub fn new(line : usize, column : usize) -> Self
    {
        Self {
            line,
            column,
        }
    }

    pub fn incr_col(&self) -> Self
    {
        Self {
            line :   self.line,
            column : self.column + 1,
        }
    }

    pub fn incr_line(&self) -> Self
    {
        Self {
            line :   self.line + 1,
            column : 0,
        }
    }
}

// Input text to be parsed
#[derive(Debug, Clone, Copy)]
pub struct ParserInput<'a>
{
    text : &'a str,
    pos :  FilePos,
}

impl ParserInput<'a>
{
    pub fn new(to_parse : &'a str) -> ParserInput<'a>
    {
        ParserInput {
            text : to_parse,
            pos :  FilePos {
                line : 0, column : 0
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct PRes<'a, DatT : Debug + Clone>
{
    val :       DatT,
    pos :       FilePos,
    remainder : &'a str,
}

#[derive(Debug, Clone)]
pub struct PErr
{
    pos : FilePos,
}

pub type POut<'a, DatT> = Result<PRes<'a, DatT>, PErr>;

pub fn get_p_out_pos<'a, DatT : Debug + Clone>(pout : &POut<'a, DatT>) -> FilePos
{
    match pout
    {
        Ok(p_succ) => p_succ.pos,
        Err(p_err) => p_err.pos,
    }
}

#[derive(Debug, Clone)]
pub enum Either2<LeftT, RightT>
{
    Left(LeftT),
    Right(RightT),
}

#[derive(Debug, Clone)]
pub enum Either3<TA, TB, TC>
{
    A(TA),
    B(TB),
    C(TC),
}

// type Out<'a, DatT> = Result<OutDat<'a, DatT>, FailDat>;

// type Parser<'a> = impl Fn(&InDat<'a>) -> Out<'a>;
trait Parser<'a, DatT : Debug + Clone> = Fn(&ParserInput<'a>) -> POut<'a, DatT> + Clone;

trait Combiner<'a, DatA : Debug + Clone, DatB : Debug + Clone, DatOut : Debug + Clone> =
    Fn(POut<'a, DatA>, POut<'a, DatB>) -> POut<'a, DatOut> + Clone;
trait CombinerOk<'a, DatA : Debug + Clone, DatB : Debug + Clone, DatOut : Debug + Clone> =
    Fn(DatA, DatB) -> DatOut + Clone;

trait Predicate<'a> = Fn(&'a str) -> bool + Clone;

trait Combinable {}

trait StrCombinable {}

fn gen_comb<'a, DatA : Debug + Clone, DatB : Debug + Clone, DatOut : Debug + Clone>(
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
fn smcomb<'a, DatA : Debug + Clone, DatB : Debug + Clone, DatOut : Debug + Clone>(
    comb : impl Fn(DatA, DatB) -> DatOut + Clone,
) -> impl Combiner<'a, DatA, DatB, DatOut>
{
    move |a, b| gen_comb(a, b, comb.clone())
}

// fn lr_comb<'a, DatT>(a : Out<'a, DatT>, b : Out<'a, DatT>) -> Out<'a, DatT>
// {
//     gen_comb(a, b, |c1 : DatT, c2 : Dat| Dat::LR {
//         l : c1.into(),
//         r : c2.into(),
//     })
// }

fn l_comb<'a, DatA : Debug + Clone, DatB : Debug + Clone>(a : POut<'a, DatA>, b : POut<'a, DatB>) -> POut<'a, DatA>
{
    gen_comb(a, b, |c1 : DatA, _| c1)
}

fn r_comb<'a, DatA : Debug + Clone, DatB : Debug + Clone>(a : POut<'a, DatA>, b : POut<'a, DatB>) -> POut<'a, DatB>
{
    gen_comb(a, b, |_, c2 : DatB| c2)
}

// fn lt_comb<'a, DatT>(a : Out<'a, DatT>, b : Out<'a, DatT>) -> Out<'a, DatT>
// {
//     gen_comb(a, b, |c1 : Dat, c2 : Dat| Dat::Tree {
//         p : c1.into(),
//         c : c2.into(),
//     })
// }

// fn rt_comb<'a, DatT>(a : Out<'a, DatT>, b : Out<'a, DatT>) -> Out<'a, DatT>
// {
//     gen_comb(a, b, |c1 : DatT, c2 : DatT| Dat::Tree {
//         p : c2.into(),
//         c : c1.into(),
//     })
// }

// fn bt_comb_gen<'a, DatT>(v : Dat) -> impl Combiner<'a, DatT>
// {
//     move |a : Out<'a>, b : Out<'a>| -> Out<'a> {
//         gen_comb(a, b, |c1 : Dat, c2 : Dat| Dat::BTree {
//             p : v.clone().into(),
//             l : c1.into(),
//             r : c2.into(),
//         })
//     }
// }

fn tuple_comb<'a, DatA : Debug + Clone, DatB : Debug + Clone>(
    a : POut<'a, DatA>,
    b : POut<'a, DatB>,
) -> POut<'a, (DatA, DatB)>
{
    gen_comb(a, b, |l, r| (l, r))
}

impl<DatT : Debug + Clone> PRes<'a, DatT>
{
    fn to_in(&self) -> ParserInput<'a>
    {
        ParserInput {
            pos :  self.pos,
            text : self.remainder,
        }
    }
}

fn then<'a, DatA : Debug + Clone, DatB : Debug + Clone, DatOut : Debug + Clone>(
    a : impl Parser<'a, DatA>,
    b : impl Parser<'a, DatB>,
    comb : impl Combiner<'a, DatA, DatB, DatOut>,
) -> impl Parser<'a, DatOut>
{
    move |ind : &ParserInput<'a>| -> POut<'a, DatOut> {
        match a(ind)
        {
            Ok(a) =>
            {
                let b_res = b(&a.to_in());
                comb(Ok(a), b_res)
            },
            Err(a) => Err(a),
        }
    }
}

fn mod_out<'a, DatIn : Debug + Clone, DatOut : Debug + Clone>(
    p : impl Parser<'a, DatIn>,
    f : impl Fn(PRes<'a, DatIn>) -> POut<'a, DatOut> + Clone,
) -> impl Parser<'a, DatOut>
{
    move |ind : &ParserInput<'a>| -> POut<'a, DatOut> {
        p(ind).and_then(&f) // TODO: See if this incurrs a slowdown over a 'more
                            // basic' implementation
    }
}

fn mod_dat<'a, DatIn : Debug + Clone, DatOut : Debug + Clone>(
    p : impl Parser<'a, DatIn>,
    f : impl Fn(PRes<'a, DatIn>) -> PRes<'a, DatOut> + Clone,
) -> impl Parser<'a, DatOut>
{
    mod_out(p, move |v : PRes<'a, DatIn>| -> POut<'a, DatOut> { Ok(f(v)) })
}

fn mod_val<'a, DatIn : Debug + Clone, DatOut : Debug + Clone>(
    p : impl Parser<'a, DatIn>,
    f : impl Fn(DatIn) -> DatOut + Clone,
) -> impl Parser<'a, DatOut>
{
    mod_dat(p, move |od : PRes<'a, DatIn>| -> PRes<'a, DatOut> {
        PRes {
            val :       f(od.val),
            pos :       od.pos,
            remainder : od.remainder,
        }
    })
}

fn replace_val<'a, DatIn : Debug + Clone, DatOut : Debug + Clone>(
    p : impl Parser<'a, DatIn>,
    v : impl Fn() -> DatOut + Clone,
) -> impl Parser<'a, DatOut>
{
    mod_dat(p, move |od : PRes<'a, DatIn>| -> PRes<'a, DatOut> {
        PRes {
            val :       v(),
            pos :       od.pos,
            remainder : od.remainder,
        }
    })
}

fn succeed_if<'a, DatT : Debug + Clone>(
    p : impl Parser<'a, DatT>,
    f : impl Fn(&PRes<'a, DatT>) -> bool + Clone,
) -> impl Parser<'a, DatT>
{
    move |ind : &ParserInput<'a>| -> POut<'a, DatT> {
        p(ind).and_then(|r| {
            if f(&r)
            {
                Ok(r)
            }
            else
            {
                Err(PErr {
                    pos : ind.pos
                })
            }
        })
    }
}

fn fail_if<'a, DatT : Debug + Clone>(
    p : impl Parser<'a, DatT>,
    f : impl Fn(&PRes<'a, DatT>) -> bool + Clone,
) -> impl Parser<'a, DatT>
{
    succeed_if(p, move |r| !f(r))
}

fn all<'a, DatT : Debug + Clone>(p : impl Parser<'a, DatT>) -> impl Parser<'a, DatT>
{
    succeed_if(p, |r| r.remainder.is_empty())
}

// TODO: Replace/Modify fail message

// TODO: This may not work, the lifetimes probably aren't right
fn always<'a, DatT : Debug + Clone>(defaultFn : impl Fn() -> DatT + Clone) -> impl Parser<'a, DatT>
{
    move |ind : &ParserInput<'a>| -> POut<'a, DatT> {
        Ok(PRes {
            val :       defaultFn(),
            pos :       ind.pos,
            remainder : ind.text,
        })
    }
}

fn not<'a, DatT : Debug + Clone>(
    p : impl Parser<'a, DatT>,
    defaultFn : impl Fn() -> DatT + Clone,
) -> impl Parser<'a, DatT>
{
    move |ind : &ParserInput<'a>| -> POut<'a, DatT> {
        match p(ind)
        {
            Ok(v) => Err(PErr {
                pos : ind.pos
            }),
            Err(v) => Ok(PRes {
                val :       defaultFn(),
                pos :       ind.pos,
                remainder : ind.text,
            }),
        }
    }
}

fn or<'a, DatT : Debug + Clone>(a : impl Parser<'a, DatT>, b : impl Parser<'a, DatT>) -> impl Parser<'a, DatT>
{
    move |ind : &ParserInput<'a>| -> POut<'a, DatT> {
        match a(ind)
        {
            Ok(a) => Ok(PRes {
                val :       a.val,
                pos :       a.pos,
                remainder : a.remainder,
            }),
            Err(a) => match b(ind)
            {
                Ok(b) => Ok(PRes {
                    val :       b.val,
                    pos :       b.pos,
                    remainder : b.remainder,
                }),
                Err(b) => Err(b),
            },
        }
    }
}

fn or_diff<'a, DatA : Debug + Clone, DatB : Debug + Clone>(
    a : impl Parser<'a, DatA>,
    b : impl Parser<'a, DatB>,
) -> impl Parser<'a, Either2<DatA, DatB>>
{
    move |ind : &ParserInput<'a>| -> POut<'a, Either2<DatA, DatB>> {
        match a(ind)
        {
            Ok(a) => Ok(PRes {
                val :       Either2::Left(a.val),
                pos :       a.pos,
                remainder : a.remainder,
            }),
            Err(a) => match b(ind)
            {
                Ok(b) => Ok(PRes {
                    val :       Either2::Right(b.val),
                    pos :       b.pos,
                    remainder : b.remainder,
                }),
                Err(b) => Err(b),
            },
        }
    }
}

fn either_or<'a, DatA : Debug + Clone, DatB : Debug + Clone, DatOut : Debug + Clone>(
    a : impl Parser<'a, DatA>,
    b : impl Parser<'a, DatB>,
    comb : impl Combiner<'a, DatA, DatB, DatOut>,
) -> impl Parser<'a, Either3<DatOut, DatA, DatB>>
{
    mod_val(or_diff(then(a.clone(), b.clone(), comb), or_diff(a, b)), |v| match v
    {
        Either2::Left(l) => Either3::A(l),
        Either2::Right(r) => match r
        {
            Either2::Left(rl) => Either3::B(rl),
            Either2::Right(rr) => Either3::C(rr),
        },
    })
}

// TODO: Only way to have variadic generic functions is to use macros
// fn chain<'a, DatA, DatB, DatOutT>(
//     ps : Vec<impl Parser<'a, DatT>>,
//     comb : impl Combiner<'a, DatA, DatB, DatOutT>,
// ) -> impl Parser<'a, DatT>
// {
//     move |ind : &ParserInput<'a>| -> Out<'a> {
//         if ps.len() == 0
//         {
//             Err(FailDat {})
//         }
//         else
//         {
//             let mut out = ps[0](ind);

//             for i in 1..ps.len()
//             {
//                 match out.as_ref()
//                 {
//                     Ok(succ) => out = comb(out.to_owned(),
// ps[i](&succ.to_in())),                     Err(fail) => return Err(FailDat
// {}),                 };
//             }

//             out
//         }
//     }
// }

fn chain_select<'a, DatT : Debug + Clone>(ps : Vec<impl Parser<'a, DatT>>, index : usize) -> impl Parser<'a, DatT>
{
    move |ind : &ParserInput<'a>| -> POut<'a, DatT> {
        if ps.len() == 0
        {
            Err(PErr {
                pos : ind.pos
            })
        }
        else
        {
            // Ensure that all succeed, but only return the value of the selected parser

            let mut selected : Option<PRes<'a, DatT>> = None;

            let mut current = ps[0](ind);

            for i in 1..ps.len()
            {
                match current
                {
                    Ok(c_val) =>
                    {
                        if i == index
                        {
                            selected = Some(c_val.clone());
                        }
                        current = r_comb(Ok(c_val.clone()), ps[i](&c_val.to_in()));
                    },
                    Err(c_err) => return Err(c_err),
                };
            }

            match (selected, current)
            {
                (Some(s), Ok(c)) => Ok(PRes {
                    val : s.val,
                    ..c
                }),
                (_, Err(c_err)) => Err(c_err),
                (None, Ok(_)) => Err(PErr {
                    pos : ind.pos
                }), // Should not be possible to reach
            }
        }
    }
}

fn or_chain<'a, DatT : Debug + Clone>(ps : Vec<impl Parser<'a, DatT>>) -> impl Parser<'a, DatT>
{
    move |ind : &ParserInput<'a>| -> POut<'a, DatT> {
        for i in 0..ps.len()
        {
            let res = ps[i](ind);
            if res.is_ok()
            {
                return res;
            }
        }

        return Err(PErr {
            pos : ind.pos
        });
    }
}

fn one_or_none<'a, DatT : Debug + Clone>(p : impl Parser<'a, DatT>) -> impl Parser<'a, Option<DatT>>
{
    mod_val(or_diff(p, always(|| ())), |v : Either2<DatT, ()>| -> Option<DatT> {
        match v
        {
            Either2::Left(lv) => Some(lv),
            Either2::Right(_) => None,
        }
    })
}

fn none_or_many<'a, DatT : Debug + Clone>(p : impl Parser<'a, DatT>) -> impl Parser<'a, Vec<DatT>>
{
    // Done this way to prevent stack overflows

    move |ind : &ParserInput<'a>| -> POut<'a, Vec<DatT>> {
        let mut ls = vec![];

        let mut ind_looped = ind.clone();

        loop
        {
            match one_or_none(p.clone())(ind)
            {
                Ok(p_result) => match &p_result.val
                {
                    Some(parsed_element) =>
                    {
                        ls.push(parsed_element.clone());
                        ind_looped = p_result.to_in();
                    },
                    None =>
                    {
                        return Ok(PRes {
                            val :       ls,
                            pos :       p_result.pos,
                            remainder : p_result.remainder,
                        });
                    },
                },
                Err(p_err) =>
                {
                    return Err(p_err);
                },
            }
        }
    }
}

fn one_or_many<'a, DatT : Debug + Clone>(p : impl Parser<'a, DatT>) -> impl Parser<'a, Vec<DatT>>
{
    succeed_if(none_or_many(p), move |v : &PRes<'a, Vec<DatT>>| v.val.len() == 0)
}

fn none_or_many_until<'a, DatA : Debug + Clone, DatB : Debug + Clone, DatOut : Debug + Clone>(
    pa : impl Parser<'a, DatA>,
    pb : impl Parser<'a, DatB>,
    comb : impl Combiner<'a, Vec<DatA>, DatB, DatOut>,
) -> impl Parser<'a, DatOut> // We either return the stop type or the combined type
{
    then(none_or_many(pa), pb, comb)
}

fn one_or_many_until<'a, DatA : Debug + Clone, DatB : Debug + Clone, DatOut : Debug + Clone>(
    pa : impl Parser<'a, DatA>,
    pb : impl Parser<'a, DatB>,
    comb : impl Combiner<'a, Vec<DatA>, DatB, DatOut>,
) -> impl Parser<'a, DatOut>
{
    then(one_or_many(pa), pb, comb)
}

fn read_char_f<'a>(predicate : impl Fn(char) -> bool + Clone) -> impl Parser<'a, String>
{
    move |ind : &ParserInput<'a>| -> POut<'a, String> {
        match ind.text.chars().nth(0)
        {
            Some(c) =>
            {
                if predicate(c)
                {
                    Ok(PRes {
                        val :       format!("{}", c),
                        pos :       FilePos {
                            line :   ind.pos.line,
                            column : ind.pos.column + 1,
                        },
                        remainder : ind.text.get(1..).unwrap(),
                    })
                }
                else
                {
                    Err(PErr {
                        pos : ind.pos
                    })
                }
            },
            None => Err(PErr {
                pos : ind.pos
            }),
        }
    }
}

fn char_in_str<'a>(chars_list : &'a str) -> impl Parser<'a, String>
{
    read_char_f(|c| chars_list.chars().any(|f| f == c))
}

fn char_single<'a>(ch : char) -> impl Parser<'a, String> { read_char_f(move |c| c == ch) }

fn keyword(word : &'a str) -> impl Parser<'a, String>
{
    // TODO: Add an error for an empty keyword
    move |ind : &ParserInput<'a>| -> POut<'a, String> {
        if ind.text.starts_with(word)
        {
            Ok(PRes {
                val :       String::from(word),
                pos :       FilePos::new(ind.pos.line, ind.pos.column + word.len()),
                remainder : ind.text.get(word.len()..).unwrap(),
            })
        }
        else
        {
            Err(PErr {
                pos : ind.pos
            })
        }
    }
}

fn any_char<'a>() -> impl Parser<'a, String> { read_char_f(|_| true) }

// fn consume_until<'a, DatEnd :Debug+Clone , DatOut :Debug+Clone >(
//     p : impl Parser<'a, DatEnd>,
//     comb : impl Combiner<'a, String,DatEnd, DatOut>+Clone,
// ) -> impl Parser<'a, DatOut>
// {
//  none_or_many_until(any_char(), p.clone(), |a,b|{
//   gen_comb(a, b, |res_l:Vec<String>,res_r:DatEnd|
//     comb(res_l.join(""), res_r)
// )
//  })
// }

fn thenr<'a, DatA : Debug + Clone, DatB : Debug + Clone, DatOut : Debug + Clone>(
    a : impl Parser<'a, DatA>,
    b : impl Parser<'a, DatB>,
    comb : impl Combiner<'a, DatA, DatB, DatOut>,
) -> impl Parser<'a, DatOut>
{
    move |ind : &ParserInput<'a>| -> POut<'a, DatOut> {
        for i in 0..ind.text.len()
        {
            let sap = ind.text.get(..i);
            let sbp = ind.text.get(i..);

            match (sap, sbp)
            {
                (Some(sa), Some(sb)) =>
                {
                    match all(a.clone())(&ParserInput {
                        text : sa,
                        pos :  ind.pos,
                    })
                    {
                        Ok(a_succ) =>
                        {
                            let b_res = b(&ParserInput {
                                text : ind.text,
                                pos :  a_succ.pos,
                            });
                            match b_res
                            {
                                Ok(b_succ) =>
                                {
                                    return comb(Ok(a_succ), Ok(b_succ));
                                },
                                Err(b_err) =>
                                {},
                            }
                        },
                        Err(a_err) =>
                        {},
                    }
                },
                (_, _) =>
                {},
            }
        }

        return Err(PErr {
            pos : ind.pos
        });
    }
}

// Now a lot more specific
//--- COMMON PARSERS ---//

fn escaped_char<'a>() -> impl Parser<'a, String>
{
    then(char_single('\\'), any_char(), smcomb(|a, b| format!("{}{}", a, b)))
}

fn normal_string<'a>() -> impl Parser<'a, String>
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

fn digit<'a>() -> impl Parser<'a, String> { char_in_str("0123456789") }

fn newline<'a>() -> impl Parser<'a, String> { or(keyword("\r\n"), keyword("\n")) }

fn air<'a, DatT : Debug + Clone>() -> impl Parser<'a, String> { or(char_in_str(" \t"), newline()) }

fn comma<'a>() -> impl Parser<'a, String> { char_single(',') }

fn dot<'a>() -> impl Parser<'a, String> { char_single('.') }

// fn in_air<'a, DatT :Debug+Clone >(p : impl Parser<'a, DatT>) -> impl
// Parser<'a,DatT> { then(air(), then(p, air(), l_comb), r_comb) }

#[derive(Debug, Clone)]
pub struct Infix<LeftT, ParentT, RightT>
{
    pub p : ParentT,
    pub l : LeftT,
    pub r : RightT,
}

fn infix<'a, DatLeft : Debug + Clone, DatParent : Debug + Clone, DatRight : Debug + Clone>(
    p_left : impl Parser<'a, DatLeft>,
    p_parent : impl Parser<'a, DatParent>,
    p_right : impl Parser<'a, DatRight>,
) -> impl Parser<'a, Infix<DatLeft, DatParent, DatRight>>
{
    mod_val(
        then(p_left, then(p_parent, p_right, tuple_comb), tuple_comb),
        move |(l, (p, r))| -> Infix<DatLeft, DatParent, DatRight> {
            Infix {
                l,
                p,
                r,
            }
        },
    )
}

fn infixp<'a, DatLeft : Debug + Clone, DatParent : Debug + Clone, DatRight : Debug + Clone>(
    p_left : impl Parser<'a, DatLeft>,
    p_parent : impl Parser<'a, DatParent>,
    p_right : impl Parser<'a, DatRight>,
) -> impl Parser<'a, DatParent>
{
    then(p_left, then(p_parent, p_right, l_comb), r_comb)
}

fn maybe<'a, DatT : Debug + Clone>(p : impl Parser<'a, DatT>) -> impl Parser<'a, Option<DatT>> { one_or_none(p) }

// Calculator specific

// fn exp<'a, DatT>() -> impl Parser<'a, DatT>
// {
//     move |ind : &ParserInput<'a>| -> Out<'a> {
//         or(
//             one_or_many(digit(), lt_comb),
//             infix(exp(), in_air(char_single('+')), exp()),
//         )(ind)
//         // or_chain(vec![
//         //     one_or_many(digit(), lr_comb),
//         //     infix(exp(), in_air(char_single('+')), exp()),
//         // ])(ind)
//     }
// }

// fn collapse_btree<'a>(p : impl Parser<'a>) -> impl Parser<'a> { mod_val(p, f)
// }

#[derive(Debug, Clone)]
enum MathAST
{
    Number(f64),
    Addition(Box<MathAST>, Box<MathAST>),
}

fn main()
{
    // let text = "1 + 2";
    // println!("{}", text);
    // println!("{:#?}", exp::<MathAST>()(&InDat::new(text)));
}
