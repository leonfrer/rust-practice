mod state_pattern;
mod trait_object_mod;

fn main() {
    use state_pattern::*;

    let mut post: Post = Post::new();

    post.add_text("some text");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("some text", post.content());
}
