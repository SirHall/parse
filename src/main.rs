#![feature(in_band_lifetimes, trait_alias, type_alias_impl_trait)]

use std::{char, fmt::Debug};

use rayon::prelude::*;

// Text position in the original file
#[derive(Debug, Default, Clone, Copy)]
struct FilePos
{
    line :   usize,
    column : usize,
}

impl FilePos
{
    fn new(line : usize, column : usize) -> Self
    {
        Self {
            line,
            column,
        }
    }

    fn incr_col(&self) -> Self
    {
        Self {
            line :   self.line,
            column : self.column + 1,
        }
    }

    fn incr_line(&self) -> Self
    {
        Self {
            line :   self.line + 1,
            column : 0,
        }
    }
}

// Input text to be parsed
#[derive(Debug, Clone, Copy)]
struct ParserInput<'a>
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
struct PRes<'a, DatT : Debug + Clone>
{
        val :       DatT,
        pos :       FilePos,
        remainder : &'a str,
}

impl<'a, DatT: Debug+Clone> PRes<'a, DatT>
{
    pub fn succeeded(&self) -> bool
    {
        match self
        {
            PRes::Success {
                ..
            } => true,
            PRes::Fail {
                ..
            } => false,
        }
    }

    pub fn failed(&self) -> bool
    {
        match self
        {
            PRes::Success {
                ..
            } => false,
            PRes::Fail {
                ..
            } => true,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Either2<LeftT, RightT>
{
    Left(LeftT),
    Right(RightT),
}

#[derive(Debug,Clone)]
pub enum Either3<TA,TB,TC>{
    A(TA),B(TB),C(TC)
}

// type Out<'a, DatT> = Result<OutDat<'a, DatT>, FailDat>;

// type Parser<'a> = impl Fn(&InDat<'a>) -> Out<'a>;
trait Parser<'a, DatT: Debug+Clone> = Fn(&ParserInput<'a>) -> PRes<'a, DatT> + Clone;

trait Combiner<'a, DatA: Debug+Clone, DatB: Debug+Clone, DatOut: Debug+Clone> = Fn(PRes<'a, DatA>, PRes<'a, DatB>) -> PRes<'a, DatOut> + Clone;
trait CombinerOk<'a, DatA: Debug+Clone, DatB: Debug+Clone, DatOut: Debug+Clone> = Fn(DatA, DatB) -> DatOut + Clone;

trait Predicate<'a> = Fn(&'a str) -> bool + Clone;

trait Combinable {}

trait StrCombinable {}

fn gen_comb<'a, DatA: Debug+Clone, DatB: Debug+Clone, DatOut: Debug+Clone>(
    a : PRes<'a, DatA>,
    b : PRes<'a, DatB>,
    comb : impl CombinerOk<'a, DatA, DatB,  DatOut>
) -> PRes<'a,  Either2<DatOut,> >
{
    match (a, b)
    {
        (
            PRes::Success {
                val:a_val,
                pos:a_pos,
                remainder:a_rem
            },
            PRes::Success {
                val:b_val,
                pos:b_pos,
                remainder:b_rem
            },
        ) => PRes::Success {
            val : comb(a_val, b_val),
            pos: b_pos,
            remainder: b_rem,
            
        },
        (PRes::Success{..},_)=>b,
        (_,_)=>a
    }
}

// fn lr_comb<'a, DatT>(a : Out<'a, DatT>, b : Out<'a, DatT>) -> Out<'a, DatT>
// {
//     gen_comb(a, b, |c1 : DatT, c2 : Dat| Dat::LR {
//         l : c1.into(),
//         r : c2.into(),
//     })
// }

fn l_comb<'a, DatA: Debug+Clone, DatB: Debug+Clone, DatOut: Debug+Clone>(a : PRes<'a, DatA>, b : PRes<'a, DatB>) -> PRes<'a, DatOut>
{
    gen_comb(a, b, |c1 : DatA, _| c1)
}

fn r_comb<'a, DatA: Debug+Clone, DatB: Debug+Clone, DatOut: Debug+Clone>(a : PRes<'a, DatA>, b : PRes<'a, DatB>) -> PRes<'a, DatOut>
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

impl<DatT: Debug+Clone> PRes<'a, DatT>
{
    fn to_in(&self) -> ParserInput<'a>
    {
        ParserInput {
            pos :  self.pos,
            text : self.remainder,
        }
    }
}

