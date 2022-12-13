pub fn hello() -> &'static str {
    let ans: &str = "Hello World!";
    return ans;
}

pub fn is_leap_year(yr: i32) -> bool {
    if yr % 4 == 0 {
        if yr % 100 == 0 {
            if yr % 400 == 0 {
                return true;
            } else {
                return false;
            }
        } else {
            return true;
        }
    } else {
        return false;
    }
}
