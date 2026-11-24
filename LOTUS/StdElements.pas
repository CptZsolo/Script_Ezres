const
  PI = 3.1415926535897932384626433832795;
  TWOPI = 6.283185307179586476925286766559;
                       
type
  TVector3 = record
    x,y,z: single;
  end;  
  
function SafeDiv(lhs, rhs, elsevalue: single): single;
begin
  if rhs = 0 then
    result := elsevalue
  else
    result := lhs/rhs;
end;
  
procedure ExponentialApproach(var value: single; exponent, target: single);
begin
  value := (1 - exp(Timegap * -exponent)) * (target - value) + value; 
end;

function BoolToFloat(b: boolean): single;
begin
  if b then
    result := 1
  else
    result := 0;
end;

function FloatToBool(f: single): boolean;
begin
  result := (f > 0.5);
end;

function Sqr(s: single): single;
begin
  result := s * s;
end;

function Sign(s: single): single;
begin
  if s > 0 then
    result := 1
  else if s < 0 then  
    result := -1
  else
    result := 0;
end;

type
  TDelayBool = record
    Delay: single;
    Input: boolean;
    Output: boolean;
    Timer: single;
  end;

procedure DelayBoolSetInputSimStep(var d: TDelayBool; newinput: boolean);
begin 
  if d.Input <> newinput then
  begin
    d.Timer := d.Delay;
    d.Input := newinput;
  end;
  if d.Timer < 0 then
    d.Output := d.Input
  else
    d.Timer := d.Timer - Timegap;
end;

type
  TDelayInt = record
    Delay: single;
    Input: integer;
    Output: integer;
    Timer: single;
  end;

procedure DelayIntSetInputSimStep(var d: TDelayInt; newinput: integer);
begin 
  if d.Input <> newinput then
  begin
    d.Timer := d.Delay;
    d.Input := newinput;
  end;
  if d.Timer < 0 then
    d.Output := d.Input
  else
    d.Timer := d.Timer - Timegap;
end;
  
function DegToRad(deg: single): single;
begin
  result := deg * PI / 180;
end;
  
function RadToDeg(rad: single): single;
begin
  result := rad * 180 / PI;
end;
  
function Vector3(x,y,z: single): TVector3;
begin
  result.x := x;
  result.y := y;
  result.z := z;
end;  

function AddVector3(a, b: TVector3): TVector3;
begin
  result.x := a.x + b.x;
  result.y := a.y + b.y;
  result.z := a.z + b.z;
end;

function SubtractVector3(a,b: TVector3): TVector3;
begin
  result.x := a.x - b.x;
  result.y := a.y - b.y;
  result.z := a.z - b.z;
end;

function RotateVector3X(vec: TVector3; angle_rad: single): TVector3;
begin                                                                
  result.x := vec.x;                                                
  result.y := vec.y * cos(angle_rad) - vec.z * sin(angle_rad);
  result.z := vec.z * cos(angle_rad) + vec.y * sin(angle_rad);
end;

function RotateVector3Y(vec: TVector3; angle_rad: single): TVector3;
begin
  result.x := vec.x * cos(angle_rad) + vec.z * sin(angle_rad);
  result.y := vec.y;
  result.z := vec.z * cos(angle_rad) - vec.x * sin(angle_rad);
end;  

function RotateVector3Z(vec: TVector3; angle_rad: single): TVector3;
begin                                              
  result.x := vec.x * cos(angle_rad) - vec.y * sin(angle_rad);
  result.y := vec.y * cos(angle_rad) + vec.x * sin(angle_rad);  
  result.z := vec.z;  
end;


//Saját hülyeségek
function LogikaiValtas(b: boolean): boolean;
begin
  if b then
    result := false;
    
  if not b then
    result := true;
end;
