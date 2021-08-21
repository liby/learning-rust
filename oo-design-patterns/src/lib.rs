pub enum PostStates {
    StateDraft,
    StatePendingFirstReview,
    StatePendingSecondReview,
    StatePublished,
    KeepState,
}

pub struct Post {
    state: Box<dyn State>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Box::new(Draft {}),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content
            .push_str(self.state.as_ref().text_to_append(text));
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().content(self)
    }

    pub fn check_state_change(&mut self, post_state: PostStates) {
        match post_state {
            PostStates::KeepState => {}
            PostStates::StateDraft => self.state = Box::new(Draft {}),
            PostStates::StatePendingFirstReview => self.state = Box::new(PendingFirstReview {}),
            PostStates::StatePendingSecondReview => self.state = Box::new(PendingSecondReview {}),
            PostStates::StatePublished => self.state = Box::new(Published {}),
        }
    }
    pub fn request_review(&mut self) {
        self.check_state_change(self.state.as_ref().request_review());
    }

    pub fn approve(&mut self) {
        self.check_state_change(self.state.as_ref().approve());
    }

    pub fn reject(&mut self) {
        self.check_state_change(self.state.as_ref().reject());
    }
}

trait State {
    fn request_review(&self) -> PostStates {
        PostStates::KeepState
    }

    fn approve(&self) -> PostStates {
        PostStates::KeepState
    }

    fn reject(&self) -> PostStates {
        PostStates::KeepState
    }
    fn text_to_append<'a>(&self, _text: &'a str) -> &'a str {
        ""
    }

    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(&self) -> PostStates {
        PostStates::StatePendingFirstReview
    }
    fn text_to_append<'a>(&self, text: &'a str) -> &'a str {
        text
    }
}

struct PendingFirstReview {}

impl State for PendingFirstReview {
    fn approve(&self) -> PostStates {
        PostStates::StatePendingSecondReview
    }

    fn reject(&self) -> PostStates {
        PostStates::StateDraft
    }
}

struct PendingSecondReview {}

impl State for PendingSecondReview {
    fn approve(&self) -> PostStates {
        PostStates::StatePublished
    }

    fn reject(&self) -> PostStates {
        PostStates::StateDraft
    }
}

struct Published {}

impl State for Published {
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());
        post.add_text("Random text at wrong place");
        post.reject(); // back to Draft
        post.add_text(". Reviewed text!");
        post.request_review(); // back to Pending review

        post.approve();
        assert_eq!("", post.content());
        post.approve();
        assert_eq!(
            "I ate a salad for lunch today. Reviewed text!",
            post.content()
        );
    }
}
