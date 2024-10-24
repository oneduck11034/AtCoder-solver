pub fn q_a(n: i64, c: i64, t_v: Vec<i64>) -> i64 {
    let mut result_candy= 1_i64;
    let mut cnt= 0_usize;
    let mut gap_time= 0_i64;
    let mut pre_time= 0_i64;
    let v_for_calc_pre_time= t_v.clone();

    let mut cnt= 0_usize;
    for e_t in t_v {
        if cnt != 0 {
            gap_time= e_t - pre_time;
            if gap_time >= c {
                result_candy+=1;
                gap_time= 0;
            }
            pre_time= v_for_calc_pre_time[cnt];
        }else if cnt == 0{
            pre_time= 1;
        }

        cnt+=1;
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

