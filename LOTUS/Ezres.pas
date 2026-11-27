


{PUBLIC_VARS
  InitReadyForMovement: integer;
  ALL_szedo_szog: single;
  ALL_szedo_szog_ABS: single;
  ALL_szedo: boolean;
  ALL_szedo_irany: boolean;
  ALL_tularamvedo: boolean;
  ALL_sinfek: boolean;
  ALL_kezifek: single;
  ALL_menetkapcsolo: single;
  ALL_iranyvalto: single;
  ALL_biztonsagi_fek: boolean;
  
  MAX_VONTATAS: integer;
  AJTOJOBB_CEL: boolean;
  AJTOBAL_CEL: boolean;
  
  ALL_fekhengerNyomas: single;
  ALL_fektartalyNyomas: single;
  VanHalozat: boolean;

  Timegap: single;
  Mouse_Y: single;
  DebugVar: single;
  DebugVarBool: boolean;
  Hint: string;
  invradius_abs_max: single;
  invradius_0_0: single;
  invradius_0_1: single;
  RC_Active: boolean;
  RC_DirLeaver: integer;
  RC_Throttle: single;
  RC_Brake: single;
  RC_Belling: integer;
  RC_sanding: boolean;
  RC_FrontLight_Front: integer;
  RC_FrontLight_Back: integer;
  RC_OtherLights_Front: integer;
  RC_OtherLights_Back: integer;
  RC_CabinLight: integer;
  RC_DoorsOpen_Left: integer;
  RC_DoorsOpen_Right: integer;
  RC_PantographApplied: integer;
  
  Pantograph_Pos: single;
  panto_0: single;
  panto_voltage_0: boolean;
  RealisticElecSupply: boolean;
  
  coupled_0: boolean;
  coupled_1: boolean;
  couplingState_0: integer;
  couplingState_1: integer;
  
  M_Axle_N_0_0: single; 
  M_Axle_N_0_1: single; 
  MBrake_Axle_N_0_0: single;  
  MBrake_Axle_N_0_1: single;
  F_RailBrake_Bogie_N_0: single; 
  loadforce_Axle_N_0_0: single;
  loadforce_Axle_N_0_1: single;
  kasztni_bolintas: single;
  
  KAMERA_X: single;
  KAMERA_Y: single;
  Function_KAMERA_X: integer;

  sanding_0_0: boolean;
  sanding_0_1: boolean;
  v_Axle_mps_0_0: single;
  v_Axle_mps_0_1: single;
  v_axle_abs: single;
  v_ground: single;
  a_ground: single;
  v_axle_kmph: single;
  railquality_0_0: integer;
  
  DoorOpen_0: boolean;
	DoorOpen_1: boolean;
	DoorOpen_2: boolean;
	DoorOpen_3: boolean;
	
	mirrortex_0: integer;
	mirrortex_1: integer;
	mirrortex_2: integer;
	mirrortex_3: integer;
  
  railbrake_pos: single;

  
  A_CP_iranyvaltoBent: boolean;
  A_CP_menetkapcsoloBent: boolean;
  A_CP_fekszelepBent: boolean;
  
  A_CP_iranyvalto: single;
  A_CP_menetkapcsolo: single;
  A_CP_fekszelep: single;
  A_CP_kezifek: single;
  A_CP_sinfek: single;
  A_CP_tularamKapcsolo: single;
  A_CP_kotelCsomo: boolean;
  A_CP_kotelNemCsomo: boolean;
  
  A_CP_csengo: single;
  A_CP_homokolo: single;
  A_CP_ajtoBalKapcsolo: single;
  A_CP_ajtoJobbKapcsolo: single;
  A_CP_sugarvetoKapcsolo: single;
  A_CP_legsuritoKapcsolo: single;
  A_CP_berregoBe: single;
  A_CP_berregoKi: single;

  B_CP_iranyvaltoBent: boolean;
  B_CP_menetkapcsoloBent: boolean;
  B_CP_fekszelepBent: boolean;
  
  B_CP_iranyvalto: single;
  B_CP_menetkapcsolo: single;
  B_CP_fekszelep: single;
  B_CP_kezifek: single;
  B_CP_sinfek: single;
  B_CP_tularamKapcsolo: single;
  B_CP_kotelCsomo: boolean;
  B_CP_kotelNemCsomo: boolean;
  
  B_CP_csengo: single;
  B_CP_homokolo: single;
  B_CP_ajtoBalKapcsolo: single;
  B_CP_ajtoJobbKapcsolo: single;
  B_CP_sugarvetoKapcsolo: single;
  B_CP_berregoBe: single;
  B_CP_berregoKi: single;

  utas_berrego: boolean;
  utas_vilagitasKapcsolo1: single;
  utas_vilagitasKapcsolo2: single;
  utas_vilagitasKapcsolo3: single;
  
  utas_ablak_B2: single;
  utas_ablak_B3: single;
  utas_ablak_B4: single;
  utas_ablak_J2: single;
  utas_ablak_J3: single;
  utas_ablak_J4: single;
  
  utas_ajto_B1: single;
  utas_ajto_B2: single;
  utas_ajto_J1: single;
  utas_ajto_J2: single;

  LM_utas_berrego: single;
  LM_A_helyzetjelzo: single;
  LM_A_tompitott: single;
  LM_A_zarfeny: single;
  LM_B_helyzetjelzo: single;
  LM_B_tompitott: single;
  LM_B_zarfeny: single;
  LM_utaster: single;
  LM_utaster_A_peron: single;
  LM_utaster_B_peron: single;
  
  csatlas_vilagitasCsatlakozo600V_front: boolean;
  csatlas_vilagitasCsatlakozo600V_back: boolean;
  csatlas_fekhengerNyomas_front: single;
  csatlas_fekhengerNyomas_back: single;
  csatlas_berrego_front: boolean;
  csatlas_berrego_back: boolean;
  
  
  HANG_ures: integer;
  HANG_A_iranyvaltoBe: integer;
  HANG_A_iranyvaltoKi: integer;
  HANG_A_menetkapcsoloBe: integer;
  HANG_A_menetkapcsoloKi: integer;
  HANG_A_fekszelepBe: integer;
  HANG_A_fekszelepKi: integer;
  HANG_A_iranyvaltoElore: integer;
  HANG_A_iranyvaltoHatra: integer;
  HANG_A_menetkapcsolo_menet: integer;
  HANG_A_menetkapcsolo_menetLe: integer;
  HANG_A_menetkapcsolo_menetMax: integer;
  HANG_A_menetkapcsolo_fek: integer;
  HANG_A_menetkapcsolo_fekLe: integer;
  HANG_A_menetkapcsolo_fekMax: integer;
  
  HANG_A_fekPlusz: integer;
  HANG_A_veszfek: integer;
  HANG_A_fekOld: integer;
  HANG_A_kapcsolo1: integer;
  HANG_A_kapcsolo2: integer;
  HANG_A_kapcsolo3: integer;
  HANG_A_csengo: integer;
  HANG_A_csengoX: integer;
  HANG_A_homokBE: integer;
  HANG_A_homokKI: integer;
  HANG_A_sugarBE: integer;
  HANG_A_sugarKI: integer;
  HANG_A_indjelzBE: integer;
  HANG_A_indjelzKI: integer;
  HANG_A_ajtoszelepBE: integer;
  HANG_A_ajtoszelepKI: integer;
  HANG_A_kezifek: integer;
  
  HANG_B_iranyvaltoBe: integer;
  HANG_B_iranyvaltoKi: integer;
  HANG_B_menetkapcsoloBe: integer;
  HANG_B_menetkapcsoloKi: integer;
  HANG_B_fekszelepBe: integer;
  HANG_B_fekszelepKi: integer;
  HANG_B_iranyvaltoElore: integer;
  HANG_B_iranyvaltoHatra: integer;
  HANG_B_menetkapcsolo_menet: integer;
  HANG_B_menetkapcsolo_menetLe: integer;
  HANG_B_menetkapcsolo_menetMax: integer;
  HANG_B_menetkapcsolo_fek: integer;
  HANG_B_menetkapcsolo_fekLe: integer;
  HANG_B_menetkapcsolo_fekMax: integer;
  
  HANG_B_fekPlusz: integer;
  HANG_B_veszfek: integer;
  HANG_B_fekOld: integer;
  HANG_B_kapcsolo1: integer;
  HANG_B_kapcsolo2: integer;
  HANG_B_kapcsolo3: integer;
  HANG_B_csengo: integer;
  HANG_B_csengoX: integer;
  HANG_B_homokBE: integer;
  HANG_B_homokKI: integer;
  HANG_B_sugarBE: integer;
  HANG_B_sugarKI: integer;
  HANG_B_indjelzBE: integer;
  HANG_B_indjelzKI: integer;
  HANG_B_ajtoszelepBE: integer;
  HANG_B_ajtoszelepKI: integer;
  HANG_B_kezifek: integer;
  
  HANG_sinfekLe: integer;
  HANG_sinfekFel: integer;
  HANG_inditasjelzo: integer;
  HANG_K1_BE: integer;
  HANG_K1_KI: integer;
  
  HANG_legsuritoStart: integer;
  HANG_legsuritoStop: integer;
  HANG_legsuritoLoop: integer;
  HANG_legsuritoLoopX: integer;
  
  HANG_kerekcsikorgas: integer;
  HANG_kerekcsattogas: integer;
  HANG_futas: integer;
  HANG_futasX: integer;
  HANG_motor: integer;
  HANG_motorhangero: single;
  HANG_rantas_start: integer;
  HANG_rantas_stop: integer;
  HANG_rantas_iv: integer;
  VOL_HANG_rantas_iv: single;
  
  HANG_ajtoJ1_loop: integer;
  HANG_ajtoJ1_start: integer;
  HANG_ajtoJ1_stop: integer;
  HANG_ajtoJ2_loop: integer;
  HANG_ajtoJ2_start: integer;
  HANG_ajtoJ2_stop: integer;
  HANG_ajtoB1_loop: integer;
  HANG_ajtoB1_start: integer;
  HANG_ajtoB1_stop: integer;
  HANG_ajtoB2_loop: integer;
  HANG_ajtoB2_start: integer;
  HANG_ajtoB2_stop: integer;
  
  HANG_ablakBAL_FEL: integer;
  HANG_ablakBAL_LE: integer;
  HANG_ablakJOBB_FEL: integer;
  HANG_ablakJOBB_LE: integer;
  
  A_CP_szamlalo1: single;
  A_CP_jottThrottle: single;
  
  HANG_A_fekUzem: integer;
  HANG_A_fekOld: integer;
  VOL_HANG_A_fekUzem: single;
  VOL_HANG_A_fekOld: single;
  
  HANG_B_fekUzem: integer;
  HANG_B_fekOld: integer;
  VOL_HANG_B_fekUzem: single;
  VOL_HANG_B_fekOld: single;
  
  
  HANG_A_fekszelep_plusz: integer;
  HANG_A_fekszelep_minusz: integer;
  HANG_B_fekszelep_plusz: integer;
  HANG_B_fekszelep_minusz: integer;
  HANG_A_sinfekkapcsolo_be: integer;
  HANG_A_sinfekkapcsolo_ki: integer;
  HANG_B_sinfekkapcsolo_be: integer;
  HANG_B_sinfekkapcsolo_ki: integer;
  HANG_sinfekBe_kint: integer;
  HANG_sinfekKi_kint: integer;
  HANG_sinfekBe_bent: integer;
  HANG_sinfekKi_bent: integer;
  HANG_fekrudazat_be: integer;
  HANG_fekrudazat_ki: integer;
  
  
  kapaszkodo_X_szog: single;
  kapaszkodo_X_szogseb: single;
  kapaszkodo_Y_szog: single;
  kapaszkodo_Y_szogseb: single;
  
  PHY_gyorsulas: single;
  PHY_oldalgyorsulas: single;
  
  KAMERA_BOLINT: single;
  KAMERA_OLDAL: single;
  Function_KAMERA_BOLINT: integer;
  Function_KAMERA_OLDAL: integer;

  a_ground_PREV: single;
  invradius_abs_max_PREV: single;
  invradius_0_0_PREV: single;
  invradius_0_1_PREV: single;

  MOTOR_ARAM: single;
  MOTOR_VONOERO: single;
  MOTOR_n_abs: single;

  KONST_VONOERO: single;
  KONST_FORDULAT: single;
  KONST_VFEK: single;

  FEKGERJEDES_T: single;
  FEKERO_VILLANY_GERJEDT: single;
}









