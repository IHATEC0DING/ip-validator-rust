pub fn is_valid_ip(ip: &str) -> bool {
    let vec: Vec<&str> = ip.split('.').collect();

    // 4-part string check
    if vec.len() != 4 {
        return false; // check fail
    }

    let int_vec: Result<Vec<i32>, _> = vec.iter().map(|s| s.parse::<i32>()).collect();
    
    match int_vec {
        Ok(int_vec) => {
            // all ip parts are <=255
            // also leading zeros check
            int_vec.iter().enumerate().all(|(i, &x)| {
                let part = vec[i]; // vec for string
                x >= 0 && x <= 255 && part == format!("{}", x) // leading zeros check
            })
        },
        Err(_) => false, // if parsing failed return false
    }
} 