fn jumping_number(n: u64) -> String 
{
    let vec_num: Vec<u8> = n.to_string()
                           .chars()
                           .map(|c| c.to_digit(10).unwrap() as u8)
                           .collect();
                           
    if vec_num.len() == 1 { return "Jumping!!".to_string(); }
    else
    {
        let diff_cons_nums :Vec<u8> = vec_num
                                      .windows(2)
                                      .map(|w| w[0].abs_diff(w[1]))
                                      .collect();
        
        // Returns true if every element is 1
        let all_ones = diff_cons_nums.iter().all(|&x| x == 1);
        
        if all_ones == true { return "Jumping!!".to_string(); }
        else { return "Not!!".to_string(); }
                             
    }
}
