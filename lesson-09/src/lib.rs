// Функционал библиотеки:

// Протестировать функции.
// Убедиться, что копилятор не позволит вернуть более одной мутабельной ссылки на один объект.

// Требования:

// Реализованы и протестированы все перечисленные функции.
// ""cargo clippy"" и ""cargo fmt --check"" не выдают предупреждений и ошибок.

// 1: Принимает мутабельную ссылку на кортеж и bool значение.
//      Если false, возвращает мутабельную ссылку на первый элемент кортежа.
//      Если true, возвращает мутабельную ссылку на второй элемент кортежа.
pub fn first(tuple: &mut (usize, usize), boolean: bool) -> &mut usize {
    if boolean {
        &mut tuple.1
    } else {
        &mut tuple.0
    }
}

// 2: Принимает мутабельную ссылку на слайс и число N. Возвращает мутабельную ссылку на N-ый элемент.
pub fn second(x: &mut [i32], n: usize) -> &mut i32 {
    if n > x.len() {
        panic!("n too large");
    }
    &mut x[n]
}

// 3: Принимает слайс и число N. Возвращает ссылку на N-ый элемент слайса с конца.
pub fn third(x: &[i32], n: usize) -> &i32 {
    if n > x.len() {
        panic!("n too large");
    }
    let rev = x.len() - 1 - n;
    &x[rev]
}

// 4: Принимает слайс и число N. Возвращает два слайса с элементами:
//      с нулевого по N-1;
//      с N-го по последний;
pub fn fourth(x: &[i32], n: usize) -> (&[i32], &[i32]) {
    if n > x.len() {
        panic!("n too large");
    }
    x.split_at(n)
}

