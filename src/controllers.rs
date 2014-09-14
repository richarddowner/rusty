use nickel::{Request, Response};
use models::{ Practice, PracticeForm };

pub fn get_home (_request: &Request, response: &mut Response) {
    response.send("ok");
}

pub fn get_healthcheck (_request: &Request, response: &mut Response) {
    response.send("ok");
}

pub fn post_practice (request: &Request, response: &mut Response) {
    let form = request.json_as::<PracticeForm>().unwrap();
    
    let mut practice = Practice{
    	id: 0,
    	name: form.name,
    	display_name: form.display_name,
    	logo_document_id: form.logo_document_id,
    	avatar_document_id: form.avatar_document_id,
    };

    Practice::insert(&mut practice);

    let text = format!("{}", practice);
    response.send(text.as_slice());
}