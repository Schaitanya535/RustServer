fn avg_salary(salaries: Vec<i32>) -> f64 {
    let mut min = 100_00_01;
    let mut max = 999;
    let mut sum = 0;

    for salary in &salaries {
        if salary < &min {
            min = *salary;
        }
        if salary > &max {
            max = *salary;
        }
        sum += salary;
    }

    return ((sum - min - max) as f64) / ((salaries.len() - 2) as f64);
}

pub fn solve() {
    avg_salary(vec![1000, 2000, 3000]);
    // println!("{}", ans);
}
