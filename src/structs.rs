pub mod database_internal;

use crate::traits::{Insertable, Updatable, *};
use anyhow::{anyhow, Result};
use mysql::{Pool, PooledConn};
