extern crate iron;
extern crate hyper;
extern crate router;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate chrono;

#[macro_use]
extern crate serde_derive;

use iron::prelude::*;
use iron::status;
use router::Router;
use rand::{Rng, thread_rng};
use chrono::{UTC, Duration};

use serde::{Serialize, Deserialize};

use std::ops::Add;

fn main() {
    let mut router = Router::new();

    router.get("/api/data", data_response, "data_generator");
    router.post("/api/data", data_redirect, "data_generator_redirect");

    Iron::new(router).http("localhost:7890");
}

fn data_response(_: &mut Request) -> IronResult<Response> {
    let data = generate_data();
    let res_payload = match serde_json::to_string(&data) {
        Ok(p) => p,
        Err(e) => return Err(IronError::new(e, (status::InternalServerError, "There was a problem.\n"))),
    };

    Ok(Response::with((status::Ok, res_payload)))
}

fn data_redirect(req: &mut Request) -> IronResult<Response> {
    let data = generate_data();
    let data_payload = match serde_json::to_string(&data) {
        Ok(p) => p,
        Err(e) => return Err(IronError::new(e, (status::InternalServerError, "There was a problem.\n"))),
    };

    Ok(Response::with((status::Ok, "Unimplemented...\n")))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Datapoint {
    timestamp: i64,
    value: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Dataset {
    tag: String,
    data: Vec<Datapoint>,
}

fn generate_data() -> Dataset {
    let mut rng = rand::thread_rng();

    let n = rng.gen_range::<i64>(40, 60);
    let mut data = Vec::with_capacity(n as usize);
    let now = UTC::now();
    for i in 0..n {
        let datapoint = Datapoint {
            timestamp: now.add(Duration::minutes(i)).timestamp(),
            value: rng.gen_range::<i64>(100, 200),
        };

        data.push(datapoint);
    }

    Dataset {
        tag: String::from("test"),
        data: data
    }
}