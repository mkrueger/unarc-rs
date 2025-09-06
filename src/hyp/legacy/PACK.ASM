; File Pack.Asm
Include Defs.Inc

CodeSeg

EDHeader Encode
             Mov [UnComprPtr],Offset UnComprData
             Mov [UnComprEnd],Offset UnComprData

             Mov [bbytepos],Offset bitpuffer        ;  bbytepos:=0
             Mov [bitpos],0008h                     ;  bitpos:=8
             Mov [Low_tsi],2*255                    ;  Low_tsi:=255

@@repeat:    Call FindDuplicates
             Call WriteSmart
             Call ClearWhenFull
             Cmp [teststrings_index],255
             Jne @@repeat

             Mov Cx,[bitpos]
             Cmp Cl,8
             Je @@1
             Mov Cx,8
             Xor Bx,Bx
             Call wBits                       ; 8 Nullbits schreiben
@@1:         Call WriteCompressedData
             Mov Ds,[CallersDS]
             Ret
EndP Encode

Assume Ds: aDataSeg

Macro ReadUnCompressedData
        Push Ax Bx Cx Dx Si Di Es Ds
        Mov Ax,Offset UnComprData
        Mov Bx,UnComprSize
        Mov Dx,Ds
        Mov Es,Dx
        Mov Di, Offset Reader
        Mov Ds,[CallersDS]
        ; Es:Di+6/4 is @Self
        Call [DWord Es:Di] Pascal, Dx, Ax, Bx, [Word Es:Di+6], [Word Es:Di+4]
        Pop Ds
        Add Ax,Offset UnComprData
        Mov [UnComprEnd],Ax
        Mov [UnComprPtr],Offset UnComprData
        Pop Es Di Si Dx Cx Bx Ax
EndM

Proc WriteCompressedData Near
        Push Ax Bx Cx Dx Si Di Es Ds
        Mov Ax,Offset bitpuffer
        Mov Bx,[bbytepos]
        Sub Bx,Ax
        Mov Dx,Ds
        Mov Es,Dx
        Mov Di, Offset Writer
        Mov Ds,[CallersDS]
        Call [DWord Es:Di] Pascal, Dx, Ax, Bx, [Word Es:Di+6], [Word Es:Di+4]
        Pop Ds Es Di Si Dx Cx Bx Ax
        Ret
EndP WriteCompressedData

Assume Es: HashSeg

Macro calc_hashval LabArg
                                       ;  (hv,Cx)=hashfun(Ax,Dx)
                                       ;  (HashValue,reHashValue)[Di]:=(hv,Cx)
             Mov Bx,Ax               ;    hashval[Di]:=  hashval[Ax]+  hashval[Dx]
             Mov Cx,[reHashValue+Bx];  rehashval[Di]:=rehashval[Ax]ñrehashval[Dx]
             Mov  Si,[HashValue+Bx]
             Mov Bx,Dx
             Add  Si,[HashValue+Bx]
             test Cx,1H
             Mov Bx,[reHashValue+Bx]
             je LabArg
             neg Bx
LabArg:      Add Cx,Bx
             Mov [HashValue+Di], Si
             Mov [reHashValue+Di],Cx
EndM

Macro HashInit
Local @@loop, @@riloop, @@for, @@endfor, @@repeat
             fillword <Offset HashArray>,free,maxhash
; Initialisierung der Hash-Werte #0..#255
             Mov bx,31415
             Mov cx,256
             Mov ax,1
             Xor Di,Di
@@loop:      mul bx
             Mov [HashValue+Di],ax
             Add Di,2
             loop @@loop
             Mov [HashValue+2*256],0

; Initialisierung der ReHash-Werte #0..#255
             Mov bx,27183
             Mov cx,256
             Mov ax,1
             Mov Di,2*255

@@riloop:    Mul Bx
             Mov [reHashValue+Di],ax
             Sub Di,2
             loop @@riloop

             Mov [reHashValue+2*256],0

;  Positionen StrIndBuf[255]..StrIndBuf[Low_tsi-1]
;  in die Hash-Tabelle aufnehmen
             Mov Di,2*255
             Mov [Word Ptr StrIndBuf+Di-2],0
             Mov Ax,[StrIndBuf+Di]         ;  Ax:=StrIndBuf[255]
