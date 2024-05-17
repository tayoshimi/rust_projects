// BitMask Sample

fn composition_u32(v1:i16, v2:i16) -> u32 {
    let low = 0x0000ffff & (v1 as u32);
    let high = 0x0000ffff & (v2 as u32);
    
    return (low | ( high << 16 )) as u32
}

fn get_low_value( val:u32 ) -> i16 {
    (0x0000ffff & val) as i16
}

fn get_high_value( val:u32 ) -> i16 {
    ( (0xffff0000 & val) >> 16) as i16
}

fn main() {
    let val1:i16 = 12;
    let val2:i16 = -333;

    println!("before: {}:{:016b}, {}:{:016b}", &val1, &val1, &val2, &val2);

    // 2つの16ビットの値を32ビットの値へ合成する
    let comp_val = composition_u32(val1, val2);

    // 1つ目は下位16ビット、２つ目は上位16ビットに詰まってる
    println!("comp val: {}", comp_val);

    // 合成した変数から2つの値を取り出す
    println!("after: {}, {}", get_low_value(comp_val), get_high_value(comp_val));


}
