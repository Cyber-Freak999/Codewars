fn spin_words(words: &str) -> String {
   words
      .split_whitespace()
      .map(|word| {
         if word.len() >= 5 {
            word.chars().rev().collect::<String>()
         } else {
             word.to_string()
         }
      })
      .collect::<Vec<String>>()
      .join(" ")
}

fn main() {
   let test1 = "hello my fellow warriors";
   let test2 = "This is a test";
   let test3 = "This is another test";
   println!("{}", spin_words(test1));
   println!("{}", spin_words(test2));
   println!("{}", spin_words(test3));
}
