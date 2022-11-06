#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Features {
    TreeStump,
    Tree,
    AppleTree,
    AppleTreeEmpty,
    CoconutTreeStump,
    CoconutTree,
    Cactus,
    BerryBush,
    BerryBushEmpty,
    Rocks,
    StoneWall,
    Wall,
    Floor,
}

impl Features {
    pub fn tile_name(&self) -> &str {
        match self {
            Features::TreeStump => "TreeStump",
            Features::Tree => "Tree",
            Features::AppleTree => "AppleTree",
            Features::AppleTreeEmpty => "AppleTreeEmpty",
            Features::CoconutTreeStump => "CoconutTreeStump",
            Features::CoconutTree => "CoconutTree",
            Features::Cactus => "Cactus",
            Features::BerryBush => "BerryBush",
            Features::BerryBushEmpty => "BerryBushEmpty",
            Features::Rocks => "Rocks",
            Features::StoneWall => "StoneWall",
            Features::Wall => "Wall",
            Features::Floor => "Floor",
        }
    }

    pub fn is_obstacle(&self) -> bool {
        matches!(self, Features::StoneWall | Features::Wall)
    }

    pub fn is_choppable(&self) -> bool {
        matches!(self, Features::Tree | Features::CoconutTree | Features::Cactus)
    }

    pub fn is_mineable(&self) -> bool {
        matches!(self, Features::StoneWall | Features::Rocks)
    }
}
