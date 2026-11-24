var
  fulke_A_aktiv: boolean;
  fulke_B_aktiv: boolean;
  
  
  
//FELADAT: RC_xy változók írása, ha aktív a jármû
procedure RC_kuldes;
begin
	if (RC_Active = false) then
	begin
	
		//RC_DirLeaver
		fulke_A_aktiv := (A_CP_iranyvaltoBent and A_CP_menetkapcsoloBent and A_CP_fekszelepBent) AND ((not B_CP_iranyvaltoBent) and (not B_CP_menetkapcsoloBent) and (not B_CP_fekszelepBent));
		fulke_B_aktiv := (B_CP_iranyvaltoBent and B_CP_menetkapcsoloBent and B_CP_fekszelepBent) AND ((not A_CP_iranyvaltoBent) and (not A_CP_menetkapcsoloBent) and (not A_CP_fekszelepBent));
		
		if fulke_A_aktiv then RC_DirLeaver := 1
		else if fulke_B_aktiv then RC_DirLeaver := -1
		else RC_DirLeaver := 0;
		
		
		//RC_Throttle
		RC_Throttle := Min(1, abs(M_Axle_N_0_0 / MAX_VONTATAS));
		
		//RC_Brake
		RC_Brake := Min(1, MBrake_Axle_N_0_0 / 18000);	//18kN a max légfék, efelett amúgy is blokkolna a tengely kb, de azért maximalizáljuk 1-ben
		
		
		
		//RC_FrontLight_Front (0 = off, 1 = daytime running lights, 2 = dipped beam, 3 = main beam)
		if (LM_A_helyzetjelzo <> 0) and (LM_A_tompitott = 0) then RC_FrontLight_Front := 2
		else if (LM_A_tompitott = 1) then RC_FrontLight_Front := 3
		else RC_FrontLight_Front := 0;
		
		//RC_FrontLight_Back
		if (LM_B_helyzetjelzo <> 0) and (LM_B_tompitott = 0) then RC_FrontLight_Back := 2
		else if (LM_B_tompitott = 1) then RC_FrontLight_Back := 3
		else RC_FrontLight_Back := 0;
		
		//RC_OtherLights_Front (zárfény elöl)
		if (LM_A_zarfeny <> 0) then RC_OtherLights_Front := 1 else RC_OtherLights_Front := 0;
		
		//RC_OtherLights_Back (zárfény elöl)
		if (LM_B_zarfeny <> 0) then RC_OtherLights_Back := 1 else RC_OtherLights_Back := 0;
		
		//RC_CabinLight (utastér +1, elsõ peron +2, hátsó peron +4)
		if (LM_utaster <> 0) and (LM_utaster_A_peron = 0) and (LM_utaster_B_peron = 0) then RC_CabinLight := 1
		else if (LM_utaster <> 0) and (LM_utaster_A_peron <> 0) and (LM_utaster_B_peron = 0) then RC_CabinLight := 3
		else if (LM_utaster <> 0) and (LM_utaster_A_peron = 0) and (LM_utaster_B_peron <> 0) then RC_CabinLight := 5
		else RC_CabinLight := 0;
		
		
		
		
		//RC_Belling
		if (A_CP_csengo <> 0) OR (B_CP_csengo <> 0) then RC_Belling := 1 else RC_Belling := 0;
		
		//RC_Sanding
		if sanding_0_0 or sanding_0_1 then RC_Sanding := true else RC_Sanding := false;
		
		
		// ajtó célok (RC_DoorsOpen_Left, RC_DoorsOpen_Right)
		// magyarázat: bit-flag az ajtókezelés (lásd lexikon), de a két ajtó úgy is egyszerre vagy nyílik, vagy záródik,
		// ezért egybevontam a +1, +2-t (elsõ, második ajtó)
		if AJTOBAL_CEL then RC_DoorsOpen_Left := 3 else RC_DoorsOpen_Left := 0;
		if AJTOJOBB_CEL then RC_DoorsOpen_Right := 3 else RC_DoorsOpen_Right := 0;
		
		
		//RC_PantographApplied
		//megj: ez is bit-flag-es, de csak egy szedõ van....
		if (ALL_szedo) then RC_PantographApplied := 1 else RC_PantographApplied := 0;

	end;
