use super::path::{PathPiece, UnsafePath, UnsafePathType};
use crate::prelude::*;

const PATH_NAME_CHARS : &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_ ";

// Parses a single dot, failing if there is another dot after this one - does not consume any following characters
pub fn single_dot<'a>() -> impl Parser<'a, String>
{
    then(
        keyword("."),
        not(no_consume(keyword(".")), || String::from("")),
        take_left,
    )
}

pub fn name_parser<'a>() -> impl Parser<'a, PathPiece>
{
    mod_val(
        fail_if(one_or_many(or(char_in_str(PATH_NAME_CHARS), single_dot())), |res| {
            res.val.len() == 1 && res.val[0] == "."
        }),
        |chars| PathPiece::Name(chars.concat()),
    )
}

pub fn current_parser<'a>() -> impl Parser<'a, PathPiece> { mod_val(keyword("."), |_| PathPiece::Current) }

pub fn up_parser<'a>() -> impl Parser<'a, PathPiece> { mod_val(keyword(".."), |_| PathPiece::Up) }

pub fn delim_parser<'a>() -> impl Parser<'a, PathPiece> { mod_val(one_or_many(keyword("/")), |_| PathPiece::Delim) }

pub fn home_parser<'a>() -> impl Parser<'a, PathPiece> { mod_val(keyword("~"), |_| PathPiece::Home) }

pub fn chain_path_piece_parser<'a>() -> impl Parser<'a, PathPiece>
{
    or(or(up_parser(), name_parser()), current_parser())
}

pub fn compress<'a, DatT : PResData>(a : POut<'a, DatT>, b : POut<'a, Option<Vec<DatT>>>) -> POut<'a, Vec<DatT>>
{
    gen_comb(a, b, |l, r : Option<Vec<DatT>>| match r
    {
        Some(mut rs) =>
        {
            rs.insert(0, l);
            rs
        },
        None => vec![l],
    })
}

pub fn local_path_parser<'a>() -> impl Parser<'a, Vec<PathPiece>>
{
    move |ind : &ParserInput<'a>| -> POut<'a, Vec<PathPiece>> {
        then(
            chain_path_piece_parser(),
            maybe(then(delim_parser(), maybe(local_path_parser()), compress)),
            compress,
        )(ind)
    }
}

// TODO: This should probably be removed
pub fn append_start<'a>() -> impl Combiner<'a, PathPiece, Vec<PathPiece>, Vec<PathPiece>>
{
    smcomb(|a, b : Vec<PathPiece>| {
        let mut b2 = b.clone();
        b2.insert(0, a);
        b2
    })
}

// Removes delimeters, home tilda, and current dot (as on this layer it isn't helpful)
fn filter_meta_path(piece : &&PathPiece) -> bool
{
    match piece
    {
        PathPiece::Delim | PathPiece::Home | PathPiece::Current => false,
        _ => true,
    }
}

// Supports absolute paths too
pub fn path_parser<'a>() -> impl Parser<'a, UnsafePath>
{
    // Possibilities:
    // * Local Path
    // * Home
    // * Home -> Local Path
    // * Root
    // * Root -> Local Path

    let root_parser = or(
        then(delim_parser(), local_path_parser(), take_right),
        mod_val(delim_parser(), |_| vec![]),
    );

    mod_val(
        all(or(
            or(
                // Paths that start with a '~' (relative to HOME - which are technically still absolute paths)
                mod_val(
                    or(
                        then(home_parser(), root_parser.clone(), take_right),
                        mod_val(home_parser(), |_| vec![]),
                    ),
                    |pieces| (UnsafePathType::Home, pieces),
                ),
                // Paths that start with a delimiter (absolute paths)
                mod_val(root_parser, |pieces| (UnsafePathType::Absolute, pieces)),
            ),
            // Relative paths
            mod_val(local_path_parser(), |pieces| (UnsafePathType::Relative, pieces)),
        )),
        |(path_type, pieces)| UnsafePath {
            pieces : pieces
                .iter()
                .filter(filter_meta_path)
                .map(|v| v.to_owned())
                .collect::<Vec<_>>(),
            path_type,
        },
    )
}
