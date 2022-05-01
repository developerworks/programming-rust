// 计算正数的个数, 负数的总和
fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    if input.is_empty() {
        return vec![];
    }

    input.iter().fold(vec![0, 0], |mut acc, &x| {
        if x > 0 {
            acc[0] += 1;
        } else {
            acc[1] += x;
        }
        acc
    })
}

fn main() {
    let test_data1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15];
    let result = count_positives_sum_negatives(test_data1);
    println!("result: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        let test_data1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15];
        let expected1 = vec![10, -65];
        assert_eq!(count_positives_sum_negatives(test_data1), expected1);
        
        
        let test_data2 = vec![0, 2, 3, 0, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14];
        let expected2 = vec![8, -50];
        assert_eq!(count_positives_sum_negatives(test_data2), expected2);
        
    }
    
    #[test]
    fn wrong_values () {
        let test_data: Vec<i32> = vec![];
        let expected: Vec<i32> = vec![];
        assert_eq!(count_positives_sum_negatives(test_data), expected);
        
    }
}