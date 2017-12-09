use std::ops;

use error::*;

#[derive(Debug)]
pub struct WorkspaceVector {
    pub x: i32,
    pub y: i32
}

impl WorkspaceVector {
    pub fn new(x: i32, y: i32) -> WorkspaceVector {
        return WorkspaceVector {x, y}
    }

    pub fn from_str(s: &str) -> Result<WorkspaceVector> {
        let num_strs: Vec<&str> = s.split("_").collect();

        if num_strs.len() != 2 {
            return Err(ErrorKind::ParseError(
                    format!("Couldn't find exactly two parts of vector string: {}", s)).into())
        }

        let x_str = num_strs[0];
        let y_str = num_strs[1];

        let x: i32 = WorkspaceVector::str_to_int(x_str)?;
        let y: i32 = WorkspaceVector::str_to_int(y_str)?;

        Ok(WorkspaceVector::new(x, y))
    }

    pub fn to_str(&self) -> String {
        format!(
            "{}_{}",
            WorkspaceVector::int_to_str(self.x),
            WorkspaceVector::int_to_str(self.y))
    }

    fn int_to_str(i: i32) -> String {
        if i >= 0 {
            format!("{}", i)
        } else {
            format!("n{}", -i)
        }
    }

    fn str_to_int(s: &str) -> Result<i32> {
        let first_char = s.chars().next()
            .chain_err(|| format!("Can't get first character of int string: {}", s))?;

        if first_char == 'n' {
            (&s[1..]).parse()
                .chain_err(|| format!("Couldn't parse to negative int: {}", s))
                .map(|i: i32| -i)
        } else {
            s.parse()
                .chain_err(|| format!("Couldn't parse to int: {}", s))
        }
    }
}

impl PartialEq for WorkspaceVector {
    fn eq (&self, other: &WorkspaceVector) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<'a, 'b> ops::Add<&'b WorkspaceVector> for &'a WorkspaceVector {
    type Output = WorkspaceVector;

    fn add(self, other: &WorkspaceVector) -> WorkspaceVector {
        WorkspaceVector::new(self.x + other.x, self.y + other.y)
    }
}

