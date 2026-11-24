//===================================================================================================================================================
//                Switches
//===================================================================================================================================================


//---------------------------------------------------
// Switch Off-On (not springloaded)

type
  TSwitch = record
    Pos: single;
    Value: boolean;
    Broken: boolean;
  end;
  
procedure Switch_Toggle(var sw: TSwitch);
begin
  sw.Pos := 1 - sw.Pos;
  if not sw.Broken then
    sw.Value := FloatToBool(sw.Pos);
end;

//---------------------------------------------------
// Pushbutton Off-On (springloaded)

type
  TPushbutton = record
    Pos: single;
    Value: boolean;
    Broken: boolean;
  end;
  
procedure Pushbutton_Press(var sw: TPushbutton; key: boolean);
begin
  if key then
    sw.Pos := 1
  else
    sw.Pos := 0;
  if not sw.Broken then
    sw.Value := FloatToBool(sw.Pos);
end;  

//---------------------------------------------------
// Step switch (no spring load) a,... -1, 0, 1 ,... b

type
  TStepSwitch = record
    A,B: shortint;
    Pos: shortint;
    Value: shortint;
    Broken: boolean;
    MinusSpring: boolean;
  end;
  
function StepSwitch_Plus(var sw: TStepSwitch; key: boolean): boolean;               // Result = switch changed pos
begin
  result := false;
  if key then
  begin
    if sw.Pos < sw.B then
    begin
      inc(sw.Pos);
      result := true;
    end;
    if not sw.Broken then    
      sw.Value := sw.Pos;
  end;
end;

function StepSwitch_Minus(var sw: TStepSwitch; key: boolean): boolean;              // Result = switch changed pos (only if not springloaded)
begin
  result := false;
  if key and (sw.A < sw.Pos) then
  begin
    dec(sw.Pos);
    result := true;
  end
  else if (not key) and (sw.Pos < 0) and sw.MinusSpring then
    sw.Pos := 0;
  if not sw.Broken then    
    sw.Value := sw.Pos;
end;

function StepSwitch_Set(var sw: TStepSwitch; newvalue: shortint): boolean;
begin
  result := sw.Pos <> newvalue; 
  
  if result then
  begin
    sw.Pos := newvalue;  
    if not sw.Broken then    
      sw.Value := sw.Pos;
  end;
end; 

procedure StepSwitch_SetElse(var sw: TStepSwitch; newvalue: shortint; elseValue: shortint);
begin
  if newvalue = sw.Pos then
    newvalue := elseValue; 
  
  sw.Pos := newvalue;  
  if not sw.Broken then    
    sw.Value := sw.Pos;
end;

//---------------------------------------------------
// Turn Switch spring loaded On (-1) - Off - On (1)

type
  TTurnSwitchTwoDirSpringloaded = record
    Pos: shortint;
    Value: shortint;
    Broken: boolean;
  end;
  
procedure TurnSwitchTwoDirSpringloaded_Plus(var sw: TTurnSwitchTwoDirSpringloaded; key: boolean);
begin
  if key then
    sw.Pos := 1
  else
    sw.Pos := 0;
  if not sw.Broken then
    sw.Value := sw.Pos;
end;

procedure TurnSwitchTwoDirSpringloaded_Minus(var sw: TTurnSwitchTwoDirSpringloaded; key: boolean);
begin
  if key then
    sw.Pos := -1
  else
    sw.Pos := 0;
  if not sw.Broken then 
    sw.Value := sw.Pos;
end;

//---------------------------------------------------
// Pushbutton with on/off function. Double deep while changing (pushing)

type
  TPushbtnInOut = record
    Pos: single;
    PrevKey: boolean;
    Value: boolean;
  end;
  
procedure PushbtnInOut_Set(var PB: TPushbtnInOut; key: boolean);
begin
  if key then
  begin
    if not PB.PrevKey then
      PB.Value := not PB.Value;
    PB.Pos := 2;
  end
  else if PB.Value then
    PB.Pos := 1
  else
    PB.Pos := 0;                   
  PB.PrevKey := key;                                                               
