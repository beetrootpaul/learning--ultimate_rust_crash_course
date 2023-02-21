use std::error::Error;

pub type ResultAnyErr<V> = Result<V, Box<dyn Error>>;