@@for:       Add Di,2                  ;  for i:=256 to Low_tsi-1 do
             Cmp Di,[Low_tsi]
             jae @@endfor
             Mov Dx,[StrIndBuf+Di]         ;      Dx:=StrIndBuf[i]
             calc_hashval LabelX
             Mov Ax,Dx

             Or Cx,2                  ; first hash ==> ggT(Cx,2)=1
@@repeat:    Add Si,Cx                    ; repeat hv=(hv+Cx) mod maxhash
             and Si,2*(maxhash-1)          ;   (lineares Re-Hashing)
             Cmp [HashArray+Si],free
             jne @@repeat                  ; until (Bx=HashArray[hv])==free

             Mov [Di+Shared.HashPos],Si           ;      Shared.HashPos[Di]:=HashValue
             Mov [HashArray+Si],Di         ;      HashArray[HashValue]:=Di
             Jmp @@for
@@endfor:
EndM

Proc FindDuplicates Near
             HashInit
             Mov Di,[Low_tsi]
             Mov Ax,[StrIndBuf+Di-2]

WhileLoop:   Cmp Di,2*StrIndBufLen    ; while Di<StrIndBufLen do
             Jae @@Return
             Mov Si,[UnComprPtr]
             Cmp Si,[UnComprEnd]
             Jne @@2
             ReadUnCompressedData
             Cmp [UnComprEnd],Offset UnComprData
             Jne @@1
@@Return:    Shr Di,1                 ; eof
             Mov [teststrings_index],Di
             Ret
@@1:         Mov Si,[UnComprPtr]
@@2:         Xor Dx,Dx
             Mov Dl,[Si]
             Inc [UnComprPtr]
             Shl Dx,1

@@loop:      calc_hashval LabelY

             Or Cx,2                  ; first hash ==> ggT(Cx,2)=1

; --- Vereinfachen
;       loop (hi,Bx)=hashindex(Ax,Dx)
;            if isfree(HashArray[hi]) then goto do_pos
@@hrepeat:   Add Si,Cx                 ; loop (hv,Cx)=rehash(hv,Cx)
             and Si,2*(maxhash-1)       ;   (lineares Re-Hashing)
             Mov Bx,[HashArray+Si]
             Cmp Bx,free
             je @@do_pos                ;       exitif (_pos=HashArray[hv])==free
             Cmp Di,[Low_tsi]
             je @@do_pos

             Cmp Ax,[StrIndBuf+Bx-2]
             jne @@test2
             Mov bp,[StrIndBuf+Bx]
             Cmp Bx,bp
             je @@square
             Cmp Dx,bp
             je @@sende
             Jmp @@test2
@@square:    Cmp Dx,Ax
             je @@sende                 ;       exitif (Ax,Dx)=decode(Bx)

@@test2:     Cmp Ax,2*256
             jb @@hrepeat               ;       continue if Ax<256

             Mov bp,[StrIndBuf+Bx-2]
             Xchg si,Ax
             Cmp bp,[StrIndBuf+si-2]
             Xchg si,Ax
             jne @@hrepeat              ;       continue if StrIndBuf[Ax-1]<>StrIndBuf[Bx-1]

             Mov bp,Bx
             Cmp Bx,[StrIndBuf+Bx]  ; Bx:= secondpart(Bx)
             je @@x1
             Add Bx,2
@@x1:        Mov Bx,[StrIndBuf+Bx-2]
             Cmp Bx,2*256             ;       continue if Bx<256
             jb @@hrepeat

             push si
             Mov si,Ax
             Cmp si,[StrIndBuf+si]  ; si:= secondpart(si)
             je @@x2
             Add si,2
@@x2:        Mov si,[StrIndBuf+si-2]
             Cmp si,[StrIndBuf+Bx-2]
             pop si
             jne @@hrepeat              ;       continue if secondpart(Ax)<>StrIndBuf[Bx-1]
             Cmp Bx,[StrIndBuf+Bx]  ; Bx:= secondpart(Bx)
             je @@x3
             Add Bx,2
@@x3:        Mov Bx,[StrIndBuf+Bx-2]
             Cmp Dx,Bx
             jne @@hrepeat              ;       continue if Dx<>secondpart(Bx)
                                        ;  endloop
             Mov Bx,bp
@@sende:
             Cmp Di,[Low_tsi]
             je @@no_match

             Mov Dx,Bx                ;            Dx:=Bx
             Sub Di,2                   ;            dec(Di)

             Cmp Dx,Di
             je @@no_match

             Mov Ax,[StrIndBuf+Di-2]        ;            Ax:=StrIndBuf[Di-1]
             Mov Bx,[Di+Shared.HashPos]   ;            HashArray[Shared.HashPos[Di]]:=free
             Mov [HashArray+Bx],free
             Jmp @@loop                     ;       endloop

