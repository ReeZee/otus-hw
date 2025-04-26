/*
Описание/Пошаговая инструкция выполнения домашнего задания:
Возьмем уже полюбившийся нам код из задания RingBuffer.
В текущей реализации интерфейс очень C-подобен, давайте сделаем его более Rust-way.
Пусть при записи мы возвращаем Ok(кол-во успешно записанных байт), если мы хоть что-то записали в буффер, и типизированную ошибку NoSpaceLeft, если мы ничего не записали в буффер.
То же самое касается чтения: если мы что-то прочитали, то давайте возвращать Some(прочитанный буффер), если же буффер был пуст, то будем возвращать None.
Ваша задача: поправить вашу реализацию RingBuffer (включая тесты) как указано выше.


Критерии оценки:
«Принято» — задание выполнено полностью.
«Возвращено на доработку» — задание не выполнено полностью.

Критерии оценивания задачи:

исправлены необходимые методы и их сигнатуры;
исправлены тесты;
абстракции корректны, код отформатирован, cargo fmt и cargo clippy не дают warnings.
*/

#[derive(Debug, PartialEq)]
pub enum RBErrors {
    NoSpaceLeft,
}

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

pub fn write(rb: &mut RingBuffer, elements: &[u8]) -> Result<usize, RBErrors> {
    if elements.is_empty() {
        return Ok(0);
    }
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
    match written {
        0 => Err(RBErrors::NoSpaceLeft),
        written => Ok(written),
    }
}

pub fn read(rb: &mut RingBuffer, num_of_elements: usize) -> Option<Vec<u8>> {
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
    if elements.is_empty() {
        return None;
    }
    Some(elements)
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
        write(&mut rb, "abcdefghij".as_bytes()).unwrap();
        print(rb.clone());
        rb
    }

    #[test]
    fn rb_created() {
        let rb: RingBuffer = create(10);
        assert_eq!(10, rb.data.len());
        assert_eq!(10, rb.data.capacity());
    }

    #[test]
    fn rb_write_empty() {
        let mut rb: RingBuffer = create(10);
        assert_eq!(Ok(0), write(&mut rb, "".as_bytes()));
    }

    #[test]
    fn rb_read_empty() {
        let mut rb: RingBuffer = create(10);
        assert!(read(&mut rb, 4).is_none());
        assert!(read(&mut rb, 14).is_none());
    }

    #[test]
    fn rb_read_until_empty() {
        let mut rb: RingBuffer = create_populated_rb();
        assert_eq!("abcd".as_bytes(), read(&mut rb, 4).unwrap());
        assert_eq!("efgh".as_bytes(), read(&mut rb, 4).unwrap());
        assert_eq!("ij".as_bytes(), read(&mut rb, 4).unwrap());
        assert!(read(&mut rb, 4).is_none());
    }

    #[test]
    fn rb_write_full() {
        let mut rb: RingBuffer = create(10);
        let control = create_populated_rb();
        assert_eq!(4, write(&mut rb, "abcd".as_bytes()).unwrap());
        assert_eq!(6, write(&mut rb, "efghij".as_bytes()).unwrap());
        assert_eq!(control.data, rb.data);
    }

    #[test]
    fn rb_write_overflow() {
        let mut rb: RingBuffer = create(10);
        let control = create_populated_rb();
        assert_eq!(4, write(&mut rb, "abcd".as_bytes()).unwrap());
        assert_eq!(4, write(&mut rb, "efgh".as_bytes()).unwrap());
        assert_eq!(2, write(&mut rb, "ijkl".as_bytes()).unwrap());
        assert_eq!(
            RBErrors::NoSpaceLeft,
            write(&mut rb, "mnop".as_bytes()).unwrap_err()
        );
        assert_eq!(control.data, rb.data);
    }

    #[test]
    fn rb_write_read() {
        let mut rb: RingBuffer = create(10);
        assert_eq!(4, write(&mut rb, "abcd".as_bytes()).unwrap());
        assert_eq!(4, write(&mut rb, "efgh".as_bytes()).unwrap());
        assert_eq!(2, write(&mut rb, "ijkl".as_bytes()).unwrap());
        print(rb.clone());

        assert_eq!("abcd".as_bytes(), read(&mut rb, 4).unwrap());
        print(rb.clone());
        assert_eq!("efgh".as_bytes(), read(&mut rb, 4).unwrap());
        print(rb.clone());
        assert_eq!("ij".as_bytes(), read(&mut rb, 4).unwrap());
        assert!(read(&mut rb, 14).is_none());
        print(rb.clone());

        assert_eq!(4, write(&mut rb, "abcd".as_bytes()).unwrap());
        print(rb.clone());
        assert_eq!(4, write(&mut rb, "efgh".as_bytes()).unwrap());
        print(rb.clone());
        read(&mut rb, 4);
        print(rb.clone());
        assert_eq!(4, write(&mut rb, "1234".as_bytes()).unwrap());
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
        assert_eq!(2, write(&mut rb, "ab".as_bytes()).unwrap());
        assert_eq!(1, write(&mut rb, "cd".as_bytes()).unwrap());
        assert_eq!("a".as_bytes(), read(&mut rb, 1).unwrap());
        assert_eq!(1, write(&mut rb, "e".as_bytes()).unwrap());
        assert_eq!("bc".as_bytes(), read(&mut rb, 2).unwrap());
    }
}
