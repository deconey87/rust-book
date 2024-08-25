use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];
    // v[99];

    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        // マッチガード: この ifが成立した時に実行される
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => {
                panic!("Tried to create file but there was a problem: {:?}", e);
            }
        },
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error);
        }
    };
    println!("{:?}", f);

    // 失敗したら panic! を呼び出してくれる
    // let f = File::open("hello2.txt").unwrap();
    // expect() は、、「この操作が成功することを期待している。もし期待が外れたら、具体的なエラーメッセージを出してパニックに陥る」という意味合い
    // let f2 = File::open("hello2.txt").expect("Failed to open hello2.txt");

    let f1 = read_username_from_file().expect("invalid user name");
    println!("name: {}", f1);

    let f2 = read_username_from_file2().expect("invalid user name");
    println!("name: {}", f2);
}

// Resultを返す関数によって、呼び出し元にエラーハンドリングを任せることができる
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// もっと簡潔な書き方もある
fn read_username_from_file2() -> Result<String, io::Error> {
    // Resultの直後に ? をおくと、
    // ResultがOkなら、その値をunwrapする
    // Errなら、Errをearly returnする
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    // 1行にまとめることもできる
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
