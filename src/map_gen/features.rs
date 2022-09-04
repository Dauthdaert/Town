#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Features {
    Tree,
    AppleTree,
}

impl Features {
    pub fn texture(&self) -> u32 {
        match self {
            Features::Tree => 0,
            Features::AppleTree => 7,
        }
    }

    pub fn is_obstacle(&self) -> bool {
        match self {
            Features::Tree => false,
            Features::AppleTree => false,
        }
    }
}
