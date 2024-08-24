struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {

        let (mut a, mut b, mut ways) = (0, 1, 0);

        for i in 1..=n {

            ways = a + b; 

            a = b; 

            b = ways;
        }
        // return the ways
        ways
    }
}


fn main() {

    // run the logic
    //
    let input = 4;
    let result = Solution::climb_stairs(input);

    println!("There are {:?} ways to climb {input} stairs", result);
}


// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_climb_stairs_1() {
        assert_eq!(Solution::climb_stairs(1), 1);
    }
    
    #[test]
    fn test_climb_stairs_2() {
        assert_eq!(Solution::climb_stairs(4), 5);
    }
}
