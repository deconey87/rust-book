fn main() {
    let mut num = 5;

    // safeな参照
    let r1 = &num;
    println!("r1 is: {}", *r1);
    let r2 = &mut num;
    println!("r1 is: {}", *r2);

    // 生ポインタを作ること自体は危険でない
    // また、不変な参照と可変な参照が同時に作れるのも生ポインタだから
    let r1_unsafe = &num as *const i32;
    let r2_unsafe = &mut num as *mut i32;
    unsafe {
        println!("r1_unsafe is: {}", *r1_unsafe);
        println!("r2_unsafe is: {}", *r2_unsafe);
    }

    // unsafeな関数はunsafeブロックで呼び出す
    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    // 可変なスライス（v全体の）
    let r = &mut v[..];
    // split_at_mutは、呼び出しはsafeだが内部ではunsafeな処理をしている
    // let (a, b) = r.split_at_mut(3);
    let (a, b) = split_at_mut(r, 3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

unsafe fn dangerous() {}

// safeなコードだけでは、split_at_mut()は実装できない
// なぜなら、可変な借用が2つ発生してしまうから
// スライスの範囲が被らないことは借用チェッカーはわからない
// fn split_at_mut(value: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = value.len();
//     assert!(mid <= len);
//     (&mut value[..mid], &mut value[mid..])
// }

// 実装はunsafeを含むが、関数自体はsafeであることに注目
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    // 可変な生ポインタを生成
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            // スライスは連続したメモリ領域なので、ptrにmidの文を普通に足していい
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// Cのabsを呼び出せるようにしている
extern "C" {
    fn abs(input: i32) -> i32;
}
