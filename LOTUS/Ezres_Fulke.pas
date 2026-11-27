const
  //MENETFOKOZATOKHOZ AZ ÉRTÉKEK (FOKBAN)
  MENETKAPCSOLO_f7 = -118;
  MENETKAPCSOLO_f6 = -105;
  MENETKAPCSOLO_f5 = -90;
  MENETKAPCSOLO_f4 = -70;
  MENETKAPCSOLO_f3 = -55;
  MENETKAPCSOLO_f2 = -40;
  MENETKAPCSOLO_f1 = -20;
  MENETKAPCSOLO_0 = 0;
  MENETKAPCSOLO_m1 = 20;
  MENETKAPCSOLO_m2 = 40;
  MENETKAPCSOLO_m3 = 60;
  MENETKAPCSOLO_m4 = 80;
  MENETKAPCSOLO_m5 = 100;
  MENETKAPCSOLO_m6 = 120;
  MENETKAPCSOLO_m7 = 140;
  MENETKAPCSOLO_m8 = 160;
  MENETKAPCSOLO_m9 = 180;
  MENETKAPCSOLO_m10 = 200;
  MENETKAPCSOLO_m11 = 228;
  
  //IRÁNYVÁLTÓ ÁLLÁSAI
  IRANYVALTO_ELORE = 70;
  IRANYVALTO_HATRA = -70;
  
  
  //FÉKRENDSZER ÁRAMLÁSOK (kPa/s)
  FEKBE_MAX = 170; // VÉSZFÉK
  FEKKI_SEB = 170; // KIENGEDÉS
  
  
  //ABLAK SEBESSÉGEK ( %/s )
  ABLAK_FEL_SEB = 0.05;
  ABLAK_LE_SEB = 0.03;
  
 
  
  /////////////////////////////////////////////////////////////////////////////////////////////////////////////
  //////// TÍPUSOK ///////////////////
  
  // KAPCSOL�K
  type
    tKapcsolo = record
      ertek: boolean;
      poz: single;
  end;
  
  type
    tTobbAllasuKapcsolo = record
      ertek: single;
      poz: single;
  end;
  
  type
    tRugosKapcsolo = record
      ertek: boolean;
      poz: single;
  end;
  

//----------------------------------------------------------------------------------

