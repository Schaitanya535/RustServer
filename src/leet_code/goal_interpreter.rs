pub fn solve() {
    interpret("G()(al)".to_owned());
    interpret2("G()(al)".to_owned());
    // println!("interpreted value is : {}", ans);
    // println!("interpreted value is : {}", ans2);
}

pub fn interpret(command: String) -> String {
    let mut stream = command.chars().peekable();
    let mut ans = String::new();

    while let Some(curr) = stream.next() {
        let next = stream.peek();
        match (curr, next) {
            ('G', _) => {
                ans += "G";
            }
            ('(', Some(')')) => {
                ans += "o";
            }
            ('(', Some('a')) => {
                ans += "al";
            }
            (_, _) => {}
        }
    }
    ans
}

pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    accounts.iter().map(|x| x.iter().sum()).max().unwrap()
}

pub fn interpret2(command: String) -> String {
    command.replace("()", "o").replace("(al)", "al")
}

pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    nums.iter()
        .map(|&x| nums.iter().filter(|&&y| x > y).count() as i32)
        .collect()
}

pub fn most_words_found(sentences: Vec<String>) -> i32 {
    sentences
        .iter()
        .map(|sentence| sentence.split_whitespace().count() as i32)
        .max()
        .unwrap()
}

pub fn subtract_product_and_sum(n: i32) -> i32 {
    let (sum, prod) = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .fold((0, 1), |(sum, prod), i| (sum + i, prod * i));
    prod - sum
}

pub fn minimum_sum(num: i32) -> i32 {
    let mut vec: Vec<i32> = num
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap_or(0) as i32)
        .collect();
    vec.sort();
    vec[2] + 10 * vec[0] + 10 * vec[1] + vec[3]
}

pub fn xor_operation(n: i32, start: i32) -> i32 {
    (0..n).fold(0, |val, curr| (val) ^ (curr * 2 + start))
}

pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
    let mut decoded = vec![first];
    encoded.iter().fold(first, |acc, curr| {
        let next = acc ^ curr;
        decoded.push(next);
        next
    });
    decoded
}

pub fn left_right_difference_2(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let left = nums.iter().take(len - 1).fold(vec![0], |mut acc, curr| {
        acc.push(acc.last().unwrap() + curr);
        acc
    });
    let right: Vec<i32> = nums
        .into_iter()
        .rev()
        .take(len - 1)
        .fold(vec![0], |mut acc, curr| {
            acc.push(acc.last().unwrap() + curr);
            acc
        })
        .into_iter()
        .rev()
        .collect();
    // std::iter::zip(left, right)
    left.iter()
        .zip(right)
        .into_iter()
        .map(|(a, b)| (a - b).abs())
        .collect()
}

pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
    if nums.len() == 1 {
        return vec![0];
    };
    let mut ans = vec![0; nums.len()];
    for i in 1..nums.len() {
        ans[i] = ans[i - 1] + nums[i - 1];
    }
    let mut acc = 0;
    for i in (0..=(nums.len() - 2)).rev() {
        acc += nums[i + 1];
        ans[i] = (ans[i] - acc).abs()
    }
    ans
}
