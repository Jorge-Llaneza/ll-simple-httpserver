use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct uri {
    pub path: String,
    pub query: Option<String>,
    // TODO pub fragment: Option<String>,
}

impl FromStr for uri {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for component in s.split('?') {
        }
    }
}

impl uri {
    fn path_from_str(str: &str) -> Option<String> {
        let mut parts = str.to_owned();
        for
    }
}