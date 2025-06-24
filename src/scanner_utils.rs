
pub fn is_digit(c: char) -> bool {

    (c >= '0' && c <= '9')

}

pub fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z')
}


pub fn is_alphanumeric(c: char) -> bool {

    is_digit(c) || is_alpha(c)
}