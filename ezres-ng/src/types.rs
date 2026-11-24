#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, nonstandard_style, dead_code, unused)]

// Imports
use lotus_script::prelude::*;
use lotus_extra::rand::gen_f32;
// use lotus_script::var::set_var;

use crate::{config, func};


// --- SWITCHES ------------------------------------------------------------------------ //

#[derive(Default)]
pub struct tKapcsolo {
    // state variables
    pub ertek: bool,
    pub poz: f32,
}
impl tKapcsolo {
    // Init 
    // This init function takes two params: _name (name of the PUBLIC_VAR light variable) and _maxlight (maximum lightness of this light)
        pub fn new() -> tKapcsolo {
            tKapcsolo  { ertek: false, poz: 0.0 }
    }
}


#[derive(Default)]
pub struct tTobbAllasuKapcsolo {
    // state variables
    pub ertek: f32,
    pub poz: f32,
}
impl tTobbAllasuKapcsolo {
    // Init 
    // This init function takes two params: _name (name of the PUBLIC_VAR light variable) and _maxlight (maximum lightness of this light)
        pub fn new() -> tTobbAllasuKapcsolo {
            tTobbAllasuKapcsolo  { ertek: 0.0, poz: 0.0 }
    }
}


// --- GLOBAL VARIABLES ---------------------------------------------------------------------------------------------------------------- //
#[derive(Default)]
pub struct t_vars {

    // --- COCKPIT A ------------------------------------------------ //
    pub A_CP_iranyvaltoBent: bool,
    pub A_CP_menetkapcsoloBent: bool,
    pub A_CP_fekszelepBent: bool,

    pub A_CP_iranyvalto: f32,
    pub A_CP_menetkapcsolo: f32,
    pub A_CP_fekszelep: f32,
    pub A_CP_kezifek: f32,
    pub A_CP_sinfek: f32,
    pub A_CP_tularamKapcsolo: f32,
    pub A_CP_kotelCsomo: bool,
    pub A_CP_kotelNemCsomo: bool,

    pub A_CP_csengo: f32,
    pub A_CP_homokolo: f32,
    pub A_CP_ajtoBalKapcsolo: f32,
    pub A_CP_ajtoJobbKapcsolo: f32,
    pub A_CP_sugarvetoKapcsolo: f32,
    pub A_CP_legsuritoKapcsolo: f32,
    pub A_CP_berregoBe: f32,
    pub A_CP_berregoKi: f32,

    pub A_CP_szamlalo1: f32,
    pub A_CP_jottThrottle: f32,

    // --- //

    pub fulke_A_aktiv: bool,

    pub fulke_A_szamlalo1: f32,
    pub fulke_A_jottThrottle: f32,
    pub fulke_A_kezifekCel: f32,
    pub menetfokozatElozo_A: f32,
    pub fek_A_cel: f32,
    pub fek_A_mozog: bool,

    pub fulke_A_iranyvaltoBent: bool,
    pub fulke_A_menetkapcsoloBent: bool,
    pub fulke_A_fekszelepBent: bool,

    pub fulke_A_menetkapcsolo: tTobbAllasuKapcsolo,
    pub fulke_A_iranyvalto: tTobbAllasuKapcsolo,
    pub fulke_A_fekszelep: tTobbAllasuKapcsolo,
    pub fulke_A_kezifek: tTobbAllasuKapcsolo,  

    pub fulke_A_sinfek: tKapcsolo,
    pub fulke_A_tularamvedo: tKapcsolo,
    pub fulke_A_kotelCsomo: bool,
    pub fulke_A_kotelNemCsomo: bool,

    pub fulke_A_ajtoBal: tKapcsolo,
    pub fulke_A_ajtoJobb: tKapcsolo,
    pub fulke_A_csengo: tKapcsolo,
    pub fulke_A_sugarveto: tKapcsolo,
    pub fulke_A_legsurito: tKapcsolo,
    pub fulke_A_indjelzoBe: tKapcsolo,
    pub fulke_A_indjelzoKi: tKapcsolo,
    pub fulke_A_homokolo: tKapcsolo,

    // --- COCKPIT B ------------------------------------------------ //
    pub B_CP_iranyvaltoBent: bool,
    pub B_CP_menetkapcsoloBent: bool,
    pub B_CP_fekszelepBent: bool,

