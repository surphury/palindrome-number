mod test;

pub fn is_palindrome(x: i32) -> bool {
    if x.is_negative() {
        return false;
    };

    let reverse = x.to_string().chars().rev().collect::<String>();

    x.to_string() == reverse
}

fn main() {}
