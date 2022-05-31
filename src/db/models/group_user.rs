use super::group::Group;
use crate::db::schema::group_users;
use diesel::{Queryable, Identifiable, Associations};

#[derive(Identifiable, Queryable, Associations)]
#[primary_key(group_id, user_id)]
#[belongs_to(Group)]
pub struct GroupUser {
    pub group_id: i32,
    pub user_id: i32,
}