    pub B_CP_iranyvalto: f32,
    pub B_CP_menetkapcsolo: f32,
    pub B_CP_fekszelep: f32,
    pub B_CP_kezifek: f32,
    pub B_CP_sinfek: f32,
    pub B_CP_tularamKapcsolo: f32,
    pub B_CP_kotelCsomo: bool,
    pub B_CP_kotelNemCsomo: bool,

    pub B_CP_csengo: f32,
    pub B_CP_homokolo: f32,
    pub B_CP_ajtoBalKapcsolo: f32,
    pub B_CP_ajtoJobbKapcsolo: f32,
    pub B_CP_sugarvetoKapcsolo: f32,
    pub B_CP_legsuritoKapcsolo: f32,
    pub B_CP_berregoBe: f32,
    pub B_CP_berregoKi: f32,

    pub B_CP_szamlalo1: f32,
    pub B_CP_jottThrottle: f32,

    // --- //

    pub fulke_B_aktiv: bool,

    pub fulke_B_szamlalo1: f32,
    pub fulke_B_jottThrottle: f32,
    pub fulke_B_kezifekCel: f32,
    pub menetfokozatElozo_B: f32,
    pub fek_B_cel: f32,
    pub fek_B_mozog: bool,

    pub fulke_B_iranyvaltoBent: bool,
    pub fulke_B_menetkapcsoloBent: bool,
    pub fulke_B_fekszelepBent: bool,

    pub fulke_B_menetkapcsolo: tTobbAllasuKapcsolo,
    pub fulke_B_iranyvalto: tTobbAllasuKapcsolo,
    pub fulke_B_fekszelep: tTobbAllasuKapcsolo,
    pub fulke_B_kezifek: tTobbAllasuKapcsolo,  

    pub fulke_B_sinfek: tKapcsolo,
    pub fulke_B_tularamvedo: tKapcsolo,
    pub fulke_B_kotelCsomo: bool,
    pub fulke_B_kotelNemCsomo: bool,

    pub fulke_B_ajtoBal: tKapcsolo,
    pub fulke_B_ajtoJobb: tKapcsolo,
    pub fulke_B_csengo: tKapcsolo,
    pub fulke_B_sugarveto: tKapcsolo,
    pub fulke_B_legsurito: tKapcsolo,
    pub fulke_B_indjelzoBe: tKapcsolo,
    pub fulke_B_indjelzoKi: tKapcsolo,
    pub fulke_B_homokolo: tKapcsolo,

    
    // --- PASSENGER ------------------------------------------------ //
    pub utas_berrego: bool,
    pub utas_vilagitasKapcsolo1: f32,
    pub utas_vilagitasKapcsolo2: f32,
    pub utas_vilagitasKapcsolo3: f32,

    pub utas_ablak_B2: f32,
    pub utas_ablak_B3: f32,
    pub utas_ablak_B4: f32,
    pub utas_ablak_J2: f32,
    pub utas_ablak_J3: f32,
    pub utas_ablak_J4: f32,

    pub utas_ajto_B1: f32,
    pub utas_ajto_B2: f32,
    pub utas_ajto_J1: f32,
    pub utas_ajto_J2: f32,

    pub kapaszkodo_X_szog: f32,
    pub kapaszkodo_X_szogseb: f32,
    pub kapaszkodo_Y_szog: f32,
    pub kapaszkodo_Y_szogseb: f32,

    pub PHY_gyorsulas: f32,
    pub PHY_oldalgyorsulas: f32,

    pub ablakB2_cel: i32,
    pub ablakB3_cel: i32,
    pub ablakB4_cel: i32,
    pub ablakJ2_cel: i32,
    pub ablakJ3_cel: i32,
    pub ablakJ4_cel: i32,

    pub K1: tKapcsolo,
    pub K2: tKapcsolo,
    pub K3: tKapcsolo,


    // --- MISC: VARS ------------------------------------------------ //
    pub ALL_szedo_szog: f32,
    pub ALL_szedo_szog_ABS: f32,
    pub ALL_szedo: bool,
    pub ALL_szedo_irany: bool,
    pub ALL_tularamvedo: bool,
    pub SZEDO_ALLAS: f32,
    pub SZEDO_SZOG_MAX: f32,

