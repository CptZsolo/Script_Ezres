#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, nonstandard_style, dead_code, unused)]

use std::f32::consts::PI;

use lotus_script::prelude::*;
use crate::Vars;
use crate::config;
use crate::func;

// --- INIT ---------------------------- //
pub fn init_electronics(v: &mut Vars) {
    let vars = v.vars();

    log::info!("// Init electronics -------------");
    set_var("railbrake_pos", 0.0);



    set_var("LM_front_headlight", 0.0);
    set_var("LM_front_dimmed", 0.0);

    let InitReadyForMovement = 2;   // perpillanat nem működik ez a beállítás, szóval felülcsapjuk (hot n run)

    //konstans beállítás
    vars.MAX_VONTATAS = config::CFG_KONST_MAX_VONTATAS; //N-ban
    vars.KONST_VONOERO = 700.0;
    vars.KONST_FORDULAT = 250.0;
    vars.KONST_VFEK = 1.5;
  
  
  
    vars.SZEDO_ALLAS = 0.0;
    vars.SZEDO_SZOG_MAX = 0.0;
    
    // vars.panto_voltage_0 = true;
    vars.VanHalozat = false;
    vars.ALL_szedo_irany = true;
    if(InitReadyForMovement > 1) {vars.ALL_szedo = true;} else {vars.ALL_szedo = false;}
    
    vars.RUGO_UT = 0.0;
    
    vars.FEKGERJEDES_T = config::CFG_const_FEKGERJEDES_T;
    
    vars.AJTOJOBB_CEL = false;
    vars.AJTOBAL_CEL = false;
    vars.utas_ajto_J1 = 0.0;
    vars.utas_ajto_J2 = 0.0;
    vars.utas_ajto_B1 = 0.0;
    vars.utas_ajto_B2 = 0.0;
    
    vars.AJTO_J1_ROGZ = true;
    vars.AJTO_J2_ROGZ = true;
    vars.AJTO_B1_ROGZ = true;
    vars.AJTO_B2_ROGZ = true;
    
    vars.kapaszkodo_X_szog = 0.0;
    vars.kapaszkodo_X_szogseb = 0.0;
    vars.kapaszkodo_Y_szog = 0.0;
    vars.kapaszkodo_Y_szogseb = 0.0;
    
    vars.ALL_biztonsagi_fek = false;
    
    
}


