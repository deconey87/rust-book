// RefCell<T>
// 借用規則が実行時に確認される
// RefCell<T>が不変でも、 RefCell<T>内の値を可変化できる

// 使い所
// 不変参照にせざるを得ないが、可変したい場合（このサンプル）

pub trait Messenger {
    // selfの不変な参照をとる
    fn send(&self, msg: &str);
}

// 'a はライフタイム、LimitTrackerのインスタンスはmessengerの参照よりも長く生きられない
// TはMessengerトレイトを実装していないといけない（トレイト境界）
pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    // set_valueは値を返さないが、どうテストする？
    // → Messengerトレイトを実装するモックを作って、そちらにデータを保持する
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        // sent_messages: Vec<String>,
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // selfが不変借用なのでsent_messagesにpushできないが、
            // &mut selfにすると Messengerトレイトの定義と合わなくなる
            // self.sent_messages.push(String::from(message));

            // borrow_mut() によって、中の値（ベクター）の可変参照を得ることができる
            let mut mut_vec = self.sent_messages.borrow_mut();
            // 同じスコープで可変借用が二つできるようにわざと実装すると、
            // 普通、コンパイルエラーになるが、RefCellは実行時に借用規則を確認するので、コンパイルできる。代わりに、実行時にpanicする
            // let mut mut_vec2 = self.sent_messages.borrow_mut();
            mut_vec.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // borrow() は不変参照を得る
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
