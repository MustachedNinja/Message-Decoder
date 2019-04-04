#[cfg(test)]
mod unique {

    use crate::unique;

    // Test empty Vec input. Expect a count of 0
    #[test]
    fn empty_vec() {
        let test_vec: Vec<i32> = Vec::new();
        let output: i32 = unique(&test_vec);
        assert_eq!(output, 0);
    }

    // Test a Vec of length 1. Expect a count of 1
    #[test]
    fn len_one() {
        let test_vec: Vec<i32> = [1].to_vec();
        let output: i32 = unique(&test_vec);
        assert_eq!(output, 1);
    }

    // Test a Vec full of duplicates. Expect a count of 1
    #[test]
    fn all_duplicates() {
        let test_vec: Vec<i32> = [1, 1, 1, 1].to_vec();
        let output: i32 = unique(&test_vec);
        assert_eq!(output, 1);
    }

    // Test a Vec of length 5 with all 5 unique. Expect a count of 5
    #[test]
    fn all_unique() {
        let test_vec: Vec<i32> = [1, 2, 3, 4, 5].to_vec();
        let output: i32 = unique(&test_vec);
        assert_eq!(output, 5);
    }

    // Test a Vec with some repeated numbers. Expect a count of 3
    #[test]
    fn standard_input() {
        let test_vec: Vec<i32> = [1, 1, 2, 3, 3].to_vec();
        let output: i32 = unique(&test_vec);
        assert_eq!(output, 3);
    }
}

mod most_variety {

    use crate::most_variety;

    // Test a Vec of emtpy Vec. Expect an index of 0
    #[test]
    fn emtpy_vec() {
        let test_vec: Vec<Vec<i32>> = vec![vec![], vec![], vec![]];
        let output: i32 = most_variety(&test_vec);
        assert_eq!(output, 0);
    }

    // Test a Vec of Vecs all length 1. Expect an index of 1
    #[test]
    fn len_one() {
        let test_vec = vec![vec![1], vec![2], vec![3]];
        let output: i32 = most_variety(&test_vec);
        assert_eq!(output, 0);
    }

    // Test a Vec of with only one full Vec
    #[test]
    fn one_val() {
        let test_vec = vec![vec![1], vec![], vec![]];
        let output: i32 = most_variety(&test_vec);
        assert_eq!(output, 0);
    }
    
    // Test a Vec with 3 different uniqueness values
    #[test]
    fn dif_uniqueness() {
        let test_vec = vec![ vec![1], vec![1, 2], vec![1, 2, 3]];
        let output: i32 = most_variety(&test_vec);
        assert_eq!(output, 2);
    }
}
// abra