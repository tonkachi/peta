use crate::postgres::schema::fusens;

#[derive(Queryable, Debug)]
pub struct FusenModel {
    pub id: String,
    pub title: String,
    pub note: String,
}

#[derive(Insertable)]
#[table_name = "fusens"]
pub struct NewFusenModel {
    pub id: String,
    pub title: String,
    pub note: String,
}
