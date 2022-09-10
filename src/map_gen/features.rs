#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Features {
    Stump,
    Tree,
    AppleTree,
    CoconutStump,
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
            Features::Stump => 3,
            Features::CoconutStump => 6,
        }
    }

    #[allow(clippy::match_single_binding)]
    pub fn is_obstacle(&self) -> bool {
        match self {
            _ => false,
        }
    }

    pub fn is_choppable(&self) -> bool {
        match self {
            Features::Tree | Features::CoconutTree | Features::Cactus => true,
            Features::AppleTree | Features::Bush | Features::Rocks | Features::Stump | Features::CoconutStump => false,
        }
    }
}
