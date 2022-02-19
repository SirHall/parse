use crate::prelude::*;

#[test]
fn then_test()
{
    let res = (then(keyword("hi"), keyword("bob"), lr_comb))(&ParserInput::new("hibob"));

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
        then(one_or_many(char_single('=')), no_consume(char_single('@')), lr_comb),
        then(char_single('@'), one_or_many(char_single('-')), lr_comb),
        lr_comb,
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
