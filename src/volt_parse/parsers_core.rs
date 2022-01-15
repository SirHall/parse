use std::fmt::Debug;

use super::{
    combiner::Combiner,
    combiners::r_comb,
    defs::{Either2, Either3},
    file_pos::FilePos,
    parser::{PErr, POut, PRes, Parser, ParserInput},
};

pub fn then<'a, DatA : Debug + Clone, DatB : Debug + Clone, DatOut : Debug + Clone>(
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

pub fn mod_out<'a, DatIn : Debug + Clone, DatOut : Debug + Clone>(
    p : impl Parser<'a, DatIn>,
    f : impl Fn(PRes<'a, DatIn>) -> POut<'a, DatOut> + Clone,
) -> impl Parser<'a, DatOut>
{
    move |ind : &ParserInput<'a>| -> POut<'a, DatOut> {
        p(ind).and_then(&f) // TODO: See if this incurrs a slowdown over a 'more
                            // basic' implementation
    }
}

pub fn mod_dat<'a, DatIn : Debug + Clone, DatOut : Debug + Clone>(
    p : impl Parser<'a, DatIn>,
    f : impl Fn(PRes<'a, DatIn>) -> PRes<'a, DatOut> + Clone,
) -> impl Parser<'a, DatOut>
{
    mod_out(p, move |v : PRes<'a, DatIn>| -> POut<'a, DatOut> { Ok(f(v)) })
}

pub fn mod_val<'a, DatIn : Debug + Clone, DatOut : Debug + Clone>(
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

pub fn replace_val<'a, DatIn : Debug + Clone, DatOut : Debug + Clone>(
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

pub fn succeed_if<'a, DatT : Debug + Clone>(
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

pub fn fail_if<'a, DatT : Debug + Clone>(
    p : impl Parser<'a, DatT>,
    f : impl Fn(&PRes<'a, DatT>) -> bool + Clone,
) -> impl Parser<'a, DatT>
{
    succeed_if(p, move |r| !f(r))
}

pub fn all<'a, DatT : Debug + Clone>(p : impl Parser<'a, DatT>) -> impl Parser<'a, DatT>
{
    succeed_if(p, |r| r.remainder.is_empty())
}

// TODO: Replace/Modify fail message

// TODO: This may not work, the lifetimes probably aren't right
pub fn always<'a, DatT : Debug + Clone>(default_fn : impl Fn() -> DatT + Clone) -> impl Parser<'a, DatT>
{
    move |ind : &ParserInput<'a>| -> POut<'a, DatT> {
        Ok(PRes {
            val :       default_fn(),
            pos :       ind.pos,
            remainder : ind.text,
        })
    }
}

pub fn not<'a, DatT : Debug + Clone>(
    p : impl Parser<'a, DatT>,
    default_fn : impl Fn() -> DatT + Clone,
) -> impl Parser<'a, DatT>
{
    move |ind : &ParserInput<'a>| -> POut<'a, DatT> {
        match p(ind)
        {
            Ok(v) => Err(PErr {
                pos : ind.pos
            }),
            Err(v) => Ok(PRes {
                val :       default_fn(),
                pos :       ind.pos,
                remainder : ind.text,
            }),
        }
    }
}

pub fn or<'a, DatT : Debug + Clone>(a : impl Parser<'a, DatT>, b : impl Parser<'a, DatT>) -> impl Parser<'a, DatT>
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

pub fn or_diff<'a, DatA : Debug + Clone, DatB : Debug + Clone>(
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

pub fn either_or<'a, DatA : Debug + Clone, DatB : Debug + Clone, DatOut : Debug + Clone>(
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

pub fn chain_select<'a, DatT : Debug + Clone>(ps : Vec<impl Parser<'a, DatT>>, index : usize) -> impl Parser<'a, DatT>
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

pub fn or_chain<'a, DatT : Debug + Clone>(ps : Vec<impl Parser<'a, DatT>>) -> impl Parser<'a, DatT>
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

pub fn one_or_none<'a, DatT : Debug + Clone>(p : impl Parser<'a, DatT>) -> impl Parser<'a, Option<DatT>>
{
    mod_val(or_diff(p, always(|| ())), |v : Either2<DatT, ()>| -> Option<DatT> {
        match v
        {
            Either2::Left(lv) => Some(lv),
            Either2::Right(_) => None,
        }
    })
}

pub fn none_or_many<'a, DatT : Debug + Clone>(p : impl Parser<'a, DatT>) -> impl Parser<'a, Vec<DatT>>
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

pub fn one_or_many<'a, DatT : Debug + Clone>(p : impl Parser<'a, DatT>) -> impl Parser<'a, Vec<DatT>>
{
    succeed_if(none_or_many(p), move |v : &PRes<'a, Vec<DatT>>| v.val.len() == 0)
}

pub fn none_or_many_until<'a, DatA : Debug + Clone, DatB : Debug + Clone, DatOut : Debug + Clone>(
    pa : impl Parser<'a, DatA>,
    pb : impl Parser<'a, DatB>,
    comb : impl Combiner<'a, Vec<DatA>, DatB, DatOut>,
) -> impl Parser<'a, DatOut> // We either return the stop type or the combined type
{
    then(none_or_many(pa), pb, comb)
}

pub fn one_or_many_until<'a, DatA : Debug + Clone, DatB : Debug + Clone, DatOut : Debug + Clone>(
    pa : impl Parser<'a, DatA>,
    pb : impl Parser<'a, DatB>,
    comb : impl Combiner<'a, Vec<DatA>, DatB, DatOut>,
) -> impl Parser<'a, DatOut>
{
    then(one_or_many(pa), pb, comb)
}

pub fn read_char_f<'a>(predicate : impl Fn(char) -> bool + Clone) -> impl Parser<'a, String>
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

pub fn char_in_str<'a>(chars_list : &'a str) -> impl Parser<'a, String>
{
    read_char_f(|c| chars_list.chars().any(|f| f == c))
}

pub fn char_single<'a>(ch : char) -> impl Parser<'a, String> { read_char_f(move |c| c == ch) }

pub fn keyword(word : &'a str) -> impl Parser<'a, String>
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

pub fn any_char<'a>() -> impl Parser<'a, String> { read_char_f(|_| true) }

pub fn consume_until<'a, DatEnd : Debug + Clone, DatOut : Debug + Clone>(
    p : impl Parser<'a, DatEnd>,
    comb : impl Combiner<'a, String, DatEnd, DatOut> + Clone,
) -> impl Parser<'a, DatOut>
{
    none_or_many_until(
        any_char(),
        p.clone(),
        move |a : POut<'a, Vec<String>>, b : POut<'a, DatEnd>| {
            comb(
                a.map(|a_succ| PRes {
                    val :       a_succ.val.join(""),
                    pos :       a_succ.pos,
                    remainder : a_succ.remainder,
                }),
                b,
            )
        },
    )
}

pub fn thenr<'a, DatA : Debug + Clone, DatB : Debug + Clone, DatOut : Debug + Clone>(
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
