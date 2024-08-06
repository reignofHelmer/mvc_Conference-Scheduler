use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

use crate::schema::sessions;

#[derive(Queryable, Serialize)]
pub struct Session {
    pub id: Uuid,
    pub title: String,
    pub speaker: String,
    pub time: NaiveDateTime,
    pub duration: i32,
    pub description: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "sessions"]
pub struct NewSession {
    pub title: String,
    pub speaker: String,
    pub time: NaiveDateTime,
    pub duration: i32,
    pub description: String,
}

// Function to create a new session
pub fn create_session(conn: &SqliteConnection, new_session: NewSession) -> QueryResult<Session> {
    use crate::schema::sessions::dsl::*;
    let new_id = Uuid::new_v4();
    diesel::insert_into(sessions)
        .values((id.eq(new_id), &new_session))
        .execute(&mut conn)?;
    sessions.filter(id.eq(new_id)).first(conn)
}

// Function to delete a session
pub fn delete_session(conn: &SqliteConnection, session_id: Uuid) -> QueryResult<usize> {
    use crate::schema::sessions::dsl::*;
    diesel::delete(sessions.filter(id.eq(session_id))).execute(&mut conn)
}

// Function to update a session
pub fn update_session(conn: &SqliteConnection, session_id: Uuid, updated_session: NewSession) -> QueryResult<Session> {
    use crate::schema::sessions::dsl::*;
    diesel::update(sessions.filter(id.eq(session_id)))
        .set((
            title.eq(updated_session.title),
            speaker.eq(updated_session.speaker),
            time.eq(updated_session.time),
            duration.eq(updated_session.duration),
            description.eq(updated_session.description),
        ))
        .execute(&mut conn)?;
    sessions.filter(id.eq(session_id)).first(conn)
}

// Function to get all sessions
pub fn get_sessions(conn: &SqliteConnection) -> QueryResult<Vec<Session>> {
    use crate::schema::sessions::dsl::*;
    sessions.load::<Session>(conn)
}