end;

procedure PushbtnInOut_SetValue(var PB: TPushbtnInOut; value: boolean);
begin
  PB.Value := value;
  if PB.Value then      
    PB.Pos := 1
  else
    PB.Pos := 0;  
end;

//---------------------------------------------------
// Handle (on/off) (springloaded)

type
  THandleSpringloaded = record
    Pos: single;
    Value: boolean;
  end;
  
procedure HandleSpringloaded_Press(var sw: THandleSpringloaded; key: boolean);
begin
  if key then
    sw.Pos := 1
  else
    sw.Pos := 0;
  sw.Value := FloatToBool(sw.Pos);
end; 


//===================================================================================================================================================
//                Lights, Blinkers, etc.
//===================================================================================================================================================

//---------------------------------------------------
// Blinkrelais

type
  TBlinkRelais = record
    Interval, OnTime: single;
    Timer: single;
    IsOn: boolean;
    ResetTime: single;
  end;
  
function BlinkRelais_SimStep(var relais: TBlinkRelais): integer;      // 1 = off => on, -1 = on => off
var
  newon: boolean;
begin
  relais.Timer := relais.Timer + Timegap;
  if relais.Timer > relais.Interval then
    relais.Timer := relais.Timer - relais.Interval;
  newon := relais.Timer < relais.OnTime;
  if newon and not relais.IsOn then
    result := 1
  else if not newon and relais.IsOn then
    result := -1
  else
    result := 0; 
  relais.IsOn := newon;
end;

procedure BlinkRelais_Reset(var relais: TBlinkRelais);
begin
  relais.Timer := relais.ResetTime;
  relais.IsOn := false;
end;

//---------------------------------------------------
// Light bulb

type
  TLightBulb = record
    onOffSpeed: single;
    light: single;
    broken: boolean;
  end;

procedure LightBulb_SimStep(var lightbulb: TLightBulb; target: boolean; voltage: single);
begin
  ExponentialApproach(lightbulb.light, lightbulb.onOffSpeed, voltage * BoolToFloat(target and (not lightbulb.broken)));
end;

//===================================================================================================================================================
//                Movables
//===================================================================================================================================================

type
  TSlider = record
    state: single;
    grabbing: boolean;
  end;

function Slider_SimStep_GetBumpSpeed(var slider: TSlider; grappingdelta: single): single;      // < 0: speed before bumping lower, > 0: speed before bumping higher
begin
  with slider do
  begin
    if grabbing then
    begin
      state := state + grappingdelta;
      if state > 1.0 then
      begin
        state := 1.0;
        result := grappingdelta/timegap;
      end
      else if state < 0.0 then
      begin
        state := 0.0;
        result := grappingdelta/timegap;
      end
      else
        result := 0;
    end;
  end;
end;

type
  TInertionSlider = record
    state: single;
    speed: single;
    lowerBumpFactor: single;               // < 0: no bump at all
    higherBumpFactor: single;
    lowerBumpPos: single;
    higherBumpPos: single;
    friction: single;
    grabbing: boolean;
    force: single;
  end;
  
function InertionSlider_SimStep_GetBumpSpeed(var slider: TInertionSlider; grappingdelta: single): single;      // < 0: speed before bumping lower, > 0: speed before bumping higher
var
  newspeed: single;
begin
  with slider do
  begin
    result := 0;
  
    if grabbing then
    begin
      state := state + grappingdelta;
      speed := grappingdelta/timegap;
    end
    else
    begin
      state := state + speed * timegap;
    end;                
    
    if state < lowerBumpPos then
    begin
      state := lowerBumpPos;
      result := speed;
      if lowerBumpFactor > 0 then
        speed := -lowerBumpFactor * speed
      else
        speed := 0;
    end;
    if state > higherBumpPos then
    begin
      state := higherBumpPos;
      result := speed;
      if higherBumpFactor > 0 then
        speed := -higherBumpFactor * speed
      else
        speed := 0;
    end;
    speed := speed + force * Timegap;  
    if speed <> 0 then
    begin
      newspeed := speed + (-sign(speed) * friction) * Timegap;
      
      if newspeed * speed < 0 then
        speed := 0
      else
        speed := newspeed;
    end;
  end;
