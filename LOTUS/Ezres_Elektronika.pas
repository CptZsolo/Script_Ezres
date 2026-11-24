const
  SZEDO_SEB = 8;       // fok/m�sodperc
  KONST_MAX_VONTATAS = 4500;
  MAX_ARAM = 300;
  MAX_VFEKERO = 12000;   //N-ban, villamosf�k
  MAX_LEGFEK = 18000;   //N-ban, l�gf�k
  MAX_KEZIFEK = 6000;   //N-ban, k�zif�k
  MAX_SINFEK = 85000;   //N-ban, s�nf�k
  
  const_FEKGERJEDES_T = 0.8;        //ENNYI IDEIG TEHETETLEN A VFÉK
  const_FEKGERJEDES_SEB = 6000;    // 15kN/s SEBESS�GGEL �P�L FEL A VF�K F�KEREJE

  
  RUGOALLANDO = 250000; //N-ban a rug��lland� (laprug�k)
  
  //AJT�K
  AJTO_J1_VMAX = 0.6;
  AJTO_J2_VMAX = 0.7;
  AJTO_B1_VMAX = 0.5;
  AJTO_B2_VMAX = 0.65;
  AJTO_P_TORESPONT = 300; //kPa-ban, hol kezd el lassulni az ajt�m�k�d�s
  
  //PHY a kapaszkodokhoz
  VILLAMOS_TOMEG = 17500; //kg-ban
  KAPASZKODO_const1 = 8;
  KAPASZKODO_const2 = 8;
  KAPASZKODO_const3 = 2;

