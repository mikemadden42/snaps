use rand::seq::IndexedRandom;
use rand::{rng, Rng};

const WORDS: &[&str] = &["begin", "great", "suddenly", "before", "enough", "probably"];

fn pick_word<R: Rng + ?Sized>(rng: &mut R) -> Option<&'static str> {
    WORDS.choose(rng).copied()
}

fn main() {
    if let Some(word) = pick_word(&mut rng()) {
        println!("{word}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picks_word_from_list() {
        let mut rng = rng();
        for _ in 0..100 {
            let word = pick_word(&mut rng).unwrap();
            assert!(WORDS.contains(&word));
        }
    }
}
