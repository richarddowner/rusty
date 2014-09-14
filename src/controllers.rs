use serialize::json;
use nickel::{Request, Response};
use models::{ Practice, PracticeForm };

pub fn get_home (_request: &Request, response: &mut Response) {
    response.send("ok");
}

pub fn get_healthcheck (_request: &Request, response: &mut Response) {
    response.send("ok");
}

pub fn post_practice (request: &Request, response: &mut Response) {
    response.set_content_type("application/json");
    
    let form = request.json_as::<PracticeForm>().unwrap();
    
    let name:String;    
    match form.name {
        Some(n) => { name = n; },
        None => {
            response.send(r#"{ "error": "name is required" }"#);
            return
        }
    };

    let mut practice = Practice{
        id: 0,
        name: name,
        display_name: form.display_name,
        logo_document_id: form.logo_document_id,
        avatar_document_id: form.avatar_document_id,
    };

    Practice::insert(&mut practice);
    
    let json = json::encode(&practice);
    response.send(json.as_slice());
}

pub fn get_practices (request: &Request, response: &mut Response) {
    response.set_content_type("application/json");
    let practices = Practice::all();
    let json = json::encode(&practices);
    response.send(json.as_slice());
}