// FÜLKE OBJEKTUM
  type
    tFulke = record
      aktiv: boolean;
      
      szamlalo1: single;
      jottThrottle: single;
      kezifekCel: single;
      
      iranyvaltoBent: boolean;
      menetkapcsoloBent: boolean;
      fekszelepBent: boolean;
      
      menetkapcsolo: tTobballasuKapcsolo;
      iranyvalto: tTobballasuKapcsolo;
      fekszelep: tTobballasuKapcsolo;
      kezifek: tTobballasuKapcsolo;  //SZ�ZAL�KBAN
      
      sinfek: tKapcsolo;
      tularamvedo: tKapcsolo;
      kotelCsomo: boolean;
      kotelNemCsomo: boolean;
      
      ajtoBal: tKapcsolo;
      ajtoJobb: tKapcsolo;
      csengo: tRugosKapcsolo;
      sugarveto: tKapcsolo;
      legsurito: tKapcsolo;
      indjelzoBe: tRugosKapcsolo;
      indjelzoKi: tRugosKapcsolo;
      homokolo: tRugosKapcsolo;
  end;
  
  // UTASTÉR OBJEKTUM
  type
    tUtaster = record
      ablakB2_cel: integer;
      ablakB3_cel: integer;
      ablakB4_cel: integer;
      ablakJ2_cel: integer;
      ablakJ3_cel: integer;
      ablakJ4_cel: integer;
      
      K1: tKapcsolo;
      K2: tKapcsolo;
      K3: tKapcsolo;
  end;

  // KAMERA PATTOGÁSHOZ
  type
    t_pattogas = record
      cfg_kiteres: single;
      cfg_freq: single;
      cfg_hossz: single;

      MAX_KITERES: single;

      allapot: integer; // 0: kezdődik a várakozás a köv. pattanásra / 1: pattogás folyamata / 2: véget ért a pattogás -> 0 jön
      ertek: single;    // méter
      szaml: single;    // másodperc
  end;

  procedure pattogas_init(var this: t_pattogas; kiteres: single; freq: single; hossz: single);
  begin
    this.cfg_freq := freq;
    this.cfg_kiteres := kiteres;
    this.cfg_hossz := hossz;

    this.allapot := 0;
    this.ertek := 0;
    this.szaml := 0;
  end;

  procedure pattogas_simstep(var this: t_pattogas);
  begin

    // Ha megállt az előző pattogás, várjuk a következőt
    if (v_axle_kmph > 5) and (this.allapot = 2) then
    begin
      this.allapot := 0;
      this.szaml := Max(4, 2 + 10*Random - (v_axle_kmph / 8.0));
    end;

    // várjuk a pattogást
    if (this.allapot = 0) then
    begin

      if this.szaml < 0 then
      begin
        this.allapot := 1;
        this.szaml := 0;
        this.MAX_KITERES := this.cfg_kiteres*Min(0.01 + 0.01*Random, (0.03/70)*v_axle_kmph);
      end
      else
        this.szaml := this.szaml - Timegap;
    end;

    // pattogás
    if (this.allapot = 1) then
    begin

      if this.MAX_KITERES * Power(2, -1 * this.cfg_hossz * this.szaml) > 0.002 then
      begin
        this.ertek := this.MAX_KITERES * Sin((this.cfg_freq + (v_axle_kmph/6))*this.szaml) * Power(2, -1 * this.cfg_hossz * this.szaml);
        this.szaml := this.szaml + Timegap;
      end
      else
      begin
        this.allapot := 2;
        this.ertek := 0;
      end;
    end;

  end;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////// FÜLKE ///////////////////

  var
    fulke_A: tFulke;
    fulke_B: tFulke;
    utaster: tUtaster;
    
    legsuritoJar: boolean;
    
    menetfokozatElozo_A: single;
    menetfokozatElozo_B: single;

    fek_A_cel: single;
    fek_A_mozog: boolean;
    fek_B_cel: single;
    fek_B_mozog: boolean;

    kam_pattog_Y_0: t_pattogas;
    kam_pattog_Y_1: t_pattogas;
    kam_pattog_Y_2: t_pattogas;
    kam_pattog_Y_3: t_pattogas;
    kam_pattog_Y_4: t_pattogas;
    //
    kam_pattog_X_0: t_pattogas;
    kam_pattog_X_1: t_pattogas;
    kam_pattog_X_2: t_pattogas;
    
   
    
    
  //------------------------------------------------------------------------------
  procedure fulke_MenetkapcsoloFrissites(var _fulke: tFulke);
  begin
    if (_fulke.menetkapcsolo.ertek = -7) then _fulke.menetkapcsolo.poz := MENETKAPCSOLO_f7;
    if (_fulke.menetkapcsolo.ertek = -6) then _fulke.menetkapcsolo.poz := MENETKAPCSOLO_f6;
    if (_fulke.menetkapcsolo.ertek = -5) then _fulke.menetkapcsolo.poz := MENETKAPCSOLO_f5;
    if (_fulke.menetkapcsolo.ertek = -4) then _fulke.menetkapcsolo.poz := MENETKAPCSOLO_f4;
    if (_fulke.menetkapcsolo.ertek = -3) then _fulke.menetkapcsolo.poz := MENETKAPCSOLO_f3;
    if (_fulke.menetkapcsolo.ertek = -2) then _fulke.menetkapcsolo.poz := MENETKAPCSOLO_f2;
    if (_fulke.menetkapcsolo.ertek = -1) then _fulke.menetkapcsolo.poz := MENETKAPCSOLO_f1;
    if (_fulke.menetkapcsolo.ertek = 0) then _fulke.menetkapcsolo.poz := MENETKAPCSOLO_0;
    if (_fulke.menetkapcsolo.ertek = 1) then _fulke.menetkapcsolo.poz := MENETKAPCSOLO_m1;
    if (_fulke.menetkapcsolo.ertek = 2) then _fulke.menetkapcsolo.poz := MENETKAPCSOLO_m2;
    if (_fulke.menetkapcsolo.ertek = 3) then _fulke.menetkapcsolo.poz := MENETKAPCSOLO_m3;
    if (_fulke.menetkapcsolo.ertek = 4) then _fulke.menetkapcsolo.poz := MENETKAPCSOLO_m4;
    if (_fulke.menetkapcsolo.ertek = 5) then _fulke.menetkapcsolo.poz := MENETKAPCSOLO_m5;
    if (_fulke.menetkapcsolo.ertek = 6) then _fulke.menetkapcsolo.poz := MENETKAPCSOLO_m6;
    if (_fulke.menetkapcsolo.ertek = 7) then _fulke.menetkapcsolo.poz := MENETKAPCSOLO_m7;
    if (_fulke.menetkapcsolo.ertek = 8) then _fulke.menetkapcsolo.poz := MENETKAPCSOLO_m8;
    if (_fulke.menetkapcsolo.ertek = 9) then _fulke.menetkapcsolo.poz := MENETKAPCSOLO_m9;
    if (_fulke.menetkapcsolo.ertek = 10) then _fulke.menetkapcsolo.poz := MENETKAPCSOLO_m10;
    if (_fulke.menetkapcsolo.ertek = 11) then _fulke.menetkapcsolo.poz := MENETKAPCSOLO_m11;
  end;
  
  
  
  //------------------------------------------------------------------------------
  
  procedure fulke_SimStep;
  begin

    // KAMERA ------------------------------------------------
    pattogas_simstep(kam_pattog_Y_0);
    pattogas_simstep(kam_pattog_Y_1);
    pattogas_simstep(kam_pattog_Y_2);
    pattogas_simstep(kam_pattog_Y_3);
    pattogas_simstep(kam_pattog_Y_4);
    //
    pattogas_simstep(kam_pattog_X_0);
    pattogas_simstep(kam_pattog_X_1);
    pattogas_simstep(kam_pattog_X_2);
    KAMERA_Y := kam_pattog_Y_0.ertek + kam_pattog_Y_1.ertek + kam_pattog_Y_2.ertek + kam_pattog_Y_3.ertek + kam_pattog_Y_4.ertek;

    
    if (fulke_A.menetkapcsoloBent and fulke_A.iranyvaltoBent and fulke_A.fekszelepBent) and (not fulke_B.aktiv) then fulke_A.aktiv := true else fulke_A.aktiv := false;
    if (fulke_B.menetkapcsoloBent and fulke_B.iranyvaltoBent and fulke_B.fekszelepBent) and (not fulke_A.aktiv) then fulke_B.aktiv := true else fulke_B.aktiv := false;
    
    fulke_A.kotelNemCsomo := not fulke_A.kotelCsomo;
    fulke_B.kotelNemCsomo := not fulke_B.kotelCsomo;
    
    
    //------------------------------------------------------------------------------------------
    //MENETSZAB�LYZ� KEZEL�SE (A)
    if (fulke_A.jottThrottle = 1) then
    begin
      if (fulke_A.szamlalo1 > 0.07) then
      begin
        if (fulke_A.menetkapcsolo.ertek < 0) then HANG_A_menetkapcsolo_fek := 1
        else if (fulke_A.menetkapcsolo.ertek < 10) then HANG_A_menetkapcsolo_menet := 1
        else if (fulke_A.menetkapcsolo.ertek = 10) then HANG_A_menetkapcsolo_menetMax := 1;
        
        fulke_A.menetkapcsolo.ertek := fulke_A.menetkapcsolo.ertek + 1;
        if (fulke_A.menetkapcsolo.ertek > 11) then fulke_A.menetkapcsolo.ertek := 11;
        
        fulke_MenetkapcsoloFrissites(fulke_A);
        
        fulke_A.szamlalo1 := 0;
      end;
      fulke_A.szamlalo1 := fulke_A.szamlalo1 + Timegap;
    end;
    
    if (fulke_A.jottThrottle = 0) then
    begin
      fulke_A.szamlalo1 := 0;
    end;
    
    //------------------------------------------------------------------------------------------
    //MENETSZAB�LYZ� KEZEL�SE (B)
    if (fulke_B.jottThrottle = 1) then
    begin
      if (fulke_B.szamlalo1 > 0.07) then
      begin
        if (fulke_B.menetkapcsolo.ertek < 0) then HANG_B_menetkapcsolo_fek := 1
        else if (fulke_B.menetkapcsolo.ertek < 10) then HANG_B_menetkapcsolo_menet := 1
        else if (fulke_B.menetkapcsolo.ertek = 10) then HANG_B_menetkapcsolo_menetMax := 1;
        
        fulke_B.menetkapcsolo.ertek := fulke_B.menetkapcsolo.ertek + 1;
        if (fulke_B.menetkapcsolo.ertek > 11) then fulke_B.menetkapcsolo.ertek := 11;
        
        fulke_MenetkapcsoloFrissites(fulke_B);
        
        fulke_B.szamlalo1 := 0;
      end;
      fulke_B.szamlalo1 := fulke_B.szamlalo1 + Timegap;
    end;
    
    if (fulke_B.jottThrottle = 0) then
    begin
      fulke_B.szamlalo1 := 0;
    end;
    
    
    
    //------------------------------------------------------------------------------------------
    // FÉKSZELEP KEZELÉSE (A) //

    // POZÍCIÓ KEZELÉSE (BILLENTYŰZET)
    if (fek_A_cel = -1) and (fek_A_mozog) then     // OLDÁS
    begin
      fulke_A.fekszelep.ertek := Max(-1.0, fulke_A.fekszelep.ertek - (Timegap * 2.0));
      if (fulke_A.fekszelep.ertek <= -1.0) then fek_A_mozog := false;
    end;

    if (fek_A_cel = 0) and (fek_A_mozog) then     // ZÁRT
    begin
      if (fulke_A.fekszelep.ertek >= 0) then
      begin
        fulke_A.fekszelep.ertek := Max(0.0, fulke_A.fekszelep.ertek - (Timegap * 2.0));
        if (fulke_A.fekszelep.ertek <= 0.0) then fek_A_mozog := false;
      end
      else
      begin
        fulke_A.fekszelep.ertek := Min(0.0, fulke_A.fekszelep.ertek + (Timegap * 2.0));
        if (fulke_A.fekszelep.ertek >= 0.0) then fek_A_mozog := false;
      end;
    end;

    if (fek_A_cel = 1) and (fek_A_mozog) then     // ÜZEMI FÉK
    begin
      if (fulke_A.fekszelep.ertek >= 1) then
      begin
        fulke_A.fekszelep.ertek := Max(1.0, fulke_A.fekszelep.ertek - (Timegap * 2.0));
        if (fulke_A.fekszelep.ertek <= 1.0) then fek_A_mozog := false;
      end
      else
      begin
        fulke_A.fekszelep.ertek := Min(1.0, fulke_A.fekszelep.ertek + (Timegap * 2.0));
        if (fulke_A.fekszelep.ertek >= 1.0) then fek_A_mozog := false;
      end;
    end;
    
    if (fek_A_cel = 2) and (fek_A_mozog) then     // VÉSZFÉK
    begin
      fulke_A.fekszelep.ertek := Min(2.0, fulke_A.fekszelep.ertek + (Timegap * 4.0));
      if (fulke_A.fekszelep.ertek >= 2.0) then fek_A_mozog := false;
    end;

    // SZIMULÁCIÓ KEZELÉSE
    if (fulke_A.fekszelep.ertek < 0) then     // OLDÁS
    begin
      if (ALL_fekhengerNyomas > 0) then ALL_fekhengerNyomas := ALL_fekhengerNyomas - (Timegap * FEKKI_SEB * abs(fulke_A.fekszelep.ertek));
      
      HANG_A_fekOld := 1;
      HANG_A_fekUzem := -1;
      VOL_HANG_A_fekOld := abs(fulke_A.fekszelep.ertek);
      if (ALL_fekhengerNyomas = 100) then HANG_fekrudazat_ki := 1;
    end;
    
    if (fulke_A.fekszelep.ertek = 0) then     // ZÁRT
    begin
      VOL_HANG_A_fekUzem := 0;
      VOL_HANG_A_fekOld := 0;
      
      HANG_A_fekUzem := -1;
      HANG_A_fekOld := -1;
    end;
    
    if (fulke_A.fekszelep.ertek > 0) then     // ÜZEM vagy VÉSZFÉK
    begin
      if (ALL_fekhengerNyomas < ALL_fektartalyNyomas) then
      begin
        ALL_fekhengerNyomas := ALL_fekhengerNyomas + (Timegap * FEKBE_MAX * abs(fulke_A.fekszelep.ertek));
        ALL_fektartalyNyomas := ALL_fektartalyNyomas - (Timegap  *FEKBE_MAX/12 * abs(fulke_A.fekszelep.ertek));
      end;
      
      HANG_A_fekUzem := 1;
      HANG_A_fekOld := -1;
      VOL_HANG_A_fekUzem := abs(fulke_A.fekszelep.ertek);
      if (ALL_fekhengerNyomas = 100) then HANG_fekrudazat_be := 1;
    end;

    // ANIMÁCIÓ
    if (fulke_A.fekszelep.ertek < 0) then fulke_A.fekszelep.poz := Max(0.0, 58 * (fulke_A.fekszelep.ertek + 1.0))
    else if (fulke_A.fekszelep.ertek < 1) then fulke_A.fekszelep.poz := 58 + (85-58) * fulke_A.fekszelep.ertek
    else fulke_A.fekszelep.poz := Min(110.0, 85 + (110-85) * (fulke_A.fekszelep.ertek - 1.0));

    
    
    
    //------------------------------------------------------------------------------------------
    //FÉKSZELEP KEZELÉSE (B)

    // POZÍCIÓ KEZELÉSE (BILLENTYŰZET)
    if (fek_B_cel = -1) and (fek_B_mozog) then     // OLDÁS
    begin
      fulke_B.fekszelep.ertek := Max(-1.0, fulke_B.fekszelep.ertek - (Timegap * 2.0));
      if (fulke_B.fekszelep.ertek <= -1.0) then fek_B_mozog := false;
    end;

    if (fek_B_cel = 0) and (fek_B_mozog) then     // ZÁRT
    begin
      if (fulke_B.fekszelep.ertek >= 0) then
      begin
        fulke_B.fekszelep.ertek := Max(0.0, fulke_B.fekszelep.ertek - (Timegap * 2.0));
        if (fulke_B.fekszelep.ertek <= 0.0) then fek_B_mozog := false;
      end
      else
      begin
        fulke_B.fekszelep.ertek := Min(0.0, fulke_B.fekszelep.ertek + (Timegap * 2.0));
        if (fulke_B.fekszelep.ertek >= 0.0) then fek_B_mozog := false;
      end;
    end;

    if (fek_B_cel = 1) and (fek_B_mozog) then     // ÜZEMI FÉK
    begin
      if (fulke_B.fekszelep.ertek >= 1) then
      begin
        fulke_B.fekszelep.ertek := Max(1.0, fulke_B.fekszelep.ertek - (Timegap * 2.0));
        if (fulke_B.fekszelep.ertek <= 1.0) then fek_B_mozog := false;
      end
      else
      begin
        fulke_B.fekszelep.ertek := Min(1.0, fulke_B.fekszelep.ertek + (Timegap * 2.0));
        if (fulke_B.fekszelep.ertek >= 1.0) then fek_B_mozog := false;
      end;
    end;
    
    if (fek_B_cel = 2) and (fek_B_mozog) then     // VÉSZFÉK
    begin
      fulke_B.fekszelep.ertek := Min(2.0, fulke_B.fekszelep.ertek + (Timegap * 4.0));
      if (fulke_B.fekszelep.ertek >= 2.0) then fek_B_mozog := false;
    end;

    // SZIMULÁCIÓ KEZELÉSE
    if (fulke_B.fekszelep.ertek < 0) then     // OLDÁS
    begin
      if (ALL_fekhengerNyomas > 0) then ALL_fekhengerNyomas := ALL_fekhengerNyomas - (Timegap * FEKKI_SEB * abs(fulke_B.fekszelep.ertek));
      
      HANG_B_fekOld := 1;
      HANG_B_fekUzem := -1;
      VOL_HANG_B_fekOld := abs(fulke_B.fekszelep.ertek);
      if (ALL_fekhengerNyomas = 100) then HANG_fekrudazat_ki := 1;
    end;
    
    if (fulke_B.fekszelep.ertek = 0) then     // ZÁRT
    begin
      VOL_HANG_B_fekUzem := 0;
      VOL_HANG_B_fekOld := 0;
      
      HANG_B_fekUzem := -1;
      HANG_B_fekOld := -1;
    end;
    
    if (fulke_B.fekszelep.ertek > 0) then     // ÜZEM vagy VÉSZFÉK
    begin
      if (ALL_fekhengerNyomas < ALL_fektartalyNyomas) then
      begin
        ALL_fekhengerNyomas := ALL_fekhengerNyomas + (Timegap * FEKBE_MAX * abs(fulke_B.fekszelep.ertek));
        ALL_fektartalyNyomas := ALL_fektartalyNyomas - (Timegap  *FEKBE_MAX/12 * abs(fulke_B.fekszelep.ertek));
      end;
      
      HANG_B_fekUzem := 1;
      HANG_B_fekOld := -1;
      VOL_HANG_B_fekUzem := abs(fulke_B.fekszelep.ertek);
      if (ALL_fekhengerNyomas = 100) then HANG_fekrudazat_be := 1;
    end;

    // ANIMÁCIÓ
    if (fulke_B.fekszelep.ertek < 0) then fulke_B.fekszelep.poz := Max(0.0, 58 * (fulke_B.fekszelep.ertek + 1.0))
    else if (fulke_B.fekszelep.ertek < 1) then fulke_B.fekszelep.poz := 58 + (85-58) * fulke_B.fekszelep.ertek
    else fulke_B.fekszelep.poz := Min(110.0, 85 + (110-85) * (fulke_B.fekszelep.ertek - 1.0));


    
    
    //------------------------------------------------------------------------------------------
    //KÉZIFÉK LEKÉPEZÉSE (A)
    if (fulke_A.kezifekCel = 1) then
    begin
      fulke_A.kezifek.ertek := fulke_A.kezifek.ertek + (Timegap*20); //ennyi % m�sodpercenk�nt
      if (fulke_A.kezifek.ertek > 100) then fulke_A.kezifek.ertek := 100;
    end;
    if (fulke_A.kezifekCel = -1) then
    begin
      fulke_A.kezifek.ertek := fulke_A.kezifek.ertek - (Timegap*25);
      if (fulke_A.kezifek.ertek < 0) then fulke_A.kezifek.ertek := 0;
    end;
    fulke_A.kezifek.poz := (fulke_A.kezifek.ertek * 15);
    
    //------------------------------------------------------------------------------------------
    //K�ZIF�K LEK�PEZ�SE (B)
    if (fulke_B.kezifekCel = 1) then
    begin
      fulke_B.kezifek.ertek := fulke_B.kezifek.ertek + (Timegap*20); //ennyi % m�sodpercenk�nt
      if (fulke_B.kezifek.ertek > 100) then fulke_B.kezifek.ertek := 100;
    end;
    if (fulke_B.kezifekCel = -1) then
    begin
      fulke_B.kezifek.ertek := fulke_B.kezifek.ertek - (Timegap*25);
      if (fulke_B.kezifek.ertek < 0) then fulke_B.kezifek.ertek := 0;
    end;
    fulke_B.kezifek.poz := (fulke_B.kezifek.ertek * 15);
    
    
    
    
    //------------------------------------------------------------------------------------------
    //L�GS�R�T� SZIMUL�L�SA
    // automatikusan kikapcsol 6 Bar felett, bekapcsol 4,5 Bar alatt (ha kapcsol� & vanHalozat)
    if ( (ALL_fektartalyNyomas < 450) and (vanHalozat) and (fulke_A.legsurito.ertek) and (not legsuritoJar) ) then
    begin
      legsuritoJar := true;
      HANG_legsuritoStart := 1;
      HANG_legsuritoLoop := 1;
      HANG_legsuritoLoopX := 1;
    end;
    
    if ( ( (ALL_fektartalyNyomas > 600) or (not vanHalozat) or (not fulke_A.legsurito.ertek)) and (legsuritoJar) ) then
    begin
      legsuritoJar := false;
      HANG_legsuritoStop := 1;
      HANG_legsuritoLoop := -1;
      HANG_legsuritoLoopX := -1;
    end;
    
    if (legsuritoJar) then ALL_fektartalyNyomas := ALL_fektartalyNyomas + (Timegap*7);
    
    
    
    
    
    //----------------------------------------------------------------------------------------------
    //ABLAKOK LEKEZEL�SE
    
    //BAL 2
    if (utaster.ablakB2_cel = -1) then utas_ablak_B2 := Min( 1.0, utas_ablak_B2 + Timegap*ABLAK_FEL_SEB);
    if (utaster.ablakB2_cel = 1) then utas_ablak_B2 := Max( 0.0, utas_ablak_B2 - Timegap*ABLAK_LE_SEB);
    //BAL 3
    if (utaster.ablakB3_cel = -1) then utas_ablak_B3 := Min( 1.0, utas_ablak_B3 + Timegap*ABLAK_FEL_SEB);
    if (utaster.ablakB3_cel = 1) then utas_ablak_B3 := Max( 0.0, utas_ablak_B3 - Timegap*ABLAK_LE_SEB);
    //BAL 4
    if (utaster.ablakB4_cel = -1) then utas_ablak_B4 := Min( 1.0, utas_ablak_B4 + Timegap*ABLAK_FEL_SEB);
    if (utaster.ablakB4_cel = 1) then utas_ablak_B4 := Max( 0.0, utas_ablak_B4 - Timegap*ABLAK_LE_SEB);
    
    //-------------------------------------------------------------------------------------------------------
    //JOBB 2
    if (utaster.ablakJ2_cel = -1) then utas_ablak_J2 := Min( 1.0, utas_ablak_J2 + Timegap*ABLAK_FEL_SEB);
    if (utaster.ablakJ2_cel = 1) then utas_ablak_J2 := Max( 0.0, utas_ablak_J2 - Timegap*ABLAK_LE_SEB);
    //JOBB 3
    if (utaster.ablakJ3_cel = -1) then utas_ablak_J3 := Min( 1.0, utas_ablak_J3 + Timegap*ABLAK_FEL_SEB);
    if (utaster.ablakJ3_cel = 1) then utas_ablak_J3 := Max( 0.0, utas_ablak_J3 - Timegap*ABLAK_LE_SEB);
    //JOBB 4
    if (utaster.ablakJ4_cel = -1) then utas_ablak_J4 := Min( 1.0, utas_ablak_J4 + Timegap*ABLAK_FEL_SEB);
    if (utaster.ablakJ4_cel = 1) then utas_ablak_J4 := Max( 0.0, utas_ablak_J4 - Timegap*ABLAK_LE_SEB);
    
    
    
    
    
    
    
    //------------------------------------------------------------------------------------------
    // V�LTOZ� �TM�SOL�SOK
    ALL_tularamvedo :=  fulke_A.tularamvedo.ertek OR fulke_B.tularamvedo.ertek;
    ALL_sinfek :=  fulke_A.sinfek.ertek OR fulke_B.sinfek.ertek;
    ALL_kezifek :=  max(fulke_A.kezifek.ertek, fulke_B.kezifek.ertek);
    
    
    if (fulke_A.aktiv) then ALL_menetkapcsolo := fulke_A.menetkapcsolo.ertek
    else if (fulke_B.aktiv) then ALL_menetkapcsolo := fulke_B.menetkapcsolo.ertek
    else ALL_menetkapcsolo := 0;
    
    if (fulke_A.aktiv) then ALL_iranyvalto := fulke_A.iranyvalto.ertek
    else if (fulke_B.aktiv) then ALL_iranyvalto := - fulke_B.iranyvalto.ertek
    else ALL_iranyvalto := 0;
    
    if (utas_berrego) then LM_utas_berrego := 1 else LM_utas_berrego := 0;


    //'A' F�LKE SPECIFIKUS DOLGOK
    A_CP_iranyvaltoBent := fulke_A.iranyvaltoBent;
    A_CP_menetkapcsoloBent := fulke_A.menetkapcsoloBent;
    A_CP_fekszelepBent := fulke_A.fekszelepBent;
    
    A_CP_iranyvalto := fulke_A.iranyvalto.poz;
    A_CP_menetkapcsolo := fulke_A.menetkapcsolo.poz;
    A_CP_fekszelep := fulke_A.fekszelep.poz;
    A_CP_kezifek := fulke_A.kezifek.poz;
    A_CP_sinfek := fulke_A.sinfek.poz;
    A_CP_tularamKapcsolo := fulke_A.tularamvedo.poz;
    A_CP_kotelCsomo := fulke_A.kotelCsomo;
    A_CP_kotelNemCsomo := fulke_A.kotelNemCsomo;
    
    A_CP_csengo := fulke_A.csengo.poz;
    A_CP_homokolo := fulke_A.homokolo.poz;
    A_CP_ajtoBalKapcsolo := fulke_A.ajtoBal.poz;
    A_CP_ajtoJobbKapcsolo := fulke_A.ajtoJobb.poz;
    A_CP_sugarvetoKapcsolo := fulke_A.sugarveto.poz;
    A_CP_legsuritoKapcsolo := fulke_A.legsurito.poz;
    A_CP_berregoBe := fulke_A.indjelzoBe.poz;
    A_CP_berregoKi := fulke_A.indjelzoKi.poz;
    
    A_CP_szamlalo1 := fulke_A.szamlalo1;
    A_CP_jottThrottle := fulke_A.jottThrottle;
    
    
    
    //'B' F�LKE SPECIFIKUS DOLGOK
    B_CP_iranyvaltoBent := fulke_B.iranyvaltoBent;
    B_CP_menetkapcsoloBent := fulke_B.menetkapcsoloBent;
    B_CP_fekszelepBent := fulke_B.fekszelepBent;
    
    B_CP_iranyvalto := fulke_B.iranyvalto.poz;
    B_CP_menetkapcsolo := fulke_B.menetkapcsolo.poz;
    B_CP_fekszelep := fulke_B.fekszelep.poz;
    B_CP_kezifek := fulke_B.kezifek.poz;
    B_CP_sinfek := fulke_B.sinfek.poz;
    B_CP_tularamKapcsolo := fulke_B.tularamvedo.poz;
    B_CP_kotelCsomo := fulke_B.kotelCsomo;
    B_CP_kotelNemCsomo := fulke_B.kotelNemCsomo;
    
    B_CP_csengo := fulke_B.csengo.poz;
    B_CP_homokolo := fulke_B.homokolo.poz;
    B_CP_ajtoBalKapcsolo := fulke_B.ajtoBal.poz;
    B_CP_ajtoJobbKapcsolo := fulke_B.ajtoJobb.poz;
    B_CP_sugarvetoKapcsolo := fulke_B.sugarveto.poz;
    B_CP_berregoBe := fulke_B.indjelzoBe.poz;
    B_CP_berregoKi := fulke_B.indjelzoKi.poz;
    
    //B_CP_szamlalo1 := fulke_B.szamlalo1;
    //B_CP_jottThrottle := fulke_B.jottThrottle;
    
    
    // UTAST�R V�LTOZ�I
    utas_vilagitasKapcsolo1 := utaster.K1.poz;
    utas_vilagitasKapcsolo2 := utaster.K2.poz;
    utas_vilagitasKapcsolo3 := utaster.K3.poz;
    
  end;
  
  
  
  //------------------------------------------------------------------------------
  
  procedure fulke_init;
  begin
    // Y tengely
    pattogas_init(kam_pattog_Y_0, 0.8, 10, 1);  // kitérés, freki, hossz (-exp)
    pattogas_init(kam_pattog_Y_1, 0.35, 2, 0.8);
    pattogas_init(kam_pattog_Y_2, 0.25, 4, 0.5);
    pattogas_init(kam_pattog_Y_3, 0.4, 3, 0.7);
    pattogas_init(kam_pattog_Y_4, 0.3, 12, 1.5);
    // X tengely
    pattogas_init(kam_pattog_X_0, 1.2, 6, 1);
    pattogas_init(kam_pattog_X_1, 0.8, 12, 0.8);
    pattogas_init(kam_pattog_X_2, 0.6, 7, 0.7);


    //FÉK INICIALIZ�L�SA
    ALL_fektartalyNyomas := 500;
    ALL_fekhengerNyomas := 0;
    legsuritoJar := false;
    
    menetfokozatElozo_A := 0;
    menetfokozatElozo_B := 0;
    
    railbrake_pos := 0;

    
    //SZ�ML�L�K NULL�Z�SA (L�SD ONBUTTON)
    fulke_A.szamlalo1 := 0;
    fulke_A.jottThrottle := -1;
    fulke_B.szamlalo1 := 0;
    fulke_B.jottThrottle := -1;
    
    
    // 'A' F�LKE INICIALIZ�L�SA
    fulke_A.ajtoBal.ertek := false;       fulke_A.ajtoBal.poz := 0;
    fulke_A.ajtoJobb.ertek := false;      fulke_A.ajtoJobb.poz := 0;
    fulke_A.csengo.ertek := false;        fulke_A.csengo.poz := 0;
    fulke_A.sugarveto.ertek := false;     fulke_A.sugarveto.poz := 0;
    fulke_A.legsurito.ertek := true;     fulke_A.legsurito.poz := 90;
    fulke_A.indjelzoBe.ertek := false;    fulke_A.indjelzoBe.poz := 0;
    fulke_A.indjelzoKi.ertek := false;    fulke_A.indjelzoKi.poz := 0;
    fulke_A.homokolo.ertek := false;       fulke_A.homokolo.poz := 0;
    fulke_A.sinfek.ertek := false;        fulke_A.sinfek.poz := 0;
    
    fulke_A.menetkapcsolo.ertek := 0;      fulke_A.menetkapcsolo.poz := MENETKAPCSOLO_0;
    fulke_A.iranyvalto.ertek := 0;         fulke_A.iranyvalto.poz := 0;
    fulke_A.fekszelep.ertek := 0;          fulke_A.fekszelep.poz := 58;       fek_A_cel := 0;   fek_A_mozog := false;
    fulke_A.kezifek.ertek := 100;          fulke_A.kezifek.poz :=  1500;
    
    
    //HA READY
    if(InitReadyForMovement > 1) then
    begin
      fulke_A.iranyvaltoBent := true;
      fulke_A.menetkapcsoloBent := true;
      fulke_A.fekszelepBent := true;
      
      fulke_A.tularamvedo.ertek := true;   fulke_A.tularamvedo.poz := 90;
    end;
    
    if(InitReadyForMovement = 2) then
    begin
      fulke_A.kezifek.ertek := 0;          fulke_A.kezifek.poz :=  0;
    end;
    
    //HA NEM READY
    if(InitReadyForMovement = 0) then
    begin
      fulke_A.iranyvaltoBent := false;
      fulke_A.menetkapcsoloBent := false;
      fulke_A.fekszelepBent := false;
      
      fulke_A.tularamvedo.ertek := false;   fulke_A.tularamvedo.poz := 0;
      
      
    end;
    
    /////////////////////////////////
    // 'B' F�LKE INICIALIZ�L�SA
    
    fulke_B.ajtoBal.ertek := false;       fulke_B.ajtoBal.poz := 0;
    fulke_B.ajtoJobb.ertek := false;      fulke_B.ajtoJobb.poz := 0;
    fulke_B.csengo.ertek := false;        fulke_B.csengo.poz := 0;
    fulke_B.sugarveto.ertek := false;     fulke_B.sugarveto.poz := 0;
    fulke_B.indjelzoBe.ertek := false;    fulke_B.indjelzoBe.poz := 0;
    fulke_B.indjelzoKi.ertek := false;    fulke_B.indjelzoKi.poz := 0;
    fulke_B.homokolo.ertek := false;      fulke_B.homokolo.poz := 0;
    fulke_B.sinfek.ertek := false;        fulke_B.sinfek.poz := 0;
    fulke_B.menetkapcsolo.ertek := 0;      fulke_B.menetkapcsolo.poz := MENETKAPCSOLO_0;
    fulke_B.fekszelep.ertek := 0;          fulke_B.fekszelep.poz := 58;   fek_B_cel := 0;   fek_B_mozog := false;
    fulke_B.kezifek.ertek := 0;            fulke_B.kezifek.poz :=  0;
    
    fulke_B.iranyvaltoBent := false;
    fulke_B.menetkapcsoloBent := false;
    fulke_B.fekszelepBent := false;
    fulke_B.aktiv := false;
    
    fulke_B.tularamvedo.ertek := false;   fulke_B.tularamvedo.poz := 0;
    fulke_B.iranyvalto.ertek := 0;    fulke_B.iranyvalto.poz := 0;
    
    
    
    
    /////////////////////////////////
    // UTAST�R INICIALIZ�L�SA
    utaster.K1.ertek := true;     utaster.K1.poz := -80;
    utaster.K2.ertek := false;    utaster.K2.poz := 0;
    utaster.K3.ertek := false;    utaster.K3.poz := 0;
    
    
    
  end;
  
  
  

  
  //------------------------------------------------------------------------------
  
  procedure fulke_OnButton(id: string; value: boolean; cockpitIndex: byte);
  begin
    
    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // 'A' F�LKE CUCCAI
    
    //----------------------------------------------------------------------------------------------
    // BEHELYEZ�SEK
    if value and (id = 'BKV1000_A_CP_iranyvaltoBe') and (not fulke_B.aktiv) and (not fulke_A.aktiv) then
    begin
      fulke_A.iranyvaltoBent := true;
      HANG_A_iranyvaltoBe := 1;
    end;
    if value and (id = 'BKV1000_A_CP_iranyvaltoKi') and (fulke_A.iranyvaltoBent) and (fulke_A.iranyvalto.ertek = 0) then
    begin
      fulke_A.iranyvaltoBent := false;
      HANG_A_iranyvaltoKi := 1;
    end;
    
    if value and (id = 'BKV1000_A_CP_menetkapcsoloBe') and (not fulke_B.aktiv) and (not fulke_A.aktiv)  then
    begin
      fulke_A.menetkapcsoloBent := true;
      HANG_A_menetkapcsoloBe := 1;
    end;
    if value and (id = 'BKV1000_A_CP_menetkapcsoloKi') and (fulke_A.menetkapcsoloBent) and (fulke_A.iranyvalto.ertek = 0)  then
    begin
      fulke_A.menetkapcsoloBent := false;
      HANG_A_menetkapcsoloKi := 1;
    end;
    
    if value and (id = 'BKV1000_A_CP_fekszelepBe') and (not fulke_B.aktiv) and (not fulke_A.aktiv) then
    begin
      fulke_A.fekszelepBent := true;
      HANG_A_fekszelepBe := 1;
    end;
    if value and (id = 'BKV1000_A_CP_fekszelepKi') and (fulke_A.fekszelepBent) and (fulke_A.iranyvalto.ertek = 0) and (fulke_A.fekszelep.ertek = 0) then
    begin
      fulke_A.fekszelepBent := false;
      HANG_A_fekszelepKi := 1;
    end;
    
    //----------------------------------------------------------------------------------------------
    //IR�NYV�LT�
    if value and (id = 'BKV1000_A_CP_iranyvaltoElore') and (fulke_A.aktiv) and (fulke_A.menetkapcsolo.ertek = 0) then
    begin
      if (fulke_A.iranyvalto.ertek < 1) then HANG_A_iranyvaltoElore := 1;
      fulke_A.iranyvalto.ertek := fulke_A.iranyvalto.ertek + 1;
      if (fulke_A.iranyvalto.ertek > 1) then fulke_A.iranyvalto.ertek := 1;
    end;
    
    if value and (id = 'BKV1000_A_CP_iranyvaltoHatra') and (fulke_A.aktiv) and (fulke_A.menetkapcsolo.ertek = 0) then
    begin
      if (fulke_A.iranyvalto.ertek > -1) then HANG_A_iranyvaltoHatra := 1;
      fulke_A.iranyvalto.ertek := fulke_A.iranyvalto.ertek - 1;
      if (fulke_A.iranyvalto.ertek < -1) then fulke_A.iranyvalto.ertek := -1;
    end;
    
    if value and (id = 'ReverserPlus') and (fulke_A.aktiv) and (cockpitIndex = 0) and (fulke_A.menetkapcsolo.ertek = 0) then
    begin
      if (fulke_A.iranyvalto.ertek < 1) then HANG_A_iranyvaltoElore := 1;
      fulke_A.iranyvalto.ertek := fulke_A.iranyvalto.ertek + 1;
      if (fulke_A.iranyvalto.ertek > 1) then fulke_A.iranyvalto.ertek := 1;
    end;
    
    if value and (id = 'ReverserMinus') and (fulke_A.aktiv) and (cockpitIndex = 0) and (fulke_A.menetkapcsolo.ertek = 0) then
    begin
      if (fulke_A.iranyvalto.ertek > -1) then HANG_A_iranyvaltoHatra := 1;
      fulke_A.iranyvalto.ertek := fulke_A.iranyvalto.ertek - 1;
      if (fulke_A.iranyvalto.ertek < -1) then fulke_A.iranyvalto.ertek := -1;
    end;
    
    if (fulke_A.iranyvalto.ertek = 1) then fulke_A.iranyvalto.poz := IRANYVALTO_ELORE;
    if (fulke_A.iranyvalto.ertek = 0) then fulke_A.iranyvalto.poz := 0;
    if (fulke_A.iranyvalto.ertek = -1) then fulke_A.iranyvalto.poz := IRANYVALTO_HATRA;
    
    
    //----------------------------------------------------------------------------------------------
    //MENETKAPCSOLÓ
    
    if value and (id = 'Throttle') and (fulke_A.aktiv) and (fulke_A.iranyvalto.ertek <> 0) then
    begin
      //kontrollerhez a v�ltoz� billent�se
      fulke_A.jottThrottle := 1;
    end;
    
    if not value and (id = 'Throttle') and (fulke_A.aktiv) and (fulke_A.iranyvalto.ertek <> 0) then
    begin
      fulke_A.jottThrottle := 0;
    end;
    
    if value and (id = 'Brake') and (fulke_A.aktiv) and (fulke_A.iranyvalto.ertek <> 0) then
    begin

      
      if (fulke_A.menetkapcsolo.ertek > 0) then HANG_A_menetkapcsolo_menet := 1
      else if (fulke_A.menetkapcsolo.ertek >-6) then HANG_A_menetkapcsolo_fek := 1
      else if (fulke_A.menetkapcsolo.ertek = -6) then HANG_A_menetkapcsolo_fekMax := 1;
      
      fulke_A.menetkapcsolo.ertek := fulke_A.menetkapcsolo.ertek - 1;
      if (fulke_A.menetkapcsolo.ertek < -7) then fulke_A.menetkapcsolo.ertek := -7;
    end;
    
    if value and (id = 'MaxBrake') and (fulke_A.aktiv) and (fulke_A.iranyvalto.ertek <> 0) then
    begin
      if (fulke_A.menetkapcsolo.ertek <> -7) then HANG_A_menetkapcsolo_fekMax := 1;
      fulke_A.menetkapcsolo.ertek := -7;      
    end;
    
    if value and (id = 'Neutral') and (fulke_A.aktiv) and (fulke_A.iranyvalto.ertek <> 0) then
    begin
      if (fulke_A.menetkapcsolo.ertek > 0) then HANG_A_menetkapcsolo_menetLe := 1
      else if (fulke_A.menetkapcsolo.ertek < 0) then HANG_A_menetkapcsolo_fekLe := 1;
      
      
      fulke_A.menetkapcsolo.ertek := 0;      
    end;
    
    fulke_MenetkapcsoloFrissites(fulke_A);
    
    
    
    //----------------------------------------------------------------------------------------------
    //FÉKSZELEP
    if value and (id = 'BKV1000_fekszelepMinusz') and (fulke_A.fekszelepBent) then
    begin
      fek_A_mozog := true;

      if (fek_A_cel = 0.0) then
      begin
        HANG_A_fekOld := 1; 
        fek_A_cel := -1;
      end;
      if (fek_A_cel > 0) then fek_A_cel := fek_A_cel - 1;
      
      HANG_A_fekszelep_minusz := 1;
    end;
    if not value and (id = 'BKV1000_fekszelepMinusz') and (fulke_A.fekszelepBent) then
    begin
      //if (fek_A_cel = -1) then fek_A_cel := 0;
    end;
    
    if value and (id = 'BKV1000_fekszelepPlusz') and (fulke_A.fekszelepBent) then
    begin
      fek_A_mozog := true;

      if (fek_A_cel = 0) then HANG_A_fekPlusz := 1; 
      if (fek_A_cel < 1) then fek_A_cel := fek_A_cel + 1;
      
      HANG_A_fekszelep_plusz := 1;   
    end;
        
    if value and (id = 'BKV1000_fekszelepVeszfek') and (fulke_A.fekszelepBent) then
    begin
      fek_A_mozog := true;

      if (fek_A_cel < 2) then
      begin
        fek_A_cel := 2;
        HANG_A_veszfek := 1;
      end;
      
      HANG_A_fekszelep_plusz := 1;
    end;
    
    
    
    
    //----------------------------------------------------------------------------------------------
    //KÉZIFÉK
    if value and (id = 'BKV1000_A_CP_kezifekPlusz') then
    begin
      fulke_A.kezifekCel := 1;
      if (fulke_A.kezifek.ertek > 99) then HANG_A_kezifek := -1 else HANG_A_kezifek := 1;
    end;
    if not value and (id = 'BKV1000_A_CP_kezifekPlusz') then
    begin
      fulke_A.kezifekCel := 0;
      HANG_A_kezifek := -1;
    end;
    
    if value and (id = 'BKV1000_A_CP_kezifekMinusz') then
    begin
      fulke_A.kezifekCel := -1;
      if (fulke_A.kezifek.ertek < 1) then HANG_A_kezifek := -1 else HANG_A_kezifek := 1;
    end;
    if not value and (id = 'BKV1000_A_CP_kezifekMinusz') then
    begin
      fulke_A.kezifekCel := 0;
      HANG_A_kezifek := -1;
    end;
    
    
    
    //----------------------------------------------------------------------------------------------
    //S�NF�K
    if value and (id = 'RailBrake') and (fulke_A.aktiv) then
    begin
      fulke_A.sinfek.ertek := true;
      HANG_A_sinfekkapcsolo_be := 1;
      HANG_sinfekBe_kint := 1;
      HANG_sinfekBe_bent := 1;
      
      railbrake_pos := -0.1;
    end;
    if not value and (id = 'RailBrake') and (fulke_A.aktiv) then
    begin
      fulke_A.sinfek.ertek := false;
      HANG_A_sinfekkapcsolo_ki := 1;
      HANG_sinfekKi_kint := 1;
      HANG_sinfekKi_bent := 1;
      
      railbrake_pos := 0;     
    end;
    
    if value and (id = 'BKV1000_A_CP_sinfek') and (fulke_A.aktiv) then
    begin
      fulke_A.sinfek.ertek := true;
      HANG_A_sinfekkapcsolo_be := 1;
      HANG_sinfekBe_kint := 1;
      HANG_sinfekBe_bent := 1;
      
      railbrake_pos := -0.1;     
    end;
    if not value and (id = 'BKV1000_A_CP_sinfek') and (fulke_A.aktiv) then
    begin
      fulke_A.sinfek.ertek := false;
      HANG_A_sinfekkapcsolo_ki := 1;
      HANG_sinfekKi_kint := 1;
      HANG_sinfekKi_bent := 1;
      
      railbrake_pos := 0;     
    end;
    
    if (fulke_A.sinfek.ertek) then fulke_A.sinfek.poz := -0.05;
    if (not fulke_A.sinfek.ertek) then fulke_A.sinfek.poz := 0;
    
    
    
    //----------------------------------------------------------------------------------------------
    //T�L�RAMV�D� KAPCSOL�
    if value and (id = 'BKV1000_A_CP_tularamvedo') then
    begin
      HANG_A_kapcsolo3 := 1;
      fulke_A.tularamvedo.ertek := not fulke_A.tularamvedo.ertek;
    end;
    if (fulke_A.tularamvedo.ertek) then fulke_A.tularamvedo.poz := 90
    else fulke_A.tularamvedo.poz := 0;
    
    
    
    
    //----------------------------------------------------------------------------------------------
    //L�GS�R�T� KAPCSOL�
    if value and (id = 'BKV1000_A_CP_legsuritoKapcsolo') then
    begin
      if (fulke_A.legsurito.ertek) then HANG_A_sugarKI := 1 else HANG_A_sugarBE := 1;
      fulke_A.legsurito.ertek := not fulke_A.legsurito.ertek;
    end;
    if (fulke_A.legsurito.ertek) then fulke_A.legsurito.poz := 80
    else fulke_A.legsurito.poz := 0;
    
   
    
    //----------------------------------------------------------------------------------------------
    //SUG�RVET� KAPCSOL�
    if value and (id = 'BKV1000_A_CP_sugarvetoKapcsolo') then
    begin
      if (fulke_A.sugarveto.ertek) then HANG_A_sugarKI := 1 else HANG_A_sugarBE := 1;
      fulke_A.sugarveto.ertek := not fulke_A.sugarveto.ertek;
    end;
    if (fulke_A.sugarveto.ertek) then fulke_A.sugarveto.poz := -80
    else fulke_A.sugarveto.poz := 0;
    
    
    
    //----------------------------------------------------------------------------------------------
    //BAL AJT�SZELEP
    if ( value and (id = 'BKV1000_A_CP_ajtoBalKapcsolo') ) or ( value and (id = 'BKV1000_ajtoBalKapcsolo') and fulke_A.aktiv) then
    begin
      if (fulke_A.ajtoBal.ertek) then HANG_A_ajtoszelepKI := 1 else HANG_A_ajtoszelepBE := 1;
      fulke_A.ajtoBal.ertek := not fulke_A.ajtoBal.ertek;
    end;
    if (fulke_A.ajtoBal.ertek) then fulke_A.ajtoBal.poz := 40
    else fulke_A.ajtoBal.poz := 0;
    
    //----------------------------------------------------------------------------------------------
    //JOBB AJT�SZELEP
    if ( value and (id = 'BKV1000_A_CP_ajtoJobbKapcsolo') ) or ( value and (id = 'BKV1000_ajtoJobbKapcsolo') and fulke_A.aktiv) then
    begin
      if (fulke_A.ajtoJobb.ertek) then HANG_A_ajtoszelepKI := 1 else HANG_A_ajtoszelepBE := 1;
      fulke_A.ajtoJobb.ertek := not fulke_A.ajtoJobb.ertek;
    end;
    if (fulke_A.ajtoJobb.ertek) then fulke_A.ajtoJobb.poz := 40
    else fulke_A.ajtoJobb.poz := 0;
    
    
    
    //----------------------------------------------------------------------------------------------
    //IND�T�SJELZ� - BE
    if (value and (id = 'BKV1000_inditasjelzo') ) or (value and (id = 'BKV1000_A_CP_inditasjelzoBe') ) then
    begin
      HANG_A_indjelzBE := 1;
      if (vanHalozat) then utas_berrego := true;
      fulke_A.indjelzoBe.ertek := true;
    end;
    if (not value and (id = 'BKV1000_inditasjelzo') ) or (not value and (id = 'BKV1000_A_CP_inditasjelzoBe') ) then fulke_A.indjelzoBe.ertek := false;
    if (fulke_A.indjelzoBe.ertek) then fulke_A.indjelzoBe.poz := -0.005
    else fulke_A.indjelzoBe.poz := 0;
    
    //IND�T�SJELZ� - KI
    if (value and (id = 'BKV1000_inditasjelzoKi')) or (value and (id = 'BKV1000_A_CP_inditasjelzoKi'))  then
    begin
      HANG_A_indjelzKI := 1;
      utas_berrego := false;
      fulke_A.indjelzoKi.ertek := true;
    end;
    if (not value and (id = 'BKV1000_inditasjelzoKi')) or (not value and (id = 'BKV1000_A_CP_inditasjelzoKi')) then fulke_A.indjelzoKi.ertek := false;
    if (fulke_A.menetkapcsolo.ertek > 1) then utas_berrego := false;
    if (fulke_A.indjelzoKi.ertek) then fulke_A.indjelzoKi.poz := -0.005
    else fulke_A.indjelzoKi.poz := 0;
    
    
    //----------------------------------------------------------------------------------------------
    //CSENG�
    if (value and (id = 'Bell1') and fulke_A.aktiv) or (value and (id = 'BKV1000_A_CP_csengo') ) then
    begin
      HANG_A_csengo := 1;
      fulke_A.csengo.ertek := true;
    end;
    if (not value and (id = 'Bell1') ) or (not value and (id = 'BKV1000_A_CP_csengo') ) then fulke_A.csengo.ertek := false;
    if (fulke_A.csengo.ertek) then fulke_A.csengo.poz := -0.03
    else fulke_A.csengo.poz := 0;
    
    
    
    //----------------------------------------------------------------------------------------------
    //HOMOKOL� PED�L
    if (value and (id = 'Sanding') and fulke_A.aktiv) or (value and (id = 'BKV1000_A_CP_homokolo') ) then
    begin
      HANG_A_homokBE := 1;
      fulke_A.homokolo.ertek := true;
      sanding_0_0 := true;
    end;
    
    if (not value and (id = 'Sanding') ) or (not value and (id = 'BKV1000_A_CP_homokolo') ) then
    begin
      HANG_A_homokKI := 1;
      fulke_A.homokolo.ertek := false;
      sanding_0_0 := false;
    end;
    
    if (fulke_A.homokolo.ertek) then fulke_A.homokolo.poz := -0.05
    else fulke_A.homokolo.poz := 0;
    
 
    
    
    
    
    
    
    
    
    
    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // 'B' F�LKE CUCCAI
    
    //----------------------------------------------------------------------------------------------
    // BEHELYEZ�SEK
    if value and (id = 'BKV1000_B_CP_iranyvaltoBe') and (not fulke_B.aktiv) and (not fulke_A.aktiv) then
    begin
      fulke_B.iranyvaltoBent := true;
      HANG_B_iranyvaltoBe := 1;
    end;
    if value and (id = 'BKV1000_B_CP_iranyvaltoKi') and (fulke_B.iranyvaltoBent) and (fulke_B.iranyvalto.ertek = 0) then
    begin
      fulke_B.iranyvaltoBent := false;
      HANG_B_iranyvaltoKi := 1;
    end;
    
    if value and (id = 'BKV1000_B_CP_menetkapcsoloBe') and (not fulke_B.aktiv) and (not fulke_A.aktiv)  then
    begin
      fulke_B.menetkapcsoloBent := true;
      HANG_B_menetkapcsoloBe := 1;
    end;
    if value and (id = 'BKV1000_B_CP_menetkapcsoloKi') and (fulke_B.menetkapcsoloBent) and (fulke_B.iranyvalto.ertek = 0)  then
    begin
      fulke_B.menetkapcsoloBent := false;
      HANG_B_menetkapcsoloKi := 1;
    end;
    
    if value and (id = 'BKV1000_B_CP_fekszelepBe') and (not fulke_B.aktiv) and (not fulke_A.aktiv) then
    begin
      fulke_B.fekszelepBent := true;
      HANG_B_fekszelepBe := 1;
    end;
    if value and (id = 'BKV1000_B_CP_fekszelepKi') and (fulke_B.fekszelepBent) and (fulke_B.iranyvalto.ertek = 0) and (fulke_B.fekszelep.ertek = 0) then
    begin
      fulke_B.fekszelepBent := false;
      HANG_B_fekszelepKi := 1;
    end;
    
    //----------------------------------------------------------------------------------------------
    //IR�NYV�LT�
    if value and (id = 'BKV1000_B_CP_iranyvaltoElore') and (fulke_B.aktiv) and (fulke_B.menetkapcsolo.ertek = 0) then
    begin
      if (fulke_B.iranyvalto.ertek < 1) then HANG_B_iranyvaltoElore := 1;
      fulke_B.iranyvalto.ertek := fulke_B.iranyvalto.ertek + 1;
      if (fulke_B.iranyvalto.ertek > 1) then fulke_B.iranyvalto.ertek := 1;
    end;
    
    if value and (id = 'BKV1000_B_CP_iranyvaltoHatra') and (fulke_B.aktiv) and (fulke_B.menetkapcsolo.ertek = 0) then
    begin
      if (fulke_B.iranyvalto.ertek > -1) then HANG_B_iranyvaltoHatra := 1;
      fulke_B.iranyvalto.ertek := fulke_B.iranyvalto.ertek - 1;
      if (fulke_B.iranyvalto.ertek < -1) then fulke_B.iranyvalto.ertek := -1;
    end;
    
    if value and (id = 'ReverserPlus') and (fulke_B.aktiv) and (cockpitIndex = 1) and (fulke_B.menetkapcsolo.ertek = 0) then
    begin
      if (fulke_B.iranyvalto.ertek < 1) then HANG_B_iranyvaltoElore := 1;
      fulke_B.iranyvalto.ertek := fulke_B.iranyvalto.ertek + 1;
      if (fulke_B.iranyvalto.ertek > 1) then fulke_B.iranyvalto.ertek := 1;
    end;
    
    if value and (id = 'ReverserMinus') and (fulke_B.aktiv) and (cockpitIndex = 1) and (fulke_B.menetkapcsolo.ertek = 0) then
    begin
      if (fulke_B.iranyvalto.ertek > -1) then HANG_B_iranyvaltoHatra := 1;
      fulke_B.iranyvalto.ertek := fulke_B.iranyvalto.ertek - 1;
      if (fulke_B.iranyvalto.ertek < -1) then fulke_B.iranyvalto.ertek := -1;
    end;
    
    if (fulke_B.iranyvalto.ertek = 1) then fulke_B.iranyvalto.poz := IRANYVALTO_ELORE;
    if (fulke_B.iranyvalto.ertek = 0) then fulke_B.iranyvalto.poz := 0;
    if (fulke_B.iranyvalto.ertek = -1) then fulke_B.iranyvalto.poz := IRANYVALTO_HATRA;
    
    
    //----------------------------------------------------------------------------------------------
    //MENETKAPCSOL�
    
    if value and (id = 'Throttle') and (fulke_B.aktiv) and (fulke_B.iranyvalto.ertek <> 0) then
    begin

      //kontrollerhez a v�ltoz� billent�se
      fulke_B.jottThrottle := 1;
    end;
    
    if not value and (id = 'Throttle') and (fulke_B.aktiv) and (fulke_B.iranyvalto.ertek <> 0) then
    begin
      fulke_B.jottThrottle := 0;
    end;
    
    if value and (id = 'Brake') and (fulke_B.aktiv) and (fulke_B.iranyvalto.ertek <> 0) then
    begin
      
      
      if (fulke_B.menetkapcsolo.ertek > 0) then HANG_B_menetkapcsolo_menet := 1
      else if (fulke_B.menetkapcsolo.ertek >-6) then HANG_B_menetkapcsolo_fek := 1
      else if (fulke_B.menetkapcsolo.ertek = -6) then HANG_B_menetkapcsolo_fekMax := 1;
      
      fulke_B.menetkapcsolo.ertek := fulke_B.menetkapcsolo.ertek - 1;
      if (fulke_B.menetkapcsolo.ertek < -7) then fulke_B.menetkapcsolo.ertek := -7;
    end;
    
    if value and (id = 'MaxBrake') and (fulke_B.aktiv) and (fulke_B.iranyvalto.ertek <> 0) then
    begin
      if (fulke_B.menetkapcsolo.ertek <> -7) then HANG_B_menetkapcsolo_fekMax := 1;
      fulke_B.menetkapcsolo.ertek := -7;      
    end;
    
    if value and (id = 'Neutral') and (fulke_B.aktiv) and (fulke_B.iranyvalto.ertek <> 0) then
    begin
      if (fulke_B.menetkapcsolo.ertek > 0) then HANG_B_menetkapcsolo_menetLe := 1
      else if (fulke_B.menetkapcsolo.ertek < 0) then HANG_B_menetkapcsolo_fekLe := 1;
      
      fulke_B.menetkapcsolo.ertek := 0;      
    end;
    
    fulke_MenetkapcsoloFrissites(fulke_B);
    
    
    
    //----------------------------------------------------------------------------------------------
    //FÉKSZELEP
    if value and (id = 'BKV1000_fekszelepMinusz') and (fulke_B.fekszelepBent) then
    begin
      fek_B_mozog := true;

      if (fek_B_cel = 0.0) then
      begin
        HANG_B_fekOld := 1; 
        fek_B_cel := -1;
      end;
      if (fek_B_cel > 0) then fek_B_cel := fek_B_cel - 1;
      
      HANG_B_fekszelep_minusz := 1;
    end;
    if not value and (id = 'BKV1000_fekszelepMinusz') and (fulke_B.fekszelepBent) then
    begin
      //if (fek_B_cel = -1) then fek_B_cel := 0;
    end;
    
    if value and (id = 'BKV1000_fekszelepPlusz') and (fulke_B.fekszelepBent) then
    begin
      fek_B_mozog := true;

      if (fek_B_cel = 0) then HANG_B_fekPlusz := 1; 
      if (fek_B_cel < 1) then fek_B_cel := fek_B_cel + 1;
      
      HANG_B_fekszelep_plusz := 1;   
    end;
        
    if value and (id = 'BKV1000_fekszelepVeszfek') and (fulke_B.fekszelepBent) then
    begin
      fek_B_mozog := true;

      if (fek_B_cel < 2) then
      begin
        fek_B_cel := 2;
        HANG_B_veszfek := 1;
      end;
      
      HANG_B_fekszelep_plusz := 1;
    end;
    
    
    
    
    //----------------------------------------------------------------------------------------------
    //K�ZIF�K
    if value and (id = 'BKV1000_B_CP_kezifekPlusz') then
    begin
      fulke_B.kezifekCel := 1;
      if (fulke_B.kezifek.ertek > 99) then HANG_B_kezifek := -1 else HANG_B_kezifek := 1;
    end;
    if not value and (id = 'BKV1000_B_CP_kezifekPlusz') then
    begin
      fulke_B.kezifekCel := 0;
      HANG_B_kezifek := -1;
    end;
    
    if value and (id = 'BKV1000_B_CP_kezifekMinusz') then
    begin
      fulke_B.kezifekCel := -1;
      if (fulke_B.kezifek.ertek < 1) then HANG_B_kezifek := -1 else HANG_B_kezifek := 1;
    end;
    if not value and (id = 'BKV1000_B_CP_kezifekMinusz') then
    begin
      fulke_B.kezifekCel := 0;
      HANG_B_kezifek := -1;
    end;
    
    
    
    //----------------------------------------------------------------------------------------------
    //S�NF�K
    if value and (id = 'RailBrake') and (fulke_B.aktiv) then
    begin
      fulke_B.sinfek.ertek := true;
      HANG_B_sinfekkapcsolo_be := 1;
      HANG_sinfekBe_kint := 1;
      HANG_sinfekBe_bent := 1;
      
      railbrake_pos := -0.1;
    end;
    if not value and (id = 'RailBrake') and (fulke_B.aktiv) then
    begin
      fulke_B.sinfek.ertek := false;
      HANG_B_sinfekkapcsolo_ki := 1;
      HANG_sinfekKi_kint := 1;
      HANG_sinfekKi_bent := 1;
      
      railbrake_pos := 0;     
    end;
    
    if value and (id = 'BKV1000_B_CP_sinfek') and (fulke_B.aktiv) then
    begin
      fulke_B.sinfek.ertek := true;
      HANG_B_sinfekkapcsolo_be := 1;
      HANG_sinfekBe_kint := 1;
      HANG_sinfekBe_bent := 1; 
      
      railbrake_pos := -0.1;    
    end;
    if not value and (id = 'BKV1000_B_CP_sinfek') and (fulke_B.aktiv) then
    begin
      fulke_B.sinfek.ertek := false;
      HANG_B_sinfekkapcsolo_ki := 1;
      HANG_sinfekKi_kint := 1;
      HANG_sinfekKi_bent := 1;
      
      railbrake_pos := 0;     
    end;
    
    if (fulke_B.sinfek.ertek) then fulke_B.sinfek.poz := -0.05;
    if (not fulke_B.sinfek.ertek) then fulke_B.sinfek.poz := 0;
    
    
    
    //----------------------------------------------------------------------------------------------
    //T�L�RAMV�D� KAPCSOL�
    if value and (id = 'BKV1000_B_CP_tularamvedo') then
    begin
      HANG_B_kapcsolo3 := 1;
      fulke_B.tularamvedo.ertek := not fulke_B.tularamvedo.ertek;
    end;
    if (fulke_B.tularamvedo.ertek) then fulke_B.tularamvedo.poz := 90
    else fulke_B.tularamvedo.poz := 0;
    
    
    //----------------------------------------------------------------------------------------------
    //SUG�RVET� KAPCSOL�
    if value and (id = 'BKV1000_B_CP_sugarvetoKapcsolo') then
    begin
      if (fulke_B.sugarveto.ertek) then HANG_B_sugarKI := 1 else HANG_B_sugarBE := 1;
      fulke_B.sugarveto.ertek := not fulke_B.sugarveto.ertek;
    end;
    if (fulke_B.sugarveto.ertek) then fulke_B.sugarveto.poz := -80
    else fulke_B.sugarveto.poz := 0;
    
    
    
    //----------------------------------------------------------------------------------------------
    //BAL AJT�SZELEP (AMI IGAZ�B�L A JOBB OLDAL)
    if ( value and (id = 'BKV1000_B_CP_ajtoBalKapcsolo') ) or ( value and (id = 'BKV1000_ajtoBalKapcsolo') and fulke_B.aktiv) then
    begin
      if (fulke_B.ajtoBal.ertek) then HANG_B_ajtoszelepKI := 1 else HANG_B_ajtoszelepBE := 1;
      fulke_B.ajtoBal.ertek := not fulke_B.ajtoBal.ertek;
    end;
    if (fulke_B.ajtoBal.ertek) then fulke_B.ajtoBal.poz := 40
    else fulke_B.ajtoBal.poz := 0;
    
    //----------------------------------------------------------------------------------------------
    //JOBB AJT�SZELEP (AMI IGAZ�B�L A BAL OLDAL)
    if ( value and (id = 'BKV1000_B_CP_ajtoJobbKapcsolo') ) or ( value and (id = 'BKV1000_ajtoJobbKapcsolo') and fulke_B.aktiv) then
    begin
      if (fulke_B.ajtoJobb.ertek) then HANG_B_ajtoszelepKI := 1 else HANG_B_ajtoszelepBE := 1;
      fulke_B.ajtoJobb.ertek := not fulke_B.ajtoJobb.ertek;
    end;
    if (fulke_B.ajtoJobb.ertek) then fulke_B.ajtoJobb.poz := 40
    else fulke_B.ajtoJobb.poz := 0;
    
    
    
    //----------------------------------------------------------------------------------------------
    //IND�T�SJELZ� - BE
    if (value and (id = 'BKV1000_inditasjelzo') ) or (value and (id = 'BKV1000_B_CP_inditasjelzoBe') ) then
    begin
      HANG_B_indjelzBE := 1;
      if (vanHalozat) then utas_berrego := true;
      fulke_B.indjelzoBe.ertek := true;
    end;
    if (not value and (id = 'BKV1000_inditasjelzo') ) or (not value and (id = 'BKV1000_B_CP_inditasjelzoBe') ) then fulke_B.indjelzoBe.ertek := false;
    if (fulke_B.indjelzoBe.ertek) then fulke_B.indjelzoBe.poz := -0.005
    else fulke_B.indjelzoBe.poz := 0;
    
    //IND�T�SJELZ� - KI
    if (value and (id = 'BKV1000_inditasjelzoKi')) or (value and (id = 'BKV1000_B_CP_inditasjelzoKi'))  then
    begin
      HANG_B_indjelzKI := 1;
      utas_berrego := false;
      fulke_B.indjelzoKi.ertek := true;
    end;
    if (not value and (id = 'BKV1000_inditasjelzoKi')) or (not value and (id = 'BKV1000_B_CP_inditasjelzoKi')) then fulke_B.indjelzoKi.ertek := false;
    if (fulke_B.menetkapcsolo.ertek > 1) then utas_berrego := false;
    if (fulke_B.indjelzoKi.ertek) then fulke_B.indjelzoKi.poz := -0.005
    else fulke_B.indjelzoKi.poz := 0;
    
    
    //----------------------------------------------------------------------------------------------
    //CSENG�
    if (value and (id = 'Bell1') and fulke_B.aktiv) or (value and (id = 'BKV1000_B_CP_csengo') ) then
    begin
      HANG_B_csengo := 1;
      fulke_B.csengo.ertek := true;
    end;
    if (not value and (id = 'Bell1') ) or (not value and (id = 'BKV1000_B_CP_csengo') ) then fulke_B.csengo.ertek := false;
    if (fulke_B.csengo.ertek) then fulke_B.csengo.poz := -0.03
    else fulke_B.csengo.poz := 0;
    
    
    
    //----------------------------------------------------------------------------------------------
    //HOMOKOL� PED�L
    if (value and (id = 'Sanding') and fulke_B.aktiv) or (value and (id = 'BKV1000_B_CP_homokolo') ) then
    begin
      HANG_B_homokBE := 1;
      fulke_B.homokolo.ertek := true;
      sanding_0_0 := true;
    end;
    
    if (not value and (id = 'Sanding') ) or (not value and (id = 'BKV1000_B_CP_homokolo') ) then
    begin
      HANG_B_homokKI := 1;
      fulke_B.homokolo.ertek := false;
      sanding_0_0 := false;
    end;
    
    if (fulke_B.homokolo.ertek) then fulke_B.homokolo.poz := -0.05
    else fulke_B.homokolo.poz := 0;
    
    
    
    
    
    
    
    
    
    
    
    //----------------------------------------------------------------------------------------------------------------------
    // F�LKE F�GGETLEN CUCCOK
    
    //----------------------------------------------------------------------------------------------
    //�RAMSZED� FEL/LE
    if value and (id = 'PantographUp') then
    begin
      //HANG_A_csengo := 1;
      ALL_szedo := true;
    end;
    if value and (id = 'PantographDn') then
    begin
      //HANG_A_csengo := 1;
      ALL_szedo := false;
    end;
    
    //----------------------------------------------------------------------------------------------
    //�RAMSZED� �TFORD�T�SA
    if value and (id = 'BKV1000_szedoAtforditas') and ALL_szedo then
    begin
      ALL_szedo_irany := not ALL_szedo_irany;
    end;
    
    
    
    
    //----------------------------------------------------------------------------------------------
    // F�NYSZ�R� KAPCSOL�S
    // HA NINCS BEKAPCSOLVA A VIL F�KAP, BEKAPCSOL AZ (HELYZETJELZ�), UT�NA KAPCSOL BE AZ ADOTT OLDALI F��NYSZ�R�
    
    if value and (id = 'FrontLightPlus') then
    begin
			//------------------//
      // HA A F�KAP KI VAN KAPCSOLVA
      if (utaster.K1.ertek = false) then
      begin
				if (utaster.K1.ertek) then HANG_K1_KI := 1 else HANG_K1_BE := 1;
				utaster.K1.ertek := not utaster.K1.ertek;
				if (utaster.K1.ertek) then utaster.K1.poz := -90 else utaster.K1.poz := 0;
      end
      
      //---------------//
			//HA A F�KAP BE VAN KAPCSOLVA, AZ ADOTT OLDALI F�NYSZ�R� KAPCSOL
			else
			begin
				//HA AZ 'A' F�LK�BEN VAGYUNK
				if (cockpitIndex = 0) then
				begin
					if (fulke_A.sugarveto.ertek) then HANG_A_sugarKI := 1 else HANG_A_sugarBE := 1;
					fulke_A.sugarveto.ertek := not fulke_A.sugarveto.ertek;
					if (fulke_A.sugarveto.ertek) then fulke_A.sugarveto.poz := -80 else fulke_A.sugarveto.poz := 0;
				end;
				
				//HA AZ 'B' F�LK�BEN VAGYUNK
				if (cockpitIndex = 1) then
				begin
					if (fulke_B.sugarveto.ertek) then HANG_B_sugarKI := 1 else HANG_B_sugarBE := 1;
					fulke_B.sugarveto.ertek := not fulke_B.sugarveto.ertek;
					if (fulke_B.sugarveto.ertek) then fulke_B.sugarveto.poz := -80 else fulke_B.sugarveto.poz := 0;
				end;
						
			end;	//f�kap ki felt�tel v�ge
    end;	//frontlightplus v�ge
    
    
    
    
    
    //----------------------------------------------------------------------------------------------
    // F�NYSZ�R� KIKAPCSOL�S
    // EL�SZ�R AZ ADOTT OLDALI F�NYSZ�R� KAPCSOL KI (HELYZETJELZ�RE), UT�NA KAPCSOL KI A VIL F�KAPCSOL�
    
    if value and (id = 'FrontLightMinus') then
    begin
			//-----------------------//
			// HA B�RMELYIK F�NYSZ�R� BE VAN KAPCSOLVA, AZT KAPCSOLJUK LE
			if (fulke_A.sugarveto.ertek or fulke_B.sugarveto.ertek) then
			begin
				//HA AZ 'A' F�LK�BEN VAGYUNK
				if (cockpitIndex = 0) then
				begin
					if (fulke_A.sugarveto.ertek) then HANG_A_sugarKI := 1 else HANG_A_sugarBE := 1;
					fulke_A.sugarveto.ertek := not fulke_A.sugarveto.ertek;
					if (fulke_A.sugarveto.ertek) then fulke_A.sugarveto.poz := -80 else fulke_A.sugarveto.poz := 0;
				end;
				
				//HA AZ 'B' F�LK�BEN VAGYUNK
				if (cockpitIndex = 1) then
				begin
					if (fulke_B.sugarveto.ertek) then HANG_B_sugarKI := 1 else HANG_B_sugarBE := 1;
					fulke_B.sugarveto.ertek := not fulke_B.sugarveto.ertek;
					if (fulke_B.sugarveto.ertek) then fulke_B.sugarveto.poz := -80 else fulke_B.sugarveto.poz := 0;
				end;
			end
			
			
			
			//------------------//
      // HA A F�NYSZ�R�K KI VANNAK KAPCSOLVA, A VIL F�KAPOT NYOMJUK LE
      else
      begin
				if (utaster.K1.ertek) then HANG_K1_KI := 1 else HANG_K1_BE := 1;
				utaster.K1.ertek := not utaster.K1.ertek;
				if (utaster.K1.ertek) then utaster.K1.poz := -90 else utaster.K1.poz := 0;
      end;//f�nysz�r� felt�tel v�ge
    end;	//frontlightminus v�ge
    
    

    
    
  end;	//fulke_onButton v�ge
  
  
  
  
  
  
  
  procedure utaster_OnButton(id: string; value: boolean; cockpitIndex: byte);
  begin
    
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // UTAST�R KAPCSOL�K
    
    //----------------------------------------------------------------------------------------------
    // Vil�g�t�s f�kapcsol�
    if value and (id = 'BKV1000_vilagitas_K1') then
    begin
      if (utaster.K1.ertek) then HANG_K1_KI := 1 else HANG_K1_BE := 1;
      utaster.K1.ertek := not utaster.K1.ertek;
    end;
    if (utaster.K1.ertek) then utaster.K1.poz := -90
    else utaster.K1.poz := 0;
    
    //----------------------------------------------------------------------------------------------
    // Vil�g�t�s szakaszkapcsol� (helyzetjelz�, z�rf�ny, vil�g�t�s, ellent�tes peronvil., sz�mjelz�)
    if value and (id = 'BKV1000_vilagitas_K2') then
    begin
      if (utaster.K2.ertek) then HANG_K1_KI := 1 else HANG_K1_BE := 1;
      utaster.K2.ertek := not utaster.K2.ertek;
    end;
    if (utaster.K2.ertek) then utaster.K2.poz := -90
    else utaster.K2.poz := 0;
    
    //----------------------------------------------------------------------------------------------
    // z�rf�ny ir�nyv�laszt�
    if value and (id = 'BKV1000_vilagitas_K3') then
    begin
      if (utaster.K3.ertek) then HANG_K1_KI := 1 else HANG_K1_BE := 1;
      utaster.K3.ertek := not utaster.K3.ertek;
    end;
    if (utaster.K3.ertek) then utaster.K3.poz := -90
    else utaster.K3.poz := 0;
    
    
    
    
    
    
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // ABLAKTEKER�K
    
    //----------------------------------------------------------------------------------------------
    // BALOLDAL
    
    if value and (id = 'BKV1000_ablaktekero_B2_Fel') then
    begin
      if (utas_ablak_B2 <= 1.0) then HANG_ablakBAL_FEL := 1;
      utaster.ablakB2_cel := 1;
    end;
    if not value and (id = 'BKV1000_ablaktekero_B2_Fel') then
    begin
      HANG_ablakBAL_FEL := -1;
      utaster.ablakB2_cel := 0;
    end;
    //---------------
    if value and (id = 'BKV1000_ablaktekero_B2_Le') then
    begin
      if (utas_ablak_B2 >= 0.0) then HANG_ablakBAL_LE := 1;
      utaster.ablakB2_cel := -1;
    end;
    if not value and (id = 'BKV1000_ablaktekero_B2_Le') then
    begin
      HANG_ablakBAL_LE := -1;
      utaster.ablakB2_cel := 0;
    end;
    //------------------------------------------------------------------------------
    if value and (id = 'BKV1000_ablaktekero_B3_Fel') then
    begin
      if (utas_ablak_B3 <= 1.0) then HANG_ablakBAL_FEL := 1;
      utaster.ablakB3_cel := 1;
    end;
    if not value and (id = 'BKV1000_ablaktekero_B3_Fel') then
    begin
      HANG_ablakBAL_FEL := -1;
      utaster.ablakB3_cel := 0;
    end;
    //---------------
    if value and (id = 'BKV1000_ablaktekero_B3_Le') then
    begin
      if (utas_ablak_B3 >= 0.0) then HANG_ablakBAL_LE := 1;
      utaster.ablakB3_cel := -1;
    end;
    if not value and (id = 'BKV1000_ablaktekero_B3_Le') then
    begin
      HANG_ablakBAL_LE := -1;
      utaster.ablakB3_cel := 0;
    end;
    //------------------------------------------------------------------------------
    if value and (id = 'BKV1000_ablaktekero_B4_Fel') then
    begin
      if (utas_ablak_B4 <= 1.0) then HANG_ablakBAL_FEL := 1;
      utaster.ablakB4_cel := 1;
    end;
    if not value and (id = 'BKV1000_ablaktekero_B4_Fel') then
    begin
      HANG_ablakBAL_FEL := -1;
      utaster.ablakB4_cel := 0;
    end;
    //---------------
    if value and (id = 'BKV1000_ablaktekero_B4_Le') then
    begin
      if (utas_ablak_B4 >= 0.0) then HANG_ablakBAL_LE := 1;
      utaster.ablakB4_cel := -1;
    end;
    if not value and (id = 'BKV1000_ablaktekero_B4_Le') then
    begin
      HANG_ablakBAL_LE := -1;
      utaster.ablakB4_cel := 0;
    end;
    
    
    //----------------------------------------------------------------------------------------------
    // JOBB OLDAL
    
    if value and (id = 'BKV1000_ablaktekero_J2_Fel') then
    begin
      if (utas_ablak_J2 <= 1.0) then HANG_ablakJOBB_FEL := 1;
      utaster.ablakJ2_cel := 1;
    end;
    if not value and (id = 'BKV1000_ablaktekero_J2_Fel') then
    begin
      HANG_ablakJOBB_FEL := -1;
      utaster.ablakJ2_cel := 0;
    end;
    //---------------
    if value and (id = 'BKV1000_ablaktekero_J2_Le') then
    begin
      if (utas_ablak_J2 >= 0.0) then HANG_ablakJOBB_LE := 1;
      utaster.ablakJ2_cel := -1;
    end;
    if not value and (id = 'BKV1000_ablaktekero_J2_Le') then
    begin
      HANG_ablakJOBB_LE := -1;
      utaster.ablakJ2_cel := 0;
    end;
    //------------------------------------------------------------------------------
    if value and (id = 'BKV1000_ablaktekero_J3_Fel') then
    begin
      if (utas_ablak_J3 <= 1.0) then HANG_ablakJOBB_FEL := 1;
      utaster.ablakJ3_cel := 1;
    end;
    if not value and (id = 'BKV1000_ablaktekero_J3_Fel') then
    begin
      HANG_ablakJOBB_FEL := -1;
      utaster.ablakJ3_cel := 0;
    end;
    //---------------
    if value and (id = 'BKV1000_ablaktekero_J3_Le') then
    begin
      if (utas_ablak_J3 >= 0.0) then HANG_ablakJOBB_LE := 1;
      utaster.ablakJ3_cel := -1;
    end;
    if not value and (id = 'BKV1000_ablaktekero_J3_Le') then
    begin
      HANG_ablakJOBB_LE := -1;
      utaster.ablakJ3_cel := 0;
    end;
    //------------------------------------------------------------------------------
    if value and (id = 'BKV1000_ablaktekero_J4_Fel') then
    begin
      if (utas_ablak_J4 <= 1.0) then HANG_ablakJOBB_FEL := 1;
      utaster.ablakJ4_cel := 1;
    end;
    if not value and (id = 'BKV1000_ablaktekero_J4_Fel') then
    begin
      HANG_ablakJOBB_FEL := -1;
      utaster.ablakJ4_cel := 0;
    end;
    //---------------
    if value and (id = 'BKV1000_ablaktekero_J4_Le') then
    begin
      if (utas_ablak_J4 >= 0.0) then HANG_ablakJOBB_LE := 1;
      utaster.ablakJ4_cel := -1;
    end;
    if not value and (id = 'BKV1000_ablaktekero_J4_Le') then
    begin
      HANG_ablakJOBB_LE := -1;
      utaster.ablakJ4_cel := 0;
    end;

  end;
  
  
  
  
  procedure fulke_OnFloatInput(id: string; value: single; cockpitIndex: byte);
  begin
		
    
		//----------------------------------------------------------------------------------------------
    //IRÁNYVÁLTÓ (A)
    if (id = 'Reverser') and (fulke_A.aktiv) and (fulke_A.iranyvaltoBent) then
    begin
			if (value < 0.40) then
      begin
				fulke_A.iranyvalto.ertek := -1;
				fulke_A.iranyvalto.poz := IRANYVALTO_HATRA;
      end
      else if (value > 0.50) then
      begin
				fulke_A.iranyvalto.ertek := 1;
				fulke_A.iranyvalto.poz := IRANYVALTO_ELORE;
      end
      else
      begin
				fulke_A.iranyvalto.ertek := 0;
				fulke_A.iranyvalto.poz := 0;
      end;
    end;

		//----------------------------------------------------------------------------------------------
    //MENETKAPCSOLÓ (A)
    
    if (id = 'Throttle') and (fulke_A.aktiv) and (fulke_A.iranyvalto.ertek <> 0) then
    begin
			menetfokozatElozo_A := fulke_A.menetkapcsolo.ertek;
      
      if (value < 0.05) then
      begin
				fulke_A.menetkapcsolo.ertek := 0;
				fulke_A.menetkapcsolo.poz := MENETKAPCSOLO_0;
      end;
      
      if (value > 0.05) then
      begin
				
				//tényleges kontroller állás (11 menet fokozat)
				fulke_A.menetkapcsolo.ertek := Round(value * 11);
				//fulke_A.menetkapcsolo.poz := value * MENETKAPCSOLO_m11;
				fulke_MenetkapcsoloFrissites(fulke_A);
      end;
      
      //HANGOK KEZELÉSE (előző állapottal összehasonlítva)
      if (fulke_A.menetkapcsolo.ertek > menetfokozatElozo_A) then HANG_A_menetkapcsolo_menet := 1
      else if (fulke_A.menetkapcsolo.ertek < menetfokozatElozo_A) then HANG_A_menetkapcsolo_menetLe := 1;
    end;
    
    
    if (id = 'Brake') and (fulke_A.aktiv) and (fulke_A.iranyvalto.ertek <> 0) then
    begin
			menetfokozatElozo_A := fulke_A.menetkapcsolo.ertek;
			
			if (value < 0.05) then
      begin
				fulke_A.menetkapcsolo.ertek := 0;
				fulke_A.menetkapcsolo.poz := MENETKAPCSOLO_0;
      end;
      
      if (value > 0.05) then
      begin
				//tényleges kontroller állás (7 fék fokozat)
				fulke_A.menetkapcsolo.ertek := -Round(value * 7);
				//fulke_A.menetkapcsolo.poz := value * MENETKAPCSOLO_f7;
				fulke_MenetkapcsoloFrissites(fulke_A);
      end;
      
      //HANGOK KEZELÉSE (előző állapottal összehasonlítva)
      if (fulke_A.menetkapcsolo.ertek < menetfokozatElozo_A) then
				if (fulke_A.menetkapcsolo.ertek = -7) then 
					HANG_A_menetkapcsolo_fekMax := 1
						else 
					HANG_A_menetkapcsolo_fek := 1
      else if (fulke_A.menetkapcsolo.ertek > menetfokozatElozo_A) then HANG_A_menetkapcsolo_fekLe := 1;
    end;
    
    //----------------------------------------------------------------------------------------------
    //IRÁNYVÁLTÓ (B)
    if (id = 'Reverser') and (fulke_B.aktiv) and (fulke_B.iranyvaltoBent) then
    begin
			if (value < 0.40) then
      begin
				fulke_B.iranyvalto.ertek := -1;
				fulke_B.iranyvalto.poz := IRANYVALTO_HATRA;
      end
      else if (value > 0.60) then
      begin
				fulke_B.iranyvalto.ertek := 1;
				fulke_B.iranyvalto.poz := IRANYVALTO_ELORE;
      end
      else
      begin
				fulke_B.iranyvalto.ertek := 0;
				fulke_B.iranyvalto.poz := 0;
      end;
    end;
    
    //----------------------------------------------------------------------------------------------
    //MENETKAPCSOLÓ (B)
    
    if (id = 'Throttle') and (fulke_B.aktiv) and (fulke_B.iranyvalto.ertek <> 0) then
    begin
			menetfokozatElozo_B := fulke_B.menetkapcsolo.ertek;
      
      if (value < 0.05) then
      begin
				fulke_B.menetkapcsolo.ertek := 0;
				fulke_B.menetkapcsolo.poz := MENETKAPCSOLO_0;
      end;
      
      if (value > 0.05) then
      begin				
				//tényleges kontroller állás (11 menet fokozat)
				fulke_B.menetkapcsolo.ertek := Round(value * 11);
				//fulke_B.menetkapcsolo.poz := value * MENETKAPCSOLO_m11;
				fulke_MenetkapcsoloFrissites(fulke_B);
      end;
      
      //HANGOK KEZELÉSE (előző állapottal összehasonlítva)
      if (fulke_B.menetkapcsolo.ertek > menetfokozatElozo_B) then HANG_B_menetkapcsolo_menet := 1
      else if (fulke_B.menetkapcsolo.ertek < menetfokozatElozo_B) then HANG_B_menetkapcsolo_menetLe := 1;
    end;
    
    
    if (id = 'Brake') and (fulke_B.aktiv) and (fulke_B.iranyvalto.ertek <> 0) then
    begin
			menetfokozatElozo_B := fulke_B.menetkapcsolo.ertek;
			
			if (value < 0.05) then
      begin
				fulke_B.menetkapcsolo.ertek := 0;
				fulke_B.menetkapcsolo.poz := MENETKAPCSOLO_0;
      end;
      
      if (value > 0.05) then
      begin
				//tényleges kontroller állás (7 fék fokozat)
				fulke_B.menetkapcsolo.ertek := -Round(value * 7);
				//fulke_B.menetkapcsolo.poz := value * MENETKAPCSOLO_f7;
				fulke_MenetkapcsoloFrissites(fulke_B);
      end;
      
      //HANGOK KEZELÉSE (előző állapottal összehasonlítva)
      if (fulke_B.menetkapcsolo.ertek < menetfokozatElozo_B) then
				if (fulke_B.menetkapcsolo.ertek = -7) then 
					HANG_B_menetkapcsolo_fekMax := 1
						else 
					HANG_B_menetkapcsolo_fek := 1
      else if (fulke_B.menetkapcsolo.ertek > menetfokozatElozo_B) then HANG_B_menetkapcsolo_fekLe := 1;
    end;



    //----------------------------------------------------------------------------------------------
    //FÉKSZELEP (A)
    if (id = 'Airbrake') and (fulke_A.aktiv) and (fulke_A.fekszelepBent) then
    begin
      if (value < 0.45) then fulke_A.fekszelep.ertek := -1.0 + ((1.0 / 0.45) * value)
      else if (value > 0.55) then fulke_A.fekszelep.ertek := (2.0 / 0.45) * (value - 0.55)
      else fulke_A.fekszelep.ertek := 0.0;
    end;
    
    //----------------------------------------------------------------------------------------------
    //FÉKSZELEP (B)
    if (id = 'Airbrake') and (fulke_B.aktiv) and (fulke_B.fekszelepBent) then
    begin
      if (value < 0.45) then fulke_B.fekszelep.ertek := -1.0 + ((1.0 / 0.45) * value)
      else if (value > 0.55) then fulke_B.fekszelep.ertek := (2.0 / 0.45) * (value - 0.55)
      else fulke_B.fekszelep.ertek := 0.0;
    end;

  end;