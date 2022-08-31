use hierarchical_pathfinding::prelude::*;

type Point = (usize, usize);

#[derive(Clone, Copy, Debug)]
pub struct EuclideanNeighborhood {
    width: usize,
    height: usize,
}

impl EuclideanNeighborhood {
    /// Creates a new EuclideanNeighborhood.
    ///
    /// 'width' and 'height' are the size of the Grid to move on.
    pub fn new(width: usize, height: usize) -> Self {
        EuclideanNeighborhood { width, height }
    }
}

impl Neighborhood for EuclideanNeighborhood {
    fn get_all_neighbors(&self, point: Point, target: &mut Vec<Point>) {
        let (width, height) = (self.width, self.height);

        #[rustfmt::skip]
        static ALL_DELTAS: [(isize, isize); 8] = [(0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1)];

        for (dx, dy) in ALL_DELTAS.iter() {
            let x = point.0 as isize + dx;
            let y = point.1 as isize + dy;
            if x >= 0 && x < width as isize && y >= 0 && y < height as isize {
                target.push((x.try_into().unwrap(), y.try_into().unwrap()))
            }
        }
    }

    fn heuristic(&self, point: Point, goal: Point) -> usize {
        let d_x = point.0.abs_diff(goal.0) as f32;
        let d_y = point.1.abs_diff(goal.1) as f32;
        ((d_x + d_y + (1.42 - 2.0) * f32::min(d_x, d_y)) * 100.0) as usize
    }
}
