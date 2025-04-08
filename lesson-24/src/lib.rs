// Возьмите код из предыдущего занятия.
// И теперь представим, что список фигур для которых мы хотим выполнить вычисления
// неизвестен на этапе компиляции программы.

// Исправьте фунцию perimeter_by_area, так чтобы она принимала параметр Box<dyn Shape>

use std::f64::consts::PI;

pub trait Shape {
    fn get_area(&self) -> f64;
    fn get_perimeter(&self) -> f64;
}

pub struct Triangle {
    sides_lens: [f64; 3],
}
impl Shape for Triangle {
    fn get_area(&self) -> f64 {
        let [a, b, c] = self.sides_lens;
        let p = (a + b + c) / 2.0;
        f64::sqrt(p * (p - a) * (p - b) * (p - c))
    }
    fn get_perimeter(&self) -> f64 {
        self.sides_lens[0] + self.sides_lens[1] + self.sides_lens[2]
    }
}

pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn get_area(&self) -> f64 {
        self.width * self.height
    }
    fn get_perimeter(&self) -> f64 {
        (self.width + self.height) * 2.0
    }
}

pub struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn get_area(&self) -> f64 {
        (PI * self.radius).powf(2.0)
    }
    fn get_perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }
}

// исправьте сигнатуру и добавьте реализацию
pub fn perimeter_by_area(b: Box<dyn Shape>) -> f64 {
    b.get_perimeter() / b.get_area()
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::relative_eq;

    #[test]
    fn test() {
        _ = relative_eq!(
            perimeter_by_area(Box::new(Triangle {
                sides_lens: [3.0, 4.0, 5.0]
            })),
            2.0
        );
        _ = relative_eq!(perimeter_by_area(Box::new(Circle { radius: 2.0 })), 1.0);
        _ = relative_eq!(
            perimeter_by_area(Box::new(Rectangle {
                width: 2.0,
                height: 3.0,
            })),
            1.6666
        );
    }
}
