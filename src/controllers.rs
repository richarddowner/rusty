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
    let text = format!("{}", form);
    response.send(text.as_slice());
}