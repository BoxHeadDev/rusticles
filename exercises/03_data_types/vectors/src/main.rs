fn main() {
    vectors_1();
    vectors_2();
    vectors_3();
    vectors_4();
    vectors_5();
    vectors_6();
    vectors_7();
    vectors_8();
    vectors_9();
    vectors_10();

    iterator_1();
    iterator_2();
    iterator_3();
}

fn vectors_1() {
    // Create an empty vector of integers

    // Add integers to the vector (eg. 5,6,7,8)
}

fn vectors_2() {
    // Create a populated vector of integers
    let v2 = ??;

    // Assign the third element in the vector (index)
    let third= ??;
    println!("The third element is {third}");

    // Assign the third element in the vector (method)
    // let third = ??;
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}

fn vectors_3() {
    // What do each of the following return below?
    let v = vec![1, 2, 3, 4, 5];
    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
}

fn vectors_4() {
    // What is wrong with the following?
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);
    println!("The first element is: {first}");
}

fn vectors_5() {
    // Loop over v and print each element after adding 1
    let v = vec![100, 32, 57];

    let n_plus_one: i32 = *n_ref + 1;
    println!("{n_plus_one}");
}

fn vectors_6() {
    // Loop over v and add 50 to each element
    let mut v = vec![100, 32, 57];

}

fn vectors_7() {
    // What is wrong with the following?
    let v = vec![String::from("Hello ")];
    let mut s = v[0];
    s.push_str("world");
    println!("{s}");
}

fn vectors_8() {
    // Fix the following? (Vectors can only have one type)
    let row = vec![3),String::from("blue"),10.12];
}

fn vectors_9() {
    // Does the following compile?
    let mut v = vec![1, 2, 3];
    for i in &mut v {
        v.push(*i);
    }
    println!("{} {} {}", v[3], v[4], v[5]);
}

fn vectors_10() {
    // What is the output?
    let mut v: Vec<i32> = vec![1, 2, 3];
    let mut v2: Vec<&mut i32> = Vec::new();
    for i in &mut v {
        v2.push(i);
    }
    *v2[0] = 5;
    let a = *v2[0];
    let b = v[0];
    println!("{a} {b}");
}

fn iterator_1() {
    // What are the values of the following Iterator?
    let mut v: Vec<i32> = vec![1, 2];
    let mut iter: Iter<'_, i32> = v.iter();
    let n1: &i32 = iter.next().unwrap();
    let n2: &i32 = iter.next().unwrap();
    let end: Option<&i32> = iter.next();
}

fn iterator_2() {
    // Iterate over a vector without using a pointer
    let mut v: Vec<i32> = vec![1, 2];
    let mut iter: ? = ??;
    let i1: usize = iter.next().unwrap();
    let n1: &i32 = &v[i1];
}

fn iterator_3() {
    // Why does the following not compile?
    fn dup_in_place(v: &mut Vec<i32>) {
        for n_ref in v.iter() {
            v.push(*n_ref);
        }
    }
}
