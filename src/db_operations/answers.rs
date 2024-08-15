use diesel::prelude::*;
use crate::models::answer::{Answer, NewAnswer};
pub fn get_all_answers(connection: &mut PgConnection) -> Vec<Answer> {
    use crate::schema::answers::dsl::*;

    let mut all_answers: Vec<Answer> = Vec::new();
    let results = answers
        .select(Answer::as_select())
        .load(connection);
    match results {
        Ok(data) => {
            for answer in data.into_iter() {
                all_answers.push(answer)
            }
            println!("Fetched all answers successfully.");
        }
        Err(e) => println!("Error occurred: {:?}", e)
    }

    return all_answers;
}

pub fn get_an_answer_by_id(connection: &mut PgConnection, answer_id: i32) -> Option<Answer> {
    use crate::schema::answers::dsl::*;

    answers
        .filter(id.eq(answer_id))
        .first::<Answer>(connection)
        .optional()
        .unwrap_or_else(|err| {
            println!("Error occurred: {:?}", err);
            None
        })
}

pub fn add_answer(new_answer: NewAnswer, connection: &mut PgConnection) -> Result<Answer, diesel::result::Error> {
    diesel::insert_into(crate::schema::answers::table)
        .values(&new_answer)
        .get_result::<Answer>(connection)
}