end;
                            
//===================================================================================================================================================
//                Doors
//===================================================================================================================================================


type
  TElectricSlidingPlugDoorPair = record
    // Konstanten:
    PlugRadius, Shift: single;
    Friction: single;
    OpenStartSpeed,
    OpenEndSpeed,
    OpenStartEndChangePos,
    CloseStartSpeed,            // > 0
    CloseEndSpeed,
    CloseStartEndChangePos: single;
    TractionStiftness: single;
    ReflectionOpen, ReflectionClose: single;
    
    // Zustandsvariablen:
    Pos: single;
    Speed: single;
    XPosRail: single;
    YPosBladeA, YPosBladeB: single;
    Closed: boolean;
    
    // Soundvariablen:
    SoundMode: integer;       // 0 = keine Soundänderung, 1 = Open start, 2 = open end, 3 = close start, 4 = close trans, 5 = close end
    
    // Steuerung:
    Target: boolean;
    Energy: boolean;
  end;
  
procedure ElectricSlidingPlugDoorPair_SimStep(var door: TElectricSlidingPlugDoorPair);
var
  a: single;
  v_soll: single;
  arc, newspeed, newpos, shiftpos: single;
begin
  door.SoundMode := 0;

  if  (door.Speed <> 0) or
      (door.Target and (door.Pos < 1)) or
      (not door.Target and (door.Pos > 0)) then
  begin
    if not door.Energy then
    begin
      if door.Speed > 0 then
        a := -door.Friction                        
      else if door.Speed < 0 then
        a := door.Friction
      else
        a := 0;
    end
    else
    begin
      if door.Target then
      begin
        if (door.Pos < 0.01) and (door.Speed <= 0) then
          door.SoundMode := 1;
        if door.Pos < door.OpenStartEndChangePos then
          v_soll := door.OpenStartSpeed
        else
          v_soll := door.OpenEndSpeed;
      end
      else
      begin
        if (door.Pos > 0.1) and (door.Speed >= 0) then
          door.SoundMode := 3;      
        if door.Pos > door.CloseStartEndChangePos then
          v_soll := -door.CloseStartSpeed
        else
          v_soll := -door.CloseEndSpeed;
      end;    
      a := (v_soll - door.Speed) * door.TractionStiftness;
    end;                 
            
    newspeed := door.Speed + Timegap * a;
    if (newspeed * door.Speed) < 0 then
      newspeed := 0;
    door.Speed := newspeed;
    
    newpos := door.Pos + door.Speed * Timegap;
    
    if (newpos < 0.1) and (door.Pos >= 0.1) then
      door.SoundMode := 4;
    if (newpos < 0.01) and (door.Pos >= 0.01) then
      door.SoundMode := 5;      
    
    if newpos > 1 then                                                                                      
    begin
      newpos := 1;
      newspeed := -door.Speed + door.ReflectionOpen;
      if newspeed * door.Speed > 0 then
        newspeed := 0;
      door.Speed := newspeed;
      door.SoundMode := 2;
    end
    else if newpos < 0 then
    begin
      newpos := 0;
      newspeed := -door.Speed - door.ReflectionClose;
      if newspeed * door.Speed > 0 then
        newspeed := 0;
      door.Speed := newspeed;
    end;
    
    door.Pos := newpos;
    
    if door.Pos < 0.1 then
    begin
      arc := door.Pos * 5 * PI;                
      door.XPosRail := sin(arc) * door.PlugRadius;
      shiftpos := (1-cos(arc)) * door.PlugRadius;
    end
    else
    begin    
      door.XPosRail := door.PlugRadius;
      shiftpos := (door.Pos - 0.1)/0.9 * door.Shift + door.PlugRadius;
    end;    
    door.YPosBladeA := shiftpos;
    door.YPosBladeB := -shiftpos;
  end;      
  door.closed := (not door.Target) and (door.Pos < 0.01);
end;