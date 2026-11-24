#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, nonstandard_style, dead_code, unused)]

use lotus_extra::bb_system;
use lotus_extra::bb_system::basic;
use lotus_extra::bb_system::cockpit;
use lotus_script::action::ActionEvent;
use lotus_script::prelude::*;
use crate::Vars;
use crate::config;
use lotus_script::vehicle::Axle;


// --- INIT ---------------------------- //
pub fn init_cockpit(v: &mut Vars) {
    let vars = v.vars();

    log::info!("// Init cockpit -------------");

    // Változók inicializálása
    // let InitReadyForMovement = get_var::<i32>("InitReadyForMovement");
    let InitReadyForMovement = 2;   // perpillanat nem működik ez a beállítás, szóval felülcsapjuk (hot n run)


    //FÉK INICIALIZ?L?SA
    vars.ALL_fektartalyNyomas = 500.0;
    vars.ALL_fekhengerNyomas = 0.0;
    vars.legsuritoJar = false;
    
    vars.menetfokozatElozo_A = 0.0;
    vars.menetfokozatElozo_B = 0.0;
    

    //SZÁMLÁLÓK NULLÁZÁSSA (L?SD ONBUTTON)
    vars.fulke_A_szamlalo1 = 0.0;
    vars.fulke_A_jottThrottle = -1.0;
    vars.fulke_B_szamlalo1 = 0.0;
    vars.fulke_B_jottThrottle = -1.0;
    
    
    // 'A' FÜLKE INICIALIZ?L?SA
    vars.fulke_A_ajtoBal.ertek = false;       vars.fulke_A_ajtoBal.poz = 0.0;
    vars.fulke_A_ajtoJobb.ertek = false;      vars.fulke_A_ajtoJobb.poz = 0.0;
    vars.fulke_A_csengo.ertek = false;        vars.fulke_A_csengo.poz = 0.0;
    vars.fulke_A_sugarveto.ertek = false;     vars.fulke_A_sugarveto.poz = 0.0;
    vars.fulke_A_legsurito.ertek = true;        vars.fulke_A_legsurito.poz = 90.0;
    vars.fulke_A_indjelzoBe.ertek = false;    vars.fulke_A_indjelzoBe.poz = 0.0;
    vars.fulke_A_indjelzoKi.ertek = false;    vars.fulke_A_indjelzoKi.poz = 0.0;
    vars.fulke_A_homokolo.ertek = false;       vars.fulke_A_homokolo.poz = 0.0;
    vars.fulke_A_sinfek.ertek = false;        vars.fulke_A_sinfek.poz = 0.0;
    
    vars.fulke_A_menetkapcsolo.ertek = 0.0;      vars.fulke_A_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_0;
    vars.fulke_A_iranyvalto.ertek = 0.0;         vars.fulke_A_iranyvalto.poz = 0.0;
    vars.fulke_A_fekszelep.ertek = 0.0;          vars.fulke_A_fekszelep.poz = 58.0;       vars.fek_A_cel = 0.0;   vars.fek_A_mozog = false;
    vars.fulke_A_kezifek.ertek = 100.0;          vars.fulke_A_kezifek.poz =  1500.0;
    
    
    //HA READY
    if(InitReadyForMovement > 1) {
      vars.fulke_A_iranyvaltoBent = true;
      vars.fulke_A_menetkapcsoloBent = true;
      vars.fulke_A_fekszelepBent = true;
      
      vars.fulke_A_tularamvedo.ertek = true;   vars.fulke_A_tularamvedo.poz = 90.0;
    }
    
    if(InitReadyForMovement == 2) {
      vars.fulke_A_kezifek.ertek = 0.0;          vars.fulke_A_kezifek.poz =  0.0;
    }
    
    //HA NEM READY
    if(InitReadyForMovement == 0) {
      vars.fulke_A_iranyvaltoBent = false;
      vars.fulke_A_menetkapcsoloBent = false;
      vars.fulke_A_fekszelepBent = false;
      
      vars.fulke_A_tularamvedo.ertek = false;   vars.fulke_A_tularamvedo.poz = 0.0;
    }
    
    /////////////////////////////////
    // 'B' F?LKE INICIALIZ?L?SA
    
    vars.fulke_B_ajtoBal.ertek = false;       vars.fulke_B_ajtoBal.poz = 0.0;
    vars.fulke_B_ajtoJobb.ertek = false;      vars.fulke_B_ajtoJobb.poz = 0.0;
    vars.fulke_B_csengo.ertek = false;        vars.fulke_B_csengo.poz = 0.0;
    vars.fulke_B_sugarveto.ertek = false;     vars.fulke_B_sugarveto.poz = 0.0;
    vars.fulke_B_indjelzoBe.ertek = false;    vars.fulke_B_indjelzoBe.poz = 0.0;
    vars.fulke_B_indjelzoKi.ertek = false;    vars.fulke_B_indjelzoKi.poz = 0.0;
    vars.fulke_B_homokolo.ertek = false;      vars.fulke_B_homokolo.poz = 0.0;
    vars.fulke_B_sinfek.ertek = false;        vars.fulke_B_sinfek.poz = 0.0;
    vars.fulke_B_menetkapcsolo.ertek = 0.0;   vars.fulke_B_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_0;
    vars.fulke_B_fekszelep.ertek = 0.0;       vars.fulke_B_fekszelep.poz = 58.0;   vars.fek_B_cel = 0.0;   vars.fek_B_mozog = false;
    vars.fulke_B_kezifek.ertek = 0.0;         vars.fulke_B_kezifek.poz =  0.0;
    
    vars.fulke_B_iranyvaltoBent = false;
    vars.fulke_B_menetkapcsoloBent = false;
    vars.fulke_B_fekszelepBent = false;
    vars.fulke_B_aktiv = false;
    
    vars.fulke_B_tularamvedo.ertek = false;   vars.fulke_B_tularamvedo.poz = 0.0;
    vars.fulke_B_iranyvalto.ertek = 0.0;      vars.fulke_B_iranyvalto.poz = 0.0;
    
    
    
    
    /////////////////////////////////
    // UTAST?R INICIALIZ?L?SA
    vars.K1.ertek = true;     vars.K1.poz = -80.0;
    vars.K2.ertek = false;    vars.K2.poz = 0.0;
    vars.K3.ertek = false;    vars.K3.poz = 0.0;
    
}


// ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- //