; --- Ende vereinfachen

@@do_pos:    Cmp Di,2*255
             je @@no_match
             Mov [Di+Shared.HashPos],Si            ;  do_pos: Shared.HashPos[Di]:=HashValue
             Mov [HashArray+Si],Di          ;       HashArray[HashValue]:=Di

@@no_match:  Mov Ax,Dx                  ;       StrIndBuf[Di]:=Ax:=Dx
             Mov [StrIndBuf+Di],Ax
             Add Di,2                   ;       inc(Di)
             Jmp WhileLoop
EndP FindDuplicates


;--- Huffman
;
Assume Ds: aDataSeg, Es: TheHuffES

EXTRN ClearWhenFull: Near
EXTRN set_vars: Near
EXTRN InitHuffTables: Near
EXTRN inc_frequency: Near
EXTRN inc_posfreq: Near

Macro wBit BitBuf, BitCnt, ByteCnt
Local Done
; write out bit in Carry
  Rcr BitBuf,1                              ;; shift bit in carry
  Dec BitCnt
  Jne Done                                  ;; if bitpos>0 Then done
  Mov BitCnt,8                              ;; Else: reset bitpos
  Mov ByteCnt,[bbytepos]                    ;;
  Mov [ByteCnt],BitBuf                      ;; write byte to buffer
  Inc ByteCnt                               ;; advance and...
  Mov [bbytepos],ByteCnt                    ;; ...update pointer
  Cmp ByteCnt,bitpufferlaenge+Offset bitpuffer ;; check, if write
  Jb Done                                   ;; no write yet
  Call WriteCompressedData                  ;; write data out
  Mov [bbytepos],Offset bitpuffer           ;; reset bbytepos
Done:
EndM

Proc wBits    Near                      ; procedure wBits(BX=item,CX=nbits)
              Jcxz @@exit               ; cx=count
              Mov Ax,[bitpos]           ;  fOr c:=1 to count do
@@for:        Shr Bx,1                ;      schiebe low-Bit von item
              wBit <Ah>,<Al>,<Di>
              Loop @@for
              Mov [bitpos],ax
@@exit:       Ret
EndP wBits


Proc EncodeHuffEntry       near                 ;  procedure EncodeHuffEntry(Value=BX)
                                        ;  Register: CX: Bits
                                        ;            BX: Value
                                        ;            BP: Stack-Position
                                        ;            SI,DI
              Xor Cx,Cx                 ;  Bits:=0
              mov di,[frequencys+Bx]    ;  id:=Frequency(Value)
              push di                   ;  id retten
              Mov bp,sp                 ;  Stackposition merken

@@loop:       Or Di,Di                  ; While id<>0 Do Begin
              Je @@ende                 ;
              Mov Si,[Di+Shared.Vater]  ;   NextId:= Vater(Id);
              Cmp [Si+Shared.Sohn],Di   ;   If Sohn(Vater(Id))=Id Then
              Rcl Bx,1                  ;     Value:= Value<1 Else Value:= Value<1+1
              Mov Di,Si                 ;   Id:= NextId
              Inc Cx                    ;   Inc(Bits)
              And Cl,0Fh                ;   Bits:= Bits Mod 16
              Jne @@loop                ;   If Bits=0 Then
              Push Bx                   ;     push(Value)
              Jmp @@loop                ;  End

@@ende:
@@repeat:     Call wBits                ;  repeat wBits(BX,CX)
              Cmp sp,bp
              Je @@empty
              Pop bx                    ;         pop BX, CX:=16
              mov cx,16
              Jmp @@repeat
