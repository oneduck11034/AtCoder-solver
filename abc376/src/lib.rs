pub fn q_a(n: i64, c: i64, t_v: Vec<i64>) -> i64 {
    let mut result_candy= 0_i64;
    let mut spand_time= 0_i64;
    let mut cnt= 0_usize;

    for e_t in t_v {
        if cnt != 0 {
            
            if spand_time >= c {
                result_candy+=1;
                spand_time=0;
            }
        }else if cnt == 0{
            result_candy+=1;
        }
        
        cnt+1;
    }

    result_candy
}

#[cfg(test)]
mod tests_a {
    use super::*;

    #[test]
    fn it_works() {
        let result = q_a(
            6
            ,5
            ,Vec::from([1,3, 7, 8, 10, 12])
        );
        assert_eq!(result, 3);
    }
    
    #[test]
    fn it_works2() {
        let result = q_a(
            3
            ,2
            ,Vec::from([0,2, 4])
        );
        assert_eq!(result, 3);
    }

    #[test]
    fn it_works3() {
        // 10 3
        let result = q_a(
            10
            ,3
            ,Vec::from([0, 3 ,4, 6, 9, 12, 15, 17, 19, 20])
        );
        assert_eq!(result, 7);
    }
}

