use rand::Rng;

pub fn random_byte_array(length: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut result: Vec<u8> = Vec::new();
    for _ in 0..length {
        result.push(rng.gen())
    }
    return result;
}