{PUBLIC_BUTTONS
  
  Throttle;
  Neutral;
  Brake;
  MaxBrake;
  ReverserPlus;
  ReverserMinus;
  BKV1000_fekszelepMinusz;
  BKV1000_fekszelepPlusz;
  BKV1000_fekszelepVeszfek;
  BKV1000_A_CP_iranyvaltoBe;
  BKV1000_A_CP_fekszelepBe;
  BKV1000_A_CP_menetkapcsoloBe;
  BKV1000_A_CP_tularamvedo;
  RailBrake;
  Sanding;
  Bell1;
  
  PantographUp;
  PantographDn;         

  FrontLightPlus;
  FrontLightMinus;
  CabinLightToggle;
  
  BKV1000_ajtoBalKapcsolo;
  BKV1000_ajtoJobbKapcsolo;
  BKV1000_inditasjelzo;
  BKV1000_inditasjelzoKi;
  BKV1000_szedoAtforditas;
  
  

  
  BKV1000_A_CP_iranyvaltoKi;
  BKV1000_A_CP_iranyvaltoElore;
  BKV1000_A_CP_iranyvaltoHatra;
  BKV1000_A_CP_fekszelepKi;
  BKV1000_A_CP_menetkapcsoloKi;
  BKV1000_A_CP_kezifekPlusz;
  BKV1000_A_CP_kezifekMinusz;
  BKV1000_A_CP_sinfek;
  BKV1000_A_CP_kotel;
  BKV1000_A_CP_homokolo;
  BKV1000_A_CP_ajtoBalKapcsolo;
  BKV1000_A_CP_ajtoJobbKapcsolo;
  BKV1000_A_CP_inditasjelzoBe;
  BKV1000_A_CP_inditasjelzoKi;
  BKV1000_A_CP_csengo;
  BKV1000_A_CP_sugarvetoKapcsolo;
  BKV1000_A_CP_legsuritoKapcsolo;
  
  BKV1000_B_CP_iranyvaltoBe;
  BKV1000_B_CP_iranyvaltoKi;
  BKV1000_B_CP_iranyvaltoElore;
  BKV1000_B_CP_iranyvaltoHatra;
  BKV1000_B_CP_fekszelepBe;
  BKV1000_B_CP_fekszelepKi;
  BKV1000_B_CP_menetkapcsoloBe;
  BKV1000_B_CP_menetkapcsoloKi;
  BKV1000_B_CP_kezifekPlusz;
  BKV1000_B_CP_kezifekMinusz;
  BKV1000_B_CP_sinfek;
  BKV1000_B_CP_tularamvedo;
  BKV1000_B_CP_kotel;
  BKV1000_B_CP_homokolo;
  BKV1000_B_CP_ajtoBalKapcsolo;
  BKV1000_B_CP_ajtoJobbKapcsolo;
  BKV1000_B_CP_inditasjelzoBe;
  BKV1000_B_CP_inditasjelzoKi;
  BKV1000_B_CP_csengo;
  BKV1000_B_CP_sugarvetoKapcsolo;
  
  BKV1000_ablaktekero_B2_Fel;
  BKV1000_ablaktekero_B2_Le;
  BKV1000_ablaktekero_B3_Fel;
  BKV1000_ablaktekero_B3_Le;
  BKV1000_ablaktekero_B4_Fel;
  BKV1000_ablaktekero_B4_Le;
  BKV1000_ablaktekero_J2_Fel;
  BKV1000_ablaktekero_J2_Le;
  BKV1000_ablaktekero_J3_Fel;
  BKV1000_ablaktekero_J3_Le;
  BKV1000_ablaktekero_J4_Fel;
  BKV1000_ablaktekero_J4_Le;
  BKV1000_vilagitas_K1;
  BKV1000_vilagitas_K2;
  BKV1000_vilagitas_K3;
  
  
  
  BKV1000_csatlasKiElol;
  BKV1000_csatlasKiHatul;
  

}

