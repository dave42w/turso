// Starting a Connection Pool for the Turso Rust Binding
//
// Will be transparent, the Database will hold the pool and depending on configuration
// option will either behave as present or provide connections from the pool.
//
// When connections are dropped (by going out of scope) they will be cleaned ands
// returned to the pool.
//
// The pool will be filled lazily. Upto the pool limit Connections will be created
// only if needed. At present no plans to use a timeout to remove connections from
// the pool if they have not been used

use crate::Connection;
use std::sync::{Arc, Mutex};

const POOL_SIZE: usize = 100;

#[derive(Clone)]
pub(crate) struct ConnectionPool {
    pool: Arc<Mutex<Vec<Connection>>>,
}

impl ConnectionPool {
    pub(crate) fn new() -> Self {
        ConnectionPool {
            pool: Arc::new(Mutex::new(Vec::with_capacity(POOL_SIZE))),
        }
    }

    pub(crate) fn get(&self) -> Option<Connection> {
        let mut pool = self.pool.lock().unwrap();
        pool.pop()
    }

    pub(crate) fn add_to_pool(&self, obj: Connection) {
        let mut pool = self.pool.lock().unwrap();
        pool.push(obj);
    }
}
