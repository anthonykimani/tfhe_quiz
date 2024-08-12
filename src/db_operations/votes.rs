use crate::models::votes::{NewVote, Vote};
use diesel::prelude::*;

pub fn get_all_votes(connection: &mut PgConnection) -> Vec<Vote> {
    use crate::schema::votes::dsl::*;

    let mut all_votes: Vec<Vote> = Vec::new();
    let results = votes
        .select(Vote::as_select())
        .load(connection);
    match results {
        Ok(data) => {
            for user in data.into_iter() {
                all_votes.push(user)
            }

            println!("todo")
        }
        Err(e) => println!("Error occured {:?}", e)
    }

    return all_votes;
}

pub fn get_a_user_by_id(connection: &mut PgConnection, user_id: i32) -> Option<Vote> {
    use crate::schema::votes::dsl::*;

    votes
        .filter(id.eq(user_id))
        .first::<Vote>(connection)
        .optional() // This will convert the result to Option
        .unwrap_or_else(|err| {
            println!("Error occurred: {:?}", err);
            None
        })
}


pub fn add_user(new_user: NewVote, connection: &mut PgConnection) -> Result<Vote, diesel::result::Error>{
    diesel::insert_into(crate::schema::votes::table)
        .values(&new_user)
        // .returning(Post::as_returning())
        .get_result::<Vote>(connection)
}