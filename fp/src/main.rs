use std::{thread, time::Duration};

fn main() {
    let user_specified_value = 10;
    let random_number = 7;

    generate_workout(user_specified_value, random_number);

    // 環境をキャプチャする
    let x = 4;
    let y = 4;

    // クロージャ ... equal_to_xと同じスコープで定義されているxの値を使用できている
    // クロージャの種類は3つ
    // FnOnce ... 環境から所有権を奪って使う
    // FnMut ... 可変で借用する
    // Fn ... 不変で借用する 今回はFn（xの値を読み取るだけなので）
    let equal_to_x = |z| z == x;
    assert!(equal_to_x(y));

    // 関数 ... これは動かない
    // fn equal_to_x2(z: i32) -> bool {
    //     z == x
    // }
    // assert!(equal_to_x2(y));

    // イテレーター
    let v1 = vec![1, 2, 3];
    // ループがv1_iterの所有権を奪い、 陰で可変にする
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {}", val);
    }

    // この場合は可変にする（nextを呼び出すことで、イテレーターを使い込むから）
    let mut v1_iter2 = v1.iter();
    assert_eq!(v1_iter2.next(), Some(&1));
    assert_eq!(v1_iter2.next(), Some(&2));
    assert_eq!(v1_iter2.next(), Some(&3));
    assert_eq!(v1_iter2.next(), None);

    // mapは新しいイテレーターを生成するだけで、消費はしない ... イテレータアダプタ
    // collect() を呼び出すことでイテレーターは消費される（nextを呼び出している） ... 消費アダプタ
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // クロージャーを渡している
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        // 重い処理を2回呼び出すが、Cacherでキャッシュしているので大丈夫
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("take a break");
        } else {
            println!(
                "Today, run for {} minutes",
                expensive_result.value(intensity)
            );
        }
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, show_size: u32) -> Vec<Shoe> {
    // show_sizeはクロージャーの引数でないがキャプチャしている
    shoes.into_iter().filter(|s| s.size == show_size).collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ]
    );
}

// イテレータを実装する
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}