// --- SIMSTEP ---------------------------- //
pub fn simstep_cockpit(v_kmph: f32, v: &mut Vars) {
    // --- GET GLOBAL VARS --- //
    let vars = v.vars();

    // KONTROLLER SZÖGEK
    if (vars.fulke_A_menetkapcsolo.ertek == -7.0) { vars.fulke_A_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_f7};
    if (vars.fulke_A_menetkapcsolo.ertek == -6.0) { vars.fulke_A_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_f6};
    if (vars.fulke_A_menetkapcsolo.ertek == -5.0) { vars.fulke_A_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_f5};
    if (vars.fulke_A_menetkapcsolo.ertek == -4.0) { vars.fulke_A_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_f4};
    if (vars.fulke_A_menetkapcsolo.ertek == -3.0) { vars.fulke_A_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_f3};
    if (vars.fulke_A_menetkapcsolo.ertek == -2.0) { vars.fulke_A_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_f2};
    if (vars.fulke_A_menetkapcsolo.ertek == -1.0) { vars.fulke_A_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_f1};
    if (vars.fulke_A_menetkapcsolo.ertek == 0.0) { vars.fulke_A_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_0};
    if (vars.fulke_A_menetkapcsolo.ertek == 1.0) { vars.fulke_A_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_m1};
    if (vars.fulke_A_menetkapcsolo.ertek == 2.0) { vars.fulke_A_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_m2};
    if (vars.fulke_A_menetkapcsolo.ertek == 3.0) { vars.fulke_A_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_m3};
    if (vars.fulke_A_menetkapcsolo.ertek == 4.0) { vars.fulke_A_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_m4};
    if (vars.fulke_A_menetkapcsolo.ertek == 5.0) { vars.fulke_A_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_m5};
    if (vars.fulke_A_menetkapcsolo.ertek == 6.0) { vars.fulke_A_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_m6};
    if (vars.fulke_A_menetkapcsolo.ertek == 7.0) { vars.fulke_A_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_m7};
    if (vars.fulke_A_menetkapcsolo.ertek == 8.0) { vars.fulke_A_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_m8};
    if (vars.fulke_A_menetkapcsolo.ertek == 9.0) { vars.fulke_A_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_m9};
    if (vars.fulke_A_menetkapcsolo.ertek == 10.0) { vars.fulke_A_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_m10};
    if (vars.fulke_A_menetkapcsolo.ertek == 11.0) { vars.fulke_A_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_m11};
    // --- //
    if (vars.fulke_B_menetkapcsolo.ertek == -7.0) { vars.fulke_B_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_f7};
    if (vars.fulke_B_menetkapcsolo.ertek == -6.0) { vars.fulke_B_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_f6};
    if (vars.fulke_B_menetkapcsolo.ertek == -5.0) { vars.fulke_B_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_f5};
    if (vars.fulke_B_menetkapcsolo.ertek == -4.0) { vars.fulke_B_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_f4};
    if (vars.fulke_B_menetkapcsolo.ertek == -3.0) { vars.fulke_B_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_f3};
    if (vars.fulke_B_menetkapcsolo.ertek == -2.0) { vars.fulke_B_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_f2};
    if (vars.fulke_B_menetkapcsolo.ertek == -1.0) { vars.fulke_B_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_f1};
    if (vars.fulke_B_menetkapcsolo.ertek == 0.0) { vars.fulke_B_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_0};
    if (vars.fulke_B_menetkapcsolo.ertek == 1.0) { vars.fulke_B_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_m1};
    if (vars.fulke_B_menetkapcsolo.ertek == 2.0) { vars.fulke_B_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_m2};
    if (vars.fulke_B_menetkapcsolo.ertek == 3.0) { vars.fulke_B_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_m3};
    if (vars.fulke_B_menetkapcsolo.ertek == 4.0) { vars.fulke_B_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_m4};
    if (vars.fulke_B_menetkapcsolo.ertek == 5.0) { vars.fulke_B_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_m5};
    if (vars.fulke_B_menetkapcsolo.ertek == 6.0) { vars.fulke_B_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_m6};
    if (vars.fulke_B_menetkapcsolo.ertek == 7.0) { vars.fulke_B_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_m7};
    if (vars.fulke_B_menetkapcsolo.ertek == 8.0) { vars.fulke_B_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_m8};
    if (vars.fulke_B_menetkapcsolo.ertek == 9.0) { vars.fulke_B_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_m9};
    if (vars.fulke_B_menetkapcsolo.ertek == 10.0) { vars.fulke_B_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_m10};
    if (vars.fulke_B_menetkapcsolo.ertek == 11.0) { vars.fulke_B_menetkapcsolo.poz = config::CFG_MENETKAPCSOLO_m11};



    if (vars.fulke_A_menetkapcsoloBent && vars.fulke_A_iranyvaltoBent && vars.fulke_A_fekszelepBent) && (! vars.fulke_B_aktiv) { vars.fulke_A_aktiv = true} else {vars.fulke_A_aktiv = false;}
    if (vars.fulke_B_menetkapcsoloBent && vars.fulke_B_iranyvaltoBent && vars.fulke_B_fekszelepBent) && (! vars.fulke_A_aktiv) { vars.fulke_B_aktiv = true} else {vars.fulke_B_aktiv = false;}
    
    vars.fulke_A_kotelNemCsomo = ! vars.fulke_A_kotelCsomo;
    vars.fulke_B_kotelNemCsomo = ! vars.fulke_B_kotelCsomo;

    //------------------------------------------------------------------------------------------
    // JOYSTICK KEZELÉS

    
    
    
    //------------------------------------------------------------------------------------------
    //MENETSZABÁLYZÓ KEZELÉSE (A)
    if (vars.fulke_A_jottThrottle == 1.0) {
      if (vars.fulke_A_szamlalo1 > 0.07) {
        if (vars.fulke_A_menetkapcsolo.ertek < 0.0) { set_var("HANG_A_menetkapcsolo_fek", true); }
        else if (vars.fulke_A_menetkapcsolo.ertek < 10.0) { set_var("HANG_A_menetkapcsolo_menet", true); }
        else if (vars.fulke_A_menetkapcsolo.ertek == 10.0) { set_var("HANG_A_menetkapcsolo_menetMax", true); }
        
        vars.fulke_A_menetkapcsolo.ertek = vars.fulke_A_menetkapcsolo.ertek + 1.0;
        if (vars.fulke_A_menetkapcsolo.ertek > 11.0) { vars.fulke_A_menetkapcsolo.ertek = 11.0;}
        
        
        vars.fulke_A_szamlalo1 = 0.0;
      };
      vars.fulke_A_szamlalo1 = vars.fulke_A_szamlalo1 + lotus_script::time::delta();
    }
    
    if (vars.fulke_A_jottThrottle == 0.0)
    {
      vars.fulke_A_szamlalo1 = 0.0;
    }
    
    //------------------------------------------------------------------------------------------
    //MENETSZABÁLYZÓ KEZELÉSE (B)
    if (vars.fulke_B_jottThrottle == 1.0) {
      if (vars.fulke_B_szamlalo1 > 0.07) {
        if (vars.fulke_B_menetkapcsolo.ertek < 0.0) { set_var("HANG_B_menetkapcsolo_fek", true);}
        else if (vars.fulke_B_menetkapcsolo.ertek < 10.0) { set_var("HANG_B_menetkapcsolo_menet", true);}
        else if (vars.fulke_B_menetkapcsolo.ertek == 10.0) { set_var("HANG_B_menetkapcsolo_menetMax", true);}
        
        vars.fulke_B_menetkapcsolo.ertek = vars.fulke_B_menetkapcsolo.ertek + 1.0;
        if (vars.fulke_B_menetkapcsolo.ertek > 11.0) { vars.fulke_B_menetkapcsolo.ertek = 11.0;}
        
        
        vars.fulke_B_szamlalo1 = 0.0;
      };
      vars.fulke_B_szamlalo1 = vars.fulke_B_szamlalo1 + lotus_script::time::delta();
    };
    
    if (vars.fulke_B_jottThrottle == 0.0) {
      vars.fulke_B_szamlalo1 = 0.0;
    };
    
    
    
    //------------------------------------------------------------------------------------------
    // FÉKSZELEP KEZELÉSE (A) //

    // POZÍCIÓ KEZELÉSE (BILLENTYŰZET)
    if (vars.fek_A_cel == -1.0) && (vars.fek_A_mozog) {     // OLDÁS
      vars.fulke_A_fekszelep.ertek = f32::max(-1.0, vars.fulke_A_fekszelep.ertek - (lotus_script::time::delta() * 2.0));
      if (vars.fulke_A_fekszelep.ertek <= -1.0) { vars.fek_A_mozog = false;}
    }

    if (vars.fek_A_cel == 0.0) && (vars.fek_A_mozog) {     // ZÁRT
      if (vars.fulke_A_fekszelep.ertek >= 0.0) {
        vars.fulke_A_fekszelep.ertek = f32::max(0.0, vars.fulke_A_fekszelep.ertek - (lotus_script::time::delta() * 2.0));
        if (vars.fulke_A_fekszelep.ertek <= 0.0) { vars.fek_A_mozog = false;}
      }
      else {
        vars.fulke_A_fekszelep.ertek = f32::min(0.0, vars.fulke_A_fekszelep.ertek + (lotus_script::time::delta() * 2.0));
        if (vars.fulke_A_fekszelep.ertek >= 0.0) { vars.fek_A_mozog = false;}
      }
    }

    if (vars.fek_A_cel == 1.0) && (vars.fek_A_mozog) {     // ÜZEMI FÉK
      if (vars.fulke_A_fekszelep.ertek >= 1.0) {
        vars.fulke_A_fekszelep.ertek = f32::max(1.0, vars.fulke_A_fekszelep.ertek - (lotus_script::time::delta() * 2.0));
        if (vars.fulke_A_fekszelep.ertek <= 1.0) { vars.fek_A_mozog = false;}
      }
      else {
        vars.fulke_A_fekszelep.ertek = f32::min(1.0, vars.fulke_A_fekszelep.ertek + (lotus_script::time::delta() * 2.0));
        if (vars.fulke_A_fekszelep.ertek >= 1.0) { vars.fek_A_mozog = false;}
      };
    };
    
    if (vars.fek_A_cel == 2.0) && (vars.fek_A_mozog) {     // VÉSZFÉK
      vars.fulke_A_fekszelep.ertek = f32::min(2.0, vars.fulke_A_fekszelep.ertek + (lotus_script::time::delta() * 4.0));
      if (vars.fulke_A_fekszelep.ertek >= 2.0) { vars.fek_A_mozog = false;}
    };

    // SZIMULÁCIÓ KEZELÉSE
    if (vars.fulke_A_fekszelep.ertek < 0.0) {     // OLDÁS
      if (vars.ALL_fekhengerNyomas > 0.0) { vars.ALL_fekhengerNyomas = vars.ALL_fekhengerNyomas - (lotus_script::time::delta() * config::CFG_FEKKI_SEB * f32::abs(vars.fulke_A_fekszelep.ertek));}
      
      set_var("HANG_A_fekOld", true);
      set_var("HANG_A_fekUzem", false);
      vars.VOL_HANG_A_fekOld = f32::abs(vars.fulke_A_fekszelep.ertek);
      if (vars.ALL_fekhengerNyomas == 100.0) { set_var("HANG_fekrudazat_ki", true);}
    };
    
    if (vars.fulke_A_fekszelep.ertek == 0.0) {     // ZÁRT
      vars.VOL_HANG_A_fekUzem = 0.0;
      vars.VOL_HANG_A_fekOld = 0.0;
      
      set_var("HANG_A_fekUzem", true);
      set_var("HANG_A_fekOld", false);
    };
    
    if (vars.fulke_A_fekszelep.ertek > 0.0) {     // ÜZEM vagy VÉSZFÉK
      if (vars.ALL_fekhengerNyomas < vars.ALL_fektartalyNyomas) {
        vars.ALL_fekhengerNyomas = vars.ALL_fekhengerNyomas + (lotus_script::time::delta() * config::CFG_FEKBE_MAX * f32::abs(vars.fulke_A_fekszelep.ertek));
        vars.ALL_fektartalyNyomas = vars.ALL_fektartalyNyomas - (lotus_script::time::delta()  *config::CFG_FEKBE_MAX/12.0 * f32::abs(vars.fulke_A_fekszelep.ertek));
      };
      
      set_var("HANG_A_fekUzem", true);
      set_var("HANG_A_fekOld", false);
      vars.VOL_HANG_A_fekUzem = f32::abs(vars.fulke_A_fekszelep.ertek);
      if (vars.ALL_fekhengerNyomas == 100.0) { set_var("HANG_fekrudazat_be", true);}
    };

    // ANIMÁCIÓ
    if (vars.fulke_A_fekszelep.ertek < 0.0) { vars.fulke_A_fekszelep.poz = f32::max(0.0, 58.0 * (vars.fulke_A_fekszelep.ertek + 1.0)) }
    else if (vars.fulke_A_fekszelep.ertek < 1.0) { vars.fulke_A_fekszelep.poz = 58.0 + (85.0-58.0) * vars.fulke_A_fekszelep.ertek; }
    else {vars.fulke_A_fekszelep.poz = f32::min(110.0, 85.0 + (110.0-85.0) * (vars.fulke_A_fekszelep.ertek - 1.0));}

    
    
    
    //------------------------------------------------------------------------------------------
    //FÉKSZELEP KEZELÉSE (B)

    // POZÍCIÓ KEZELÉSE (BILLENTYŰZET)
    if (vars.fek_B_cel == -1.0) && (vars.fek_B_mozog) {     // OLDÁS
      vars.fulke_B_fekszelep.ertek = f32::max(-1.0, vars.fulke_B_fekszelep.ertek - (lotus_script::time::delta() * 2.0));
      if (vars.fulke_B_fekszelep.ertek <= -1.0) { vars.fek_B_mozog = false;}
    };

    if (vars.fek_B_cel == 0.0) && (vars.fek_B_mozog) {     // ZÁRT
      if (vars.fulke_B_fekszelep.ertek >= 0.0) {
        vars.fulke_B_fekszelep.ertek = f32::max(0.0, vars.fulke_B_fekszelep.ertek - (lotus_script::time::delta() * 2.0));
        if (vars.fulke_B_fekszelep.ertek <= 0.0) { vars.fek_B_mozog = false;}
      }
      else {
        vars.fulke_B_fekszelep.ertek = f32::min(0.0, vars.fulke_B_fekszelep.ertek + (lotus_script::time::delta() * 2.0));
        if (vars.fulke_B_fekszelep.ertek >= 0.0) { vars.fek_B_mozog = false;}
      }
    }

    if (vars.fek_B_cel == 1.0) && (vars.fek_B_mozog) {     // ÜZEMI FÉK
      if (vars.fulke_B_fekszelep.ertek >= 1.0) {
        vars.fulke_B_fekszelep.ertek = f32::max(1.0, vars.fulke_B_fekszelep.ertek - (lotus_script::time::delta() * 2.0));
        if (vars.fulke_B_fekszelep.ertek <= 1.0) { vars.fek_B_mozog = false;}
      }
      else {
        vars.fulke_B_fekszelep.ertek = f32::min(1.0, vars.fulke_B_fekszelep.ertek + (lotus_script::time::delta() * 2.0));
        if (vars.fulke_B_fekszelep.ertek >= 1.0) { vars.fek_B_mozog = false;}
      }
    }
    
    if (vars.fek_B_cel == 2.0) && (vars.fek_B_mozog) {     // VÉSZFÉK
      vars.fulke_B_fekszelep.ertek = f32::min(2.0, vars.fulke_B_fekszelep.ertek + (lotus_script::time::delta() * 4.0));
      if (vars.fulke_B_fekszelep.ertek >= 2.0) { vars.fek_B_mozog = false;}
    }

    // SZIMULÁCIÓ KEZELÉSE
    if (vars.fulke_B_fekszelep.ertek < 0.0) {     // OLDÁS
      if (vars.ALL_fekhengerNyomas > 0.0) { vars.ALL_fekhengerNyomas = vars.ALL_fekhengerNyomas - (lotus_script::time::delta() * config::CFG_FEKKI_SEB * f32::abs(vars.fulke_B_fekszelep.ertek));}
      
      set_var("HANG_B_fekOld", true);
      set_var("HANG_B_fekUzem", false);
      vars.VOL_HANG_B_fekOld = f32::abs(vars.fulke_B_fekszelep.ertek);
      if (vars.ALL_fekhengerNyomas == 100.0) { set_var("HANG_fekrudazat_ki", true);}
    }
    
    if (vars.fulke_B_fekszelep.ertek == 0.0) {     // ZÁRT
      vars.VOL_HANG_B_fekUzem = 0.0;
      vars.VOL_HANG_B_fekOld = 0.0;
      
      set_var("HANG_B_fekUzem", false);
      set_var("HANG_B_fekOld", false);
    }
    
    if (vars.fulke_B_fekszelep.ertek > 0.0) {     // ÜZEM vagy VÉSZFÉK
      if (vars.ALL_fekhengerNyomas < vars.ALL_fektartalyNyomas) {
        vars.ALL_fekhengerNyomas = vars.ALL_fekhengerNyomas + (lotus_script::time::delta() * config::CFG_FEKBE_MAX * f32::abs(vars.fulke_B_fekszelep.ertek));
        vars.ALL_fektartalyNyomas = vars.ALL_fektartalyNyomas - (lotus_script::time::delta()  *config::CFG_FEKBE_MAX/12.0 * f32::abs(vars.fulke_B_fekszelep.ertek));
      }
      
      set_var("HANG_B_fekUzem", true);
      set_var("HANG_B_fekOld", false);
      vars.VOL_HANG_B_fekUzem = f32::abs(vars.fulke_B_fekszelep.ertek);
      if (vars.ALL_fekhengerNyomas == 100.0) { set_var("HANG_fekrudazat_be", true);}
    }

    // ANIMÁCIÓ
    if (vars.fulke_B_fekszelep.ertek < 0.0) { vars.fulke_B_fekszelep.poz = f32::max(0.0, 58.0 * (vars.fulke_B_fekszelep.ertek + 1.0))}
    else if (vars.fulke_B_fekszelep.ertek < 1.0) { vars.fulke_B_fekszelep.poz = 58.0 + (85.0-58.0) * vars.fulke_B_fekszelep.ertek;}
    else {vars.fulke_B_fekszelep.poz = f32::min(110.0, 85.0 + (110.0-85.0) * (vars.fulke_B_fekszelep.ertek - 1.0));}


    
    
    //------------------------------------------------------------------------------------------
    //KÉZIFÉK LEKÉPEZÉSE (A)
    if (vars.fulke_A_kezifekCel == 1.0) {
      vars.fulke_A_kezifek.ertek = vars.fulke_A_kezifek.ertek + (lotus_script::time::delta()*20.0); //ennyi % m?sodpercenk?nt
      if (vars.fulke_A_kezifek.ertek > 100.0) { vars.fulke_A_kezifek.ertek = 100.0;}
    }
    if (vars.fulke_A_kezifekCel == -1.0) {
      vars.fulke_A_kezifek.ertek = vars.fulke_A_kezifek.ertek - (lotus_script::time::delta()*25.0);
      if (vars.fulke_A_kezifek.ertek < 0.0) { vars.fulke_A_kezifek.ertek = 0.0;}
    }
    vars.fulke_A_kezifek.poz = (vars.fulke_A_kezifek.ertek * 15.0);
    
    //------------------------------------------------------------------------------------------
    //K?ZIF?K LEK?PEZ?SE (B)
    if (vars.fulke_B_kezifekCel == 1.0) {
      vars.fulke_B_kezifek.ertek = vars.fulke_B_kezifek.ertek + (lotus_script::time::delta()*20.0); //ennyi % m?sodpercenk?nt
      if (vars.fulke_B_kezifek.ertek > 100.0) { vars.fulke_B_kezifek.ertek = 100.0;}
    }
    if (vars.fulke_B_kezifekCel == -1.0) {
      vars.fulke_B_kezifek.ertek = vars.fulke_B_kezifek.ertek - (lotus_script::time::delta()*25.0);
      if (vars.fulke_B_kezifek.ertek < 0.0) { vars.fulke_B_kezifek.ertek = 0.0;}
    }
    vars.fulke_B_kezifek.poz = (vars.fulke_B_kezifek.ertek * 15.0);
    
    
    
    
    //------------------------------------------------------------------------------------------
    //L?GS?R?T? SZIMUL?L?SA
    // automatikusan kikapcsol 6 Bar felett, bekapcsol 4,5 Bar alatt (ha kapcsol? & vanHalozat)
    if ( (vars.ALL_fektartalyNyomas < 450.0) && (vars.VanHalozat) && (vars.fulke_A_legsurito.ertek) && (! vars.legsuritoJar) ) {
      vars.legsuritoJar = true;
      set_var("HANG_legsuritoStart", true);
      set_var("HANG_legsuritoLoop", true);
      set_var("HANG_legsuritoLoopX", true);
    }
    
    if ( ( (vars.ALL_fektartalyNyomas > 600.0) || (! vars.VanHalozat) || (! vars.fulke_A_legsurito.ertek)) && (vars.legsuritoJar) ) {
      vars.legsuritoJar = false;
      set_var("HANG_legsuritoStop", true);
      set_var("HANG_legsuritoLoop", false);
      set_var("HANG_legsuritoLoopX", false);
    }
    
    if (vars.legsuritoJar) { vars.ALL_fektartalyNyomas = vars.ALL_fektartalyNyomas + (lotus_script::time::delta()*7.0);}
    
    
    
    
    
    //----------------------------------------------------------------------------------------------
    //ABLAKOK LEKEZELÉSE
    
    //BAL 2
    if (vars.ablakB2_cel == -1) { vars.utas_ablak_B2 = f32::min( 1.0, vars.utas_ablak_B2 + lotus_script::time::delta()*config::CFG_ABLAK_FEL_SEB);}
    if (vars.ablakB2_cel == 1) { vars.utas_ablak_B2 = f32::max( 0.0, vars.utas_ablak_B2 - lotus_script::time::delta()*config::CFG_ABLAK_LE_SEB);}
    //BAL 3
    if (vars.ablakB3_cel == -1) { vars.utas_ablak_B3 = f32::min( 1.0, vars.utas_ablak_B3 + lotus_script::time::delta()*config::CFG_ABLAK_FEL_SEB);}
    if (vars.ablakB3_cel == 1) { vars.utas_ablak_B3 = f32::max( 0.0, vars.utas_ablak_B3 - lotus_script::time::delta()*config::CFG_ABLAK_LE_SEB);}
    //BAL 4
    if (vars.ablakB4_cel == -1) {vars. utas_ablak_B4 = f32::min( 1.0, vars.utas_ablak_B4 + lotus_script::time::delta()*config::CFG_ABLAK_FEL_SEB);}
    if (vars.ablakB4_cel == 1) { vars.utas_ablak_B4 = f32::max( 0.0, vars.utas_ablak_B4 - lotus_script::time::delta()*config::CFG_ABLAK_LE_SEB);}
    
    //-------------------------------------------------------------------------------------------------------
    //JOBB 2
    if (vars.ablakJ2_cel == -1) { vars.utas_ablak_J2 = f32::min( 1.0, vars.utas_ablak_J2 + lotus_script::time::delta()*config::CFG_ABLAK_FEL_SEB);}
    if (vars.ablakJ2_cel == 1) { vars.utas_ablak_J2 = f32::max( 0.0, vars.utas_ablak_J2 - lotus_script::time::delta()*config::CFG_ABLAK_LE_SEB);}
    //JOBB 3
    if (vars.ablakJ3_cel == -1) { vars.utas_ablak_J3 = f32::min( 1.0, vars.utas_ablak_J3 + lotus_script::time::delta()*config::CFG_ABLAK_FEL_SEB);}
    if (vars.ablakJ3_cel == 1) { vars.utas_ablak_J3 = f32::max( 0.0, vars.utas_ablak_J3 - lotus_script::time::delta()*config::CFG_ABLAK_LE_SEB);}
    //JOBB 4
    if (vars.ablakJ4_cel == -1) { vars.utas_ablak_J4 = f32::min( 1.0, vars.utas_ablak_J4 + lotus_script::time::delta()*config::CFG_ABLAK_FEL_SEB);}
    if (vars.ablakJ4_cel == 1) { vars.utas_ablak_J4 = f32::max( 0.0, vars.utas_ablak_J4 - lotus_script::time::delta()*config::CFG_ABLAK_LE_SEB);}
    
    
    
    //------------------------------------------------------------------------------------------
    // "JOYSTICK" KEZELÉS //

    // NOTES: Perpillanat így beragad a változó értéke, azaz folyton felülcsapja ez a billentyűzet parancsot - erre ki kell valamit találni majd
    //FÉKSZELEP (A)
    // let JOY_Airbrake = get_var::<f32>("JOY_Airbrake");

    // if (vars.fulke_A_aktiv) && (vars.fulke_A_fekszelepBent) {
    //   if (JOY_Airbrake < 0.45) {vars.fulke_A_fekszelep.ertek = -1.0 + ((1.0 / 0.45) * JOY_Airbrake);}
    //   else if (JOY_Airbrake > 0.55) {vars.fulke_A_fekszelep.ertek = (2.0 / 0.45) * (JOY_Airbrake - 0.55);}
    //   else {vars.fulke_A_fekszelep.ertek = 0.0;}
    // }
    
    
    
    //------------------------------------------------------------------------------------------
    // VÁLTOZÓ MÁSOLÁSOK //
    vars.ALL_tularamvedo =  vars.fulke_A_tularamvedo.ertek || vars.fulke_B_tularamvedo.ertek;
    vars.ALL_sinfek =  vars.fulke_A_sinfek.ertek | vars.fulke_B_sinfek.ertek;
    vars.ALL_kezifek =  f32::max(vars.fulke_A_kezifek.ertek, vars.fulke_B_kezifek.ertek);
    
    
    if (vars.fulke_A_aktiv) { vars.ALL_menetkapcsolo = vars.fulke_A_menetkapcsolo.ertek;}
    else if (vars.fulke_B_aktiv) { vars.ALL_menetkapcsolo = vars.fulke_B_menetkapcsolo.ertek;}
    else {vars.ALL_menetkapcsolo = 0.0;}
    
    if (vars.fulke_A_aktiv) { vars.ALL_iranyvalto = vars.fulke_A_iranyvalto.ertek;}
    else if (vars.fulke_B_aktiv) { vars.ALL_iranyvalto = - vars.fulke_B_iranyvalto.ertek;}
    else {vars.ALL_iranyvalto = 0.0;}
    
    if (vars.utas_berrego) { 
        set_var("LM_utas_berrego", 1.0);
    } else {
        set_var("LM_utas_berrego", 0.0);
    }


    //'A' F?LKE SPECIFIKUS DOLGOK
    vars.A_CP_iranyvaltoBent = vars.fulke_A_iranyvaltoBent;
    vars.A_CP_menetkapcsoloBent = vars.fulke_A_menetkapcsoloBent;
    vars.A_CP_fekszelepBent = vars.fulke_A_fekszelepBent;
    
    vars.A_CP_iranyvalto = vars.fulke_A_iranyvalto.poz;
    vars.A_CP_menetkapcsolo = vars.fulke_A_menetkapcsolo.poz;
    vars.A_CP_fekszelep = vars.fulke_A_fekszelep.poz;
    vars.A_CP_kezifek = vars.fulke_A_kezifek.poz;
    vars.A_CP_sinfek = vars.fulke_A_sinfek.poz;
    vars.A_CP_tularamKapcsolo = vars.fulke_A_tularamvedo.poz;
    vars.A_CP_kotelCsomo = vars.fulke_A_kotelCsomo;
    vars.A_CP_kotelNemCsomo = vars.fulke_A_kotelNemCsomo;
    
    vars.A_CP_csengo = vars.fulke_A_csengo.poz;
    vars.A_CP_homokolo = vars.fulke_A_homokolo.poz;
    vars.A_CP_ajtoBalKapcsolo = vars.fulke_A_ajtoBal.poz;
    vars.A_CP_ajtoJobbKapcsolo = vars.fulke_A_ajtoJobb.poz;
    vars.A_CP_sugarvetoKapcsolo = vars.fulke_A_sugarveto.poz;
    vars.A_CP_legsuritoKapcsolo = vars.fulke_A_legsurito.poz;
    vars.A_CP_berregoBe = vars.fulke_A_indjelzoBe.poz;
    vars.A_CP_berregoKi = vars.fulke_A_indjelzoKi.poz;
    
    vars.A_CP_szamlalo1 = vars.fulke_A_szamlalo1;
    vars.A_CP_jottThrottle = vars.fulke_A_jottThrottle;
    
    
    
    //'B' F?LKE SPECIFIKUS DOLGOK
    vars.B_CP_iranyvaltoBent = vars.fulke_B_iranyvaltoBent;
    vars.B_CP_menetkapcsoloBent = vars.fulke_B_menetkapcsoloBent;
    vars.B_CP_fekszelepBent = vars.fulke_B_fekszelepBent;
    
    vars.B_CP_iranyvalto = vars.fulke_B_iranyvalto.poz;
    vars.B_CP_menetkapcsolo = vars.fulke_B_menetkapcsolo.poz;
    vars.B_CP_fekszelep = vars.fulke_B_fekszelep.poz;
    vars.B_CP_kezifek = vars.fulke_B_kezifek.poz;
    vars.B_CP_sinfek = vars.fulke_B_sinfek.poz;
    vars.B_CP_tularamKapcsolo = vars.fulke_B_tularamvedo.poz;
    vars.B_CP_kotelCsomo = vars.fulke_B_kotelCsomo;
    vars.B_CP_kotelNemCsomo = vars.fulke_B_kotelNemCsomo;
    
    vars.B_CP_csengo = vars.fulke_B_csengo.poz;
    vars.B_CP_homokolo = vars.fulke_B_homokolo.poz;
    vars.B_CP_ajtoBalKapcsolo = vars.fulke_B_ajtoBal.poz;
    vars.B_CP_ajtoJobbKapcsolo = vars.fulke_B_ajtoJobb.poz;
    vars.B_CP_sugarvetoKapcsolo = vars.fulke_B_sugarveto.poz;
    vars.B_CP_berregoBe = vars.fulke_B_indjelzoBe.poz;
    vars.B_CP_berregoKi = vars.fulke_B_indjelzoKi.poz;
    
    //vars.B_CP_szamlalo1 = vars.fulke_B_szamlalo1;
    //vars.B_CP_jottThrottle = vars.fulke_B_jottThrottle;
    
    
    // UTASTÉR VÁLTOZÓI
    vars.utas_vilagitasKapcsolo1 = vars.K1.poz;
    vars.utas_vilagitasKapcsolo2 = vars.K2.poz;
    vars.utas_vilagitasKapcsolo3 = vars.K3.poz;




}

// ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- //

// --- HANDLE INPUTS --- //
pub fn on_message_cockpit(v: &mut Vars, a: ActionEvent) {
    // --- GET GLOBAL VARS --- //
    let vars = v.vars();

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // "A" F?LKE CUCCAI
    
    //----------------------------------------------------------------------------------------------
    // BEHELYEZ?SEK
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_A_CP_iranyvaltoBe") && (! vars.fulke_B_aktiv) && (! vars.fulke_A_aktiv) 
    {
      vars.fulke_A_iranyvaltoBent = true;
      set_var("HANG_A_iranyvaltoBe", true);
    }
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_A_CP_iranyvaltoKi") && (vars.fulke_A_iranyvaltoBent) && (vars.fulke_A_iranyvalto.ertek == 0.0) 
    {
      vars.fulke_A_iranyvaltoBent = false;
      set_var("HANG_A_iranyvaltoKi", true);
    }
    
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_A_CP_menetkapcsoloBe") && (! vars.fulke_B_aktiv) && (! vars.fulke_A_aktiv)  
    {
      vars.fulke_A_menetkapcsoloBent = true;
      set_var("HANG_A_menetkapcsoloBe", true);
    }
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_A_CP_menetkapcsoloKi") && (vars.fulke_A_menetkapcsoloBent) && (vars.fulke_A_iranyvalto.ertek == 0.0)  
    {
      vars.fulke_A_menetkapcsoloBent = false;
      set_var("HANG_A_menetkapcsoloKi", true);
    }
    
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_A_CP_fekszelepBe") && (! vars.fulke_B_aktiv) && (! vars.fulke_A_aktiv) 
    {
      vars.fulke_A_fekszelepBent = true;
      set_var("HANG_A_fekszelepBe", true);
    }
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_A_CP_fekszelepKi") && (vars.fulke_A_fekszelepBent) && (vars.fulke_A_iranyvalto.ertek == 0.0) && (vars.fulke_A_fekszelep.ertek == 0.0) 
    {
      vars.fulke_A_fekszelepBent = false;
      set_var("HANG_A_fekszelepKi", true);
    }
    
    //----------------------------------------------------------------------------------------------
    //IR?NYV?LT?
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_A_CP_iranyvaltoElore") && (vars.fulke_A_aktiv) && (vars.fulke_A_menetkapcsolo.ertek == 0.0) 
    {
      if (vars.fulke_A_iranyvalto.ertek < 1.0)  {set_var("HANG_A_iranyvaltoElore", true);}
      vars.fulke_A_iranyvalto.ertek = vars.fulke_A_iranyvalto.ertek + 1.0;
      if (vars.fulke_A_iranyvalto.ertek > 1.0)  {vars.fulke_A_iranyvalto.ertek = 1.0;}
    }
    
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_A_CP_iranyvaltoHatra") && (vars.fulke_A_aktiv) && (vars.fulke_A_menetkapcsolo.ertek == 0.0) 
    {
      if (vars.fulke_A_iranyvalto.ertek > -1.0)  {set_var("HANG_A_iranyvaltoHatra", true);}
      vars.fulke_A_iranyvalto.ertek = vars.fulke_A_iranyvalto.ertek - 1.0;
      if (vars.fulke_A_iranyvalto.ertek < -1.0)  {vars.fulke_A_iranyvalto.ertek = -1.0;}
    }
    
    if a.state.kind.is_just_pressed() && (a.name == "ReverserPlus") && (vars.fulke_A_aktiv) && (a.state.cockpit_index == Some(0)) && (vars.fulke_A_menetkapcsolo.ertek == 0.0) 
    {
      if (vars.fulke_A_iranyvalto.ertek < 1.0)  {set_var("HANG_A_iranyvaltoElore", true);}
      vars.fulke_A_iranyvalto.ertek = vars.fulke_A_iranyvalto.ertek + 1.0;
      if (vars.fulke_A_iranyvalto.ertek > 1.0)  {vars.fulke_A_iranyvalto.ertek = 1.0;}
    }
    
    if a.state.kind.is_just_pressed() && (a.name == "ReverserMinus") && (vars.fulke_A_aktiv) && (a.state.cockpit_index == Some(0)) && (vars.fulke_A_menetkapcsolo.ertek == 0.0) 
    {
      if (vars.fulke_A_iranyvalto.ertek > -1.0)  {set_var("HANG_A_iranyvaltoHatra", true);}
      vars.fulke_A_iranyvalto.ertek = vars.fulke_A_iranyvalto.ertek - 1.0;
      if (vars.fulke_A_iranyvalto.ertek < -1.0)  {vars.fulke_A_iranyvalto.ertek = -1.0;}
    }
    
    if (vars.fulke_A_iranyvalto.ertek == 1.0)  {vars.fulke_A_iranyvalto.poz = config::CFG_IRANYVALTO_ELORE;}
    if (vars.fulke_A_iranyvalto.ertek == 0.0)  {vars.fulke_A_iranyvalto.poz = 0.0;}
    if (vars.fulke_A_iranyvalto.ertek == -1.0)  {vars.fulke_A_iranyvalto.poz = config::CFG_IRANYVALTO_HATRA;}
    
    
    //----------------------------------------------------------------------------------------------
    //MENETKAPCSOLÓ
    
    if a.state.kind.is_just_pressed() && (a.name == "Throttle") && (vars.fulke_A_aktiv) && (vars.fulke_A_iranyvalto.ertek != 0.0) 
    {
      //kontrollerhez a v?ltoz? billent?se
      vars.fulke_A_jottThrottle = 1.0;
    }
    
    if a.state.kind.is_just_released() && (a.name == "Throttle") && (vars.fulke_A_aktiv) && (vars.fulke_A_iranyvalto.ertek != 0.0) 
    {
      vars.fulke_A_jottThrottle = 0.0;
    }
    
    if a.state.kind.is_just_pressed() && (a.name == "Brake") && (vars.fulke_A_aktiv) && (vars.fulke_A_iranyvalto.ertek != 0.0) 
    {

      
      if (vars.fulke_A_menetkapcsolo.ertek > 0.0)  {set_var("HANG_A_menetkapcsolo_menet", true);}
      else if (vars.fulke_A_menetkapcsolo.ertek >-6.0)  {set_var("HANG_A_menetkapcsolo_fek", true);}
      else if (vars.fulke_A_menetkapcsolo.ertek == -6.0)  {set_var("HANG_A_menetkapcsolo_fekMax", true);}
      
      vars.fulke_A_menetkapcsolo.ertek = vars.fulke_A_menetkapcsolo.ertek - 1.0;
      if (vars.fulke_A_menetkapcsolo.ertek < -7.0)  {vars.fulke_A_menetkapcsolo.ertek = -7.0;}
    }
    
    if a.state.kind.is_just_pressed() && (a.name == "MaxBrake") && (vars.fulke_A_aktiv) && (vars.fulke_A_iranyvalto.ertek != 0.0) 
    {
      if (vars.fulke_A_menetkapcsolo.ertek != -7.0)  {set_var("HANG_A_menetkapcsolo_fekMax", true);}
      vars.fulke_A_menetkapcsolo.ertek = -7.0;      
    }
    
    if a.state.kind.is_just_pressed() && (a.name == "Neutral") && (vars.fulke_A_aktiv) && (vars.fulke_A_iranyvalto.ertek != 0.0) 
    {
      if (vars.fulke_A_menetkapcsolo.ertek > 0.0)  {set_var("HANG_A_menetkapcsolo_menetLe", true);}
      else if (vars.fulke_A_menetkapcsolo.ertek < 0.0)  {set_var("HANG_A_menetkapcsolo_fekLe", true);}
      
      
      vars.fulke_A_menetkapcsolo.ertek = 0.0;      
    }
    
    
    
    
    //----------------------------------------------------------------------------------------------
    //FÉKSZELEP
    if a.state.kind.is_just_pressed() && (a.name == "BrakeMinus") && (vars.fulke_A_fekszelepBent) 
    {
      vars.fek_A_mozog = true;

      if (vars.fek_A_cel == 0.0) 
      {
        set_var("HANG_A_fekOld", true);
        vars.fek_A_cel = -1.0;
      }
      if (vars.fek_A_cel > 0.0)  {vars.fek_A_cel = vars.fek_A_cel - 1.0;}
      
      set_var("HANG_A_fekszelep_minusz", true);
    }
    if a.state.kind.is_just_released() && (a.name == "BrakeMinus") && (vars.fulke_A_fekszelepBent) 
    {
      //if (vars.fek_A_cel = -1.0)  vars.fek_A_cel = 0;
    }
    
    if a.state.kind.is_just_pressed() && (a.name == "BrakePlus") && (vars.fulke_A_fekszelepBent) 
    {
      vars.fek_A_mozog = true;

      if (vars.fek_A_cel == 0.0) { set_var("HANG_A_fekPlusz", true);}
      if (vars.fek_A_cel < 1.0)  {vars.fek_A_cel = vars.fek_A_cel + 1.0;}
      
      set_var("HANG_A_fekszelep_plusz", true);
    }
        
    if a.state.kind.is_just_pressed() && (a.name == "MaxBrake") && (vars.fulke_A_fekszelepBent) 
    {
      vars.fek_A_mozog = true;

      if (vars.fek_A_cel < 2.0) 
      {
        vars.fek_A_cel = 2.0;
        set_var("HANG_A_veszfek", true);
      }
      
      set_var("HANG_A_fekszelep_plusz", true);
    }
    
    
    
    
    //----------------------------------------------------------------------------------------------
    //KÉZIFÉK
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_A_CP_kezifekPlusz") 
    {
      vars.fulke_A_kezifekCel = 1.0;
      if (vars.fulke_A_kezifek.ertek > 99.0)  {set_var("HANG_A_kezifek", false);} else {set_var("HANG_A_kezifek", true);}
    }
    if a.state.kind.is_just_released() && (a.name == "BKV1000_A_CP_kezifekPlusz") 
    {
      vars.fulke_A_kezifekCel = 0.0;
      set_var("HANG_A_kezifek", false);
      
    }
    
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_A_CP_kezifekMinusz") 
    {
      vars.fulke_A_kezifekCel = -1.0;
      if (vars.fulke_A_kezifek.ertek < 1.0)  {set_var("HANG_A_kezifek", false);} else {set_var("HANG_A_kezifek", true);}
    }
    if a.state.kind.is_just_released() && (a.name == "BKV1000_A_CP_kezifekMinusz") 
    {
      vars.fulke_A_kezifekCel = 0.0;
      set_var("HANG_A_kezifek", false);
    }
    
    
    
    //----------------------------------------------------------------------------------------------
    // SÍNFÉK
    if a.state.kind.is_just_pressed() && (a.name == "RailBrake") && (vars.fulke_A_aktiv) 
    {
      vars.fulke_A_sinfek.ertek = true;
      set_var("HANG_A_sinfekkapcsolo_be", true);
      set_var("HANG_sinfekBe_kint", true);
      set_var("HANG_sinfekBe_bent", true);
      
      set_var("railbrake_pos", -0.1);
    }
    if a.state.kind.is_just_released() && (a.name == "RailBrake") && (vars.fulke_A_aktiv) 
    {
      vars.fulke_A_sinfek.ertek = false;
      set_var("HANG_A_sinfekkapcsolo_ki", true);
      set_var("HANG_sinfekKi_kint", true);
      set_var("HANG_sinfekKi_bent", true);
      
      set_var("railbrake_pos", 0.0);
    }
    
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_A_CP_sinfek") && (vars.fulke_A_aktiv) 
    {
      vars.fulke_A_sinfek.ertek = true;
      set_var("HANG_A_sinfekkapcsolo_be", true);
      set_var("HANG_sinfekBe_kint", true);
      set_var("HANG_sinfekBe_bent", true);
       
      set_var("railbrake_pos", -0.1); 
    }
    if a.state.kind.is_just_released() && (a.name == "BKV1000_A_CP_sinfek") && (vars.fulke_A_aktiv) 
    {
      vars.fulke_A_sinfek.ertek = false;
      set_var("HANG_A_sinfekkapcsolo_ki", true);
      set_var("HANG_sinfekKi_kint", true);
      set_var("HANG_sinfekKi_bent", true);
       
      set_var("railbrake_pos", 0.0); 
    }
    
    if (vars.fulke_A_sinfek.ertek)  {vars.fulke_A_sinfek.poz = -0.05;}
    if (! vars.fulke_A_sinfek.ertek)  {vars.fulke_A_sinfek.poz = 0.0;}
    
    
    
    //----------------------------------------------------------------------------------------------
    //T?L?RAMV?D? KAPCSOL?
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_A_CP_tularamvedo") 
    {
      set_var("HANG_A_kapcsolo3", true);
      vars.fulke_A_tularamvedo.ertek = ! vars.fulke_A_tularamvedo.ertek;
    }
    if (vars.fulke_A_tularamvedo.ertek)  {vars.fulke_A_tularamvedo.poz = 90.0;}
    else {vars.fulke_A_tularamvedo.poz = 0.0;}
    
    
    
    
    //----------------------------------------------------------------------------------------------
    //L?GS?R?T? KAPCSOL?
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_A_CP_legsuritoKapcsolo") 
    {
      if (vars.fulke_A_legsurito.ertek)  {set_var("HANG_A_sugarKI", true);} else {set_var("HANG_A_sugarBE", true);}
      vars.fulke_A_legsurito.ertek = ! vars.fulke_A_legsurito.ertek;
    }
    if (vars.fulke_A_legsurito.ertek)  {vars.fulke_A_legsurito.poz = 80.0;}
    else {vars.fulke_A_legsurito.poz = 0.0;}
    
   
    
    //----------------------------------------------------------------------------------------------
    //SUG?RVET? KAPCSOL?
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_A_CP_sugarvetoKapcsolo") 
    {
      if (vars.fulke_A_sugarveto.ertek)  {set_var("HANG_A_sugarKI", true);} else {set_var("HANG_A_sugarBE", true);}
      vars.fulke_A_sugarveto.ertek = ! vars.fulke_A_sugarveto.ertek;
    }
    if (vars.fulke_A_sugarveto.ertek)  {vars.fulke_A_sugarveto.poz = -80.0;}
    else {vars.fulke_A_sugarveto.poz = 0.0;}
    
    
    
    //----------------------------------------------------------------------------------------------
    //BAL AJT?SZELEP
    if ( a.state.kind.is_just_pressed() && (a.name == "BKV1000_A_CP_ajtoBalKapcsolo") ) || ( a.state.kind.is_just_pressed() && (a.name == "BKV1000_ajtoBalKapcsolo") && vars.fulke_A_aktiv) 
    {
      if (vars.fulke_A_ajtoBal.ertek)  {set_var("HANG_A_ajtoszelepKI", true);} else {set_var("HANG_A_ajtoszelepBE", true);}
      vars.fulke_A_ajtoBal.ertek = ! vars.fulke_A_ajtoBal.ertek;
    }
    if (vars.fulke_A_ajtoBal.ertek)  {vars.fulke_A_ajtoBal.poz = 40.0}
    else {vars.fulke_A_ajtoBal.poz = 0.0;}
    
    //----------------------------------------------------------------------------------------------
    //JOBB AJT?SZELEP
    if ( a.state.kind.is_just_pressed() && (a.name == "BKV1000_A_CP_ajtoJobbKapcsolo") ) || ( a.state.kind.is_just_pressed() && (a.name == "BKV1000_ajtoJobbKapcsolo") && vars.fulke_A_aktiv) 
    {
      if (vars.fulke_A_ajtoJobb.ertek)  {set_var("HANG_A_ajtoszelepKI", true);} else {set_var("HANG_A_ajtoszelepBE", true);}
      vars.fulke_A_ajtoJobb.ertek = ! vars.fulke_A_ajtoJobb.ertek;
    }
    if (vars.fulke_A_ajtoJobb.ertek)  {vars.fulke_A_ajtoJobb.poz = 40.0}
    else {vars.fulke_A_ajtoJobb.poz = 0.0;}
    
    
    
    //----------------------------------------------------------------------------------------------
    //IND?T?SJELZ? - BE
    if (a.state.kind.is_just_pressed() && (a.name == "BKV1000_inditasjelzo") ) || (a.state.kind.is_just_pressed() && (a.name == "BKV1000_A_CP_inditasjelzoBe") ) 
    {
      set_var("HANG_A_indjelzBE", true);
      if (vars.VanHalozat)  {vars.utas_berrego = true;}
      vars.fulke_A_indjelzoBe.ertek = true;
    }
    if (a.state.kind.is_just_released() && (a.name == "BKV1000_inditasjelzo") ) || (a.state.kind.is_just_released() && (a.name == "BKV1000_A_CP_inditasjelzoBe") )  {vars.fulke_A_indjelzoBe.ertek = false;}
    if (vars.fulke_A_indjelzoBe.ertek)  {vars.fulke_A_indjelzoBe.poz = -0.005;}
    else {vars.fulke_A_indjelzoBe.poz = 0.0;}
    
    //IND?T?SJELZ? - KI
    if (a.state.kind.is_just_pressed() && (a.name == "BKV1000_inditasjelzoKi")) || (a.state.kind.is_just_pressed() && (a.name == "BKV1000_A_CP_inditasjelzoKi"))  
    {
      set_var("HANG_A_indjelzKI", true);
      vars.utas_berrego = false;
      vars.fulke_A_indjelzoKi.ertek = true;
    }
    if (a.state.kind.is_just_released() && (a.name == "BKV1000_inditasjelzoKi")) || (a.state.kind.is_just_released() && (a.name == "BKV1000_A_CP_inditasjelzoKi"))  {vars.fulke_A_indjelzoKi.ertek = false;}
    if (vars.fulke_A_menetkapcsolo.ertek > 1.0)  {vars.utas_berrego = false;}
    if (vars.fulke_A_indjelzoKi.ertek)  {vars.fulke_A_indjelzoKi.poz = -0.005}
    else {vars.fulke_A_indjelzoKi.poz = 0.0;}
    
    
    //----------------------------------------------------------------------------------------------
    //CSENG?
    if (a.state.kind.is_just_pressed() && (a.name == "Bell1.0") && vars.fulke_A_aktiv) || (a.state.kind.is_just_pressed() && (a.name == "BKV1000_A_CP_csengo") ) 
    {
      set_var("HANG_A_csengo", true);
      vars.fulke_A_csengo.ertek = true;
    }
    if (a.state.kind.is_just_released() && (a.name == "Bell1.0") ) || (a.state.kind.is_just_released() && (a.name == "BKV1000_A_CP_csengo") )  {vars.fulke_A_csengo.ertek = false;}
    if (vars.fulke_A_csengo.ertek)  {vars.fulke_A_csengo.poz = -0.03;}
    else {vars.fulke_A_csengo.poz = 0.0;}
    
    
    
    //----------------------------------------------------------------------------------------------
    //HOMOKOL? PED?L
    if (a.state.kind.is_just_pressed() && (a.name == "Sanding") && vars.fulke_A_aktiv) || (a.state.kind.is_just_pressed() && (a.name == "BKV1000_A_CP_homokolo") ) 
    {
      set_var("HANG_A_homokBE", true);
      vars.fulke_A_homokolo.ertek = true;
    //   sanding_0_0 = true;    // TODO
    }
    
    if (a.state.kind.is_just_released() && (a.name == "Sanding") ) || (a.state.kind.is_just_released() && (a.name == "BKV1000_A_CP_homokolo") ) 
    {
      set_var("HANG_A_homokKI", true);
      vars.fulke_A_homokolo.ertek = false;
    //   sanding_0_0 = false;   // TODO
    }
    
    if (vars.fulke_A_homokolo.ertek)  {vars.fulke_A_homokolo.poz = -0.05}
    else {vars.fulke_A_homokolo.poz = 0.0;}
    
 

    
    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // "B" F?LKE CUCCAI
    
    //----------------------------------------------------------------------------------------------
    // BEHELYEZ?SEK
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_B_CP_iranyvaltoBe") && (! vars.fulke_B_aktiv) && (! vars.fulke_A_aktiv) 
    {
      vars.fulke_B_iranyvaltoBent = true;
      set_var("HANG_B_iranyvaltoBe", true);
    }
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_B_CP_iranyvaltoKi") && (vars.fulke_B_iranyvaltoBent) && (vars.fulke_B_iranyvalto.ertek == 0.0) 
    {
      vars.fulke_B_iranyvaltoBent = false;
      set_var("HANG_B_iranyvaltoKi", true);
    }
    
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_B_CP_menetkapcsoloBe") && (! vars.fulke_B_aktiv) && (! vars.fulke_A_aktiv)  
    {
      vars.fulke_B_menetkapcsoloBent = true;
      set_var("HANG_B_menetkapcsoloBe", true);
    }
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_B_CP_menetkapcsoloKi") && (vars.fulke_B_menetkapcsoloBent) && (vars.fulke_B_iranyvalto.ertek == 0.0)  
    {
      vars.fulke_B_menetkapcsoloBent = false;
      set_var("HANG_B_menetkapcsoloKi", true);
    }
    
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_B_CP_fekszelepBe") && (! vars.fulke_B_aktiv) && (! vars.fulke_A_aktiv) 
    {
      vars.fulke_B_fekszelepBent = true;
      set_var("HANG_B_fekszelepBe", true);
    }
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_B_CP_fekszelepKi") && (vars.fulke_B_fekszelepBent) && (vars.fulke_B_iranyvalto.ertek == 0.0) && (vars.fulke_B_fekszelep.ertek == 0.0) 
    {
      vars.fulke_B_fekszelepBent = false;
      set_var("HANG_B_fekszelepKi", true);
    }
    
    //----------------------------------------------------------------------------------------------
    //IR?NYV?LT?
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_B_CP_iranyvaltoElore") && (vars.fulke_B_aktiv) && (vars.fulke_B_menetkapcsolo.ertek == 0.0) 
    {
      if (vars.fulke_B_iranyvalto.ertek < 1.0)  {set_var("HANG_B_iranyvaltoElore", true);}
      vars.fulke_B_iranyvalto.ertek = vars.fulke_B_iranyvalto.ertek + 1.0;
      if (vars.fulke_B_iranyvalto.ertek > 1.0)  {vars.fulke_B_iranyvalto.ertek = 1.0;}
    }
    
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_B_CP_iranyvaltoHatra") && (vars.fulke_B_aktiv) && (vars.fulke_B_menetkapcsolo.ertek == 0.0) 
    {
      if (vars.fulke_B_iranyvalto.ertek > -1.0)  {set_var("HANG_B_iranyvaltoHatra", true);}
      vars.fulke_B_iranyvalto.ertek = vars.fulke_B_iranyvalto.ertek - 1.0;
      if (vars.fulke_B_iranyvalto.ertek < -1.0)  {vars.fulke_B_iranyvalto.ertek = -1.0;}
    }
    
    if a.state.kind.is_just_pressed() && (a.name == "ReverserPlus") && (vars.fulke_B_aktiv) && (a.state.cockpit_index == Some(1)) && (vars.fulke_B_menetkapcsolo.ertek == 0.0) 
    {
      if (vars.fulke_B_iranyvalto.ertek < 1.0)  {set_var("HANG_B_iranyvaltoElore", true);}
      vars.fulke_B_iranyvalto.ertek = vars.fulke_B_iranyvalto.ertek + 1.0;
      if (vars.fulke_B_iranyvalto.ertek > 1.0)  {vars.fulke_B_iranyvalto.ertek = 1.0;}
    }
    
    if a.state.kind.is_just_pressed() && (a.name == "ReverserMinus") && (vars.fulke_B_aktiv) && (a.state.cockpit_index == Some(1)) && (vars.fulke_B_menetkapcsolo.ertek == 0.0) 
    {
      if (vars.fulke_B_iranyvalto.ertek > -1.0)  {set_var("HANG_B_iranyvaltoHatra", true);}
      vars.fulke_B_iranyvalto.ertek = vars.fulke_B_iranyvalto.ertek - 1.0;
      if (vars.fulke_B_iranyvalto.ertek < -1.0)  {vars.fulke_B_iranyvalto.ertek = -1.0;}
    }
    
    if (vars.fulke_B_iranyvalto.ertek == 1.0)  {vars.fulke_B_iranyvalto.poz = config::CFG_IRANYVALTO_ELORE;}
    if (vars.fulke_B_iranyvalto.ertek == 0.0)  {vars.fulke_B_iranyvalto.poz = 0.0;}
    if (vars.fulke_B_iranyvalto.ertek == -1.0)  {vars.fulke_B_iranyvalto.poz = config::CFG_IRANYVALTO_HATRA;}
    
    
    //----------------------------------------------------------------------------------------------
    //MENETKAPCSOL?
    
    if a.state.kind.is_just_pressed() && (a.name == "Throttle") && (vars.fulke_B_aktiv) && (vars.fulke_B_iranyvalto.ertek != 0.0) 
    {

      //kontrollerhez a v?ltoz? billent?se
      vars.fulke_B_jottThrottle = 1.0;
    }
    
    if a.state.kind.is_just_released() && (a.name == "Throttle") && (vars.fulke_B_aktiv) && (vars.fulke_B_iranyvalto.ertek != 0.0) 
    {
      vars.fulke_B_jottThrottle = 0.0;
    }
    
    if a.state.kind.is_just_pressed() && (a.name == "Brake") && (vars.fulke_B_aktiv) && (vars.fulke_B_iranyvalto.ertek != 0.0) 
    {
      
      
      if (vars.fulke_B_menetkapcsolo.ertek > 0.0)  {set_var("HANG_B_menetkapcsolo_menet", true);}
      else if (vars.fulke_B_menetkapcsolo.ertek >-6.0)  {set_var("HANG_B_menetkapcsolo_fek", true);}
      else if (vars.fulke_B_menetkapcsolo.ertek == -6.0)  {set_var("HANG_B_menetkapcsolo_fekMax", true);}
      
      vars.fulke_B_menetkapcsolo.ertek = vars.fulke_B_menetkapcsolo.ertek - 1.0;
      if (vars.fulke_B_menetkapcsolo.ertek < -7.0)  {vars.fulke_B_menetkapcsolo.ertek = -7.0;}
    }
    
    if a.state.kind.is_just_pressed() && (a.name == "MaxBrake") && (vars.fulke_B_aktiv) && (vars.fulke_B_iranyvalto.ertek != 0.0) 
    {
      if (vars.fulke_B_menetkapcsolo.ertek != -7.0)  {set_var("HANG_B_menetkapcsolo_fekMax", true);}
      vars.fulke_B_menetkapcsolo.ertek = -7.0;      
    }
    
    if a.state.kind.is_just_pressed() && (a.name == "Neutral") && (vars.fulke_B_aktiv) && (vars.fulke_B_iranyvalto.ertek != 0.0) 
    {
      if (vars.fulke_B_menetkapcsolo.ertek > 0.0)  {set_var("HANG_B_menetkapcsolo_menetLe", true);}
      else if (vars.fulke_B_menetkapcsolo.ertek < 0.0)  {set_var("HANG_B_menetkapcsolo_fekLe", true);}
      
      vars.fulke_B_menetkapcsolo.ertek = 0.0;      
    }
    
    
    
    
    //----------------------------------------------------------------------------------------------
    //FÉKSZELEP
    if a.state.kind.is_just_pressed() && (a.name == "BrakeMinus") && (vars.fulke_B_fekszelepBent) 
    {
      vars.fek_B_mozog = true;

      if (vars.fek_B_cel == 0.0) 
      {
        set_var("HANG_B_fekOld", true);
        vars.fek_B_cel = -1.0;
      }
      if (vars.fek_B_cel > 0.0)  {vars.fek_B_cel = vars.fek_B_cel - 1.0;}
      
      set_var("HANG_B_fekszelep_minusz", true);
    }
    if a.state.kind.is_just_released() && (a.name == "BrakeMinus") && (vars.fulke_B_fekszelepBent) 
    {
      //if (vars.fek_B_cel = -1.0)  vars.fek_B_cel = 0;
    }
    
    if a.state.kind.is_just_pressed() && (a.name == "BrakePlus") && (vars.fulke_B_fekszelepBent) 
    {
      vars.fek_B_mozog = true;

      if (vars.fek_B_cel == 0.0)  {set_var("HANG_B_fekPlusz", true);}
      if (vars.fek_B_cel < 1.0)  {vars.fek_B_cel = vars.fek_B_cel + 1.0;}
      
      set_var("HANG_B_fekszelep_plusz", true);
    }
        
    if a.state.kind.is_just_pressed() && (a.name == "MaxBrake") && (vars.fulke_B_fekszelepBent) 
    {
      vars.fek_B_mozog = true;

      if (vars.fek_B_cel < 2.0) 
      {
        vars.fek_B_cel = 2.0;
        set_var("HANG_B_veszfek", true);
      }
      
      set_var("HANG_B_fekszelep_plusz", true);
    }
    
    
    
    
    //----------------------------------------------------------------------------------------------
    //K?ZIF?K
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_B_CP_kezifekPlusz") 
    {
      vars.fulke_B_kezifekCel = 1.0;
      if (vars.fulke_B_kezifek.ertek > 99.0)  {set_var("HANG_B_kezifek", false);} else {set_var("HANG_B_kezifek", true);}
    }
    if a.state.kind.is_just_released() && (a.name == "BKV1000_B_CP_kezifekPlusz") 
    {
      vars.fulke_B_kezifekCel = 0.0;
      set_var("HANG_B_kezifek", false);
    }
    
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_B_CP_kezifekMinusz") 
    {
      vars.fulke_B_kezifekCel = -1.0;
      if (vars.fulke_B_kezifek.ertek < 1.0)  {set_var("HANG_B_kezifek", false);} else {set_var("HANG_B_kezifek", true);}
    }
    if a.state.kind.is_just_released() && (a.name == "BKV1000_B_CP_kezifekMinusz") 
    {
      vars.fulke_B_kezifekCel = 0.0;
      set_var("HANG_B_kezifek", false);
    }
    
    
    
    //----------------------------------------------------------------------------------------------
    //S?NF?K
    if a.state.kind.is_just_pressed() && (a.name == "RailBrake") && (vars.fulke_B_aktiv) 
    {
      vars.fulke_B_sinfek.ertek = true;
      set_var("HANG_B_sinfekkapcsolo_be", true);
      set_var("HANG_sinfekBe_kint", true);
      set_var("HANG_sinfekBe_bent", true);
      
      set_var("railbrake_pos", -0.1);
    }
    if a.state.kind.is_just_released() && (a.name == "RailBrake") && (vars.fulke_B_aktiv) 
    {
      vars.fulke_B_sinfek.ertek = false;
      set_var("HANG_B_sinfekkapcsolo_ki", true);
      set_var("HANG_sinfekKi_kint", true);
      set_var("HANG_sinfekKi_bent", true);
         
      set_var("railbrake_pos", 0.0);    
    }
    
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_B_CP_sinfek") && (vars.fulke_B_aktiv) 
    {
      vars.fulke_B_sinfek.ertek = true;
      set_var("HANG_B_sinfekkapcsolo_be", true);
      set_var("HANG_sinfekBe_kint", true);
      set_var("HANG_sinfekBe_bent", true);
        
      set_var("railbrake_pos", -0.1);
    }
    if a.state.kind.is_just_released() && (a.name == "BKV1000_B_CP_sinfek") && (vars.fulke_B_aktiv) 
    {
      vars.fulke_B_sinfek.ertek = false;
      set_var("HANG_B_sinfekkapcsolo_ki", true);
      set_var("HANG_sinfekKi_kint", true);
      set_var("HANG_sinfekKi_bent", true);
      
      set_var("railbrake_pos", 0.0);   
    }
    
    if (vars.fulke_B_sinfek.ertek)  {vars.fulke_B_sinfek.poz = -0.05;}
    if (! vars.fulke_B_sinfek.ertek)  {vars.fulke_B_sinfek.poz = 0.0;}
    
    
    
    //----------------------------------------------------------------------------------------------
    //T?L?RAMV?D? KAPCSOL?
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_B_CP_tularamvedo") 
    {
      set_var("HANG_B_kapcsolo3", true);
      vars.fulke_B_tularamvedo.ertek = ! vars.fulke_B_tularamvedo.ertek;
    }
    if (vars.fulke_B_tularamvedo.ertek)  {vars.fulke_B_tularamvedo.poz = 90.0}
    else {vars.fulke_B_tularamvedo.poz = 0.0;}
    
    
    //----------------------------------------------------------------------------------------------
    //SUG?RVET? KAPCSOL?
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_B_CP_sugarvetoKapcsolo") 
    {
      if (vars.fulke_B_sugarveto.ertek)  {set_var("HANG_B_sugarKI", true);} else {set_var("HANG_B_sugarBE", true);}
      vars.fulke_B_sugarveto.ertek = ! vars.fulke_B_sugarveto.ertek;
    }
    if (vars.fulke_B_sugarveto.ertek)  {vars.fulke_B_sugarveto.poz = -80.0}
    else {vars.fulke_B_sugarveto.poz = 0.0;}
    
    
    
    //----------------------------------------------------------------------------------------------
    //BAL AJT?SZELEP (AMI IGAZ?B?L A JOBB OLDAL)
    if ( a.state.kind.is_just_pressed() && (a.name == "BKV1000_B_CP_ajtoBalKapcsolo") ) || ( a.state.kind.is_just_pressed() && (a.name == "BKV1000_ajtoBalKapcsolo") && vars.fulke_B_aktiv) 
    {
      if (vars.fulke_B_ajtoBal.ertek)  {set_var("HANG_B_ajtoszelepKI", true);} else {set_var("HANG_B_ajtoszelepBE", true);}
      vars.fulke_B_ajtoBal.ertek = ! vars.fulke_B_ajtoBal.ertek;
    }
    if (vars.fulke_B_ajtoBal.ertek)  {vars.fulke_B_ajtoBal.poz = 40.0}
    else {vars.fulke_B_ajtoBal.poz = 0.0;}
    
    //----------------------------------------------------------------------------------------------
    //JOBB AJT?SZELEP (AMI IGAZ?B?L A BAL OLDAL)
    if ( a.state.kind.is_just_pressed() && (a.name == "BKV1000_B_CP_ajtoJobbKapcsolo") ) || ( a.state.kind.is_just_pressed() && (a.name == "BKV1000_ajtoJobbKapcsolo") && vars.fulke_B_aktiv) 
    {
      if (vars.fulke_B_ajtoJobb.ertek)  {set_var("HANG_B_ajtoszelepKI", true);} else {set_var("HANG_B_ajtoszelepBE", true);}
      vars.fulke_B_ajtoJobb.ertek = ! vars.fulke_B_ajtoJobb.ertek;
    }
    if (vars.fulke_B_ajtoJobb.ertek)  {vars.fulke_B_ajtoJobb.poz = 40.0}
    else {vars.fulke_B_ajtoJobb.poz = 0.0;}
    
    
    
    //----------------------------------------------------------------------------------------------
    //IND?T?SJELZ? - BE
    if (a.state.kind.is_just_pressed() && (a.name == "BKV1000_inditasjelzo") ) || (a.state.kind.is_just_pressed() && (a.name == "BKV1000_B_CP_inditasjelzoBe") ) 
    {
      set_var("HANG_B_indjelzBE", true);
      if (vars.VanHalozat)  {vars.utas_berrego = true;}
      vars.fulke_B_indjelzoBe.ertek = true;
    }
    if (a.state.kind.is_just_released() && (a.name == "BKV1000_inditasjelzo") ) || (a.state.kind.is_just_released() && (a.name == "BKV1000_B_CP_inditasjelzoBe") )  {vars.fulke_B_indjelzoBe.ertek = false;}
    if (vars.fulke_B_indjelzoBe.ertek)  {vars.fulke_B_indjelzoBe.poz = -0.005}
    else {vars.fulke_B_indjelzoBe.poz = 0.0;}
    
    //IND?T?SJELZ? - KI
    if (a.state.kind.is_just_pressed() && (a.name == "BKV1000_inditasjelzoKi")) || (a.state.kind.is_just_pressed() && (a.name == "BKV1000_B_CP_inditasjelzoKi"))  
    {
      set_var("HANG_B_indjelzKI", true);
      vars.utas_berrego = false;
      vars.fulke_B_indjelzoKi.ertek = true;
    }
    if (a.state.kind.is_just_released() && (a.name == "BKV1000_inditasjelzoKi")) || (a.state.kind.is_just_released() && (a.name == "BKV1000_B_CP_inditasjelzoKi"))  {vars.fulke_B_indjelzoKi.ertek = false;}
    if (vars.fulke_B_menetkapcsolo.ertek > 1.0)  {vars.utas_berrego = false;}
    if (vars.fulke_B_indjelzoKi.ertek)  {vars.fulke_B_indjelzoKi.poz = -0.005}
    else {vars.fulke_B_indjelzoKi.poz = 0.0;}
    
    
    //----------------------------------------------------------------------------------------------
    //CSENG?
    if (a.state.kind.is_just_pressed() && (a.name == "Bell1.0") && vars.fulke_B_aktiv) || (a.state.kind.is_just_pressed() && (a.name == "BKV1000_B_CP_csengo") ) 
    {
      set_var("HANG_B_csengo", true);
      vars.fulke_B_csengo.ertek = true;
    }
    if (a.state.kind.is_just_released() && (a.name == "Bell1.0") ) || (a.state.kind.is_just_released() && (a.name == "BKV1000_B_CP_csengo") )  {vars.fulke_B_csengo.ertek = false;}
    if (vars.fulke_B_csengo.ertek)  {vars.fulke_B_csengo.poz = -0.03}
    else {vars.fulke_B_csengo.poz = 0.0;}
    
    
    
    //----------------------------------------------------------------------------------------------
    //HOMOKOL? PED?L
    if (a.state.kind.is_just_pressed() && (a.name == "Sanding") && vars.fulke_B_aktiv) || (a.state.kind.is_just_pressed() && (a.name == "BKV1000_B_CP_homokolo") ) 
    {
      set_var("HANG_B_homokBE", true);
      vars.fulke_B_homokolo.ertek = true;
    //   sanding_0_0 = true;    // TODO
    }
    
    if (a.state.kind.is_just_released() && (a.name == "Sanding") ) || (a.state.kind.is_just_released() && (a.name == "BKV1000_B_CP_homokolo") ) 
    {
      set_var("HANG_B_homokKI", true);
      vars.fulke_B_homokolo.ertek = false;
    //   sanding_0_0 = false;  // TODO
    }
    
    if (vars.fulke_B_homokolo.ertek)  {vars.fulke_B_homokolo.poz = -0.05}
    else {vars.fulke_B_homokolo.poz = 0.0;}
    
    
    
    
    
    
    
    
    
    //----------------------------------------------------------------------------------------------------------------------
    // F?LKE F?GGETLEN CUCCOK
    
    //----------------------------------------------------------------------------------------------
    //?RAMSZED? FEL/LE
    if a.state.kind.is_just_pressed() && (a.name == "PantographUp") 
    {
      vars.ALL_szedo = !vars.ALL_szedo;
    }
    // if a.state.kind.is_just_pressed() && (a.name == "PantographDn") 
    // {
    //   //set_var("HANG_A_csengo = 1.0;
    //   vars.ALL_szedo = false;
    // }
    
    //----------------------------------------------------------------------------------------------
    //?RAMSZED? ?TFORD?T?SA
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_szedoAtforditas") && vars.ALL_szedo 
    {
      vars.ALL_szedo_irany = ! vars.ALL_szedo_irany;
    }
    
    
    
    
    //----------------------------------------------------------------------------------------------
    // F?NYSZ?R? KAPCSOL?S
    // HA NINCS BEKAPCSOLVA A VIL F?KAP, BEKAPCSOL AZ (HELYZETJELZ?), UT?NA KAPCSOL BE AZ ADOTT OLDALI F??NYSZ?R?
    
    if a.state.kind.is_just_pressed() && (a.name == "FrontLightPlus") 
    {
			//------------------//
      // HA A F?KAP KI VAN KAPCSOLVA
      if (vars.K1.ertek == false) 
      {
				if (vars.K1.ertek)  {set_var("HANG_K1_KI", true);} else {set_var("HANG_K1_BE", true);}
				vars.K1.ertek = ! vars.K1.ertek;
				if (vars.K1.ertek)  {vars.K1.poz = -90.0} else {vars.K1.poz = 0.0;}
      }
      
      //---------------//
			//HA A F?KAP BE VAN KAPCSOLVA, AZ ADOTT OLDALI F?NYSZ?R? KAPCSOL
			else
			{
				//HA AZ "A" F?LK?BEN VAGYUNK
				if (a.state.cockpit_index == Some(0)) 
				{
					if (vars.fulke_A_sugarveto.ertek)  {set_var("HANG_A_sugarKI", true);} else {set_var("HANG_A_sugarBE", true);}
					vars.fulke_A_sugarveto.ertek = ! vars.fulke_A_sugarveto.ertek;
					if (vars.fulke_A_sugarveto.ertek)  {vars.fulke_A_sugarveto.poz = -80.0} else {vars.fulke_A_sugarveto.poz = 0.0;}
				}
				
				//HA AZ "B" F?LK?BEN VAGYUNK
				if (a.state.cockpit_index == Some(1)) 
				{
					if (vars.fulke_B_sugarveto.ertek)  {set_var("HANG_B_sugarKI", true);} else {set_var("HANG_B_sugarBE", true);}
					vars.fulke_B_sugarveto.ertek = ! vars.fulke_B_sugarveto.ertek;
					if (vars.fulke_B_sugarveto.ertek)  {vars.fulke_B_sugarveto.poz = -80.0} else {vars.fulke_B_sugarveto.poz = 0.0;}
				}
						
			}	//f?kap ki felt?tel v?ge
    }	//frontlightplus v?ge
    
    
    
    
    
    //----------------------------------------------------------------------------------------------
    // F?NYSZ?R? KIKAPCSOL?S
    // EL?SZ?R AZ ADOTT OLDALI F?NYSZ?R? KAPCSOL KI (HELYZETJELZ?RE), UT?NA KAPCSOL KI A VIL F?KAPCSOL?
    
    if a.state.kind.is_just_pressed() && (a.name == "FrontLightMinus") 
    {
        //-----------------------//
        // HA B?RMELYIK F?NYSZ?R? BE VAN KAPCSOLVA, AZT KAPCSOLJUK LE
        if (vars.fulke_A_sugarveto.ertek || vars.fulke_B_sugarveto.ertek) 
        {
            //HA AZ "A" F?LK?BEN VAGYUNK
            if (a.state.cockpit_index == Some(0)) 
            {
                if (vars.fulke_A_sugarveto.ertek)  {set_var("HANG_A_sugarKI", true);} else {set_var("HANG_A_sugarBE", true);}
                vars.fulke_A_sugarveto.ertek = ! vars.fulke_A_sugarveto.ertek;
                if (vars.fulke_A_sugarveto.ertek)  {vars.fulke_A_sugarveto.poz = -80.0} else {vars.fulke_A_sugarveto.poz = 0.0;}
            }
            
            //HA AZ "B" F?LK?BEN VAGYUNK
            if (a.state.cockpit_index == Some(1)) 
            {
                if (vars.fulke_B_sugarveto.ertek)  {set_var("HANG_B_sugarKI", true);} else {set_var("HANG_B_sugarBE", true);}
                vars.fulke_B_sugarveto.ertek = ! vars.fulke_B_sugarveto.ertek;
                if (vars.fulke_B_sugarveto.ertek)  {vars.fulke_B_sugarveto.poz = -80.0} else {vars.fulke_B_sugarveto.poz = 0.0;}
            }
        }

            
            
        //------------------//
        // HA A F?NYSZ?R?K KI VANNAK KAPCSOLVA, A VIL F?KAPOT NYOMJUK LE
        else
        {
            if (vars.K1.ertek)  {set_var("HANG_K1_KI", true);} else {set_var("HANG_K1_BE", true);}
            vars.K1.ertek = ! vars.K1.ertek;
            if (vars.K1.ertek)  {vars.K1.poz = -90.0} else {vars.K1.poz = 0.0;}
        }//f?nysz?r? felt?tel v?ge
    }	//frontlightminus v?ge
    
    
    
    
    
    
    
    
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // UTAST?R KAPCSOL?K
    
    //----------------------------------------------------------------------------------------------
    // Vil?g?t?s f?kapcsol?
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_vilagitas_K1") 
    {
      if (vars.K1.ertek)  {set_var("HANG_K1_KI", true);} else {set_var("HANG_K1_BE", true);}
      vars.K1.ertek = ! vars.K1.ertek;
    }
    if (vars.K1.ertek)  {vars.K1.poz = -90.0}
    else {vars.K1.poz = 0.0;}
    
    //----------------------------------------------------------------------------------------------
    // Vil?g?t?s szakaszkapcsol? (helyzetjelz?, z?rf?ny, vil?g?t?s, ellent?tes peronvil., sz?mjelz?)
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_vilagitas_K2") 
    {
      if (vars.K2.ertek)  {set_var("HANG_K1_KI", true);} else {set_var("HANG_K1_BE", true);}
      vars.K2.ertek = ! vars.K2.ertek;
    }
    if (vars.K2.ertek)  {vars.K2.poz = -90.0}
    else {vars.K2.poz = 0.0;}
    
    //----------------------------------------------------------------------------------------------
    // z?rf?ny ir?nyv?laszt?
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_vilagitas_K3") 
    {
      if (vars.K3.ertek)  {set_var("HANG_K1_KI", true);} else {set_var("HANG_K1_BE", true);}
      vars.K3.ertek = ! vars.K3.ertek;
    }
    if (vars.K3.ertek)  {vars.K3.poz = -90.0}
    else {vars.K3.poz = 0.0;}
    
    
    
    
    
    
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // ABLAKTEKER?K
    
    //----------------------------------------------------------------------------------------------
    // BALOLDAL
    
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_ablaktekero_B2_Fel") 
    {
      if (vars.utas_ablak_B2 <= 1.0)  {set_var("HANG_ablakBAL_FEL", true);}
      vars.ablakB2_cel = 1;
    }
    if a.state.kind.is_just_released() && (a.name == "BKV1000_ablaktekero_B2_Fel") 
    {
      set_var("HANG_ablakBAL_FEL", false);
      vars.ablakB2_cel = 0;
    }
    //---------------
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_ablaktekero_B2_Le") 
    {
      if (vars.utas_ablak_B2 >= 0.0)  {set_var("HANG_ablakBAL_LE", true);}
      vars.ablakB2_cel = -1;
    }
    if a.state.kind.is_just_released() && (a.name == "BKV1000_ablaktekero_B2_Le") 
    {
      set_var("HANG_ablakBAL_LE", false);
      vars.ablakB2_cel = 0;
    }
    //------------------------------------------------------------------------------
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_ablaktekero_B3_Fel") 
    {
      if (vars.utas_ablak_B3 <= 1.0)  {set_var("HANG_ablakBAL_FEL", true);}
      vars.ablakB3_cel = 1;
    }
    if a.state.kind.is_just_released() && (a.name == "BKV1000_ablaktekero_B3_Fel") 
    {
      set_var("HANG_ablakBAL_FEL", false);
      vars.ablakB3_cel = 0;
    }
    //---------------
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_ablaktekero_B3_Le") 
    {
      if (vars.utas_ablak_B3 >= 0.0)  {set_var("HANG_ablakBAL_LE", true);}
      vars.ablakB3_cel = -1;
    }
    if a.state.kind.is_just_released() && (a.name == "BKV1000_ablaktekero_B3_Le") 
    {
      set_var("HANG_ablakBAL_LE", false);
      vars.ablakB3_cel = 0;
    }
    //------------------------------------------------------------------------------
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_ablaktekero_B4_Fel") 
    {
      if (vars.utas_ablak_B4 <= 1.0)  {set_var("HANG_ablakBAL_FEL", true);}
      vars.ablakB4_cel = 1;
    }
    if a.state.kind.is_just_released() && (a.name == "BKV1000_ablaktekero_B4_Fel") 
    {
      set_var("HANG_ablakBAL_FEL", false);
      vars.ablakB4_cel = 0;
    }
    //---------------
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_ablaktekero_B4_Le") 
    {
      if (vars.utas_ablak_B4 >= 0.0)  {set_var("HANG_ablakBAL_LE", true);}
      vars.ablakB4_cel = -1;
    }
    if a.state.kind.is_just_released() && (a.name == "BKV1000_ablaktekero_B4_Le") 
    {
      set_var("HANG_ablakBAL_LE", false);
      vars.ablakB4_cel = 0;
    }
    
    
    //----------------------------------------------------------------------------------------------
    // JOBB OLDAL
    
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_ablaktekero_J2_Fel") 
    {
      if (vars.utas_ablak_J2 <= 1.0)  {set_var("HANG_ablakJOBB_FEL", true);}
      vars.ablakJ2_cel = 1;
    }
    if a.state.kind.is_just_released() && (a.name == "BKV1000_ablaktekero_J2_Fel") 
    {
      set_var("HANG_ablakJOBB_FEL", false);
      vars.ablakJ2_cel = 0;
    }
    //---------------
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_ablaktekero_J2_Le") 
    {
      if (vars.utas_ablak_J2 >= 0.0)  {set_var("HANG_ablakJOBB_LE", true);}
      vars.ablakJ2_cel = -1;
    }
    if a.state.kind.is_just_released() && (a.name == "BKV1000_ablaktekero_J2_Le") 
    {
      set_var("HANG_ablakJOBB_LE", false);
      vars.ablakJ2_cel = 0;
    }
    //------------------------------------------------------------------------------
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_ablaktekero_J3_Fel") 
    {
      if (vars.utas_ablak_J3 <= 1.0)  {set_var("HANG_ablakJOBB_FEL", true);}
      vars.ablakJ3_cel = 1;
    }
    if a.state.kind.is_just_released() && (a.name == "BKV1000_ablaktekero_J3_Fel") 
    {
      set_var("HANG_ablakJOBB_FEL", false);
      vars.ablakJ3_cel = 0;
    }
    //---------------
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_ablaktekero_J3_Le") 
    {
      if (vars.utas_ablak_J3 >= 0.0)  {set_var("HANG_ablakJOBB_LE", true);}
      vars.ablakJ3_cel = -1;
    }
    if a.state.kind.is_just_released() && (a.name == "BKV1000_ablaktekero_J3_Le") 
    {
      set_var("HANG_ablakJOBB_LE", false);
      vars.ablakJ3_cel = 0;
    }
    //------------------------------------------------------------------------------
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_ablaktekero_J4_Fel") 
    {
      if (vars.utas_ablak_J4 <= 1.0)  {set_var("HANG_ablakJOBB_FEL", true);}
      vars.ablakJ4_cel = 1;
    }
    if a.state.kind.is_just_released() && (a.name == "BKV1000_ablaktekero_J4_Fel") 
    {
      set_var("HANG_ablakJOBB_FEL", false);
      vars.ablakJ4_cel = 0;
    }
    //---------------
    if a.state.kind.is_just_pressed() && (a.name == "BKV1000_ablaktekero_J4_Le") 
    {
      if (vars.utas_ablak_J4 >= 0.0)  {set_var("HANG_ablakJOBB_LE", true);}
      vars.ablakJ4_cel = -1;
    }
    if a.state.kind.is_just_released() && (a.name == "BKV1000_ablaktekero_J4_Le") 
    {
      set_var("HANG_ablakJOBB_LE", false);
      vars.ablakJ4_cel = 0;
    }



}


