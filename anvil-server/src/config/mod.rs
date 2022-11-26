pub mod main;

use crate::*;

pub fn load_all() {
    main::load();
    cstm!("📁", "loaded configuration");
}