    pub ALL_sinfek: bool,
    pub ALL_kezifek: f32,
    pub ALL_menetkapcsolo: f32,
    pub ALL_iranyvalto: f32,
    pub ALL_biztonsagi_fek: bool,

    pub AJTOJOBB_CEL: bool,
    pub AJTOBAL_CEL: bool,
    pub AJTO_J1_SEB: f32,
    pub AJTO_J2_SEB: f32,
    pub AJTO_B1_SEB: f32,
    pub AJTO_B2_SEB: f32,
    pub AJTO_J1_ROGZ: bool,
    pub AJTO_J2_ROGZ: bool,
    pub AJTO_B1_ROGZ: bool,
    pub AJTO_B2_ROGZ: bool,

    pub ALL_fekhengerNyomas: f32,
    pub ALL_fektartalyNyomas: f32,
    pub VanHalozat: bool,
    
    pub legsuritoJar: bool,
    pub RUGO_UT: f32,
    pub VAR_PHY_lassulas: f32,
    


    // --- MISC: SOUNDS ------------------------------------------------ //
    pub HANG_motorhangero: f32,
    pub VOL_HANG_rantas_iv: f32,

    pub VOL_HANG_A_fekUzem: f32,
    pub VOL_HANG_A_fekOld: f32,
    pub VOL_HANG_B_fekUzem: f32,
    pub VOL_HANG_B_fekOld: f32,


    // --- MISC: LOTUS VARS ------------------------------------------------ //
    pub invradius_0_0: f32,
    pub invradius_0_1: f32,
    pub a_ground: f32,
    pub a_ground_PREV: f32,
    pub v_ground: f32,
    pub v_ground_PREV: f32,
    pub invradius_0_0_PREV: f32,
    pub invradius_0_1_PREV: f32,
    pub v_axle_abs: f32,

    
    // --- MISC: ELECTRONICS ------------------------------------------------ //
    pub MAX_VONTATAS: f32,
    pub MOTOR_ARAM: f32,
    pub MOTOR_VONOERO: f32,
    pub MOTOR_n_abs: f32,

    pub KONST_VONOERO: f32,
    pub KONST_FORDULAT: f32,
    pub KONST_VFEK: f32,

    pub FEKGERJEDES_T: f32,
    pub FEKERO_VILLANY_GERJEDT: f32,
    pub FEKERO_VILLANY: f32,
    pub FEKERO_LEGFEK: f32,
    pub FEKERO_KEZIFEK: f32,
    
}

impl t_vars {

