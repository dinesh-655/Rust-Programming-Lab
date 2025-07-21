//8. Create array of 10 elements and implement slicing operations
fn main() {
    let arr = [1,2,3,4,5,6,7,8,9,10];

    // a. Slice of 2nd and 3rd element (index 1 and 2)
    let slice_a = &arr[1..3];
    println!("Slice a: {:?}", slice_a);

    // b. Omit start index of slice (start to 4th element)
    let slice_b = &arr[..4];
    println!("Slice b: {:?}", slice_b);

    // c. Omit end index of slice (5th element to end)
    let slice_c = &arr[4..];
    println!("Slice c: {:?}", slice_c);

    // d. Omit both start and end (whole array)
    let slice_d = &arr[..];
    println!("Slice d: {:?}", slice_d);
}
