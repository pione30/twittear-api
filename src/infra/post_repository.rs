use crate::domain::model::{IPostRepository, NewPost, Post, User};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct PostRepository {
    conn: Arc<Mutex<PgConnection>>,
}

impl PostRepository {
    pub fn new(conn: Arc<Mutex<PgConnection>>) -> Self {
        PostRepository { conn }
    }
}

impl IPostRepository for PostRepository {
    fn create<'a>(&self, body: &'a str, user: &'a User) -> QueryResult<Post> {
        use crate::schema::posts;

        let new_post = NewPost {
            body,
            user_id: &user.id,
        };

        diesel::insert_into(posts::table)
            .values(&new_post)
            .get_result(&*self.conn.lock().unwrap())
    }

    fn pagenate_posts_of_user<'a>(
        &self,
        user: &'a User,
        limit: i64,
        offset: i64,
    ) -> QueryResult<Vec<Post>> {
        use crate::schema::posts;
        use crate::schema::users;

        posts::table
            .inner_join(users::table)
            .filter(users::id.eq(&user.id))
            .select(posts::all_columns)
            .order_by(posts::created_at.desc())
            .limit(limit)
            .offset(offset)
            .load(&*self.conn.lock().unwrap())
    }
}
