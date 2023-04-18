use rand::{self, Rng};
use uuid::Uuid;

const dict35: [char; 35] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', // 10 digits
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
    'k', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u',
    'v', 'w', 'x', 'y', 'z', // 25 lower-case alphabets
];

const dict58: [char; 58] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', // 9 digits
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 
    'k', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 
    'v', 'w', 'x', 'y', 'z', // 25 lower-case alphabets
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K',
    'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V',
    'W', 'X', 'Y', 'Z'
];


pub fn randid_base35(len: usize) -> String {
    assert!(
        len >= 1,
        "The ID you want to generate should have at least 1 character"
    );
    let mut rng = rand::thread_rng();
    let mut id = String::new();
    let mut x = rng.gen_range(10..35);
    id.push(dict35[x]);
    for _ in 1..len {
        x = rng.gen_range(0..35);
        id.push(dict35[x]);
    }
    return id;
}

pub fn uuid_base35() -> String {
    let bytes = *Uuid::new_v4().as_bytes();
    let uuid: u128 = u128::from_be_bytes(bytes);
    let mut id = ['0'; 25]; // 128 / log2(35) = 24.95
    let base = dict35.len() as u128;
    let mut n = uuid;
    for i in 0..25 {
        let digit = (n % base) as usize;
        id[24 - i] = dict35[digit];
        n /= base;
        if n == 0 {
            break;
        }
    }
    return id.iter().collect();
}

pub fn uuid_base58() -> String {
    let bytes = *Uuid::new_v4().as_bytes();
    let uuid: u128 = u128::from_be_bytes(bytes);
    let mut id = ['0'; 22]; // 128 / log2(58) = 21.85
    let base = dict58.len() as u128;
    let mut n = uuid;
    for i in 0..22 {
        let digit: usize = (n % base) as usize;
        id[21 - i] = dict58[digit];
        n /= base;
        if n == 0 {
            break;
        }
    }
    return id.iter().collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_randid_base35() {
        let id = randid_base35(16);
        println!("{}", &id);
    }

    #[test]
    fn test_uuid_base35() {
        let id = uuid_base35();
        println!("{}", &id);
    }

    #[test]
    fn test_uuid_base58() {
        let id = uuid_base58();
        println!("{}", &id);
    }
}
