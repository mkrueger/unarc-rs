; File Comm.Asm
Include Defs.Inc

CodeSeg

Assume Ds: aDataSeg, Es: Nothing

Public ClearWhenFull

maxmcount    = 1350                    ;  #Characters for partial clearing
marked       = 1

Proc markiere Near ; (di)
             Cmp [Di+Shared.NewIndex],0
             Jne @@exit                ;  exitif (Shared.NewIndex[Di]=marked)

             Mov Si,[StrIndBuf+Di]   ;  Si:=StrIndBuf[Di]
             Cmp Si,2*256
             jb @@no_recurse

             Push Di Si
             Mov Di,Si
             Sub Di,2
             Call markiere             ;  then markiere(ti-1)
             Pop Si Di
             Cmp Di,Si
             je @@no_recurse

             Push Di
             Mov Di,Si
             Call markiere      ;  if ti<>Di then markiere(ti)
             Pop Di

@@no_recurse:Mov [Di+Shared.NewIndex],marked
             Inc Cx
@@exit:      Ret
EndP markiere

Proc ClearWhenFull Near
             Mov [Low_tsi],2*255
             Cmp [teststrings_index],StrIndBufLen
             jne @@exit

             Push Es
             Push Ds
             Pop Es
             fillword <Offset Shared.NewIndex>,0,StrIndBufLen
             Pop Es

             Xor Cx,Cx
             Mov Di,2*StrIndBufLen
@@for:       Sub Di,2
             Call markiere
             Cmp Cx,maxmcount
             jb @@fOr

@@endfor:    Add Cx,255
             shl Cx,1
             Mov [Low_tsi],Cx

             Mov Bx,2*255
             Mov Di,2*254
@@for2:      Add Di,2            ;  for(Di:=255;Bx<>255+maxmcount;++Di)do
             Cmp [Di+Shared.NewIndex],0
             je @@for2
             Mov [Di+Shared.NewIndex],Bx
             Mov Si,[StrIndBuf+Di]
             Cmp Si,2*256
             jb @@1
             Mov Si,[Si+Shared.NewIndex] ; if ti>=256 then ti:=Shared.NewIndex[ti]
@@1:         Mov [StrIndBuf+Bx],Si     ;   StrIndBuf[mcount]:=ti

             Add Bx,2                  ;
             Cmp Bx,[Low_tsi]
             jb @@for2

@@exit:      Ret
EndP ClearWhenFull

;-------------------- Huffman part

Assume Ds: aDataSeg, Es: TheHuffES

Public set_vars
Public InitHuffTables
Public inc_frequency
Public inc_posfreq

;--- Huffman initializing
;
Proc set_vars      near

maxlocal_40     =  2
maxlocal_41     =  80

; maxlocal berechnen: im wesentlichen wird ein Wert zwischen maxlocal_40
; und maxlocal_41 entsprechend dem Verh„ltnis von teststrings_index zu
; StrIndBufLen berechnet:
; maxlocal:= maxlocal_40+ (maxlocal_41-maxlocal_40)* (teststrings_index/StrIndBufLen)

              Mov ax,2*(maxlocal_41-maxlocal_40)
              Mov Cx,[teststrings_index]
              Sub Cx,255
              IMul Cx
              Mov Cx,StrIndBufLen-255
              IDiv Cx
              And Ax,0FFFEh
              Add Ax,2*maxlocal_40
              Mov [maxlocal],Ax
              Add Ax,2*255
              mov [maxlocal255],AX        ;  maxlocal255:=maxlocal+2*255
              mov AX,2*(diff_offset+1)+maxdiff-mindiff
              mov [local_offset],AX       ;  local_offset:=diff_offset+(maxdiff-mindiff+1)
              Add AX,[maxlocal]
              Add AX,2
              mov [pos_offset],AX         ;  pos_offset:=local_offset+(maxlocal+1)
              Add AX,2*maxfreq+2
              mov [char_offset],AX        ;  char_offset:=pos_offset+2*(maxfreq+1)
              Ret
EndP set_vars

Proc inc_posfreq   near
              mov bx,[frequencys+si]
              mov di,[nfreq+bx+2]        ;  f:=freq[SI^.frequenz]
              mov bx,[nvalue+si]
              mov [nindex+bx],di         ;  nindex[i]:=f
              xchg bx,[nvalue+di]        ;  SI^.itemindex --> f^.itemindex --> BX
              mov [nindex+bx],si         ;  nindex[f^.itemindex]:=@items[i]
              mov [nvalue+si],bx         ;  BX --> SI^.itemindex
              mov bx,[frequencys+di]
              Cmp bx,2*maxfreq
              je @@ins_huff
              Add bx,2
              mov [frequencys+di],bx     ;  inc(f^.frequenz)
              Add di,2
              mov [nfreq+bx],di          ;  inc(freq[SI^.frequenz])
              Ret
