use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Uri {
    pub path: String,
    pub query: Option<String>,
    // TODO pub fragment: Option<String>,
}

impl FromStr for Uri {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first_split = s.trim().split('?');
        if first_split.clone().count() > 2 { return Err(()); }
        let split_vec = first_split.collect::<Vec<&str>>();
        if split_vec.len() == 1 {
            return Ok(Uri {
                path: split_vec[0].to_string(),
                query: None,
            })
        }
        let query = match split_vec[1] {
            "" => None,
            _ => Some(split_vec[0].to_string())
        };
        if split_vec[1].len() == 0 { return Err(()); }

        Ok(Uri {
            path: split_vec[0].to_string(),
            query,
        })
    }
}
