// BitMask Struct Sample

pub struct CompositionVal(u32);

impl CompositionVal {
    pub fn new(val1: i16, val2: i16) -> CompositionVal {
        let low = 0x0000ffff & (val1 as u32);
        let high =  0x0000ffff & (val2 as u32);
        let comp_val = (low | (high << 16)) as u32;
        CompositionVal(comp_val)
    }

    pub fn val(&self) -> u32 {
        self.0
    }

    pub fn low(&self) -> i16 {
        (0x0000ffff & self.0) as i16
    }

    pub fn high(&self) -> i16 {
        ( (0xffff0000 & self.0) >> 16 ) as i16
    }
}

#[test]
fn composition_val_test() {
    let val1:i16 = 12;
    let val2:i16 = -333;
    let comp_val = CompositionVal::new(val1, val2);

    assert_eq!(
        comp_val.low(),
        val1
    );
    assert_eq!(
        comp_val.high(),
        val2
    );
}


fn main() {
    let val1:i16 = 12;
    let val2:i16 = -333;

    println!("before: {}:{:016b}, {}:{:016b}", &val1, &val1, &val2, &val2);

    // 2つの16ビットの値を32ビットの値へ合成する
    let comp_val = CompositionVal::new(val1, val2);

    // 1つ目は下位16ビット、２つ目は上位16ビットに詰まってる
    println!("comp val: {}", comp_val.val());

    // 合成した変数から2つの値を取り出す
    println!("after: {}, {}", comp_val.low(), comp_val.high());
}
