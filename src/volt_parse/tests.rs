mod vfs;

use crate::prelude::*;

#[test]
fn then_test()
{
    let res = (then(keyword("hi"), keyword("bob"), left_right))(&ParserInput::new("hibob"));

    println!("{:#?}", res);

    assert_eq!(
        res,
        Ok(PRes {
            val :       (String::from("hi"), String::from("bob")),
            pos :       FilePos {
                line : 1, column : 5
            },
            remainder : "",
        },)
    );
}

#[test]
fn multi_element_parsers_test()
{
    let res = one_or_many(char_single('a'))(&ParserInput::new("aaaaabb123"));

    println!("{:#?}", res);

    assert_eq!(
        res,
        Ok(PRes {
            val :       vec![String::from("a"); 5],
            pos :       FilePos {
                line : 1, column : 5
            },
            remainder : "bb123",
        },)
    );
}

#[test]
fn no_consume_test()
{
    let res = then(
        then(one_or_many(char_single('=')), no_consume(char_single('@')), left_right),
        then(char_single('@'), one_or_many(char_single('-')), left_right),
        left_right,
    )(&ParserInput::new("=======@--------"));

    println!("{:#?}", res);

    assert_eq!(
        res,
        Ok(PRes {
            val :       (
                (vec![String::from("="); 7], String::from("@")),
                (String::from("@"), vec![String::from("-"); 8]),
            ),
            pos :       FilePos {
                line :   1,
                column : 16,
            },
            remainder : "",
        },)
    );
}

#[test]
fn defer_test()
{
    let res =
        defer(|| then(defer(|| keyword("abc")), defer(|| keyword("123")), left_right))(&ParserInput::new("abc123"));

    println!("{:#?}", res);

    assert_eq!(
        res,
        Ok(PRes {
            val :       (String::from("abc"), String::from("123"),),
            pos :       FilePos {
                line : 1, column : 6
            },
            remainder : "",
        },)
    );
}
