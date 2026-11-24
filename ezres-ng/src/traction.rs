#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, nonstandard_style, dead_code, unused)]

use std::f32::consts::PI;

use lotus_script::prelude::*;
use crate::Vars;
use crate::config;
use crate::func;

// --- INIT ---------------------------- //
pub fn init_traction(v: &mut Vars) {
    let vars = v.vars();
    log::info!("// Init traction -------------");


}


// --- SIMSTEP ---------------------------- //
pub fn simstep_traction(v_kmph: f32, v: &mut Vars) {
    let vars = v.vars();

    
    
    
    //----------------------------------------------------------------------------------------------------
    //VONTATÁS
    if (vars.ALL_tularamvedo && vars.VanHalozat) 
    {
      let MOTOR_n_abs = (vars.KONST_FORDULAT / 100.0) * 4.59 * 60.0 * (f32::abs(v_kmph/3.6) / (0.96*3.14159));

      //soros
      if (vars.ALL_menetkapcsolo == 0.0)  {vars.MOTOR_ARAM = 0.0;}
      if (vars.ALL_menetkapcsolo == 1.0)  {vars.MOTOR_ARAM = vars.ALL_iranyvalto * f32::min(config::CFG_MAX_ARAM, 278.352136 * f32::exp(-MOTOR_n_abs/384.635834));}
      if (vars.ALL_menetkapcsolo == 2.0)  {vars.MOTOR_ARAM = vars.ALL_iranyvalto * f32::min(config::CFG_MAX_ARAM, 523.408852 * f32::exp(-MOTOR_n_abs/438.300827));}
      if (vars.ALL_menetkapcsolo == 3.0)  {vars.MOTOR_ARAM = vars.ALL_iranyvalto * f32::min(config::CFG_MAX_ARAM, 987.436016 * f32::exp(-MOTOR_n_abs/369.425108));}
      if (vars.ALL_menetkapcsolo == 4.0)  {vars.MOTOR_ARAM = vars.ALL_iranyvalto * f32::min(config::CFG_MAX_ARAM, 2784.955342 * f32::exp(-MOTOR_n_abs/271.456658));}
      if (vars.ALL_menetkapcsolo == 5.0)  {vars.MOTOR_ARAM = vars.ALL_iranyvalto * f32::min(config::CFG_MAX_ARAM, 3397.588377 * f32::exp(-MOTOR_n_abs/286.902986));}
      //párhuzamos
      if (vars.ALL_menetkapcsolo == 6.0)  {vars.MOTOR_ARAM = vars.ALL_iranyvalto * f32::min(config::CFG_MAX_ARAM, 688.549854 * f32::exp(-MOTOR_n_abs/824.836537));}
      if (vars.ALL_menetkapcsolo == 7.0)  {vars.MOTOR_ARAM = vars.ALL_iranyvalto * f32::min(config::CFG_MAX_ARAM, 1019.405278 * f32::exp(-MOTOR_n_abs/748.305776));}
      if (vars.ALL_menetkapcsolo == 8.0)  {vars.MOTOR_ARAM = vars.ALL_iranyvalto * f32::min(config::CFG_MAX_ARAM, 1521.695394 * f32::exp(-MOTOR_n_abs/670.666604));}
      if (vars.ALL_menetkapcsolo == 9.0)  {vars.MOTOR_ARAM = vars.ALL_iranyvalto * f32::min(config::CFG_MAX_ARAM, 2828.184917 * f32::exp(-MOTOR_n_abs/602.971971));}
      //sönt
      if (vars.ALL_menetkapcsolo == 10.0)  {vars.MOTOR_ARAM = vars.ALL_iranyvalto * f32::min(config::CFG_MAX_ARAM, 3328.184917 * f32::exp(-MOTOR_n_abs/602.971971));}
      if (vars.ALL_menetkapcsolo == 11.0)  {vars.MOTOR_ARAM = vars.ALL_iranyvalto * f32::min(config::CFG_MAX_ARAM, 3828.184917 * f32::exp(-MOTOR_n_abs/602.971971));}

      // VONÓERŐ
      vars.MOTOR_VONOERO = vars.KONST_VONOERO * 55.071177 * (f32::exp(0.000960144 * vars.MOTOR_ARAM) - 1.0);

    };
    
    
    if ! vars.ALL_szedo  {vars.MOTOR_VONOERO = 0.0;}
    
  
    
    
    //----------------------------------------------------------------------------------------------------
    //VILLAMOS FÉKEZÉS
    if (vars.ALL_menetkapcsolo >= 0.0) 
    {
      vars.FEKERO_VILLANY = 0.0;
      vars.FEKGERJEDES_T = config::CFG_const_FEKGERJEDES_T;
      vars.FEKERO_VILLANY_GERJEDT = 0.0;
    };
    
    //ELSŐ FÉKFOKOZAT - KÉSLELTETÉSSEL, GERJEDÉSSEL
    if (vars.ALL_menetkapcsolo < 0.0) && (vars.FEKGERJEDES_T > 0.0)  {vars.FEKGERJEDES_T = f32::max(0.0, vars.FEKGERJEDES_T - lotus_script::time::delta());}
    if (vars.ALL_menetkapcsolo < 0.0) && (vars.FEKGERJEDES_T < 0.1) 
    {
      vars.FEKERO_VILLANY_GERJEDT = f32::min(config::CFG_MAX_VFEKERO, vars.FEKERO_VILLANY_GERJEDT + config::CFG_const_FEKGERJEDES_SEB*lotus_script::time::delta());  //folyamatosan növekvő nagyságú fékerő (gerjed fel), felső korlát a max_vfékerő
    };

    if (vars.ALL_menetkapcsolo == -1.0)  {vars.FEKERO_VILLANY = 0.5 * f32::min(config::CFG_MAX_VFEKERO, ( f32::abs(v_kmph -5.5)*config::CFG_MAX_VFEKERO*0.8/42.0 ) -4000.0 );}
    if (vars.ALL_menetkapcsolo == -2.0)  {vars.FEKERO_VILLANY = f32::min(config::CFG_MAX_VFEKERO, ( f32::abs(v_kmph -5.5)*config::CFG_MAX_VFEKERO*0.8/42.0 ) -4000.0 );}
    if (vars.ALL_menetkapcsolo == -3.0)  {vars.FEKERO_VILLANY = f32::min(config::CFG_MAX_VFEKERO, ( f32::abs(v_kmph -4.25)*config::CFG_MAX_VFEKERO/38.0 ) -4000.0 );}
    if (vars.ALL_menetkapcsolo == -4.0)  {vars.FEKERO_VILLANY = f32::min(config::CFG_MAX_VFEKERO, ( f32::abs(v_kmph -3.0)*config::CFG_MAX_VFEKERO/30.0 ) -4000.0 );}
    if (vars.ALL_menetkapcsolo == -5.0)  {vars.FEKERO_VILLANY = f32::min(config::CFG_MAX_VFEKERO, ( f32::abs(v_kmph -2.0)*config::CFG_MAX_VFEKERO/23.0 ) -4000.0 );}
    if (vars.ALL_menetkapcsolo == -6.0)  {vars.FEKERO_VILLANY = f32::min(config::CFG_MAX_VFEKERO, ( f32::abs(v_kmph -1.0)*config::CFG_MAX_VFEKERO/17.0 ) -4000.0 );}
    if (vars.ALL_menetkapcsolo == -7.0)  {vars.FEKERO_VILLANY = f32::min(config::CFG_MAX_VFEKERO, ( f32::abs(v_kmph -1.0)*config::CFG_MAX_VFEKERO/10.0 ) -4000.0 );}

    vars.FEKERO_VILLANY = f32::max(0.0, vars.KONST_VFEK * vars.FEKERO_VILLANY) * (vars.FEKERO_VILLANY_GERJEDT / config::CFG_MAX_VFEKERO); // a normalizált gerjesztéssel megszorozva kapjuk a felgerjedő fékerőt
    
    //BIZTONSÁGI FÉK
    if (vars.ALL_iranyvalto == 0.0) 
    {
      if ((v_kmph > 8.0) || (v_kmph < -8.0)) && (vars.ALL_biztonsagi_fek == false)  {vars.ALL_biztonsagi_fek = true;}
      if (v_kmph < 3.0) && (v_kmph > -3.0) && (vars.ALL_biztonsagi_fek == true)  {vars.ALL_biztonsagi_fek = false;}
    };
    
    if (vars.ALL_iranyvalto != 0.0)  {vars.ALL_biztonsagi_fek = false;}
    
    if (vars.ALL_biztonsagi_fek)  {vars.FEKERO_VILLANY = f32::min(config::CFG_MAX_VFEKERO, ( f32::abs(v_kmph -1.0)*config::CFG_MAX_VFEKERO/12.0 ) -4000.0 );}
    
    
    //----------------------------------------------------------------------------------------------------
    //LÉGFÉK
    vars.FEKERO_LEGFEK = f32::abs(vars.ALL_fekhengerNyomas / 600.0 * config::CFG_MAX_LEGFEK);
    
    //----------------------------------------------------------------------------------------------------
    //KÉZIFÉK
    vars.FEKERO_KEZIFEK = vars.ALL_kezifek / 100.0 * config::CFG_MAX_KEZIFEK;
    

    // --- Erők összesítve --- //
    if let Ok(axle) = lotus_script::vehicle::Axle::get(0, 0) {
        axle.set_traction_force_newton(vars.MOTOR_VONOERO);
        axle.set_brake_force_newton(f32::abs(vars.FEKERO_VILLANY + vars.FEKERO_LEGFEK + vars.FEKERO_KEZIFEK));
        vars.invradius_0_0 = axle.inverse_radius();

        // Váltó hangok
        if (axle.rail_quality() == lotus_script::vehicle::RailQuality::FroggyRough) || (axle.rail_quality() == lotus_script::vehicle::RailQuality::Rough) || (axle.rail_quality() == lotus_script::vehicle::RailQuality::RoughDirt) || (axle.rail_quality() == lotus_script::vehicle::RailQuality::Deraileur)
            { set_var("HANG_kerekcsattogas", true) ;}
        else
            { set_var("HANG_kerekcsattogas", false) ;}

    }

    if let Ok(axle) = lotus_script::vehicle::Axle::get(0, 1) {
        axle.set_traction_force_newton(vars.MOTOR_VONOERO);
        axle.set_brake_force_newton(f32::abs(vars.FEKERO_VILLANY + vars.FEKERO_LEGFEK + vars.FEKERO_KEZIFEK));
        vars.invradius_0_1 = axle.inverse_radius();
    }

    


    // --- Sínfékek --- //
    if let Ok(boogie) = lotus_script::vehicle::Bogie::get(0) {
        boogie.set_rail_brake_force_newton(( (f32::abs(vars.fulke_A_sinfek.poz)+f32::abs(vars.fulke_B_sinfek.poz)) * 20.0 * config::CFG_MAX_SINFEK ) + 0.3*vars.FEKERO_VILLANY);
    }



    // { Váltó hangok }
    // if (railquality_0_0 in [2, 3, 4]) then SndLoop_point_0 := 1 else SndLoop_point_0 := -1;
    // if (railquality_1_0 in [2, 3, 4]) then SndLoop_point_1 := 1 else SndLoop_point_1 := -1;
    // if (railquality_2_0 in [2, 3, 4]) then SndLoop_point_2 := 1 else SndLoop_point_2 := -1;
    // if (railquality_3_0 in [2, 3, 4]) then SndLoop_point_3 := 1 else SndLoop_point_3 := -1;



    //--------------------------------------
    // ÁLTALÁNOS HANGOK
    if (v_kmph != 0.0) {
        set_var("HANG_kerekcsikorgas", true);
        set_var("HANG_futas", true);
        set_var("HANG_futasX", false);
    }
    
    if (v_kmph == 0.0) {
        set_var("HANG_kerekcsikorgas", false);
        set_var("HANG_futas", false);
        set_var("HANG_futasX", false);
    }


    //--------------------------------------
    // MOTOR HANGOK
    if (!(vars.MOTOR_VONOERO == 0.0) || (vars.FEKERO_VILLANY > 0.0)) && (f32::abs(v_kmph) > 2.0) {
        set_var("HANG_motor", true);
    }
    else {
        set_var("HANG_motor", false);
    }
    
    if (f32::abs(vars.FEKERO_VILLANY) > 0.0) {
        set_var("HANG_motorhangero", 0.5 * f32::max(0.25, (3.0*vars.FEKERO_VILLANY / config::CFG_MAX_VFEKERO)));
    }
    else if (f32::abs(vars.MOTOR_VONOERO) > 0.0) {
        set_var("HANG_motorhangero", 0.5 * f32::max(0.25 , (f32::abs(vars.MOTOR_VONOERO)/config::CFG_KONST_MAX_VONTATAS) ));
    }
    else {
        set_var("HANG_motorhangero", 0.0);
    }

    
    //--------------------------------------
    // KERÉK CSATTOGÁS
    // if (railquality_0_0 = 2) or (railquality_0_0 = 3) then
    // begin
    //     set_var("HANG_kerekcsattogas",  1);
    // end
    // else
    //     set_var("HANG_kerekcsattogas", false);
        
    //--------------------------------------
    // EXTRA HANGOK

    // RÁNTÁS
    if (f32::abs(vars.a_ground - vars.a_ground_PREV)) > 0.35 {set_var("HANG_rantas_start", true);}
    if (f32::abs(vars.a_ground - vars.a_ground_PREV)) < -0.8 {set_var("HANG_rantas_stop", true);}

    // ÍVBEN KATTANÁS/dobbanás
    if (vars.fulke_A_aktiv) {set_var("VOL_HANG_rantas_iv",f32::abs(vars.invradius_0_0));}
    else if (vars.fulke_B_aktiv) {set_var("VOL_HANG_rantas_iv",f32::abs(vars.invradius_0_1));}

    if ((f32::abs(vars.invradius_0_0_PREV - vars.invradius_0_0) > 0.001) && vars.fulke_A_aktiv) || ((f32::abs(vars.invradius_0_1_PREV - vars.invradius_0_1) > 0.001) && vars.fulke_B_aktiv) {
        set_var("HANG_rantas_iv", true);
        log::info!("csatt");
    }


    set_var("a_ground", vars.a_ground);
    set_var("a_ground_PREV", vars.a_ground_PREV);
    
    set_var("v_ground", vars.v_ground);
    set_var("v_ground_PREV", vars.v_ground_PREV);


    set_var("invradius_0_0", vars.invradius_0_0);
    set_var("invradius_0_1", vars.invradius_0_1);
    set_var("invradius_0_0_PREV", vars.invradius_0_0_PREV);
    set_var("invradius_0_1_PREV", vars.invradius_0_1_PREV);

}