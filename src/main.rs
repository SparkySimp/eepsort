use eepsort::eepy_sort;

fn main() {
    let numbers: Vec<i64> = vec![1, 6, 10, 273, 562, 1269, 1236, 1237, 1471, 12783];
    println!("{:?}", eepy_sort(numbers));
}

#[test]
fn ensure_integrity() {
    let numbers: Vec<i64> = vec![6, 10, 8, 273, 562, 1269, 1237, 1471, 1236, 12783];
    let mut sorted_numbers = vec![];
    let expected_order = vec![6i64, 8i64, 10i64, 273i64, 562i64, 1236i64, 1237i64, 1471i64, 12783i64];

    for _ in 0..=100 {
        sorted_numbers.push(eepy_sort(numbers.clone()));
    }

    let mut all_equal = true;
    
    for v in sorted_numbers {
        all_equal = all_equal && (v == expected_order);
    }

    assert!(all_equal, "Not all vectors are in the order they are expected to be in.");
}
