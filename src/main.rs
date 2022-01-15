use std::{char, fmt::Debug};

// use rayon::prelude::*;

// Text position in the original file

// Now a lot more specific
//--- COMMON PARSERS ---//

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
