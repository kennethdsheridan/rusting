struct Solution {}


impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        // check if empty
        if matrix.is_empty() || matrix[0].is_empty() {
            return false;
        }

        // set the boundaries
        let m = matrix.len(); // # of matrix rows
        let n = matrix[0].len(); // # of matrix columns

        let mut left = 0; 
        let mut right = m * n - 1;

        while left <= right {
            let mid = left + (right - left) / 2;

            let row = mid / n; 
            let col = mid % n;
            
            let mid_value = matrix[row][col];

            match mid_value.cmp(&target) {
                std::cmp::Ordering::Greater => {
                    right = mid - 1;
                }
                std::cmp::Ordering::Equal => {
                    return true;
                }
                std::cmp::Ordering::Less => {
                    left = mid + 1;
                }
            }

         }
        false
    }
}


fn main() {

    let vec = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
    ];
    

    let result = Solution::search_matrix(vec, 7);

    println!("Solution is: {}", result);
}
