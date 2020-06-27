fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
fn main() {
    let s = String::from("hello");
    let bytes = s.into_bytes();
    assert_eq!(&[104, 101, 108, 108, 111][..], &bytes[..]);
    print_type_of(&bytes);
    let mut mb = bytes;
    print_type_of(&mb);
    print_type_of(&mut mb);
    let mut x = &mut mb;
    print_type_of(&x);
    let y = &mut x;
    print_type_of(&y);
}
