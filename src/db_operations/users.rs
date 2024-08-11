use crate::models::users::{NewUser, User};
use diesel::prelude::*;

pub fn get_all_users(connection: &mut PgConnection) -> Vec<User> {
    use crate::schema::users::dsl::*;

    let mut all_users: Vec<User> = Vec::new();
    let results = users
        .select(User::as_select())
        .load(connection);
    match results {
        Ok(data) => {
            for user in data.into_iter() {
                all_users.push(user)
            }

            println!("todo")
        }
        Err(e) => println!("Error occured {:?}", e)
    }

    return all_users;
}

pub fn get_a_user_by_id(connection: &mut PgConnection, user_id: i32) -> Option<User> {
    use crate::schema::users::dsl::*;

    users
        .filter(id.eq(user_id))
        .first::<User>(connection)
        .optional() // This will convert the result to Option
        .unwrap_or_else(|err| {
            println!("Error occurred: {:?}", err);
            None
        })
}


pub fn add_user(new_user: NewUser, connection: &mut PgConnection) -> Result<User, diesel::result::Error>{
    diesel::insert_into(crate::schema::users::table)
        .values(&new_user)
        // .returning(Post::as_returning())
        .get_result::<User>(connection)
}