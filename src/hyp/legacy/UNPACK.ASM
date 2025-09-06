; File UnPack.Asm
Include Defs.Inc

CodeSeg

Assume Ds: aDataSeg, Es: Nothing

Proc WriteUnCompressedData Near
             Push Ax Bx Cx Dx Si Di Es Ds
             Mov AX,Offset UnComprData
             Mov BX,[UnComprEnd]
             Sub Bx,Ax                    ; size of data
             Mov Dx,Ds
             Mov Es,Dx
             Mov Di, Offset Writer
             Mov Ds,[CallersDS]
             Call [DWord Es:Di] Pascal, Dx, Ax, Bx
             Pop Ds Es Di Si Dx Cx Bx Ax
             Mov [UnComprEnd],Offset UnComprData
             Ret
EndP WriteUnCompressedData

Proc DecodeData Near
             ;  Dx = free
             Mov Si,[Low_tsi]
             Mov Di,[UnComprEnd]
             Mov Cx,[teststrings_index]
             Shl Cx,1
             Sub Cx,Si
             Shr Cx,1                   ;  CX:=teststrings_index-Low_tsi
                                        ;   (#Bytes to decode)
             Mov bp,sp                  ;  merke die Position im Stack
             Or Cx,Cx
             Jne @@EnterLoop
@@Return:    Mov [UnComprEnd],Di
             Ret

@@GetStack:  Pop Ax                     ;                  i:=pop
             Or Ah,Ah
             Je @@one_part
@@two_parts: Mov bx,ax                  ;             else push(StrIndBuf[i])
             Mov ax,[StrIndBuf+bx]
             Cmp ax,bx
             jne @@direct
             Mov ax,[StrIndBuf+bx-2]
@@direct:    push ax
             Mov Ax,[StrIndBuf+bx-2] ;                  i:=StrIndBuf[i-1]
             Or Ah,Ah
             Jne @@two_parts            ; i>255: two_parts
@@one_part:  Mov [Di],Al                ; store i
             Inc Di
             Cmp Di,Offset UnComprData+UnComprSize
             Jbe @@nowrite
             Mov [UnComprEnd],Di        ; write out buffer
             Call WriteUnCompressedData
             Mov Di,[UnComprEnd]
@@nowrite:   Cmp sp,bp
             Jne @@GetStack
             Jcxz @@Return              ;  for index:=255
@@EnterLoop: Dec cx                     ;  to teststrings_index-1 do
             Mov Ax,[Si]
             Add Si,2                   ;  begin i:=StrIndBuf[index]

             Or ah,ah                   ;        loop
             Je @@one_part              ; i>255: two_parts
             Jmp @@two_parts
EndP DecodeData

EDHeader Decode

             Mov [UnComprEnd],Offset UnComprData
             Mov [bbytepos],Offset bitpuffer   ;  bbytepos:=0
             Call ReadCompressedData                   ;  init_input
             Mov bh,[bitpuffer]
             Mov bl,8
             Mov [bitpos],bx          ;  (bitpos,puffer_byte):=(8,1.Byte im Puffer)
             Mov [Low_tsi],2*255
             Mov [teststrings_index],255

@@repeat:    Call ClearWhenFull
             Call ReadSmart
             Call DecodeData
             Cmp [teststrings_index],255
             jne @@repeat

             Call WriteUnCompressedData            ;  flush_output

             Mov Ds,[CallersDS]
             Ret
EndP Decode

Assume Ds: aDataSeg, Es: TheHuffES

EXTRN ClearWhenFull: Near
EXTRN set_vars: Near
EXTRN InitHuffTables: Near
EXTRN inc_frequency: Near
EXTRN inc_posfreq: Near

Proc ReadCompressedData Near ;(DI=bbytepos)
              Push Ax Bx Cx Dx Si Di Es Ds
              Mov Ax,Offset bitpuffer
              Mov Bx,bitpufferlaenge
              Mov Dx,Ds
              Mov Es,Dx
              Mov Di, Offset Reader
              Mov Ds,[CallersDS]
              Call [DWord Es:Di] Pascal, Dx, Ax, Bx   ; ReadBinaryBlock(@bitpuffer,bitpufferlaenge)
              Pop Ds
              Add Ax,Offset bitpuffer
              Mov [max_bitpufpos],Ax    ;  max_bitpufpos:=#gelesene Bytes
              Pop Es Di Si Dx Cx Bx Ax
              Ret
EndP ReadCompressedData

Macro rBit BitBuf, BitCnt, ByteCnt
Local @@1, @@2
; read bit, return in carry
; folgendes muss in Registern sein:
; BitBuf= [Byte Ptr bitpos+1]
; BitCnt= [Byte Ptr bitpos]
; ByteCnt= [bbytepos]
              Shr BitBuf,1
              Dec BitCnt
              Jne @@2
              PushF
              Mov ByteCnt,[bbytepos]
              Inc ByteCnt
              Cmp ByteCnt,[max_bitpufpos]
              Jne @@1
              Call ReadCompressedData
              Mov ByteCnt, Offset bitpuffer
@@1:          Mov BitCnt,8
              Mov BitBuf,[ByteCnt]
              Mov [bbytepos],ByteCnt
              PopF
@@2:
EndM


Macro TabDecode
; Parameter: Bx=Frequenz
; Result:    Ax
              Sub Bx,[pos_offset]
              Mov Ax,[nfreq+Bx]
              Mov Si,[nfreq+Bx+2]
              Sub Ax,Si
              Shr Ax,1
              Mov Bx,[bitpos]

              Mov dx,ax
              Mov Cx,1
              Mov Ax,Cx
@@repeat:     rBit <Bh>,<Bl>,<Di>
              jc @@skp
              Xor Ax,Cx
@@skp:        shl Cx,1
              Or Ax,Cx
              Cmp Ax,Dx
              jbe @@repeat

              Xor Ax,Cx
              Mov [bitpos],Bx

              Shl Ax,1
              Add si,ax
              Mov ax,[nvalue+si]
              Call inc_posfreq
EndM

Macro rBits
Local @@loop,@@1
              Mov Bx,[bitpos]
              Mov Dx,1
              Xor Ax,Ax
@@loop:       rBit <Bh>,<Bl>,<Di>
              Jnc @@1
              Or Ax,Dx
@@1:          Shl Dx,1
              Loop @@loop
              Mov [bitpos],Bx
EndM

Proc DecodeHuffEntry Near
; AX: word
              Xor Si,Si                 ;  i:=0
              Mov Bx,[bitpos]
@@loop:       mov si,[si+Shared.Sohn]         ; Si:= Sohn(Si)
              test si,1
              jne @@ende                ;       exitif "hufftree[i] ist Blatt"
              rBit <Bh>,<Bl>,<Di>
              Jnc @@loop
              Add Si,2                 ;       inc(i,get_Bit)
              Jmp @@loop

@@ende:       Mov [bitpos],Bx
              dec si
              mov ax,si                 ;  dehuff:=not i
              mov di,[frequencys+si]
              Jmp inc_frequency         ;  inc_frequency(huffindex[i])
EndP DecodeHuffEntry

Proc ReadSmart Near
              Mov Cx,13                 ; 13 Bits (2^13=StrIndBufLen)
              rBits
              mov [teststrings_index],ax  ;  teststrings_index:=rBits(StrIndBufLen)
              Call set_vars
              Call InitHuffTables
              shl [teststrings_index],1
              mov Di,[Low_tsi]
              Sub Di,2                 ;  tsi:=Low_tsi-1
              mov [lastposition],2*256    ;  lastposition:=256

              mov Ax,[char_offset]
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

@@endcase:    Push Di
              Call DecodeHuffEntry              ;  AX:=dehuff
              Pop Di
              mov Bx,AX
              Cmp Bx,2*lsequence_key
              je @@dec_lseq             ;        if ti=lsequence_key then dec_lseq
              Cmp Bx,[local_offset]
              jb @@dec_diff             ;        if ti<=pos_offset then dec_diff
              Cmp Bx,[pos_offset]
              jb @@dec_local
              Cmp Bx,[char_offset]
              jb @@dec_table            ;        if ti<=char_offset then dec_table
              mov ax,[nvalue+Bx]
              Jmp @@mov_ti

@@dec_table:  Push Di
              TabDecode            ;             TabDecode(AX)
              Pop Di

@@mov_ti:     Cmp ax,2*256
              jb @@move_char

@@move_pos:   mov [StrIndBuf+Di],ax
              mov [lastposition],ax
              Jmp @@while

@@move_char:  shr ax,1
              mov [StrIndBuf+Di],ax
              Jmp @@while

@@dec_lseq:   mov ax,[lastposition]
              Add ax,2*2                ;           repeat lastposition+:=2
              mov [StrIndBuf+Di],ax        ;                  StrIndBuf[Di]:=lastposition
              Add Di,2                 ;                  Di++
              Mov Cx,[bitpos]
              Mov Bx,[bbytepos]
@@dloop:      Add ax,2*2                ;           repeat lastposition+:=2
              Mov [StrIndBuf+Di],ax        ;                  StrIndBuf[Di]:=lastposition
              Add Di,2                 ;                  Di++
              rBit <Ch>,<Cl>,<Bx>
              Jc @@dloop
              Sub Di,2
              mov [lastposition],ax
              Mov [bitpos],Cx
              Mov [bbytepos],Bx
              Jmp @@while

@@dec_diff:   Sub Bx,2*diff_offset-mindiff      ;    dec_diff:
              Add Bx,[lastposition]       ;           lastposition+:=ti+mindiff
              mov [StrIndBuf+Di],Bx        ;           StrIndBuf[Di]:=lastposition
              mov [lastposition],Bx       ;
              Jmp @@while

@@dec_local:  Sub Bx,[local_offset]       ;        dec_local:
              neg Bx                    ;           ti:=ti-local_offset
              Add Bx,Di                ;           ti:=Di-ti
              mov [StrIndBuf+Di],Bx
              mov [lastposition],Bx
              Jmp @@while

@@endwhile:   shr [teststrings_index],1
              Ret
EndP ReadSmart
End