fn then<'a, DatA: Debug+Clone, DatB: Debug+Clone, DatOut: Debug+Clone>(
    a : impl Parser<'a, DatA>,
    b : impl Parser<'a, DatB>,
    comb : impl Combiner<'a, DatA, DatB, DatOut>,
) -> impl Parser<'a, DatOut>
{
    move |ind : &ParserInput<'a>| -> PRes<'a,DatOut> {
        let a_res = a(ind);
        match a_res
        {
            PRes::Success {
                ..
            } =>
            {
                let b_res = b(&a_res.to_in());
                comb(a_res, b_res)
            },
            PRes::Fail {
                ..
            } => a_res,
        }
    }
}

fn mod_dat<'a, DatIn: Debug+Clone, DatOut: Debug+Clone>(
    p : impl Parser<'a, DatIn>,
    f : impl Fn(PRes<'a, DatIn>) -> PRes<'a, DatOut> + Clone,
) -> impl Parser<'a, DatOut>
{
    move |ind : &ParserInput<'a>| -> PRes<'a, DatOut> {
        let p_res = p(ind);
        match p_res
        {
            PRes::Success {
                ..
            } => f(p_res),
            PRes::Fail {
                ..
            } => p_res,
        }
    }
}

fn mod_val<'a, DatIn: Debug+Clone, DatOut: Debug+Clone>(
    p : impl Parser<'a, DatIn>,
    f : impl Fn(DatIn) -> DatOut + Clone,
) -> impl Parser<'a, DatOut>
{
    mod_dat(p, move |od : PRes<'a, DatIn>| -> PRes<'a, DatOut> {
        PRes::Success {
            val :       f(od.val),
            pos :       od.pos,
            remainder : od.remainder,
        }
    })
}

fn replace_val<'a, DatIn: Debug+Clone, DatOut: Debug+Clone>(p : impl Parser<'a, DatIn>, v : impl Fn() -> DatOut) -> impl Parser<'a, DatOut>
{
    mod_dat(p, move |od : PRes<'a, DatIn>| -> PRes<'a, DatOut> {
        PRes::Success {
            val :       p(),
            pos :       od.pos,
            remainder : od.remainder,
        }
    })
}

fn succeed_if<'a, DatT: Debug+Clone>(
    p : impl Parser<'a, DatT>,
    f : impl Fn(&PRes<'a, DatT>) -> bool + Clone,
) -> impl Parser<'a, DatT>
{
    move |ind : &ParserInput<'a>| -> PRes<'a, DatT> {
        let res = p(ind);
        res.and_then(|r| {
            if f(&r)
            {
                res
            }
            else
            {
                PRes::Fail {}
            }
        })
    }
}

fn fail_if<'a, DatT: Debug+Clone>(p : impl Parser<'a, DatT>, f : impl Fn(&PRes<'a, DatT>) -> bool + Clone) -> impl Parser<'a, DatT>
{
    succeed_if(p, move |r| !f(r))
}

fn all<'a, DatT: Debug+Clone>(p : impl Parser<'a, DatT>) -> impl Parser<'a, DatT> { succeed_if(p, |r| r.remainder.is_empty()) }

// TODO: Replace/Modify fail message

// TODO: This may not work, the lifetimes probably aren't right
fn always<'a, DatT: Debug+Clone>(defaultFn : impl Fn() -> DatT) -> impl Parser<'a, DatT>
{
    move |ind : &ParserInput<'a, DatT>| -> PRes<'a, DatT> {
        PRes::Success {
            val :       defaultFn(),
            pos :       ind.pos,
            remainder : ind.text,
        }
    }
}

fn not<'a, DatT: Debug+Clone>(p : impl Parser<'a, DatT>, defaultFn : impl Fn() -> DatT) -> impl Parser<'a, DatT>
{
    move |ind : &ParserInput<'a>| -> PRes<'a, DatT> {
        let res = p(ind);
        match res
        {
            PRes::Success {
                ..
            } => PRes::Fail {},
            PRes::Fail {
                ..
            } => PRes::Success {
                val :       defaultFn(),
                pos :       ind.pos,
                remainder : ind.text,
            },
        }
    }
}

fn or<'a, DatT: Debug+Clone>(a : impl Parser<'a, DatT>, b : impl Parser<'a, DatT>) -> impl Parser<'a, DatT>
{
    move |ind : &ParserInput<'a>| -> PRes<'a, DatT> {
        let res = a(ind);
        match res
        {
            PRes::Success {
                ..
            } => res,
            PRes::Fail {
                ..
            } => b(ind),
        }
    }
}

