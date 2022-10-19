#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Features {
    Stump,
    Tree,
    AppleTree,
    CoconutStump,
    CoconutTree,
    Cactus,
    Bush,
    Rocks,
    Stone,
    Wall,
    Floor,
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
            Features::Stone => 18,
            Features::Wall => 24,
            Features::Floor => 21,
        }
    }

    pub fn is_obstacle(&self) -> bool {
        matches!(self, Features::Stone | Features::Wall)
    }

    pub fn is_choppable(&self) -> bool {
        matches!(self, Features::Tree | Features::CoconutTree | Features::Cactus)
    }

    pub fn is_destructable(&self) -> bool {
        matches!(
            self,
            Features::Stone | Features::Rocks | Features::Wall | Features::Floor
        )
    }
}
