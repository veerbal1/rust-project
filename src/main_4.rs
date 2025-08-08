use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Post {
    userId: u32,
    id: u32,
    title: String,
    body: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = "https://jsonplaceholder.typicode.com/posts";
    let posts: Vec<Post> = reqwest::get(url)
        .await? // इन-ऑर्डर वेट करो जब तक response न आ जाए
        .json() // JSON डिकोड करो
        .await?;

    for (size, post) in posts.iter().enumerate() {
        println!("Size:{size} | ID: {} | Title: {} | Id: {} | Body: {}", post.id, post.title, post.userId, post.body);
        if size == 0 {
            return Ok(())
        }
    }

    Ok(())
}
