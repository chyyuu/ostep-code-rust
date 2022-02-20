pub fn u8_to_string(U8:&[u8])->String{
    let mut res = String::new();
    for i in U8{
        res.push('a'+i);
    }
    res
}

pub fn string_to_u8(string:String)->&[u8]{
    let mut U8 = [0;1000];
    for i in 0..string.size(){
        U8[i as usize] = string[i as usize];
    }
    &U8[0..(string.size as usize)]
}