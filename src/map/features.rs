use super::auto_tile::AutoTileCategory;

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
    Door,
    Road,
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
            Features::Door => "Door",
            Features::Road => "Road",
        }
    }

    pub fn auto_tile_category(&self) -> AutoTileCategory {
        match self {
            Features::Wall | Features::Door => AutoTileCategory::Wall,
            _ => AutoTileCategory::None,
        }
    }

    pub fn cost(&self) -> Option<isize> {
        match self {
            Features::StoneWall | Features::Wall => Some(-1),
            Features::Road => Some(70),
            _ => None,
        }
    }

    pub fn is_obstacle(&self) -> bool {
        self.cost().map_or(false, |c| c == -1)
    }

    pub fn is_choppable(&self) -> bool {
        matches!(self, Features::Tree | Features::CoconutTree | Features::Cactus)
    }

    pub fn is_mineable(&self) -> bool {
        matches!(self, Features::StoneWall | Features::Rocks)
    }
}