fn either_or<'a, DatA: Debug+Clone, DatB: Debug+Clone, DatOut: Debug+Clone>(
    a : impl Parser<'a, DatA>,
    b : impl Parser<'a, DatB>,
    comb : impl Combiner<'a, DatA, DatB, DatOut>,
) -> impl Parser<'a, DatOut>
{
    or(then(a.clone(), b.clone(), comb), or(a, b))
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

fn chain_select<'a, DatT: Debug+Clone>(ps : Vec<impl Parser<'a, DatT>>, index : usize) -> impl Parser<'a, DatT>
{
    move |ind : &ParserInput<'a>| -> PRes<'a, DatT> {
        if ps.len() == 0
        {
            PRes::Fail {}
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
                    PRes::Success {
                        ..
                    } =>
                    {
                        if i == index
                        {
                            selected = current.clone().ok();
                        }
                        current = r_comb(current.to_owned(), ps[i](&current.to_in()));
                    },
                    PRes::Fail {
                        ..
                    } => return current,
                };
            }

            match (selected, current)
            {
                (
                    Some(PRes::Success {
                        val: s_val,
                        pos: s_pos,
                        remainder: s_remainder,
                    }),
                    PRes::Success {
                        ..
                    },
                ) => PRes::Success {
                    val : (*selected).val,
                    ..current
                },
                (_, _) => current,
            }
        }
    }
}

fn or_chain<'a, DatT: Debug+Clone>(ps : Vec<impl Parser<'a, DatT>>) -> impl Parser<'a, DatT>
{
    move |ind : &ParserInput<'a>| -> PRes<'a, DatT> {
        for i in 0..ps.len()
        {
            let res = ps[i](ind);
            if res.succeeded()
            {
                return res;
            }
        }

        return PRes::Fail {};
    }
}

fn one_or_none<'a, DatT: Debug+Clone>(p : impl Parser<'a, DatT>) -> impl Parser<'a, DatT> { or(p, always("")) }

fn one_or_many<'a, DatIn: Debug+Clone, DatOut: Debug+Clone>(
    p : impl Parser<'a, DatIn>,
    comb : impl Combiner<'a, DatIn, DatIn, DatOut>,
) -> impl Parser<'a, DatOut>
{
    move |ind : &ParserInput<'a>| -> PRes<'a, DatOut> {
        or(
            then(p.clone(), one_or_many(p.clone(), comb.clone()), comb.clone()),
            p.clone(),
        )(ind)
    }
}

fn none_or_many<'a, DatIn :Debug+Clone  , DatOut :Debug+Clone >(
    p : impl Parser<'a, DatIn>,
    comb : impl Combiner<'a, DatIn, DatIn, DatOut>,
) -> impl Parser<'a, DatOut>
{
    one_or_none(one_or_many(p, comb))
}

fn none_or_many_until<'a, DatA :Debug+Clone , DatB :Debug+Clone , DatOut :Debug+Clone >(
    pa : impl Parser<'a, DatA>,
    pb : impl Parser<'a, DatB>,
    comb : impl Combiner<'a, DatA, DatA, DatOut>,
) -> impl Parser<'a, Either2<DatOut, DatB>> // We either return the stop type or the combined type
{
    move |ind : &ParserInput<'a>| -> PRes<'a> {
        or(
            pb.clone(),
            then(
                pa.clone(),
                none_or_many_until(pa.clone(), pb.clone(), comb.clone()),
                comb.clone(),
            ),
        )(ind)
    }
}

fn one_or_many_until<'a, DatA :Debug+Clone , DatB :Debug+Clone , DatOut :Debug+Clone >(
    pa : impl Parser<'a, DatA>,
    pb : impl Parser<'a, DatB>,
    comb : impl Combiner<'a, DatA, DatA, DatOut>,
) -> impl Parser<'a, Either2<DatOut, DatB>>
{
    then(
        pa.clone(),
        or(pb.clone(), none_or_many_until(pa.clone(), pb.clone(), comb.clone())),
        comb.clone(),
    )
}

fn read_char_f<'a>(predicate : impl Fn(char) -> bool + Clone) -> impl Parser<'a, char>
{
    move |ind : &ParserInput<'a>| -> PRes<'a, char> {
        match ind.text.chars().nth(0)
        {
            Some(c) =>
            {
                if predicate(c)
                {
                    PRes::Success {
                        val :       c,
                        pos :       FilePos {
                            line :   ind.pos.line,
                            column : ind.pos.column + 1,
                        },
                        remainder : ind.text.get(1..).unwrap(),
                    }
                }
                else
                {
                    PRes::Fail {}
                }
            },
            None => PRes::Fail {},
        }
    }
}

fn char_in_str<'a>(chars_list : &'a str) -> impl Parser<'a, char>
{
    read_char_f(|c| chars_list.chars().any(|f| f == c))
}

fn char_single<'a>(ch : char) -> impl Parser<'a, char> { read_char_f(move |c| c == ch) }

