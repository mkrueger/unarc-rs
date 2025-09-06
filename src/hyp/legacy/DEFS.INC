;File Defs.Inc

           Ideal
           Jumps
           Model Large, Pascal

bitpufferlaenge = 8192
StrIndBufLen  = 8191
UnComprSize = 2500

Macro fillword _offset,w,anzahl    ;  for i:=0 to anzahl-1 do
                                   ;    (ES:_offset)[i]:=w
              Cld
              Mov Ax,w
              Mov Cx,anzahl
              Mov Di,_Offset
              rep stosw
EndM

Macro EDHeader PName
Proc PName Far aReader: QWord, aWriter: QWord, aSeg1: Word, aSeg2: Word
Public PName
  Assume Ds: DGroup, Es: Nothing
  Mov Es,[aSeg1]
  Mov [Es:CallersDS],Ds
  Mov Es,[aSeg2]
  Mov Ds,[aSeg1] ; aDataSeg
  Assume Ds: aDataSeg

  Mov Ax,[Word aReader]
  Mov [Word Reader], Ax
  Mov Ax,[Word aReader+2]
  Mov [Word Reader+2],Ax
  Mov Ax,[Word aReader+4]
  Mov [Word Reader+4],Ax
  Mov Ax,[Word aReader+6]
  Mov [Word Reader+6],Ax
  Mov Ax,[Word aWriter]
  Mov [Word Writer], Ax
  Mov Ax,[Word aWriter+2]
  Mov [Word Writer+2],Ax
  Mov Ax,[Word aWriter+4]
  Mov [Word Writer+4],Ax
  Mov Ax,[Word aWriter+6]
  Mov [Word Writer+6],Ax
EndM

Segment aDataSeg Private Para
  StrIndBuf         DW StrIndBufLen DUP (?)

  bbytepos          DW ?
  bitpos            DW ?
  bitpuffer         DB bitpufferlaenge+2 DUP (?)

  UNION tSharedUnion      ; maken eine Typ fr eine šberlagerung...
    ;  Huffman-Parameter
    huffsize    = 3200
    ;             ^^^^ >= StrIndBufLen div (maxfreq+1)+1+(maxdiff-mindiff+1)+(maxfreq+1)
    Struc
      Vater     DW 2*huffsize+1 DUP (?)
      Sohn      DW 2*huffsize+1 DUP (?)
      TheFreq   DW 2*huffsize+1 DUP (?)
    EndS
    Struc
      HashPos   DW StrIndBufLen DUP (?)
      newindex  DW StrIndBufLen DUP (?)
    EndS
  EndS tSharedUnion
  Shared tSharedUnion <>

  CallersDS dw ?            ; Datensegment des Aufrufer's
  ; alle Variablen ins DatenSegment verlagert
  ; (wg. Windows und sauberer Programmierung)
;  modus             DW ?
  max_bitpufpos     DW ?
  TestStrings_Index DW ?
  Low_tsi           DW ?
  UnComprData       Db UnComprSize Dup (?)
  UnComprPtr        Dw ?  ; pointer to UnComprData (nur bei read)
  UnComprEnd        Dw ?  ; pointer to end of UnComprData (bei read und write)
  ; --- huffman
  HuffMax       dw ?
  lastposition  dw ?
  maxlocal      dw ?
  maxlocal255   dw ?                      ; maxlocal+2*255
  local_offset  dw ?                      ; diff_offset+(maxdiff-mindiff+1)
  pos_offset    dw ?                      ; local_offset+(maxlocal+1)
  char_offset   dw ?                      ; pos_offset+(maxfreq+1)

  Reader    dd ?,?          ; Leseroutine des Aufrufer's
  Writer    dd ?,?          ; Leseroutine des Aufrufer's
EndS aDataSeg

;---- hashing
free        = 1                         ;  Markierung freier Eintr„ge in der Hash-Tabelle

maxhash = 16384    ;*** 16384

Segment HashSeg Private Para
  hasharray     DW maxhash DUP (?)
  hashvalue     DW StrIndBufLen DUP (?)
  rehashvalue   DW StrIndBufLen DUP (?)
EndS HashSeg

;------------------- Huffman
;
mindiff         = -4
maxdiff         = 8
maxfreq         = 2
lsequence_key   = 0
diff_offset     = 1                     ;  reserve  1 Char for sequences

maxrecfrequency = 4096                  ;  Huffman will be updated up to this frequency
tabsize         = StrIndBufLen+200;
;                              ^^^  >= 1+(maxdiff-mindiff+1)+(maxfreq+1)

Segment TheHuffES Private Para
  ;  Index-Tabelle
  nindex     DW StrIndBufLen+1 DUP (?)
  nvalue     DW tabsize+1        DUP (?)
  frequencys DW tabsize+1        DUP (?)

  ;  Freq-Array
  nfreq         DW maxfreq+1        DUP (?)
  HuffMaxindex DW ?

  ;  Teil des dynamischen Huffman
  nfreqmax   DW maxrecfrequency  DUP (?)
EndS TheHuffES

Macro DebugBreak
ifdef Debug
  Int 3
endif
EndM

