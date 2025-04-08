/*
Цель:
В рамках выполнения ДЗ необходимо превратить процедурный код из результата ДЗ ""Реализация функции"" (занятие от 10.12.2024) в объектный.

Описание/Пошаговая инструкция выполнения домашнего задания:
Требования:

Библиотека предоставляет тот же функционал.
Все функции заменены на методы.
Все методы протестированы.
""cargo clippy"" и ""cargo fmt --check"" не выдают предупреждений и ошибок.
*/
pub struct SimpleMethods {}

impl SimpleMethods {
    pub fn double_int32(x: i32) -> i32 {
        x * 2
    }

    pub fn double_int64(x: i32) -> i64 {
        x as i64 * 2_i64
    }

    pub fn double_float32(x: f32) -> f32 {
        x * 2_f32
    }

    pub fn double_float64(x: f32) -> f64 {
        x as f64 * 2_f64
    }

    pub fn int_plus_float_to_float(x: i32, y: f32) -> f64 {
        (x as f32 + y) as f64
    }

    pub fn int_plus_float_to_int(x: i32, y: f32) -> i64 {
        (x as f32 + y) as i64
    }

    pub fn tuple_sum(tup: (i32, i32)) -> i32 {
        tup.0 + tup.1
    }

    pub fn array_sum(arr: [i32; 3]) -> i32 {
        let mut sum: i32 = 0;
        for x in arr {
            sum += x;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4i32, SimpleMethods::double_int32(2));
    }

    #[test]
    fn test_2() {
        assert_eq!(4i64, SimpleMethods::double_int64(2));
    }

    #[test]
    fn test_3() {
        assert_eq!(4.0f32, SimpleMethods::double_float32(2.0));
    }

    #[test]
    fn test_4() {
        assert_eq!(4.0f64, SimpleMethods::double_float64(2.0));
    }

    #[test]
    fn test_5() {
        assert_eq!(4.0f64, SimpleMethods::int_plus_float_to_float(2, 2.0));
    }

    #[test]
    fn test_6() {
        assert_eq!(4i64, SimpleMethods::int_plus_float_to_int(2, 2.0));
    }

    #[test]
    fn test_7() {
        assert_eq!(4, SimpleMethods::tuple_sum((2, 2)));
    }

    #[test]
    fn test_8() {
        assert_eq!(4, SimpleMethods::array_sum([1, 1, 2]));
    }
}
