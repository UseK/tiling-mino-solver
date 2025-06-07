use crate::Shape;

pub trait Scale
where
    Self: Clone + Sized,
{
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn increment_width(&mut self);
    fn decrement_width(&mut self);
    fn increment_height(&mut self);
    fn decrement_height(&mut self);
}

impl Scale for Shape {
    fn width(&self) -> usize {
        self.width()
    }
    fn height(&self) -> usize {
        self.height()
    }
    fn increment_width(&mut self) {
        self.0.iter_mut().for_each(|row| {
            row.push(bool::default());
        });
    }
    fn decrement_width(&mut self) {
        self.0.iter_mut().for_each(|row| {
            row.pop();
        });
    }
    fn increment_height(&mut self) {
        let width = self.width();
        self.0.push(vec![bool::default(); width]);
    }
    fn decrement_height(&mut self) {
        self.0.pop();
    }
}

#[test]
fn scale_shape_test() {
    // #.
    // .#
    let mut shape = Shape::new(vec![vec![true, false], vec![false, true]]);
    let first = shape.clone();
    assert_eq!(shape.width(), 2);
    assert_eq!(shape.height(), 2);
    // #..
    // .#.
    shape.increment_width();
    assert_eq!(shape.width(), 3);
    assert_eq!(shape.height(), 2);
    assert!(!shape.is_wall(2, 0));
    assert!(!shape.is_wall(2, 1));
    assert!(shape.is_wall(1, 1));
    assert_eq!(
        shape,
        Shape::new(vec![vec![true, false, false], vec![false, true, false]])
    );
    // #..
    // .#.
    // ...
    shape.increment_height();
    assert_eq!(shape.width(), 3);
    assert_eq!(shape.height(), 3);
    assert!(!shape.is_wall(0, 2));
    assert!(!shape.is_wall(1, 2));
    assert!(!shape.is_wall(2, 2));
    assert_eq!(
        shape,
        Shape::new(vec![
            vec![true, false, false],
            vec![false, true, false],
            vec![false, false, false]
        ])
    );
    // #.
    // .#
    // ..
    shape.decrement_width();
    assert_eq!(shape.width(), 2);
    assert_eq!(shape.height(), 3);
    assert_eq!(
        shape,
        Shape::new(vec![
            vec![true, false],
            vec![false, true],
            vec![false, false]
        ])
    );
    shape.decrement_height();
    assert_eq!(shape.width(), 2);
    assert_eq!(shape.height(), 2);
    assert_eq!(shape, first);
}
