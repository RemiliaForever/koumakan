use rocket::Catcher;

#[catch(404)]
fn not_found() -> String {
    "Not Found".to_owned()
}

#[catch(401)]
fn unauthorized() -> String {
    "Unauthorized".to_owned()
}

pub fn get_catchers() -> Vec<Catcher> {
    catchers![not_found, unauthorized]
}