// 5: Принимает слайс и возвращает массив слайсов, содержащий четыре равные (насколько возможно) части исходного слайса.
pub fn fifth(x: &[i32]) -> [&[i32]; 4] {
    let first_split = if x.len() % 4 == 0 {
        x.len().div_ceil(2)
    } else {
        (x.len() + 1).div_ceil(2)
    };
    let (left, right) = x.split_at(first_split);
    let left = left.split_at(left.len().div_ceil(2));
    let right = right.split_at(right.len().div_ceil(2));

    let array = [left.0, left.1, right.0, right.1];
    array
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_works() {
        let mut tuple = (4, 5);
        let test = first(&mut tuple, true);
        // Second mutable borrow
        // let test2 = first(&mut tuple, false);

        *test = 6;

        assert_eq!(6, *first(&mut tuple, true));
        assert_eq!(4, *first(&mut tuple, false));
        assert_ne!(5, *first(&mut tuple, false));
        assert_ne!(5, *first(&mut tuple, true));
    }

    #[test]
    fn second_works() {
        let x = &mut [0, 1, 2, 3, 4, 5];

        let value = second(x, 1);
        // Second borrow
        // let value2 = second(x, 3);
        *value = 44;

        assert_eq!(2, *second(&mut x[..], 2));
        assert_eq!(0, *second(&mut x[..], 0));
        assert_eq!(44, *second(&mut x[..], 1));
        assert_ne!(3, *second(&mut x[..], 4));
        assert_ne!(4, *second(&mut x[..], 3));
    }

    #[test]
    #[should_panic]
    fn second_panics() {
        let x = &mut [0, 1, 2, 3, 4, 5];
        assert_eq!(0, *second(&mut x[..], usize::MAX - 1));
    }

    #[test]
    fn third_works() {
        let x = &mut [0, 1, 2, 3, 4, 5];
        assert_eq!(0, *third(&x[..], 5));
        assert_eq!(1, *third(&x[..], 4));
        assert_eq!(2, *third(&x[..], 3));
        assert_eq!(3, *third(&x[..], 2));
        assert_eq!(4, *third(&x[..], 1));
        assert_eq!(5, *third(&x[..], 0));
        assert_ne!(3, *third(&x[..], 4));
        assert_ne!(4, *third(&x[..], 3));
    }

    #[test]
    #[should_panic]
    fn third_panics() {
        let x = &mut [0, 1, 2, 3, 4, 5];
        println!("{}", *third(&x[..], usize::MAX - 1));
    }

    #[test]
    fn fourth_works() {
        let x = &mut [0, 1, 2, 3, 4, 5];

        let (left, right) = fourth(&x[..], 1);
        assert_eq!(&[0], left);
        assert_eq!(&[1, 2, 3, 4, 5], right);

        let (left, right) = fourth(&x[..], 2);
        assert_eq!(&[0, 1], left);
        assert_eq!(&[2, 3, 4, 5], right);

        let (left, right) = fourth(&x[..], 3);
        assert_eq!(&[0, 1, 2], left);
        assert_eq!(&[3, 4, 5], right);

        let (left, right) = fourth(&x[..], 4);
        assert_eq!(&[0, 1, 2, 3], left);
        assert_eq!(&[4, 5], right);

        let (left, right) = fourth(&x[..], 5);
        assert_eq!(&[0, 1, 2, 3, 4], left);
        assert_eq!(&[5], right);
    }

    #[test]
    #[should_panic]
    fn fourth_panics() {
        let x = &mut [0, 1, 2, 3, 4, 5];
        fourth(&x[..], usize::MAX - 1);
    }

    #[test]
    fn fifth_works() {
        let result = fifth(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
        assert_eq!(&[1, 2, 3, 4], result[0]);
        assert_eq!(&[5, 6, 7, 8], result[1]);
        assert_eq!(&[9, 10, 11, 12], result[2]);
        assert_eq!(&[13, 14, 15], result[3]);

        let result = fifth(&[1, 2, 3]);
        println!("{result:?}");
        assert_eq!(&[1], result[0]);
        assert_eq!(&[2], result[1]);
        assert_eq!(&[3], result[2]);
        assert!(result[3].is_empty());

        let result = fifth(&[1, 2, 3, 4]);
        assert_eq!([[1], [2], [3], [4]], result);
        println!("{result:?}");
        assert_eq!(&[1], result[0]);
        assert_eq!(&[2], result[1]);
        assert_eq!(&[3], result[2]);
        assert_eq!(&[4], result[3]);

        let result = fifth(&[1, 2, 3, 4, 5]);
        println!("{result:?}");
        assert_eq!(&[1, 2], result[0]);
        assert_eq!(&[3], result[1]);
        assert_eq!(&[4], result[2]);
        assert_eq!(&[5], result[3]);

        let result = fifth(&[1, 2, 3, 4, 5, 6]);
        println!("{result:?}");
        assert_eq!(&[1, 2], result[0]);
        assert_eq!(&[3, 4], result[1]);
        assert_eq!(&[5], result[2]);
        assert_eq!(&[6], result[3]);

        let result = fifth(&[1, 2, 3, 4, 5, 6, 7]);
        println!("{result:?}");
        assert_eq!(&[1, 2], result[0]);
        assert_eq!(&[3, 4], result[1]);
        assert_eq!(&[5, 6], result[2]);
        assert_eq!(&[7], result[3]);

        let result = fifth(&[1, 2, 3, 4, 5, 6, 7, 8]);
        println!("{result:?}");
        assert_eq!(&[1, 2], result[0]);
        assert_eq!(&[3, 4], result[1]);
        assert_eq!(&[5, 6], result[2]);
        assert_eq!(&[7, 8], result[3]);

        let result = fifth(&[1, 2, 3, 4, 5, 6, 7, 8, 9]);
        println!("{result:?}");
        assert_eq!(&[1, 2, 3], result[0]);
        assert_eq!(&[4, 5], result[1]);
        assert_eq!(&[6, 7], result[2]);
        assert_eq!(&[8, 9], result[3]);

        let result = fifth(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        println!("{result:?}");
        assert_eq!(&[1, 2, 3], result[0]);
        assert_eq!(&[4, 5, 6], result[1]);
        assert_eq!(&[7, 8], result[2]);
        assert_eq!(&[9, 10], result[3]);

        let result = fifth(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
        println!("{result:?}");
        assert_eq!(&[1, 2, 3], result[0]);
        assert_eq!(&[4, 5, 6], result[1]);
        assert_eq!(&[7, 8, 9], result[2]);
        assert_eq!(&[10, 11], result[3]);

        let result = fifth(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
        println!("{result:?}");
        assert_eq!(&[1, 2, 3], result[0]);
        assert_eq!(&[4, 5, 6], result[1]);
        assert_eq!(&[7, 8, 9], result[2]);
        assert_eq!(&[10, 11, 12], result[3]);
    }
}
