// Слайсы. (мы спрашиваем эту задачку на собеседования на уровено Junior Engineer)
// Ring Buffer (кольцевой буффер) - структура данных, которая позволяет очень удобно реализовывать очередь на массиве фиксированного размера.
// https://ru.wikipedia.org/wiki/%D0%9A%D0%BE%D0%BB%D1%8C%D1%86%D0%B5%D0%B2%D0%BE%D0%B9_%D0%B1%D1%83%D1%84%D0%B5%D1%80
// Ключевая идея в том, что заполняя буффер до конца мы переходим в начало
// Пример API, вызовов и как меняется состояние буффера:
// [ _ _ _ ] create(3)
// [ a b _ ] write "ab" -> return 2
// [ a b c ] write "cd" -> return 1
// [ _ b c ] read(1) -> return "a"
// [ e b c ] write "e" -> return 1
// [ e _ _ ] read(2) -> return "bc"
// Ваша задача написать такой буффер и добавить тесты

#[derive(Clone)]
pub struct RingBuffer {
    read_idx: usize,
    write_idx: usize,
    data: Vec<u8>,
}

pub fn create(size: usize) -> RingBuffer {
    RingBuffer {
        read_idx: 0,
        write_idx: 0,
        data: vec![0; size],
    }
}

pub fn write(rb: &mut RingBuffer, elements: &[u8]) -> usize {
    let mut written = 0;
    for element in elements {
        if rb.write_idx >= rb.data.len() {
            if rb.data[0] == 0 {
                rb.write_idx = 0;
            } else {
                match rb.data[rb.read_idx] {
                    0 => rb.write_idx = rb.read_idx,
                    _ => continue,
                }
            }
        }
        rb.data[rb.write_idx] = *element;
        rb.write_idx += 1;
        written += 1;
    }
    written
}

pub fn read(rb: &mut RingBuffer, num_of_elements: usize) -> Vec<u8> {
    let mut elements = Vec::new();

    for _ in 0..num_of_elements {
        if rb.read_idx == rb.data.len() {
            rb.read_idx = 0;
        }
        if rb.data[rb.read_idx] == 0 {
            continue;
        }
        elements.push(rb.data[rb.read_idx]);
        rb.data[rb.read_idx] = 0;
        rb.read_idx += 1;
    }
    if rb.write_idx >= rb.data.len() {
        if rb.data[0] == 0 {
            rb.write_idx = 0;
        } else {
            rb.write_idx = rb.read_idx;
        }
    }
    elements
}

pub fn print(rb: RingBuffer) {
    println!(
        "len = {}, write_idx = {}, read_idx = {}, data = {:?}",
        rb.data.len(),
        rb.write_idx,
        rb.read_idx,
        rb.data,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_populated_rb() -> RingBuffer {
        let mut rb = create(10);
        write(&mut rb, "abcdefghij".as_bytes());
        print(rb.clone());
        println!("{:?}", String::from_utf8(rb.data.clone()));
        rb
    }

    #[test]
    fn rb_created() {
        let rb: RingBuffer = create(10);
        assert_eq!(10, rb.data.capacity());
    }

    #[test]
    fn rb_read_empty() {
        let mut rb: RingBuffer = create(10);
        assert!(read(&mut rb, 4).is_empty());
        assert!(read(&mut rb, 14).is_empty());
    }

    #[test]
    fn rb_read_until_empty() {
        let mut rb: RingBuffer = create_populated_rb();
        assert_eq!("abcd".as_bytes(), read(&mut rb, 4));
        assert_eq!("efgh".as_bytes(), read(&mut rb, 4));
        assert_eq!("ij".as_bytes(), read(&mut rb, 4));
        assert!(read(&mut rb, 4).is_empty());
    }

    #[test]
    fn rb_write_full() {
        let mut rb: RingBuffer = create(10);
        let control = create_populated_rb();
        assert_eq!(4, write(&mut rb, "abcd".as_bytes()));
        assert_eq!(6, write(&mut rb, "efghij".as_bytes()));
        assert_eq!(control.data, rb.data);
    }

    #[test]
    fn rb_write_overflow() {
        let mut rb: RingBuffer = create(10);
        let control = create_populated_rb();
        assert_eq!(4, write(&mut rb, "abcd".as_bytes()));
        assert_eq!(4, write(&mut rb, "efgh".as_bytes()));
        assert_eq!(2, write(&mut rb, "ijkl".as_bytes()));
        assert_eq!(0, write(&mut rb, "mnop".as_bytes()));
        assert_eq!(control.data, rb.data);
    }
    #[test]
    fn rb_write_read() {
        let mut rb: RingBuffer = create(10);
        assert_eq!(4, write(&mut rb, "abcd".as_bytes()));
        assert_eq!(4, write(&mut rb, "efgh".as_bytes()));
        assert_eq!(2, write(&mut rb, "ijkl".as_bytes()));
        print(rb.clone());

        assert_eq!("abcd".as_bytes(), read(&mut rb, 4));
        print(rb.clone());
        assert_eq!("efgh".as_bytes(), read(&mut rb, 4));
        print(rb.clone());
        assert_eq!("ij".as_bytes(), read(&mut rb, 4));
        print(rb.clone());

        assert_eq!(4, write(&mut rb, "abcd".as_bytes()));
        print(rb.clone());
        assert_eq!(4, write(&mut rb, "efgh".as_bytes()));
        print(rb.clone());
        read(&mut rb, 4);
        print(rb.clone());
        assert_eq!(4, write(&mut rb, "1234".as_bytes()));
        print(rb.clone());
    }

    // Пример API, вызовов и как меняется состояние буффера:
    // [ _ _ _ ] create(3)
    // [ a b _ ] write "ab" -> return 2
    // [ a b c ] write "cd" -> return 1
    // [ _ b c ] read(1) -> return "a"
    // [ e b c ] write "e" -> return 1
    // [ e _ _ ] read(2) -> return "bc"
    // Ваша задача написать такой буффер и добавить тесты

    #[test]
    fn test() {
        let mut rb = create(3);
        println!("{:?}", String::from_utf8(rb.data.clone()));
        assert_eq!(2, write(&mut rb, "ab".as_bytes()));
        println!("{:?}", String::from_utf8(rb.data.clone()));
        assert_eq!(1, write(&mut rb, "cd".as_bytes()));
        println!("{:?}", String::from_utf8(rb.data.clone()));
        assert_eq!("a".as_bytes(), read(&mut rb, 1));
        println!("{:?}", String::from_utf8(rb.data.clone()));
        assert_eq!(1, write(&mut rb, "e".as_bytes()));
        println!("{:?}", String::from_utf8(rb.data.clone()));
        assert_eq!("bc".as_bytes(), read(&mut rb, 2));
        println!("{:?}", String::from_utf8(rb.data.clone()));
    }
}