// --- SIMSTEP ---------------------------- //
pub fn simstep_electronics(v_kmph: f32, v: &mut Vars) {
    let vars = v.vars();


    // --- GAME CONFIGS --------------------------- //
    let mut RealisticElecSupply = get_var::<bool>("RealisticElecSupply");
    RealisticElecSupply = false;    // TESZTELÉSHEZ

    // let panto_voltage_0 = get_var::<bool>("panto_voltage_0");
    // let panto_0 = get_var::<f32>("panto_0");
    let mut panto_0 = 0.0;
    let mut panto_voltage_0 = 0.0;

    if let Ok(panto) = lotus_script::vehicle::Pantograph::get(0) {
        panto_0 = lotus_script::vehicle::Pantograph::height(panto);
        panto_voltage_0 = lotus_script::vehicle::Pantograph::voltage(panto);
    }


    // vars.a_ground = lotus_script::vehicle::acceleration_vs_ground(); // Ez valamiért szar...
    vars.v_ground = lotus_script::vehicle::velocity_vs_ground();


    //SZEDŐ SZIMULÁLÁSA - ELŐRE
    if (vars.ALL_szedo && vars.ALL_szedo_irany) 
    {
      vars.SZEDO_ALLAS = ((panto_0)-3.8)/3.15;
      if ((vars.SZEDO_ALLAS > 1.0) || (vars.SZEDO_ALLAS < -1.0))  {vars.SZEDO_ALLAS = 1.0;}
      vars.SZEDO_SZOG_MAX = f32::max(0.0, ((f32::asin(vars.SZEDO_ALLAS))/PI*180.0)-4.0);
      vars.ALL_szedo_szog = f32::min( vars.SZEDO_SZOG_MAX, vars.ALL_szedo_szog+(lotus_script::time::delta()*config::CFG_SZEDO_SEB) );
      
    };
    if (! vars.ALL_szedo && vars.ALL_szedo_irany) 
    {
      vars.ALL_szedo_szog = f32::max(0.0,vars.ALL_szedo_szog-(lotus_script::time::delta()*config::CFG_SZEDO_SEB) );
    };
    
    
    //-----------------------------------------------------------------------
    //SZEDŐ SZIMULÁLÁSA - HÁTRA
    if (vars.ALL_szedo && !vars.ALL_szedo_irany) 
    {
      vars.SZEDO_ALLAS = ((panto_0)-3.8)/3.15;
      if ((vars.SZEDO_ALLAS > 1.0) || (vars.SZEDO_ALLAS < -1.0))  {vars.SZEDO_ALLAS = 1.0;}
      vars.SZEDO_SZOG_MAX = f32::max(0.0, ((f32::asin(vars.SZEDO_ALLAS))/PI*180.0)-4.0);
      vars.ALL_szedo_szog_ABS = f32::min( vars.SZEDO_SZOG_MAX, vars.ALL_szedo_szog_ABS+(lotus_script::time::delta()*config::CFG_SZEDO_SEB) );
      
      vars.ALL_szedo_szog = 180.0-vars.ALL_szedo_szog_ABS-8.0;
    };
    if (! vars.ALL_szedo && ! vars.ALL_szedo_irany) 
    {
      vars.ALL_szedo_szog_ABS = f32::max(0.0,vars.ALL_szedo_szog_ABS-(lotus_script::time::delta()*config::CFG_SZEDO_SEB) );
      
      vars.ALL_szedo_szog = 180.0-vars.ALL_szedo_szog_ABS-8.0;
      
    };
    
    
    //FÜLKE KÖTELEK BEÁLLÍTÁSA
    if (vars.ALL_szedo && vars.ALL_szedo_irany) 
    {
      vars.fulke_A_kotelCsomo = true;
      vars.fulke_B_kotelCsomo = false;
    };
    if (! vars.ALL_szedo && vars.ALL_szedo_irany) 
    {
      vars.fulke_A_kotelCsomo = false;
      vars.fulke_B_kotelCsomo = true;
    };
    if (! vars.ALL_szedo && ! vars.ALL_szedo_irany) 
    {
      vars.fulke_A_kotelCsomo = true;
      vars.fulke_B_kotelCsomo = false;
    };
    if (vars.ALL_szedo && ! vars.ALL_szedo_irany) 
    {
      vars.fulke_A_kotelCsomo = false;
      vars.fulke_B_kotelCsomo = true;
    };
    
    
    
    //VAN-E HÁLÓZAT
    vars.VanHalozat = (vars.ALL_szedo && (panto_voltage_0 != 0.0) && RealisticElecSupply) || (vars.ALL_szedo && ! RealisticElecSupply);


    //----------------------------------------------------------------------------------------------------
    //AJTÓMÜKÖDÉS
    
    //Cél értékek
    vars.AJTOJOBB_CEL = vars.fulke_A_ajtoJobb.ertek || vars.fulke_B_ajtoBal.ertek;
    vars.AJTOBAL_CEL = vars.fulke_A_ajtoBal.ertek || vars.fulke_B_ajtoJobb.ertek;
    
    //AJtók sebessége
    vars.AJTO_J1_SEB = f32::min(config::CFG_AJTO_J1_VMAX, config::CFG_AJTO_J1_VMAX / config::CFG_AJTO_P_TORESPONT * vars.ALL_fektartalyNyomas);
    vars.AJTO_J2_SEB = f32::min(config::CFG_AJTO_J2_VMAX, config::CFG_AJTO_J2_VMAX / config::CFG_AJTO_P_TORESPONT * vars.ALL_fektartalyNyomas);
    vars.AJTO_B1_SEB = f32::min(config::CFG_AJTO_B1_VMAX, config::CFG_AJTO_B1_VMAX / config::CFG_AJTO_P_TORESPONT * vars.ALL_fektartalyNyomas);
    vars.AJTO_B2_SEB = f32::min(config::CFG_AJTO_B2_VMAX, config::CFG_AJTO_B2_VMAX / config::CFG_AJTO_P_TORESPONT * vars.ALL_fektartalyNyomas);
    
    //Ajtók helyzete
    if (vars.AJTOJOBB_CEL == true) 
    {
      vars.utas_ajto_J1 = f32::min(1.0, vars.utas_ajto_J1 + vars.AJTO_J1_SEB*lotus_script::time::delta());
      vars.utas_ajto_J2 = f32::min(1.0, vars.utas_ajto_J2 + vars.AJTO_J2_SEB*lotus_script::time::delta());}
    else if (vars.AJTOJOBB_CEL == false) 
    {
      vars.utas_ajto_J1 = f32::max(0.0, vars.utas_ajto_J1 - vars.AJTO_J1_SEB*lotus_script::time::delta());
      vars.utas_ajto_J2 = f32::max(0.0, vars.utas_ajto_J2 - vars.AJTO_J2_SEB*lotus_script::time::delta());
    };
    //--------------------------------------------------------------
    if (vars.AJTOBAL_CEL == true) 
    {
      vars.utas_ajto_B1 = f32::min(1.0, vars.utas_ajto_B1 + vars.AJTO_B1_SEB*lotus_script::time::delta());
      vars.utas_ajto_B2 = f32::min(1.0, vars.utas_ajto_B2 + vars.AJTO_B2_SEB*lotus_script::time::delta());
    }
    else if (vars.AJTOBAL_CEL == false) 
    {
      vars.utas_ajto_B1 = f32::max(0.0, vars.utas_ajto_B1 - vars.AJTO_B1_SEB*lotus_script::time::delta());
      vars.utas_ajto_B2 = f32::max(0.0, vars.utas_ajto_B2 - vars.AJTO_B2_SEB*lotus_script::time::delta());
    };
    
    
    //---------------------------------------------------------------
    //HANGOK
    
    if ( func::BoolToFloat(vars.AJTOJOBB_CEL) != vars.utas_ajto_J1) 
    {
      set_var("HANG_ajtoJ1_loop", true);
      vars.AJTO_J1_ROGZ = false;
    }
    else {
      set_var("HANG_ajtoJ1_loop", false);
    }
    
    if ( func::BoolToFloat(vars.AJTOJOBB_CEL) != vars.utas_ajto_J2) 
    {
      set_var("HANG_ajtoJ2_loop", true);
      vars.AJTO_J2_ROGZ = false;
    }
    else {
      set_var("HANG_ajtoJ2_loop", false);
    }
    
    if ( func::BoolToFloat(vars.AJTOBAL_CEL) != vars.utas_ajto_B1) 
    {
      set_var("HANG_ajtoB1_loop", true);
      vars.AJTO_B1_ROGZ = false;
    }
    else {
      set_var("HANG_ajtoB1_loop", false);
    }
    
    if ( func::BoolToFloat(vars.AJTOBAL_CEL) != vars.utas_ajto_B2) 
    {
      set_var("HANG_ajtoB2_loop", true);
      vars.AJTO_B2_ROGZ = false;
    }
    else {
      set_var("HANG_ajtoB2_loop", false);
    }
    
    //------------------------------------------------------
    if ( (func::BoolToFloat(vars.AJTOJOBB_CEL) == vars.utas_ajto_J1) && ! vars.AJTO_J1_ROGZ) 
    {
      set_var("HANG_ajtoJ1_stop", true);
      vars.AJTO_J1_ROGZ = true;
    };
    
    if ( (func::BoolToFloat(vars.AJTOJOBB_CEL) == vars.utas_ajto_J2) && ! vars.AJTO_J2_ROGZ) 
    {
      set_var("HANG_ajtoJ2_stop", true);
      vars.AJTO_J2_ROGZ = true;
    };
    
    if ( (func::BoolToFloat(vars.AJTOBAL_CEL) == vars.utas_ajto_B1) && ! vars.AJTO_B1_ROGZ) 
    {
      set_var("HANG_ajtoB1_stop", true);
      vars.AJTO_B1_ROGZ = true;
    };
    
    if ( (func::BoolToFloat(vars.AJTOBAL_CEL) == vars.utas_ajto_B2) && ! vars.AJTO_B2_ROGZ) 
    {
      set_var("HANG_ajtoB2_stop", true);
      vars.AJTO_B2_ROGZ = true;
    };
    
    //---------------------------------------------------------------
    // UTASAJT? PARANCSOK
    // if (utas_ajto_B1 > 0.9)  DoorOpen_0 = true else DoorOpen_0 = false;
    // if (utas_ajto_B2 > 0.9)  DoorOpen_1 = true else DoorOpen_1 = false;
    // if (utas_ajto_J1 > 0.9)  DoorOpen_2 = true else DoorOpen_2 = false;
    // if (utas_ajto_J2 > 0.9)  DoorOpen_3 = true else DoorOpen_3 = false;
    
    
    //----------------------------------------------------------------------------------------------------
    // INDÍTÁSJELZŐ
    if (! vars.VanHalozat)  {vars.utas_berrego = false;}
    
    //T?VVEZ?RL?S A BERREG?RE
    //if (coupled_0 = true) && (csatlas_berrego_front = true)  utas_berrego = true;
    
    
    if (vars.utas_berrego)  {
        set_var("HANG_inditasjelzo", true);
        set_var("LM_utas_berrego", 1.0);
    } else {
        set_var("HANG_inditasjelzo", false);
        set_var("LM_utas_berrego", 0.0);
    }

    
    
    
    
    //----------------------------------------------------------------------------------------------------
    //F?NYEK
    
    // HELYZETJELZ?K
    if vars.VanHalozat && vars.K1.ertek && vars.K2.ertek && (! vars.K3.ertek) && (! vars.fulke_A_sugarveto.ertek)  {
        set_var("LM_A_helyzetjelzo", 1.0);
    } else {
        set_var("LM_A_helyzetjelzo", 0.0);
    }
    if vars.VanHalozat && vars.K1.ertek && vars.K2.ertek && vars.K3.ertek && (! vars.fulke_B_sugarveto.ertek)  {
        set_var("LM_B_helyzetjelzo", 1.0);
    } else {
        set_var("LM_B_helyzetjelzo", 0.0);
    }

    // TOMPÍTOTT FÉNYSZÓRÓ
    if vars.VanHalozat && vars.K1.ertek && vars.K2.ertek && (! vars.K3.ertek) && vars.fulke_A_sugarveto.ertek  {
        set_var("LM_A_tompitott", 1.0);
    } else {
        set_var("LM_A_tompitott", 0.0);
    }
    if vars.VanHalozat && vars.K1.ertek && vars.K2.ertek && vars.K3.ertek && vars.fulke_B_sugarveto.ertek  {
        set_var("LM_B_tompitott", 1.0);
    } else {
        set_var("LM_B_tompitott", 0.0);
    }
    
    // ZÁRFÉNYEK
    if vars.VanHalozat && vars.K1.ertek && vars.K2.ertek && (! vars.K3.ertek)  {
        set_var("LM_B_zarfeny", 1.0);
    } else {
        set_var("LM_B_zarfeny", 0.0);
    }
    if vars.VanHalozat && vars.K1.ertek && vars.K2.ertek && vars.K3.ertek  {
        set_var("LM_A_zarfeny", 1.0);
    } else {
        set_var("LM_A_zarfeny", 0.0);
    }
    
    //UTAST?R
    // if (vars.VanHalozat && utaster.K1.ertek && utaster.K2.ertek) || (csatlas_vilagitasCsatlakozo600V_front && coupled_0) || (csatlas_vilagitasCsatlakozo600V_back && coupled_1)  LM_utaster = 1 else LM_utaster = 0;
    if (vars.VanHalozat && vars.K1.ertek && vars.K2.ertek) {
        set_var("LM_utaster", 0.25);
    } else {
        set_var("LM_utaster", 0.0);
    }
    
    // //PERONVIL?G?T?SOK
    // if ((vars.VanHalozat && vars.K1.ertek && vars.K2.ertek) OR (csatlas_vilagitasCsatlakozo600V_front && coupled_0) || (csatlas_vilagitasCsatlakozo600V_back && coupled_1)) && (! vars.K3.ertek)  LM_utaster_B_peron = 1 else LM_utaster_B_peron = 0;
    // if ((vars.VanHalozat && vars.K1.ertek && vars.K2.ertek) OR (csatlas_vilagitasCsatlakozo600V_front && coupled_0) || (csatlas_vilagitasCsatlakozo600V_back && coupled_1)) && vars.K3.ertek  LM_utaster_A_peron = 1 else LM_utaster_A_peron = 0;
    
    if (vars.VanHalozat && vars.K1.ertek && vars.K2.ertek) && (! vars.K3.ertek)  {
        set_var("LM_utaster_B_peron", 0.25);
    } else {
        set_var("LM_utaster_B_peron", 0.0);
    }
    if (vars.VanHalozat && vars.K1.ertek && vars.K2.ertek) && vars.K3.ertek  {
        set_var("LM_utaster_A_peron", 0.25);
    } else {
        set_var("LM_utaster_A_peron", 0.0);
    }
    
    
    
    
    //----------------------------------------------------------------------------------------------------
    // KAPASZKOD?K MOZG?SA
    
    //FIZIKAI CUCCOK
    vars.PHY_gyorsulas = vars.a_ground;
    vars.PHY_oldalgyorsulas = v_kmph * v_kmph * vars.invradius_0_0;
    
    
    
    if (vars.kapaszkodo_Y_szogseb > 0.0) {
      vars.kapaszkodo_Y_szogseb = vars.kapaszkodo_Y_szogseb + ((-1.0*config::CFG_KAPASZKODO_const1*vars.PHY_gyorsulas + config::CFG_KAPASZKODO_const2*vars.kapaszkodo_Y_szog - config::CFG_KAPASZKODO_const3*vars.kapaszkodo_Y_szogseb*vars.kapaszkodo_Y_szogseb)*lotus_script::time::delta())
    }
    else {
      vars.kapaszkodo_Y_szogseb = vars.kapaszkodo_Y_szogseb + ((-1.0*config::CFG_KAPASZKODO_const1*vars.PHY_gyorsulas + config::CFG_KAPASZKODO_const2*vars.kapaszkodo_Y_szog + config::CFG_KAPASZKODO_const3*vars.kapaszkodo_Y_szogseb*vars.kapaszkodo_Y_szogseb)*lotus_script::time::delta());
    }
    vars.kapaszkodo_Y_szog = vars.kapaszkodo_Y_szog - (vars.kapaszkodo_Y_szogseb * lotus_script::time::delta());
    vars.kapaszkodo_Y_szog = 0.0;
    
    //-------------------------------------------------------------------------
    
    if (vars.kapaszkodo_X_szogseb > 0.0) {
      vars.kapaszkodo_X_szogseb = vars.kapaszkodo_X_szogseb + ((-1.0*config::CFG_KAPASZKODO_const1*vars.PHY_oldalgyorsulas + config::CFG_KAPASZKODO_const2*vars.kapaszkodo_X_szog - config::CFG_KAPASZKODO_const3*vars.kapaszkodo_X_szogseb*vars.kapaszkodo_X_szogseb)*lotus_script::time::delta())
    }
    else {
      vars.kapaszkodo_X_szogseb = vars.kapaszkodo_X_szogseb + ((-1.0*config::CFG_KAPASZKODO_const1*vars.PHY_oldalgyorsulas + config::CFG_KAPASZKODO_const2*vars.kapaszkodo_X_szog + config::CFG_KAPASZKODO_const3*vars.kapaszkodo_X_szogseb*vars.kapaszkodo_X_szogseb)*lotus_script::time::delta());
    }
    vars.kapaszkodo_X_szog = vars.kapaszkodo_X_szog - (vars.kapaszkodo_X_szogseb * lotus_script::time::delta());









    // --- VÁLTOZÓK MÁSOLÁSA A SZIMULÁTORBA ---------------------------------------------- //
        
        
    set_var("ALL_fekhengerNyomas", vars.ALL_fekhengerNyomas);
    set_var("ALL_fektartalyNyomas", vars.ALL_fektartalyNyomas);

    set_var("kasztni_bolintas", 0.0);
    set_var("v_axle_abs", f32::abs(v_kmph / 3.6));
    set_var("v_ground", (v_kmph / 3.6));


    set_var("A_CP_iranyvaltoBent", vars.A_CP_iranyvaltoBent);
    set_var("A_CP_menetkapcsoloBent", vars.A_CP_menetkapcsoloBent);
    set_var("A_CP_fekszelepBent", vars.A_CP_fekszelepBent);

    set_var("A_CP_iranyvalto", vars.A_CP_iranyvalto);
    set_var("A_CP_menetkapcsolo", vars.A_CP_menetkapcsolo);
    set_var("A_CP_fekszelep", vars.A_CP_fekszelep);
    set_var("A_CP_kezifek", vars.A_CP_kezifek);
    set_var("A_CP_sinfek", vars.A_CP_sinfek);
    set_var("A_CP_tularamKapcsolo", vars.A_CP_tularamKapcsolo);
    set_var("A_CP_kotelCsomo", vars.A_CP_kotelCsomo);
    set_var("A_CP_kotelNemCsomo", vars.A_CP_kotelNemCsomo);

    set_var("A_CP_csengo", vars.A_CP_csengo);
    set_var("A_CP_homokolo", vars.A_CP_homokolo);
    set_var("A_CP_ajtoBalKapcsolo", vars.A_CP_ajtoBalKapcsolo);
    set_var("A_CP_ajtoJobbKapcsolo", vars.A_CP_ajtoJobbKapcsolo);
    set_var("A_CP_sugarvetoKapcsolo", vars.A_CP_sugarvetoKapcsolo);
    set_var("A_CP_legsuritoKapcsolo", vars.A_CP_legsuritoKapcsolo);
    set_var("A_CP_berregoBe", vars.A_CP_berregoBe);
    set_var("A_CP_berregoKi", vars.A_CP_berregoKi);

    set_var("B_CP_iranyvaltoBent", vars.B_CP_iranyvaltoBent);
    set_var("B_CP_menetkapcsoloBent", vars.B_CP_menetkapcsoloBent);
    set_var("B_CP_fekszelepBent", vars.B_CP_fekszelepBent);

    set_var("B_CP_iranyvalto", vars.B_CP_iranyvalto);
    set_var("B_CP_menetkapcsolo", vars.B_CP_menetkapcsolo);
    set_var("B_CP_fekszelep", vars.B_CP_fekszelep);
    set_var("B_CP_kezifek", vars.B_CP_kezifek);
    set_var("B_CP_sinfek", vars.B_CP_sinfek);
    set_var("B_CP_tularamKapcsolo", vars.B_CP_tularamKapcsolo);
    set_var("B_CP_kotelCsomo", vars.B_CP_kotelCsomo);
    set_var("B_CP_kotelNemCsomo", vars.B_CP_kotelNemCsomo);

    set_var("B_CP_csengo", vars.B_CP_csengo);
    set_var("B_CP_homokolo", vars.B_CP_homokolo);
    set_var("B_CP_ajtoBalKapcsolo", vars.B_CP_ajtoBalKapcsolo);
    set_var("B_CP_ajtoJobbKapcsolo", vars.B_CP_ajtoJobbKapcsolo);
    set_var("B_CP_sugarvetoKapcsolo", vars.B_CP_sugarvetoKapcsolo);
    set_var("B_CP_berregoBe", vars.B_CP_berregoBe);
    set_var("B_CP_berregoKi", vars.B_CP_berregoKi);

    set_var("utas_berrego", vars.utas_berrego);
    set_var("utas_vilagitasKapcsolo1", vars.utas_vilagitasKapcsolo1);
    set_var("utas_vilagitasKapcsolo2", vars.utas_vilagitasKapcsolo2);
    set_var("utas_vilagitasKapcsolo3", vars.utas_vilagitasKapcsolo3);

    set_var("utas_ablak_B2", vars.utas_ablak_B2);
    set_var("utas_ablak_B3", vars.utas_ablak_B3);
    set_var("utas_ablak_B4", vars.utas_ablak_B4);
    set_var("utas_ablak_J2", vars.utas_ablak_J2);
    set_var("utas_ablak_J3", vars.utas_ablak_J3);
    set_var("utas_ablak_J4", vars.utas_ablak_J4);

    set_var("utas_ajto_B1", vars.utas_ajto_B1);
    set_var("utas_ajto_B2", vars.utas_ajto_B2);
    set_var("utas_ajto_J1", vars.utas_ajto_J1);
    set_var("utas_ajto_J2", vars.utas_ajto_J2);

    set_var("kapaszkodo_X_szog", vars.kapaszkodo_X_szog);
    set_var("kapaszkodo_Y_szog", vars.kapaszkodo_Y_szog);

    set_var("ALL_szedo_szog", vars.ALL_szedo_szog);

    
    set_var("VOL_HANG_A_fekOld", vars.VOL_HANG_A_fekOld);
    set_var("VOL_HANG_A_fekUzem", vars.VOL_HANG_A_fekUzem);
    set_var("VOL_HANG_B_fekOld", vars.VOL_HANG_B_fekOld);
    set_var("VOL_HANG_B_fekUzem", vars.VOL_HANG_B_fekUzem);
    // set_var("VOL_HANG_rantas_iv", vars.VOL_HANG_rantas_iv);


    //--------------------------------------
    // VÁLTOZÓ MÁSOLÁSOK
    vars.a_ground_PREV = vars.a_ground;
    // invradius_abs_max_PREV := invradius_abs_max;
    vars.invradius_0_0_PREV = vars.invradius_0_0;
    vars.invradius_0_1_PREV = vars.invradius_0_1;

    vars.a_ground = (vars.v_ground - vars.v_ground_PREV) / lotus_script::time::delta();
    vars.v_ground_PREV = vars.v_ground;



    
}