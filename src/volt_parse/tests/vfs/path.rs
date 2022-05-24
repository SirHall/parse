use super::parser::path_parser;
use crate::prelude::*;
use anyhow::bail;
use anyhow::Result;
use itertools::Itertools;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathPiece
{
    Name(String),
    Current,
    Up,
    Delim,
    Home,
}

// A path that is fully validated, and is always absolute
#[derive(Debug, Clone)]
pub struct SafePath
{
    // The path must be completely absolute, and thus cannot have anything except names
    pub pieces : Vec<String>,
}

// This is to denote a path that has been 'parsed' but is not yet safe to perform on-disk file access operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnsafePath
{
    pub pieces :    Vec<PathPiece>,
    pub path_type : UnsafePathType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnsafePathType
{
    Relative,
    Absolute, // This path begins at home
    Home,
}

impl From<SafePath> for UnsafePath
{
    fn from(sp : SafePath) -> Self
    {
        UnsafePath {
            pieces :    sp.pieces.iter().map(|name| PathPiece::Name(name.to_owned())).collect(),
            path_type : UnsafePathType::Relative,
        }
    }
}

impl UnsafePath
{
    pub fn break_path(path : &str) -> Result<UnsafePath>
    {
        match path_parser()(&ParserInput::new(path))
        {
            Ok(pres) => Ok(pres.val),
            Err(perr) => bail!("{:#?}", perr),
        }
    }

    pub fn cannonize(base : &SafePath, tail : UnsafePath) -> Result<SafePath>
    {
        let mut new_path = base.clone();

        for piece in tail.pieces.as_slice()
        {
            match piece
            {
                PathPiece::Name(name) => new_path.pieces.push(name.clone()),
                PathPiece::Up =>
                {
                    if new_path.pieces.is_empty()
                    {
                        //// We don't bail here as apparently standard Unix behaviour is simply to clamp to root
                        // Actually we do, as this can hide some pretty nasty bugs in the user's own code
                        bail!(format!(
                            "Combining base path {:?} with {} attempts to move outside of the root directory",
                            base, tail
                        ));
                    }
                    else
                    {
                        new_path.pieces.pop();
                    }
                },
                // These should never happen, but may if the user has been very sneaky
                PathPiece::Current => bail!("'.' should not exist in an unsafe path"),
                PathPiece::Delim => bail!("'/' should not exist in an unsafe path"),
                PathPiece::Home => todo!("'~' should not exist in an unsafe path"),
            }
        }

        Ok(new_path)
    }

    pub fn cannonize_environment(home : &SafePath, working_dir : &SafePath, tail : UnsafePath) -> Result<SafePath>
    {
        match tail.path_type
        {
            UnsafePathType::Relative => Self::cannonize(working_dir, tail),
            UnsafePathType::Absolute => Self::cannonize(&SafePath::root(), tail),
            UnsafePathType::Home => Self::cannonize(home, tail),
        }
    }
}

impl Display for UnsafePath
{
    fn fmt(&self, f : &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let prefix = match self.path_type
        {
            UnsafePathType::Relative => "./",
            UnsafePathType::Absolute => "/",
            UnsafePathType::Home => "~/",
        };
        let message = String::from(prefix) + self.pieces.iter().map(|p| p.to_str()).join("/").as_str();
        f.write_str(message.as_str())
    }
}

impl SafePath
{
    pub fn root() -> SafePath
    {
        SafePath {
            pieces : vec![]
        }
    }
}

impl Default for SafePath
{
    fn default() -> Self { Self::root() }
}

impl Display for SafePath
{
    fn fmt(&self, f : &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let message = String::from("/") + self.pieces.iter().join("/").as_str();
        f.write_str(message.as_str())
    }
}

impl PathPiece
{
    pub fn to_str<'a>(&'a self) -> &'a str
    {
        match self
        {
            PathPiece::Name(name) => name,
            PathPiece::Current => ".",
            PathPiece::Up => "..",
            PathPiece::Delim => "/",
            PathPiece::Home => "~",
        }
    }
}

impl Display for PathPiece
{
    fn fmt(&self, f : &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str(self.to_str()) }
}
