
// Вам нужно реализовать программу обработки команд для дисплея.
// На вход пользователь подает:
// * 2 числа: размер дисплея
// * 1 число: цвет дисплея по-умолчанию (1 - красный, 2 - зеленый, 3 - синий)
// * Последовательность команд: набор чисел.
//
// Дисплей поддерживает следующие команды:
// * 1 x y - переместить курсор в позицию x y
// * 2 colour - перекрасить пиксель в цвет colour
//
// Пример входных данных:
// 4 4
// 1
// 1 2 2 2 3
// В результате пиксель по позиции (2,2) будет перекрашен в синий цвет

// Обновлять состояние дисплея нужно через метод matrix.set_colour(pos_x, pos_y, colour)

// Важно! Обязательна проверка на ошибки. Если пользователь просит переместиться на пиксель за пределами дисплея или ввел неправильный цвет, то вам нужно кинуть панику!

use std::io;
mod matrix;
use matrix::Matrix;

struct Display {
    // можете добавить сюда любые дополнительные поля
    current_pixel: (u64, u64),
    boundaries: (u32, u32),
    matrix: Matrix,
}

fn create_display(max_width: u32, max_height: u32, default_colour: u8) -> Display {
    // ваш код сюда
    Display {
        current_pixel: (0, 0),
        boundaries: (max_width, max_height),
        matrix: Matrix::new(max_width, max_height, default_colour),
    }
}

fn process_commands(display: &mut Display, input: Vec<u64>) {
    let mut offset = 0;
    for (idx, command) in input.iter().enumerate() {
        if idx < offset {
            continue;
        }
        match command {
            1 => {
                let x = input.get(idx + 1).unwrap().clone();
                let y = input.get(idx + 2).unwrap().clone();
                if x > display.boundaries.0 as u64 || y > display.boundaries.1 as u64 {
                    panic!("Out of display boundaries");
                }
                display.current_pixel = (x, y);
                offset += 3;
            }
            2 => {
                let colour = input.get(idx + 1).unwrap().clone() as u8;
                if colour > 3 || colour < 1 {
                    panic!("No such colour");
                }
                display
                    .matrix
                    .set_colour(display.current_pixel.0, display.current_pixel.1, colour);
                offset += 2;
            }
            _ => {}
        }
    }
}

// код ниже трогать не нужно, можете просто посмотреть его

// тесты
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_happy_case() {
        let mut display = create_display(4, 4, 1);
        process_commands(&mut display, vec![1, 2, 2, 2, 3]);
        let mut expected = Matrix::new(4, 4, 1);
        expected.set_colour(2, 2, 3);
        assert_eq!(display.matrix, expected);
        println!("Happy case: ");
        display.matrix.display();
    }

    #[test]
    #[should_panic]
    fn test_error() {
        let mut display = create_display(4, 4, 1);
        process_commands(&mut display, vec![1, 5, 5, 2, 3]);
    }

    #[test]
    #[should_panic]
    fn test_error_invalid_colour() {
        let mut display = create_display(4, 4, 1);
        process_commands(&mut display, vec![1, 2, 2, 2, 5]);
    }

    #[test]
    fn test_other_case() {
        let mut display = create_display(5, 5, 3);
        process_commands(&mut display, vec![1, 3, 2, 2, 1]);
        let mut expected = Matrix::new(5, 5, 3);
        expected.set_colour(3, 2, 1);
        assert_eq!(display.matrix, expected);
        println!("Other case: ");
        display.matrix.display();
    }
    #[test]
    fn test_complex_case() {
        let mut display = create_display(5, 5, 3);
        process_commands(&mut display, vec![1, 3, 2, 2, 1, 1, 2, 3, 2, 2]);
        let mut expected = Matrix::new(5, 5, 3);
        expected.set_colour(3, 2, 1);
        expected.set_colour(2, 3, 2);
        assert_eq!(display.matrix, expected);
        println!("Complex case: ");
        display.matrix.display();
    }
    #[test]
    fn test_more_complex_case() {
        let mut display = create_display(6, 6, 3);
        process_commands(
            &mut display,
            vec![1, 3, 3, 2, 1, 1, 4, 4, 2, 2, 1, 5, 5, 2, 1],
        );
        let mut expected = Matrix::new(6, 6, 3);
        expected.set_colour(3, 3, 1);
        expected.set_colour(4, 4, 2);
        expected.set_colour(5, 5, 1);
        assert_eq!(display.matrix, expected);
        println!("Complex case: ");
        display.matrix.display();
    }
}

fn main() {
    println!("Введите размеры дисплея (ширина высота):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let (width, height) = parse_dimensions(&input);

    println!("Введите стандартный цвет дисплея (1 - красный, 2 - зеленый, 3 - синий):");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let default_colour = match input.trim() {
        "1" => 1, // Красный
        "2" => 2, // Зеленый
        "3" => 3, // Синий
        _ => panic!("Неверный ввод цвета. Ожидалось 1, 2 или 3."),
    };

    // Создаём дисплей и заполняем его стандартным цветом
    let mut display = create_display(width, height, default_colour);

    // Ввод действий
    println!("Введите строку с действиями:");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let commands = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    // Отображение дисплея
    process_commands(&mut display, commands);

    display.matrix.display();
}

fn parse_dimensions(input: &str) -> (u32, u32) {
    let parts: Vec<u32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Неверный ввод размера"))
        .collect();
    if parts.len() != 2 {
        panic!("Ожидалось два числа для размеров дисплея.");
    }
    (parts[0], parts[1])
}
