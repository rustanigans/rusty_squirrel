pub mod database_internal;

use crate::traits::{*};
use anyhow::{Result};
use mysql::{Pool, PooledConn};
