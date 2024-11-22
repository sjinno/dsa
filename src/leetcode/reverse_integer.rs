fn reverse_integer(x: i32) -> i32 {
    let mut num = x;
    let mut reversed_x = 0;

    let bound = i32::MAX / 10;

    while num != 0 {
        let digit = num % 10;

        if bound < reversed_x || (bound == reversed_x && digit > 7) {
            return 0;
        }
        if -bound > reversed_x || (-bound == reversed_x && digit < -8) {
            return 0;
        }

        reversed_x = reversed_x * 10 + digit;
        num /= 10;
    }

    reversed_x
}