var
  FEKERO_VILLANY: single;
  FEKERO_LEGFEK: single;
  FEKERO_KEZIFEK: single;
  
  // FEKGERJEDES_T: single;
  // FEKERO_VILLANY_GERJEDT: single;
  
  RUGO_UT: single;
  
  SZEDO_ALLAS: single;
  SZEDO_SZOG_MAX: single;

  AJTO_J1_SEB: single;
  AJTO_J2_SEB: single;
  AJTO_B1_SEB: single;
  AJTO_B2_SEB: single;
  AJTO_J1_ROGZ: boolean;
  AJTO_J2_ROGZ: boolean;
  AJTO_B1_ROGZ: boolean;
  AJTO_B2_ROGZ: boolean;
  
  VAR_PHY_lassulas: single;
  

  //------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
  
  procedure elektronika_SimStep;
  begin
  
    //SZED� SZIMUL�L�SA - EL�RE
    if (ALL_szedo and ALL_szedo_irany) then
    begin
      SZEDO_ALLAS := ((panto_0)-3.8)/3.15;
      if ((SZEDO_ALLAS > 1) OR (SZEDO_ALLAS < -1)) then SZEDO_ALLAS := 1;
      SZEDO_SZOG_MAX := max(0, ((Arcsin(SZEDO_ALLAS))/PI*180)-4);
      ALL_szedo_szog := Min( SZEDO_SZOG_MAX, ALL_szedo_szog+(Timegap*SZEDO_SEB) );
      
    end;
    if (not ALL_szedo and ALL_szedo_irany) then
    begin
      ALL_szedo_szog := Max(0,ALL_szedo_szog-(Timegap*SZEDO_SEB) );
    end;
    
    
    //-----------------------------------------------------------------------
    //SZED� SZIMUL�L�SA - H�TRA
    if (ALL_szedo and not ALL_szedo_irany) then
    begin
      SZEDO_ALLAS := ((panto_0)-3.8)/3.15;
      if ((SZEDO_ALLAS > 1) OR (SZEDO_ALLAS < -1)) then SZEDO_ALLAS := 1;
      SZEDO_SZOG_MAX := max(0, ((Arcsin(SZEDO_ALLAS))/PI*180)-4);
      ALL_szedo_szog_ABS := Min( SZEDO_SZOG_MAX, ALL_szedo_szog_ABS+(Timegap*SZEDO_SEB) );
      
      ALL_szedo_szog := 180-ALL_szedo_szog_ABS-8;
    end;
    if (not ALL_szedo and not ALL_szedo_irany) then
    begin
      ALL_szedo_szog_ABS := Max(0,ALL_szedo_szog_ABS-(Timegap*SZEDO_SEB) );
      
      ALL_szedo_szog := 180-ALL_szedo_szog_ABS-8;
      
    end;
    
    
    //F�LKE K�TELEK BE�LL�T�SA
    if (ALL_szedo and ALL_szedo_irany) then
    begin
      fulke_A.kotelCsomo := true;
      fulke_B.kotelCsomo := false;
    end;
    if (not ALL_szedo and ALL_szedo_irany) then
    begin
      fulke_A.kotelCsomo := false;
      fulke_B.kotelCsomo := true;
    end;
    if (not ALL_szedo and not ALL_szedo_irany) then
    begin
      fulke_A.kotelCsomo := true;
      fulke_B.kotelCsomo := false;
    end;
    if (ALL_szedo and not ALL_szedo_irany) then
    begin
      fulke_A.kotelCsomo := false;
      fulke_B.kotelCsomo := true;
    end;
    
    
    
    //VAN-E H�L�ZAT
    vanHalozat := (ALL_szedo and panto_voltage_0 and RealisticElecSupply) OR (ALL_szedo and not RealisticElecSupply);
    
    
    //----------------------------------------------------------------------------------------------------
    //VONTATÁS
    if (ALL_tularamvedo and vanHalozat) then
    begin
      MOTOR_n_abs := (KONST_FORDULAT / 100) * 4.59 * 60 * (v_axle_abs / (0.96*3.14159));

      //soros
      if (ALL_menetkapcsolo = 0) then MOTOR_ARAM := 0;
      if (ALL_menetkapcsolo = 1) then MOTOR_ARAM := ALL_iranyvalto * min(MAX_ARAM, 278.352136 * exp(-MOTOR_n_abs/384.635834));
      if (ALL_menetkapcsolo = 2) then MOTOR_ARAM := ALL_iranyvalto * min(MAX_ARAM, 523.408852 * exp(-MOTOR_n_abs/438.300827));
      if (ALL_menetkapcsolo = 3) then MOTOR_ARAM := ALL_iranyvalto * min(MAX_ARAM, 987.436016 * exp(-MOTOR_n_abs/369.425108));
      if (ALL_menetkapcsolo = 4) then MOTOR_ARAM := ALL_iranyvalto * min(MAX_ARAM, 2784.955342 * exp(-MOTOR_n_abs/271.456658));
      if (ALL_menetkapcsolo = 5) then MOTOR_ARAM := ALL_iranyvalto * min(MAX_ARAM, 3397.588377 * exp(-MOTOR_n_abs/286.902986));
      //párhuzamos
      if (ALL_menetkapcsolo = 6) then MOTOR_ARAM := ALL_iranyvalto * min(MAX_ARAM, 688.549854 * exp(-MOTOR_n_abs/824.836537));
      if (ALL_menetkapcsolo = 7) then MOTOR_ARAM := ALL_iranyvalto * min(MAX_ARAM, 1019.405278 * exp(-MOTOR_n_abs/748.305776));
      if (ALL_menetkapcsolo = 8) then MOTOR_ARAM := ALL_iranyvalto * min(MAX_ARAM, 1521.695394 * exp(-MOTOR_n_abs/670.666604));
      if (ALL_menetkapcsolo = 9) then MOTOR_ARAM := ALL_iranyvalto * min(MAX_ARAM, 2828.184917 * exp(-MOTOR_n_abs/602.971971));
      //sönt
      if (ALL_menetkapcsolo = 10) then MOTOR_ARAM := ALL_iranyvalto * min(MAX_ARAM, 3328.184917 * exp(-MOTOR_n_abs/602.971971));
      if (ALL_menetkapcsolo = 11) then MOTOR_ARAM := ALL_iranyvalto * min(MAX_ARAM, 3828.184917 * exp(-MOTOR_n_abs/602.971971));

      // VONÓERŐ
      MOTOR_VONOERO := KONST_VONOERO * 55.071177 * (Exp(0.000960144 * MOTOR_ARAM) - 1);
      M_Axle_N_0_0 := MOTOR_VONOERO;

    end;
    
    
    if not ALL_szedo then M_Axle_N_0_0 := 0;
    
  
    
    
    
    //----------------------------------------------------------------------------------------------------
    //VILLAMOS FÉKEZÉS
    if (ALL_menetkapcsolo >= 0) then
    begin
      FEKERO_VILLANY := 0.0;
      FEKGERJEDES_T := const_FEKGERJEDES_T;
      FEKERO_VILLANY_GERJEDT := 0.0;
    end;
    
    //ELSŐ FÉKFOKOZAT - KÉSLELTETÉSSEL, GERJEDÉSSEL
    if (ALL_menetkapcsolo < 0) and (FEKGERJEDES_T > 0.0) then FEKGERJEDES_T := Max(0.0, FEKGERJEDES_T - Timegap);
    if (ALL_menetkapcsolo < 0) and (FEKGERJEDES_T < 0.1) then
    begin
      FEKERO_VILLANY_GERJEDT := Min(MAX_VFEKERO, FEKERO_VILLANY_GERJEDT + const_FEKGERJEDES_SEB*Timegap);  //folyamatosan növekvő nagyságú fékerő (gerjed fel), felső korlát a max_vfékerő
    end;

    if (ALL_menetkapcsolo = -1) then FEKERO_VILLANY := 0.5 * min(MAX_VFEKERO, ( abs(v_Axle_mps_0_0*3.6 -5.5)*MAX_VFEKERO*0.8/42 ) -4000 );
    if (ALL_menetkapcsolo = -2) then FEKERO_VILLANY := min(MAX_VFEKERO, ( abs(v_Axle_mps_0_0*3.6 -5.5)*MAX_VFEKERO*0.8/42 ) -4000 );
    if (ALL_menetkapcsolo = -3) then FEKERO_VILLANY := min(MAX_VFEKERO, ( abs(v_Axle_mps_0_0*3.6 -4.25)*MAX_VFEKERO/38 ) -4000 );
    if (ALL_menetkapcsolo = -4) then FEKERO_VILLANY := min(MAX_VFEKERO, ( abs(v_Axle_mps_0_0*3.6 -3)*MAX_VFEKERO/30 ) -4000 );
    if (ALL_menetkapcsolo = -5) then FEKERO_VILLANY := min(MAX_VFEKERO, ( abs(v_Axle_mps_0_0*3.6 -2)*MAX_VFEKERO/23 ) -4000 );
    if (ALL_menetkapcsolo = -6) then FEKERO_VILLANY := min(MAX_VFEKERO, ( abs(v_Axle_mps_0_0*3.6 -1)*MAX_VFEKERO/17 ) -4000 );
    if (ALL_menetkapcsolo = -7) then FEKERO_VILLANY := min(MAX_VFEKERO, ( abs(v_Axle_mps_0_0*3.6 -1)*MAX_VFEKERO/10 ) -4000 );

    FEKERO_VILLANY := Max(0, KONST_VFEK * FEKERO_VILLANY) * (FEKERO_VILLANY_GERJEDT / MAX_VFEKERO); // a normalizált gerjesztéssel megszorozva kapjuk a felgerjedő fékerőt
    
    //BIZTONSÁGI FÉK
    if (ALL_iranyvalto = 0) then
    begin
      if ((v_Axle_mps_0_0 > 2.2) OR (v_Axle_mps_0_0 < -2.2)) AND (ALL_biztonsagi_fek = false) then ALL_biztonsagi_fek := true;
      if (v_Axle_mps_0_0 < 1.5) AND (v_Axle_mps_0_0 > -1.5) AND (ALL_biztonsagi_fek = true) then ALL_biztonsagi_fek := false;
    end;
    
    if (ALL_iranyvalto <> 0) then ALL_biztonsagi_fek := false;
    
    if (ALL_biztonsagi_fek) then FEKERO_VILLANY := min(MAX_VFEKERO, ( abs(v_Axle_mps_0_0*3.6 -1)*MAX_VFEKERO/12 ) -4000 );
    
    
    //----------------------------------------------------------------------------------------------------
    //LÉGFÉK
    FEKERO_LEGFEK := abs(ALL_fekhengerNyomas / 600 * MAX_LEGFEK);
    
    //----------------------------------------------------------------------------------------------------
    //KÉZIFÉK
    FEKERO_KEZIFEK := ALL_kezifek / 100 * MAX_KEZIFEK;
    
    //----------------------------------------------------------------------------------------------------
    //SÍNFÉK
    F_RailBrake_Bogie_N_0 := ( (abs(fulke_A.sinfek.poz)+abs(fulke_B.sinfek.poz)) * 20 * MAX_SINFEK ) + 0.3*FEKERO_VILLANY;  //mert ugye keresztmezőben a sínféket is gerjeszti a villamosfék
    
    MBrake_Axle_N_0_0 := abs(FEKERO_VILLANY + FEKERO_LEGFEK + FEKERO_KEZIFEK);
    M_Axle_N_0_1 := M_Axle_N_0_0;
    MBrake_Axle_N_0_1 := MBrake_Axle_N_0_0;

    
    //----------------------------------------------------------------------------------------------------
    // FELFÜGGESZTÉS RUGÓK, BEBÓLINTÁS
    RUGO_UT := ( ((loadforce_Axle_N_0_1 - 87500) / RUGOALLANDO) - ((loadforce_Axle_N_0_0 - 87500) / RUGOALLANDO) ) / 2;
    if (abs(RUGO_UT) > 2.25) then RUGO_UT := 0; //Hogy az arkuszszinusz argumentuma véletlen se legyen nagyobb +-1-nél
    
    kasztni_bolintas := Arcsin(RUGO_UT / 2.25);
    
    
    //----------------------------------------------------------------------------------------------------
    //AJTÓMÜÖDÉS
    
    //Cél értékek
    AJTOJOBB_CEL := fulke_A.ajtoJobb.ertek OR fulke_B.ajtoBal.ertek;
    AJTOBAL_CEL := fulke_A.ajtoBal.ertek OR fulke_B.ajtoJobb.ertek;
    
    //AJtók sebessége
    AJTO_J1_SEB := min(AJTO_J1_VMAX, AJTO_J1_VMAX / AJTO_P_TORESPONT * ALL_fektartalyNyomas);
    AJTO_J2_SEB := min(AJTO_J2_VMAX, AJTO_J2_VMAX / AJTO_P_TORESPONT * ALL_fektartalyNyomas);
    AJTO_B1_SEB := min(AJTO_B1_VMAX, AJTO_B1_VMAX / AJTO_P_TORESPONT * ALL_fektartalyNyomas);
    AJTO_B2_SEB := min(AJTO_B2_VMAX, AJTO_B2_VMAX / AJTO_P_TORESPONT * ALL_fektartalyNyomas);
    
    //Ajtók helyzete
    if (AJTOJOBB_CEL = true) then
    begin
      utas_ajto_J1 := min(1.0, utas_ajto_J1 + AJTO_J1_SEB*Timegap);
      utas_ajto_J2 := min(1.0, utas_ajto_J2 + AJTO_J2_SEB*Timegap);end
    else if (AJTOJOBB_CEL = false) then
    begin
      utas_ajto_J1 := max(0.0, utas_ajto_J1 - AJTO_J1_SEB*Timegap);
      utas_ajto_J2 := max(0.0, utas_ajto_J2 - AJTO_J2_SEB*Timegap);
    end;
    //--------------------------------------------------------------
    if (AJTOBAL_CEL = true) then
    begin
      utas_ajto_B1 := min(1.0, utas_ajto_B1 + AJTO_B1_SEB*Timegap);
      utas_ajto_B2 := min(1.0, utas_ajto_B2 + AJTO_B2_SEB*Timegap);
    end
    else if (AJTOBAL_CEL = false) then
    begin
      utas_ajto_B1 := max(0.0, utas_ajto_B1 - AJTO_B1_SEB*Timegap);
      utas_ajto_B2 := max(0.0, utas_ajto_B2 - AJTO_B2_SEB*Timegap);
    end;
    
    
    //---------------------------------------------------------------
    //HANGOK
    
    if ( BoolToFloat(AJTOJOBB_CEL) <> utas_ajto_J1) then
    begin
      HANG_ajtoJ1_loop := 1;
      AJTO_J1_ROGZ := false;
    end
    else HANG_ajtoJ1_loop := -1;
    
    if ( BoolToFloat(AJTOJOBB_CEL) <> utas_ajto_J2) then
    begin
      HANG_ajtoJ2_loop := 1;
      AJTO_J2_ROGZ := false;
    end
    else HANG_ajtoJ2_loop := -1;
    
    if ( BoolToFloat(AJTOBAL_CEL) <> utas_ajto_B1) then
    begin
      HANG_ajtoB1_loop := 1;
      AJTO_B1_ROGZ := false;
    end
    else HANG_ajtoB1_loop := -1;
    
    if ( BoolToFloat(AJTOBAL_CEL) <> utas_ajto_B2) then
    begin
      HANG_ajtoB2_loop := 1;
      AJTO_B2_ROGZ := false;
    end
    else HANG_ajtoB2_loop := -1;
    
    //------------------------------------------------------
    if ( (BoolToFloat(AJTOJOBB_CEL) = utas_ajto_J1) and not AJTO_J1_ROGZ) then
    begin
      HANG_ajtoJ1_stop := 1;
      AJTO_J1_ROGZ := true;
    end;
    
    if ( (BoolToFloat(AJTOJOBB_CEL) = utas_ajto_J2) and not AJTO_J2_ROGZ) then
    begin
      HANG_ajtoJ2_stop := 1;
      AJTO_J2_ROGZ := true;
    end;
    
    if ( (BoolToFloat(AJTOBAL_CEL) = utas_ajto_B1) and not AJTO_B1_ROGZ) then
    begin
      HANG_ajtoB1_stop := 1;
      AJTO_B1_ROGZ := true;
    end;
    
    if ( (BoolToFloat(AJTOBAL_CEL) = utas_ajto_B2) and not AJTO_B2_ROGZ) then
    begin
      HANG_ajtoB2_stop := 1;
      AJTO_B2_ROGZ := true;
    end;
    
		//---------------------------------------------------------------
		// UTASAJT� PARANCSOK
		if (utas_ajto_B1 > 0.9) then DoorOpen_0 := true else DoorOpen_0 := false;
		if (utas_ajto_B2 > 0.9) then DoorOpen_1 := true else DoorOpen_1 := false;
		if (utas_ajto_J1 > 0.9) then DoorOpen_2 := true else DoorOpen_2 := false;
		if (utas_ajto_J2 > 0.9) then DoorOpen_3 := true else DoorOpen_3 := false;
    
    
    //----------------------------------------------------------------------------------------------------
    //IND�T�SJELZ�
    if (not vanHalozat) then utas_berrego := false;
    
    //T�VVEZ�RL�S A BERREG�RE
    //if (coupled_0 = true) AND (csatlas_berrego_front = true) then utas_berrego := true;
    
    
    if (utas_berrego) then HANG_inditasjelzo := 1 else HANG_inditasjelzo := -1;
    if (utas_berrego) then LM_utas_berrego := 1 else LM_utas_berrego := 0;
    
    
    
    
    //----------------------------------------------------------------------------------------------------
    //F�NYEK
    
    //HELYZETJELZ�K
    if vanHalozat AND utaster.K1.ertek AND utaster.K2.ertek AND (not utaster.K3.ertek) AND (not fulke_A.sugarveto.ertek) then LM_A_helyzetjelzo := 1 else LM_A_helyzetjelzo := 0;
    if vanHalozat AND utaster.K1.ertek AND utaster.K2.ertek AND utaster.K3.ertek AND (not fulke_B.sugarveto.ertek) then LM_B_helyzetjelzo := 1 else LM_B_helyzetjelzo := 0;

    //TOMP�TOTT F�NYSZ�R�
    if vanHalozat AND utaster.K1.ertek AND utaster.K2.ertek AND (not utaster.K3.ertek) AND fulke_A.sugarveto.ertek then LM_A_tompitott := 1 else LM_A_tompitott := 0;
    if vanHalozat AND utaster.K1.ertek AND utaster.K2.ertek AND utaster.K3.ertek AND fulke_B.sugarveto.ertek then LM_B_tompitott := 1 else LM_B_tompitott := 0;
    
    //Z�RF�NYEK
    if vanHalozat AND utaster.K1.ertek AND utaster.K2.ertek AND (not utaster.K3.ertek) then LM_B_zarfeny := 1 else LM_B_zarfeny := 0;
    if vanHalozat AND utaster.K1.ertek AND utaster.K2.ertek AND utaster.K3.ertek then LM_A_zarfeny := 1 else LM_A_zarfeny := 0;
    
    //UTAST�R
    if (vanHalozat AND utaster.K1.ertek AND utaster.K2.ertek) OR (csatlas_vilagitasCsatlakozo600V_front AND coupled_0) OR (csatlas_vilagitasCsatlakozo600V_back AND coupled_1) then LM_utaster := 1 else LM_utaster := 0;
    
    //PERONVIL�G�T�SOK
    if ((vanHalozat AND utaster.K1.ertek AND utaster.K2.ertek) OR (csatlas_vilagitasCsatlakozo600V_front AND coupled_0) OR (csatlas_vilagitasCsatlakozo600V_back AND coupled_1)) AND (not utaster.K3.ertek) then LM_utaster_B_peron := 1 else LM_utaster_B_peron := 0;
    if ((vanHalozat AND utaster.K1.ertek AND utaster.K2.ertek) OR (csatlas_vilagitasCsatlakozo600V_front AND coupled_0) OR (csatlas_vilagitasCsatlakozo600V_back AND coupled_1)) AND utaster.K3.ertek then LM_utaster_A_peron := 1 else LM_utaster_A_peron := 0;
    
    
    
    
    //----------------------------------------------------------------------------------------------------
    // KAPASZKOD�K MOZG�SA
    
    //FIZIKAI CUCCOK
    if (v_ground = 0) then
      VAR_PHY_lassulas := 0
    else
      VAR_PHY_lassulas := ALL_iranyvalto * MBrake_Axle_N_0_0;
    
    PHY_gyorsulas := (M_Axle_N_0_0 - VAR_PHY_lassulas) / VILLAMOS_TOMEG;
    PHY_oldalgyorsulas := v_ground * v_ground * invradius_0_0;
    
    
    
    if (kapaszkodo_Y_szogseb > 0) then
      kapaszkodo_Y_szogseb := kapaszkodo_Y_szogseb + ((-1*KAPASZKODO_const1*PHY_gyorsulas + KAPASZKODO_const2*kapaszkodo_Y_szog - KAPASZKODO_const3*kapaszkodo_Y_szogseb*kapaszkodo_Y_szogseb)*Timegap)
    else
      kapaszkodo_Y_szogseb := kapaszkodo_Y_szogseb + ((-1*KAPASZKODO_const1*PHY_gyorsulas + KAPASZKODO_const2*kapaszkodo_Y_szog + KAPASZKODO_const3*kapaszkodo_Y_szogseb*kapaszkodo_Y_szogseb)*Timegap);
    
    kapaszkodo_Y_szog := kapaszkodo_Y_szog - (kapaszkodo_Y_szogseb * Timegap);
    
    //-------------------------------------------------------------------------
    
    if (kapaszkodo_X_szogseb > 0) then
      kapaszkodo_X_szogseb := kapaszkodo_X_szogseb + ((-1*KAPASZKODO_const1*PHY_oldalgyorsulas + KAPASZKODO_const2*kapaszkodo_X_szog - KAPASZKODO_const3*kapaszkodo_X_szogseb*kapaszkodo_X_szogseb)*Timegap)
    else
      kapaszkodo_X_szogseb := kapaszkodo_X_szogseb + ((-1*KAPASZKODO_const1*PHY_oldalgyorsulas + KAPASZKODO_const2*kapaszkodo_X_szog + KAPASZKODO_const3*kapaszkodo_X_szogseb*kapaszkodo_X_szogseb)*Timegap);
    
    kapaszkodo_X_szog := kapaszkodo_X_szog - (kapaszkodo_X_szogseb * Timegap);
    
    
    
    //----------------------------------------------------------------------------------------------------
    //CSATL�SOK SZIMUL�L�SA
    
    //V�LTOZ� K�LD�SEK
    SendToECouplerBoolean(Self, 'csatlas_vilagitasCsatlakozo600V', (vanHalozat AND utaster.K1.ertek), 0); //Vil�g�t�s el�re
    SendToECouplerBoolean(Self, 'csatlas_vilagitasCsatlakozo600V', (vanHalozat AND utaster.K1.ertek), 1); //Vil�g�t�s h�tra
    SendToECouplerBoolean(Self, 'csatlas_berrego', utas_berrego, 0); //berreg� el�re
    SendToECouplerBoolean(Self, 'csatlas_berrego', utas_berrego, 1); //berreg� h�tra
    
    SendToECouplerSingle(Self, 'csatlas_fekhengerNyomas', ALL_fekhengerNyomas, 0); //f�ker� el�re
    SendToECouplerSingle(Self, 'csatlas_fekhengerNyomas', ALL_fekhengerNyomas, 1); //f�ker� h�tra
    
    
    
    

  end;
  

  
  
  
  
  
  //------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
  
  procedure elektronika_init;
  begin
  
		//konstans be�ll�t�s
		MAX_VONTATAS := KONST_MAX_VONTATAS; //N-ban
    KONST_VONOERO := 700;
    KONST_FORDULAT := 250;
    KONST_VFEK := 1.5;
  
  
  
    SZEDO_ALLAS := 0;
    SZEDO_SZOG_MAX := 0;
    
    panto_voltage_0 := true;
    vanHalozat := false;
    ALL_szedo_irany := true;
    if(InitReadyForMovement > 1) then ALL_szedo := true else ALL_szedo := false;
    
    M_Axle_N_0_0 := 0;
    M_Axle_N_0_1 := 0;
    
    MBrake_Axle_N_0_0 := 0;
    MBrake_Axle_N_0_1 := 0;
    F_RailBrake_Bogie_N_0 := 0;
    
    RUGO_UT := 0;
    
    FEKGERJEDES_T := const_FEKGERJEDES_T;
    
    AJTOJOBB_CEL := false;
    AJTOBAL_CEL := false;
    utas_ajto_J1 := 0;
    utas_ajto_J2 := 0;
    utas_ajto_B1 := 0;
    utas_ajto_B2 := 0;
    
    AJTO_J1_ROGZ := true;
    AJTO_J2_ROGZ := true;
    AJTO_B1_ROGZ := true;
    AJTO_B2_ROGZ := true;
    
    kapaszkodo_X_szog := 0;
    kapaszkodo_X_szogseb := 0;
    kapaszkodo_Y_szog := 0;
    kapaszkodo_Y_szogseb := 0;
    
    ALL_biztonsagi_fek := false;
    
  end;