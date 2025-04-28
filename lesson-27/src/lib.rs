/*
Описание/Пошаговая инструкция выполнения домашнего задания:

Реализуем сортировку слиянием: https://en.wikipedia.org/wiki/Merge_sort

Это одна из самых эффективных сортировок, потому что гарантирует O(N log N) сложность во всех случаях.

Вам требуется реализовать эту сортировку на Rust, но с ограничением: нельзя писать for или while циклы, используйте только итераторы.

Напишите функцию merge_sort(input: &[u64]) -> Vec и покройте свое решение тестами.

Подсказка: посмотрите на std::iter::from_fn.
*/

fn merge(left: &[u64], right: &[u64]) -> Vec<u64> {
    let mut left_iter = left.iter().peekable();
    let mut right_iter = right.iter().peekable();

    std::iter::from_fn(|| match (left_iter.peek(), right_iter.peek()) {
        (Some(&l), Some(&r)) => {
            if l <= r {
                left_iter.next()
            } else {
                right_iter.next()
            }
        }
        (Some(_), None) => left_iter.next(),
        (None, Some(_)) => right_iter.next(),
        (None, None) => None,
    })
    .cloned()
    .collect()
}

pub fn merge_sort(input: &[u64]) -> Vec<u64> {
    if input.len() <= 1 {
        return input.to_vec();
    }

    let mid = input.len() / 2;
    let (left, right) = input.split_at(mid);
    let left_sorted = merge_sort(left);
    let right_sorted = merge_sort(right);
    merge(&left_sorted, &right_sorted)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_empty() {
        let input = [];
        let output = merge_sort(&input);
        assert_eq!(output, []);
    }

    #[test]
    fn test_single() {
        let input = [42];
        let output = merge_sort(&input);
        assert_eq!(output, [42]);
    }

    #[test]
    fn test_sorted() {
        let input = [1, 2, 3, 4, 5];
        let output = merge_sort(&input);
        assert_eq!(output, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reversed() {
        let input = [5, 4, 3, 2, 1];
        let output = merge_sort(&input);
        assert_eq!(output, [1, 2, 3, 4, 5]);
    }
    #[test]
    fn sort_test() {
        let x = [6, 2, 8, 3, 9, 234, 76, 33, 75, 22, 343, 7, 45, 2, 1, 675];
        merge_sort(&x);
    }
}
