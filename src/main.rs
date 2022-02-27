use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let words = ["begin", "great", "suddenly", "before", "enough", "probably"];
    let mut rng = thread_rng();
    // https://www.dotnetperls.com/if-rust
    if let Some(word) = words.choose(&mut rng) {
        println!("{word}");
    }
}
