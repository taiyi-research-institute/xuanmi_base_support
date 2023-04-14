use rand::{self, Rng};

const dict: [char; 35] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', // 10 digits
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
    'k', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u',
    'v', 'w', 'x', 'y', 'z'
];

pub fn id09akmz(len: usize) -> String {
    assert!(len >= 1, "The ID you want to generate should have at least 1 character");
    let mut rng = rand::thread_rng();
    let mut id = String::new();
    let mut x = rng.gen_range(10..35);
    id.push(dict[x]);
    for _ in 1..len {
        x = rng.gen_range(0..35);
        id.push(dict[x]);
    }
    return id;
}

#[cfg(test)]
mod tests {
    use super::id09akmz;

    #[test]
    fn test_id09akmz() {
        let id = id09akmz(16);
        println!("{}", &id);
    }
}