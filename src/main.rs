use sha2::{Sha256, Digest};
use std::hash::{Hash, Hasher};

fn main() {
    struct Person {
        id: u32,
        name: String,
        phone: u64,
    }

    impl Hash for Person {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.id.hash(state);
            self.phone.hash(state);
            self.name.hash(state);
        }
    }

    let person1 = Person {
        id: 5,
        name: "Janet".to_string(),
        phone: 555_666_7777,
    };
    let person2 = Person {
        id: 5,
        name: "Janet".to_string(),
        phone: 555_666_7777,
    };

    fn calculate_hash<T: Hash>(t: &T) -> String {
        let mut hasher = Sha256::new();
        let mut state = std::collections::hash_map::DefaultHasher::new();
        t.hash(&mut state);
        let hash_value = state.finish().to_ne_bytes();
        hasher.update(&hash_value);
        let result = hasher.finalize();
        hex::encode(result)
    }

    assert_eq!(calculate_hash(&person1), calculate_hash(&person2));
    println!("{}", calculate_hash(&person1));
    println!("{}", calculate_hash(&person2));
}
