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
            return Err(ErrorKind::ParseError("Couldn't parse into WorkspaceVector".into()).into())
        }

        let x_str = num_strs[0];
        let y_str = num_strs[1];

        let x: i32 = x_str.parse().chain_err(|| "Couldn't parse to int")?;
        let y: i32 = y_str.parse().chain_err(|| "Couldn't parse to int")?;

        Ok(WorkspaceVector::new(x, y))
    }

    pub fn to_str(&self) -> String {
        format!("{}_{}", self.x, self.y)
    }
}

