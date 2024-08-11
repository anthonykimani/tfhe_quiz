use crate::models::positions::{NewPosition, Position};
use diesel::prelude::*;

pub fn get_all_positions(connection: &mut PgConnection) -> Vec<Position> {
    use crate::schema::positions::dsl::*;

    let mut all_positions: Vec<Position> = Vec::new();
    let results = positions
        .select(Position::as_select())
        .load(connection);
    match results {
        Ok(data) => {
            for position in data.into_iter() {
                all_positions.push(position)
            }

            println!("todo")
        }
        Err(e) => println!("Error occured {:?}", e)
    }

    return all_positions;
}

pub fn get_a_position_by_id(connection: &mut PgConnection, position_id: i32) -> Option<Position> {
    use crate::schema::positions::dsl::*;

    positions
        .filter(id.eq(position_id))
        .first::<Position>(connection)
        .optional() // This will convert the result to Option
        .unwrap_or_else(|err| {
            println!("Error occurred: {:?}", err);
            None
        })
}


pub fn add_position(new_position: NewPosition, connection: &mut PgConnection) -> Result<Position, diesel::result::Error>{
    diesel::insert_into(crate::schema::positions::table)
        .values(&new_position)
        // .returning(Post::as_returning())
        .get_result::<Position>(connection)
}