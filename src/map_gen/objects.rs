#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Objects {
    Tree,
    AppleTree,
}

impl Objects {
    pub fn texture(&self) -> u32 {
        match self {
            Objects::Tree => 0,
            Objects::AppleTree => 7,
        }
    }

    pub fn is_obstacle(&self) -> bool {
        match self {
            Objects::Tree => false,
            Objects::AppleTree => false,
        }
    }
}
