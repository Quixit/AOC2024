use std::collections::HashSet;

pub struct Results {
    pub peaks: HashSet<(usize, usize)>,
    pub trails: HashSet<Vec<(usize, usize)>>
}

impl Results {
    pub fn new() -> Results {
        Results{
            peaks: HashSet::new(),
            trails: HashSet::new()
        }
    }
}