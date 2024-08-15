use diesel::prelude::*;
use crate::models::result::{ResultModel, NewResult};

pub fn add_result(new_result: NewResult, connection: &mut PgConnection) -> Result<ResultModel, diesel::result::Error> {
    diesel::insert_into(crate::schema::results::table)
        .values(&new_result)
        .get_result::<ResultModel>(connection)
}

pub fn get_all_results(connection: &mut PgConnection) -> Vec<ResultModel> {
    use crate::schema::results::dsl::*;

    let mut all_results: Vec<ResultModel> = Vec::new();
    let quiz_results = results
        .select(ResultModel::as_select())
        .load(connection);
    match quiz_results {
        Ok(data) => {
            for result in data.into_iter() {
                all_results.push(result)
            }
            println!("Fetched all results successfully.");
        }
        Err(e) => println!("Error occurred: {:?}", e)
    }

    return all_results;
}

pub fn get_a_result_by_id(connection: &mut PgConnection, result_id: i32) -> Option<ResultModel> {
    use crate::schema::results::dsl::*;

    results
        .filter(id.eq(result_id))
        .first::<ResultModel>(connection)
        .optional()
        .unwrap_or_else(|err| {
            println!("Error occurred: {:?}", err);
            None
        })
}

// pub fn update_result_by_id(user_answer_id: i32, updated_result: ResultModel, connection: &mut PgConnection) -> Result<ResultModel, diesel::result::Error> {
//     use crate::schema::results::dsl::*;
//
//     diesel::update(results.find(user_answer_id))
//         .set(&updated_result)
//         .get_result::<ResultModel>(connection)
// }
//
// pub fn delete_result_by_id(result_id: i32, connection: &mut PgConnection) -> Result<usize, diesel::result::Error> {
//     use crate::schema::results::dsl::*;
//
//     diesel::delete(results.find(result_id)).execute(connection)
// }
