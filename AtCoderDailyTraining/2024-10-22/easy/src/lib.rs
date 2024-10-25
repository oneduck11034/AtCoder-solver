pub fn q_a(number: u64) -> u64 {
    let mut result_n= 0_u64;
    for i in 1..=number{
        result_n+= number * u64::pow(10,i as u32);
    }

    let string_number= result_n.to_string();
    let str_number= &string_number;
    let sliced_str= &str_number[..str_number.len() - 1];

    sliced_str.parse::<u64>().unwrap()    
}

#[cfg(test)]
mod tests_a {
    use super::*;

    #[test]
    fn it_works() {
        let result = q_a(9);
        assert_eq!(result, 999999999);
    }
}
