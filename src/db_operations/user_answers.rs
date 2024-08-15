use diesel::prelude::*;
use crate::models::user_answer::{UserAnswer, NewUserAnswer};
use crate::schema::user_answers;

pub fn get_all_user_answers(connection: &mut PgConnection) -> Vec<UserAnswer> {
    use crate::schema::user_answers::dsl::*;

    let mut all_user_answers: Vec<UserAnswer> = Vec::new();
    let results = user_answers
        .select(UserAnswer::as_select())
        .load(connection);
    match results {
        Ok(data) => {
            for user_answer in data.into_iter() {
                all_user_answers.push(user_answer)
            }
            println!("Fetched all user answers successfully.");
        }
        Err(e) => println!("Error occurred: {:?}", e)
    }

    return all_user_answers;
}

pub fn get_a_user_answer_by_id(connection: &mut PgConnection, user_answer_id: i32) -> Option<UserAnswer> {
    use crate::schema::user_answers::dsl::*;

    user_answers
        .filter(id.eq(user_answer_id))
        .first::<UserAnswer>(connection)
        .optional()
        .unwrap_or_else(|err| {
            println!("Error occurred: {:?}", err);
            None
        })
}

pub fn add_user_answer(new_user_answer: NewUserAnswer, connection: &mut PgConnection) -> Result<UserAnswer, diesel::result::Error> {
    diesel::insert_into(crate::schema::user_answers::table)
        .values(&new_user_answer)
        .get_result::<UserAnswer>(connection)
}


pub fn get_user_answer_by_id(connection: &mut PgConnection, user_answer_id: i32) -> Option<UserAnswer> {
    use crate::schema::user_answers::dsl::*;

    user_answers
        .filter(id.eq(user_answer_id))
        .first::<UserAnswer>(connection)
        .optional()
        .unwrap_or_else(|err| {
            println!("Error occurred: {:?}", err);
            None
        })
}

// pub fn update_user_answer_by_id(user_answer_id: i32, updated_user_answer: UserAnswer, connection: &mut PgConnection) -> Result<UserAnswer, diesel::result::Error> {
//     use crate::schema::user_answers::dsl::*;
//
//     diesel::update(user_answers.find(user_answer_id))
//         .set(&updated_user_answer)
//         .get_result::<UserAnswer>(connection)
// }
// pub fn delete_user_answer_by_id(user_answer_id: i32, connection: &mut PgConnection) -> Result<usize, diesel::result::Error> {
//     use crate::schema::user_answers::dsl::*;
//
//     diesel::delete(user_answers.find(user_answer_id)).execute(connection)
// }