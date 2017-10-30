extern crate futures;
extern crate hyper;

use futures::{Future, future};
use hyper::{HttpVersion, server};
use hyper::header::Connection;

enum Route {
    Root,
}

impl Route {
    fn try_parse(uri: hyper::Uri) -> Option<Route> {
        let mut path_parts = uri.path().split('/').skip(1);
        
        match path_parts.next() {
            None => Some(Route::Root),
            _ => None,
        }

    }
}

#[derive(Clone)]
struct Router {
    // store: Arc<store::StoreWrapper>,
    // handler: tokio_core::reactor::Handle,
}

impl Router {
    fn connection_header(http_version: HttpVersion, headers: &hyper::Headers) -> Option<Connection>
    {
        match (http_version, headers.get::<Connection>()) {
            (Http11, None) => Some(Connection::keep_alive()),
            (_, Some(client_connection)) => Some(client_connection.clone()),
            _ => None,
        }
    }

    fn not_found() -> Box<Future<Item = server::Response, Error = hyper::Error>> {
        Box::new(future::ok(server::Response::new().with_status(hyper::StatusCode::NotFound)))
    }
}

impl server::Service for Router {
    type Request = server::Request;
    type Response = server::Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let (method, uri, http_version, headers, body) = req.deconstruct();

        let connection_header = Self::connection_header(http_version, &headers);
        let route = Route::try_parse(uri);

        let result = match (method, route) {
            /*
            (_, _, _, _, Some(_)) => Self::not_found(),
            (hyper::Method::Get, Some(entity), Some(id_src), action, None) =>
                match (entity, id_src.parse(), action) {
                    ("users", Ok(id), None) =>
                        self.clone().get_user(id),
                    ("users", Ok(id), Some("visits")) =>
                        self.clone().get_user_visits(id, uri.query()),
                    ("locations", Ok(id), None) =>
                        self.clone().get_location(id),
                    ("locations", Ok(id), Some("avg")) =>
                        self.clone().get_location_rating(id, uri.query()),
                    ("visits", Ok(id), None) =>
                        self.clone().get_visit(id),
                    _ => Self::not_found(),
                }
            (hyper::Method::Post, Some(entity), Some("new"), None, None) =>
                match entity {
                    "users" => self.clone().add_user(body),
                    "locations" => self.clone().add_location(body),
                    "visits" => self.clone().add_visit(body),
                    _ => Self::not_found(),
                },
            (hyper::Method::Post, Some(entity), Some(id_src), None, None) =>
                match (entity, id_src.parse()) {
                    ("users", Ok(id)) => self.clone().update_user(id, body),
                    ("locations", Ok(id)) => self.clone().update_location(id, body),
                    ("visits", Ok(id)) => self.clone().update_visit(id, body),
                    _ => Self::not_found(),
                }
                */
            _ => Self::not_found(),
        }.map(move |response|
            if let Some(connection_header) = connection_header {
                response.with_header(connection_header)
            } else {
                response
            }
        );

        Box::new(result)
    }
}

fn main() {
    println!("Hello, world!");
}
