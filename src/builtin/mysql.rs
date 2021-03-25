use futures::executor;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::ops::Index;
use std::sync::{Arc, Mutex};

use boa::{
    builtins::object::Object,
    exec::Interpreter,
};

use sqlx::{
    mysql::MySqlConnection,
    pool::Pool,
    row::{Row, RowIndex},
};

use lazy_static::lazy_static;

lazy_static! {
    static ref GLOBAL: Arc<Mutex<Option<Pool<MySqlConnection>>>> = Arc::new(Mutex::new(None));
}
