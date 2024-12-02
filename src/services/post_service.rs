use serde::{Deserialize, Serialize};
use crate::services::HttpClient;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub userId: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PostDetails {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub userId: i32,
    pub comments: Vec<Comment>,
    pub user: User,
    pub author_text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Comment {
    pub id: i32,
    pub postId: i32,
    pub name: String,
    pub email: String,
    pub body: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub username: String,
    pub email: String,
}

pub struct PostService {
    http_client: HttpClient,
}

impl PostService {
    pub fn new() -> Self {
        Self {
            http_client: HttpClient::new(),
        }
    }

    pub async fn get_posts(&self) -> Result<Vec<Post>, reqwest::Error> {
        self.http_client.get("/posts").await
    }

    pub async fn get_post(&self, id: i32) -> Result<Post, reqwest::Error> {
        self.http_client.get(&format!("/posts/{}", id)).await
    }

    pub async fn get_post_details(&self, id: i32) -> Result<PostDetails, reqwest::Error> {
        let post = self.get_post(id).await?;
        let comments: Vec<Comment> = self.http_client.get(&format!("/posts/{}/comments", id)).await?;
        let user: User = self.http_client.get(&format!("/users/{}", post.userId)).await?;

        Ok(PostDetails {
            id: post.id,
            title: post.title,
            body: post.body,
            userId: post.userId,
            comments,
            user: user.clone(),
            author_text: format!("By: {}", user.name),
        })
    }
}

impl Default for PostService {
    fn default() -> Self {
        Self::new()
    }
} 