// Обобщаем поведение с помощью шаблонов и статического полиморфизма.
// Пусть у нас 3 типа фигур: треугольник, прямоугольник и круг
// Создайте трейт Shape, в котором есть методы:
// get_area(&self) -> f64 // возвращает зачение площади фигуры
// get_perimeter(&self) -> f64 // возвращает значение периметра фигуры
// Реализуйте данный трейт для треугольника, прямоугольника и круга

// Напишите 1 функцию perimeter_by_area, которая может принимать любую фигуру
// и возвращает отнощение ее периметра к площади (P/A)

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
pub fn perimeter_by_area(s: impl Shape) -> f64 {
    s.get_perimeter() / s.get_area()
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::relative_eq;

    #[test]
    fn test() {
        _ = relative_eq!(
            perimeter_by_area(Triangle {
                sides_lens: [3.0, 4.0, 5.0]
            }),
            2.0
        );
        _ = relative_eq!(perimeter_by_area(Circle { radius: 2.0 }), 1.0);
        _ = relative_eq!(
            perimeter_by_area(Rectangle {
                width: 2.0,
                height: 3.0,
            }),
            1.6666
        );
    }
}
