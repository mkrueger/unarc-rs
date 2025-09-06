Program HypTest;
{$IfDef Windows}
Uses
  WinCrt;
{$EndIf}
Type
  TDataReader = Function (aPointer: Pointer; BufLen: Word): Word;
  TDataWriter = Procedure (aPointer: Pointer; BufLen: Word);
Var
  fRead,
  fWrite: File;

  CPR_Seg1, CPR_Seg2: Word;

{$L COMM.OBJ}
{$L PACK.OBJ}
  Procedure Encode(Reader: TDataReader; Writer: TDataWriter);
    Far; External;
{$L UNPACK.OBJ}
  Procedure Decode(Reader: TDataReader; Writer: TDataWriter);
    Far; External;

  Function ReadUnCompressedData  (Buffer: Pointer; Len: Word): Word;
  Far;
  Begin
    Write('Lesen unkompr Bytes ...');
    BlockRead(fRead, Buffer^, Len, Len);
    WriteLn('geliefert: ', Len);
    ReadUnCompressedData:= Len
  End;

  Function ReadCompressedData    (Buffer: Pointer; Len: Word): Word;
  Far;
  Begin
    Write('angefordert: ', Len, ' kompr. Bytes...');
    BlockRead(fRead, Buffer^, Len, Len);
    WriteLn('geliefert: ', Len);
    ReadCompressedData:= Len
  End;

  Procedure WriteUnCompressedData (Buffer: Pointer; Len: Word);
  Far;
  Begin
    WriteLn('schreibe: ', Len, ' unkompr. Bytes');
    BlockWrite(fWrite, Buffer^, Len, Len);
  End;

  Procedure WriteCompressedData   (Buffer: Pointer; Len: Word);
  Far;
  Begin
    WriteLn('schreibe: ', Len, ' kompr. Bytes');
    BlockWrite(fWrite, Buffer^, Len, Len);
  End;

Procedure GetSegs;
Var
  aPtr: Pointer;
  aRec: Record l,h: Word End Absolute aPtr;
Begin
  GetMem(aPtr, 65535); CPR_Seg1:= aRec.h;
  GetMem(aPtr, 65535); CPR_Seg2:= aRec.h;
End;

Begin
  If ParamCount<>3 Then Halt(10);
  Assign(fRead, ParamStr(2)); Reset(fRead, 1);
  Assign(fWrite, ParamStr(3)); ReWrite(fWrite, 1);

  GetSegs;

  If ParamStr(1)='c' Then Begin
    WriteLn('Compressing ...');
    Encode(ReadUnCompressedData, WriteCompressedData);
  End Else Begin
    WriteLn('DeCompressing ...');
    Decode(ReadCompressedData, WriteUnCompressedData);
  End;
  WriteLn('Done.')
End.
