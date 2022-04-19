use bevy::prelude::*;

const T:bool = true;
const F:bool = false;

struct TileGroup {
    step: f32,
    coord: Vec2,
    occupancy: [[bool; 3]; 3]
}

#[derive(Debug)]
pub struct Line2 {
    start: Vec2,
    stop: Vec2
}

impl Line2 {
    fn new(start: Vec2, stop: Vec2) -> Line2 {
        Line2{start, stop}
    }
}

impl TileGroup {
    pub fn border(&self) -> Vec<Line2> {
        match self.occupancy {
            //wall
            [
                [F, F, F],
                [T, T, T],
                [T, T, T]
            ] => vec![self.draw_line(3,5)],
            [
                [T, T, F],
                [T, T, F],
                [T, T, F]
            ] => vec![self.draw_line(1,7)],
            [
                [T, T, T],
                [T, T, T],
                [F, F, F]
            ] => vec![self.draw_line(3,5)],
            [
                [F, T, T],
                [F, T, T],
                [F, T, T]
            ] => vec![self.draw_line(1,7)],

            //inside corner
            [
                [F, T, T],
                [T, T, T],
                [T, T, T]
            ] => vec![self.draw_line(1, 4), 
                      self.draw_line(4, 3)],
            [
                [T, T, F],
                [T, T, T],
                [T, T, T]
            ] => vec![self.draw_line(1, 4), 
                      self.draw_line(4, 5)],
            [
                [T, T, T],
                [T, T, T],
                [T, T, F]
            ] => vec![self.draw_line(5, 4), 
                      self.draw_line(4, 7)],
            [
                [T, T, T],
                [T, T, T],
                [F, T, T]
            ] => vec![self.draw_line(3, 4), 
                        self.draw_line(4, 7)],

            //inside L
            [
                [F, F, T],
                [T, T, T],
                [T, T, T]
            ] => vec![self.draw_line(2, 5), 
                        self.draw_line(5, 3)],
            [
                [T, T, F],
                [T, T, F],
                [T, T, T]
            ] => vec![self.draw_line(1, 7), 
                        self.draw_line(7, 8)],
            [
                [T, T, T],
                [T, T, T],
                [T, F, F]
            ] => vec![self.draw_line(5, 3), 
                        self.draw_line(3, 6)],
            [
                [T, T, T],
                [F, T, T],
                [F, T, T]
            ] => vec![self.draw_line(0, 1), 
                        self.draw_line(1, 7)],

            //outside L
            [
                [T, F, F],
                [T, T, T],
                [T, T, T]
            ] => vec![self.draw_line(0, 3), 
                        self.draw_line(3, 5)],
            [
                [T, T, T],
                [T, T, F],
                [T, T, F]
            ] => vec![self.draw_line(2, 1), 
                        self.draw_line(1, 7)],
            [
                [T, T, T],
                [T, T, T],
                [F, F, T]
            ] => vec![self.draw_line(3, 5), 
                        self.draw_line(5, 8)],
            [
                [F, T, T],
                [F, T, T],
                [T, T, T]
            ] => vec![self.draw_line(1, 7), 
                        self.draw_line(7, 6)],

            //hole
            [
                [T, F, T],
                [T, T, T],
                [T, T, T]
            ] => vec![self.draw_line(0, 3), 
                        self.draw_line(3, 5),
                        self.draw_line(5, 2)],
            [
                [T, T, T],
                [T, T, F],
                [T, T, T]
            ] => vec![self.draw_line(2, 1), 
                        self.draw_line(1, 7), 
                        self.draw_line(7, 8)],
            [
                [T, T, T],
                [T, T, T],
                [T, F, T]
            ] => vec![self.draw_line(6, 3), 
                        self.draw_line(3, 5), 
                        self.draw_line(5, 8)],
            [
                [T, T, T],
                [F, T, T],
                [T, T, T]
            ] => vec![self.draw_line(0, 1), 
                        self.draw_line(1, 7), 
                        self.draw_line(7, 6)],
            
            //spike
            [
                [F, T, F],
                [T, T, T],
                [T, T, T]
            ] => vec![self.draw_line(3, 4), 
                        self.draw_line(4, 1), 
                        self.draw_line(4, 5)],
            [
                [T, T, F],
                [T, T, T],
                [T, T, F]
            ] => vec![self.draw_line(1, 4), 
                        self.draw_line(4, 5), 
                        self.draw_line(4, 7)],
            [
                [T, T, T],
                [T, T, T],
                [F, T, F]
            ] => vec![self.draw_line(3, 4), 
                        self.draw_line(4, 7), 
                        self.draw_line(4, 5)],
            [
                [F, T, T],
                [T, T, T],
                [F, T, T]
            ] => vec![self.draw_line(1, 4), 
                        self.draw_line(4, 3), 
                        self.draw_line(4, 7)],

            //H
            [
                [T, F, T],
                [T, T, T],
                [T, F, T]
            ] => vec![self.draw_line(0, 3), 
                      self.draw_line(3, 5), 
                        self.draw_line(5, 2), 
                        self.draw_line(3, 6), 
                        self.draw_line(5, 8)],
            [
                [T, T, T],
                [F, T, F],
                [T, T, T]
            ] => vec![self.draw_line(0, 1), 
                        self.draw_line(1, 7), 
                        self.draw_line(7, 6), 
                        self.draw_line(7, 8), 
                        self.draw_line(1, 2)],
            //zigzag
            [
                [T, F, F],
                [T, T, F],
                [T, T, T]
            ] => vec![self.draw_line(0, 3), 
                        self.draw_line(3, 4), 
                        self.draw_line(4, 7), 
                        self.draw_line(7, 8)],
            [
                [T, T, T],
                [T, T, F],
                [T, F, F]
            ] => vec![self.draw_line(2, 1), 
                        self.draw_line(1, 4), 
                        self.draw_line(4, 3), 
                        self.draw_line(3, 6)],
            [
                [T, T, T],
                [F, T, T],
                [F, F, T]
            ] => vec![self.draw_line(0, 1), 
                        self.draw_line(1, 4), 
                        self.draw_line(4, 5), 
                        self.draw_line(5, 8)],
            [
                [F, F, T],
                [F, T, T],
                [T, T, T]
            ] => vec![self.draw_line(2, 5), 
                        self.draw_line(5, 4), 
                        self.draw_line(4, 7), 
                        self.draw_line(7, 6)],

            //all remaining shapes are variations on stars
            _ => vec![
                self.draw_line(1, 4), 
                self.draw_line(3, 4), 
                self.draw_line(4, 5), 
                self.draw_line(4, 7)]
        }
    }


    fn draw_line(&self, start: i32, stop: i32) -> Line2 {
        Line2::new(self.to_point(start), self.to_point(stop))
    }

    fn to_point(&self, c: i32) -> Vec2 {
        let x = self.coord.x;
        let y = self.coord.y;
        let step = self.step;
        match c {
            0 => Vec2::new(x - step, y - step),
            1 => Vec2::new(x, y - step),
            2 => Vec2::new(x + step, y - step),
            3 => Vec2::new(x - step, y),
            4 => Vec2::new(x, y),
            5 => Vec2::new(x+step, y),
            6 => Vec2::new(x - step, y + step),
            7 => Vec2::new(x, y + step),
            8 => Vec2::new(x+step, y + step),
            _ => Vec2::new(x, y)
        }
    }
}