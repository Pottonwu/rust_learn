pub mod sequence1{
    pub fn sort(start : char,end : char){
        let char_bool  : bool = start > end;
        let mut acsll_num : u32 = start as u32;
        let mut length: u32 = match char_bool{
            true => start as u32 - end as u32 +1,
            false => end as u32 - start as u32 +1,

        } ;

        if char_bool{
            while length != 0{
                if let Some(x) = std::char::from_u32(acsll_num ){
                println!("{}",x);
                }
                acsll_num -= 1;
                length -= 1;
            }
        }
        else{
            while length != 0{
                if let Some(x) = std::char::from_u32(acsll_num){
                println!("{}",x);
                }
                acsll_num += 1;
                length -= 1;
            }
        }

    }

}