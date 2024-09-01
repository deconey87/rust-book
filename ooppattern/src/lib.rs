pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        // as_ref() により、Option<&Box<dyn State>> を得る
        let self_ref = self.state.as_ref();
        // unwrap() で &Box<dyn State>
        // unwrap()はpanicしない（実装上）
        let unwrapped = self_ref.unwrap();
        // これをcontent() を呼び出すときに、deref coercion が働いて、
        // Stateトレイトを実装している型、が要求される
        unwrapped.content(self)
    }

    pub fn request_review(&mut self) {
        // take() によって、所有権が移り、元の位置にはNoneが残る
        // それから代入することで、古いstateが使えないことを保証する
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    // Box<> に包まれた型の時だけ有効になる
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    // デフォルト実装
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // postへの参照を引数にとって、それの一部への参照（文字列スライス）を返すから、
    // ライフタイムが必要
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
