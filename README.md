# CodeWars-Jumping-Number-Special-Numbers-Series-4--7-kyu---Passed-
Definition
Jumping number is the number that All adjacent digits in it differ by 1.

Task
Given a number, Find if it is Jumping or not .

Warm-up (Highly recommended)
Playing With Numbers Series
Notes
Number passed is always Positive .

Return the result as String .

The difference between ‘9’ and ‘0’ is not considered as 1 .

All single digit numbers are considered as Jumping numbers.

Input >> Output Examples
jumpingNumber(9) ==> return "Jumping!!"
Explanation:
It's single-digit number
jumpingNumber(79) ==> return "Not!!"
Explanation:
Adjacent digits don't differ by 1
jumpingNumber(23) ==> return "Jumping!!"
Explanation:
Adjacent digits differ by 1
jumpingNumber(556847) ==> return "Not!!"
Explanation:
Adjacent digits don't differ by 1
jumpingNumber(4343456) ==> return "Jumping!!"
Explanation:
Adjacent digits differ by 1
jumpingNumber(89098) ==> return "Not!!"
Explanation:
Adjacent digits don't differ by 1
jumpingNumber(32) ==> return "Jumping!!"
Explanation:
Adjacent digits differ by 1



TEST CASE:
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(jumping_number(1), "Jumping!!");
        assert_eq!(jumping_number(7), "Jumping!!");
        assert_eq!(jumping_number(9), "Jumping!!");
        assert_eq!(jumping_number(23), "Jumping!!");
        assert_eq!(jumping_number(32), "Jumping!!");
        assert_eq!(jumping_number(79), "Not!!");
        assert_eq!(jumping_number(98), "Jumping!!");
        assert_eq!(jumping_number(8987), "Jumping!!");
        assert_eq!(jumping_number(4343456), "Jumping!!");
        assert_eq!(jumping_number(98789876), "Jumping!!");
    }
    
    use rand::Rng;
    
    #[test]
    fn random() {
        let mut rng = rand::thread_rng();
        
        for _ in 0..200 {
            let rand_max = 10u64.pow(rng.gen_range(1..7));
            let n = rng.gen_range(1..rand_max);
            
            assert_eq!(jumping_number(n), jumping_number_solution(n));
        }
    }
    
    fn jumping_number_solution(n: u64) -> String {
        let is_jumping = n.to_string()
            .as_bytes()
            .windows(2)
            .all(|bs| bs[0] + 1 == bs[1] || bs[1] + 1 == bs[0]);
    
        if is_jumping { "Jumping!!" } else { "Not!!" }.to_owned()
    }
}
