pub trait Messenger {
    // 不変なselfへの参照
    fn send(&self, msg: &str);
}

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

    // これをテストしたいが、assertできるような値を返すわけではない
    // sendを呼び出した時に、そのメッセージを追跡できるようなモックオブジェクトが欲しい
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
                sent_messages: RefCell::new(vec![]), // RefCellは現在活動中のRef<T>とRefMut<T>スマートポインタの数を追いかける
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            // pushするためには self.sent_messages を可変借用しないといけないが、selfは不変
            // self.sent_messages.push(String::from(msg));

            // borrow_mut() により、 RefMut<T>が返る。 Vec<String> への可変参照を得る
            let mut messages = self.sent_messages.borrow_mut();
            // Automatic Referencing and Dereferencing
            // メソッド呼び出しは自動でデリファレンスされるので * は不要
            messages.push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);

        // borrow() により、 Ref<T> が返る
        let messages = mock_messenger.sent_messages.borrow();
        assert_eq!(messages.len(), 1);
    }
}