    pub fn new() -> t_vars {
        t_vars {
                    // --- COCKPIT A ------------------------------------------------ //
                    A_CP_iranyvaltoBent: false,
                    A_CP_menetkapcsoloBent: false,
                    A_CP_fekszelepBent: false,

                    A_CP_iranyvalto: 0.0,
                    A_CP_menetkapcsolo: 0.0,
                    A_CP_fekszelep: 0.0,
                    A_CP_kezifek: 0.0,
                    A_CP_sinfek: 0.0,
                    A_CP_tularamKapcsolo: 0.0,
                    A_CP_kotelCsomo: false,
                    A_CP_kotelNemCsomo: false,

                    A_CP_csengo: 0.0,
                    A_CP_homokolo: 0.0,
                    A_CP_ajtoBalKapcsolo: 0.0,
                    A_CP_ajtoJobbKapcsolo: 0.0,
                    A_CP_sugarvetoKapcsolo: 0.0,
                    A_CP_legsuritoKapcsolo: 0.0,
                    A_CP_berregoBe: 0.0,
                    A_CP_berregoKi: 0.0,

                    A_CP_szamlalo1: 0.0,
                    A_CP_jottThrottle: 0.0,

                    // --- //

                    fulke_A_aktiv: false,

                    fulke_A_szamlalo1: 0.0,
                    fulke_A_jottThrottle: 0.0,
                    fulke_A_kezifekCel: 0.0,
                    menetfokozatElozo_A: 0.0,
                    fek_A_cel: 0.0,
                    fek_A_mozog: false,

                    fulke_A_iranyvaltoBent: false,
                    fulke_A_menetkapcsoloBent: false,
                    fulke_A_fekszelepBent: false,

                    fulke_A_menetkapcsolo: tTobbAllasuKapcsolo::new(),
                    fulke_A_iranyvalto: tTobbAllasuKapcsolo::new(),
                    fulke_A_fekszelep: tTobbAllasuKapcsolo::new(),
                    fulke_A_kezifek: tTobbAllasuKapcsolo::new(),  

                    fulke_A_sinfek: tKapcsolo::new(),
                    fulke_A_tularamvedo: tKapcsolo::new(),
                    fulke_A_kotelCsomo: false,
                    fulke_A_kotelNemCsomo: false,

                    fulke_A_ajtoBal: tKapcsolo::new(),
                    fulke_A_ajtoJobb: tKapcsolo::new(),
                    fulke_A_csengo: tKapcsolo::new(),
                    fulke_A_sugarveto: tKapcsolo::new(),
                    fulke_A_legsurito: tKapcsolo::new(),
                    fulke_A_indjelzoBe: tKapcsolo::new(),
                    fulke_A_indjelzoKi: tKapcsolo::new(),
                    fulke_A_homokolo: tKapcsolo::new(),

                    // --- COCKPIT B ------------------------------------------------ //
                    B_CP_iranyvaltoBent: false,
                    B_CP_menetkapcsoloBent: false,
                    B_CP_fekszelepBent: false,

                    B_CP_iranyvalto: 0.0,
                    B_CP_menetkapcsolo: 0.0,
                    B_CP_fekszelep: 0.0,
                    B_CP_kezifek: 0.0,
                    B_CP_sinfek: 0.0,
                    B_CP_tularamKapcsolo: 0.0,
                    B_CP_kotelCsomo: false,
                    B_CP_kotelNemCsomo: false,

                    B_CP_csengo: 0.0,
                    B_CP_homokolo: 0.0,
                    B_CP_ajtoBalKapcsolo: 0.0,
                    B_CP_ajtoJobbKapcsolo: 0.0,
                    B_CP_sugarvetoKapcsolo: 0.0,
                    B_CP_legsuritoKapcsolo: 0.0,
                    B_CP_berregoBe: 0.0,
                    B_CP_berregoKi: 0.0,

                    B_CP_szamlalo1: 0.0,
                    B_CP_jottThrottle: 0.0,

                    // --- //

                    fulke_B_aktiv: false,

                    fulke_B_szamlalo1: 0.0,
                    fulke_B_jottThrottle: 0.0,
                    fulke_B_kezifekCel: 0.0,
                    menetfokozatElozo_B: 0.0,
                    fek_B_cel: 0.0,
                    fek_B_mozog: false,

                    fulke_B_iranyvaltoBent: false,
                    fulke_B_menetkapcsoloBent: false,
                    fulke_B_fekszelepBent: false,

                    fulke_B_menetkapcsolo: tTobbAllasuKapcsolo::new(),
                    fulke_B_iranyvalto: tTobbAllasuKapcsolo::new(),
                    fulke_B_fekszelep: tTobbAllasuKapcsolo::new(),
                    fulke_B_kezifek: tTobbAllasuKapcsolo::new(),  

                    fulke_B_sinfek: tKapcsolo::new(),
                    fulke_B_tularamvedo: tKapcsolo::new(),
                    fulke_B_kotelCsomo: false,
                    fulke_B_kotelNemCsomo: false,

                    fulke_B_ajtoBal: tKapcsolo::new(),
                    fulke_B_ajtoJobb: tKapcsolo::new(),
                    fulke_B_csengo: tKapcsolo::new(),
                    fulke_B_sugarveto: tKapcsolo::new(),
                    fulke_B_legsurito: tKapcsolo::new(),
                    fulke_B_indjelzoBe: tKapcsolo::new(),
                    fulke_B_indjelzoKi: tKapcsolo::new(),
                    fulke_B_homokolo: tKapcsolo::new(),

                    
                    // --- PASSENGER ------------------------------------------------ //
                    utas_berrego: false,
                    utas_vilagitasKapcsolo1: 0.0,
                    utas_vilagitasKapcsolo2: 0.0,
                    utas_vilagitasKapcsolo3: 0.0,

                    utas_ablak_B2: 0.0,
                    utas_ablak_B3: 0.0,
                    utas_ablak_B4: 0.0,
                    utas_ablak_J2: 0.0,
                    utas_ablak_J3: 0.0,
                    utas_ablak_J4: 0.0,

                    utas_ajto_B1: 0.0,
                    utas_ajto_B2: 0.0,
                    utas_ajto_J1: 0.0,
                    utas_ajto_J2: 0.0,

                    kapaszkodo_X_szog: 0.0,
                    kapaszkodo_X_szogseb: 0.0,
                    kapaszkodo_Y_szog: 0.0,
                    kapaszkodo_Y_szogseb: 0.0,

                    PHY_gyorsulas: 0.0,
                    PHY_oldalgyorsulas: 0.0,

                    ablakB2_cel: 0,
                    ablakB3_cel: 0,
                    ablakB4_cel: 0,
                    ablakJ2_cel: 0,
                    ablakJ3_cel: 0,
                    ablakJ4_cel: 0,

                    K1: tKapcsolo::new(),
                    K2: tKapcsolo::new(),
                    K3: tKapcsolo::new(),


                    // --- MISC: VARS ------------------------------------------------ //
                    ALL_szedo_szog: 0.0,
                    ALL_szedo_szog_ABS: 0.0,
                    ALL_szedo: false,
                    ALL_szedo_irany: false,
                    ALL_tularamvedo: false,
                    SZEDO_ALLAS: 0.0,
                    SZEDO_SZOG_MAX: 0.0,

                    ALL_sinfek: false,
                    ALL_kezifek: 0.0,
                    ALL_menetkapcsolo: 0.0,
                    ALL_iranyvalto: 0.0,
                    ALL_biztonsagi_fek: false,

                    AJTOJOBB_CEL: false,
                    AJTOBAL_CEL: false,
                    AJTO_J1_SEB: 0.0,
                    AJTO_J2_SEB: 0.0,
                    AJTO_B1_SEB: 0.0,
                    AJTO_B2_SEB: 0.0,
                    AJTO_J1_ROGZ: false,
                    AJTO_J2_ROGZ: false,
                    AJTO_B1_ROGZ: false,
                    AJTO_B2_ROGZ: false,

                    ALL_fekhengerNyomas: 0.0,
                    ALL_fektartalyNyomas: 0.0,
                    VanHalozat: false,
                    
                    legsuritoJar: false,
                    RUGO_UT: 0.0,
                    VAR_PHY_lassulas: 0.0,
                    


                    // --- MISC: SOUNDS ------------------------------------------------ //
                    HANG_motorhangero: 0.0,
                    VOL_HANG_rantas_iv: 0.0,

                    VOL_HANG_A_fekUzem: 0.0,
                    VOL_HANG_A_fekOld: 0.0,
                    VOL_HANG_B_fekUzem: 0.0,
                    VOL_HANG_B_fekOld: 0.0,


                    // --- MISC: LOTUS VARS ------------------------------------------------ //
                    invradius_0_0: 0.0,
                    invradius_0_1: 0.0,
                    a_ground: 0.0,
                    a_ground_PREV: 0.0,
                    invradius_0_0_PREV: 0.0,
                    invradius_0_1_PREV: 0.0,
                    v_axle_abs: 0.0,
                    v_ground: 0.0,
                    v_ground_PREV: 0.0,



                    
                    // --- MISC: ELECTRONICS ------------------------------------------------ //
                    MAX_VONTATAS: 0.0,
                    MOTOR_ARAM: 0.0,
                    MOTOR_VONOERO: 0.0,
                    MOTOR_n_abs: 0.0,

                    KONST_VONOERO: 0.0,
                    KONST_FORDULAT: 0.0,
                    KONST_VFEK: 0.0,

                    FEKGERJEDES_T: 0.0,
                    FEKERO_VILLANY_GERJEDT: 0.0,
                    FEKERO_VILLANY: 0.0,
                    FEKERO_LEGFEK: 0.0,
                    FEKERO_KEZIFEK: 0.0,
        }
    }

}


// --- ALL VARIABLES --------------------------------------------------------------------------------- //
#[derive(Default)]
pub struct Vars {
    pub vars: t_vars,
}

impl Vars {
    pub fn new() -> Self {
        Self {
            vars: t_vars::new(),
        }
    }

    pub fn vars(&mut self) -> &mut t_vars {
        &mut self.vars
    }
}


// --- OTHER VARS ------------------------------------------------------------------------------------ //