{PUBLIC_AXIS
  Throttle; 
  Brake;
  Airbrake;
  Reverser;
}





{$I StdElements.pas}
{$I Ezres_Fulke.pas}
{$I Ezres_Elektronika.pas}
{$I Ezres_RC.pas}



  





procedure SimStep;
begin

  //--------------------------------------
  // ÁLTALÁNOS
  fulke_SimStep;
  elektronika_SimStep;
  
  RC_kuldes;

  v_axle_abs := abs(v_Axle_mps_0_0);
  v_axle_kmph := v_axle_abs * 3.6;
  
  if (not coupled_0) then couplingState_0 := 1;
  if (not coupled_1) then couplingState_1 := 1;
  
  
  
  //--------------------------------------
  // ÁLTALÁNOS HANGOK
  if (v_Axle_mps_0_0 <> 0) then
  begin
    HANG_kerekcsikorgas := 1;
    HANG_futas := 1;
    HANG_futasX := -1;
  end;
  
  if (v_Axle_mps_0_0 = 0) then 
  begin
    HANG_kerekcsikorgas := -1;
    HANG_futas := -1;
    HANG_futasX := -1;
  end;


  //--------------------------------------
  // MOTOR HANGOK
  if (not(M_Axle_N_0_0 = 0) or (FEKERO_VILLANY > 0)) and (v_axle_abs > 0.4) then
  begin
    HANG_motor := 1;
  end
  else
  begin
    HANG_motor := -1;
  end;
  
  if (FEKERO_VILLANY > 0) then  HANG_motorhangero := 0.5 * max(0.25, (3*FEKERO_VILLANY/MAX_VFEKERO))
  else if (M_Axle_N_0_0 > 0) then HANG_motorhangero := 0.5 * max(0.25 , (abs(M_Axle_N_0_0)/MAX_VONTATAS) )
  else HANG_motorhangero := 0;

  
  //--------------------------------------
  // KERÉK CSATTOGÁS
  if (railquality_0_0 = 2) or (railquality_0_0 = 3) then
  begin
    HANG_kerekcsattogas := 1
  end
  else
    HANG_kerekcsattogas := -1;
    
  //--------------------------------------
  // EXTRA HANGOK

  // RÁNTÁS
  if (abs(a_ground - a_ground_PREV)) > 0.35 then HANG_rantas_start := 1;
  if (abs(a_ground - a_ground_PREV)) < -0.8 then HANG_rantas_stop := 1;

  // ÍVBEN KATTANÁS/dobbanás
  // if (fulke_A.aktiv) then VOL_HANG_rantas_iv := abs(invradius_0_0_PREV - invradius_0_0)
  // else if (fulke_B.aktiv) then VOL_HANG_rantas_iv := abs(invradius_0_1_PREV - invradius_0_1);
  if (fulke_A.aktiv) then VOL_HANG_rantas_iv := abs(invradius_0_0)
  else if (fulke_B.aktiv) then VOL_HANG_rantas_iv := abs(invradius_0_1);
  if ((abs(invradius_0_0_PREV - invradius_0_0) > 0.001) and fulke_A.aktiv) or ((abs(invradius_0_1_PREV - invradius_0_1) > 0.001) and fulke_B.aktiv) then HANG_rantas_iv := 1;
    
  //--------------------------------------
  // KAMERA MOZGÁSOK
  KAMERA_BOLINT := LinFuncGetValue(Function_KAMERA_BOLINT, PHY_gyorsulas);
  KAMERA_OLDAL := LinFuncGetValue(Function_KAMERA_OLDAL, PHY_oldalgyorsulas);

  KAMERA_BOLINT := LinFuncGetValue(Function_KAMERA_BOLINT, PHY_gyorsulas);
  KAMERA_X := KAMERA_OLDAL + kam_pattog_X_0.ertek + kam_pattog_X_1.ertek + kam_pattog_X_2.ertek;
     

  //--------------------------------------
  // VÁLTOZÓ MÁSOLÁSOK
  a_ground_PREV := a_ground;
  invradius_abs_max_PREV := invradius_abs_max;
  invradius_0_0_PREV := invradius_0_0;
  invradius_0_1_PREV := invradius_0_1;
