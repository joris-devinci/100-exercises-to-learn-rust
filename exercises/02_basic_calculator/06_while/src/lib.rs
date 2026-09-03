// Rewrite the factorial function using a `while` loop.
pub fn factorial(n: u32) -> u32 {
    
    let mut val: u32 = n; 

    if val == 0 {
        return 1;
    } else {
        let mut res: u32 = 1;

        while val > 0 {
            res = res * val;
            val -= 1;
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