@@ins_huff:   Call ninsert
              mov di,[HuffMax]
              Sub di,2
              ; continue with inc_frequency!
EndP inc_posfreq

Proc inc_frequency Near ; (di)
              mov bx,[di+Shared.TheFreq]  ; f:=hufftree[i].frequenz
              Cmp bx,2*maxrecfrequency
              jae @@exit                  ; exitif f>=maxrecfrequency
              mov si,[nfreqmax+bx-2]      ; j:=freqmax[f]
              Add [nfreqmax+bx-2],2     ; inc(freqmax[f])

              Cmp di,si
              je @@loop                 ; if i=j then goto loop
              mov bx,[di+Shared.Sohn]   ; Sohn[i] ist kein Blatt:
              mov [bx-1+frequencys],si  ; frequencys[Sohn[i]]:=j

@@entry:      xchg bx,[si+Shared.Sohn]
              test bx,1                 ; Blatt ?
              jne @@Blatt
              mov [bx+2+Shared.Vater],di
              mov [bx+Shared.Vater],di
              Jmp @@endcase12
@@Blatt:      mov [bx-1+frequencys],di
@@endcase12:  mov [di+Shared.Sohn],bx   ; Blatt
@@loop:       Add [si+Shared.TheFreq],2 ; TheFreq[j]++
              mov di,[si+Shared.Vater]  ; i:=Vater[j]
              Cmp si,0
              je @@exit

ientry:       mov bx,[di+Shared.TheFreq]; f:=TheFreq[i]
              Cmp bx,2*maxrecfrequency
              jae @@exit
              mov si,[nfreqmax+bx-2]
              Add [nfreqmax+bx-2],2
              Cmp di,si
              je @@loop
              mov bx,[di+Shared.Sohn]
              mov [bx+2+Shared.Vater],si
              mov [bx+Shared.Vater],si
              Jmp @@entry

@@exit:       Ret
EndP inc_frequency

Proc ninsert Near
              mov di,[HuffMax]
              mov bx,di
              Sub bx,2                    ; HuffMax-1
              mov [di+Shared.Vater],bx    ; Vater[HuffMax]:=HuffMax-1
              mov [di+2+Shared.Vater],bx  ; Vater[HuffMax+1]:=HuffMax-1
              mov [di+2+Shared.TheFreq],2 ; TheFreq[HuffMax+1]:=1
              mov bx,[HuffMaxindex]
              inc bx
              mov [di+2+Shared.Sohn],bx   ; Sohn[HuffMax+1]:=HuffMaxindex or 1
              mov bx,di
              xchg bx,[di-2+Shared.Sohn]  ; HuffMax --> Sohn[HuffMax-1] --> BX
              mov [di+Shared.Sohn],bx     ; Sohn[HuffMax]:=Sohn[HuffMax-1]
              mov [frequencys+bx-1],di    ; huffindex[bx]:=HuffMax
              mov bx,[di-2+Shared.TheFreq]
              mov [di+Shared.TheFreq],bx  ; TheFreq[HuffMax]:=Sohn[HuffMax-1]
              Add di,2
              mov bx,[HuffMaxindex]
              mov [frequencys+bx],di      ; frequencys[HuffMaxindex]:= HuffMax+1
              Add bx,2
              mov [HuffMaxindex],bx       ; HuffMaxindex++
              Add di,2
              mov [HuffMax],di            ; HuffMax+:=2
              Sub di,3*2                  ; inc_frequency(HuffMax-3)
              Jmp ientry
EndP ninsert

Proc InitHuffTables Near
              fillword <offset nfreqmax>,0,maxrecfrequency
              mov cx,StrIndBufLen
              Xor Si,Si
              mov di,[char_offset]
@@for1:       mov [nindex+si],di
              mov [nvalue+di],si
              mov [frequencys+di],0
              Add di,2
              Add si,2
              loop @@for1

              fillword <offset nfreq>,<[char_offset]>,maxfreq+2
              mov [Shared.Sohn],1
              mov [Shared.TheFreq],2
              mov [Shared.Vater],0
              mov [HuffMaxindex],2
              mov [frequencys+0],0
              mov [HuffMax],2
              mov cx,[char_offset]
              shr cx,1
              dec cx
@@for:        Call ninsert
              Loop @@for
              Ret
EndP InitHuffTables

End
