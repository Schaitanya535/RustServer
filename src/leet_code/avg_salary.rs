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

fn build_array(nums: Vec<i32>) -> Vec<i32> {
    let mut ans: Vec<i32> = Vec::with_capacity(nums.len());
    for &i in nums.iter() {
        ans.push(nums[i as usize]);
    }
    ans
}

pub fn defang_ip_addr(address: String) -> String {
    address.replace('.', "[.]")
}

pub fn solve() {
    let vector = vec![1000, 2000, 3000];
    let vector2 = vec![0, 2, 1];
    avg_salary(vector);
    build_array(vector2);
    defang_ip_addr("1.2.3.4".to_owned());
    // let arr1 = vec![1, 2, 3];
    // let mut arr1 = arr1;
    // let arr2: Vec<i32> = vec![3, 4, 4];
    // arr1.extend(arr2);
}
