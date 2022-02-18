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
                line : 0, column : 5
            },
            remainder : "",
        },)
    );
}
