// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests2` for hints :)



#[cfg(test)]
mod tests {
    
    #[test]
    fn you_can_assert_eq() {
        let vc1 = vec![1,2,3];
        let vc2 = vec![1,2,3];
        assert_eq!(vc1,vc2);
    }
}
