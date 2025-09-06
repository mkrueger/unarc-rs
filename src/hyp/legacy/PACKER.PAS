unit Packer;

interface

uses
  SysUtils, WinTypes, WinProcs, Messages, Classes, Graphics, Controls,
  Forms;

Type
  TPacker = class(TComponent)
  private
    { Private declarations }
    fS1,fS2: Word;
  protected
    { Protected declarations }
    Function GetS1: Word;
    Function GetS2: Word;
    Property S1: Word Read GetS1 Write fS1;
    Property S2: Word Read GetS2 Write fS2;
    Destructor Destroy;
  public
    { Public declarations }
    Function Reader (Buffer: Array Of Byte; Len: Word): Word; Virtual;
    Procedure Writer (Buffer: Array Of Byte; Len: Word); Virtual;
    Procedure Compress;
    Procedure Decompress;
  published
    { Published declarations }
  end;

procedure Register;

implementation

Type
  TDataReader = Function (Buffer: Array Of Byte; Len: Word): Word Of Object;
  TDataWriter = Procedure (Buffer: Array Of Byte; Len: Word) Of Object;

{$L COMM.OBJ}
{$L PACK.OBJ}
Procedure Encode(Reader: TDataReader; Writer: TDataWriter; Seg1, Seg2: Word);
  Far; External;
{$L UNPACK.OBJ}
Procedure Decode(Reader: TDataReader; Writer: TDataWriter; Seg1, Seg2: Word);
  Far; External;

Destructor TPacker.Destroy;
Begin
  {S1 deallocate}
  {S2 deallocate}
  Inherited Destroy
End;

Function TPacker.Reader (Buffer: Array Of Byte; Len: Word): Word;
Begin
  Write('Lesen unkompr Bytes ...');
  {BlockRead(fRead, Buffer^, Len, Len);}
  WriteLn('geliefert: ', Len);
  Reader:= Len
End;

Procedure TPacker.Writer (Buffer: Array Of Byte; Len: Word);
Begin
  {BlockWrite(fRead, Buffer^, Len, Len);}
End;

Function TPacker.GetS1: Word;
Begin
  If fS1=0 Then
    S1:= HiWord(LongInt(GlobalAllocPtr(GHnd, 65536)));
  Result:= fS1
End;

Function TPacker.GetS2: Word;
Begin
  If fS2=0 Then
    S2:= HiWord(LongInt(GlobalAllocPtr(GHnd, 65536)));
  Result:= fS2
End;

Procedure TPacker.Compress;
Begin
  Encode(Reader, Writer, S1, S2)
End;

Procedure TPacker.DeCompress;
Begin
  Decode(Reader, Writer, S1, S2)
End;

Procedure Register;
Begin
  RegisterComponents('Custom', [TPacker]);
End;

End.
