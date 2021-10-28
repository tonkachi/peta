use crate::repository::postgres::schema::{tags, tags_fusen_ids};
use chrono::{DateTime, Utc};

#[derive(Queryable, Identifiable)]
#[primary_key(hash)]
#[table_name = "tags"]
pub struct Tag {
    pub hash: String,
    pub name: String,
    pub created_at: DateTime<Utc>, // MEMO: knowledgeとして残す
    pub updated_at: DateTime<Utc>, // MEMO: knowledgeとして残す
}

#[derive(Insertable, Identifiable)]
#[primary_key(hash)]
#[table_name = "tags"]
pub struct InsertTagData {
    pub hash: String,
    pub name: String,
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(Tag, foreign_key = "tag_hash")]
#[primary_key(tag_hash, fusen_id)]
#[table_name = "tags_fusen_ids"]
pub struct TagFusenId {
    pub tag_hash: String,
    pub fusen_id: String,
    pub created_at: DateTime<Utc>,
}
