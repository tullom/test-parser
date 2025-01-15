

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use rand::Rng;
    use sha2::{ Sha256, Digest};

    use super::*;

    #[test]
    fn it_works() {

        let mut rng = rand::thread_rng();
        let random_bytes: [u8; 32] = rng.gen();

        let mut hasher = Sha256::new();
        hasher.update(random_bytes);
        let result = hasher.finalize();
        let hash = String::from_utf8_lossy(&result[..]);
        println!("{}", hash);
        for byte in &result[..] {
            print!("{}", byte);
        }
    }
}
