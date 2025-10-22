#[cfg(feature = "heapless")]
mod heapless_tests {
    use core::fmt::Debug;

    use collections2::{Collection as _, Error, List as _, ListMut};

    fn new_list<T: PartialEq + Debug, const N: usize>() -> impl ListMut<T> {
        heapless::Vec::<T, N>::new()
    }

    #[test]
    fn test_push_at_capacity() {
        let mut vec = new_list::<u32, 3>();

        // Fill to capacity
        assert!(vec.push(1).is_ok());
        assert!(vec.push(2).is_ok());
        assert!(vec.push(3).is_ok());

        // Try to push beyond capacity
        let result = vec.push(4);
        assert_eq!(result, Err(Error::InsertFailed(4u32)));

        // Verify vec still has original contents
        assert_eq!(vec.len(), 3);
        assert_eq!(vec.as_slice(), &[1, 2, 3]);
    }

    #[test]
    fn test_insert_at_capacity() {
        let mut vec = new_list::<u32, 3>();

        // Fill to capacity
        vec.push(1).unwrap();
        vec.push(2).unwrap();
        vec.push(3).unwrap();

        // Try to insert beyond capacity
        let result = vec.insert(1, 99);
        assert_eq!(result, Err(Error::InsertFailed(99)));

        // Verify vec still has original contents
        assert_eq!(vec.len(), 3);
        assert_eq!(vec.as_slice(), &[1, 2, 3]);
    }

    #[test]
    fn test_insert_success() {
        let mut vec: heapless::Vec<u32, 4> = heapless::Vec::new();

        vec.push(1).unwrap();
        vec.push(3).unwrap();

        // Insert should succeed
        let result = vec.insert(1, 2);
        assert!(result.is_ok());

        assert_eq!(vec.len(), 3);
        assert_eq!(vec.as_slice(), &[1, 2, 3]);
    }

    #[test]
    fn test_append_exceeds_capacity() {
        let mut vec1: heapless::Vec<u32, 3> = heapless::Vec::new();
        let mut vec2: heapless::Vec<u32, 3> = heapless::Vec::new();

        vec1.push(1).unwrap();
        vec1.push(2).unwrap();

        vec2.push(3).unwrap();
        vec2.push(4).unwrap();

        // Total would be 4 elements, but capacity is 3
        let result = vec1.append(&mut vec2);
        assert_eq!(result, Err(Error::CapacityExceeded));

        // vec1 should be unchanged
        assert_eq!(vec1.as_slice(), &[1, 2]);
        // vec2 should still have its elements since append failed
        assert_eq!(vec2.as_slice(), &[3, 4]);
    }

    #[test]
    fn test_append_success() {
        let mut vec1: heapless::Vec<u32, 5> = heapless::Vec::new();
        let mut vec2: heapless::Vec<u32, 5> = heapless::Vec::new();

        vec1.push(1).unwrap();
        vec1.push(2).unwrap();

        vec2.push(3).unwrap();
        vec2.push(4).unwrap();

        let result = vec1.append(&mut vec2);
        assert!(result.is_ok());

        assert_eq!(vec1.as_slice(), &[1, 2, 3, 4]);
        assert_eq!(vec2.len(), 0);
    }

    #[test]
    fn test_push_pop_roundtrip() {
        let mut vec: heapless::Vec<String, 2> = heapless::Vec::new();

        let s = String::from("hello");
        vec.push(s.clone()).unwrap();

        let popped = vec.pop();
        assert_eq!(popped, Some(s));
    }

    #[test]
    fn test_push_failed_returns_original() {
        let mut vec = new_list::<String, 1>();

        let s1 = String::from("first");
        vec.push(s1).unwrap();

        let s2 = String::from("second");
        let result = vec.push(s2.clone());

        // Should get back our original string
        assert_eq!(result, Err(Error::InsertFailed(s2)));
    }
}

#[cfg(feature = "alloc")]
mod vec_tests {
    use core::fmt::Debug;

    use collections2::{Collection as _, List as _, ListMut};
    extern crate alloc;
    use alloc::vec::Vec;

    fn new_list<T: PartialEq + Debug>() -> impl ListMut<T> {
        Vec::<T>::new()
    }

    #[test]
    fn test_vec_push_success() {
        let mut vec = new_list();

        assert!(vec.push(1).is_ok());
        assert!(vec.push(2).is_ok());
        assert!(vec.push(3).is_ok());

        assert_eq!(vec.as_slice(), &[1, 2, 3]);
    }

    #[test]
    fn test_vec_insert_success() {
        let mut vec = new_list();

        vec.push(1).unwrap();
        vec.push(3).unwrap();

        assert!(vec.insert(1, 2).is_ok());
        assert_eq!(vec.as_slice(), &[1, 2, 3]);
    }

    #[test]
    fn test_vec_append_success() {
        let mut vec1 = new_list();
        let mut vec2 = new_list();

        vec1.push(1).unwrap();
        vec1.push(2).unwrap();

        vec2.push(3).unwrap();
        vec2.push(4).unwrap();

        assert!(vec1.append(&mut vec2).is_ok());
        assert_eq!(vec1.as_slice(), &[1, 2, 3, 4]);
        assert_eq!(vec2.len(), 0);
    }

    #[test]
    fn test_vec_with_strings() {
        let mut vec = new_list();

        vec.push(String::from("hello")).unwrap();
        vec.push(String::from("world")).unwrap();

        assert_eq!(vec.len(), 2);
        assert_eq!(vec.get(0).unwrap(), "hello");
        assert_eq!(vec.get(1).unwrap(), "world");
    }
}
