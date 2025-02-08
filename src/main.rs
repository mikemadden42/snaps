use rand::rng;
use rand::seq::IndexedRandom;

fn main() {
    let words = ["begin", "great", "suddenly", "before", "enough", "probably"];
    let mut rng = rng();
    // https://www.dotnetperls.com/if-rust
    if let Some(word) = words.choose(&mut rng) {
        println!("{word}");
    }
}
