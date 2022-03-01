// hashmap1.rs
// A basket of fruits in the form of a hash map needs to be defined.
// The key represents the name of the fruit and the value represents
// how many of that particular fruit is in the basket. You have to put
// at least three different types of fruits (e.g apple, banana, mango)
// in the basket and the total count of all the fruits should be at
// least five.
//
// Make me compile and pass the tests!
//
// Execute the command `rustlings hint hashmap1` if you need
// hints.



use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    // TODO: declare your hash map here.

    // Two bananas are already given for you :)
    let fruits = vec![String::from("banana"),String::from("mango"),String::from("apple")];
    let count = vec![2,2,1];
    let mut basket: HashMap<_,_> = 
                    fruits.into_iter().zip(count.into_iter()).collect();

    // TODO: Put more fruits in your basket here.

    basket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
