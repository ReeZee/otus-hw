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

#[derive(Clone)]
pub struct RingBuffer {
    read_idx: usize,
    write_idx: usize,
    data: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub enum RBErrors {
    NoSpaceLeft,
}

pub fn create(size: usize) -> RingBuffer {
    RingBuffer {
        read_idx: 0,
        write_idx: 0,
        data: Vec::with_capacity(size),
    }
}

pub fn write(rb: &mut RingBuffer, elements: &[u8]) -> Result<usize, RBErrors> {
    let mut written = 0;
    for element in elements {
        // if buffer full - stop writing
        if rb.data.len() == rb.data.capacity() {
            continue;
        }

        // Set write_idx equal to last read_idx when write_idx can't increase due to capacity
        if rb.write_idx == rb.data.capacity() {
            rb.write_idx = rb.read_idx;
        }
        // Insert and increment
        rb.data.insert(rb.write_idx, *element);
        written += 1;
        rb.write_idx += 1;
    }
    match written {
        0 => Err(RBErrors::NoSpaceLeft),
        written => Ok(written),
    }
}

pub fn read(rb: &mut RingBuffer, num_of_elements: usize) -> Option<Vec<u8>> {
    let mut elements: Vec<u8> = Vec::with_capacity(num_of_elements);
    // null read idx
    rb.read_idx = 0;

    for _ in 0..num_of_elements {
        // check if there is data to read and add the read data to a temporary vector
        if rb.read_idx < rb.data.len() {
            elements.push(rb.data[rb.read_idx]);
            rb.read_idx += 1;
        }
    }
    // clean the read data from original vector
    for _ in 0..rb.read_idx {
        rb.data.remove(0);
    }

    // If the vector is empty - reset the pointers
    if rb.data.is_empty() {
        rb.write_idx = 0;
        rb.read_idx = 0;
    }
    // Keep write pointer smaller than the length
    rb.write_idx -= rb.read_idx;

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
        assert_eq!(0, rb.data.len());
        assert_eq!(10, rb.data.capacity());
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

/*
//Circular buffer with ovetwrite

pub fn write_v2(rb: &mut RingBuffer, elements: &[u8]) -> usize {
    let mut written = 0;
    for element in elements {
        if rb.data.len() < rb.data.capacity() {
            rb.data.push(*element);
            written += 1;
            continue;
        }
        rb.data[rb.write_idx] = *element;
        if rb.write_idx < rb.data.capacity() - 1 {
            rb.write_idx += 1
        } else {
            rb.write_idx = 0
        };
        written += 1;
    }
    written
}

pub fn read_v2(rb: &mut RingBuffer, num_of_elements: usize) -> Vec<u8> {
    let mut elements: Vec<u8> = Vec::with_capacity(num_of_elements);
    for _ in 0..num_of_elements {
        elements.push(rb.data.remove(0));
        rb.read_idx += 1;
    }
    elements
}

*/
