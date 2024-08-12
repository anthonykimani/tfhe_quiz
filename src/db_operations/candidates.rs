use crate::models::candidates::{NewCandidate, Candidate};
use diesel::prelude::*;

pub fn get_all_candidates(connection: &mut PgConnection) -> Vec<Candidate> {
    use crate::schema::candidates::dsl::*;

    let mut all_candidates: Vec<Candidate> = Vec::new();
    let results = candidates
        .select(Candidate::as_select())
        .load(connection);
    match results {
        Ok(data) => {
            for candidate in data.into_iter() {
                all_candidates.push(candidate)
            }

            println!("todo")
        }
        Err(e) => println!("Error occured {:?}", e)
    }

    return all_candidates;
}

pub fn get_a_candidate_by_id(connection: &mut PgConnection, candidate_id: i32) -> Option<Candidate> {
    use crate::schema::candidates::dsl::*;

    candidates
        .filter(id.eq(candidate_id))
        .first::<Candidate>(connection)
        .optional() // This will convert the result to Option
        .unwrap_or_else(|err| {
            println!("Error occurred: {:?}", err);
            None
        })
}


pub fn add_candidate(new_candidate: NewCandidate, connection: &mut PgConnection) -> Result<Candidate, diesel::result::Error>{
    diesel::insert_into(crate::schema::candidates::table)
        .values(&new_candidate)
        // .returning(Post::as_returning())
        .get_result::<Candidate>(connection)
}