fn keyword(word : &'a str) -> impl Parser<'a, String>
{
    // TODO: Add an error for an empty keyword
    move |ind : &ParserInput<'a>| -> PRes<'a, String> {
        if ind.text.starts_with(word)
        {
            PRes::Success {
                val :       String::from(word),
                pos :       FilePos::new(ind.pos.line, ind.pos.column + word.len()),
                remainder : ind.text.get(word.len()..).unwrap(),
            }
        }
        else
        {
            PRes::Fail {}
        }
    }
}

fn any_char<'a>() -> impl Parser<'a, char> { read_char_f(|_| true) }

fn consume_until<'a, DatIn :Debug+Clone , DatOut :Debug+Clone >(
    p : impl Parser<'a, DatIn>,
    comb : impl Combiner<'a, DatIn, DatIn, DatOut>,
) -> impl Parser<'a, DatOut>
{
    move |ind : &ParserInput<'a>| -> PRes<'a, DatOut> { none_or_many_until(any_char(), p.clone(), comb.clone())(ind) }
}

fn thenr<'a, DatA :Debug+Clone , DatB :Debug+Clone , DatOut :Debug+Clone >(
    a : impl Parser<'a, DatA>,
    b : impl Parser<'a, DatB>,
    comb : impl Combiner<'a, DatA, DatB, DatOut>,
) -> impl Parser<'a, DatOut>
{
    move |ind : &ParserInput<'a>| -> PRes<'a, DatOut> {
        for i in 0..ind.text.len()
        {
            let sap = ind.text.get(..i);
            let sbp = ind.text.get(i..);

            match (sap, sbp)
            {
                (Some(sa), Some(sb)) =>
                {
                    let a_dat = ParserInput {
                        text : sa,
                        pos :  ind.pos,
                    };
                    let b_dat = ParserInput {
                        text : sb,
                        pos :  ind.pos,
                    };

                    let a_res = all(a.clone())(&a_dat);
                    let b_res = a_res.as_ref().and_then(|_| Ok(b(&b_dat)));
                    match (a_res.as_ref(), b_res)
                    {
                        (
                            PRes::Success {
                                ..
                            },
                            PRes::Success {
                                ..
                            },
                        ) => return comb(a_res, b_res),
                        (_, _) =>
                        {},
                    }
                },
                (_, _) =>
                {},
            }
        }

        return PRes::Fail {};
    }
}

// Now a lot more specific
//--- COMMON PARSERS ---//

fn escaped_char<'a>() -> impl Parser<'a, String> { then(char_single('\\'), any_char(), |a, b| gen_comb(a, b,|pl,pr| )) }

fn normal_string<'a>() -> impl Parser<'a,String>
{
    thenr(
        char_single('"'),
        none_or_many_until(any_char(), char_single('"'), lt_comb),
        lt_comb,
    )
}

fn digit<'a>() -> impl Parser<'a, char> { char_in_str("0123456789") }

fn newline<'a>() -> impl Parser<'a>
{
    mod_dat(or(keyword("\r\n"), keyword("\n")), move |od : PRes<'a>| -> PRes<'a> {
        OutDat {
            pos : od.pos.incr_line(),
            ..od
        }
    })
}

fn air<'a, DatT :Debug+Clone >() -> impl Parser<'a> { or(char_in_str(" \t"), newline()) }

fn comma<'a>() -> impl Parser<'a> { char_single(',') }

fn dot<'a>() -> impl Parser<'a> { char_single('.') }

fn in_air<'a, DatT :Debug+Clone >(p : impl Parser<'a, DatT>) -> impl Parser<'a> { then(air(), then(p, air(), l_comb), r_comb) }

fn infix<'a, DatLeft :Debug+Clone , DatParent :Debug+Clone , DatRight :Debug+Clone ,DatOut :Debug+Clone >(
    p_left : impl Parser<'a, DatLeft>,
    p_parent : impl Parser<'a, DatParent>,
    p_right : impl Parser<'a, DatRight>,
) -> impl Parser<'a,DatOut>
{
    mod_val(
        then(p_left, then(p_parent, p_right, lt_comb), rt_comb),
        move |dat : Dat| -> Dat {
            match dat
            {
                Dat::Tree {
                    p: p1,
                    c: c1,
                } => match *p1
                {
                    Dat::Tree {
                        p: p2,
                        c: c2,
                    } => Dat::BTree {
                        p : p2, l : c1, r : c2
                    },
                    _ => panic!("This should not be possible, the Dat type should be a single Parent-Child tree"),
                },

                _ => panic!("This should not be possible, the Dat type should be a single Parent-Child tree"),
            }
        },
    )
}

fn maybe<'a, DatT :Debug+Clone >(p : impl Parser<'a, DatT>) -> impl Parser<'a, DatT> { or(p, always("")) }

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
