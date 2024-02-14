use bevy::prelude::*;

#[derive(Default, Clone, Copy)]
pub struct Line {
    pub start: IVec2,
    pub end: IVec2,
}

impl Line {
    pub fn new(start: IVec2, end: IVec2) -> Self { Self { start, end } }
}

impl IntoIterator for Line {
    type IntoIter = LineBresenhamIter;
    type Item = IVec2;

    fn into_iter(self) -> Self::IntoIter { LineBresenhamIter::new(self.start, self.end) }
}

#[derive(Debug, Clone)]
pub struct LineBresenhamIter {
    abs_x: i32,
    abs_y: i32,
    end_x: i32,
    delta_step: i32,
    delta_x: i32,
    delta_y: i32,
    octant: Octant,
}

impl LineBresenhamIter {
    pub fn new(start: IVec2, end: IVec2) -> Self {
        let octant = Octant::new(start, end);

        let start_offset = octant.to_offset(start);
        let end_offset = octant.to_offset(end);

        let delta_x = end_offset.0 - start_offset.0;
        let delta_y = end_offset.1 - start_offset.1;

        Self {
            abs_x: start_offset.0,
            abs_y: start_offset.1,
            end_x: end_offset.0,
            delta_step: delta_y - delta_x,
            delta_x,
            delta_y,
            octant,
        }
    }

    pub fn advance(&mut self) -> IVec2 {
        let current_point = (self.abs_x, self.abs_y);
        if self.delta_step >= 0 {
            self.abs_y += 1;
            self.delta_step -= self.delta_x;
        }

        self.delta_step += self.delta_y;

        self.abs_x += 1;

        self.octant.from_offset(current_point)
    }
}

impl Iterator for LineBresenhamIter {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.abs_x > self.end_x {
            None
        } else {
            Some(self.advance())
        }
    }
}

/// An octant
#[derive(Debug, Clone)]
pub struct Octant(pub u8);

impl Octant {
    #[inline]
    pub fn to_offset(&self, position: IVec2) -> (i32, i32) {
        match self.0 {
            0 => (position.x, position.y),
            1 => (position.y, position.x),
            2 => (position.y, -position.x),
            3 => (-position.x, position.y),
            4 => (-position.x, -position.y),
            5 => (-position.y, -position.x),
            6 => (-position.y, position.x),
            7 => (position.x, -position.y),
            _ => unreachable!(),
        }
    }

    #[inline]
    #[allow(clippy::wrong_self_convention)]
    pub fn from_offset(&self, offset: (i32, i32)) -> IVec2 {
        let p = match self.0 {
            0 => (offset.0, offset.1),
            1 => (offset.1, offset.0),
            2 => (-offset.1, offset.0),
            3 => (-offset.0, offset.1),
            4 => (-offset.0, -offset.1),
            5 => (-offset.1, -offset.0),
            6 => (offset.1, -offset.0),
            7 => (offset.0, -offset.1),
            _ => unreachable!(),
        };
        IVec2::new(p.0, p.1)
    }

    #[inline(always)]
    pub fn new(position: IVec2, other: IVec2) -> Self {
        let start = position;
        let end = other;

        let mut dx = end.x - start.x;
        let mut dy = end.y - start.y;
        let mut octant = 0;
        if dy < 0 {
            dx = -dx;
            dy = -dy;
            octant += 4;
        }
        if dx < 0 {
            let tmp = dx;
            dx = dy;
            dy = -tmp;
            octant += 2;
        }
        if dx < dy {
            octant += 1;
        }

        Self(octant)
    }
}
