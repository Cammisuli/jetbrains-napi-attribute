#![deny(clippy::all)]

use napi::Either;
use napi_derive::napi;

#[napi(object)]
pub struct InputsInput {
  pub input: String,
  pub dependencies: Option<bool>,
}

#[napi(object)]
pub struct ProjectsInput {
  pub input: String,
  pub projects: Either<String, Vec<String>>,
}

pub(crate) type JsInputs = Either<InputsInput, ProjectsInput>;

#[napi]
pub fn check_object(input: JsInputs) -> () {
  match input {
    Either::A(inputs) => println!("Inputs: {:?}", inputs.dependencies),
    Either::B(projects) => println!("Projects: {:?}", projects.input),
  };
}
