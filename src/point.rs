use enum_iterator::Sequence;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Default)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl Point {
    //! Rectilinear distance is also known as L1, taxicab or manhattan distance
    pub fn rectilinear_dist(&self, p: &Point) -> usize {
        (isize::abs(self.x - p.x) + isize::abs(self.y - p.y)) as usize
    }

    pub fn neighbour(&self, dir: Direction) -> Point {
        match dir {
            Direction::North => Point {
                x: self.x,
                y: self.y - 1,
            },
            Direction::East => Point {
                x: self.x + 1,
                y: self.y,
            },
            Direction::South => Point {
                x: self.x,
                y: self.y + 1,
            },
            Direction::West => Point {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Point3WithVel {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
}

impl Point3WithVel {
    pub fn from_vec(v: Vec<f64>) -> Point3WithVel {
        assert_eq!(v.len(), 6);
        Point3WithVel {
            x: v[0],
            y: v[1],
            z: v[2],
            dx: v[3],
            dy: v[4],
            dz: v[5],
        }
    }

    pub fn from_pos_and_vel_vec(pos: Vec<f64>, vel: Vec<f64>) -> Point3WithVel {
        assert!(pos.len() == 3 && vel.len() == 3);
        Point3WithVel {
            x: pos[0],
            y: pos[1],
            z: pos[2],
            dx: vel[0],
            dy: vel[1],
            dz: vel[2],
        }
    }

    pub fn intersection_x_y(&self, other: Point3WithVel) -> Option<(f64, f64)> {
        let m1 = self.dy / self.dx;
        let m2 = other.dy / other.dx;
        if (m2 - m1).abs() < f64::EPSILON {
            return None;
        }
        let x = (m1 * self.x - m2 * other.x + other.y - self.y) / (m1 - m2);
        let y = (m1 * m2 * (other.x - self.x) + m2 * self.y - m1 * other.y) / (m2 - m1);
        Some((x, y))
    }
}

pub struct Point3 {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl std::fmt::Display for Point3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Point3 {
    //! Rectilinear distance is also known as L1, taxicab or manhattan distance
    pub fn rectilinear_dist(&self, p: &Point3) -> usize {
        (isize::abs(self.x - p.x) + isize::abs(self.y - p.y) + isize::abs(self.z - p.z)) as usize
    }

    pub fn from_vec(v: Vec<isize>) -> Point3 {
        assert_eq!(v.len(), 3);
        Point3 {
            x: v[0],
            y: v[1],
            z: v[2],
        }
    }

    pub fn max(&self) -> isize {
        std::cmp::max(std::cmp::max(self.x, self.y), self.z)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Sequence)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn reverse(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dist() {
        let a = Point { x: 0, y: 0 };
        let b = Point { x: 1, y: 0 };
        let c = Point { x: -1, y: 1 };

        assert_eq!(a.rectilinear_dist(&a), 0);
        assert_eq!(a.rectilinear_dist(&b), 1);
        assert_eq!(a.rectilinear_dist(&c), 2);
        assert_eq!(a.rectilinear_dist(&c), c.rectilinear_dist(&a));
    }
    #[test]
    fn test_neighbour() {
        let a = Point { x: 0, y: 0 };
        assert_eq!(a.neighbour(Direction::North), Point { x: 0, y: -1 });
        assert_eq!(a.neighbour(Direction::East), Point { x: 1, y: 0 });
        assert_eq!(a.neighbour(Direction::South), Point { x: 0, y: 1 });
        assert_eq!(a.neighbour(Direction::West), Point { x: -1, y: 0 });
    }
}