end;  
  
  
  
procedure fulke_SimStep_RC;
begin

	//TODO fülke hangok, pl. csengõ
	
	if (RC_Belling = 1) then HANG_A_csengo := 1;

end;
  
  
  
  
  
  
  
  
procedure elektronika_SimStep_RC;
begin
	//TODO utastérvilágítás, fények, ajtók
	
	//-----------------------------------------------------------------------
	//SZEDÕ SZIMULÁLÁSA - ELÕRE
	if (ALL_szedo and (RC_DirLeaver = 1)) then
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
	
	
	//SZEDÕ SZIMULÁLÁSA - HÁTRA
	if (ALL_szedo and (RC_DirLeaver = -1)) then
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
	
	
	//FÜLKE KÖTELEK BEÁLLÍTÁSA
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
	
	
	//----------------------------------------------------------------------------------------------------
	//AJTÓMÛKÖDÉS
	
	//Cél értékek
	AJTOJOBB_CEL := (RC_DoorsOpen_Right <> 0);
	AJTOBAL_CEL := (RC_DoorsOpen_Left <> 0);
	
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
	
	
	//----------------------------------------------------------------------------------------------------
	//FÉNYEK
	
	//HELYZETJELZÕK
	if (RC_FrontLight_Front >= 2) then LM_A_helyzetjelzo := 1 else LM_A_helyzetjelzo := 0;
	if (RC_FrontLight_Back >= 2) then LM_B_helyzetjelzo := 1 else LM_B_helyzetjelzo := 0;

	//TOMPÍTOTT FÉNYSZÓRÓ
	if (RC_FrontLight_Front = 3) then LM_A_tompitott := 1 else LM_A_tompitott := 0;
	if (RC_FrontLight_Back = 3) then LM_B_tompitott := 1 else LM_B_tompitott := 0;
	
	//ZÁRFÉNYEK
	if (RC_OtherLights_Front <> 0) then LM_A_zarfeny := 1 else LM_A_zarfeny := 0;
	if (RC_OtherLights_Back <> 0) then LM_B_zarfeny := 1 else LM_B_zarfeny := 0;
	
	//UTASTÉR
	if (RC_CabinLight > 0) then LM_utaster := 1 else LM_utaster := 0;
	
	//PERONVILÁGÍTÁSOK
	if (RC_CabinLight = 3) then LM_utaster_A_peron := 1 else LM_utaster_A_peron := 0;
	if (RC_CabinLight = 5) then LM_utaster_B_peron := 1 else LM_utaster_B_peron := 0;

	
	
	//----------------------------------------------------------------------------------------------------
	//EGYEBEK
	
	if RC_Sanding then
	begin
		sanding_0_0 := true;
		sanding_0_1 := true;
	end
		else
	begin
		sanding_0_0 := false;
		sanding_0_1 := false;
	end;
	
	
	
	
	
	

end;
  
  
  
  
  
  
  
  
  
  
  
  
  

procedure RCscript_simstep;
begin

  fulke_SimStep_RC;
  elektronika_SimStep_RC;
  
  //--------------------------------------
  //ÁLTALÁNOS VÁLTOZÓK
  
  if (not coupled_0) then couplingState_0 := 1;
  if (not coupled_1) then couplingState_1 := 1;
  
  v_axle_abs := abs(v_Axle_mps_0_0);

  
  
  

  //--------------------------------------
  //ÁLTALÁNOS HANGOK
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
  //KERÉK CSATTOGÁS
  HANG_kerekcsattogas := -1;


  //--------------------------------------
  //MOTOR HANGOK (csak menetre)
  if (RC_Throttle <> 0) and (v_axle_abs > 0.4) then
  begin
    HANG_motor := 1;
  end
  else
  begin
    HANG_motor := -1;
  end;
  HANG_motorhangero := max(0.65 , (abs(RC_Throttle * min(MAX_VONTATAS, Exp( -abs(v_Axle_mps_0_0)*3.6*0.2+6)*MAX_VONTATAS * 1 ) )/MAX_VONTATAS) );


  
  
end;