@@empty:      pop di                    ; inc_frequency(huffindex[item]
              Jmp inc_frequency         ; exit
EndP EncodeHuffEntry


;;;=================== Output
Proc WriteSmart    near
              Call set_vars
              Call InitHuffTables            ;  InitHuffTables

              Mov Cx,13                 ; 13 Bits schreiben
              mov bx,[teststrings_index] ; < 2^13 (=StrIndBufLen)
              Call wBits
              shl [teststrings_index],1
              mov Di,[Low_tsi]
              Sub Di,2                 ;  tsi:=Low_tsi-1
              mov [lastposition],2*256    ;  lastposition:=256

              Mov Ax,[char_offset]
              Add Ax,2*255
              mov [nfreq+0],Ax           ;  freq[-1]:=char_offset+2*255
@@while:      Add Di,2                 ;  while ++Di<teststrings_index do
              Cmp Di,[teststrings_index]
              jae @@endwhile
              Cmp Di,[maxlocal255]
              jbe @@endcase

              mov Ax,Di                ;  if Di>=maxlocal+2*255
              Add Ax,[char_offset]
              Sub Ax,[maxlocal]
              mov [nfreq+0],Ax           ;  then freq[-1]:=Di+(char_offset-maxlocal)

@@endcase:    mov Bx,[StrIndBuf+Di]        ;  begin ti:=StrIndBuf[Di]
              Cmp Bx,2*256
              jb @@002                  ;        if ti<256 then enhuff(ti)

              mov Ax,Bx
              Sub Ax,[lastposition]
              mov [lastposition],Bx
              Cmp Ax,2*2
              je @@enc_seq
              Cmp Ax,mindiff
              jl  @@001
              Cmp Ax,maxdiff
              jle @@enc_diff

@@001:        mov Ax,Di
              Sub Ax,Bx
              Cmp Ax,[maxlocal]
              jbe @@enc_local
              Jmp @@002

@@enc_seq:    Add Bx,Ax
              Cmp Bx,[StrIndBuf+Di+2]
              jne @@enc_diff            ;             and StrIndBuf[Di+2]=ti+2
              Add Di,2
              Cmp Di,[teststrings_index] ;             and ++Di<teststrings_index-1
              jae @@enc_diff

              Push Di Bx                ;             then enhuff(lsequence_key)
              mov bx,2*lsequence_key
              Call EncodeHuffEntry
              Pop Bx Di

@@seqloop:    Add Bx,2*2                ;                  loop ac+:=2
              Cmp Bx,[StrIndBuf+Di+2]
              jne @@endsloop            ;                       exitif ac<>StrIndBuf[Di+1]
              Add Di,2
              Cmp Di,[teststrings_index]
              jae @@endsloop            ;                       exitif ++Di>=teststrings_index
              Push Di
              Stc
              Mov Ax,[bitpos]
              wBit <Ah>,<Al>,<Di>       ; ein Bit=1 schreiben
              Mov [bitpos],Ax
              Pop Di
              Jmp @@seqloop             ;                  endloop

@@endsloop:   Sub Bx,2*2
              mov [lastposition],Bx     ;                  lastposition:=ac-2
              Push Di
              Clc
              Mov Ax,[bitpos]
              wBit <Ah>,<Al>,<Di>       ; ein Bit=0 schreiben
              Mov [bitpos],Ax
              Pop Di
              Jmp @@while

@@enc_diff:   Push Di            ;             else enhuff(differenz+(diff_offset-mindiff))
              mov bx,Ax
              Add bx,2*diff_offset-mindiff
              Call EncodeHuffEntry
              Pop Di
              Jmp @@while

@@enc_local:  Push Di
              mov bx,Ax
              Add bx,[local_offset]
              Call EncodeHuffEntry
              Pop Di
              Jmp @@while

@@002:        mov bx,[Bx+nindex]         ;        BX:=Indices[ti]
              Cmp bx,[HuffMaxindex]
              jae TabEncode

              Push Di                  ;        if BX<=HuffMaxindex
              Call EncodeHuffEntry              ;        then enhuff(BX)
              Pop Di
              Jmp @@while

TabEncode:  Push Di
              push bx
              mov bx,[frequencys+bx]
              Add bx,[pos_offset]
              Call EncodeHuffEntry              ;        else enhuff(frequence(BX)+pos_offset)
              pop si
              mov bx,[frequencys+si]
              mov ax,[nfreq+bx]
              mov bx,[nfreq+bx+2]
              neg bx
              Add ax,bx
              Add bx,si
              shr ax,1
              shr bx,1

              Mov Di,1
              Xor Cx,Cx
@@loop:       Shl Di,1                  ; Di=2^Cx
              Inc Cx
              Cmp Di,Ax
              jbe @@loop                ; bis 2^Cx>Bx
              Shr Di,1
              Or Di,Bx
              Cmp Di,Ax
              Jbe @@wrt
              Dec Cx                    ;  if item Or 2^(count-1)>bound
                                        ;  then dec(count)
@@wrt:        Call wBits
              Call inc_posfreq
              Pop Di
              Jmp @@while

@@endwhile:   shr [teststrings_index],1
              Ret
EndP WriteSmart

End
