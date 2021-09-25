mod blog;

use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("Some text");

    let post = post.request_review();
    let post = post.approve();

    assert_eq!("Some text", post.content());
}
