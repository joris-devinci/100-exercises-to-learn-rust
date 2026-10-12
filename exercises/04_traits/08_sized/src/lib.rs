pub fn example() -> usize {
    // Trying to get the size of a str (or any other DST)
    // via `std::mem::size_of` will result in a compile-time error.
    //
    // TODO: Comment out the following line and move on to the next exercise.
    let size: usize = std::mem::size_of::<String>();
    println!("{size}");
    return size
}

#[cfg(test)]
mod test {
    
    #[test]
    fn test() {
        use super::example;

        assert_eq!(24, example());   
    }
}