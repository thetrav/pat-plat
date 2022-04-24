use bevy::prelude::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Line {
    origin: Vec2,
    vector: Vec2
}

impl Line {
    pub fn new(origin: Vec2, vector: Vec2) -> Self {
        Line{origin, vector}
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct LineHit {
    line: Line,
    aabb: AxisAlignedBoundingBox,
    pos: Vec2,
    t: f32
}

impl LineHit {
    pub fn new(line: Line, aabb: AxisAlignedBoundingBox, pos: Vec2, t: f32) -> Self {
        LineHit{line, aabb, pos, t}
    }

    pub fn _distance(&self) -> f32 {
        let p1 = self.line.origin;
        let p2 = p1 + (self.line.vector * self.t);
        (p2 - p1).length()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct AxisAlignedBoundingBox {
    min: Vec2,
    max: Vec2,
}

fn swap(a: f32, b:f32) -> (f32, f32) {
    return (b, a);
}

impl AxisAlignedBoundingBox {
    pub fn new(center: Vec2, size: Vec2) -> Self {
        let hs = size/2.;
        AxisAlignedBoundingBox{ 
            min: center - hs,
            max: center + hs
        }
    }


    pub fn intersection(self, line: Line) -> Option<LineHit> {
        let origin = line.origin;
        let dir = line.vector;
        let min = self.min;
        let max = self.max;
        //non-penetrating intersections (grazing at distance zero) don't count
        if dir.x == 0. && (min.x == origin.x || max.x == origin.x) {
            return None;
        }
        if dir.y == 0. && (min.y == origin.y || max.y == origin.y) {
            return None;
        }

        let mut txmin = (min.x - origin.x) / dir.x; 
        let mut txmax = (max.x - origin.x) / dir.x; 
        // println!("x intersections: {}, {}", txmin, txmax);
        
        if txmin > txmax {
            (txmin, txmax) = swap(txmin, txmax); 
        }
        
        let mut tymin = (min.y - origin.y) / dir.y; 
        let mut tymax = (max.y - origin.y) / dir.y; 
        // println!("y intersections: {}, {}", tymin, tymax);
        
        if tymin > tymax {
            (tymin, tymax) = swap(tymin, tymax);
        }
        
        if txmin > tymax || tymin > txmax {
            return None; 
        }
        
        let t_intercept = if txmin < 0. {tymin} else {txmin};
        // println!("t_intercept: {}", t_intercept);
        if t_intercept > 1. {
            return None;
        }
        //This happens when the origin is on an axis
        if t_intercept < 0. {
            return None;
        }
        return Some(LineHit::new(line, self,
            origin + dir * t_intercept, t_intercept));
    }
}