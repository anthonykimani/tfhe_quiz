use diesel::prelude::*;
use crate::models::quiz::{Quiz, NewQuiz};

pub fn get_all_quizzes(connection: &mut PgConnection) -> Vec<Quiz> {
    use crate::schema::quizzes::dsl::*;

    let mut all_quizzes: Vec<Quiz> = Vec::new();
    let results = quizzes
        .select(Quiz::as_select())
        .load(connection);
    match results {
        Ok(data) => {
            for quiz in data.into_iter() {
                all_quizzes.push(quiz)
            }
            println!("Fetched all quizzes successfully.");
        }
        Err(e) => println!("Error occurred: {:?}", e)
    }

    return all_quizzes;
}

pub fn get_a_quiz_by_id(connection: &mut PgConnection, quiz_id: i32) -> Option<Quiz> {
    use crate::schema::quizzes::dsl::*;

    quizzes
        .filter(id.eq(quiz_id))
        .first::<Quiz>(connection)
        .optional()
        .unwrap_or_else(|err| {
            println!("Error occurred: {:?}", err);
            None
        })
}

pub fn add_quiz(new_quiz: NewQuiz, connection: &mut PgConnection) -> Result<Quiz, diesel::result::Error> {
    diesel::insert_into(crate::schema::quizzes::table)
        .values(&new_quiz)
        .get_result::<Quiz>(connection)
}
