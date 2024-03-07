#[allow(warnings)]
pub fn add_groupings(mut number: u64) -> String {
    let mut result = String::new();
    let mut count = 0;

    loop {
        result.insert(0, (b'0' + (number % 10) as u8) as char);
        number /= 10;
        count += 1;

        if number == 0 {
            break;
        }

        if count % 3 == 0 {
            result.insert(0, ',');
        }
    }

    result
}

pub fn add_groupings_usize(number: usize) -> String {
    let mut result = Vec::new();
    let mut count = 0;

    let mut num = number as u64;
    loop {
        result.push((b'0' + (num % 10) as u8) as char);
        num /= 10;
        count += 1;

        if num == 0 {
            break;
        }

        if count % 3 == 0 {
            result.push(',');
        }
    }

    result.iter().rev().collect()
}

pub fn add_groupings_u32(number: u32) -> String {
    let mut result = Vec::new();
    let mut count = 0;

    let mut num = number as u64;
    loop {
        result.push((b'0' + (num % 10) as u8) as char);
        num /= 10;
        count += 1;

        if num == 0 {
            break;
        }

        if count % 3 == 0 {
            result.push(',');
        }
    }

    result.iter().rev().collect()
}

pub fn add_groupings_u64(number: u64) -> String {
    let mut result = Vec::new();
    let mut count = 0;

    let mut num = number;
    loop {
        result.push((b'0' + (num % 10) as u8) as char);
        num /= 10;
        count += 1;

        if num == 0 {
            break;
        }

        if count % 3 == 0 {
            result.push(',');
        }
    }

    result.iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_groupings_with_u64() {
        assert_eq!(add_groupings(1234567890), "1,234,567,890");
        assert_eq!(add_groupings(987654321), "987,654,321");
        assert_eq!(add_groupings(123), "123");
        assert_eq!(add_groupings(0), "0");
    }

    #[test]
    fn test_add_groupings_with_usize() {
        assert_eq!(add_groupings_usize(1234567890), "1,234,567,890");
        assert_eq!(add_groupings_usize(987654321), "987,654,321");
        assert_eq!(add_groupings_usize(123), "123");
        assert_eq!(add_groupings_usize(0), "0");
    }

    #[test]
    fn test_add_groupings_with_u32() {
        assert_eq!(add_groupings_u32(1234567890), "1,234,567,890");
        assert_eq!(add_groupings_u32(987654321), "987,654,321");
        assert_eq!(add_groupings_u32(123), "123");
        assert_eq!(add_groupings_u32(0), "0");
    }
}
