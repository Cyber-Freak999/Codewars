fn main() {
    for i in 1..5 {
        println!("{}", i*2);
    }
    let sum = solution(10);
    println!("{}", sum);
}

fn solution(num: i32) -> i32 {
    let mut array = vec![];
    for i in 1..num {
        if i%3 == 0 {
            array.push(i)
        } else if i%5 == 0 {
            array.push(i)
        }
    }
    return array.iter().fold(0, |acc, x| acc + x);
}
