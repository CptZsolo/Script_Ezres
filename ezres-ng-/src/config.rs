#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, nonstandard_style, dead_code, unused)]


//MENETFOKOZATOKHOZ AZ ÉRTÉKEK (FOKBAN)
pub const CFG_MENETKAPCSOLO_f7: f32 = -118.0;
pub const CFG_MENETKAPCSOLO_f6: f32 = -105.0;
pub const CFG_MENETKAPCSOLO_f5: f32 = -90.0;
pub const CFG_MENETKAPCSOLO_f4: f32 = -70.0;
pub const CFG_MENETKAPCSOLO_f3: f32 = -55.0;
pub const CFG_MENETKAPCSOLO_f2: f32 = -40.0;
pub const CFG_MENETKAPCSOLO_f1: f32 = -20.0;
pub const CFG_MENETKAPCSOLO_0: f32 = 0.0;
pub const CFG_MENETKAPCSOLO_m1: f32 = 20.0;
pub const CFG_MENETKAPCSOLO_m2: f32 = 40.0;
pub const CFG_MENETKAPCSOLO_m3: f32 = 60.0;
pub const CFG_MENETKAPCSOLO_m4: f32 = 80.0;
pub const CFG_MENETKAPCSOLO_m5: f32 = 100.0;
pub const CFG_MENETKAPCSOLO_m6: f32 = 120.0;
pub const CFG_MENETKAPCSOLO_m7: f32 = 140.0;
pub const CFG_MENETKAPCSOLO_m8: f32 = 160.0;
pub const CFG_MENETKAPCSOLO_m9: f32 = 180.0;
pub const CFG_MENETKAPCSOLO_m10: f32 = 200.0;
pub const CFG_MENETKAPCSOLO_m11: f32 = 228.0;

//IRÁNYVÁLTÓ ÁLLÁSAI
pub const CFG_IRANYVALTO_ELORE: f32 = 70.0;
pub const CFG_IRANYVALTO_HATRA: f32 = -70.0;


//FÉKRENDSZER ÁRAMLÁSOK (kPa/s)
pub const CFG_FEKBE_MAX: f32 = 170.0; // VÉSZFÉK
pub const CFG_FEKKI_SEB: f32 = 170.0; // KIENGEDÉS


//ABLAK SEBESSÉGEK ( %/s )
pub const CFG_ABLAK_FEL_SEB: f32 = 0.05;
pub const CFG_ABLAK_LE_SEB: f32 = 0.03;


pub const CFG_SZEDO_SEB: f32 = 8.0;       // fok/m�sodperc
pub const CFG_KONST_MAX_VONTATAS: f32 = 4500.0;
pub const CFG_MAX_ARAM: f32 = 300.0;
pub const CFG_MAX_VFEKERO: f32 = 12000.0;   //N-ban, villamosf?k
pub const CFG_MAX_LEGFEK: f32 = 18000.0;   //N-ban, l?gf?k
pub const CFG_MAX_KEZIFEK: f32 = 6000.0;   //N-ban, k?zif?k
pub const CFG_MAX_SINFEK: f32 = 85000.0;   //N-ban, s?nf?k

pub const CFG_const_FEKGERJEDES_T: f32 = 0.8;        //ENNYI IDEIG TEHETETLEN A VFÉK
pub const CFG_const_FEKGERJEDES_SEB: f32 = 6000.0;    // 15kN/s SEBESS?GGEL ?P?L FEL A VF?K F?KEREJE


pub const CFG_RUGOALLANDO: f32 = 250000.0; //N-ban a rug??lland? (laprug?k)

//AJT?K
pub const CFG_AJTO_J1_VMAX: f32 = 0.6;
pub const CFG_AJTO_J2_VMAX: f32 = 0.7;
pub const CFG_AJTO_B1_VMAX: f32 = 0.5;
pub const CFG_AJTO_B2_VMAX: f32 = 0.65;
pub const CFG_AJTO_P_TORESPONT: f32 = 300.0; //kPa-ban, hol kezd el lassulni az ajt?m?k?d?s

//PHY a kapaszkodokhoz
pub const CFG_VILLAMOS_TOMEG: f32 = 17500.0; //kg-ban
pub const CFG_KAPASZKODO_const1: f32 = 8.0;
pub const CFG_KAPASZKODO_const2: f32 = 8.0;
pub const CFG_KAPASZKODO_const3: f32 = 2.0;
