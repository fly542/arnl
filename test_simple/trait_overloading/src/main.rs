pub trait AddU8<T> {
    fn addu8(&mut self, v: T) -> &mut Self;
}

#[derive(Debug)]
struct Foo {
    data: Vec<u8>
}

impl Foo {
    fn new() -> Self {
        Foo {
            data : Vec::new(),
        }
    }
}

impl AddU8<i32> for Foo {
    fn addu8(&mut self, x: i32) -> &mut Self {
        self.data.append(&mut x.to_string().into_bytes());
        self
    }
}

impl AddU8<&mut Vec<u8>> for Foo {
    fn addu8(&mut self, x:&mut Vec<u8>) -> &mut Self {
        self.data.append(x); //x 的类型已经是&mut 所以不用再增加&mut
        self
    }
}

fn main() {
    let mut tmp:Vec<u8> = vec![1,2,3];
    let mut x = Foo::new();
    let tmp2 = &mut tmp; //tmp2已经是一个&mut 类型
    x.addu8(32);
    //x.addu8(&mut  tmp);
    x.addu8(tmp2); //所以此处不用再写&mut
    println!("x={:#?}", x);
}
