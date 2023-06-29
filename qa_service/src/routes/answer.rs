use warp::http::StatusCode;
use crate::{
	store::Store,
	types::answer::NewAnswer,
	types::account::Session,
	profanity::check_profanity
};


pub async fn add_answer(
	session: Session,
    store: Store,
	new_answer: NewAnswer
) -> Result<impl warp::Reply, warp::Rejection> {
	let content = match check_profanity(new_answer.content).await{
		Ok(res) =>res,
		Err(e) => return Err(warp::reject::custom(e)) 
	};

	let answer = NewAnswer{
		content,
		question_id: new_answer.question_id
	};

	match store.add_answer(answer, session.account_id).await{
		Ok(_) => Ok(warp::reply::with_status("Answer added", StatusCode::OK)),
		Err(e) => Err(warp::reject::custom(e))
	}
}
