// Calculates the power of 2 using a bit shift.
// `1 << n` is equivalent to "2 to the power of n".
fn power_of_2(n: u8) -> u64 {
    1 << n
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        // TODO: Test the function `power_of_2` with some values.
        let result1 = power_of_2(1);
        let result2 = power_of_2(2);
        let result3 = power_of_2(5);
        let result4 = power_of_2(10);
        assert_eq!(result1, 2);
        assert_eq!(result2, 4);
        assert_eq!(result3, 32);
        assert_eq!(result4, 1024);
    }
}
