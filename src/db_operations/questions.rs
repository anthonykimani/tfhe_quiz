use diesel::prelude::*;
use crate::models::question::{Question, NewQuestion};
use crate::schema::questions;

pub fn get_all_questions(connection: &mut PgConnection) -> Vec<Question> {
    use crate::schema::questions::dsl::*;

    let mut all_questions: Vec<Question> = Vec::new();
    let results = questions
        .select(Question::as_select())
        .load(connection);
    match results {
        Ok(data) => {
            for question in data.into_iter() {
                all_questions.push(question)
            }
            println!("Fetched all questions successfully.");
        }
        Err(e) => println!("Error occurred: {:?}", e)
    }

    return all_questions;
}

pub fn get_a_question_by_id(connection: &mut PgConnection, question_id: i32) -> Option<Question> {
    use crate::schema::questions::dsl::*;

    questions
        .filter(id.eq(question_id))
        .first::<Question>(connection)
        .optional()
        .unwrap_or_else(|err| {
            println!("Error occurred: {:?}", err);
            None
        })
}

pub fn add_question(new_question: NewQuestion, connection: &mut PgConnection) -> Result<Question, diesel::result::Error> {
    diesel::insert_into(crate::schema::questions::table)
        .values(&new_question)
        .get_result::<Question>(connection)
}
