use super::path::SafePath;
use crate::prelude::*;
use crate::volt_parse::tests::vfs::parser::path_parser;
use crate::volt_parse::tests::vfs::path::PathPiece;
use crate::volt_parse::tests::vfs::path::UnsafePath;
use crate::volt_parse::tests::vfs::path::UnsafePathType;
use anyhow::anyhow;
use anyhow::Result;

#[test]
fn test_path_parse_relative()
{
    let res = path_parser()(&ParserInput::new("my/normal/relative/path.txt"));

    println!("{:?}", res);

    assert_eq!(
        res,
        Ok(PRes {
            val :       UnsafePath {
                pieces :    vec![
                    PathPiece::Name(String::from("my")),
                    PathPiece::Name(String::from("normal")),
                    PathPiece::Name(String::from("relative")),
                    PathPiece::Name(String::from("path.txt"))
                ],
                path_type : UnsafePathType::Relative,
            },
            pos :       FilePos {
                line :   1,
                column : 27,
            },
            remainder : "",
        })
    );
}

#[test]
fn test_path_parse_absolute()
{
    let res = path_parser()(&ParserInput::new("/home/username/etc.txt"));

    println!("{:?}", res);

    assert_eq!(
        res,
        Ok(PRes {
            val :       UnsafePath {
                pieces :    vec![
                    PathPiece::Name(String::from("home")),
                    PathPiece::Name(String::from("username")),
                    PathPiece::Name(String::from("etc.txt"))
                ],
                path_type : UnsafePathType::Absolute,
            },
            pos :       FilePos {
                line :   1,
                column : 22,
            },
            remainder : "",
        })
    );
}

#[test]
fn test_path_parse_home_full()
{
    let res = path_parser()(&ParserInput::new("~/.config/service.toml"));

    println!("{:?}", res);

    assert_eq!(
        res,
        Ok(PRes {
            val :       UnsafePath {
                pieces :    vec![
                    PathPiece::Name(String::from(".config")),
                    PathPiece::Name(String::from("service.toml"))
                ],
                path_type : UnsafePathType::Home,
            },
            pos :       FilePos {
                line :   1,
                column : 22,
            },
            remainder : "",
        })
    );
}

#[test]
fn test_path_parse_home_only()
{
    let res1 = path_parser()(&ParserInput::new("~"));

    println!("{:?}", res1);

    assert_eq!(
        res1,
        Ok(PRes {
            val :       UnsafePath {
                pieces :    vec![],
                path_type : UnsafePathType::Home,
            },
            pos :       FilePos {
                line : 1, column : 1
            },
            remainder : "",
        })
    );
}

#[test]
fn test_path_parse_up()
{
    let res = path_parser()(&ParserInput::new("~/.."));

    println!("{:?}", res);

    assert_eq!(
        res,
        Ok(PRes {
            val :       UnsafePath {
                pieces :    vec![PathPiece::Up],
                path_type : UnsafePathType::Home,
            },
            pos :       FilePos {
                line : 1, column : 4
            },
            remainder : "",
        })
    );
}

#[test]
fn test_path_parse_route()
{
    let res = path_parser()(&ParserInput::new("~/../././//test2./..//username/etc.txt"));

    println!("{:?}", res);

    assert_eq!(
        res,
        Ok(PRes {
            val :       UnsafePath {
                pieces :    vec![
                    PathPiece::Up,
                    PathPiece::Name(String::from("test2.")),
                    PathPiece::Up,
                    PathPiece::Name(String::from("username")),
                    PathPiece::Name(String::from("etc.txt"))
                ],
                path_type : UnsafePathType::Home,
            },
            pos :       FilePos {
                line :   1,
                column : 38,
            },
            remainder : "",
        })
    );
}

#[test]
fn test_path_forbid_name_with_double_dots()
{
    let res = path_parser()(&ParserInput::new("an/inva..lid/name.txt"));

    println!("{:?}", res);

    assert_eq!(
        res,
        Err(PErr {
            pos : FilePos {
                line : 1, column : 0
            },
        })
    );
}

#[test]
fn test_path_forbid_name_with_double_dots_2()
{
    let res = path_parser()(&ParserInput::new("..invalid_2"));

    println!("{:?}", res);

    assert_eq!(
        res,
        Err(PErr {
            pos : FilePos {
                line : 1, column : 0
            },
        })
    );
}

#[test]
fn test_path_cannonization() -> Result<()>
{
    let home = path_parser()(&ParserInput::new("/home/username/"))
        .map_err(|err| anyhow!("{:?}", err))?
        .val;

    let home_rooted = UnsafePath::cannonize(&SafePath::root(), home)?;

    let path : UnsafePath = path_parser()(&ParserInput::new("to/the/work.txt"))
        .map_err(|err| anyhow!("{:?}", err))?
        .val;

    Ok(())
}
