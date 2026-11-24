#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, nonstandard_style, dead_code, unused)]

use lotus_script::prelude::*;
use crate::Vars;
use crate::config;
use crate::func;


// --- INIT ---------------------------- //
pub fn init_passenger() {
    log::info!("// Init passengers -------------");
    
}


// --- SIMSTEP ---------------------------- //
pub fn simstep_passenger(v_kmph: f32, v: &mut Vars) {
    let vars = v.vars();

    
}