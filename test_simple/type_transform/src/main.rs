
/// 各种类型转Vec<u8>
fn type_change_to_vec_u8() {
    let c:char = '\u{01f355}'; // utf8 字符
    let mystring = String::from("whb");
    let tmpstr = "tmpstr";
    let string2 = tmpstr.to_string();
    let u32v:u32 = 123456u32;


    let u: usize = 6666666;
    let u2: String = u.to_string();      //usize 转 String
    let char_to_string = c.to_string();  // char 转String
    let string_to_vec_u8 = mystring.clone().into_bytes();
    let mut _static_str_to_vec_u8 = "SET".to_string().into_bytes();
    let u32_to_string = u32v.to_string().into_bytes();
    let char_to_vec_u8 = c.to_string().into_bytes();

    println!("tmpstr={}, string2={}", tmpstr, string2);
    println!("char 转 u8 可能存在数据丢失，所以char最好转为Vec<u8>");
    println!("char={:#?} ={}  转 Vec<u8>={:#?}", c,c, char_to_vec_u8);
    println!("usize={:#?}    转  String={:#?}",u, u2 );
    println!("char={:#?}     转 String={:#?}",c, char_to_string );
    println!("u32={:#?}    转  Vec<u8>={:#?}",u32v, u32_to_string );
    println!("string={:#?}  转 Vec<u8>={:#?}", mystring, string_to_vec_u8);
    println!("字符串直接转Vec<u8> \"SET\".to_string().into_bytes() ={:#?}",_static_str_to_vec_u8);

}

fn main() {
    type_change_to_vec_u8();
}
