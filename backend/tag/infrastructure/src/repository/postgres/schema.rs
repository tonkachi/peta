table! {
    tags (hash) {
        hash -> Varchar,
        name -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    tags_fusen_ids (tag_hash, fusen_id) {
        tag_hash -> Varchar,
        fusen_id -> Varchar,
        created_at -> Timestamptz,
    }
}

joinable!(tags_fusen_ids -> tags (tag_hash));

allow_tables_to_appear_in_same_query!(tags, tags_fusen_ids,);