end;





procedure Initialize;
begin
  fulke_init;
  elektronika_init;
  
  // VÁLTOZÓK NULLÁZÁSA
  kasztni_bolintas := 0;
  KAMERA_X := 0;
  KAMERA_Y := 0;
  sanding_0_0 := false;
  sanding_0_1 := false;
  
  
  // HANGOK NULLÁZÁSA
  HANG_ures := -1;
  
  VOL_HANG_A_fekUzem := 0;
  VOL_HANG_A_fekOld := 0;
  VOL_HANG_B_fekUzem := 0;
  VOL_HANG_B_fekOld := 0;
  HANG_A_fekUzem := -1;
  HANG_A_fekOld := -1; 
  HANG_B_fekUzem := -1;
  HANG_B_fekOld := -1;
  
  HANG_A_fekszelep_plusz := -1;
  HANG_A_fekszelep_minusz := -1;
  HANG_B_fekszelep_plusz := -1;
  HANG_B_fekszelep_minusz := -1;
    
  HANG_A_iranyvaltoBe := -1;
  HANG_A_iranyvaltoKi := -1;
  HANG_A_menetkapcsoloBe := -1;
  HANG_A_menetkapcsoloKi := -1;
  HANG_A_fekszelepBe := -1;
  HANG_A_fekszelepKi := -1;
  HANG_A_iranyvaltoElore := -1;
  HANG_A_iranyvaltoHatra := -1;
  HANG_A_menetkapcsolo_menet := -1;
  HANG_A_menetkapcsolo_menetLe := -1;
  HANG_A_menetkapcsolo_menetMax := -1;
  HANG_A_menetkapcsolo_fek := -1;
  HANG_A_menetkapcsolo_fekLe := -1;
  HANG_A_menetkapcsolo_fekMax := -1;
  
  HANG_A_fekPlusz := -1;
  HANG_A_veszfek := -1;
  HANG_A_fekOld := -1;
  HANG_A_kapcsolo1 := -1;
  HANG_A_kapcsolo2 := -1;
  HANG_A_kapcsolo3 := -1;
  HANG_A_csengo := -1;
  HANG_A_csengoX := -1;
  HANG_A_homokBE := -1;
  HANG_A_homokKI := -1;
  HANG_A_sugarBE := -1;
  HANG_A_sugarKI := -1;
  HANG_A_indjelzBE := -1;
  HANG_A_indjelzKI := -1;
  HANG_A_ajtoszelepBE := -1;
  HANG_A_ajtoszelepKI := -1;
  HANG_A_kezifek := -1;
  
  HANG_B_iranyvaltoBe := -1;
  HANG_B_iranyvaltoKi := -1;
  HANG_B_menetkapcsoloBe := -1;
  HANG_B_menetkapcsoloKi := -1;
  HANG_B_fekszelepBe := -1;
  HANG_B_fekszelepKi := -1;
  HANG_B_iranyvaltoElore := -1;
  HANG_B_iranyvaltoHatra := -1;
  HANG_B_menetkapcsolo_menet := -1;
  HANG_B_menetkapcsolo_menetLe := -1;
  HANG_B_menetkapcsolo_menetMax := -1;
  HANG_B_menetkapcsolo_fek := -1;
  HANG_B_menetkapcsolo_fekLe := -1;
  HANG_B_menetkapcsolo_fekMax := -1;
  
  HANG_B_fekPlusz := -1;
  HANG_B_veszfek := -1;
  HANG_B_fekOld := -1;
  HANG_B_kapcsolo1 := -1;
  HANG_B_kapcsolo2 := -1;
  HANG_B_kapcsolo3 := -1;
  HANG_B_csengo := -1;
  HANG_B_csengoX := -1;
  HANG_B_homokBE := -1;
  HANG_B_homokKI := -1;
  HANG_B_sugarBE := -1;
  HANG_B_sugarKI := -1;
  HANG_B_indjelzBE := -1;
  HANG_B_indjelzKI := -1;
  HANG_B_ajtoszelepBE := -1;
  HANG_B_ajtoszelepKI := -1;
  HANG_B_kezifek := -1;
  
  HANG_sinfekLe := -1;
  HANG_sinfekFel := -1;
  HANG_inditasjelzo := -1;
  HANG_K1_BE := -1;
  HANG_K1_KI := -1;
  
  HANG_legsuritoStart := -1;
  HANG_legsuritoStop := -1;
  HANG_legsuritoLoop := -1;
  HANG_legsuritoLoopX := -1;
  
  HANG_kerekcsikorgas := -1;
  HANG_kerekcsattogas := -1;
  HANG_futas := 1;
  HANG_futasX := -1;
  HANG_motor := 1;
  HANG_motorhangero := -1;
  HANG_rantas_start := -1;
  HANG_rantas_stop := -1;
  HANG_rantas_iv := -1;
  VOL_HANG_rantas_iv := 0;
  
  HANG_ajtoJ1_loop := -1;
  HANG_ajtoJ1_start := -1;
  HANG_ajtoJ1_stop := -1;
  HANG_ajtoJ2_loop := -1;
  HANG_ajtoJ2_start := -1;
  HANG_ajtoJ2_stop := -1;
  HANG_ajtoB1_loop := -1;
  HANG_ajtoB1_start := -1;
  HANG_ajtoB1_stop := -1;
  HANG_ajtoB2_loop := -1;
  HANG_ajtoB2_start := -1;
  HANG_ajtoB2_stop := -1;
  
  HANG_ablakBAL_FEL := -1;
  HANG_ablakBAL_LE := -1;
  HANG_ablakJOBB_FEL := -1;
  HANG_ablakJOBB_LE := -1;
  
  // KAMERA MOZGÁSOK (LIN. FV.)
  Function_KAMERA_BOLINT := LinFuncNew;
  LinFuncAddPair(Function_KAMERA_BOLINT, -4.0, -0.5);
  LinFuncAddPair(Function_KAMERA_BOLINT, -2.0, -0.5);
  LinFuncAddPair(Function_KAMERA_BOLINT, 0.0, 0.0);
  LinFuncAddPair(Function_KAMERA_BOLINT, 2.0, 0.8);
  LinFuncAddPair(Function_KAMERA_BOLINT, 4.0, 0.8);

  Function_KAMERA_OLDAL := LinFuncNew;
  LinFuncAddPair(Function_KAMERA_OLDAL, -4.0, -0.05);
  LinFuncAddPair(Function_KAMERA_OLDAL, -2.0, -0.05);
  LinFuncAddPair(Function_KAMERA_OLDAL, 0.0, 0.0);
  LinFuncAddPair(Function_KAMERA_OLDAL, 2.0, 0.05);
  LinFuncAddPair(Function_KAMERA_OLDAL, 4.0, 0.05);
end; 



procedure SimStep_RC;
begin
	RCscript_simstep;
end;



procedure OnButton(id: string; value: boolean; cockpitIndex: integer);
begin
  fulke_OnButton(id, value, cockpitIndex);
  utaster_OnButton(id, value, cockpitIndex);
  
  if value and (id = 'BKV1000_csatlasKiElol') then couplingState_0 := 0;
  if value and (id = 'BKV1000_csatlasKiHatul') then couplingState_1 := 0;
end;




procedure OnFloatInput(id: string; value: single; cockpitIndex: integer);
begin
  fulke_OnFloatInput(id, value, cockpitIndex);
end;

end.

