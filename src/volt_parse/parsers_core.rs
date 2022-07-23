use super::{
    combiner::{smcomb, Combiner},
    combiners::{take_right, tuple_left_char_vec_to_str},
    defs::{Or2, Or3},
    file_pos::FilePos,
    parser::{PErr, POut, PRes, PResData, Parser, ParserInput},
    parsers_core_ors::or2,
};
use std::fmt::Display;

#[inline]
pub fn then<'a, DatA : PResData, DatB : PResData, DatOut : PResData>(
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

#[inline]
pub fn mod_out<'a, DatIn : PResData, DatOut : PResData>(
    p : impl Parser<'a, DatIn>,
    f : impl Fn(PRes<'a, DatIn>) -> POut<'a, DatOut> + Clone,
) -> impl Parser<'a, DatOut>
{
    move |ind : &ParserInput<'a>| -> POut<'a, DatOut> { p(ind).and_then(&f) }
}

#[inline]
pub fn mod_dat<'a, DatIn : PResData, DatOut : PResData>(
    p : impl Parser<'a, DatIn>,
    f : impl Fn(PRes<'a, DatIn>) -> PRes<'a, DatOut> + Clone,
) -> impl Parser<'a, DatOut>
{
    mod_out(p, move |v : PRes<'a, DatIn>| -> POut<'a, DatOut> { Ok(f(v)) })
}

#[inline]
pub fn mod_val<'a, DatIn : PResData, DatOut : PResData>(
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

#[inline]
pub fn replace_val<'a, DatIn : PResData, DatOut : PResData>(
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

// pub trait ParserGen<'a, DatT> = FnOnce() -> impl Parser<'a, DatT>;

#[inline]
pub fn defer<'a, DatT, G, R>(p_fn : G) -> impl Parser<'a, DatT>
where
    DatT : PResData,
    G : Fn() -> R + Clone,
    R : Parser<'a, DatT>,
{
    move |ind : &ParserInput<'a>| -> POut<'a, DatT> { p_fn()(ind) }
}

#[inline]
pub fn succeed_if<'a, DatT : PResData>(
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

#[inline]
pub fn fail_if<'a, DatT : PResData>(
    p : impl Parser<'a, DatT>,
    f : impl Fn(&PRes<'a, DatT>) -> bool + Clone,
) -> impl Parser<'a, DatT>
{
    succeed_if(p, move |r| !f(r))
}

#[inline]
pub fn all<'a, DatT : PResData>(p : impl Parser<'a, DatT>) -> impl Parser<'a, DatT>
{
    succeed_if(p, |r| r.remainder.is_empty())
}

// TODO: Replace/Modify fail message

// TODO: This may not work, the lifetimes probably aren't right
#[inline]
pub fn always<'a, DatT : PResData>(default_fn : impl Fn() -> DatT + Clone) -> impl Parser<'a, DatT>
{
    move |ind : &ParserInput<'a>| -> POut<'a, DatT> {
        Ok(PRes {
            val :       default_fn(),
            pos :       ind.pos,
            remainder : ind.text,
        })
    }
}

#[inline]
pub fn not<'a, DatT : PResData>(
    p : impl Parser<'a, DatT>,
    default_fn : impl Fn() -> DatT + Clone,
) -> impl Parser<'a, DatT>
{
    move |ind : &ParserInput<'a>| -> POut<'a, DatT> {
        match p(ind)
        {
            Ok(_v) => Err(PErr {
                pos : ind.pos
            }),
            Err(_v) => Ok(PRes {
                val :       default_fn(),
                pos :       ind.pos,
                remainder : ind.text,
            }),
        }
    }
}

