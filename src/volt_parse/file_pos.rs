#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct FilePos
{
    pub line :   usize,
    pub column : usize,
}

impl FilePos
{
    pub fn new(line : usize, column : usize) -> Self
    {
        Self {
            line,
            column,
        }
    }

    pub fn incr_col(&self) -> Self
    {
        Self {
            line :   self.line,
            column : self.column + 1,
        }
    }

    pub fn incr_line(&self) -> Self
    {
        Self {
            line :   self.line + 1,
            column : 0,
        }
    }
}
