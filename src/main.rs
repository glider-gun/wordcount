//! wordcount はシンプルな文字、単語、行の出現頻度の計数機能を提供します。
//! 詳しくは[`count`](wordcount::count) 関数のドキュメントを見てください。
#![warn(missing_docs)]

use std::env;
use std::fs::File;
use std::io::BufReader;

use wordcount::count;

/* pub fn count(input: impl BufRead) -> HashMap<String, usize> {
    let re = Regex::new(r"\w+").unwrap();
    let mut freqs = HashMap::new(); // Hashmap<String, usize>型

    for line in input.lines() {
        let line = line.unwrap();
        for m in re.find_iter(&line) {
            let word = m.as_str().to_string();

            *freqs.entry(word).or_insert(0) += 1;
        }
    }
    freqs
}
 */
fn main() {
    let filename = env::args().nth(1).expect("1 argument FILENAME required");

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(&file);

    let freqs = count(reader, Default::default());
    println!("{:?}", freqs);
}