#[inline]
pub fn or<'a, DatT : PResData>(a : impl Parser<'a, DatT>, b : impl Parser<'a, DatT>) -> impl Parser<'a, DatT>
{
    move |ind : &ParserInput<'a>| -> POut<'a, DatT> {
        match a(ind)
        {
            Ok(a) => Ok(PRes {
                val :       a.val,
                pos :       a.pos,
                remainder : a.remainder,
            }),
            Err(_a) => match b(ind)
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

#[inline]
pub fn either_or<'a, DatA : PResData, DatB : PResData, DatOut : PResData>(
    a : impl Parser<'a, DatA>,
    b : impl Parser<'a, DatB>,
    comb : impl Combiner<'a, DatA, DatB, DatOut>,
) -> impl Parser<'a, Or3<DatOut, DatA, DatB>>
{
    mod_val(or2(then(a.clone(), b.clone(), comb), or2(a, b)), |v| match v
    {
        Or2::A(l) => Or3::A(l),
        Or2::B(r) => match r
        {
            Or2::A(rl) => Or3::B(rl),
            Or2::B(rr) => Or3::C(rr),
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

#[inline]
pub fn chain_select<'a, DatT : PResData>(ps : Vec<impl Parser<'a, DatT>>, index : usize) -> impl Parser<'a, DatT>
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
                        current = take_right(Ok(c_val.clone()), ps[i](&c_val.to_in()));
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

#[inline]
pub fn one_or_none<'a, DatT : PResData>(p : impl Parser<'a, DatT>) -> impl Parser<'a, Option<DatT>>
{
    mod_val(or2(p, always(|| ())), |v : Or2<DatT, ()>| -> Option<DatT> {
        match v
        {
            Or2::A(lv) => Some(lv),
            Or2::B(_) => None,
        }
    })
}

#[inline]
pub fn none_or_many<'a, DatT : PResData>(p : impl Parser<'a, DatT>) -> impl Parser<'a, Vec<DatT>>
{
    move |ind : &ParserInput<'a>| -> POut<'a, Vec<DatT>> {
        mod_val(
            one_or_none(then(
                p.clone(),
                none_or_many(p.clone()),
                smcomb(|a : DatT, mut b : Vec<DatT>| {
                    b.insert(0, a);
                    b
                }),
            )),
            |opt_list| opt_list.unwrap_or_else(|| vec![]),
        )(ind)
    }
}

#[inline]
pub fn one_or_many<'a, DatT : PResData>(p : impl Parser<'a, DatT>) -> impl Parser<'a, Vec<DatT>>
{
    succeed_if(none_or_many(p), move |v : &PRes<'a, Vec<DatT>>| v.val.len() > 0)
}

#[inline]
pub fn none_or_many_until<'a, DatA : PResData, DatB : PResData, DatOut : PResData>(
    pa : impl Parser<'a, DatA>,
    pb : impl Parser<'a, DatB>,
    comb : impl Combiner<'a, Vec<DatA>, DatB, DatOut>,
) -> impl Parser<'a, DatOut> // We either return the stop type or the combined type
{
    then(none_or_many(pa), pb, comb)
}

#[inline]
pub fn one_or_many_until<'a, DatA : PResData, DatB : PResData, DatOut : PResData>(
    pa : impl Parser<'a, DatA>,
    pb : impl Parser<'a, DatB>,
    comb : impl Combiner<'a, Vec<DatA>, DatB, DatOut>,
) -> impl Parser<'a, DatOut>
{
    then(one_or_many(pa), pb, comb)
}

#[inline]
pub fn read_char_f<'a>(predicate : impl Fn(char) -> bool + Clone) -> impl Parser<'a, char>
{
    move |ind : &ParserInput<'a>| -> POut<'a, char> {
        match ind.text.chars().nth(0)
        {
            Some(c) =>
            {
                if predicate(c)
                {
                    Ok(PRes {
                        val :       c,
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

#[inline]
pub fn char_in_str<'a>(chars_list : &'a str) -> impl Parser<'a, char>
{
    read_char_f(|c| chars_list.chars().any(|f| f == c))
}

#[inline]
pub fn char_single<'a>(ch : char) -> impl Parser<'a, char> { read_char_f(move |c| c == ch) }

#[inline]
pub fn keyword<'a>(word : &'a str) -> impl Parser<'a, String>
{
    // TODO: Add an error for an empty keyword
    move |ind : &ParserInput<'a>| -> POut<'a, String> {
        if ind.text.starts_with(word)
        {
            // TODO: This should not be handled here, as the keyword could contain either
            // CR/LF
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

#[inline]
pub fn any_char<'a>() -> impl Parser<'a, char> { read_char_f(|_| true) }

#[inline]
pub fn consume_chars_until<'a, DatEnd : PResData>(p : impl Parser<'a, DatEnd>) -> impl Parser<'a, (String, DatEnd)>
{
    none_or_many_until(any_char(), p.clone(), tuple_left_char_vec_to_str)
}

// A right-handed then, try not to use if not necessary
#[inline]
pub fn thenr<'a, DatA : PResData, DatB : PResData, DatOut : PResData>(
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
                (Some(sa), Some(_sb)) =>
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
                                Err(_b_err) =>
                                {},
                            }
                        },
                        Err(_a_err) =>
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

// Parses a block of text without incrementing the position, useful for
// look-ahead operations
#[inline]
pub fn no_consume<'a, DatT : PResData>(p : impl Parser<'a, DatT>) -> impl Parser<'a, DatT>
{
    move |ind : &ParserInput<'a>| -> POut<'a, DatT> {
        let always_res = always(|| ())(ind)?;

        p(ind).map(|p_res| -> PRes<'a, DatT> {
            PRes {
                val :       p_res.val,
                pos :       always_res.pos,
                remainder : always_res.remainder,
            }
        })
    }
}

#[inline]
pub fn display<'a, DatT : PResData + Display>(p : impl Parser<'a, DatT>) -> impl Parser<'a, String>
{
    mod_val(p, |v| format!("{}", v))
}
