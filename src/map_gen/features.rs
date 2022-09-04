#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Features {
    Tree,
    AppleTree,
    CoconutTree,
    Cactus,
    Bush,
    Rocks,
}

impl Features {
    pub fn texture(&self) -> u32 {
        match self {
            Features::Tree => 4,
            Features::AppleTree => 1,
            Features::CoconutTree => 7,
            Features::Cactus => 12,
            Features::Bush => 10,
            Features::Rocks => 15,
        }
    }

    pub fn is_obstacle(&self) -> bool {
        match self {
            Features::Tree => false,
            Features::AppleTree => false,
            Features::CoconutTree => false,
            Features::Cactus => false,
            Features::Bush => false,
            Features::Rocks => false,
        }
    }
}
