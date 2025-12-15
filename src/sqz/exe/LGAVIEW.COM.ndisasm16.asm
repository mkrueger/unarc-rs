00000000  FC                cld
00000001  B827BF            mov ax,0xbf27
00000004  050008            add ax,0x800
00000007  8BE0              mov sp,ax
00000009  B104              mov cl,0x4
0000000B  D3E8              shr ax,cl
0000000D  40                inc ax
0000000E  8CC3              mov bx,es
00000010  03C3              add ax,bx
00000012  A325BF            mov [0xbf25],ax
00000015  A323BF            mov [0xbf23],ax
00000018  A10200            mov ax,[0x2]
0000001B  A321BF            mov [0xbf21],ax
0000001E  BF6CB1            mov di,0xb16c
00000021  B9B50D            mov cx,0xdb5
00000024  33C0              xor ax,ax
00000026  F3AA              rep stosb
00000028  E83500            call 0x60
0000002B  B9E204            mov cx,0x4e2
0000002E  E80101            call 0x132
00000031  50                push ax
00000032  B90010            mov cx,0x1000
00000035  E8FA00            call 0x132
00000038  1E                push ds
00000039  07                pop es
0000003A  BF34BD            mov di,0xbd34
0000003D  AB                stosw
0000003E  58                pop ax
0000003F  AB                stosw
00000040  E87101            call 0x1b4
00000043  B95200            mov cx,0x52
00000046  E8E900            call 0x132
00000049  1E                push ds
0000004A  07                pop es
0000004B  BF32BD            mov di,0xbd32
0000004E  AB                stosw
0000004F  E89E49            call 0x49f0
00000052  33C0              xor ax,ax
00000054  E83900            call 0x90
00000057  B44C              mov ah,0x4c
00000059  CD21              int byte 0x21
0000005B  B8FF00            mov ax,0xff
0000005E  EBF4              jmp 0x54
00000060  BF15BE            mov di,0xbe15
00000063  FA                cli
00000064  1E                push ds
00000065  33C0              xor ax,ax
00000067  8ED8              mov ds,ax
00000069  BE6C00            mov si,0x6c
0000006C  A5                movsw
0000006D  A5                movsw
0000006E  BE8C00            mov si,0x8c
00000071  A5                movsw
00000072  A5                movsw
00000073  A5                movsw
00000074  A5                movsw
00000075  1F                pop ds
00000076  FB                sti
00000077  BA9F02            mov dx,0x29f
0000007A  B82425            mov ax,0x2524
0000007D  CD21              int byte 0x21
0000007F  BA4D07            mov dx,0x74d
00000082  B81B25            mov ax,0x251b
00000085  CD21              int byte 0x21
00000087  BA5B01            mov dx,0x15b
0000008A  B82325            mov ax,0x2523
0000008D  CD21              int byte 0x21
0000008F  C3                ret
00000090  FA                cli
00000091  BE15BE            mov si,0xbe15
00000094  33DB              xor bx,bx
00000096  8EC3              mov es,bx
00000098  BF6C00            mov di,0x6c
0000009B  A5                movsw
0000009C  A5                movsw
0000009D  BF8C00            mov di,0x8c
000000A0  A5                movsw
000000A1  A5                movsw
000000A2  A5                movsw
000000A3  A5                movsw
000000A4  FB                sti
000000A5  C3                ret
000000A6  48                dec ax
000000A7  D1E0              shl ax,0x0
000000A9  50                push ax
000000AA  91                xchg ax,cx
000000AB  E88100            call 0x12f
000000AE  8E0636BD          mov es,word [0xbd36]
000000B2  5F                pop di
000000B3  AB                stosw
000000B4  C3                ret
000000B5  050F00            add ax,0xf
000000B8  B104              mov cl,0x4
000000BA  D3E8              shr ax,cl
000000BC  91                xchg ax,cx
000000BD  C3                ret
000000BE  8E0636BD          mov es,word [0xbd36]
000000C2  D1E7              shl di,0x0
000000C4  268B7DFE          mov di,[es:di-0x2]
000000C8  E8EAFF            call 0xb5
000000CB  8BD7              mov dx,di
000000CD  03D1              add dx,cx
000000CF  3B1625BF          cmp dx,[0xbf25]
000000D3  7505              jnz 0xda
000000D5  893E25BF          mov [0xbf25],di
000000D9  C3                ret
000000DA  833E0FBE00        cmp word [0xbe0f],0x0
000000DF  7504              jnz 0xe5
000000E1  33C0              xor ax,ax
000000E3  EB09              jmp 0xee
000000E5  3B3E0FBE          cmp di,[0xbe0f]
000000E9  7309              jnc 0xf4
000000EB  A10FBE            mov ax,[0xbe0f]
000000EE  893E0FBE          mov [0xbe0f],di
000000F2  EB2E              jmp 0x122
000000F4  33F6              xor si,si
000000F6  1E                push ds
000000F7  8E1E0FBE          mov ds,word [0xbe0f]
000000FB  8CC0              mov ax,es
000000FD  034402            add ax,[si+0x2]
00000100  3BC7              cmp ax,di
00000102  7426              jz 0x12a
00000104  8B04              mov ax,[si]
00000106  0BC0              or ax,ax
00000108  7415              jz 0x11f
0000010A  3BC2              cmp ax,dx
0000010C  7406              jz 0x114
0000010E  770F              ja 0x11f
00000110  8ED8              mov ds,ax
00000112  EBE7              jmp 0xfb
00000114  893C              mov [si],di
00000116  8ED8              mov ds,ax
00000118  8B04              mov ax,[si]
0000011A  034C02            add cx,[si+0x2]
0000011D  EB03              jmp 0x122
0000011F  893C              mov [si],di
00000121  1F                pop ds
00000122  8EC7              mov es,di
00000124  33FF              xor di,di
00000126  AB                stosw
00000127  91                xchg ax,cx
00000128  AB                stosw
00000129  C3                ret
0000012A  014C02            add [si+0x2],cx
0000012D  1F                pop ds
0000012E  C3                ret
0000012F  E883FF            call 0xb5
00000132  833E0FBE00        cmp word [0xbe0f],0x0
00000137  744D              jz 0x186
00000139  8E060FBE          mov es,word [0xbe0f]
0000013D  33FF              xor di,di
0000013F  8BC7              mov ax,di
00000141  268B5502          mov dx,[es:di+0x2]
00000145  268B1D            mov bx,[es:di]
00000148  3BCA              cmp cx,dx
0000014A  7732              ja 0x17e
0000014C  06                push es
0000014D  7510              jnz 0x15f
0000014F  0BC0              or ax,ax
00000151  7506              jnz 0x159
00000153  891E0FBE          mov [0xbe0f],bx
00000157  58                pop ax
00000158  C3                ret
00000159  93                xchg ax,bx
0000015A  8EC3              mov es,bx
0000015C  AB                stosw
0000015D  58                pop ax
0000015E  C3                ret
0000015F  2BD1              sub dx,cx
00000161  52                push dx
00000162  8BD3              mov dx,bx
00000164  8CC3              mov bx,es
00000166  03D9              add bx,cx
00000168  0BC0              or ax,ax
0000016A  750A              jnz 0x176
0000016C  891E0FBE          mov [0xbe0f],bx
00000170  8EC3              mov es,bx
00000172  AB                stosw
00000173  58                pop ax
00000174  EBE6              jmp 0x15c
00000176  8EC0              mov es,ax
00000178  8BC3              mov ax,bx
0000017A  AB                stosw
0000017B  92                xchg ax,dx
0000017C  EBF2              jmp 0x170
0000017E  8CC0              mov ax,es
00000180  8EC3              mov es,bx
00000182  0BDB              or bx,bx
00000184  75BB              jnz 0x141
00000186  A125BF            mov ax,[0xbf25]
00000189  03C1              add ax,cx
0000018B  3B0621BF          cmp ax,[0xbf21]
0000018F  7705              ja 0x196
00000191  870625BF          xchg ax,[0xbf25]
00000195  C3                ret
00000196  BF03A8            mov di,0xa803
00000199  BE1AA8            mov si,0xa81a
0000019C  E90B21            jmp 0x22aa
0000019F  FB                sti
000001A0  83C408            add sp,0x8
000001A3  8BEC              mov bp,sp
000001A5  804E0E01          or byte [bp+0xe],0x1
000001A9  33C0              xor ax,ax
000001AB  5B                pop bx
000001AC  59                pop cx
000001AD  5A                pop dx
000001AE  5E                pop si
000001AF  5F                pop di
000001B0  5D                pop bp
000001B1  1F                pop ds
000001B2  07                pop es
000001B3  CF                iret
000001B4  E87F07            call 0x936
000001B7  E8F807            call 0x9b2
000001BA  A2C0B9            mov [0xb9c0],al
000001BD  A1B8BD            mov ax,[0xbdb8]
000001C0  A357BD            mov [0xbd57],ax
000001C3  A362BD            mov [0xbd62],ax
000001C6  3C03              cmp al,0x3
000001C8  7410              jz 0x1da
000001CA  3C02              cmp al,0x2
000001CC  740C              jz 0x1da
000001CE  3C07              cmp al,0x7
000001D0  7408              jz 0x1da
000001D2  B003              mov al,0x3
000001D4  A362BD            mov [0xbd62],ax
000001D7  E81507            call 0x8ef
000001DA  1E                push ds
000001DB  BF1BBA            mov di,0xba1b
000001DE  57                push di
000001DF  BEF7B0            mov si,0xb0f7
000001E2  E8790D            call 0xf5e
000001E5  58                pop ax
000001E6  58                pop ax
000001E7  A0B3BD            mov al,[0xbdb3]
000001EA  A261BD            mov [0xbd61],al
000001ED  BF8100            mov di,0x81
000001F0  B9FF00            mov cx,0xff
000001F3  33C0              xor ax,ax
000001F5  F2AE              repne scasb
000001F7  57                push di
000001F8  BED7AF            mov si,0xafd7
000001FB  B001              mov al,0x1
000001FD  B105              mov cl,0x5
000001FF  F3A6              repe cmpsb
00000201  5E                pop si
00000202  7529              jnz 0x22d
00000204  4E                dec si
00000205  56                push si
00000206  BF38BD            mov di,0xbd38
00000209  B01E              mov al,0x1e
0000020B  AA                stosb
0000020C  91                xchg ax,cx
0000020D  F3A4              rep movsb
0000020F  5F                pop di
00000210  268B450F          mov ax,[es:di+0xf]
00000214  3C01              cmp al,0x1
00000216  7511              jnz 0x229
00000218  A2C1B9            mov [0xb9c1],al
0000021B  268B4511          mov ax,[es:di+0x11]
0000021F  A3C0BC            mov [0xbcc0],ax
00000222  268B4515          mov ax,[es:di+0x15]
00000226  A3C4BC            mov [0xbcc4],ax
00000229  268A4509          mov al,[es:di+0x9]
0000022D  A2C6B9            mov [0xb9c6],al
00000230  A801              test al,0x1
00000232  750F              jnz 0x243
00000234  BEE0A3            mov si,0xa3e0
00000237  BFD8A3            mov di,0xa3d8
0000023A  A5                movsw
0000023B  AC                lodsb
0000023C  8AE0              mov ah,al
0000023E  AB                stosw
0000023F  AD                lodsw
00000240  AB                stosw
00000241  AC                lodsb
00000242  AB                stosw
00000243  A0DCA3            mov al,[0xa3dc]
00000246  A2B3BD            mov [0xbdb3],al
00000249  803EC1B900        cmp byte [0xb9c1],0x0
0000024E  9C                pushf
0000024F  7503              jnz 0x254
00000251  E87505            call 0x7c9
00000254  A0B7BD            mov al,[0xbdb7]
00000257  A2C9B9            mov [0xb9c9],al
0000025A  40                inc ax
0000025B  A2ACB1            mov [0xb1ac],al
0000025E  2C05              sub al,0x5
00000260  A2C7B9            mov [0xb9c7],al
00000263  E89A07            call 0xa00
00000266  B83335            mov ax,0x3533
00000269  CD21              int byte 0x21
0000026B  8CC0              mov ax,es
0000026D  0BC3              or ax,bx
0000026F  7410              jz 0x281
00000271  26813FCF00        cmp word [es:bx],0xcf
00000276  7409              jz 0x281
00000278  33C0              xor ax,ax
0000027A  CD33              int byte 0x33
0000027C  3DFFFF            cmp ax,0xffff
0000027F  7414              jz 0x295
00000281  0E                push cs
00000282  07                pop es
00000283  B0C3              mov al,0xc3
00000285  BF2E06            mov di,0x62e
00000288  AA                stosb
00000289  BF3406            mov di,0x634
0000028C  AA                stosb
0000028D  BF0407            mov di,0x704
00000290  AA                stosb
00000291  BF480B            mov di,0xb48
00000294  AA                stosb
00000295  C606C3B901        mov byte [0xb9c3],0x1
0000029A  E89702            call 0x534
0000029D  9D                popf
0000029E  9C                pushf
0000029F  7509              jnz 0x2aa
000002A1  E89A0B            call 0xe3e
000002A4  E8E613            call 0x168d
000002A7  E82302            call 0x4cd
000002AA  B430              mov ah,0x30
000002AC  CD21              int byte 0x21
000002AE  3C04              cmp al,0x4
000002B0  7214              jc 0x2c6
000002B2  B80069            mov ax,0x6900
000002B5  B303              mov bl,0x3
000002B7  BABEB1            mov dx,0xb1be
000002BA  CD21              int byte 0x21
000002BC  8BF2              mov si,dx
000002BE  AD                lodsw
000002BF  AD                lodsw
000002C0  93                xchg ax,bx
000002C1  AD                lodsw
000002C2  03D8              add bx,ax
000002C4  EB29              jmp 0x2ef
000002C6  BAF701            mov dx,0x1f7
000002C9  B0EC              mov al,0xec
000002CB  EE                out dx,al
000002CC  B414              mov ah,0x14
000002CE  33C9              xor cx,cx
000002D0  EC                in al,dx
000002D1  A880              test al,0x80
000002D3  7406              jz 0x2db
000002D5  E2F9              loop 0x2d0
000002D7  FECC              dec ah
000002D9  75F3              jnz 0x2ce
000002DB  B91400            mov cx,0x14
000002DE  B2F0              mov dl,0xf0
000002E0  ED                in ax,dx
000002E1  80FC30            cmp ah,0x30
000002E4  7C07              jl 0x2ed
000002E6  80FC39            cmp ah,0x39
000002E9  7F02              jg 0x2ed
000002EB  03D8              add bx,ax
000002ED  E2F1              loop 0x2e0
000002EF  83C313            add bx,0x13
000002F2  BE81A4            mov si,0xa481
000002F5  BF6102            mov di,0x261
000002F8  33FB              xor di,bx
000002FA  B106              mov cl,0x6
000002FC  BB3412            mov bx,0x1234
000002FF  AC                lodsb
00000300  98                cbw
00000301  2C2F              sub al,0x2f
00000303  50                push ax
00000304  F7E3              mul bx
00000306  33F8              xor di,ax
00000308  58                pop ax
00000309  03F8              add di,ax
0000030B  46                inc si
0000030C  E2F1              loop 0x2ff
0000030E  57                push di
0000030F  BE82A4            mov si,0xa482
00000312  B105              mov cl,0x5
00000314  BB0A00            mov bx,0xa
00000317  33C0              xor ax,ax
00000319  F7E3              mul bx
0000031B  97                xchg ax,di
0000031C  AD                lodsw
0000031D  98                cbw
0000031E  2C30              sub al,0x30
00000320  03C7              add ax,di
00000322  E2F5              loop 0x319
00000324  59                pop cx
00000325  3BC1              cmp ax,cx
00000327  7404              jz 0x32d
00000329  FE067EBD          inc byte [0xbd7e]
0000032D  1E                push ds
0000032E  BF9EBB            mov di,0xbb9e
00000331  57                push di
00000332  B201              mov dl,0x1
00000334  E8360F            call 0x126d
00000337  5E                pop si
00000338  58                pop ax
00000339  9D                popf
0000033A  7510              jnz 0x34c
0000033C  C606BFB9FF        mov byte [0xb9bf],0xff
00000341  AC                lodsb
00000342  0AC0              or al,al
00000344  7506              jnz 0x34c
00000346  BFD3A9            mov di,0xa9d3
00000349  E85B1F            call 0x22a7
0000034C  E85A0C            call 0xfa9
0000034F  7305              jnc 0x356
00000351  BFF0A9            mov di,0xa9f0
00000354  EBF3              jmp 0x349
00000356  A3B4B1            mov [0xb1b4],ax
00000359  E85F20            call 0x23bb
0000035C  E8B00F            call 0x130f
0000035F  A378BD            mov [0xbd78],ax
00000362  50                push ax
00000363  92                xchg ax,dx
00000364  E87820            call 0x23df
00000367  E86310            call 0x13cd
0000036A  A2A0BD            mov [0xbda0],al
0000036D  5A                pop dx
0000036E  0AC0              or al,al
00000370  9C                pushf
00000371  7411              jz 0x384
00000373  0E                push cs
00000374  07                pop es
00000375  BEBC4A            mov si,0x4abc
00000378  BFFC4A            mov di,0x4afc
0000037B  98                cbw
0000037C  D1E0              shl ax,0x0
0000037E  03F0              add si,ax
00000380  A5                movsw
00000381  E85B20            call 0x23df
00000384  803EC1B900        cmp byte [0xb9c1],0x0
00000389  744D              jz 0x3d8
0000038B  9D                popf
0000038C  7401              jz 0x38f
0000038E  C3                ret
0000038F  E81C04            call 0x7ae
00000392  8A169EBB          mov dl,[0xbb9e]
00000396  80C20E            add dl,0xe
00000399  D0EA              shr dl,0x0
0000039B  F6DA              neg dl
0000039D  80C213            add dl,0x13
000003A0  B602              mov dh,0x2
000003A2  BFF5AD            mov di,0xadf5
000003A5  E80305            call 0x8ab
000003A8  BF9EBB            mov di,0xbb9e
000003AB  E80205            call 0x8b0
000003AE  BF00AE            mov di,0xae00
000003B1  E8FC04            call 0x8b0
000003B4  BA0103            mov dx,0x301
000003B7  BF05AE            mov di,0xae05
000003BA  E8EE04            call 0x8ab
000003BD  BA0907            mov dx,0x709
000003C0  BF2AAE            mov di,0xae2a
000003C3  E8E504            call 0x8ab
000003C6  A0DAA3            mov al,[0xa3da]
000003C9  A2B3BD            mov [0xbdb3],al
000003CC  BA0F07            mov dx,0x70f
000003CF  BFF2AD            mov di,0xadf2
000003D2  E8D604            call 0x8ab
000003D5  E9EF43            jmp 0x47c7
000003D8  9D                popf
000003D9  7505              jnz 0x3e0
000003DB  B009              mov al,0x9
000003DD  E8D21E            call 0x22b2
000003E0  E80A2F            call 0x32ed
000003E3  A01DBB            mov al,[0xbb1d]
000003E6  D0E8              shr al,0x0
000003E8  3C07              cmp al,0x7
000003EA  7D02              jnl 0x3ee
000003EC  B007              mov al,0x7
000003EE  50                push ax
000003EF  0405              add al,0x5
000003F1  91                xchg ax,cx
000003F2  BF6FAF            mov di,0xaf6f
000003F5  E8BE08            call 0xcb6
000003F8  58                pop ax
000003F9  2C03              sub al,0x3
000003FB  50                push ax
000003FC  98                cbw
000003FD  92                xchg ax,dx
000003FE  BFFFA9            mov di,0xa9ff
00000401  E8A704            call 0x8ab
00000404  A0A0BD            mov al,[0xbda0]
00000407  B304              mov bl,0x4
00000409  F6E3              mul bl
0000040B  BFE6A3            mov di,0xa3e6
0000040E  03F8              add di,ax
00000410  E87204            call 0x885
00000413  58                pop ax
00000414  D0E0              shl al,0x0
00000416  040B              add al,0xb
00000418  2A061DBB          sub al,[0xbb1d]
0000041C  D0E8              shr al,0x0
0000041E  92                xchg ax,dx
0000041F  B601              mov dh,0x1
00000421  BF1DBB            mov di,0xbb1d
00000424  E88404            call 0x8ab
00000427  B447              mov ah,0x47
00000429  32D2              xor dl,dl
0000042B  BE6CB1            mov si,0xb16c
0000042E  CD21              int byte 0x21
00000430  B419              mov ah,0x19
00000432  CD21              int byte 0x21
00000434  0441              add al,0x41
00000436  A269B1            mov [0xb169],al
00000439  BE9EBB            mov si,0xbb9e
0000043C  807C023A          cmp byte [si+0x2],0x3a
00000440  7525              jnz 0x467
00000442  807C035C          cmp byte [si+0x3],0x5c
00000446  7462              jz 0x4aa
00000448  8A4401            mov al,[si+0x1]
0000044B  E8740F            call 0x13c2
0000044E  BF3EBC            mov di,0xbc3e
00000451  AA                stosb
00000452  2C40              sub al,0x40
00000454  92                xchg ax,dx
00000455  B83A5C            mov ax,0x5c3a
00000458  AB                stosw
00000459  8BF7              mov si,di
0000045B  B447              mov ah,0x47
0000045D  CD21              int byte 0x21
0000045F  BE3EBC            mov si,0xbc3e
00000462  BF9DBA            mov di,0xba9d
00000465  EB11              jmp 0x478
00000467  BF9DBA            mov di,0xba9d
0000046A  807C015C          cmp byte [si+0x1],0x5c
0000046E  7505              jnz 0x475
00000470  AA                stosb
00000471  B03A              mov al,0x3a
00000473  EB15              jmp 0x48a
00000475  BE69B1            mov si,0xb169
00000478  AC                lodsb
00000479  0AC0              or al,al
0000047B  7403              jz 0x480
0000047D  AA                stosb
0000047E  EBF8              jmp 0x478
00000480  BE9EBB            mov si,0xbb9e
00000483  B05C              mov al,0x5c
00000485  3845FF            cmp [di-0x1],al
00000488  7401              jz 0x48b
0000048A  AA                stosb
0000048B  AC                lodsb
0000048C  98                cbw
0000048D  807C013A          cmp byte [si+0x1],0x3a
00000491  7504              jnz 0x497
00000493  46                inc si
00000494  46                inc si
00000495  48                dec ax
00000496  48                dec ax
00000497  91                xchg ax,cx
00000498  F3A4              rep movsb
0000049A  81EF9DBA          sub di,0xba9d
0000049E  97                xchg ax,di
0000049F  BE9CBA            mov si,0xba9c
000004A2  8804              mov [si],al
000004A4  BF9EBB            mov di,0xbb9e
000004A7  E8780A            call 0xf22
000004AA  E83A02            call 0x6e7
000004AD  EB7F              jmp 0x52e
000004AF  1E                push ds
000004B0  8E0672AF          mov es,word [0xaf72]
000004B4  33FF              xor di,di
000004B6  A0ACB1            mov al,[0xb1ac]
000004B9  F6268BAF          mul byte [0xaf8b]
000004BD  91                xchg ax,cx
000004BE  8E1E32BD          mov ds,word [0xbd32]
000004C2  8BF7              mov si,di
000004C4  F3A5              rep movsw
000004C6  1F                pop ds
000004C7  A0E0A3            mov al,[0xa3e0]
000004CA  A2C0B9            mov [0xb9c0],al
000004CD  8A0EC0B9          mov cl,[0xb9c0]
000004D1  E8DE04            call 0x9b2
000004D4  753B              jnz 0x511
000004D6  0AC9              or cl,cl
000004D8  753D              jnz 0x517
000004DA  40                inc ax
000004DB  A2C0B9            mov [0xb9c0],al
000004DE  50                push ax
000004DF  E82201            call 0x604
000004E2  33C0              xor ax,ax
000004E4  3C09              cmp al,0x9
000004E6  7D30              jnl 0x518
000004E8  50                push ax
000004E9  B207              mov dl,0x7
000004EB  F6E2              mul dl
000004ED  BF89AE            mov di,0xae89
000004F0  03F8              add di,ax
000004F2  A0C0B9            mov al,[0xb9c0]
000004F5  B23F              mov dl,0x3f
000004F7  F6E2              mul dl
000004F9  03F8              add di,ax
000004FB  58                pop ax
000004FC  40                inc ax
000004FD  50                push ax
000004FE  1E                push ds
000004FF  57                push di
00000500  B103              mov cl,0x3
00000502  D2E0              shl al,cl
00000504  2C07              sub al,0x7
00000506  98                cbw
00000507  97                xchg ax,di
00000508  A071AF            mov al,[0xaf71]
0000050B  E8BC04            call 0x9ca
0000050E  58                pop ax
0000050F  EBD3              jmp 0x4e4
00000511  33C0              xor ax,ax
00000513  0AC9              or cl,cl
00000515  75C4              jnz 0x4db
00000517  C3                ret
00000518  58                pop ax
00000519  1E                push ds
0000051A  BF39A8            mov di,0xa839
0000051D  0AC0              or al,al
0000051F  7403              jz 0x524
00000521  BF40A8            mov di,0xa840
00000524  57                push di
00000525  A071AF            mov al,[0xaf71]
00000528  BF4A00            mov di,0x4a
0000052B  E89C04            call 0x9ca
0000052E  B80100            mov ax,0x1
00000531  CD33              int byte 0x33
00000533  C3                ret
00000534  33D2              xor dx,dx
00000536  B103              mov cl,0x3
00000538  A18BAF            mov ax,[0xaf8b]
0000053B  48                dec ax
0000053C  D3E0              shl ax,cl
0000053E  91                xchg ax,cx
0000053F  B80700            mov ax,0x7
00000542  CD33              int byte 0x33
00000544  B103              mov cl,0x3
00000546  33D2              xor dx,dx
00000548  8B1E71AF          mov bx,[0xaf71]
0000054C  D3E3              shl bx,cl
0000054E  8BCB              mov cx,bx
00000550  40                inc ax
00000551  EBDE              jmp 0x531
00000553  BBD1A8            mov bx,0xa8d1
00000556  53                push bx
00000557  56                push si
00000558  57                push di
00000559  8A07              mov al,[bx]
0000055B  8A1C              mov bl,[si]
0000055D  8A0D              mov cl,[di]
0000055F  BF6FAF            mov di,0xaf6f
00000562  EB0F              jmp 0x573
00000564  BBD1A8            mov bx,0xa8d1
00000567  53                push bx
00000568  56                push si
00000569  57                push di
0000056A  8A07              mov al,[bx]
0000056C  8A1C              mov bl,[si]
0000056E  8A0D              mov cl,[di]
00000570  BF47A8            mov di,0xa847
00000573  80C10E            add cl,0xe
00000576  3AC3              cmp al,bl
00000578  7F01              jg 0x57b
0000057A  93                xchg ax,bx
0000057B  3AC1              cmp al,cl
0000057D  7F01              jg 0x580
0000057F  91                xchg ax,cx
00000580  98                cbw
00000581  50                push ax
00000582  D1E8              shr ax,0x0
00000584  0404              add al,0x4
00000586  91                xchg ax,cx
00000587  BB0309            mov bx,0x903
0000058A  A0D9A3            mov al,[0xa3d9]
0000058D  E82C07            call 0xcbc
00000590  58                pop ax
00000591  0405              add al,0x5
00000593  5F                pop di
00000594  50                push ax
00000595  2A05              sub al,[di]
00000597  D1E8              shr ax,0x0
00000599  48                dec ax
0000059A  92                xchg ax,dx
0000059B  E80D03            call 0x8ab
0000059E  58                pop ax
0000059F  5F                pop di
000005A0  50                push ax
000005A1  2A05              sub al,[di]
000005A3  D1E8              shr ax,0x0
000005A5  92                xchg ax,dx
000005A6  B601              mov dh,0x1
000005A8  4A                dec dx
000005A9  E8FF02            call 0x8ab
000005AC  BF2AA8            mov di,0xa82a
000005AF  5A                pop dx
000005B0  5E                pop si
000005B1  56                push si
000005B2  2A14              sub dl,[si]
000005B4  2A15              sub dl,[di]
000005B6  D1EA              shr dx,0x0
000005B8  4A                dec dx
000005B9  B602              mov dh,0x2
000005BB  E8ED02            call 0x8ab
000005BE  5F                pop di
000005BF  E8C302            call 0x885
000005C2  B007              mov al,0x7
000005C4  E85002            call 0x817
000005C7  E81D01            call 0x6e7
000005CA  E861FF            call 0x52e
000005CD  B401              mov ah,0x1
000005CF  CD16              int byte 0x16
000005D1  750A              jnz 0x5dd
000005D3  E87204            call 0xa48
000005D6  7505              jnz 0x5dd
000005D8  E8F2FE            call 0x4cd
000005DB  EBF0              jmp 0x5cd
000005DD  33F6              xor si,si
000005DF  4E                dec si
000005E0  B401              mov ah,0x1
000005E2  CD16              int byte 0x16
000005E4  7407              jz 0x5ed
000005E6  E8D403            call 0x9bd
000005E9  8BF0              mov si,ax
000005EB  EB0E              jmp 0x5fb
000005ED  E85804            call 0xa48
000005F0  7409              jz 0x5fb
000005F2  8B367EB0          mov si,[0xb07e]
000005F6  E84F04            call 0xa48
000005F9  75FB              jnz 0x5f6
000005FB  83FEFF            cmp si,0xffffffffffffffff
000005FE  74E0              jz 0x5e0
00000600  89365BBD          mov [0xbd5b],si
00000604  B80200            mov ax,0x2
00000607  CD33              int byte 0x33
00000609  C3                ret
0000060A  C606B9BD00        mov byte [0xbdb9],0x0
0000060F  B8001A            mov ax,0x1a00
00000612  CD10              int byte 0x10
00000614  3C1A              cmp al,0x1a
00000616  7509              jnz 0x621
00000618  93                xchg ax,bx
00000619  3C08              cmp al,0x8
0000061B  7704              ja 0x621
0000061D  3C04              cmp al,0x4
0000061F  7302              jnc 0x623
00000621  B000              mov al,0x0
00000623  A2BBBD            mov [0xbdbb],al
00000626  E80B00            call 0x634
00000629  A271AF            mov [0xaf71],al
0000062C  7505              jnz 0x633
0000062E  C606B9BD01        mov byte [0xbdb9],0x1
00000633  C3                ret
00000634  8E0689AF          mov es,word [0xaf89]
00000638  26A08400          mov al,[es:0x84]
0000063C  3C2A              cmp al,0x2a
0000063E  7402              jz 0x642
00000640  3C31              cmp al,0x31
00000642  C3                ret
00000643  8E0689AF          mov es,word [0xaf89]
00000647  268B165000        mov dx,[es:0x50]
0000064C  C3                ret
0000064D  2EC606BABD01      mov byte [cs:0xbdba],0x1
00000653  CF                iret
00000654  32FF              xor bh,bh
00000656  B408              mov ah,0x8
00000658  CD10              int byte 0x10
0000065A  C3                ret
0000065B  B40F              mov ah,0xf
0000065D  CD10              int byte 0x10
0000065F  247F              and al,0x7f
00000661  A2B8BD            mov [0xbdb8],al
00000664  88268BAF          mov [0xaf8b],ah
00000668  98                cbw
00000669  3C07              cmp al,0x7
0000066B  7505              jnz 0x672
0000066D  C60673AFB0        mov byte [0xaf73],0xb0
00000672  50                push ax
00000673  E8BEFF            call 0x634
00000676  58                pop ax
00000677  7502              jnz 0x67b
00000679  B401              mov ah,0x1
0000067B  C3                ret
0000067C  B117              mov cl,0x17
0000067E  BB0515            mov bx,0x1505
00000681  BF07AF            mov di,0xaf07
00000684  E83206            call 0xcb9
00000687  33C0              xor ax,ax
00000689  50                push ax
0000068A  40                inc ax
0000068B  3C1A              cmp al,0x1a
0000068D  7F3C              jg 0x6cb
0000068F  50                push ax
00000690  B140              mov cl,0x40
00000692  02C8              add cl,al
00000694  880E24AF          mov [0xaf24],cl
00000698  E89C12            call 0x1937
0000069B  732B              jnc 0x6c8
0000069D  8BF2              mov si,dx
0000069F  59                pop cx
000006A0  5A                pop dx
000006A1  42                inc dx
000006A2  52                push dx
000006A3  4A                dec dx
000006A4  80FA0C            cmp dl,0xc
000006A7  7E05              jng 0x6ae
000006A9  80EA0D            sub dl,0xd
000006AC  B616              mov dh,0x16
000006AE  86F2              xchg dh,dl
000006B0  51                push cx
000006B1  1E                push ds
000006B2  BF9CBA            mov di,0xba9c
000006B5  57                push di
000006B6  56                push si
000006B7  50                push ax
000006B8  BF1DAF            mov di,0xaf1d
000006BB  E8ED01            call 0x8ab
000006BE  58                pop ax
000006BF  5B                pop bx
000006C0  B10C              mov cl,0xc
000006C2  E83A0B            call 0x11ff
000006C5  E8DA01            call 0x8a2
000006C8  58                pop ax
000006C9  EBBF              jmp 0x68a
000006CB  58                pop ax
000006CC  E8FBFE            call 0x5ca
000006CF  EB13              jmp 0x6e4
000006D1  B114              mov cl,0x14
000006D3  BB090D            mov bx,0xd09
000006D6  BFDDA8            mov di,0xa8dd
000006D9  E8DD05            call 0xcb9
000006DC  BF8EBC            mov di,0xbc8e
000006DF  B020              mov al,0x20
000006E1  E8D51D            call 0x24b9
000006E4  E8C8FD            call 0x4af
000006E7  8B168AAF          mov dx,[0xaf8a]
000006EB  FECE              dec dh
000006ED  8B0E70AF          mov cx,[0xaf70]
000006F1  8816B4BD          mov [0xbdb4],dl
000006F5  8836B6BD          mov [0xbdb6],dh
000006F9  880EB5BD          mov [0xbdb5],cl
000006FD  882EB7BD          mov [0xbdb7],ch
00000701  C3                ret
00000702  8A3EB3BD          mov bh,[0xbdb3]
00000706  8B0EB4BD          mov cx,[0xbdb4]
0000070A  8B16B6BD          mov dx,[0xbdb6]
0000070E  EB0F              jmp 0x71f
00000710  E830FF            call 0x643
00000713  8BCA              mov cx,dx
00000715  8A16B6BD          mov dl,[0xbdb6]
00000719  8A3EB3BD          mov bh,[0xbdb3]
0000071D  32C0              xor al,al
0000071F  B406              mov ah,0x6
00000721  55                push bp
00000722  CD10              int byte 0x10
00000724  5D                pop bp
00000725  C3                ret
00000726  B03E              mov al,0x3e
00000728  BA0E42            mov dx,0x420e
0000072B  BB0513            mov bx,0x1305
0000072E  BE72AA            mov si,0xaa72
00000731  E8F005            call 0xd24
00000734  B01F              mov al,0x1f
00000736  E88D00            call 0x7c6
00000739  33C0              xor ax,ax
0000073B  8BD0              mov dx,ax
0000073D  B104              mov cl,0x4
0000073F  BEB8A4            mov si,0xa4b8
00000742  0AD2              or dl,dl
00000744  7546              jnz 0x78c
00000746  81FEB5A5          cmp si,0xa5b5
0000074A  7D31              jnl 0x77d
0000074C  AC                lodsb
0000074D  50                push ax
0000074E  D2E8              shr al,cl
00000750  3C0C              cmp al,0xc
00000752  7F0E              jg 0x762
00000754  42                inc dx
00000755  8BF8              mov di,ax
00000757  8A858CA4          mov al,[di-0x5b74]
0000075B  E8AE00            call 0x80c
0000075E  32E4              xor ah,ah
00000760  EBE0              jmp 0x742
00000762  3C0E              cmp al,0xe
00000764  7C09              jl 0x76f
00000766  58                pop ax
00000767  8BF8              mov di,ax
00000769  8A85B9A3          mov al,[di-0x5c47]
0000076D  EBEC              jmp 0x75b
0000076F  5B                pop bx
00000770  83E30F            and bx,0xf
00000773  B020              mov al,0x20
00000775  E89F00            call 0x817
00000778  4B                dec bx
00000779  75F8              jnz 0x773
0000077B  EBE1              jmp 0x75e
0000077D  BF4DA6            mov di,0xa64d
00000780  E80201            call 0x885
00000783  E861FF            call 0x6e7
00000786  8A36C9B9          mov dh,[0xb9c9]
0000078A  EB53              jmp 0x7df
0000078C  58                pop ax
0000078D  240F              and al,0xf
0000078F  3C0C              cmp al,0xc
00000791  7F03              jg 0x796
00000793  4A                dec dx
00000794  EBBF              jmp 0x755
00000796  3C0E              cmp al,0xe
00000798  7C0C              jl 0x7a6
0000079A  D2E0              shl al,cl
0000079C  8BD8              mov bx,ax
0000079E  AC                lodsb
0000079F  50                push ax
000007A0  D2E8              shr al,cl
000007A2  02C3              add al,bl
000007A4  EBC1              jmp 0x767
000007A6  AC                lodsb
000007A7  50                push ax
000007A8  D2E8              shr al,cl
000007AA  8BD8              mov bx,ax
000007AC  EBC5              jmp 0x773
000007AE  8B16C0BC          mov dx,[0xbcc0]
000007B2  42                inc dx
000007B3  B101              mov cl,0x1
000007B5  8AF2              mov dh,dl
000007B7  80C625            add dh,0x25
000007BA  8A2EC4BC          mov ch,[0xbcc4]
000007BE  EB12              jmp 0x7d2
000007C0  E824FF            call 0x6e7
000007C3  A0E3A3            mov al,[0xa3e3]
000007C6  A2B3BD            mov [0xbdb3],al
000007C9  32C0              xor al,al
000007CB  E834FF            call 0x702
000007CE  EB05              jmp 0x7d5
000007D0  FECD              dec ch
000007D2  E81CFF            call 0x6f1
000007D5  8B16B4BD          mov dx,[0xbdb4]
000007D9  EB04              jmp 0x7df
000007DB  0316B4BD          add dx,[0xbdb4]
000007DF  32FF              xor bh,bh
000007E1  B402              mov ah,0x2
000007E3  CD10              int byte 0x10
000007E5  C3                ret
000007E6  B90100            mov cx,0x1
000007E9  8A1EB3BD          mov bl,[0xbdb3]
000007ED  32FF              xor bh,bh
000007EF  B409              mov ah,0x9
000007F1  CD10              int byte 0x10
000007F3  C3                ret
000007F4  FEC6              inc dh
000007F6  3A36B7BD          cmp dh,[0xbdb7]
000007FA  760F              jna 0x80b
000007FC  8A36B7BD          mov dh,[0xbdb7]
00000800  53                push bx
00000801  51                push cx
00000802  52                push dx
00000803  B001              mov al,0x1
00000805  E8FAFE            call 0x702
00000808  5A                pop dx
00000809  59                pop cx
0000080A  5B                pop bx
0000080B  C3                ret
0000080C  3C0A              cmp al,0xa
0000080E  7507              jnz 0x817
00000810  B00D              mov al,0xd
00000812  E80200            call 0x817
00000815  B00A              mov al,0xa
00000817  53                push bx
00000818  51                push cx
00000819  52                push dx
0000081A  1E                push ds
0000081B  06                push es
0000081C  0E                push cs
0000081D  1F                pop ds
0000081E  E822FE            call 0x643
00000821  3C0D              cmp al,0xd
00000823  7437              jz 0x85c
00000825  3C0A              cmp al,0xa
00000827  741A              jz 0x843
00000829  3C09              cmp al,0x9
0000082B  7435              jz 0x862
0000082D  3C08              cmp al,0x8
0000082F  7422              jz 0x853
00000831  3C07              cmp al,0x7
00000833  7418              jz 0x84d
00000835  E8AEFF            call 0x7e6
00000838  42                inc dx
00000839  3A16B6BD          cmp dl,[0xbdb6]
0000083D  7607              jna 0x846
0000083F  8A16B4BD          mov dl,[0xbdb4]
00000843  E8AEFF            call 0x7f4
00000846  E896FF            call 0x7df
00000849  07                pop es
0000084A  1F                pop ds
0000084B  EBBB              jmp 0x808
0000084D  B40E              mov ah,0xe
0000084F  CD10              int byte 0x10
00000851  EBF6              jmp 0x849
00000853  3A16B4BD          cmp dl,[0xbdb4]
00000857  76F0              jna 0x849
00000859  4A                dec dx
0000085A  EBEA              jmp 0x846
0000085C  8A16B4BD          mov dl,[0xbdb4]
00000860  EBE4              jmp 0x846
00000862  8AC2              mov al,dl
00000864  2A06B4BD          sub al,[0xbdb4]
00000868  0408              add al,0x8
0000086A  24F8              and al,0xf8
0000086C  0206B4BD          add al,[0xbdb4]
00000870  2AC2              sub al,dl
00000872  98                cbw
00000873  91                xchg ax,cx
00000874  B020              mov al,0x20
00000876  E870FF            call 0x7e9
00000879  EBCB              jmp 0x846
0000087B  BF2AA8            mov di,0xa82a
0000087E  EB05              jmp 0x885
00000880  57                push di
00000881  E88CFF            call 0x810
00000884  5F                pop di
00000885  1E                push ds
00000886  07                pop es
00000887  EB27              jmp 0x8b0
00000889  B10C              mov cl,0xc
0000088B  BB090D            mov bx,0xd09
0000088E  BF6FAF            mov di,0xaf6f
00000891  E82504            call 0xcb9
00000894  BA0300            mov dx,0x3
00000897  BFEFA8            mov di,0xa8ef
0000089A  EB0F              jmp 0x8ab
0000089C  B601              mov dh,0x1
0000089E  D0EA              shr dl,0x0
000008A0  EB0B              jmp 0x8ad
000008A2  58                pop ax
000008A3  5F                pop di
000008A4  07                pop es
000008A5  50                push ax
000008A6  EB08              jmp 0x8b0
000008A8  BF31AF            mov di,0xaf31
000008AB  1E                push ds
000008AC  07                pop es
000008AD  E82BFF            call 0x7db
000008B0  268A05            mov al,[es:di]
000008B3  47                inc di
000008B4  32E4              xor ah,ah
000008B6  91                xchg ax,cx
000008B7  E309              jcxz 0x8c2
000008B9  268A05            mov al,[es:di]
000008BC  E858FF            call 0x817
000008BF  47                inc di
000008C0  E2F7              loop 0x8b9
000008C2  803EBABD00        cmp byte [0xbdba],0x0
000008C7  7419              jz 0x8e2
000008C9  C606BABD00        mov byte [0xbdba],0x0
000008CE  B401              mov ah,0x1
000008D0  CD16              int byte 0x16
000008D2  7406              jz 0x8da
000008D4  B400              mov ah,0x0
000008D6  CD16              int byte 0x16
000008D8  EBF4              jmp 0x8ce
000008DA  BFB0AF            mov di,0xafb0
000008DD  E8D0FF            call 0x8b0
000008E0  CD23              int byte 0x23
000008E2  C3                ret
000008E3  E875FD            call 0x65b
000008E6  8B1662BD          mov dx,[0xbd62]
000008EA  3BC2              cmp ax,dx
000008EC  74F4              jz 0x8e2
000008EE  92                xchg ax,dx
000008EF  8E0689AF          mov es,word [0xaf89]
000008F3  2680268700FE      and byte [es:0x87],0xfe
000008F9  50                push ax
000008FA  B400              mov ah,0x0
000008FC  CD10              int byte 0x10
000008FE  58                pop ax
000008FF  A90001            test ax,0x100
00000902  7432              jz 0x936
00000904  E803FD            call 0x60a
00000907  803EBBBD00        cmp byte [0xbdbb],0x0
0000090C  7428              jz 0x936
0000090E  32DB              xor bl,bl
00000910  B81211            mov ax,0x1112
00000913  CD10              int byte 0x10
00000915  8E0689AF          mov es,word [0xaf89]
00000919  2680268700FE      and byte [es:0x87],0xfe
0000091F  B90706            mov cx,0x607
00000922  803EB8BD07        cmp byte [0xbdb8],0x7
00000927  7404              jz 0x92d
00000929  B401              mov ah,0x1
0000092B  CD10              int byte 0x10
0000092D  E8DAFC            call 0x60a
00000930  B320              mov bl,0x20
00000932  B412              mov ah,0x12
00000934  CD10              int byte 0x10
00000936  E822FD            call 0x65b
00000939  E8CEFC            call 0x60a
0000093C  E8A8FD            call 0x6e7
0000093F  E812FD            call 0x654
00000942  8826B3BD          mov [0xbdb3],ah
00000946  C3                ret
00000947  33D2              xor dx,dx
00000949  3B1659BD          cmp dx,[0xbd59]
0000094D  7D14              jnl 0x963
0000094F  8BC2              mov ax,dx
00000951  E8DE18            call 0x2232
00000954  268A05            mov al,[es:di]
00000957  A801              test al,0x1
00000959  7503              jnz 0x95e
0000095B  40                inc ax
0000095C  EB01              jmp 0x95f
0000095E  48                dec ax
0000095F  AA                stosb
00000960  42                inc dx
00000961  EBE6              jmp 0x949
00000963  A159BD            mov ax,[0xbd59]
00000966  2B06BEBC          sub ax,[0xbcbe]
0000096A  A3BEBC            mov [0xbcbe],ax
0000096D  C4066CBD          les ax,word [0xbd6c]
00000971  8CC2              mov dx,es
00000973  2B067ABD          sub ax,[0xbd7a]
00000977  1B167CBD          sbb dx,[0xbd7c]
0000097B  A37ABD            mov [0xbd7a],ax
0000097E  89167CBD          mov [0xbd7c],dx
00000982  A1C4BC            mov ax,[0xbcc4]
00000985  3B06C6BC          cmp ax,[0xbcc6]
00000989  7F08              jg 0x993
0000098B  50                push ax
0000098C  E83607            call 0x10c5
0000098F  58                pop ax
00000990  40                inc ax
00000991  EBF2              jmp 0x985
00000993  1E                push ds
00000994  A0C0B9            mov al,[0xb9c0]
00000997  A2E0A3            mov [0xa3e0],al
0000099A  A0ACB1            mov al,[0xb1ac]
0000099D  F6268BAF          mul byte [0xaf8b]
000009A1  8E0632BD          mov es,word [0xbd32]
000009A5  33FF              xor di,di
000009A7  8BF7              mov si,di
000009A9  8E1E72AF          mov ds,word [0xaf72]
000009AD  91                xchg ax,cx
000009AE  F3A5              rep movsw
000009B0  1F                pop ds
000009B1  C3                ret
000009B2  B402              mov ah,0x2
000009B4  CD16              int byte 0x16
000009B6  2408              and al,0x8
000009B8  7402              jz 0x9bc
000009BA  B001              mov al,0x1
000009BC  C3                ret
000009BD  CD28              int byte 0x28
000009BF  B401              mov ah,0x1
000009C1  CD16              int byte 0x16
000009C3  74F8              jz 0x9bd
000009C5  B400              mov ah,0x0
000009C7  CD16              int byte 0x16
000009C9  C3                ret
000009CA  98                cbw
000009CB  F6268BAF          mul byte [0xaf8b]
000009CF  03F8              add di,ax
000009D1  D1E7              shl di,0x0
000009D3  8E0672AF          mov es,word [0xaf72]
000009D7  8CDB              mov bx,ds
000009D9  58                pop ax
000009DA  5E                pop si
000009DB  1F                pop ds
000009DC  50                push ax
000009DD  33C9              xor cx,cx
000009DF  AC                lodsb
000009E0  8AC8              mov cl,al
000009E2  A4                movsb
000009E3  47                inc di
000009E4  E2FC              loop 0x9e2
000009E6  8EDB              mov ds,bx
000009E8  C3                ret
000009E9  E8D4FD            call 0x7c0
000009EC  E81900            call 0xa08
000009EF  BE1BBA            mov si,0xba1b
000009F2  1E                push ds
000009F3  BF6FAF            mov di,0xaf6f
000009F6  57                push di
000009F7  E8990D            call 0x1793
000009FA  E8E6FE            call 0x8e3
000009FD  E8AFFA            call 0x4af
00000A00  B90020            mov cx,0x2000
00000A03  B401              mov ah,0x1
00000A05  CD10              int byte 0x10
00000A07  C3                ret
00000A08  A062BD            mov al,[0xbd62]
00000A0B  B90706            mov cx,0x607
00000A0E  3C07              cmp al,0x7
00000A10  75F1              jnz 0xa03
00000A12  B90D0C            mov cx,0xc0d
00000A15  EBEC              jmp 0xa03
00000A17  A062BD            mov al,[0xbd62]
00000A1A  B90700            mov cx,0x7
00000A1D  3C07              cmp al,0x7
00000A1F  75E2              jnz 0xa03
00000A21  B10D              mov cl,0xd
00000A23  EBDE              jmp 0xa03
00000A25  92                xchg ax,dx
00000A26  8A3EDCA3          mov bh,[0xa3dc]
00000A2A  32DB              xor bl,bl
00000A2C  B90002            mov cx,0x200
00000A2F  8A36C9B9          mov dh,[0xb9c9]
00000A33  FECE              dec dh
00000A35  FECE              dec dh
00000A37  B80106            mov ax,0x601
00000A3A  80FA01            cmp dl,0x1
00000A3D  7402              jz 0xa41
00000A3F  FEC4              inc ah
00000A41  B24F              mov dl,0x4f
00000A43  55                push bp
00000A44  CD10              int byte 0x10
00000A46  5D                pop bp
00000A47  C3                ret
00000A48  B80300            mov ax,0x3
00000A4B  CD33              int byte 0x33
00000A4D  0ADB              or bl,bl
00000A4F  7418              jz 0xa69
00000A51  91                xchg ax,cx
00000A52  B103              mov cl,0x3
00000A54  D3E8              shr ax,cl
00000A56  40                inc ax
00000A57  A3ADB1            mov [0xb1ad],ax
00000A5A  D3EA              shr dx,cl
00000A5C  42                inc dx
00000A5D  8916AFB1          mov [0xb1af],dx
00000A61  B800F0            mov ax,0xf000
00000A64  2AE3              sub ah,bl
00000A66  A37EB0            mov [0xb07e],ax
00000A69  C3                ret
00000A6A  80E407            and ah,0x7
00000A6D  B105              mov cl,0x5
00000A6F  D3E8              shr ax,cl
00000A71  8BDC              mov bx,sp
00000A73  C47F02            les di,word [bx+0x2]
00000A76  92                xchg ax,dx
00000A77  B002              mov al,0x2
00000A79  AA                stosb
00000A7A  92                xchg ax,dx
00000A7B  32E4              xor ah,ah
00000A7D  B164              mov cl,0x64
00000A7F  F6F1              div cl
00000A81  86E0              xchg ah,al
00000A83  D40A              aam
00000A85  86E0              xchg ah,al
00000A87  053030            add ax,0x3030
00000A8A  AB                stosw
00000A8B  C3                ret
00000A8C  AC                lodsb
00000A8D  98                cbw
00000A8E  8BD6              mov dx,si
00000A90  03F0              add si,ax
00000A92  8824              mov [si],ah
00000A94  C3                ret
00000A95  0000              add [bx+si],al
00000A97  0000              add [bx+si],al
00000A99  FB                sti
00000A9A  80FC02            cmp ah,0x2
00000A9D  7423              jz 0xac2
00000A9F  80FC06            cmp ah,0x6
00000AA2  7419              jz 0xabd
00000AA4  80FC09            cmp ah,0x9
00000AA7  7421              jz 0xaca
00000AA9  80FC40            cmp ah,0x40
00000AAC  750A              jnz 0xab8
00000AAE  83FB01            cmp bx,0x1
00000AB1  742B              jz 0xade
00000AB3  83FB02            cmp bx,0x2
00000AB6  7426              jz 0xade
00000AB8  2EFF2E950B        jmp word far [cs:0xb95]
00000ABD  80FAFF            cmp dl,0xff
00000AC0  74F6              jz 0xab8
00000AC2  50                push ax
00000AC3  8AC2              mov al,dl
00000AC5  E84FFD            call 0x817
00000AC8  EB11              jmp 0xadb
00000ACA  50                push ax
00000ACB  53                push bx
00000ACC  8BDA              mov bx,dx
00000ACE  8A07              mov al,[bx]
00000AD0  3C24              cmp al,0x24
00000AD2  7406              jz 0xada
00000AD4  E840FD            call 0x817
00000AD7  43                inc bx
00000AD8  EBF4              jmp 0xace
00000ADA  5B                pop bx
00000ADB  58                pop ax
00000ADC  EB14              jmp 0xaf2
00000ADE  E312              jcxz 0xaf2
00000AE0  50                push ax
00000AE1  53                push bx
00000AE2  51                push cx
00000AE3  8BDA              mov bx,dx
00000AE5  8A07              mov al,[bx]
00000AE7  E82DFD            call 0x817
00000AEA  43                inc bx
00000AEB  E2F8              loop 0xae5
00000AED  59                pop cx
00000AEE  5B                pop bx
00000AEF  58                pop ax
00000AF0  8BC1              mov ax,cx
00000AF2  F8                clc
00000AF3  CA0200            retf word 0x2
00000AF6  8B1EC4B9          mov bx,[0xb9c4]
00000AFA  81C35122          add bx,0x2251
00000AFE  E93A05            jmp 0x103b
00000B01  E93305            jmp 0x1037
00000B04  268A05            mov al,[es:di]
00000B07  A880              test al,0x80
00000B09  7475              jz 0xb80
00000B0B  8CC3              mov bx,es
00000B0D  8CDA              mov dx,ds
00000B0F  8BF7              mov si,di
00000B11  BFCABC            mov di,0xbcca
00000B14  B93400            mov cx,0x34
00000B17  8EC2              mov es,dx
00000B19  8EDB              mov ds,bx
00000B1B  F3A5              rep movsw
00000B1D  8EDA              mov ds,dx
00000B1F  BFCABC            mov di,0xbcca
00000B22  803EE7BC14        cmp byte [0xbce7],0x14
00000B27  7208              jc 0xb31
00000B29  252000            and ax,0x20
00000B2C  A38DBD            mov [0xbd8d],ax
00000B2F  EB12              jmp 0xb43
00000B31  2420              and al,0x20
00000B33  A29CBD            mov [0xbd9c],al
00000B36  A08EBC            mov al,[0xbc8e]
00000B39  0AC0              or al,al
00000B3B  7402              jz 0xb3f
00000B3D  B001              mov al,0x1
00000B3F  0806A1BD          or [0xbda1],al
00000B43  8B5518            mov dx,[di+0x18]
00000B46  8B4D1A            mov cx,[di+0x1a]
00000B49  8A451C            mov al,[di+0x1c]
00000B4C  A29FBD            mov [0xbd9f],al
00000B4F  8A4517            mov al,[di+0x17]
00000B52  247F              and al,0x7f
00000B54  A2E4A3            mov [0xa3e4],al
00000B57  8B4513            mov ax,[di+0x13]
00000B5A  A393BD            mov [0xbd93],ax
00000B5D  8B4515            mov ax,[di+0x15]
00000B60  A395BD            mov [0xbd95],ax
00000B63  32E4              xor ah,ah
00000B65  8A451D            mov al,[di+0x1d]
00000B68  803EA0BD03        cmp byte [0xbda0],0x3
00000B6D  7505              jnz 0xb74
00000B6F  A3A1BD            mov [0xbda1],ax
00000B72  EB32              jmp 0xba6
00000B74  803EA0BD06        cmp byte [0xbda0],0x6
00000B79  7508              jnz 0xb83
00000B7B  A2A3BD            mov [0xbda3],al
00000B7E  EB26              jmp 0xba6
00000B80  33C0              xor ax,ax
00000B82  C3                ret
00000B83  803EA0BD18        cmp byte [0xbda0],0x18
00000B88  721C              jc 0xba6
00000B8A  3C14              cmp al,0x14
00000B8C  7215              jc 0xba3
00000B8E  8A05              mov al,[di]
00000B90  2404              and al,0x4
00000B92  7407              jz 0xb9b
00000B94  830EA1BD20        or word [0xbda1],0x20
00000B99  EB0B              jmp 0xba6
00000B9B  8126A1BDDF00      and word [0xbda1],0xdf
00000BA1  EB03              jmp 0xba6
00000BA3  A29CBD            mov [0xbd9c],al
00000BA6  8D7507            lea si,[di+0x7]
00000BA9  BF7FBD            mov di,0xbd7f
00000BAC  1E                push ds
00000BAD  07                pop es
00000BAE  A5                movsw
00000BAF  A5                movsw
00000BB0  A5                movsw
00000BB1  A5                movsw
00000BB2  E81E04            call 0xfd3
00000BB5  803EA0BD03        cmp byte [0xbda0],0x3
00000BBA  751D              jnz 0xbd9
00000BBC  50                push ax
00000BBD  52                push dx
00000BBE  B98200            mov cx,0x82
00000BC1  E8FA03            call 0xfbe
00000BC4  83C61A            add si,0x1a
00000BC7  AD                lodsw
00000BC8  93                xchg ax,bx
00000BC9  AD                lodsw
00000BCA  03C3              add ax,bx
00000BCC  051E00            add ax,0x1e
00000BCF  59                pop cx
00000BD0  5A                pop dx
00000BD1  03D0              add dx,ax
00000BD3  83D100            adc cx,0x0
00000BD6  E8FA03            call 0xfd3
00000BD9  A1B4B1            mov ax,[0xb1b4]
00000BDC  A38FBD            mov [0xbd8f],ax
00000BDF  B001              mov al,0x1
00000BE1  C3                ret
00000BE2  803EE5A300        cmp byte [0xa3e5],0x0
00000BE7  7497              jz 0xb80
00000BE9  E81CFE            call 0xa08
00000BEC  803E9FBD00        cmp byte [0xbd9f],0x0
00000BF1  7505              jnz 0xbf8
00000BF3  B8F09B            mov ax,0x9bf0
00000BF6  EB6B              jmp 0xc63
00000BF8  A0A0BD            mov al,[0xbda0]
00000BFB  3C06              cmp al,0x6
00000BFD  7404              jz 0xc03
00000BFF  3C02              cmp al,0x2
00000C01  7705              ja 0xc08
00000C03  B80080            mov ax,0x8000
00000C06  EB5B              jmp 0xc63
00000C08  3C0C              cmp al,0xc
00000C0A  750E              jnz 0xc1a
00000C0C  B80080            mov ax,0x8000
00000C0F  803E9FBD02        cmp byte [0xbd9f],0x2
00000C14  754D              jnz 0xc63
00000C16  D1E8              shr ax,0x0
00000C18  EB49              jmp 0xc63
00000C1A  3C03              cmp al,0x3
00000C1C  7507              jnz 0xc25
00000C1E  A09FBD            mov al,[0xbd9f]
00000C21  3C01              cmp al,0x1
00000C23  752B              jnz 0xc50
00000C25  3C0A              cmp al,0xa
00000C27  7507              jnz 0xc30
00000C29  803E9FBD02        cmp byte [0xbd9f],0x2
00000C2E  74D3              jz 0xc03
00000C30  3C13              cmp al,0x13
00000C32  7505              jnz 0xc39
00000C34  B8FCFE            mov ax,0xfefc
00000C37  EB2A              jmp 0xc63
00000C39  3C18              cmp al,0x18
00000C3B  7205              jc 0xc42
00000C3D  B8EFFE            mov ax,0xfeef
00000C40  EB21              jmp 0xc63
00000C42  3C11              cmp al,0x11
00000C44  7505              jnz 0xc4b
00000C46  B8E079            mov ax,0x79e0
00000C49  EB18              jmp 0xc63
00000C4B  B8D95B            mov ax,0x5bd9
00000C4E  EB13              jmp 0xc63
00000C50  3C06              cmp al,0x6
00000C52  7405              jz 0xc59
00000C54  B80080            mov ax,0x8000
00000C57  EB0A              jmp 0xc63
00000C59  B80020            mov ax,0x2000
00000C5C  F606A1BD02        test byte [0xbda1],0x2
00000C61  75B3              jnz 0xc16
00000C63  C41E83BD          les bx,word [0xbd83]
00000C67  8CC2              mov dx,es
00000C69  93                xchg ax,bx
00000C6A  F7F3              div bx
00000C6C  0BD2              or dx,dx
00000C6E  7401              jz 0xc71
00000C70  40                inc ax
00000C71  B201              mov dl,0x1
00000C73  8BC8              mov cx,ax
00000C75  8BD8              mov bx,ax
00000C77  3D0900            cmp ax,0x9
00000C7A  7F2F              jg 0xcab
00000C7C  8BC3              mov ax,bx
00000C7E  F6F2              div dl
00000C80  4A                dec dx
00000C81  98                cbw
00000C82  A2B9BC            mov [0xbcb9],al
00000C85  8816BABC          mov [0xbcba],dl
00000C89  8816E1A3          mov [0xa3e1],dl
00000C8D  91                xchg ax,cx
00000C8E  B020              mov al,0x20
00000C90  E884FB            call 0x817
00000C93  E315              jcxz 0xcaa
00000C95  E8ABF9            call 0x643
00000C98  8916B7BC          mov [0xbcb7],dx
00000C9C  B0B0              mov al,0xb0
00000C9E  E876FB            call 0x817
00000CA1  E2F9              loop 0xc9c
00000CA3  8B16B7BC          mov dx,[0xbcb7]
00000CA7  E835FB            call 0x7df
00000CAA  C3                ret
00000CAB  B90900            mov cx,0x9
00000CAE  2BC1              sub ax,cx
00000CB0  48                dec ax
00000CB1  42                inc dx
00000CB2  EBC3              jmp 0xc77
00000CB4  B126              mov cl,0x26
00000CB6  BB080D            mov bx,0xd08
00000CB9  A0DEA3            mov al,[0xa3de]
00000CBC  55                push bp
00000CBD  8BEC              mov bp,sp
00000CBF  57                push di
00000CC0  53                push bx
00000CC1  A2B3BD            mov [0xbdb3],al
00000CC4  A08BAF            mov al,[0xaf8b]
00000CC7  98                cbw
00000CC8  8BF0              mov si,ax
00000CCA  D1E6              shl si,0x0
00000CCC  46                inc si
00000CCD  D0E8              shr al,0x0
00000CCF  48                dec ax
00000CD0  8BD0              mov dx,ax
00000CD2  2AD1              sub dl,cl
00000CD4  02C1              add al,cl
00000CD6  8AF0              mov dh,al
00000CD8  52                push dx
00000CD9  43                inc bx
00000CDA  42                inc dx
00000CDB  FEC6              inc dh
00000CDD  FEC7              inc bh
00000CDF  8E0672AF          mov es,word [0xaf72]
00000CE3  8AC7              mov al,bh
00000CE5  98                cbw
00000CE6  F6268BAF          mul byte [0xaf8b]
00000CEA  97                xchg ax,di
00000CEB  8AC2              mov al,dl
00000CED  98                cbw
00000CEE  8ACE              mov cl,dh
00000CF0  2AC8              sub cl,al
00000CF2  03F8              add di,ax
00000CF4  32ED              xor ch,ch
00000CF6  41                inc cx
00000CF7  D1E7              shl di,0x0
00000CF9  B007              mov al,0x7
00000CFB  47                inc di
00000CFC  AA                stosb
00000CFD  E2FC              loop 0xcfb
00000CFF  8ACF              mov cl,bh
00000D01  2ACB              sub cl,bl
00000D03  2BFE              sub di,si
00000D05  AA                stosb
00000D06  E2FB              loop 0xd03
00000D08  8B56FA            mov dx,[bp-0x6]
00000D0B  8B4EFC            mov cx,[bp-0x4]
00000D0E  E8C1FA            call 0x7d2
00000D11  E8B5FA            call 0x7c9
00000D14  5A                pop dx
00000D15  81EAFE01          sub dx,0x1fe
00000D19  5B                pop bx
00000D1A  43                inc bx
00000D1B  FECF              dec bh
00000D1D  5E                pop si
00000D1E  5D                pop bp
00000D1F  EB06              jmp 0xd27
00000D21  A0DEA3            mov al,[0xa3de]
00000D24  A2B3BD            mov [0xbdb3],al
00000D27  55                push bp
00000D28  8BEC              mov bp,sp
00000D2A  52                push dx
00000D2B  53                push bx
00000D2C  8AF3              mov dh,bl
00000D2E  8A56FE            mov dl,[bp-0x2]
00000D31  32FF              xor bh,bh
00000D33  B402              mov ah,0x2
00000D35  CD10              int byte 0x10
00000D37  B8C909            mov ax,0x9c9
00000D3A  B90100            mov cx,0x1
00000D3D  8A1EB3BD          mov bl,[0xbdb3]
00000D41  CD10              int byte 0x10
00000D43  8BC2              mov ax,dx
00000D45  8A66FD            mov ah,[bp-0x3]
00000D48  97                xchg ax,di
00000D49  FEC6              inc dh
00000D4B  B402              mov ah,0x2
00000D4D  CD10              int byte 0x10
00000D4F  3BD7              cmp dx,di
00000D51  7407              jz 0xd5a
00000D53  B8BA09            mov ax,0x9ba
00000D56  CD10              int byte 0x10
00000D58  EBEF              jmp 0xd49
00000D5A  B8C809            mov ax,0x9c8
00000D5D  CD10              int byte 0x10
00000D5F  42                inc dx
00000D60  B402              mov ah,0x2
00000D62  CD10              int byte 0x10
00000D64  8A4EFF            mov cl,[bp-0x1]
00000D67  2ACA              sub cl,dl
00000D69  B8CD09            mov ax,0x9cd
00000D6C  CD10              int byte 0x10
00000D6E  02D1              add dl,cl
00000D70  B402              mov ah,0x2
00000D72  CD10              int byte 0x10
00000D74  B101              mov cl,0x1
00000D76  B8BC09            mov ax,0x9bc
00000D79  CD10              int byte 0x10
00000D7B  8BC2              mov ax,dx
00000D7D  8A66FC            mov ah,[bp-0x4]
00000D80  97                xchg ax,di
00000D81  FECE              dec dh
00000D83  B402              mov ah,0x2
00000D85  CD10              int byte 0x10
00000D87  3BD7              cmp dx,di
00000D89  7407              jz 0xd92
00000D8B  B8BA09            mov ax,0x9ba
00000D8E  CD10              int byte 0x10
00000D90  EBEF              jmp 0xd81
00000D92  B8BB09            mov ax,0x9bb
00000D95  CD10              int byte 0x10
00000D97  8A56FE            mov dl,[bp-0x2]
00000D9A  42                inc dx
00000D9B  B402              mov ah,0x2
00000D9D  CD10              int byte 0x10
00000D9F  8A4EFF            mov cl,[bp-0x1]
00000DA2  1E                push ds
00000DA3  07                pop es
00000DA4  2ACA              sub cl,dl
00000DA6  AC                lodsb
00000DA7  4E                dec si
00000DA8  98                cbw
00000DA9  50                push ax
00000DAA  2AC8              sub cl,al
00000DAC  D1E9              shr cx,0x0
00000DAE  B8CD09            mov ax,0x9cd
00000DB1  CD10              int byte 0x10
00000DB3  02D1              add dl,cl
00000DB5  B402              mov ah,0x2
00000DB7  CD10              int byte 0x10
00000DB9  8BFE              mov di,si
00000DBB  E8F2FA            call 0x8b0
00000DBE  59                pop cx
00000DBF  02D1              add dl,cl
00000DC1  B402              mov ah,0x2
00000DC3  CD10              int byte 0x10
00000DC5  8A4EFF            mov cl,[bp-0x1]
00000DC8  2ACA              sub cl,dl
00000DCA  B8CD09            mov ax,0x9cd
00000DCD  CD10              int byte 0x10
00000DCF  59                pop cx
00000DD0  41                inc cx
00000DD1  5A                pop dx
00000DD2  42                inc dx
00000DD3  FECE              dec dh
00000DD5  5D                pop bp
00000DD6  E9F7F9            jmp 0x7d0
00000DD9  55                push bp
00000DDA  8BEC              mov bp,sp
00000DDC  81EC5001          sub sp,0x150
00000DE0  BE3EBC            mov si,0xbc3e
00000DE3  8A04              mov al,[si]
00000DE5  0AC0              or al,al
00000DE7  7450              jz 0xe39
00000DE9  8D7EB0            lea di,[bp-0x50]
00000DEC  57                push di
00000DED  E83C01            call 0xf2c
00000DF0  4E                dec si
00000DF1  AC                lodsb
00000DF2  5F                pop di
00000DF3  3C5C              cmp al,0x5c
00000DF5  7502              jnz 0xdf9
00000DF7  FE0D              dec byte [di]
00000DF9  8A05              mov al,[di]
00000DFB  0AC0              or al,al
00000DFD  743A              jz 0xe39
00000DFF  98                cbw
00000E00  93                xchg ax,bx
00000E01  80393A            cmp byte [bx+di],0x3a
00000E04  7433              jz 0xe39
00000E06  47                inc di
00000E07  8BD7              mov dx,di
00000E09  C60100            mov byte [bx+di],0x0
00000E0C  52                push dx
00000E0D  8D96B0FE          lea dx,[bp-0x150]
00000E11  B41A              mov ah,0x1a
00000E13  CD21              int byte 0x21
00000E15  B91000            mov cx,0x10
00000E18  5A                pop dx
00000E19  B44E              mov ah,0x4e
00000E1B  CD21              int byte 0x21
00000E1D  7303              jnc 0xe22
00000E1F  F8                clc
00000E20  EB18              jmp 0xe3a
00000E22  8A86C5FE          mov al,[bp-0x13b]
00000E26  A810              test al,0x10
00000E28  74F5              jz 0xe1f
00000E2A  BF3EBC            mov di,0xbc3e
00000E2D  8D76B0            lea si,[bp-0x50]
00000E30  FE04              inc byte [si]
00000E32  E8ED00            call 0xf22
00000E35  4F                dec di
00000E36  B05C              mov al,0x5c
00000E38  AA                stosb
00000E39  F9                stc
00000E3A  8BE5              mov sp,bp
00000E3C  5D                pop bp
00000E3D  C3                ret
00000E3E  55                push bp
00000E3F  8BEC              mov bp,sp
00000E41  83EC50            sub sp,0x50
00000E44  33D2              xor dx,dx
00000E46  8A26D8A3          mov ah,[0xa3d8]
00000E4A  8826B3BD          mov [0xbdb3],ah
00000E4E  BF57A8            mov di,0xa857
00000E51  E857FA            call 0x8ab
00000E54  16                push ss
00000E55  8D7EB0            lea di,[bp-0x50]
00000E58  57                push di
00000E59  1E                push ds
00000E5A  BE9EBB            mov si,0xbb9e
00000E5D  56                push si
00000E5E  E81103            call 0x1172
00000E61  E83EFA            call 0x8a2
00000E64  A170BD            mov ax,[0xbd70]
00000E67  0B0672BD          or ax,[0xbd72]
00000E6B  7405              jz 0xe72
00000E6D  B02A              mov al,0x2a
00000E6F  E8A5F9            call 0x817
00000E72  E89BF8            call 0x710
00000E75  803EA0BD00        cmp byte [0xbda0],0x0
00000E7A  7409              jz 0xe85
00000E7C  BA1700            mov dx,0x17
00000E7F  BF60A8            mov di,0xa860
00000E82  E826FA            call 0x8ab
00000E85  EBB3              jmp 0xe3a
00000E87  32E4              xor ah,ah
00000E89  88269CBA          mov [0xba9c],ah
00000E8D  05BEB1            add ax,0xb1be
00000E90  50                push ax
00000E91  96                xchg ax,si
00000E92  AD                lodsw
00000E93  0BC0              or ax,ax
00000E95  745E              jz 0xef5
00000E97  2D0300            sub ax,0x3
00000E9A  91                xchg ax,cx
00000E9B  AC                lodsb
00000E9C  3C01              cmp al,0x1
00000E9E  750A              jnz 0xeaa
00000EA0  BF1DBB            mov di,0xbb1d
00000EA3  8BC1              mov ax,cx
00000EA5  AA                stosb
00000EA6  F3A4              rep movsb
00000EA8  EBE8              jmp 0xe92
00000EAA  3C02              cmp al,0x2
00000EAC  750A              jnz 0xeb8
00000EAE  BF9CBA            mov di,0xba9c
00000EB1  8BC1              mov ax,cx
00000EB3  AA                stosb
00000EB4  F3A4              rep movsb
00000EB6  EBDA              jmp 0xe92
00000EB8  3C40              cmp al,0x40
00000EBA  7510              jnz 0xecc
00000EBC  AD                lodsw
00000EBD  A3E1BC            mov [0xbce1],ax
00000EC0  251800            and ax,0x18
00000EC3  74CD              jz 0xe92
00000EC5  800ECABC10        or byte [0xbcca],0x10
00000ECA  EBC6              jmp 0xe92
00000ECC  03F1              add si,cx
00000ECE  3CFE              cmp al,0xfe
00000ED0  75C0              jnz 0xe92
00000ED2  E8D814            call 0x23ad
00000ED5  91                xchg ax,cx
00000ED6  8BC6              mov ax,si
00000ED8  2DC1B1            sub ax,0xb1c1
00000EDB  2B06C8BC          sub ax,[0xbcc8]
00000EDF  83DA00            sbb dx,0x0
00000EE2  A159BD            mov ax,[0xbd59]
00000EE5  E84A13            call 0x2232
00000EE8  BF0300            mov di,0x3
00000EEB  03C1              add ax,cx
00000EED  83D200            adc dx,0x0
00000EF0  AB                stosw
00000EF1  92                xchg ax,dx
00000EF2  AB                stosw
00000EF3  EB9D              jmp 0xe92
00000EF5  58                pop ax
00000EF6  803ED2B101        cmp byte [0xb1d2],0x1
00000EFB  7517              jnz 0xf14
00000EFD  2BF0              sub si,ax
00000EFF  96                xchg ax,si
00000F00  48                dec ax
00000F01  48                dec ax
00000F02  2906D1BC          sub [0xbcd1],ax
00000F06  831ED3BC00        sbb word [0xbcd3],0x0
00000F0B  010664BD          add [0xbd64],ax
00000F0F  831666BD00        adc word [0xbd66],0x0
00000F14  BE1DBB            mov si,0xbb1d
00000F17  E81F00            call 0xf39
00000F1A  BE9CBA            mov si,0xba9c
00000F1D  BF1DBB            mov di,0xbb1d
00000F20  1E                push ds
00000F21  07                pop es
00000F22  AC                lodsb
00000F23  98                cbw
00000F24  AA                stosb
00000F25  91                xchg ax,cx
00000F26  F3A4              rep movsb
00000F28  C3                ret
00000F29  BE9CBA            mov si,0xba9c
00000F2C  16                push ss
00000F2D  07                pop es
00000F2E  EBF2              jmp 0xf22
00000F30  BE9EBB            mov si,0xbb9e
00000F33  E80300            call 0xf39
00000F36  BE77B0            mov si,0xb077
00000F39  BF9CBA            mov di,0xba9c
00000F3C  1E                push ds
00000F3D  07                pop es
00000F3E  268A0D            mov cl,[es:di]
00000F41  32ED              xor ch,ch
00000F43  AC                lodsb
00000F44  98                cbw
00000F45  260005            add [es:di],al
00000F48  03F9              add di,cx
00000F4A  47                inc di
00000F4B  EBD8              jmp 0xf25
00000F4D  92                xchg ax,dx
00000F4E  B436              mov ah,0x36
00000F50  CD21              int byte 0x21
00000F52  8BD0              mov dx,ax
00000F54  3DFFFF            cmp ax,0xffff
00000F57  7404              jz 0xf5d
00000F59  F7E1              mul cx
00000F5B  F7E3              mul bx
00000F5D  C3                ret
00000F5E  8BDC              mov bx,sp
00000F60  83EC20            sub sp,0x20
00000F63  8D7FE0            lea di,[bx-0x20]
00000F66  16                push ss
00000F67  07                pop es
00000F68  AC                lodsb
00000F69  98                cbw
00000F6A  8BC8              mov cx,ax
00000F6C  40                inc ax
00000F6D  92                xchg ax,dx
00000F6E  F3A4              rep movsb
00000F70  B03D              mov al,0x3d
00000F72  AA                stosb
00000F73  1E                push ds
00000F74  8E1E2C00          mov ds,word [0x2c]
00000F78  33F6              xor si,si
00000F7A  32C0              xor al,al
00000F7C  3804              cmp [si],al
00000F7E  7411              jz 0xf91
00000F80  8D7FE0            lea di,[bx-0x20]
00000F83  8BCA              mov cx,dx
00000F85  F3A6              repe cmpsb
00000F87  7408              jz 0xf91
00000F89  4E                dec si
00000F8A  AC                lodsb
00000F8B  0AC0              or al,al
00000F8D  75FB              jnz 0xf8a
00000F8F  EBE9              jmp 0xf7a
00000F91  8BFE              mov di,si
00000F93  1E                push ds
00000F94  07                pop es
00000F95  B90001            mov cx,0x100
00000F98  F2AE              repne scasb
00000F9A  F6D1              not cl
00000F9C  36C47F02          les di,word [ss:bx+0x2]
00000FA0  8AC1              mov al,cl
00000FA2  AA                stosb
00000FA3  F3A4              rep movsb
00000FA5  1F                pop ds
00000FA6  8BE3              mov sp,bx
00000FA8  C3                ret
00000FA9  33C9              xor cx,cx
00000FAB  BE9EBB            mov si,0xbb9e
00000FAE  E8DBFA            call 0xa8c
00000FB1  91                xchg ax,cx
00000FB2  33C9              xor cx,cx
00000FB4  B43D              mov ah,0x3d
00000FB6  3C01              cmp al,0x1
00000FB8  7574              jnz 0x102e
00000FBA  FECC              dec ah
00000FBC  EB70              jmp 0x102e
00000FBE  BABEB1            mov dx,0xb1be
00000FC1  8BF2              mov si,dx
00000FC3  8B1EB4B1          mov bx,[0xb1b4]
00000FC7  B43F              mov ah,0x3f
00000FC9  EB63              jmp 0x102e
00000FCB  B90200            mov cx,0x2
00000FCE  8D56F6            lea dx,[bp-0xa]
00000FD1  EB0B              jmp 0xfde
00000FD3  B80042            mov ax,0x4200
00000FD6  EB70              jmp 0x1048
00000FD8  B90400            mov cx,0x4
00000FDB  8D56F8            lea dx,[bp-0x8]
00000FDE  1E                push ds
00000FDF  16                push ss
00000FE0  EB11              jmp 0xff3
00000FE2  81F91E08          cmp cx,0x81e
00000FE6  7603              jna 0xfeb
00000FE8  B91E08            mov cx,0x81e
00000FEB  8E0634BD          mov es,word [0xbd34]
00000FEF  33D2              xor dx,dx
00000FF1  1E                push ds
00000FF2  06                push es
00000FF3  8B1EB4B1          mov bx,[0xb1b4]
00000FF7  1F                pop ds
00000FF8  B43F              mov ah,0x3f
00000FFA  CD21              int byte 0x21
00000FFC  1F                pop ds
00000FFD  C3                ret
00000FFE  E88BFA            call 0xa8c
00001001  B80143            mov ax,0x4301
00001004  33C9              xor cx,cx
00001006  CD21              int byte 0x21
00001008  B441              mov ah,0x41
0000100A  EB22              jmp 0x102e
0000100C  B90143            mov cx,0x4301
0000100F  EB03              jmp 0x1014
00001011  B90043            mov cx,0x4300
00001014  8CDB              mov bx,ds
00001016  58                pop ax
00001017  5E                pop si
00001018  1F                pop ds
00001019  50                push ax
0000101A  53                push bx
0000101B  51                push cx
0000101C  52                push dx
0000101D  E86CFA            call 0xa8c
00001020  59                pop cx
00001021  58                pop ax
00001022  EBD6              jmp 0xffa
00001024  8106C4B98200      add word [0xb9c4],0x82
0000102A  EB0B              jmp 0x1037
0000102C  B43E              mov ah,0x3e
0000102E  CD21              int byte 0x21
00001030  C3                ret
00001031  812EC4B98200      sub word [0xb9c4],0x82
00001037  8B1EC4B9          mov bx,[0xb9c4]
0000103B  B44A              mov ah,0x4a
0000103D  0E                push cs
0000103E  07                pop es
0000103F  EBED              jmp 0x102e
00001041  33C9              xor cx,cx
00001043  8BD1              mov dx,cx
00001045  B80242            mov ax,0x4202
00001048  8B1EB4B1          mov bx,[0xb1b4]
0000104C  EBE0              jmp 0x102e
0000104E  E83BFA            call 0xa8c
00001051  B80043            mov ax,0x4300
00001054  EBD8              jmp 0x102e
00001056  56                push si
00001057  E832FA            call 0xa8c
0000105A  5E                pop si
0000105B  B439              mov ah,0x39
0000105D  EBCF              jmp 0x102e
0000105F  2BD3              sub dx,bx
00001061  3BC3              cmp ax,bx
00001063  7202              jc 0x1067
00001065  8BC3              mov ax,bx
00001067  AA                stosb
00001068  91                xchg ax,cx
00001069  03DE              add bx,si
0000106B  F3A4              rep movsb
0000106D  8BF3              mov si,bx
0000106F  C3                ret
00001070  AC                lodsb
00001071  98                cbw
00001072  92                xchg ax,dx
00001073  8BDA              mov bx,dx
00001075  0BDB              or bx,bx
00001077  740F              jz 0x1088
00001079  8078FF5C          cmp byte [bx+si-0x1],0x5c
0000107D  7409              jz 0x1088
0000107F  8078FF3A          cmp byte [bx+si-0x1],0x3a
00001083  7403              jz 0x1088
00001085  4B                dec bx
00001086  75F1              jnz 0x1079
00001088  C3                ret
00001089  55                push bp
0000108A  8BEC              mov bp,sp
0000108C  1E                push ds
0000108D  C5760C            lds si,word [bp+0xc]
00001090  E8DDFF            call 0x1070
00001093  2BD3              sub dx,bx
00001095  03F3              add si,bx
00001097  33DB              xor bx,bx
00001099  EB06              jmp 0x10a1
0000109B  80382E            cmp byte [bx+si],0x2e
0000109E  7405              jz 0x10a5
000010A0  43                inc bx
000010A1  3BDA              cmp bx,dx
000010A3  75F6              jnz 0x109b
000010A5  B80800            mov ax,0x8
000010A8  C47E08            les di,word [bp+0x8]
000010AB  E8B1FF            call 0x105f
000010AE  8BDA              mov bx,dx
000010B0  B80400            mov ax,0x4
000010B3  C47E04            les di,word [bp+0x4]
000010B6  E8A6FF            call 0x105f
000010B9  1F                pop ds
000010BA  5D                pop bp
000010BB  C20C00            ret word 0xc
000010BE  A1C8BC            mov ax,[0xbcc8]
000010C1  B101              mov cl,0x1
000010C3  EB02              jmp 0x10c7
000010C5  32C9              xor cl,cl
000010C7  32ED              xor ch,ch
000010C9  8B36C4BC          mov si,[0xbcc4]
000010CD  8A36D8A3          mov dh,[0xa3d8]
000010D1  49                dec cx
000010D2  E311              jcxz 0x10e5
000010D4  8A36DCA3          mov dh,[0xa3dc]
000010D8  8B0EC0BC          mov cx,[0xbcc0]
000010DC  E307              jcxz 0x10e5
000010DE  80E580            and ch,0x80
000010E1  7502              jnz 0x10e5
000010E3  8BF1              mov si,cx
000010E5  8BD8              mov bx,ax
000010E7  E84711            call 0x2231
000010EA  93                xchg ax,bx
000010EB  268A0D            mov cl,[es:di]
000010EE  80E101            and cl,0x1
000010F1  740C              jz 0x10ff
000010F3  80E6F0            and dh,0xf0
000010F6  8A16DAA3          mov dl,[0xa3da]
000010FA  80E20F            and dl,0xf
000010FD  02F2              add dh,dl
000010FF  8E0672AF          mov es,word [0xaf72]
00001103  2BC6              sub ax,si
00001105  7C13              jl 0x111a
00001107  8B0E8BAF          mov cx,[0xaf8b]
0000110B  40                inc ax
0000110C  40                inc ax
0000110D  F6E1              mul cl
0000110F  D1E0              shl ax,0x0
00001111  40                inc ax
00001112  8BF8              mov di,ax
00001114  8AC6              mov al,dh
00001116  AA                stosb
00001117  47                inc di
00001118  E2FC              loop 0x1116
0000111A  C3                ret
0000111B  8BDC              mov bx,sp
0000111D  C47F02            les di,word [bx+0x2]
00001120  57                push di
00001121  E8FEFD            call 0xf22
00001124  5F                pop di
00001125  8A0D              mov cl,[di]
00001127  32ED              xor ch,ch
00001129  92                xchg ax,dx
0000112A  98                cbw
0000112B  AA                stosb
0000112C  03F9              add di,cx
0000112E  91                xchg ax,cx
0000112F  2AC8              sub cl,al
00001131  7204              jc 0x1137
00001133  B020              mov al,0x20
00001135  F3AA              rep stosb
00001137  C3                ret
00001138  C47E04            les di,word [bp+0x4]
0000113B  06                push es
0000113C  57                push di
0000113D  8D7EF2            lea di,[bp-0xe]
00001140  16                push ss
00001141  57                push di
00001142  8D7EFB            lea di,[bp-0x5]
00001145  16                push ss
00001146  57                push di
00001147  E83FFF            call 0x1089
0000114A  C3                ret
0000114B  55                push bp
0000114C  8BEC              mov bp,sp
0000114E  83EC0E            sub sp,0xe
00001151  C47E04            les di,word [bp+0x4]
00001154  83C71E            add di,0x1e
00001157  E8E1FF            call 0x113b
0000115A  C47E08            les di,word [bp+0x8]
0000115D  06                push es
0000115E  57                push di
0000115F  8D76FB            lea si,[bp-0x5]
00001162  B204              mov dl,0x4
00001164  E8B4FF            call 0x111b
00001167  C47E08            les di,word [bp+0x8]
0000116A  8D76F2            lea si,[bp-0xe]
0000116D  E8CEFD            call 0xf3e
00001170  EB3A              jmp 0x11ac
00001172  55                push bp
00001173  8BEC              mov bp,sp
00001175  83EC0E            sub sp,0xe
00001178  E8BDFF            call 0x1138
0000117B  C47E08            les di,word [bp+0x8]
0000117E  8D76F2            lea si,[bp-0xe]
00001181  E89EFD            call 0xf22
00001184  8D76FB            lea si,[bp-0x5]
00001187  AC                lodsb
00001188  98                cbw
00001189  8BC8              mov cx,ax
0000118B  E31F              jcxz 0x11ac
0000118D  F3A4              rep movsb
0000118F  C47E08            les di,word [bp+0x8]
00001192  260005            add [es:di],al
00001195  EB15              jmp 0x11ac
00001197  803EBFB900        cmp byte [0xb9bf],0x0
0000119C  74D4              jz 0x1172
0000119E  55                push bp
0000119F  8BEC              mov bp,sp
000011A1  1E                push ds
000011A2  C47E08            les di,word [bp+0x8]
000011A5  C57604            lds si,word [bp+0x4]
000011A8  E877FD            call 0xf22
000011AB  1F                pop ds
000011AC  8BE5              mov sp,bp
000011AE  5D                pop bp
000011AF  C20400            ret word 0x4
000011B2  55                push bp
000011B3  8BEC              mov bp,sp
000011B5  83EC0E            sub sp,0xe
000011B8  E87DFF            call 0x1138
000011BB  C47E08            les di,word [bp+0x8]
000011BE  C57604            lds si,word [bp+0x4]
000011C1  E85EFD            call 0xf22
000011C4  8D76FB            lea si,[bp-0x5]
000011C7  AC                lodsb
000011C8  0AC0              or al,al
000011CA  75E0              jnz 0x11ac
000011CC  C47E08            les di,word [bp+0x8]
000011CF  FE05              inc byte [di]
000011D1  8A1D              mov bl,[di]
000011D3  32FF              xor bh,bh
000011D5  C6012E            mov byte [bx+di],0x2e
000011D8  EBD2              jmp 0x11ac
000011DA  3BF7              cmp si,di
000011DC  7307              jnc 0x11e5
000011DE  FD                std
000011DF  03F9              add di,cx
000011E1  03F1              add si,cx
000011E3  4F                dec di
000011E4  4E                dec si
000011E5  F3A4              rep movsb
000011E7  FC                cld
000011E8  C3                ret
000011E9  26C4450B          les ax,word [es:di+0xb]
000011ED  8CC3              mov bx,es
000011EF  EB06              jmp 0x11f7
000011F1  C4066CBD          les ax,word [0xbd6c]
000011F5  8CC3              mov bx,es
000011F7  B108              mov cl,0x8
000011F9  EB04              jmp 0x11ff
000011FB  33C9              xor cx,cx
000011FD  33DB              xor bx,bx
000011FF  55                push bp
00001200  8BEC              mov bp,sp
00001202  51                push cx
00001203  8BFC              mov di,sp
00001205  83EC20            sub sp,0x20
00001208  16                push ss
00001209  07                pop es
0000120A  8BCF              mov cx,di
0000120C  BE0A00            mov si,0xa
0000120F  33D2              xor dx,dx
00001211  93                xchg ax,bx
00001212  F7F6              div si
00001214  93                xchg ax,bx
00001215  F7F6              div si
00001217  80C230            add dl,0x30
0000121A  4F                dec di
0000121B  8815              mov [di],dl
0000121D  8BD0              mov dx,ax
0000121F  0BD3              or dx,bx
00001221  75EC              jnz 0x120f
00001223  2BCF              sub cx,di
00001225  8BF7              mov si,di
00001227  C47E04            les di,word [bp+0x4]
0000122A  8A46FE            mov al,[bp-0x2]
0000122D  98                cbw
0000122E  91                xchg ax,cx
0000122F  E31D              jcxz 0x124e
00001231  91                xchg ax,cx
00001232  3D0900            cmp ax,0x9
00001235  7E03              jng 0x123a
00001237  B80900            mov ax,0x9
0000123A  AA                stosb
0000123B  8BD0              mov dx,ax
0000123D  2BD1              sub dx,cx
0000123F  7E08              jng 0x1249
00001241  87D1              xchg dx,cx
00001243  B020              mov al,0x20
00001245  F3AA              rep stosb
00001247  87CA              xchg cx,dx
00001249  F3A4              rep movsb
0000124B  E9ECFB            jmp 0xe3a
0000124E  8BC8              mov cx,ax
00001250  EBE0              jmp 0x1232
00001252  1E                push ds
00001253  AC                lodsb
00001254  98                cbw
00001255  32F6              xor dh,dh
00001257  32ED              xor ch,ch
00001259  E301              jcxz 0x125c
0000125B  49                dec cx
0000125C  03F1              add si,cx
0000125E  2AC1              sub al,cl
00001260  7302              jnc 0x1264
00001262  32C0              xor al,al
00001264  3AC2              cmp al,dl
00001266  7E59              jng 0x12c1
00001268  92                xchg ax,dx
00001269  EB56              jmp 0x12c1
0000126B  33D2              xor dx,dx
0000126D  8BDC              mov bx,sp
0000126F  0E                push cs
00001270  07                pop es
00001271  0AD2              or dl,dl
00001273  7424              jz 0x1299
00001275  BF8000            mov di,0x80
00001278  8A0D              mov cl,[di]
0000127A  32ED              xor ch,ch
0000127C  47                inc di
0000127D  E308              jcxz 0x1287
0000127F  803D20            cmp byte [di],0x20
00001282  7703              ja 0x1287
00001284  47                inc di
00001285  E2F8              loop 0x127f
00001287  8BF7              mov si,di
00001289  E308              jcxz 0x1293
0000128B  803D20            cmp byte [di],0x20
0000128E  7603              jna 0x1293
00001290  47                inc di
00001291  E2F8              loop 0x128b
00001293  8BC7              mov ax,di
00001295  2BC6              sub ax,si
00001297  EB22              jmp 0x12bb
00001299  33C0              xor ax,ax
0000129B  268E062C00        mov es,word [es:0x2c]
000012A0  33FF              xor di,di
000012A2  263A05            cmp al,[es:di]
000012A5  7407              jz 0x12ae
000012A7  B9FFFF            mov cx,0xffff
000012AA  F2AE              repne scasb
000012AC  EBF4              jmp 0x12a2
000012AE  83C703            add di,0x3
000012B1  8BF7              mov si,di
000012B3  B90001            mov cx,0x100
000012B6  F2AE              repne scasb
000012B8  91                xchg ax,cx
000012B9  F6D0              not al
000012BB  1E                push ds
000012BC  06                push es
000012BD  C47F02            les di,word [bx+0x2]
000012C0  1F                pop ds
000012C1  AA                stosb
000012C2  91                xchg ax,cx
000012C3  F3A4              rep movsb
000012C5  1F                pop ds
000012C6  C3                ret
000012C7  E81511            call 0x23df
000012CA  B106              mov cl,0x6
000012CC  EB02              jmp 0x12d0
000012CE  B102              mov cl,0x2
000012D0  32ED              xor ch,ch
000012D2  51                push cx
000012D3  C406BAB1          les ax,word [0xb1ba]
000012D7  06                push es
000012D8  8CC3              mov bx,es
000012DA  2BD8              sub bx,ax
000012DC  3BD9              cmp bx,cx
000012DE  7D10              jnl 0x12f0
000012E0  C416B6B1          les dx,word [0xb1b6]
000012E4  8CC1              mov cx,es
000012E6  03D0              add dx,ax
000012E8  83D100            adc cx,0x0
000012EB  E80C11            call 0x23fa
000012EE  33C0              xor ax,ax
000012F0  5B                pop bx
000012F1  59                pop cx
000012F2  93                xchg ax,bx
000012F3  3BC1              cmp ax,cx
000012F5  7E02              jng 0x12f9
000012F7  8BC1              mov ax,cx
000012F9  BFBEB1            mov di,0xb1be
000012FC  57                push di
000012FD  1E                push ds
000012FE  07                pop es
000012FF  1E                push ds
00001300  8E1E34BD          mov ds,word [0xbd34]
00001304  8BF3              mov si,bx
00001306  F3A4              rep movsb
00001308  1F                pop ds
00001309  8936BAB1          mov [0xb1ba],si
0000130D  5E                pop si
0000130E  C3                ret
0000130F  E8B8FF            call 0x12ca
00001312  E82CFD            call 0x1041
00001315  0BD2              or dx,dx
00001317  7505              jnz 0x131e
00001319  3D2A00            cmp ax,0x2a
0000131C  726F              jc 0x138d
0000131E  813C4D5A          cmp word [si],0x5a4d
00001322  742A              jz 0x134e
00001324  BA0600            mov dx,0x6
00001327  E89DFF            call 0x12c7
0000132A  813C4C48          cmp word [si],0x484c
0000132E  7418              jz 0x1348
00001330  BA1B02            mov dx,0x21b
00001333  E891FF            call 0x12c7
00001336  813C332E          cmp word [si],0x2e33
0000133A  7551              jnz 0x138d
0000133C  817C023333        cmp word [si+0x2],0x3333
00001341  754A              jnz 0x138d
00001343  B85202            mov ax,0x252
00001346  EB5A              jmp 0x13a2
00001348  B8EF04            mov ax,0x4ef
0000134B  50                push ax
0000134C  EB55              jmp 0x13a3
0000134E  8B4404            mov ax,[si+0x4]
00001351  48                dec ax
00001352  B109              mov cl,0x9
00001354  D3E0              shl ax,cl
00001356  034402            add ax,[si+0x2]
00001359  50                push ax
0000135A  BA2500            mov dx,0x25
0000135D  E867FF            call 0x12c7
00001360  813C4C48          cmp word [si],0x484c
00001364  742A              jz 0x1390
00001366  BA2D00            mov dx,0x2d
00001369  E85BFF            call 0x12c7
0000136C  813C332E          cmp word [si],0x2e33
00001370  7531              jnz 0x13a3
00001372  817C023333        cmp word [si+0x2],0x3333
00001377  752A              jnz 0x13a3
00001379  58                pop ax
0000137A  B88902            mov ax,0x289
0000137D  EB23              jmp 0x13a2
0000137F  3DEF31            cmp ax,0x31ef
00001382  752D              jnz 0x13b1
00001384  40                inc ax
00001385  0BD2              or dx,dx
00001387  7506              jnz 0x138f
00001389  3BC1              cmp ax,cx
0000138B  7202              jc 0x138f
0000138D  33C0              xor ax,ax
0000138F  C3                ret
00001390  5A                pop dx
00001391  52                push dx
00001392  E832FF            call 0x12c7
00001395  58                pop ax
00001396  813C636F          cmp word [si],0x6f63
0000139A  7505              jnz 0x13a1
0000139C  050800            add ax,0x8
0000139F  EB01              jmp 0x13a2
000013A1  40                inc ax
000013A2  50                push ax
000013A3  E89BFC            call 0x1041
000013A6  59                pop cx
000013A7  91                xchg ax,cx
000013A8  3D311C            cmp ax,0x1c31
000013AB  75D2              jnz 0x137f
000013AD  40                inc ax
000013AE  40                inc ax
000013AF  EBD3              jmp 0x1384
000013B1  8BD0              mov dx,ax
000013B3  52                push dx
000013B4  E810FF            call 0x12c7
000013B7  58                pop ax
000013B8  817C0260EA        cmp word [si+0x2],0xea60
000013BD  75D0              jnz 0x138f
000013BF  40                inc ax
000013C0  40                inc ax
000013C1  C3                ret
000013C2  3C61              cmp al,0x61
000013C4  7C06              jl 0x13cc
000013C6  3C7A              cmp al,0x7a
000013C8  7F02              jg 0x13cc
000013CA  2C20              sub al,0x20
000013CC  C3                ret
000013CD  B164              mov cl,0x64
000013CF  E8FEFE            call 0x12d0
000013D2  817C072A2A        cmp word [si+0x7],0x2a2a
000013D7  7511              jnz 0x13ea
000013D9  817C094143        cmp word [si+0x9],0x4341
000013DE  750A              jnz 0x13ea
000013E0  817C0C2A2A        cmp word [si+0xc],0x2a2a
000013E5  7503              jnz 0x13ea
000013E7  B013              mov al,0x13
000013E9  C3                ret
000013EA  813C424F          cmp word [si],0x4f42
000013EE  7509              jnz 0x13f9
000013F0  837C0241          cmp word [si+0x2],0x41
000013F4  7503              jnz 0x13f9
000013F6  B015              mov al,0x15
000013F8  C3                ret
000013F9  AD                lodsw
000013FA  3D9133            cmp ax,0x3391
000013FD  750F              jnz 0x140e
000013FF  813C4846          cmp word [si],0x4648
00001403  7509              jnz 0x140e
00001405  837C0200          cmp word [si+0x2],0x0
00001409  7503              jnz 0x140e
0000140B  B016              mov al,0x16
0000140D  C3                ret
0000140E  3D4F5A            cmp ax,0x5a4f
00001411  7508              jnz 0x141b
00001413  803CDD            cmp byte [si],0xdd
00001416  7503              jnz 0x141b
00001418  B014              mov al,0x14
0000141A  C3                ret
0000141B  3D3704            cmp ax,0x437
0000141E  7508              jnz 0x1428
00001420  833C10            cmp word [si],0x10
00001423  7503              jnz 0x1428
00001425  B017              mov al,0x17
00001427  C3                ret
00001428  3D4841            cmp ax,0x4148
0000142B  7503              jnz 0x1430
0000142D  B011              mov al,0x11
0000142F  C3                ret
00001430  3D4C47            cmp ax,0x474c
00001433  7503              jnz 0x1438
00001435  B012              mov al,0x12
00001437  C3                ret
00001438  3D5343            cmp ax,0x4353
0000143B  7503              jnz 0x1440
0000143D  B008              mov al,0x8
0000143F  C3                ret
00001440  3D504B            cmp ax,0x4b50
00001443  7503              jnz 0x1448
00001445  B003              mov al,0x3
00001447  C3                ret
00001448  3D4850            cmp ax,0x5048
0000144B  750A              jnz 0x1457
0000144D  813C414B          cmp word [si],0x4b41
00001451  757B              jnz 0x14ce
00001453  B010              mov al,0x10
00001455  C3                ret
00001456  90                nop
00001457  3D4C4D            cmp ax,0x4d4c
0000145A  7421              jz 0x147d
0000145C  3D5261            cmp ax,0x6152
0000145F  750E              jnz 0x146f
00001461  AD                lodsw
00001462  3D7221            cmp ax,0x2172
00001465  756F              jnz 0x14d6
00001467  AC                lodsb
00001468  3C1A              cmp al,0x1a
0000146A  756A              jnz 0x14d6
0000146C  B019              mov al,0x19
0000146E  C3                ret
0000146F  3DFF42            cmp ax,0x42ff
00001472  7511              jnz 0x1485
00001474  AD                lodsw
00001475  3D5347            cmp ax,0x4753
00001478  755C              jnz 0x14d6
0000147A  B00D              mov al,0xd
0000147C  C3                ret
0000147D  AC                lodsb
0000147E  3C1A              cmp al,0x1a
00001480  7554              jnz 0x14d6
00001482  B00F              mov al,0xf
00001484  C3                ret
00001485  3D5245            cmp ax,0x4552
00001488  7509              jnz 0x1493
0000148A  AD                lodsw
0000148B  3D7E5E            cmp ax,0x5e7e
0000148E  7546              jnz 0x14d6
00001490  B018              mov al,0x18
00001492  C3                ret
00001493  3D484C            cmp ax,0x4c48
00001496  750E              jnz 0x14a6
00001498  AD                lodsw
00001499  3D5351            cmp ax,0x5153
0000149C  7538              jnz 0x14d6
0000149E  AC                lodsb
0000149F  3C5A              cmp al,0x5a
000014A1  7533              jnz 0x14d6
000014A3  B00E              mov al,0xe
000014A5  C3                ret
000014A6  3D5A4F            cmp ax,0x4f5a
000014A9  7516              jnz 0x14c1
000014AB  BA1400            mov dx,0x14
000014AE  E82E0F            call 0x23df
000014B1  0BC2              or ax,dx
000014B3  7421              jz 0x14d6
000014B5  E816FE            call 0x12ce
000014B8  813CDCA7          cmp word [si],0xa7dc
000014BC  7518              jnz 0x14d6
000014BE  B00A              mov al,0xa
000014C0  C3                ret
000014C1  3D1A46            cmp ax,0x461a
000014C4  7508              jnz 0x14ce
000014C6  AC                lodsb
000014C7  3C43              cmp al,0x43
000014C9  750B              jnz 0x14d6
000014CB  B00B              mov al,0xb
000014CD  C3                ret
000014CE  3D4F72            cmp ax,0x724f
000014D1  7506              jnz 0x14d9
000014D3  B007              mov al,0x7
000014D5  C3                ret
000014D6  B000              mov al,0x0
000014D8  C3                ret
000014D9  3D60EA            cmp ax,0xea60
000014DC  7503              jnz 0x14e1
000014DE  B006              mov al,0x6
000014E0  C3                ret
000014E1  91                xchg ax,cx
000014E2  AD                lodsw
000014E3  8ADD              mov bl,ch
000014E5  8AF8              mov bh,al
000014E7  93                xchg ax,bx
000014E8  81FB2D6C          cmp bx,0x6c2d
000014EC  743B              jz 0x1529
000014EE  80F91A            cmp cl,0x1a
000014F1  754F              jnz 0x1542
000014F3  3D4850            cmp ax,0x5048
000014F6  7405              jz 0x14fd
000014F8  3D5354            cmp ax,0x5453
000014FB  7503              jnz 0x1500
000014FD  B009              mov al,0x9
000014FF  C3                ret
00001500  3C10              cmp al,0x10
00001502  7FD2              jg 0x14d6
00001504  80FC21            cmp ah,0x21
00001507  7CCD              jl 0x14d6
00001509  E835FB            call 0x1041
0000150C  2D0200            sub ax,0x2
0000150F  83DA00            sbb dx,0x0
00001512  92                xchg ax,dx
00001513  91                xchg ax,cx
00001514  E8CA0E            call 0x23e1
00001517  E8B4FD            call 0x12ce
0000151A  AD                lodsw
0000151B  3CFE              cmp al,0xfe
0000151D  7503              jnz 0x1522
0000151F  B005              mov al,0x5
00001521  C3                ret
00001522  0AE4              or ah,ah
00001524  75B0              jnz 0x14d6
00001526  B004              mov al,0x4
00001528  C3                ret
00001529  BA1400            mov dx,0x14
0000152C  031678BD          add dx,[0xbd78]
00001530  E8AC0E            call 0x23df
00001533  E898FD            call 0x12ce
00001536  0BC0              or ax,ax
00001538  7414              jz 0x154e
0000153A  AD                lodsw
0000153B  3C02              cmp al,0x2
0000153D  7F10              jg 0x154f
0000153F  B002              mov al,0x2
00001541  C3                ret
00001542  3D00AE            cmp ax,0xae00
00001545  758F              jnz 0x14d6
00001547  80FF0A            cmp bh,0xa
0000154A  7F8A              jg 0x14d6
0000154C  B00C              mov al,0xc
0000154E  C3                ret
0000154F  80FC00            cmp ah,0x0
00001552  7582              jnz 0x14d6
00001554  B001              mov al,0x1
00001556  C3                ret
00001557  B90400            mov cx,0x4
0000155A  A0A0BD            mov al,[0xbda0]
0000155D  8A26BFB9          mov ah,[0xb9bf]
00001561  BEFFB0            mov si,0xb0ff
00001564  3C16              cmp al,0x16
00001566  7505              jnz 0x156d
00001568  BE11B1            mov si,0xb111
0000156B  EB5B              jmp 0x15c8
0000156D  3C03              cmp al,0x3
0000156F  7465              jz 0x15d6
00001571  723E              jc 0x15b1
00001573  3C05              cmp al,0x5
00001575  7274              jc 0x15eb
00001577  742D              jz 0x15a6
00001579  3C07              cmp al,0x7
0000157B  7242              jc 0x15bf
0000157D  7450              jz 0x15cf
0000157F  3C09              cmp al,0x9
00001581  7263              jc 0x15e6
00001583  7445              jz 0x15ca
00001585  3C0B              cmp al,0xb
00001587  7211              jc 0x159a
00001589  747F              jz 0x160a
0000158B  3C0E              cmp al,0xe
0000158D  7471              jz 0x1600
0000158F  7767              ja 0x15f8
00001591  0AE4              or ah,ah
00001593  7525              jnz 0x15ba
00001595  BE2CB1            mov si,0xb12c
00001598  EB33              jmp 0x15cd
0000159A  BE20B1            mov si,0xb120
0000159D  0AE4              or ah,ah
0000159F  752C              jnz 0x15cd
000015A1  BE51B1            mov si,0xb151
000015A4  EB7F              jmp 0x1625
000015A6  BE11B1            mov si,0xb111
000015A9  0AE4              or ah,ah
000015AB  741B              jz 0x15c8
000015AD  B109              mov cl,0x9
000015AF  EB57              jmp 0x1608
000015B1  0AE4              or ah,ah
000015B3  7513              jnz 0x15c8
000015B5  BE11B1            mov si,0xb111
000015B8  EB61              jmp 0x161b
000015BA  BE3DB1            mov si,0xb13d
000015BD  EB5C              jmp 0x161b
000015BF  B107              mov cl,0x7
000015C1  0AE4              or ah,ah
000015C3  757B              jnz 0x1640
000015C5  03F1              add si,cx
000015C7  4E                dec si
000015C8  EB75              jmp 0x163f
000015CA  BE1CB1            mov si,0xb11c
000015CD  EB7E              jmp 0x164d
000015CF  0AE4              or ah,ah
000015D1  7513              jnz 0x15e6
000015D3  E9B200            jmp 0x1688
000015D6  BE0BB1            mov si,0xb10b
000015D9  0AE4              or ah,ah
000015DB  7563              jnz 0x1640
000015DD  03F1              add si,cx
000015DF  4E                dec si
000015E0  EB5E              jmp 0x1640
000015E2  0AE4              or ah,ah
000015E4  750D              jnz 0x15f3
000015E6  BE0EB1            mov si,0xb10e
000015E9  EB55              jmp 0x1640
000015EB  BE77B0            mov si,0xb077
000015EE  AC                lodsb
000015EF  8AC8              mov cl,al
000015F1  EB4F              jmp 0x1642
000015F3  BE5BB1            mov si,0xb15b
000015F6  EB48              jmp 0x1640
000015F8  3C15              cmp al,0x15
000015FA  74E6              jz 0x15e2
000015FC  3C11              cmp al,0x11
000015FE  720F              jc 0x160f
00001600  49                dec cx
00001601  0AE4              or ah,ah
00001603  753B              jnz 0x1640
00001605  BE11B1            mov si,0xb111
00001608  EB36              jmp 0x1640
0000160A  BE33B1            mov si,0xb133
0000160D  EB31              jmp 0x1640
0000160F  3C10              cmp al,0x10
00001611  750A              jnz 0x161d
00001613  BE55B1            mov si,0xb155
00001616  0AE4              or ah,ah
00001618  7525              jnz 0x163f
0000161A  41                inc cx
0000161B  EB2F              jmp 0x164c
0000161D  BE46B1            mov si,0xb146
00001620  49                dec cx
00001621  0AE4              or ah,ah
00001623  7427              jz 0x164c
00001625  EB19              jmp 0x1640
00001627  B90400            mov cx,0x4
0000162A  BE11B1            mov si,0xb111
0000162D  A0A0BD            mov al,[0xbda0]
00001630  3C02              cmp al,0x2
00001632  7705              ja 0x1639
00001634  BE24B1            mov si,0xb124
00001637  EB13              jmp 0x164c
00001639  3C05              cmp al,0x5
0000163B  72AE              jc 0x15eb
0000163D  7516              jnz 0x1655
0000163F  49                dec cx
00001640  8AC1              mov al,cl
00001642  8BDC              mov bx,sp
00001644  36C47F02          les di,word [ss:bx+0x2]
00001648  AA                stosb
00001649  F3A4              rep movsb
0000164B  C3                ret
0000164C  41                inc cx
0000164D  41                inc cx
0000164E  EBF0              jmp 0x1640
00001650  BE29B1            mov si,0xb129
00001653  EBF8              jmp 0x164d
00001655  3C06              cmp al,0x6
00001657  7507              jnz 0x1660
00001659  B107              mov cl,0x7
0000165B  BE05B1            mov si,0xb105
0000165E  EBE0              jmp 0x1640
00001660  3C09              cmp al,0x9
00001662  7282              jc 0x15e6
00001664  740A              jz 0x1670
00001666  3C0B              cmp al,0xb
00001668  72E6              jc 0x1650
0000166A  749E              jz 0x160a
0000166C  3C0E              cmp al,0xe
0000166E  730B              jnc 0x167b
00001670  BE2CB1            mov si,0xb12c
00001673  41                inc cx
00001674  EBCA              jmp 0x1640
00001676  BE0EB1            mov si,0xb10e
00001679  EBC5              jmp 0x1640
0000167B  3C15              cmp al,0x15
0000167D  74F7              jz 0x1676
0000167F  3C10              cmp al,0x10
00001681  75BC              jnz 0x163f
00001683  BE55B1            mov si,0xb155
00001686  EB92              jmp 0x161a
00001688  BE19B1            mov si,0xb119
0000168B  EBB3              jmp 0x1640
0000168D  55                push bp
0000168E  8BEC              mov bp,sp
00001690  83EC04            sub sp,0x4
00001693  8B1670AF          mov dx,[0xaf70]
00001697  52                push dx
00001698  E844F1            call 0x7df
0000169B  A0D8A3            mov al,[0xa3d8]
0000169E  A2B3BD            mov [0xbdb3],al
000016A1  E86CF0            call 0x710
000016A4  A0E3A3            mov al,[0xa3e3]
000016A7  A2B3BD            mov [0xbdb3],al
000016AA  8A46FA            mov al,[bp-0x6]
000016AD  40                inc ax
000016AE  8D7EFD            lea di,[bp-0x3]
000016B1  98                cbw
000016B2  16                push ss
000016B3  57                push di
000016B4  E844FB            call 0x11fb
000016B7  5F                pop di
000016B8  07                pop es
000016B9  8A05              mov al,[di]
000016BB  8B56FA            mov dx,[bp-0x6]
000016BE  B103              mov cl,0x3
000016C0  D2E2              shl dl,cl
000016C2  0AD2              or dl,dl
000016C4  7408              jz 0x16ce
000016C6  4A                dec dx
000016C7  C60520            mov byte [di],0x20
000016CA  4F                dec di
000016CB  40                inc ax
000016CC  8805              mov [di],al
000016CE  E8DCF1            call 0x8ad
000016D1  FE46FA            inc byte [bp-0x6]
000016D4  807EFA0A          cmp byte [bp-0x6],0xa
000016D8  7CD0              jl 0x16aa
000016DA  E95DF7            jmp 0xe3a
000016DD  16                push ss
000016DE  07                pop es
000016DF  B80300            mov ax,0x3
000016E2  AA                stosb
000016E3  91                xchg ax,cx
000016E4  AC                lodsb
000016E5  3C3A              cmp al,0x3a
000016E7  7406              jz 0x16ef
000016E9  E8D6FC            call 0x13c2
000016EC  AA                stosb
000016ED  E2F5              loop 0x16e4
000016EF  B020              mov al,0x20
000016F1  F3AA              rep stosb
000016F3  C3                ret
000016F4  0ADB              or bl,bl
000016F6  7418              jz 0x1710
000016F8  E8C3F9            call 0x10be
000016FB  BF2DAA            mov di,0xaa2d
000016FE  E8B3F5            call 0xcb4
00001701  BF36AA            mov di,0xaa36
00001704  E87EF1            call 0x885
00001707  BFEEBB            mov di,0xbbee
0000170A  E8AA0D            call 0x24b7
0000170D  E8D4EF            call 0x6e4
00001710  1E                push ds
00001711  BFEEBB            mov di,0xbbee
00001714  57                push di
00001715  A1C8BC            mov ax,[0xbcc8]
00001718  E8440B            call 0x225f
0000171B  A3C8BC            mov [0xbcc8],ax
0000171E  EB07              jmp 0x1727
00001720  F616BFB9          not byte [0xb9bf]
00001724  E87908            call 0x1fa0
00001727  C606C1BCFE        mov byte [0xbcc1],0xfe
0000172C  A1C8BC            mov ax,[0xbcc8]
0000172F  3B06C4BC          cmp ax,[0xbcc4]
00001733  7C06              jl 0x173b
00001735  3B06C6BC          cmp ax,[0xbcc6]
00001739  7E3A              jng 0x1775
0000173B  93                xchg ax,bx
0000173C  A1C7B9            mov ax,[0xb9c7]
0000173F  8BC8              mov cx,ax
00001741  8B1659BD          mov dx,[0xbd59]
00001745  2BD0              sub dx,ax
00001747  3BDA              cmp bx,dx
00001749  7C11              jl 0x175c
0000174B  A159BD            mov ax,[0xbd59]
0000174E  A3C6BC            mov [0xbcc6],ax
00001751  2BC1              sub ax,cx
00001753  7F03              jg 0x1758
00001755  B80100            mov ax,0x1
00001758  A3C4BC            mov [0xbcc4],ax
0000175B  C3                ret
0000175C  3BCB              cmp cx,bx
0000175E  7E16              jng 0x1776
00001760  C706C4BC0100      mov word [0xbcc4],0x1
00001766  8B1659BD          mov dx,[0xbd59]
0000176A  41                inc cx
0000176B  3BCA              cmp cx,dx
0000176D  7E02              jng 0x1771
0000176F  8BCA              mov cx,dx
00001771  890EC6BC          mov [0xbcc6],cx
00001775  C3                ret
00001776  891EC4BC          mov [0xbcc4],bx
0000177A  03CB              add cx,bx
0000177C  EBF3              jmp 0x1771
0000177E  E8C105            call 0x1d42
00001781  A1C4BC            mov ax,[0xbcc4]
00001784  3B06C6BC          cmp ax,[0xbcc6]
00001788  7F67              jg 0x17f1
0000178A  50                push ax
0000178B  50                push ax
0000178C  E89D06            call 0x1e2c
0000178F  58                pop ax
00001790  40                inc ax
00001791  EBF1              jmp 0x1784
00001793  8B1EB4B1          mov bx,[0xb1b4]
00001797  E892F8            call 0x102c
0000179A  0E                push cs
0000179B  07                pop es
0000179C  A12C00            mov ax,[0x2c]
0000179F  A311BF            mov [0xbf11],ax
000017A2  BFA1BE            mov di,0xbea1
000017A5  E87AF7            call 0xf22
000017A8  91                xchg ax,cx
000017A9  AA                stosb
000017AA  E8E3E8            call 0x90
000017AD  1E                push ds
000017AE  07                pop es
000017AF  BF21BE            mov di,0xbe21
000017B2  58                pop ax
000017B3  5E                pop si
000017B4  892611BE          mov [0xbe11],sp
000017B8  8C1613BE          mov word [0xbe13],ss
000017BC  1F                pop ds
000017BD  50                push ax
000017BE  E861F7            call 0xf22
000017C1  B00D              mov al,0xd
000017C3  AA                stosb
000017C4  C70613BF21BE      mov word [0xbf13],0xbe21
000017CA  8C1E15BF          mov word [0xbf15],ds
000017CE  BAA2BE            mov dx,0xbea2
000017D1  BB11BF            mov bx,0xbf11
000017D4  B8004B            mov ax,0x4b00
000017D7  CD21              int byte 0x21
000017D9  8CC8              mov ax,cs
000017DB  8ED8              mov ds,ax
000017DD  FA                cli
000017DE  8E1613BE          mov ss,word [0xbe13]
000017E2  8B2611BE          mov sp,[0xbe11]
000017E6  FB                sti
000017E7  FC                cld
000017E8  E875E8            call 0x60
000017EB  E8BBF7            call 0xfa9
000017EE  A3B4B1            mov [0xb1b4],ax
000017F1  C3                ret
000017F2  55                push bp
000017F3  8BEC              mov bp,sp
000017F5  81EC8000          sub sp,0x80
000017F9  1E                push ds
000017FA  C57608            lds si,word [bp+0x8]
000017FD  C47E04            les di,word [bp+0x4]
00001800  06                push es
00001801  57                push di
00001802  E81DF7            call 0xf22
00001805  E809F8            call 0x1011
00001808  7203              jc 0x180d
0000180A  E99900            jmp 0x18a6
0000180D  8D7E80            lea di,[bp-0x80]
00001810  1F                pop ds
00001811  1E                push ds
00001812  16                push ss
00001813  57                push di
00001814  E854FA            call 0x126b
00001817  5E                pop si
00001818  1F                pop ds
00001819  E854F8            call 0x1070
0000181C  B84300            mov ax,0x43
0000181F  C47E04            les di,word [bp+0x4]
00001822  E83AF8            call 0x105f
00001825  C47E04            les di,word [bp+0x4]
00001828  06                push es
00001829  57                push di
0000182A  C57608            lds si,word [bp+0x8]
0000182D  E80EF7            call 0xf3e
00001830  E8DEF7            call 0x1011
00001833  7371              jnc 0x18a6
00001835  8D7E80            lea di,[bp-0x80]
00001838  1F                pop ds
00001839  16                push ss
0000183A  57                push di
0000183B  BEE4AF            mov si,0xafe4
0000183E  E81DF7            call 0xf5e
00001841  5F                pop di
00001842  07                pop es
00001843  1E                push ds
00001844  268A05            mov al,[es:di]
00001847  32E4              xor ah,ah
00001849  8BD8              mov bx,ax
0000184B  B23B              mov dl,0x3b
0000184D  263811            cmp [es:bx+di],dl
00001850  7408              jz 0x185a
00001852  26FE05            inc byte [es:di]
00001855  40                inc ax
00001856  26885101          mov [es:bx+di+0x1],dl
0000185A  8BD0              mov dx,ax
0000185C  47                inc di
0000185D  8BCA              mov cx,dx
0000185F  E33B              jcxz 0x189c
00001861  B03B              mov al,0x3b
00001863  F2AE              repne scasb
00001865  2BD1              sub dx,cx
00001867  87CA              xchg cx,dx
00001869  06                push es
0000186A  57                push di
0000186B  8BF7              mov si,di
0000186D  2BF1              sub si,cx
0000186F  C47E04            les di,word [bp+0x4]
00001872  57                push di
00001873  49                dec cx
00001874  8BC1              mov ax,cx
00001876  AA                stosb
00001877  93                xchg ax,bx
00001878  F3A4              rep movsb
0000187A  5F                pop di
0000187B  52                push dx
0000187C  06                push es
0000187D  57                push di
0000187E  B05C              mov al,0x5c
00001880  263801            cmp [es:bx+di],al
00001883  7407              jz 0x188c
00001885  43                inc bx
00001886  26FE05            inc byte [es:di]
00001889  268801            mov [es:bx+di],al
0000188C  C57608            lds si,word [bp+0x8]
0000188F  E8ACF6            call 0xf3e
00001892  E87CF7            call 0x1011
00001895  5A                pop dx
00001896  5F                pop di
00001897  07                pop es
00001898  730C              jnc 0x18a6
0000189A  EBC1              jmp 0x185d
0000189C  C47E04            les di,word [bp+0x4]
0000189F  C57608            lds si,word [bp+0x8]
000018A2  E87DF6            call 0xf22
000018A5  F9                stc
000018A6  1F                pop ds
000018A7  8BE5              mov sp,bp
000018A9  5D                pop bp
000018AA  C20800            ret word 0x8
000018AD  58                pop ax
000018AE  16                push ss
000018AF  57                push di
000018B0  50                push ax
000018B1  16                push ss
000018B2  07                pop es
000018B3  32C0              xor al,al
000018B5  AA                stosb
000018B6  4F                dec di
000018B7  38068EBC          cmp [0xbc8e],al
000018BB  7441              jz 0x18fe
000018BD  A0A0BD            mov al,[0xbda0]
000018C0  3C15              cmp al,0x15
000018C2  743A              jz 0x18fe
000018C4  3C03              cmp al,0x3
000018C6  7C36              jl 0x18fe
000018C8  7415              jz 0x18df
000018CA  3C17              cmp al,0x17
000018CC  740C              jz 0x18da
000018CE  3C13              cmp al,0x13
000018D0  7312              jnc 0x18e4
000018D2  3C07              cmp al,0x7
000018D4  7F28              jg 0x18fe
000018D6  3C05              cmp al,0x5
000018D8  740F              jz 0x18e9
000018DA  BE34B0            mov si,0xb034
000018DD  EB0D              jmp 0x18ec
000018DF  BE00B0            mov si,0xb000
000018E2  EB08              jmp 0x18ec
000018E4  BE4CB0            mov si,0xb04c
000018E7  EB03              jmp 0x18ec
000018E9  BE5FB1            mov si,0xb15f
000018EC  57                push di
000018ED  E832F6            call 0xf22
000018F0  5F                pop di
000018F1  57                push di
000018F2  BE8EBC            mov si,0xbc8e
000018F5  E846F6            call 0xf3e
000018F8  B020              mov al,0x20
000018FA  AA                stosb
000018FB  5F                pop di
000018FC  FE05              inc byte [di]
000018FE  C3                ret
000018FF  A1BEBC            mov ax,[0xbcbe]
00001902  0BC0              or ax,ax
00001904  7519              jnz 0x191f
00001906  A0A0BD            mov al,[0xbda0]
00001909  3C03              cmp al,0x3
0000190B  7404              jz 0x1911
0000190D  3C06              cmp al,0x6
0000190F  7545              jnz 0x1956
00001911  A1C8BC            mov ax,[0xbcc8]
00001914  E81A09            call 0x2231
00001917  26F60518          test byte [es:di],0x18
0000191B  7439              jz 0x1956
0000191D  EB16              jmp 0x1935
0000191F  8B0E59BD          mov cx,[0xbd59]
00001923  8BC1              mov ax,cx
00001925  E80909            call 0x2231
00001928  268A05            mov al,[es:di]
0000192B  A801              test al,0x1
0000192D  7404              jz 0x1933
0000192F  A818              test al,0x18
00001931  7423              jz 0x1956
00001933  E2EE              loop 0x1923
00001935  F9                stc
00001936  C3                ret
00001937  3C02              cmp al,0x2
00001939  750E              jnz 0x1949
0000193B  50                push ax
0000193C  B010              mov al,0x10
0000193E  E670              out byte 0x70,al
00001940  EB00              jmp 0x1942
00001942  E471              in al,byte 0x71
00001944  240F              and al,0xf
00001946  58                pop ax
00001947  740D              jz 0x1956
00001949  E801F6            call 0xf4d
0000194C  3DFFFF            cmp ax,0xffff
0000194F  75E4              jnz 0x1935
00001951  83FAFF            cmp dx,0xffffffffffffffff
00001954  75DF              jnz 0x1935
00001956  F8                clc
00001957  C3                ret
00001958  BE1DBB            mov si,0xbb1d
0000195B  AC                lodsb
0000195C  8BFE              mov di,si
0000195E  98                cbw
0000195F  91                xchg ax,cx
00001960  C3                ret
00001961  55                push bp
00001962  8BEC              mov bp,sp
00001964  83EC50            sub sp,0x50
00001967  8A3ED8A3          mov bh,[0xa3d8]
0000196B  883EB3BD          mov [0xbdb3],bh
0000196F  B91D00            mov cx,0x1d
00001972  BA2700            mov dx,0x27
00001975  32C0              xor al,al
00001977  E8A5ED            call 0x71f
0000197A  A1C8BC            mov ax,[0xbcc8]
0000197D  8D7EB0            lea di,[bp-0x50]
00001980  16                push ss
00001981  57                push di
00001982  E876F8            call 0x11fb
00001985  BA1C00            mov dx,0x1c
00001988  5F                pop di
00001989  07                pop es
0000198A  E820EF            call 0x8ad
0000198D  BF44A7            mov di,0xa744
00001990  E8F2EE            call 0x885
00001993  8D7EB0            lea di,[bp-0x50]
00001996  A159BD            mov ax,[0xbd59]
00001999  16                push ss
0000199A  57                push di
0000199B  E85DF8            call 0x11fb
0000199E  E801EF            call 0x8a2
000019A1  E996F4            jmp 0xe3a
000019A4  55                push bp
000019A5  8BEC              mov bp,sp
000019A7  06                push es
000019A8  57                push di
000019A9  E81300            call 0x19bf
000019AC  52                push dx
000019AD  53                push bx
000019AE  50                push ax
000019AF  C44604            les ax,word [bp+0x4]
000019B2  06                push es
000019B3  50                push ax
000019B4  E80800            call 0x19bf
000019B7  59                pop cx
000019B8  5E                pop si
000019B9  5F                pop di
000019BA  E8D701            call 0x1b94
000019BD  EB3E              jmp 0x19fd
000019BF  55                push bp
000019C0  8BEC              mov bp,sp
000019C2  C47E04            les di,word [bp+0x4]
000019C5  268B4507          mov ax,[es:di+0x7]
000019C9  268B5509          mov dx,[es:di+0x9]
000019CD  263B550D          cmp dx,[es:di+0xd]
000019D1  7C08              jl 0x19db
000019D3  7F22              jg 0x19f7
000019D5  263B450B          cmp ax,[es:di+0xb]
000019D9  731C              jnc 0x19f7
000019DB  E87600            call 0x1a54
000019DE  52                push dx
000019DF  53                push bx
000019E0  50                push ax
000019E1  C47E04            les di,word [bp+0x4]
000019E4  268B450B          mov ax,[es:di+0xb]
000019E8  268B550D          mov dx,[es:di+0xd]
000019EC  E86500            call 0x1a54
000019EF  59                pop cx
000019F0  5E                pop si
000019F1  5F                pop di
000019F2  E80701            call 0x1afc
000019F5  EB06              jmp 0x19fd
000019F7  33C0              xor ax,ax
000019F9  8BD8              mov bx,ax
000019FB  8BD0              mov dx,ax
000019FD  E9ACF7            jmp 0x11ac
00001A00  55                push bp
00001A01  8BEC              mov bp,sp
00001A03  C47E04            les di,word [bp+0x4]
00001A06  268B450B          mov ax,[es:di+0xb]
00001A0A  268B550D          mov dx,[es:di+0xd]
00001A0E  263B5509          cmp dx,[es:di+0x9]
00001A12  7CE3              jl 0x19f7
00001A14  7F06              jg 0x1a1c
00001A16  263B4507          cmp ax,[es:di+0x7]
00001A1A  76DB              jna 0x19f7
00001A1C  E83500            call 0x1a54
00001A1F  52                push dx
00001A20  53                push bx
00001A21  50                push ax
00001A22  C47E04            les di,word [bp+0x4]
00001A25  268B450B          mov ax,[es:di+0xb]
00001A29  268B550D          mov dx,[es:di+0xd]
00001A2D  262B4507          sub ax,[es:di+0x7]
00001A31  261B5509          sbb dx,[es:di+0x9]
00001A35  B96400            mov cx,0x64
00001A38  8BFA              mov di,dx
00001A3A  F7E1              mul cx
00001A3C  97                xchg ax,di
00001A3D  8BDA              mov bx,dx
00001A3F  F7E1              mul cx
00001A41  97                xchg ax,di
00001A42  87D3              xchg dx,bx
00001A44  03D7              add dx,di
00001A46  E80B00            call 0x1a54
00001A49  59                pop cx
00001A4A  5E                pop si
00001A4B  5F                pop di
00001A4C  E8AD00            call 0x1afc
00001A4F  E83D00            call 0x1a8f
00001A52  EBA9              jmp 0x19fd
00001A54  8BD8              mov bx,ax
00001A56  0BDA              or bx,dx
00001A58  7434              jz 0x1a8e
00001A5A  8AEE              mov ch,dh
00001A5C  0BD2              or dx,dx
00001A5E  7907              jns 0x1a67
00001A60  F7DA              neg dx
00001A62  F7D8              neg ax
00001A64  83DA00            sbb dx,0x0
00001A67  8BD8              mov bx,ax
00001A69  B8A000            mov ax,0xa0
00001A6C  0BD2              or dx,dx
00001A6E  750C              jnz 0x1a7c
00001A70  87D3              xchg dx,bx
00001A72  B090              mov al,0x90
00001A74  0AF6              or dh,dh
00001A76  7504              jnz 0x1a7c
00001A78  86F2              xchg dh,dl
00001A7A  B088              mov al,0x88
00001A7C  0BD2              or dx,dx
00001A7E  7807              js 0x1a87
00001A80  48                dec ax
00001A81  03DB              add bx,bx
00001A83  13D2              adc dx,dx
00001A85  79F9              jns 0x1a80
00001A87  0AED              or ch,ch
00001A89  7803              js 0x1a8e
00001A8B  80E67F            and dh,0x7f
00001A8E  C3                ret
00001A8F  93                xchg ax,bx
00001A90  B1A0              mov cl,0xa0
00001A92  2ACB              sub cl,bl
00001A94  725B              jc 0x1af1
00001A96  8ADE              mov bl,dh
00001A98  80CE80            or dh,0x80
00001A9B  80F920            cmp cl,0x20
00001A9E  7352              jnc 0x1af2
00001AA0  80F910            cmp cl,0x10
00001AA3  7209              jc 0x1aae
00001AA5  8AFC              mov bh,ah
00001AA7  8BC2              mov ax,dx
00001AA9  33D2              xor dx,dx
00001AAB  80E910            sub cl,0x10
00001AAE  80F908            cmp cl,0x8
00001AB1  720D              jc 0x1ac0
00001AB3  8AF8              mov bh,al
00001AB5  8AC4              mov al,ah
00001AB7  8AE2              mov ah,dl
00001AB9  8AD6              mov dl,dh
00001ABB  32F6              xor dh,dh
00001ABD  80E908            sub cl,0x8
00001AC0  0AC9              or cl,cl
00001AC2  740A              jz 0x1ace
00001AC4  D1EA              shr dx,0x0
00001AC6  D1D8              rcr ax,0x0
00001AC8  D0DF              rcr bh,0x0
00001ACA  FEC9              dec cl
00001ACC  75F6              jnz 0x1ac4
00001ACE  0AED              or ch,ch
00001AD0  740A              jz 0x1adc
00001AD2  02FF              add bh,bh
00001AD4  150000            adc ax,0x0
00001AD7  83D200            adc dx,0x0
00001ADA  7215              jc 0x1af1
00001ADC  8BC8              mov cx,ax
00001ADE  0BCA              or cx,dx
00001AE0  740F              jz 0x1af1
00001AE2  0ADB              or bl,bl
00001AE4  7907              jns 0x1aed
00001AE6  F7DA              neg dx
00001AE8  F7D8              neg ax
00001AEA  83DA00            sbb dx,0x0
00001AED  32DE              xor bl,dh
00001AEF  02DB              add bl,bl
00001AF1  C3                ret
00001AF2  8AFE              mov bh,dh
00001AF4  B80000            mov ax,0x0
00001AF7  8BD0              mov dx,ax
00001AF9  74D3              jz 0x1ace
00001AFB  C3                ret
00001AFC  0AC0              or al,al
00001AFE  7454              jz 0x1b54
00001B00  55                push bp
00001B01  8BEA              mov bp,dx
00001B03  33D7              xor dx,di
00001B05  81CF0080          or di,0x8000
00001B09  81CD0080          or bp,0x8000
00001B0D  81E20080          and dx,0x8000
00001B11  86C2              xchg al,dl
00001B13  2AD1              sub dl,cl
00001B15  2AF0              sub dh,al
00001B17  1AF0              sbb dh,al
00001B19  52                push dx
00001B1A  B002              mov al,0x2
00001B1C  BA0100            mov dx,0x1
00001B1F  3BEF              cmp bp,di
00001B21  7506              jnz 0x1b29
00001B23  3BDE              cmp bx,si
00001B25  7502              jnz 0x1b29
00001B27  3AE5              cmp ah,ch
00001B29  7206              jc 0x1b31
00001B2B  2AE5              sub ah,ch
00001B2D  1BDE              sbb bx,si
00001B2F  1BEF              sbb bp,di
00001B31  D1D2              rcl dx,0x0
00001B33  7211              jc 0x1b46
00001B35  D0E4              shl ah,0x0
00001B37  D1D3              rcl bx,0x0
00001B39  D1D5              rcl bp,0x0
00001B3B  73E2              jnc 0x1b1f
00001B3D  2AE5              sub ah,ch
00001B3F  1BDE              sbb bx,si
00001B41  1BEF              sbb bp,di
00001B43  F8                clc
00001B44  EBEB              jmp 0x1b31
00001B46  FEC8              dec al
00001B48  7811              js 0x1b5b
00001B4A  52                push dx
00001B4B  BA0100            mov dx,0x1
00001B4E  75E5              jnz 0x1b35
00001B50  B240              mov dl,0x40
00001B52  EBE1              jmp 0x1b35
00001B54  33C0              xor ax,ax
00001B56  8BD8              mov bx,ax
00001B58  8BD0              mov dx,ax
00001B5A  C3                ret
00001B5B  8BC2              mov ax,dx
00001B5D  B106              mov cl,0x6
00001B5F  D3E0              shl ax,cl
00001B61  5B                pop bx
00001B62  5A                pop dx
00001B63  59                pop cx
00001B64  5D                pop bp
00001B65  F7D0              not ax
00001B67  F7D3              not bx
00001B69  83F2FF            xor dx,0xffffffffffffffff
00001B6C  7807              js 0x1b75
00001B6E  D1D0              rcl ax,0x0
00001B70  D1D3              rcl bx,0x0
00001B72  D1D2              rcl dx,0x0
00001B74  49                dec cx
00001B75  81C18080          add cx,0x8080
00001B79  058000            add ax,0x80
00001B7C  83D300            adc bx,0x0
00001B7F  83D200            adc dx,0x0
00001B82  7303              jnc 0x1b87
00001B84  D1DA              rcr dx,0x0
00001B86  41                inc cx
00001B87  F6C540            test ch,0x40
00001B8A  75C8              jnz 0x1b54
00001B8C  41                inc cx
00001B8D  8AC1              mov al,cl
00001B8F  32F5              xor dh,ch
00001B91  D0ED              shr ch,0x0
00001B93  C3                ret
00001B94  52                push dx
00001B95  33D7              xor dx,di
00001B97  5A                pop dx
00001B98  7906              jns 0x1ba0
00001B9A  52                push dx
00001B9B  D1D2              rcl dx,0x0
00001B9D  5A                pop dx
00001B9E  EB1E              jmp 0x1bbe
00001BA0  F6C680            test dh,0x80
00001BA3  7407              jz 0x1bac
00001BA5  E80400            call 0x1bac
00001BA8  7414              jz 0x1bbe
00001BAA  F5                cmc
00001BAB  C3                ret
00001BAC  3AC1              cmp al,cl
00001BAE  750E              jnz 0x1bbe
00001BB0  0AC0              or al,al
00001BB2  740A              jz 0x1bbe
00001BB4  3BD7              cmp dx,di
00001BB6  7506              jnz 0x1bbe
00001BB8  3BDE              cmp bx,si
00001BBA  7502              jnz 0x1bbe
00001BBC  3AE5              cmp ah,ch
00001BBE  C3                ret
00001BBF  BFD9BC            mov di,0xbcd9
00001BC2  BE3C00            mov si,0x3c
00001BC5  8B4502            mov ax,[di+0x2]
00001BC8  33D2              xor dx,dx
00001BCA  F7F6              div si
00001BCC  91                xchg ax,cx
00001BCD  8B05              mov ax,[di]
00001BCF  F7F6              div si
00001BD1  D1EA              shr dx,0x0
00001BD3  8BDA              mov bx,dx
00001BD5  91                xchg ax,cx
00001BD6  33D2              xor dx,dx
00001BD8  F7F6              div si
00001BDA  91                xchg ax,cx
00001BDB  F7F6              div si
00001BDD  51                push cx
00001BDE  B105              mov cl,0x5
00001BE0  D3E2              shl dx,cl
00001BE2  59                pop cx
00001BE3  03DA              add bx,dx
00001BE5  BE1800            mov si,0x18
00001BE8  91                xchg ax,cx
00001BE9  33D2              xor dx,dx
00001BEB  F7F6              div si
00001BED  91                xchg ax,cx
00001BEE  F7F6              div si
00001BF0  B90B00            mov cx,0xb
00001BF3  D3E2              shl dx,cl
00001BF5  03DA              add bx,dx
00001BF7  93                xchg ax,bx
00001BF8  AB                stosw
00001BF9  93                xchg ax,bx
00001BFA  B9F4FF            mov cx,0xfff4
00001BFD  059F02            add ax,0x29f
00001C00  7205              jc 0x1c07
00001C02  3DA82D            cmp ax,0x2da8
00001C05  7226              jc 0x1c2d
00001C07  2DA82D            sub ax,0x2da8
00001C0A  B91400            mov cx,0x14
00001C0D  33D2              xor dx,dx
00001C0F  BEB13A            mov si,0x3ab1
00001C12  F7F6              div si
00001C14  52                push dx
00001C15  BE9001            mov si,0x190
00001C18  F7E6              mul si
00001C1A  03C8              add cx,ax
00001C1C  58                pop ax
00001C1D  33D2              xor dx,dx
00001C1F  BEAC8E            mov si,0x8eac
00001C22  F7F6              div si
00001C24  52                push dx
00001C25  BE6400            mov si,0x64
00001C28  F7E6              mul si
00001C2A  03C8              add cx,ax
00001C2C  58                pop ax
00001C2D  33D2              xor dx,dx
00001C2F  BEB505            mov si,0x5b5
00001C32  F7F6              div si
00001C34  D1E0              shl ax,0x0
00001C36  D1E0              shl ax,0x0
00001C38  03C8              add cx,ax
00001C3A  BE6D01            mov si,0x16d
00001C3D  8BC2              mov ax,dx
00001C3F  33D2              xor dx,dx
00001C41  F7F6              div si
00001C43  3D0400            cmp ax,0x4
00001C46  7203              jc 0x1c4b
00001C48  48                dec ax
00001C49  03D6              add dx,si
00001C4B  03C8              add cx,ax
00001C4D  BB0300            mov bx,0x3
00001C50  BE8BB0            mov si,0xb08b
00001C53  AD                lodsw
00001C54  2BD0              sub dx,ax
00001C56  7203              jc 0x1c5b
00001C58  43                inc bx
00001C59  EBF8              jmp 0x1c53
00001C5B  03D0              add dx,ax
00001C5D  80FB0C            cmp bl,0xc
00001C60  7204              jc 0x1c66
00001C62  41                inc cx
00001C63  83EB0C            sub bx,0xc
00001C66  42                inc dx
00001C67  0BDB              or bx,bx
00001C69  7503              jnz 0x1c6e
00001C6B  B30C              mov bl,0xc
00001C6D  49                dec cx
00001C6E  91                xchg ax,cx
00001C6F  B109              mov cl,0x9
00001C71  D3E0              shl ax,cl
00001C73  B105              mov cl,0x5
00001C75  D3E3              shl bx,cl
00001C77  03C3              add ax,bx
00001C79  03C2              add ax,dx
00001C7B  AB                stosw
00001C7C  C3                ret
00001C7D  55                push bp
00001C7E  8BEC              mov bp,sp
00001C80  81EC8000          sub sp,0x80
00001C84  BECBB9            mov si,0xb9cb
00001C87  8D7E80            lea di,[bp-0x80]
00001C8A  57                push di
00001C8B  E89EF2            call 0xf2c
00001C8E  BE80B0            mov si,0xb080
00001C91  5F                pop di
00001C92  16                push ss
00001C93  57                push di
00001C94  E8A7F2            call 0xf3e
00001C97  32C0              xor al,al
00001C99  AA                stosb
00001C9A  E874F3            call 0x1011
00001C9D  7209              jc 0x1ca8
00001C9F  FE0689B0          inc byte [0xb089]
00001CA3  BECBB9            mov si,0xb9cb
00001CA6  EBDC              jmp 0x1c84
00001CA8  8D7680            lea si,[bp-0x80]
00001CAB  B101              mov cl,0x1
00001CAD  E8FEF2            call 0xfae
00001CB0  8946FE            mov [bp-0x2],ax
00001CB3  727D              jc 0x1d32
00001CB5  93                xchg ax,bx
00001CB6  8D7680            lea si,[bp-0x80]
00001CB9  C47E06            les di,word [bp+0x6]
00001CBC  E863F2            call 0xf22
00001CBF  A0A0BD            mov al,[0xbda0]
00001CC2  3C02              cmp al,0x2
00001CC4  750A              jnz 0x1cd0
00001CC6  BA7CAF            mov dx,0xaf7c
00001CC9  B90400            mov cx,0x4
00001CCC  B440              mov ah,0x40
00001CCE  CD21              int byte 0x21
00001CD0  33C0              xor ax,ax
00001CD2  3B0659BD          cmp ax,[0xbd59]
00001CD6  7D5D              jnl 0x1d35
00001CD8  50                push ax
00001CD9  E85605            call 0x2232
00001CDC  268A05            mov al,[es:di]
00001CDF  A801              test al,0x1
00001CE1  740C              jz 0x1cef
00001CE3  26F6451780        test byte [es:di+0x17],0x80
00001CE8  7409              jz 0x1cf3
00001CEA  268065177F        and byte [es:di+0x17],0x7f
00001CEF  58                pop ax
00001CF0  40                inc ax
00001CF1  EBDF              jmp 0x1cd2
00001CF3  807E0400          cmp byte [bp+0x4],0x0
00001CF7  7504              jnz 0x1cfd
00001CF9  2418              and al,0x18
00001CFB  75F2              jnz 0x1cef
00001CFD  8D7EA7            lea di,[bp-0x59]
00001D00  57                push di
00001D01  1E                push ds
00001D02  BE1E00            mov si,0x1e
00001D05  06                push es
00001D06  1F                pop ds
00001D07  16                push ss
00001D08  07                pop es
00001D09  E816F2            call 0xf22
00001D0C  1F                pop ds
00001D0D  5F                pop di
00001D0E  BECCAF            mov si,0xafcc
00001D11  E82AF2            call 0xf3e
00001D14  8D56A8            lea dx,[bp-0x58]
00001D17  8A46A7            mov al,[bp-0x59]
00001D1A  8B5EFE            mov bx,[bp-0x2]
00001D1D  91                xchg ax,cx
00001D1E  B440              mov ah,0x40
00001D20  CD21              int byte 0x21
00001D22  73CB              jnc 0x1cef
00001D24  8B5EFE            mov bx,[bp-0x2]
00001D27  E802F3            call 0x102c
00001D2A  1E                push ds
00001D2B  C57606            lds si,word [bp+0x6]
00001D2E  E8CDF2            call 0xffe
00001D31  1F                pop ds
00001D32  F9                stc
00001D33  EB07              jmp 0x1d3c
00001D35  8B5EFE            mov bx,[bp-0x2]
00001D38  E8F1F2            call 0x102c
00001D3B  F8                clc
00001D3C  8BE5              mov sp,bp
00001D3E  5D                pop bp
00001D3F  C20600            ret word 0x6
00001D42  55                push bp
00001D43  8BEC              mov bp,sp
00001D45  83EC50            sub sp,0x50
00001D48  A0DBA3            mov al,[0xa3db]
00001D4B  A2B3BD            mov [0xbdb3],al
00001D4E  8D7EB0            lea di,[bp-0x50]
00001D51  57                push di
00001D52  BE66A8            mov si,0xa866
00001D55  E8D4F1            call 0xf2c
00001D58  BEE6A3            mov si,0xa3e6
00001D5B  A0A0BD            mov al,[0xbda0]
00001D5E  98                cbw
00001D5F  D1E0              shl ax,0x0
00001D61  D1E0              shl ax,0x0
00001D63  03F0              add si,ax
00001D65  5F                pop di
00001D66  57                push di
00001D67  E8D4F1            call 0xf3e
00001D6A  BE9CA8            mov si,0xa89c
00001D6D  5F                pop di
00001D6E  57                push di
00001D6F  E8CCF1            call 0xf3e
00001D72  5F                pop di
00001D73  E86700            call 0x1ddd
00001D76  BA0001            mov dx,0x100
00001D79  E831EB            call 0x8ad
00001D7C  E891E9            call 0x710
00001D7F  A1C7B9            mov ax,[0xb9c7]
00001D82  8B1659BD          mov dx,[0xbd59]
00001D86  3BD0              cmp dx,ax
00001D88  7E07              jng 0x1d91
00001D8A  8B1671AF          mov dx,[0xaf71]
00001D8E  4A                dec dx
00001D8F  EB03              jmp 0x1d94
00001D91  80C202            add dl,0x2
00001D94  86F2              xchg dh,dl
00001D96  E846EA            call 0x7df
00001D99  8D7EE0            lea di,[bp-0x20]
00001D9C  16                push ss
00001D9D  57                push di
00001D9E  E850F4            call 0x11f1
00001DA1  8D7EC0            lea di,[bp-0x40]
00001DA4  16                push ss
00001DA5  57                push di
00001DA6  C40668BD          les ax,word [0xbd68]
00001DAA  8CC3              mov bx,es
00001DAC  B109              mov cl,0x9
00001DAE  E84EF4            call 0x11ff
00001DB1  E84900            call 0x1dfd
00001DB4  1E                push ds
00001DB5  BE61BD            mov si,0xbd61
00001DB8  56                push si
00001DB9  E844FC            call 0x1a00
00001DBC  8D7EC0            lea di,[bp-0x40]
00001DBF  16                push ss
00001DC0  57                push di
00001DC1  B104              mov cl,0x4
00001DC3  E837F4            call 0x11fd
00001DC6  E83400            call 0x1dfd
00001DC9  BFF3AF            mov di,0xaff3
00001DCC  E82A00            call 0x1df9
00001DCF  5F                pop di
00001DD0  07                pop es
00001DD1  E80900            call 0x1ddd
00001DD4  E8D9EA            call 0x8b0
00001DD7  E836E9            call 0x710
00001DDA  E95DF0            jmp 0xe3a
00001DDD  8A1D              mov bl,[di]
00001DDF  A0C3B9            mov al,[0xb9c3]
00001DE2  48                dec ax
00001DE3  98                cbw
00001DE4  2AD8              sub bl,al
00001DE6  7304              jnc 0x1dec
00001DE8  33C0              xor ax,ax
00001DEA  8BD8              mov bx,ax
00001DEC  03F8              add di,ax
00001DEE  881D              mov [di],bl
00001DF0  C3                ret
00001DF1  BF4FB0            mov di,0xb04f
00001DF4  EB03              jmp 0x1df9
00001DF6  BF77B0            mov di,0xb077
00001DF9  5A                pop dx
00001DFA  1E                push ds
00001DFB  57                push di
00001DFC  52                push dx
00001DFD  8BDC              mov bx,sp
00001DFF  1E                push ds
00001E00  36C47F06          les di,word [ss:bx+0x6]
00001E04  36C57702          lds si,word [ss:bx+0x2]
00001E08  268A0D            mov cl,[es:di]
00001E0B  32ED              xor ch,ch
00001E0D  AC                lodsb
00001E0E  98                cbw
00001E0F  260005            add [es:di],al
00001E12  03F9              add di,cx
00001E14  47                inc di
00001E15  91                xchg ax,cx
00001E16  F3A4              rep movsb
00001E18  1F                pop ds
00001E19  C20400            ret word 0x4
00001E1C  58                pop ax
00001E1D  8DBEFCFD          lea di,[bp-0x204]
00001E21  16                push ss
00001E22  57                push di
00001E23  50                push ax
00001E24  C47EFC            les di,word [bp-0x4]
00001E27  268B4511          mov ax,[es:di+0x11]
00001E2B  C3                ret
00001E2C  55                push bp
00001E2D  8BEC              mov bp,sp
00001E2F  8B4604            mov ax,[bp+0x4]
00001E32  8BD8              mov bx,ax
00001E34  E8FA03            call 0x2231
00001E37  06                push es
00001E38  57                push di
00001E39  81EC0002          sub sp,0x200
00001E3D  3B1EC8BC          cmp bx,[0xbcc8]
00001E41  7505              jnz 0x1e48
00001E43  A0D8A3            mov al,[0xa3d8]
00001E46  EB03              jmp 0x1e4b
00001E48  A0DCA3            mov al,[0xa3dc]
00001E4B  26F60501          test byte [es:di],0x1
00001E4F  740B              jz 0x1e5c
00001E51  24F0              and al,0xf0
00001E53  8AD0              mov dl,al
00001E55  A0DAA3            mov al,[0xa3da]
00001E58  240F              and al,0xf
00001E5A  0AC2              or al,dl
00001E5C  A2B3BD            mov [0xbdb3],al
00001E5F  8B4604            mov ax,[bp+0x4]
00001E62  2B06C4BC          sub ax,[0xbcc4]
00001E66  40                inc ax
00001E67  40                inc ax
00001E68  8AF0              mov dh,al
00001E6A  32D2              xor dl,dl
00001E6C  E870E9            call 0x7df
00001E6F  E8E2E7            call 0x654
00001E72  3A26B3BD          cmp ah,[0xbdb3]
00001E76  7403              jz 0x1e7b
00001E78  E895E8            call 0x710
00001E7B  8DBEFCFE          lea di,[bp-0x104]
00001E7F  16                push ss
00001E80  57                push di
00001E81  C47EFC            les di,word [bp-0x4]
00001E84  E862F3            call 0x11e9
00001E87  E892FF            call 0x1e1c
00001E8A  26C44507          les ax,word [es:di+0x7]
00001E8E  8CC3              mov bx,es
00001E90  B109              mov cl,0x9
00001E92  E86AF3            call 0x11ff
00001E95  E865FF            call 0x1dfd
00001E98  E881FF            call 0x1e1c
00001E9B  06                push es
00001E9C  57                push di
00001E9D  E860FB            call 0x1a00
00001EA0  B104              mov cl,0x4
00001EA2  E858F3            call 0x11fd
00001EA5  E855FF            call 0x1dfd
00001EA8  BFF3AF            mov di,0xaff3
00001EAB  E84BFF            call 0x1df9
00001EAE  E86BFF            call 0x1e1c
00001EB1  241F              and al,0x1f
00001EB3  E8BBEB            call 0xa71
00001EB6  E844FF            call 0x1dfd
00001EB9  E835FF            call 0x1df1
00001EBC  E85DFF            call 0x1e1c
00001EBF  80E401            and ah,0x1
00001EC2  E8A8EB            call 0xa6d
00001EC5  E835FF            call 0x1dfd
00001EC8  E826FF            call 0x1df1
00001ECB  E84EFF            call 0x1e1c
00001ECE  B109              mov cl,0x9
00001ED0  D3E8              shr ax,cl
00001ED2  0450              add al,0x50
00001ED4  E89AEB            call 0xa71
00001ED7  E823FF            call 0x1dfd
00001EDA  E819FF            call 0x1df6
00001EDD  E83CFF            call 0x1e1c
00001EE0  268B450F          mov ax,[es:di+0xf]
00001EE4  B10B              mov cl,0xb
00001EE6  E886EB            call 0xa6f
00001EE9  E811FF            call 0x1dfd
00001EEC  BF0EB0            mov di,0xb00e
00001EEF  E807FF            call 0x1df9
00001EF2  E827FF            call 0x1e1c
00001EF5  268B450F          mov ax,[es:di+0xf]
00001EF9  E86EEB            call 0xa6a
00001EFC  E8FEFE            call 0x1dfd
00001EFF  E8F4FE            call 0x1df6
00001F02  C47EFC            les di,word [bp-0x4]
00001F05  268A15            mov dl,[es:di]
00001F08  8DBEFCFD          lea di,[bp-0x204]
00001F0C  16                push ss
00001F0D  57                push di
00001F0E  16                push ss
00001F0F  07                pop es
00001F10  B005              mov al,0x5
00001F12  AA                stosb
00001F13  B020              mov al,0x20
00001F15  D1EA              shr dx,0x0
00001F17  D1EA              shr dx,0x0
00001F19  7302              jnc 0x1f1d
00001F1B  B02B              mov al,0x2b
00001F1D  AA                stosb
00001F1E  B020              mov al,0x20
00001F20  D1EA              shr dx,0x0
00001F22  7302              jnc 0x1f26
00001F24  B02B              mov al,0x2b
00001F26  AA                stosb
00001F27  B042              mov al,0x42
00001F29  D1EA              shr dx,0x0
00001F2B  7302              jnc 0x1f2f
00001F2D  B056              mov al,0x56
00001F2F  D1EA              shr dx,0x0
00001F31  7302              jnc 0x1f35
00001F33  B044              mov al,0x44
00001F35  D1EA              shr dx,0x0
00001F37  7314              jnc 0x1f4d
00001F39  803EA0BD19        cmp byte [0xbda0],0x19
00001F3E  7407              jz 0x1f47
00001F40  803EA0BD13        cmp byte [0xbda0],0x13
00001F45  7504              jnz 0x1f4b
00001F47  B053              mov al,0x53
00001F49  EB02              jmp 0x1f4d
00001F4B  B054              mov al,0x54
00001F4D  AA                stosb
00001F4E  B020              mov al,0x20
00001F50  D1EA              shr dx,0x0
00001F52  7302              jnc 0x1f56
00001F54  B02B              mov al,0x2b
00001F56  AA                stosb
00001F57  B020              mov al,0x20
00001F59  D1EA              shr dx,0x0
00001F5B  7302              jnc 0x1f5f
00001F5D  B02B              mov al,0x2b
00001F5F  AA                stosb
00001F60  E89AFE            call 0x1dfd
00001F63  E890FE            call 0x1df6
00001F66  E8B3FE            call 0x1e1c
00001F69  83C71E            add di,0x1e
00001F6C  06                push es
00001F6D  57                push di
00001F6E  E826F2            call 0x1197
00001F71  E889FE            call 0x1dfd
00001F74  5E                pop si
00001F75  8DBEFCFD          lea di,[bp-0x204]
00001F79  56                push si
00001F7A  57                push di
00001F7B  16                push ss
00001F7C  07                pop es
00001F7D  8A0EC3B9          mov cl,[0xb9c3]
00001F81  B250              mov dl,0x50
00001F83  E8CCF2            call 0x1252
00001F86  5E                pop si
00001F87  B250              mov dl,0x50
00001F89  E88FF1            call 0x111b
00001F8C  8B4604            mov ax,[bp+0x4]
00001F8F  0402              add al,0x2
00001F91  2B06C4BC          sub ax,[0xbcc4]
00001F95  33FF              xor di,di
00001F97  E830EA            call 0x9ca
00001F9A  8BE5              mov sp,bp
00001F9C  5D                pop bp
00001F9D  C20200            ret word 0x2
00001FA0  55                push bp
00001FA1  8BEC              mov bp,sp
00001FA3  33C0              xor ax,ax
00001FA5  89469E            mov [bp-0x62],ax
00001FA8  89469C            mov [bp-0x64],ax
00001FAB  40                inc ax
00001FAC  50                push ax
00001FAD  A159BD            mov ax,[0xbd59]
00001FB0  50                push ax
00001FB1  48                dec ax
00001FB2  8946A0            mov [bp-0x60],ax
00001FB5  A05FBD            mov al,[0xbd5f]
00001FB8  50                push ax
00001FB9  81EC5E02          sub sp,0x25e
00001FBD  8B7E9C            mov di,[bp-0x64]
00001FC0  D1E7              shl di,0x0
00001FC2  D1E7              shl di,0x0
00001FC4  8B439E            mov ax,[bp+di-0x62]
00001FC7  8946FE            mov [bp-0x2],ax
00001FCA  8B43A0            mov ax,[bp+di-0x60]
00001FCD  8946FC            mov [bp-0x4],ax
00001FD0  FF4E9C            dec word [bp-0x64]
00001FD3  8B46FE            mov ax,[bp-0x2]
00001FD6  8946F8            mov [bp-0x8],ax
00001FD9  93                xchg ax,bx
00001FDA  8B46FC            mov ax,[bp-0x4]
00001FDD  8946F6            mov [bp-0xa],ax
00001FE0  03C3              add ax,bx
00001FE2  D1E8              shr ax,0x0
00001FE4  E84B02            call 0x2232
00001FE7  897EF2            mov [bp-0xe],di
00001FEA  8C46F4            mov word [bp-0xc],es
00001FED  8A46FA            mov al,[bp-0x6]
00001FF0  0AC0              or al,al
00001FF2  754D              jnz 0x2041
00001FF4  8DB69CFE          lea si,[bp-0x164]
00001FF8  16                push ss
00001FF9  56                push si
00001FFA  E82A02            call 0x2227
00001FFD  83C71E            add di,0x1e
00002000  06                push es
00002001  57                push di
00002002  E892F1            call 0x1197
00002005  8DBE9CFD          lea di,[bp-0x264]
00002009  16                push ss
0000200A  57                push di
0000200B  8E46F4            mov es,word [bp-0xc]
0000200E  06                push es
0000200F  BF1E00            mov di,0x1e
00002012  57                push di
00002013  E881F1            call 0x1197
00002016  E82702            call 0x2240
00002019  7305              jnc 0x2020
0000201B  FF46F8            inc word [bp-0x8]
0000201E  EBD4              jmp 0x1ff4
00002020  8DB69CFE          lea si,[bp-0x164]
00002024  16                push ss
00002025  56                push si
00002026  E80302            call 0x222c
00002029  06                push es
0000202A  BF1E00            mov di,0x1e
0000202D  57                push di
0000202E  E866F1            call 0x1197
00002031  8DBE9CFD          lea di,[bp-0x264]
00002035  16                push ss
00002036  57                push di
00002037  E80602            call 0x2240
0000203A  764D              jna 0x2089
0000203C  FF4EF6            dec word [bp-0xa]
0000203F  EBDF              jmp 0x2020
00002041  3C01              cmp al,0x1
00002043  7547              jnz 0x208c
00002045  8DB69CFE          lea si,[bp-0x164]
00002049  16                push ss
0000204A  56                push si
0000204B  E8D901            call 0x2227
0000204E  06                push es
0000204F  57                push di
00002050  E8F8F0            call 0x114b
00002053  8DBE9CFD          lea di,[bp-0x264]
00002057  16                push ss
00002058  57                push di
00002059  C47EF2            les di,word [bp-0xe]
0000205C  06                push es
0000205D  57                push di
0000205E  E8EAF0            call 0x114b
00002061  E8DC01            call 0x2240
00002064  7305              jnc 0x206b
00002066  FF46F8            inc word [bp-0x8]
00002069  EBDA              jmp 0x2045
0000206B  8DB69CFE          lea si,[bp-0x164]
0000206F  16                push ss
00002070  56                push si
00002071  E8B801            call 0x222c
00002074  06                push es
00002075  57                push di
00002076  E8D2F0            call 0x114b
00002079  8DBE9CFD          lea di,[bp-0x264]
0000207D  16                push ss
0000207E  57                push di
0000207F  E8BE01            call 0x2240
00002082  7605              jna 0x2089
00002084  FF4EF6            dec word [bp-0xa]
00002087  EBE2              jmp 0x206b
00002089  E9D600            jmp 0x2162
0000208C  3C02              cmp al,0x2
0000208E  753E              jnz 0x20ce
00002090  E89401            call 0x2227
00002093  26C4450B          les ax,word [es:di+0xb]
00002097  8CC2              mov dx,es
00002099  C47EF2            les di,word [bp-0xe]
0000209C  263B550D          cmp dx,[es:di+0xd]
000020A0  7F08              jg 0x20aa
000020A2  7C0B              jl 0x20af
000020A4  263B450B          cmp ax,[es:di+0xb]
000020A8  7605              jna 0x20af
000020AA  FF46F8            inc word [bp-0x8]
000020AD  EBE1              jmp 0x2090
000020AF  E87A01            call 0x222c
000020B2  26C4450B          les ax,word [es:di+0xb]
000020B6  8CC2              mov dx,es
000020B8  C47EF2            les di,word [bp-0xe]
000020BB  263B550D          cmp dx,[es:di+0xd]
000020BF  7C08              jl 0x20c9
000020C1  7FC6              jg 0x2089
000020C3  263B450B          cmp ax,[es:di+0xb]
000020C7  73C0              jnc 0x2089
000020C9  FF4EF6            dec word [bp-0xa]
000020CC  EBE1              jmp 0x20af
000020CE  3C03              cmp al,0x3
000020D0  7524              jnz 0x20f6
000020D2  E85201            call 0x2227
000020D5  06                push es
000020D6  57                push di
000020D7  C47EF2            les di,word [bp-0xe]
000020DA  E8C7F8            call 0x19a4
000020DD  7605              jna 0x20e4
000020DF  FF46F8            inc word [bp-0x8]
000020E2  EBEE              jmp 0x20d2
000020E4  C47EF2            les di,word [bp-0xe]
000020E7  06                push es
000020E8  57                push di
000020E9  E84001            call 0x222c
000020EC  E8B5F8            call 0x19a4
000020EF  7698              jna 0x2089
000020F1  FF4EF6            dec word [bp-0xa]
000020F4  EBEE              jmp 0x20e4
000020F6  3C04              cmp al,0x4
000020F8  753E              jnz 0x2138
000020FA  E82A01            call 0x2227
000020FD  26C4450F          les ax,word [es:di+0xf]
00002101  8CC2              mov dx,es
00002103  C47EF2            les di,word [bp-0xe]
00002106  263B5511          cmp dx,[es:di+0x11]
0000210A  7F08              jg 0x2114
0000210C  7C0B              jl 0x2119
0000210E  263B450F          cmp ax,[es:di+0xf]
00002112  7605              jna 0x2119
00002114  FF46F8            inc word [bp-0x8]
00002117  EBE1              jmp 0x20fa
00002119  E81001            call 0x222c
0000211C  26C4450F          les ax,word [es:di+0xf]
00002120  8CC2              mov dx,es
00002122  C47EF2            les di,word [bp-0xe]
00002125  263B5511          cmp dx,[es:di+0x11]
00002129  7C08              jl 0x2133
0000212B  7F35              jg 0x2162
0000212D  263B450F          cmp ax,[es:di+0xf]
00002131  732F              jnc 0x2162
00002133  FF4EF6            dec word [bp-0xa]
00002136  EBE1              jmp 0x2119
00002138  E8EC00            call 0x2227
0000213B  268B4501          mov ax,[es:di+0x1]
0000213F  C47EF2            les di,word [bp-0xe]
00002142  263B4501          cmp ax,[es:di+0x1]
00002146  7D05              jnl 0x214d
00002148  FF46F8            inc word [bp-0x8]
0000214B  EBEB              jmp 0x2138
0000214D  E8DC00            call 0x222c
00002150  268B4501          mov ax,[es:di+0x1]
00002154  C47EF2            les di,word [bp-0xe]
00002157  263B4501          cmp ax,[es:di+0x1]
0000215B  7E05              jng 0x2162
0000215D  FF4EF6            dec word [bp-0xa]
00002160  EBEB              jmp 0x214d
00002162  8B56F8            mov dx,[bp-0x8]
00002165  3B56F6            cmp dx,[bp-0xa]
00002168  7F48              jg 0x21b2
0000216A  A1C8BC            mov ax,[0xbcc8]
0000216D  48                dec ax
0000216E  3BC2              cmp ax,dx
00002170  7505              jnz 0x2177
00002172  8B46F6            mov ax,[bp-0xa]
00002175  EB07              jmp 0x217e
00002177  3B46F6            cmp ax,[bp-0xa]
0000217A  7506              jnz 0x2182
0000217C  8BC2              mov ax,dx
0000217E  40                inc ax
0000217F  A3C8BC            mov [0xbcc8],ax
00002182  E8A200            call 0x2227
00002185  897EEE            mov [bp-0x12],di
00002188  8C46F0            mov word [bp-0x10],es
0000218B  E89E00            call 0x222c
0000218E  8CC3              mov bx,es
00002190  8B46F8            mov ax,[bp-0x8]
00002193  D1E0              shl ax,0x0
00002195  8E0636BD          mov es,word [0xbd36]
00002199  97                xchg ax,di
0000219A  93                xchg ax,bx
0000219B  AB                stosw
0000219C  8B46F6            mov ax,[bp-0xa]
0000219F  D1E0              shl ax,0x0
000021A1  97                xchg ax,di
000021A2  8B46F0            mov ax,[bp-0x10]
000021A5  AB                stosw
000021A6  FF46F8            inc word [bp-0x8]
000021A9  FF4EF6            dec word [bp-0xa]
000021AC  8B46F8            mov ax,[bp-0x8]
000021AF  3B46F6            cmp ax,[bp-0xa]
000021B2  7D03              jnl 0x21b7
000021B4  E936FE            jmp 0x1fed
000021B7  8B46FC            mov ax,[bp-0x4]
000021BA  2B46F8            sub ax,[bp-0x8]
000021BD  0346FE            add ax,[bp-0x2]
000021C0  2B46F6            sub ax,[bp-0xa]
000021C3  7826              js 0x21eb
000021C5  8B46F8            mov ax,[bp-0x8]
000021C8  3B46FC            cmp ax,[bp-0x4]
000021CB  7D16              jnl 0x21e3
000021CD  FF469C            inc word [bp-0x64]
000021D0  8B46F8            mov ax,[bp-0x8]
000021D3  8B7E9C            mov di,[bp-0x64]
000021D6  D1E7              shl di,0x0
000021D8  D1E7              shl di,0x0
000021DA  89439E            mov [bp+di-0x62],ax
000021DD  8B46FC            mov ax,[bp-0x4]
000021E0  8943A0            mov [bp+di-0x60],ax
000021E3  8B46F6            mov ax,[bp-0xa]
000021E6  8946FC            mov [bp-0x4],ax
000021E9  EB24              jmp 0x220f
000021EB  8B46FE            mov ax,[bp-0x2]
000021EE  3B46F6            cmp ax,[bp-0xa]
000021F1  7D16              jnl 0x2209
000021F3  FF469C            inc word [bp-0x64]
000021F6  8B46FE            mov ax,[bp-0x2]
000021F9  8B7E9C            mov di,[bp-0x64]
000021FC  D1E7              shl di,0x0
000021FE  D1E7              shl di,0x0
00002200  89439E            mov [bp+di-0x62],ax
00002203  8B46F6            mov ax,[bp-0xa]
00002206  8943A0            mov [bp+di-0x60],ax
00002209  8B46F8            mov ax,[bp-0x8]
0000220C  8946FE            mov [bp-0x2],ax
0000220F  8B46FE            mov ax,[bp-0x2]
00002212  3B46FC            cmp ax,[bp-0x4]
00002215  7D03              jnl 0x221a
00002217  E9B9FD            jmp 0x1fd3
0000221A  8B469C            mov ax,[bp-0x64]
0000221D  0BC0              or ax,ax
0000221F  7803              js 0x2224
00002221  E999FD            jmp 0x1fbd
00002224  E913EC            jmp 0xe3a
00002227  8B46F8            mov ax,[bp-0x8]
0000222A  EB06              jmp 0x2232
0000222C  8B46F6            mov ax,[bp-0xa]
0000222F  EB01              jmp 0x2232
00002231  48                dec ax
00002232  D1E0              shl ax,0x0
00002234  8E0636BD          mov es,word [0xbd36]
00002238  8BF8              mov di,ax
0000223A  268E05            mov es,word [es:di]
0000223D  33FF              xor di,di
0000223F  C3                ret
00002240  8BDC              mov bx,sp
00002242  C47F06            les di,word [bx+0x6]
00002245  C57702            lds si,word [bx+0x2]
00002248  AC                lodsb
00002249  98                cbw
0000224A  268A0D            mov cl,[es:di]
0000224D  32ED              xor ch,ch
0000224F  47                inc di
00002250  3BC1              cmp ax,cx
00002252  7602              jna 0x2256
00002254  8BC8              mov cx,ax
00002256  9F                lahf
00002257  F3A6              repe cmpsb
00002259  7501              jnz 0x225c
0000225B  9E                sahf
0000225C  C20800            ret word 0x8
0000225F  55                push bp
00002260  8BEC              mov bp,sp
00002262  8B1E59BD          mov bx,[0xbd59]
00002266  53                push bx
00002267  81ECA000          sub sp,0xa0
0000226B  3B0659BD          cmp ax,[0xbd59]
0000226F  7202              jc 0x2273
00002271  33C0              xor ax,ax
00002273  50                push ax
00002274  8D7EAE            lea di,[bp-0x52]
00002277  16                push ss
00002278  57                push di
00002279  8DBE5EFF          lea di,[bp-0xa2]
0000227D  16                push ss
0000227E  57                push di
0000227F  E8B0FF            call 0x2232
00002282  06                push es
00002283  BF1E00            mov di,0x1e
00002286  57                push di
00002287  E80DEF            call 0x1197
0000228A  E825EF            call 0x11b2
0000228D  C47E04            les di,word [bp+0x4]
00002290  06                push es
00002291  57                push di
00002292  E88701            call 0x241c
00002295  720A              jc 0x22a1
00002297  58                pop ax
00002298  40                inc ax
00002299  FF4EFE            dec word [bp-0x2]
0000229C  75CD              jnz 0x226b
0000229E  F8                clc
0000229F  EB03              jmp 0x22a4
000022A1  58                pop ax
000022A2  40                inc ax
000022A3  F9                stc
000022A4  E905EF            jmp 0x11ac
000022A7  BE6FAF            mov si,0xaf6f
000022AA  BB4FA8            mov bx,0xa84f
000022AD  E8B7E2            call 0x567
000022B0  B001              mov al,0x1
000022B2  55                push bp
000022B3  8BEC              mov bp,sp
000022B5  81EC0002          sub sp,0x200
000022B9  98                cbw
000022BA  50                push ax
000022BB  A061BD            mov al,[0xbd61]
000022BE  A2B3BD            mov [0xbdb3],al
000022C1  A157BD            mov ax,[0xbd57]
000022C4  3B0662BD          cmp ax,[0xbd62]
000022C8  7403              jz 0x22cd
000022CA  E822E6            call 0x8ef
000022CD  8B1EB4B1          mov bx,[0xb1b4]
000022D1  E858ED            call 0x102c
000022D4  58                pop ax
000022D5  50                push ax
000022D6  3C09              cmp al,0x9
000022D8  753E              jnz 0x2318
000022DA  A123BF            mov ax,[0xbf23]
000022DD  A325BF            mov [0xbf25],ax
000022E0  E80A10            call 0x32ed
000022E3  803EC1B900        cmp byte [0xb9c1],0x0
000022E8  7550              jnz 0x233a
000022EA  E84AED            call 0x1037
000022ED  1E                push ds
000022EE  BF9CBA            mov di,0xba9c
000022F1  57                push di
000022F2  32C0              xor al,al
000022F4  50                push ax
000022F5  E85E03            call 0x2656
000022F8  1E                push ds
000022F9  BFBEB1            mov di,0xb1be
000022FC  57                push di
000022FD  E8F2F4            call 0x17f2
00002300  8DBE00FE          lea di,[bp-0x200]
00002304  16                push ss
00002305  57                push di
00002306  BE9EBB            mov si,0xbb9e
00002309  E820EC            call 0xf2c
0000230C  BF38BD            mov di,0xbd38
0000230F  E8E7FA            call 0x1df9
00002312  BEBEB1            mov si,0xb1be
00002315  E87BF4            call 0x1793
00002318  803EC1B900        cmp byte [0xb9c1],0x0
0000231D  751B              jnz 0x233a
0000231F  A061BD            mov al,[0xbd61]
00002322  A2B3BD            mov [0xbdb3],al
00002325  E8A1E4            call 0x7c9
00002328  E8D9E2            call 0x604
0000232B  803E7EBD00        cmp byte [0xbd7e],0x0
00002330  740B              jz 0x233d
00002332  E8F1E3            call 0x726
00002335  E892E2            call 0x5ca
00002338  EB03              jmp 0x233d
0000233A  E8F1E1            call 0x52e
0000233D  E8C8E6            call 0xa08
00002340  58                pop ax
00002341  3C09              cmp al,0x9
00002343  7502              jnz 0x2347
00002345  32C0              xor al,al
00002347  E90ADD            jmp 0x54
0000234A  8B1EB4B1          mov bx,[0xb1b4]
0000234E  E8DBEC            call 0x102c
00002351  58                pop ax
00002352  5F                pop di
00002353  07                pop es
00002354  50                push ax
00002355  06                push es
00002356  57                push di
00002357  803E7EBD00        cmp byte [0xbd7e],0x0
0000235C  7409              jz 0x2367
0000235E  BF72AA            mov di,0xaa72
00002361  BE6FAF            mov si,0xaf6f
00002364  E8ECE1            call 0x553
00002367  E89AE2            call 0x604
0000236A  803EBEB900        cmp byte [0xb9be],0x0
0000236F  741F              jz 0x2390
00002371  803ECAB900        cmp byte [0xb9ca],0x0
00002376  7503              jnz 0x237b
00002378  E88804            call 0x2803
0000237B  B82135            mov ax,0x3521
0000237E  CD21              int byte 0x21
00002380  891E950B          mov [0xb95],bx
00002384  8C06970B          mov word [0xb97],es
00002388  BA990B            mov dx,0xb99
0000238B  B82125            mov ax,0x2521
0000238E  CD21              int byte 0x21
00002390  E89BE1            call 0x52e
00002393  BE1DBB            mov si,0xbb1d
00002396  E801F4            call 0x179a
00002399  803EBEB900        cmp byte [0xb9be],0x0
0000239E  740C              jz 0x23ac
000023A0  1E                push ds
000023A1  2EC516950B        lds dx,word [cs:0xb95]
000023A6  B82125            mov ax,0x2521
000023A9  CD21              int byte 0x21
000023AB  1F                pop ds
000023AC  C3                ret
000023AD  C406B6B1          les ax,word [0xb1b6]
000023B1  8CC2              mov dx,es
000023B3  0306BAB1          add ax,[0xb1ba]
000023B7  83D200            adc dx,0x0
000023BA  C3                ret
000023BB  33C0              xor ax,ax
000023BD  8BD0              mov dx,ax
000023BF  1E                push ds
000023C0  07                pop es
000023C1  BFB6B1            mov di,0xb1b6
000023C4  AB                stosw
000023C5  92                xchg ax,dx
000023C6  AB                stosw
000023C7  33C0              xor ax,ax
000023C9  AB                stosw
000023CA  1E                push ds
000023CB  8B1EB4B1          mov bx,[0xb1b4]
000023CF  8E1E34BD          mov ds,word [0xbd34]
000023D3  33D2              xor dx,dx
000023D5  B90008            mov cx,0x800
000023D8  B43F              mov ah,0x3f
000023DA  CD21              int byte 0x21
000023DC  AB                stosw
000023DD  1F                pop ds
000023DE  C3                ret
000023DF  33C9              xor cx,cx
000023E1  51                push cx
000023E2  52                push dx
000023E3  C406B6B1          les ax,word [0xb1b6]
000023E7  8CC3              mov bx,es
000023E9  2BD0              sub dx,ax
000023EB  1BCB              sbb cx,bx
000023ED  7209              jc 0x23f8
000023EF  A1BCB1            mov ax,[0xb1bc]
000023F2  2BD0              sub dx,ax
000023F4  7302              jnc 0x23f8
000023F6  E30D              jcxz 0x2405
000023F8  5A                pop dx
000023F9  59                pop cx
000023FA  E8D6EB            call 0xfd3
000023FD  52                push dx
000023FE  50                push ax
000023FF  E8BDFF            call 0x23bf
00002402  58                pop ax
00002403  5A                pop dx
00002404  C3                ret
00002405  81FA6AFF          cmp dx,0xff6a
00002409  77ED              ja 0x23f8
0000240B  5A                pop dx
0000240C  59                pop cx
0000240D  C406B6B1          les ax,word [0xb1b6]
00002411  8BDA              mov bx,dx
00002413  2BD8              sub bx,ax
00002415  891EBAB1          mov [0xb1ba],bx
00002419  91                xchg ax,cx
0000241A  92                xchg ax,dx
0000241B  C3                ret
0000241C  55                push bp
0000241D  8BEC              mov bp,sp
0000241F  C47E08            les di,word [bp+0x8]
00002422  8B7604            mov si,[bp+0x4]
00002425  268A05            mov al,[es:di]
00002428  98                cbw
00002429  47                inc di
0000242A  03C7              add ax,di
0000242C  91                xchg ax,cx
0000242D  AC                lodsb
0000242E  98                cbw
0000242F  03C6              add ax,si
00002431  92                xchg ax,dx
00002432  58                pop ax
00002433  83EC7D            sub sp,0x7d
00002436  50                push ax
00002437  8BDC              mov bx,sp
00002439  3BF9              cmp di,cx
0000243B  722F              jc 0x246c
0000243D  3BF2              cmp si,dx
0000243F  7357              jnc 0x2498
00002441  AC                lodsb
00002442  3C3F              cmp al,0x3f
00002444  74F7              jz 0x243d
00002446  3C2A              cmp al,0x2a
00002448  74F3              jz 0x243d
0000244A  4E                dec si
0000244B  3BDC              cmp bx,sp
0000244D  7464              jz 0x24b3
0000244F  8B37              mov si,[bx]
00002451  8B7F02            mov di,[bx+0x2]
00002454  47                inc di
00002455  8BC1              mov ax,cx
00002457  48                dec ax
00002458  3BF8              cmp di,ax
0000245A  7406              jz 0x2462
0000245C  807F043F          cmp byte [bx+0x4],0x3f
00002460  7505              jnz 0x2467
00002462  83EB05            sub bx,0x5
00002465  EBD2              jmp 0x2439
00002467  FF4702            inc word [bx+0x2]
0000246A  EBCD              jmp 0x2439
0000246C  3BF2              cmp si,dx
0000246E  73DB              jnc 0x244b
00002470  AC                lodsb
00002471  263A05            cmp al,[es:di]
00002474  7503              jnz 0x2479
00002476  47                inc di
00002477  EBC0              jmp 0x2439
00002479  3C2A              cmp al,0x2a
0000247B  7406              jz 0x2483
0000247D  3C3F              cmp al,0x3f
0000247F  75CA              jnz 0x244b
00002481  EB1B              jmp 0x249e
00002483  3BF2              cmp si,dx
00002485  7311              jnc 0x2498
00002487  AC                lodsb
00002488  3C2A              cmp al,0x2a
0000248A  74F7              jz 0x2483
0000248C  3C3F              cmp al,0x3f
0000248E  74F3              jz 0x2483
00002490  3BF9              cmp di,cx
00002492  7217              jc 0x24ab
00002494  3BF2              cmp si,dx
00002496  72B3              jc 0x244b
00002498  F9                stc
00002499  EB19              jmp 0x24b4
0000249B  B02A              mov al,0x2a
0000249D  4E                dec si
0000249E  83C305            add bx,0x5
000024A1  884704            mov [bx+0x4],al
000024A4  8937              mov [bx],si
000024A6  897F02            mov [bx+0x2],di
000024A9  EB8E              jmp 0x2439
000024AB  263805            cmp [es:di],al
000024AE  74EB              jz 0x249b
000024B0  47                inc di
000024B1  EBDD              jmp 0x2490
000024B3  F8                clc
000024B4  E9F0F3            jmp 0x18a7
000024B7  B044              mov al,0x44
000024B9  55                push bp
000024BA  8BEC              mov bp,sp
000024BC  98                cbw
000024BD  57                push di
000024BE  50                push ax
000024BF  8BD8              mov bx,ax
000024C1  3C3C              cmp al,0x3c
000024C3  7202              jc 0x24c7
000024C5  B401              mov ah,0x1
000024C7  B001              mov al,0x1
000024C9  50                push ax
000024CA  D1EB              shr bx,0x0
000024CC  B9270B            mov cx,0xb27
000024CF  8BD1              mov dx,cx
000024D1  2ACB              sub cl,bl
000024D3  02D3              add dl,bl
000024D5  B80007            mov ax,0x700
000024D8  8A3EDDA3          mov bh,[0xa3dd]
000024DC  883EB3BD          mov [0xbdb3],bh
000024E0  55                push bp
000024E1  CD10              int byte 0x10
000024E3  5D                pop bp
000024E4  33C0              xor ax,ax
000024E6  50                push ax
000024E7  81ECA300          sub sp,0xa3
000024EB  8BF7              mov si,di
000024ED  8D7EA9            lea di,[bp-0x57]
000024F0  E839EA            call 0xf2c
000024F3  8DBE59FF          lea di,[bp-0xa7]
000024F7  16                push ss
000024F8  57                push di
000024F9  8D76A9            lea si,[bp-0x57]
000024FC  8A56FC            mov dl,[bp-0x4]
000024FF  E819EC            call 0x111b
00002502  5F                pop di
00002503  07                pop es
00002504  8B56FA            mov dx,[bp-0x6]
00002507  B201              mov dl,0x1
00002509  E8A1E3            call 0x8ad
0000250C  8B56FA            mov dx,[bp-0x6]
0000250F  E8C9E2            call 0x7db
00002512  807EF900          cmp byte [bp-0x7],0x0
00002516  7505              jnz 0x251d
00002518  E8EDE4            call 0xa08
0000251B  EB03              jmp 0x2520
0000251D  E8F7E4            call 0xa17
00002520  E8A7E0            call 0x5ca
00002523  16                push ss
00002524  07                pop es
00002525  A15BBD            mov ax,[0xbd5b]
00002528  3C08              cmp al,0x8
0000252A  7505              jnz 0x2531
0000252C  FE4EFA            dec byte [bp-0x6]
0000252F  EB73              jmp 0x25a4
00002531  3C20              cmp al,0x20
00002533  725D              jc 0x2592
00002535  3CFE              cmp al,0xfe
00002537  7759              ja 0x2592
00002539  803EA0BD05        cmp byte [0xbda0],0x5
0000253E  740D              jz 0x254d
00002540  803EA0BD04        cmp byte [0xbda0],0x4
00002545  7406              jz 0x254d
00002547  807EFB01          cmp byte [bp-0x5],0x1
0000254B  7503              jnz 0x2550
0000254D  E872EE            call 0x13c2
00002550  A25BBD            mov [0xbd5b],al
00002553  8A46FA            mov al,[bp-0x6]
00002556  3A46A9            cmp al,[bp-0x57]
00002559  7F25              jg 0x2580
0000255B  807EF900          cmp byte [bp-0x7],0x0
0000255F  7522              jnz 0x2583
00002561  8D76A9            lea si,[bp-0x57]
00002564  8BFE              mov di,si
00002566  98                cbw
00002567  91                xchg ax,cx
00002568  E323              jcxz 0x258d
0000256A  AC                lodsb
0000256B  3AC1              cmp al,cl
0000256D  7C1E              jl 0x258d
0000256F  98                cbw
00002570  03F0              add si,ax
00002572  40                inc ax
00002573  AA                stosb
00002574  2BC1              sub ax,cx
00002576  8BFE              mov di,si
00002578  4E                dec si
00002579  91                xchg ax,cx
0000257A  FD                std
0000257B  F3A4              rep movsb
0000257D  FC                cld
0000257E  EB09              jmp 0x2589
00002580  FE46A9            inc byte [bp-0x57]
00002583  8D7EA9            lea di,[bp-0x57]
00002586  98                cbw
00002587  03F8              add di,ax
00002589  A05BBD            mov al,[0xbd5b]
0000258C  AA                stosb
0000258D  FE46FA            inc byte [bp-0x6]
00002590  EB4C              jmp 0x25de
00002592  3C19              cmp al,0x19
00002594  7519              jnz 0x25af
00002596  C646A900          mov byte [bp-0x57],0x0
0000259A  EB42              jmp 0x25de
0000259C  3C4D              cmp al,0x4d
0000259E  74ED              jz 0x258d
000025A0  3C53              cmp al,0x53
000025A2  751A              jnz 0x25be
000025A4  8D76A9            lea si,[bp-0x57]
000025A7  8A4EFA            mov cl,[bp-0x6]
000025AA  E88100            call 0x262e
000025AD  EB2F              jmp 0x25de
000025AF  0AC0              or al,al
000025B1  7514              jnz 0x25c7
000025B3  86E0              xchg ah,al
000025B5  3C4B              cmp al,0x4b
000025B7  75E3              jnz 0x259c
000025B9  FE4EFA            dec byte [bp-0x6]
000025BC  EB20              jmp 0x25de
000025BE  3C52              cmp al,0x52
000025C0  750B              jnz 0x25cd
000025C2  F656F9            not byte [bp-0x7]
000025C5  EB17              jmp 0x25de
000025C7  3C0D              cmp al,0xd
000025C9  750F              jnz 0x25da
000025CB  EB52              jmp 0x261f
000025CD  3CEF              cmp al,0xef
000025CF  744E              jz 0x261f
000025D1  3CEE              cmp al,0xee
000025D3  7535              jnz 0x260a
000025D5  B01B              mov al,0x1b
000025D7  A35BBD            mov [0xbd5b],ax
000025DA  3C1B              cmp al,0x1b
000025DC  744A              jz 0x2628
000025DE  8A46FC            mov al,[bp-0x4]
000025E1  3A46A9            cmp al,[bp-0x57]
000025E4  7703              ja 0x25e9
000025E6  8846A9            mov [bp-0x57],al
000025E9  8A46FA            mov al,[bp-0x6]
000025EC  0AC0              or al,al
000025EE  7502              jnz 0x25f2
000025F0  B001              mov al,0x1
000025F2  8A5EA9            mov bl,[bp-0x57]
000025F5  43                inc bx
000025F6  3AC3              cmp al,bl
000025F8  7E02              jng 0x25fc
000025FA  8AC3              mov al,bl
000025FC  3A46FC            cmp al,[bp-0x4]
000025FF  7E03              jng 0x2604
00002601  8A46FC            mov al,[bp-0x4]
00002604  8846FA            mov [bp-0x6],al
00002607  E9E9FE            jmp 0x24f3
0000260A  3C47              cmp al,0x47
0000260C  7504              jnz 0x2612
0000260E  B001              mov al,0x1
00002610  EB08              jmp 0x261a
00002612  3C4F              cmp al,0x4f
00002614  75C8              jnz 0x25de
00002616  8A46A9            mov al,[bp-0x57]
00002619  40                inc ax
0000261A  8846FA            mov [bp-0x6],al
0000261D  EBBF              jmp 0x25de
0000261F  8B7EFE            mov di,[bp-0x2]
00002622  8D76A9            lea si,[bp-0x57]
00002625  E8FAE8            call 0xf22
00002628  E8D5E3            call 0xa00
0000262B  E90CE8            jmp 0xe3a
0000262E  B201              mov dl,0x1
00002630  32ED              xor ch,ch
00002632  E321              jcxz 0x2655
00002634  8BFE              mov di,si
00002636  AC                lodsb
00002637  3AC1              cmp al,cl
00002639  7C1A              jl 0x2655
0000263B  49                dec cx
0000263C  8AE0              mov ah,al
0000263E  2AE1              sub ah,cl
00002640  3AE2              cmp ah,dl
00002642  7D02              jnl 0x2646
00002644  8AD4              mov dl,ah
00002646  2AC2              sub al,dl
00002648  AA                stosb
00002649  2AE2              sub ah,dl
0000264B  F3A4              rep movsb
0000264D  32F6              xor dh,dh
0000264F  03F2              add si,dx
00002651  8ACC              mov cl,ah
00002653  F3A4              rep movsb
00002655  C3                ret
00002656  55                push bp
00002657  8BEC              mov bp,sp
00002659  83EC60            sub sp,0x60
0000265C  8A4604            mov al,[bp+0x4]
0000265F  0AC0              or al,al
00002661  755E              jnz 0x26c1
00002663  A159BD            mov ax,[0xbd59]
00002666  0BC0              or ax,ax
00002668  740B              jz 0x2675
0000266A  A1C8BC            mov ax,[0xbcc8]
0000266D  E8C1FB            call 0x2231
00002670  BF1E00            mov di,0x1e
00002673  EB05              jmp 0x267a
00002675  BF9EBB            mov di,0xbb9e
00002678  1E                push ds
00002679  07                pop es
0000267A  E8BEEA            call 0x113b
0000267D  8D76FB            lea si,[bp-0x5]
00002680  56                push si
00002681  B101              mov cl,0x1
00002683  E8A8FF            call 0x262e
00002686  5F                pop di
00002687  8BF7              mov si,di
00002689  46                inc si
0000268A  E850F0            call 0x16dd
0000268D  33C0              xor ax,ax
0000268F  40                inc ax
00002690  3C19              cmp al,0x19
00002692  7F3D              jg 0x26d1
00002694  50                push ax
00002695  8D7EFB            lea di,[bp-0x5]
00002698  16                push ss
00002699  57                push di
0000269A  BFE6A3            mov di,0xa3e6
0000269D  D1E0              shl ax,0x0
0000269F  D1E0              shl ax,0x0
000026A1  03F8              add di,ax
000026A3  1E                push ds
000026A4  57                push di
000026A5  E898FB            call 0x2240
000026A8  58                pop ax
000026A9  75E4              jnz 0x268f
000026AB  0AC0              or al,al
000026AD  7412              jz 0x26c1
000026AF  A159BD            mov ax,[0xbd59]
000026B2  0BC0              or ax,ax
000026B4  741B              jz 0x26d1
000026B6  C47E06            les di,word [bp+0x6]
000026B9  06                push es
000026BA  57                push di
000026BB  E8ADEB            call 0x126b
000026BE  E9EE00            jmp 0x27af
000026C1  98                cbw
000026C2  BEE6A3            mov si,0xa3e6
000026C5  D1E0              shl ax,0x0
000026C7  D1E0              shl ax,0x0
000026C9  03F0              add si,ax
000026CB  8D7EFB            lea di,[bp-0x5]
000026CE  E85BE8            call 0xf2c
000026D1  BFC7B0            mov di,0xb0c7
000026D4  1E                push ds
000026D5  57                push di
000026D6  8D7EAB            lea di,[bp-0x55]
000026D9  16                push ss
000026DA  57                push di
000026DB  E814F1            call 0x17f2
000026DE  7303              jnc 0x26e3
000026E0  E9BA00            jmp 0x279d
000026E3  8D76AB            lea si,[bp-0x55]
000026E6  33C9              xor cx,cx
000026E8  E8C3E8            call 0xfae
000026EB  8706B4B1          xchg ax,[0xb1b4]
000026EF  50                push ax
000026F0  B9FE00            mov cx,0xfe
000026F3  E8C8E8            call 0xfbe
000026F6  8BD8              mov bx,ax
000026F8  33C9              xor cx,cx
000026FA  8D7EAC            lea di,[bp-0x54]
000026FD  33D2              xor dx,dx
000026FF  16                push ss
00002700  07                pop es
00002701  0BDB              or bx,bx
00002703  741D              jz 0x2722
00002705  3AD9              cmp bl,cl
00002707  720A              jc 0x2713
00002709  AC                lodsb
0000270A  3C0D              cmp al,0xd
0000270C  7405              jz 0x2713
0000270E  41                inc cx
0000270F  42                inc dx
00002710  AA                stosb
00002711  EBF2              jmp 0x2705
00002713  3AD9              cmp bl,cl
00002715  770B              ja 0x2722
00002717  52                push dx
00002718  B9FE00            mov cx,0xfe
0000271B  E8A0E8            call 0xfbe
0000271E  93                xchg ax,bx
0000271F  5A                pop dx
00002720  EBDD              jmp 0x26ff
00002722  AC                lodsb
00002723  41                inc cx
00002724  3C0A              cmp al,0xa
00002726  74FA              jz 0x2722
00002728  49                dec cx
00002729  4E                dec si
0000272A  8856AB            mov [bp-0x55],dl
0000272D  807EAC2B          cmp byte [bp-0x54],0x2b
00002731  7505              jnz 0x2738
00002733  C606BEB901        mov byte [0xb9be],0x1
00002738  51                push cx
00002739  56                push si
0000273A  53                push bx
0000273B  8D7EA0            lea di,[bp-0x60]
0000273E  8D76AC            lea si,[bp-0x54]
00002741  16                push ss
00002742  57                push di
00002743  E897EF            call 0x16dd
00002746  8D7EFB            lea di,[bp-0x5]
00002749  16                push ss
0000274A  57                push di
0000274B  E8F2FA            call 0x2240
0000274E  5B                pop bx
0000274F  5E                pop si
00002750  59                pop cx
00002751  7405              jz 0x2758
00002753  0BDB              or bx,bx
00002755  75A3              jnz 0x26fa
00002757  4B                dec bx
00002758  5B                pop bx
00002759  9C                pushf
0000275A  871EB4B1          xchg bx,[0xb1b4]
0000275E  E8CBE8            call 0x102c
00002761  9D                popf
00002762  7415              jz 0x2779
00002764  8D7EA0            lea di,[bp-0x60]
00002767  16                push ss
00002768  57                push di
00002769  BFE6A3            mov di,0xa3e6
0000276C  1E                push ds
0000276D  57                push di
0000276E  E8CFFA            call 0x2240
00002771  752A              jnz 0x279d
00002773  807E0400          cmp byte [bp+0x4],0x0
00002777  7524              jnz 0x279d
00002779  8D7EAC            lea di,[bp-0x54]
0000277C  33C9              xor cx,cx
0000277E  41                inc cx
0000277F  36803D3A          cmp byte [ss:di],0x3a
00002783  7403              jz 0x2788
00002785  47                inc di
00002786  EBF6              jmp 0x277e
00002788  47                inc di
00002789  41                inc cx
0000278A  36803D20          cmp byte [ss:di],0x20
0000278E  74F8              jz 0x2788
00002790  C47E06            les di,word [bp+0x6]
00002793  8D76AB            lea si,[bp-0x55]
00002796  B24F              mov dl,0x4f
00002798  E8B7EA            call 0x1252
0000279B  EB12              jmp 0x27af
0000279D  8A4604            mov al,[bp+0x4]
000027A0  B20D              mov dl,0xd
000027A2  F6E2              mul dl
000027A4  BE26AF            mov si,0xaf26
000027A7  03F0              add si,ax
000027A9  C47E06            les di,word [bp+0x6]
000027AC  E873E7            call 0xf22
000027AF  E9E8F7            jmp 0x1f9a
000027B2  55                push bp
000027B3  8BEC              mov bp,sp
000027B5  83EC50            sub sp,0x50
000027B8  8D7EB0            lea di,[bp-0x50]
000027BB  57                push di
000027BC  AC                lodsb
000027BD  AA                stosb
000027BE  98                cbw
000027BF  8BD0              mov dx,ax
000027C1  91                xchg ax,cx
000027C2  F3A4              rep movsb
000027C4  8BCA              mov cx,dx
000027C6  FD                std
000027C7  8BF7              mov si,di
000027C9  4E                dec si
000027CA  AC                lodsb
000027CB  3C5C              cmp al,0x5c
000027CD  7402              jz 0x27d1
000027CF  E2F9              loop 0x27ca
000027D1  FC                cld
000027D2  AC                lodsb
000027D3  5E                pop si
000027D4  E315              jcxz 0x27eb
000027D6  3C3A              cmp al,0x3a
000027D8  7411              jz 0x27eb
000027DA  49                dec cx
000027DB  880C              mov [si],cl
000027DD  E86EE8            call 0x104e
000027E0  720C              jc 0x27ee
000027E2  F7C11000          test cx,0x10
000027E6  7503              jnz 0x27eb
000027E8  F9                stc
000027E9  EB15              jmp 0x2800
000027EB  F8                clc
000027EC  EB12              jmp 0x2800
000027EE  8D76B0            lea si,[bp-0x50]
000027F1  56                push si
000027F2  E861E8            call 0x1056
000027F5  7309              jnc 0x2800
000027F7  E8B8FF            call 0x27b2
000027FA  7204              jc 0x2800
000027FC  5E                pop si
000027FD  E856E8            call 0x1056
00002800  E937E6            jmp 0xe3a
00002803  8B168AAF          mov dx,[0xaf8a]
00002807  FECE              dec dh
00002809  8A3E71AF          mov bh,[0xaf71]
0000280D  B309              mov bl,0x9
0000280F  BE65AA            mov si,0xaa65
00002812  E80CE5            call 0xd21
00002815  E8B1DF            call 0x7c9
00002818  C3                ret
00002819  3C02              cmp al,0x2
0000281B  7504              jnz 0x2821
0000281D  BFE8A6            mov di,0xa6e8
00002820  C3                ret
00002821  3C01              cmp al,0x1
00002823  7504              jnz 0x2829
00002825  BFF5A6            mov di,0xa6f5
00002828  C3                ret
00002829  3C03              cmp al,0x3
0000282B  7504              jnz 0x2831
0000282D  BF03A7            mov di,0xa703
00002830  C3                ret
00002831  3C05              cmp al,0x5
00002833  7504              jnz 0x2839
00002835  BFC4A6            mov di,0xa6c4
00002838  C3                ret
00002839  3C07              cmp al,0x7
0000283B  7504              jnz 0x2841
0000283D  BF2DA9            mov di,0xa92d
00002840  C3                ret
00002841  BFD3A6            mov di,0xa6d3
00002844  C3                ret
00002845  55                push bp
00002846  8BEC              mov bp,sp
00002848  81EC8001          sub sp,0x180
0000284C  E8B0F0            call 0x18ff
0000284F  7305              jnc 0x2856
00002851  BF69A7            mov di,0xa769
00002854  EB6E              jmp 0x28c4
00002856  A0A0BD            mov al,[0xbda0]
00002859  3C08              cmp al,0x8
0000285B  7C18              jl 0x2875
0000285D  3C0A              cmp al,0xa
0000285F  7C08              jl 0x2869
00002861  3C0B              cmp al,0xb
00002863  7404              jz 0x2869
00002865  3C16              cmp al,0x16
00002867  750C              jnz 0x2875
00002869  BFBEA7            mov di,0xa7be
0000286C  BEEAA7            mov si,0xa7ea
0000286F  E8F2DC            call 0x564
00002872  E97A03            jmp 0x2bef
00002875  32C0              xor al,al
00002877  A23EBC            mov [0xbc3e],al
0000287A  40                inc ax
0000287B  A29BBD            mov [0xbd9b],al
0000287E  BF9CBA            mov di,0xba9c
00002881  1E                push ds
00002882  57                push di
00002883  A0A0BD            mov al,[0xbda0]
00002886  BEA3B0            mov si,0xb0a3
00002889  3C05              cmp al,0x5
0000288B  7422              jz 0x28af
0000288D  BEACB0            mov si,0xb0ac
00002890  3C06              cmp al,0x6
00002892  741B              jz 0x28af
00002894  3C02              cmp al,0x2
00002896  7617              jna 0x28af
00002898  BEB2B0            mov si,0xb0b2
0000289B  3C15              cmp al,0x15
0000289D  7410              jz 0x28af
0000289F  3C0A              cmp al,0xa
000028A1  7409              jz 0x28ac
000028A3  3C0E              cmp al,0xe
000028A5  7C08              jl 0x28af
000028A7  BEBBB0            mov si,0xb0bb
000028AA  EB03              jmp 0x28af
000028AC  BEB6B0            mov si,0xb0b6
000028AF  E86EE6            call 0xf20
000028B2  E98F00            jmp 0x2944
000028B5  55                push bp
000028B6  8BEC              mov bp,sp
000028B8  81EC8001          sub sp,0x180
000028BC  E840F0            call 0x18ff
000028BF  7308              jnc 0x28c9
000028C1  BF5BA7            mov di,0xa75b
000028C4  BE74A7            mov si,0xa774
000028C7  EB39              jmp 0x2902
000028C9  33C0              xor ax,ax
000028CB  A29BBD            mov [0xbd9b],al
000028CE  A23EBC            mov [0xbc3e],al
000028D1  803E5CBD40        cmp byte [0xbd5c],0x40
000028D6  7430              jz 0x2908
000028D8  BF57A9            mov di,0xa957
000028DB  E8D6E3            call 0xcb4
000028DE  BA1400            mov dx,0x14
000028E1  BF83A9            mov di,0xa983
000028E4  E8C4DF            call 0x8ab
000028E7  BF3EBC            mov di,0xbc3e
000028EA  E8CAFB            call 0x24b7
000028ED  A15BBD            mov ax,[0xbd5b]
000028F0  3C1B              cmp al,0x1b
000028F2  7503              jnz 0x28f7
000028F4  E9F802            jmp 0x2bef
000028F7  E8DFE4            call 0xdd9
000028FA  7209              jc 0x2905
000028FC  BFA1A9            mov di,0xa9a1
000028FF  BE3EBC            mov si,0xbc3e
00002902  E99402            jmp 0x2b99
00002905  E8A7DB            call 0x4af
00002908  BF9CBA            mov di,0xba9c
0000290B  1E                push ds
0000290C  57                push di
0000290D  E847EC            call 0x1557
00002910  BE3EBC            mov si,0xbc3e
00002913  AC                lodsb
00002914  0AC0              or al,al
00002916  742C              jz 0x2944
00002918  98                cbw
00002919  8BD8              mov bx,ax
0000291B  8BD6              mov dx,si
0000291D  8078FF5C          cmp byte [bx+si-0x1],0x5c
00002921  750C              jnz 0x292f
00002923  80FB03            cmp bl,0x3
00002926  7506              jnz 0x292e
00002928  807C013A          cmp byte [si+0x1],0x3a
0000292C  7401              jz 0x292f
0000292E  4B                dec bx
0000292F  C60000            mov byte [bx+si],0x0
00002932  B43B              mov ah,0x3b
00002934  CD21              int byte 0x21
00002936  807C013A          cmp byte [si+0x1],0x3a
0000293A  7508              jnz 0x2944
0000293C  AC                lodsb
0000293D  2C41              sub al,0x41
0000293F  B60E              mov dh,0xe
00002941  92                xchg ax,dx
00002942  CD21              int byte 0x21
00002944  A0A0BD            mov al,[0xbda0]
00002947  A2E5A3            mov [0xa3e5],al
0000294A  3C18              cmp al,0x18
0000294C  7206              jc 0x2954
0000294E  BE63B1            mov si,0xb163
00002951  E8E5E5            call 0xf39
00002954  803EC2B900        cmp byte [0xb9c2],0x0
00002959  7503              jnz 0x295e
0000295B  E96E01            jmp 0x2acc
0000295E  E8A2FE            call 0x2803
00002961  A1C8BC            mov ax,[0xbcc8]
00002964  E8CAF8            call 0x2231
00002967  E89AE1            call 0xb04
0000296A  E8A003            call 0x2d0d
0000296D  3D0100            cmp ax,0x1
00002970  7204              jc 0x2976
00002972  741D              jz 0x2991
00002974  EBE5              jmp 0x295b
00002976  833EBEBC00        cmp word [0xbcbe],0x0
0000297B  7514              jnz 0x2991
0000297D  A159BD            mov ax,[0xbd59]
00002980  A35FBD            mov [0xbd5f],ax
00002983  50                push ax
00002984  A09BBD            mov al,[0xbd9b]
00002987  50                push ax
00002988  1E                push ds
00002989  07                pop es
0000298A  BFCABC            mov di,0xbcca
0000298D  8A05              mov al,[di]
0000298F  EB6A              jmp 0x29fb
00002991  A159BD            mov ax,[0xbd59]
00002994  8BC8              mov cx,ax
00002996  8BC1              mov ax,cx
00002998  E896F8            call 0x2231
0000299B  26F7050100        test word [es:di],0x1
000029A0  7506              jnz 0x29a8
000029A2  E2F2              loop 0x2996
000029A4  8B0EC8BC          mov cx,[0xbcc8]
000029A8  890E5FBD          mov [0xbd5f],cx
000029AC  33C0              xor ax,ax
000029AE  A35DBD            mov [0xbd5d],ax
000029B1  3B065FBD          cmp ax,[0xbd5f]
000029B5  7C03              jl 0x29ba
000029B7  E9DA00            jmp 0x2a94
000029BA  50                push ax
000029BB  E874F8            call 0x2232
000029BE  A09BBD            mov al,[0xbd9b]
000029C1  50                push ax
000029C2  268A05            mov al,[es:di]
000029C5  803EA0BD13        cmp byte [0xbda0],0x13
000029CA  7407              jz 0x29d3
000029CC  803EA0BD18        cmp byte [0xbda0],0x18
000029D1  7224              jc 0x29f7
000029D3  F606A1BD08        test byte [0xbda1],0x8
000029D8  741D              jz 0x29f7
000029DA  A801              test al,0x1
000029DC  751D              jnz 0x29fb
000029DE  833EBEBC00        cmp word [0xbcbe],0x0
000029E3  750B              jnz 0x29f0
000029E5  5B                pop bx
000029E6  59                pop cx
000029E7  51                push cx
000029E8  53                push bx
000029E9  41                inc cx
000029EA  3B0EC8BC          cmp cx,[0xbcc8]
000029EE  740F              jz 0x29ff
000029F0  C6069BBD01        mov byte [0xbd9b],0x1
000029F5  EB04              jmp 0x29fb
000029F7  A801              test al,0x1
000029F9  7460              jz 0x2a5b
000029FB  A818              test al,0x18
000029FD  7558              jnz 0x2a57
000029FF  268B450F          mov ax,[es:di+0xf]
00002A03  A3AFBD            mov [0xbdaf],ax
00002A06  268B4511          mov ax,[es:di+0x11]
00002A0A  A3B1BD            mov [0xbdb1],ax
00002A0D  E8F4E0            call 0xb04
00002A10  0AC0              or al,al
00002A12  7447              jz 0x2a5b
00002A14  800EE1BC80        or byte [0xbce1],0x80
00002A19  BEE8BC            mov si,0xbce8
00002A1C  8D7E80            lea di,[bp-0x80]
00002A1F  36C60500          mov byte [ss:di],0x0
00002A23  16                push ss
00002A24  57                push di
00002A25  8DBE30FF          lea di,[bp-0xd0]
00002A29  16                push ss
00002A2A  57                push di
00002A2B  1E                push ds
00002A2C  56                push si
00002A2D  E867E7            call 0x1197
00002A30  EB32              jmp 0x2a64
00002A32  5E                pop si
00002A33  07                pop es
00002A34  06                push es
00002A35  56                push si
00002A36  E82F28            call 0x5268
00002A39  BFDFA6            mov di,0xa6df
00002A3C  0AC0              or al,al
00002A3E  750B              jnz 0x2a4b
00002A40  8B16E4A3          mov dx,[0xa3e4]
00002A44  32F6              xor dh,dh
00002A46  E8C3E5            call 0x100c
00002A49  EB09              jmp 0x2a54
00002A4B  5E                pop si
00002A4C  07                pop es
00002A4D  3C0C              cmp al,0xc
00002A4F  7406              jz 0x2a57
00002A51  E8C5FD            call 0x2819
00002A54  E82EDE            call 0x885
00002A57  FF065DBD          inc word [0xbd5d]
00002A5B  58                pop ax
00002A5C  A29BBD            mov [0xbd9b],al
00002A5F  58                pop ax
00002A60  40                inc ax
00002A61  E94DFF            jmp 0x29b1
00002A64  E896F3            call 0x1dfd
00002A67  BF1BA6            mov di,0xa61b
00002A6A  803E9BBD00        cmp byte [0xbd9b],0x0
00002A6F  7403              jz 0x2a74
00002A71  BF2CA6            mov di,0xa62c
00002A74  E809DE            call 0x880
00002A77  5F                pop di
00002A78  57                push di
00002A79  16                push ss
00002A7A  07                pop es
00002A7B  E832DE            call 0x8b0
00002A7E  803E9BBD00        cmp byte [0xbd9b],0x0
00002A83  75AD              jnz 0x2a32
00002A85  8D7680            lea si,[bp-0x80]
00002A88  16                push ss
00002A89  07                pop es
00002A8A  E825FD            call 0x27b2
00002A8D  73A3              jnc 0x2a32
00002A8F  BF3AA6            mov di,0xa63a
00002A92  EBC0              jmp 0x2a54
00002A94  A0A0BD            mov al,[0xbda0]
00002A97  3C13              cmp al,0x13
00002A99  7404              jz 0x2a9f
00002A9B  3C18              cmp al,0x18
00002A9D  7206              jc 0x2aa5
00002A9F  E85FE0            call 0xb01
00002AA2  E8FA67            call 0x929f
00002AA5  BF6FAF            mov di,0xaf6f
00002AA8  E8D5DD            call 0x880
00002AAB  A15DBD            mov ax,[0xbd5d]
00002AAE  3B06BEBC          cmp ax,[0xbcbe]
00002AB2  7C18              jl 0x2acc
00002AB4  33C0              xor ax,ax
00002AB6  3B0659BD          cmp ax,[0xbd59]
00002ABA  7D0D              jnl 0x2ac9
00002ABC  50                push ax
00002ABD  E872F7            call 0x2232
00002AC0  268065177F        and byte [es:di+0x17],0x7f
00002AC5  58                pop ax
00002AC6  40                inc ax
00002AC7  EBED              jmp 0x2ab6
00002AC9  E90D01            jmp 0x2bd9
00002ACC  8D7EB0            lea di,[bp-0x50]
00002ACF  A0A0BD            mov al,[0xbda0]
00002AD2  16                push ss
00002AD3  57                push di
00002AD4  50                push ax
00002AD5  E87EFB            call 0x2656
00002AD8  BF1DBB            mov di,0xbb1d
00002ADB  1E                push ds
00002ADC  57                push di
00002ADD  E812ED            call 0x17f2
00002AE0  7303              jnc 0x2ae5
00002AE2  E98200            jmp 0x2b67
00002AE5  803EBEB900        cmp byte [0xb9be],0x0
00002AEA  7503              jnz 0x2aef
00002AEC  E8D1DC            call 0x7c0
00002AEF  E816DF            call 0xa08
00002AF2  BF9CBA            mov di,0xba9c
00002AF5  1E                push ds
00002AF6  57                push di
00002AF7  8D7EB0            lea di,[bp-0x50]
00002AFA  E8B0ED            call 0x18ad
00002AFD  E8FDF2            call 0x1dfd
00002B00  A0A0BD            mov al,[0xbda0]
00002B03  3C13              cmp al,0x13
00002B05  7510              jnz 0x2b17
00002B07  803E9BBD00        cmp byte [0xbd9b],0x0
00002B0C  7413              jz 0x2b21
00002B0E  8D7EB0            lea di,[bp-0x50]
00002B11  36C60500          mov byte [ss:di],0x0
00002B15  EB48              jmp 0x2b5f
00002B17  3C06              cmp al,0x6
00002B19  7506              jnz 0x2b21
00002B1B  BFDDB0            mov di,0xb0dd
00002B1E  E8D8F2            call 0x1df9
00002B21  A1BEBC            mov ax,[0xbcbe]
00002B24  0BC0              or ax,ax
00002B26  7417              jz 0x2b3f
00002B28  3B0659BD          cmp ax,[0xbd59]
00002B2C  7541              jnz 0x2b6f
00002B2E  A0A0BD            mov al,[0xbda0]
00002B31  3C0A              cmp al,0xa
00002B33  7404              jz 0x2b39
00002B35  3C16              cmp al,0x16
00002B37  751C              jnz 0x2b55
00002B39  BE73B0            mov si,0xb073
00002B3C  1E                push ds
00002B3D  EB0C              jmp 0x2b4b
00002B3F  A1C8BC            mov ax,[0xbcc8]
00002B42  E8ECF6            call 0x2231
00002B45  BE1E00            mov si,0x1e
00002B48  1E                push ds
00002B49  06                push es
00002B4A  1F                pop ds
00002B4B  8D7EB0            lea di,[bp-0x50]
00002B4E  E8DBE3            call 0xf2c
00002B51  1F                pop ds
00002B52  A0A0BD            mov al,[0xbda0]
00002B55  3C02              cmp al,0x2
00002B57  7506              jnz 0x2b5f
00002B59  BE66B0            mov si,0xb066
00002B5C  E8DAE3            call 0xf39
00002B5F  E8CEE3            call 0xf30
00002B62  8D7EB0            lea di,[bp-0x50]
00002B65  EB6A              jmp 0x2bd1
00002B67  BF1DBB            mov di,0xbb1d
00002B6A  BE8EA7            mov si,0xa78e
00002B6D  EB2A              jmp 0x2b99
00002B6F  A0A0BD            mov al,[0xbda0]
00002B72  3C09              cmp al,0x9
00002B74  7210              jc 0x2b86
00002B76  3C17              cmp al,0x17
00002B78  7440              jz 0x2bba
00002B7A  3C16              cmp al,0x16
00002B7C  743C              jz 0x2bba
00002B7E  3C13              cmp al,0x13
00002B80  7304              jnc 0x2b86
00002B82  3C0B              cmp al,0xb
00002B84  7534              jnz 0x2bba
00002B86  8D7EB0            lea di,[bp-0x50]
00002B89  16                push ss
00002B8A  57                push di
00002B8B  33C0              xor ax,ax
00002B8D  50                push ax
00002B8E  E8ECF0            call 0x1c7d
00002B91  730B              jnc 0x2b9e
00002B93  BFFEA8            mov di,0xa8fe
00002B96  BE0CA9            mov si,0xa90c
00002B99  E8C8D9            call 0x564
00002B9C  EB51              jmp 0x2bef
00002B9E  E88FE3            call 0xf30
00002BA1  BF98AF            mov di,0xaf98
00002BA4  E852F2            call 0x1df9
00002BA7  8D7EB0            lea di,[bp-0x50]
00002BAA  16                push ss
00002BAB  57                push di
00002BAC  E84EF2            call 0x1dfd
00002BAF  E898F7            call 0x234a
00002BB2  8D76B0            lea si,[bp-0x50]
00002BB5  E846E4            call 0xffe
00002BB8  EB1F              jmp 0x2bd9
00002BBA  E873E3            call 0xf30
00002BBD  33C0              xor ax,ax
00002BBF  884680            mov [bp-0x80],al
00002BC2  3B0659BD          cmp ax,[0xbd59]
00002BC6  723F              jc 0x2c07
00002BC8  807E8000          cmp byte [bp-0x80],0x0
00002BCC  7421              jz 0x2bef
00002BCE  8D7E80            lea di,[bp-0x80]
00002BD1  16                push ss
00002BD2  57                push di
00002BD3  E827F2            call 0x1dfd
00002BD6  E871F7            call 0x234a
00002BD9  803EA0BD13        cmp byte [0xbda0],0x13
00002BDE  7503              jnz 0x2be3
00002BE0  E82DDC            call 0x810
00002BE3  E895DC            call 0x87b
00002BE6  E827DC            call 0x810
00002BE9  E814DE            call 0xa00
00002BEC  E8DBD9            call 0x5ca
00002BEF  8A1669B1          mov dl,[0xb169]
00002BF3  80EA41            sub dl,0x41
00002BF6  B40E              mov ah,0xe
00002BF8  CD21              int byte 0x21
00002BFA  BA6CB1            mov dx,0xb16c
00002BFD  B43B              mov ah,0x3b
00002BFF  CD21              int byte 0x21
00002C01  E8FCDD            call 0xa00
00002C04  E98F04            jmp 0x3096
00002C07  50                push ax
00002C08  E827F6            call 0x2232
00002C0B  268A05            mov al,[es:di]
00002C0E  D0E8              shr al,0x0
00002C10  7204              jc 0x2c16
00002C12  58                pop ax
00002C13  40                inc ax
00002C14  EBAC              jmp 0x2bc2
00002C16  BF1E00            mov di,0x1e
00002C19  8A4680            mov al,[bp-0x80]
00002C1C  02069CBA          add al,[0xba9c]
00002C20  260205            add al,[es:di]
00002C23  7215              jc 0x2c3a
00002C25  3C7F              cmp al,0x7f
00002C27  7311              jnc 0x2c3a
00002C29  8D7680            lea si,[bp-0x80]
00002C2C  16                push ss
00002C2D  56                push si
00002C2E  06                push es
00002C2F  57                push di
00002C30  E8CAF1            call 0x1dfd
00002C33  E8C0F1            call 0x1df6
00002C36  58                pop ax
00002C37  58                pop ax
00002C38  EBD8              jmp 0x2c12
00002C3A  8DBE80FE          lea di,[bp-0x180]
00002C3E  16                push ss
00002C3F  57                push di
00002C40  E8E6E2            call 0xf29
00002C43  8D7E80            lea di,[bp-0x80]
00002C46  16                push ss
00002C47  57                push di
00002C48  E8B2F1            call 0x1dfd
00002C4B  E8FCF6            call 0x234a
00002C4E  FE06CAB9          inc byte [0xb9ca]
00002C52  C6468000          mov byte [bp-0x80],0x0
00002C56  58                pop ax
00002C57  EBAE              jmp 0x2c07
00002C59  A0ACB1            mov al,[0xb1ac]
00002C5C  D0E8              shr al,0x0
00002C5E  B126              mov cl,0x26
00002C60  2C0C              sub al,0xc
00002C62  8AE0              mov ah,al
00002C64  80C417            add ah,0x17
00002C67  93                xchg ax,bx
00002C68  A0D8A3            mov al,[0xa3d8]
00002C6B  BFAEA9            mov di,0xa9ae
00002C6E  E84BE0            call 0xcbc
00002C71  BA0B00            mov dx,0xb
00002C74  BF8BAA            mov di,0xaa8b
00002C77  E831DC            call 0x8ab
00002C7A  803E7EBD00        cmp byte [0xbd7e],0x0
00002C7F  7408              jz 0x2c89
00002C81  BFAAAA            mov di,0xaaaa
00002C84  E8FEDB            call 0x885
00002C87  EB2D              jmp 0x2cb6
00002C89  BFB8AA            mov di,0xaab8
00002C8C  E8F6DB            call 0x885
00002C8F  BF81A4            mov di,0xa481
00002C92  B90B00            mov cx,0xb
00002C95  1E                push ds
00002C96  07                pop es
00002C97  E81FDC            call 0x8b9
00002C9A  BFEDB0            mov di,0xb0ed
00002C9D  E8E5DB            call 0x885
00002CA0  BA1D01            mov dx,0x11d
00002CA3  A04EA4            mov al,[0xa44e]
00002CA6  D0E8              shr al,0x0
00002CA8  2AD0              sub dl,al
00002CAA  BFBFAA            mov di,0xaabf
00002CAD  E8FBDB            call 0x8ab
00002CB0  BF4EA4            mov di,0xa44e
00002CB3  E8CFDB            call 0x885
00002CB6  BA2602            mov dx,0x226
00002CB9  BFBBA5            mov di,0xa5bb
00002CBC  8ADE              mov bl,dh
00002CBE  D1E3              shl bx,0x0
00002CC0  8B39              mov di,[bx+di]
00002CC2  52                push dx
00002CC3  E8E5DB            call 0x8ab
00002CC6  5A                pop dx
00002CC7  FEC6              inc dh
00002CC9  80FE13            cmp dh,0x13
00002CCC  7CEB              jl 0x2cb9
00002CCE  BA0002            mov dx,0x200
00002CD1  BFE2AB            mov di,0xabe2
00002CD4  E8D4DB            call 0x8ab
00002CD7  B83000            mov ax,0x30
00002CDA  40                inc ax
00002CDB  3C36              cmp al,0x36
00002CDD  7D19              jnl 0x2cf8
00002CDF  50                push ax
00002CE0  BFE8AB            mov di,0xabe8
00002CE3  884507            mov [di+0x7],al
00002CE6  E897DB            call 0x880
00002CE9  BF53A5            mov di,0xa553
00002CEC  5B                pop bx
00002CED  53                push bx
00002CEE  D1E3              shl bx,0x0
00002CF0  8B39              mov di,[bx+di]
00002CF2  E890DB            call 0x885
00002CF5  58                pop ax
00002CF6  EBE2              jmp 0x2cda
00002CF8  BF23AC            mov di,0xac23
00002CFB  E882DB            call 0x880
00002CFE  BF0BAD            mov di,0xad0b
00002D01  E87CDB            call 0x880
00002D04  E8C3D8            call 0x5ca
00002D07  E8DAD9            call 0x6e4
00002D0A  33C0              xor ax,ax
00002D0C  C3                ret
00002D0D  A0A0BD            mov al,[0xbda0]
00002D10  3C13              cmp al,0x13
00002D12  7414              jz 0x2d28
00002D14  3C18              cmp al,0x18
00002D16  7265              jc 0x2d7d
00002D18  3C19              cmp al,0x19
00002D1A  740C              jz 0x2d28
00002D1C  E8D7DD            call 0xaf6
00002D1F  734B              jnc 0x2d6c
00002D21  E8DDDD            call 0xb01
00002D24  B80200            mov ax,0x2
00002D27  C3                ret
00002D28  BBFFFF            mov bx,0xffff
00002D2B  E80DE3            call 0x103b
00002D2E  2B1EC4B9          sub bx,[0xb9c4]
00002D32  81EB5122          sub bx,0x2251
00002D36  72E9              jc 0x2d21
00002D38  B10C              mov cl,0xc
00002D3A  D3EB              shr bx,cl
00002D3C  891E9DBD          mov [0xbd9d],bx
00002D40  3B1EBCBD          cmp bx,[0xbdbc]
00002D44  7315              jnc 0x2d5b
00002D46  E87365            call 0x92bc
00002D49  72D6              jc 0x2d21
00002D4B  E86065            call 0x92ae
00002D4E  8B16BCBD          mov dx,[0xbdbc]
00002D52  42                inc dx
00002D53  B106              mov cl,0x6
00002D55  D3E2              shl dx,cl
00002D57  42                inc dx
00002D58  E83365            call 0x928e
00002D5B  803EE7BC14        cmp byte [0xbce7],0x14
00002D60  720A              jc 0x2d6c
00002D62  33C0              xor ax,ax
00002D64  39068DBD          cmp [0xbd8d],ax
00002D68  7413              jz 0x2d7d
00002D6A  EB0D              jmp 0x2d79
00002D6C  F606A1BD08        test byte [0xbda1],0x8
00002D71  740A              jz 0x2d7d
00002D73  C7068DBD0000      mov word [0xbd8d],0x0
00002D79  B80100            mov ax,0x1
00002D7C  C3                ret
00002D7D  33C0              xor ax,ax
00002D7F  C3                ret
00002D80  55                push bp
00002D81  8BEC              mov bp,sp
00002D83  81ECE401          sub sp,0x1e4
00002D87  33C0              xor ax,ax
00002D89  A29BBD            mov [0xbd9b],al
00002D8C  A2E5A3            mov [0xa3e5],al
00002D8F  A1C8BC            mov ax,[0xbcc8]
00002D92  E89CF4            call 0x2231
00002D95  A0A0BD            mov al,[0xbda0]
00002D98  B401              mov ah,0x1
00002D9A  8C46A0            mov word [bp-0x60],es
00002D9D  89469E            mov [bp-0x62],ax
00002DA0  268A1D            mov bl,[es:di]
00002DA3  80E318            and bl,0x18
00002DA6  744F              jz 0x2df7
00002DA8  3C0A              cmp al,0xa
00002DAA  744B              jz 0x2df7
00002DAC  BE74A7            mov si,0xa774
00002DAF  BF50A7            mov di,0xa750
00002DB2  E9AD02            jmp 0x3062
00002DB5  C6069BBD01        mov byte [0xbd9b],0x1
00002DBA  33C0              xor ax,ax
00002DBC  40                inc ax
00002DBD  3B06C8BC          cmp ax,[0xbcc8]
00002DC1  7D23              jnl 0x2de6
00002DC3  50                push ax
00002DC4  E86AF4            call 0x2231
00002DC7  26F60518          test byte [es:di],0x18
00002DCB  7403              jz 0x2dd0
00002DCD  58                pop ax
00002DCE  EBEC              jmp 0x2dbc
00002DD0  C6069BBD01        mov byte [0xbd9b],0x1
00002DD5  E82CDD            call 0xb04
00002DD8  0AC0              or al,al
00002DDA  7439              jz 0x2e15
00002DDC  E88924            call 0x5268
00002DDF  0AC0              or al,al
00002DE1  58                pop ax
00002DE2  74D8              jz 0x2dbc
00002DE4  EB2F              jmp 0x2e15
00002DE6  C6069BBD00        mov byte [0xbd9b],0x0
00002DEB  A1C8BC            mov ax,[0xbcc8]
00002DEE  E840F4            call 0x2231
00002DF1  E810DD            call 0xb04
00002DF4  E98300            jmp 0x2e7a
00002DF7  A0C2B9            mov al,[0xb9c2]
00002DFA  0AC0              or al,al
00002DFC  7403              jz 0x2e01
00002DFE  E803DD            call 0xb04
00002E01  88469C            mov [bp-0x64],al
00002E04  0AC0              or al,al
00002E06  740D              jz 0x2e15
00002E08  E802FF            call 0x2d0d
00002E0B  3D0100            cmp ax,0x1
00002E0E  726A              jc 0x2e7a
00002E10  74A3              jz 0x2db5
00002E12  E92302            jmp 0x3038
00002E15  A0A0BD            mov al,[0xbda0]
00002E18  8D7EA2            lea di,[bp-0x5e]
00002E1B  16                push ss
00002E1C  57                push di
00002E1D  50                push ax
00002E1E  3C06              cmp al,0x6
00002E20  7516              jnz 0x2e38
00002E22  50                push ax
00002E23  16                push ss
00002E24  57                push di
00002E25  BEF0B0            mov si,0xb0f0
00002E28  E833E1            call 0xf5e
00002E2B  5F                pop di
00002E2C  07                pop es
00002E2D  268A05            mov al,[es:di]
00002E30  88469F            mov [bp-0x61],al
00002E33  0AC0              or al,al
00002E35  58                pop ax
00002E36  751C              jnz 0x2e54
00002E38  803EBEB900        cmp byte [0xb9be],0x0
00002E3D  7525              jnz 0x2e64
00002E3F  3C02              cmp al,0x2
00002E41  7611              jna 0x2e54
00002E43  3C09              cmp al,0x9
00002E45  740D              jz 0x2e54
00002E47  3C0C              cmp al,0xc
00002E49  7409              jz 0x2e54
00002E4B  3C0D              cmp al,0xd
00002E4D  7405              jz 0x2e54
00002E4F  E837DA            call 0x889
00002E52  EB10              jmp 0x2e64
00002E54  8B168AAF          mov dx,[0xaf8a]
00002E58  8A2E71AF          mov ch,[0xaf71]
00002E5C  B101              mov cl,0x1
00002E5E  E86FD9            call 0x7d0
00002E61  E85FD9            call 0x7c3
00002E64  E8C7D6            call 0x52e
00002E67  E8ECF7            call 0x2656
00002E6A  BF9CBA            mov di,0xba9c
00002E6D  1E                push ds
00002E6E  57                push di
00002E6F  E880E9            call 0x17f2
00002E72  7306              jnc 0x2e7a
00002E74  BF9CBA            mov di,0xba9c
00002E77  E9E501            jmp 0x305f
00002E7A  A0BFB9            mov al,[0xb9bf]
00002E7D  50                push ax
00002E7E  A069B1            mov al,[0xb169]
00002E81  803ECBB900        cmp byte [0xb9cb],0x0
00002E86  7403              jz 0x2e8b
00002E88  A0CCB9            mov al,[0xb9cc]
00002E8B  2C40              sub al,0x40
00002E8D  E8BDE0            call 0xf4d
00002E90  8E46A0            mov es,word [bp-0x60]
00002E93  26C40E0B00        les cx,word [es:0xb]
00002E98  8CC3              mov bx,es
00002E9A  3BD3              cmp dx,bx
00002E9C  7F13              jg 0x2eb1
00002E9E  7C04              jl 0x2ea4
00002EA0  3BC1              cmp ax,cx
00002EA2  720D              jc 0x2eb1
00002EA4  58                pop ax
00002EA5  A2BFB9            mov [0xb9bf],al
00002EA8  BFFEA8            mov di,0xa8fe
00002EAB  BE0CA9            mov si,0xa90c
00002EAE  E9B101            jmp 0x3062
00002EB1  32C0              xor al,al
00002EB3  A2BFB9            mov [0xb9bf],al
00002EB6  803ECBB900        cmp byte [0xb9cb],0x0
00002EBB  7412              jz 0x2ecf
00002EBD  BACCB9            mov dx,0xb9cc
00002EC0  B43B              mov ah,0x3b
00002EC2  CD21              int byte 0x21
00002EC4  8A16CCB9          mov dl,[0xb9cc]
00002EC8  80EA41            sub dl,0x41
00002ECB  B40E              mov ah,0xe
00002ECD  CD21              int byte 0x21
00002ECF  8D7EA2            lea di,[bp-0x5e]
00002ED2  16                push ss
00002ED3  57                push di
00002ED4  8E46A0            mov es,word [bp-0x60]
00002ED7  BF1E00            mov di,0x1e
00002EDA  06                push es
00002EDB  57                push di
00002EDC  E8B8E2            call 0x1197
00002EDF  E82FE1            call 0x1011
00002EE2  7225              jc 0x2f09
00002EE4  807E9C00          cmp byte [bp-0x64],0x0
00002EE8  7506              jnz 0x2ef0
00002EEA  807E9F00          cmp byte [bp-0x61],0x0
00002EEE  75B4              jnz 0x2ea4
00002EF0  8D7EA2            lea di,[bp-0x5e]
00002EF3  16                push ss
00002EF4  07                pop es
00002EF5  E843E2            call 0x113b
00002EF8  8D7EA2            lea di,[bp-0x5e]
00002EFB  57                push di
00002EFC  8D76F2            lea si,[bp-0xe]
00002EFF  E82AE0            call 0xf2c
00002F02  BEBDAF            mov si,0xafbd
00002F05  5F                pop di
00002F06  E835E0            call 0xf3e
00002F09  58                pop ax
00002F0A  A2BFB9            mov [0xb9bf],al
00002F0D  BE1BBA            mov si,0xba1b
00002F10  E80AE0            call 0xf1d
00002F13  807E9C00          cmp byte [bp-0x64],0x0
00002F17  7469              jz 0x2f82
00002F19  8A46A2            mov al,[bp-0x5e]
00002F1C  D0E8              shr al,0x0
00002F1E  3C0A              cmp al,0xa
00002F20  7D02              jnl 0x2f24
00002F22  B00A              mov al,0xa
00002F24  50                push ax
00002F25  0405              add al,0x5
00002F27  91                xchg ax,cx
00002F28  BFE1A5            mov di,0xa5e1
00002F2B  E888DD            call 0xcb6
00002F2E  58                pop ax
00002F2F  D0E0              shl al,0x0
00002F31  0405              add al,0x5
00002F33  50                push ax
00002F34  2C11              sub al,0x11
00002F36  D0E8              shr al,0x0
00002F38  98                cbw
00002F39  92                xchg ax,dx
00002F3A  BF1BA6            mov di,0xa61b
00002F3D  E86BD9            call 0x8ab
00002F40  5A                pop dx
00002F41  16                push ss
00002F42  07                pop es
00002F43  8D7EA2            lea di,[bp-0x5e]
00002F46  262A15            sub dl,[es:di]
00002F49  E850D9            call 0x89c
00002F4C  33D2              xor dx,dx
00002F4E  E88ED8            call 0x7df
00002F51  8D76A2            lea si,[bp-0x5e]
00002F54  16                push ss
00002F55  07                pop es
00002F56  E80F23            call 0x5268
00002F59  803EA0BD13        cmp byte [0xbda0],0x13
00002F5E  7407              jz 0x2f67
00002F60  803EA0BD18        cmp byte [0xbda0],0x18
00002F65  7208              jc 0x2f6f
00002F67  50                push ax
00002F68  E896DB            call 0xb01
00002F6B  E83163            call 0x929f
00002F6E  58                pop ax
00002F6F  0AC0              or al,al
00002F71  740C              jz 0x2f7f
00002F73  BE6FAF            mov si,0xaf6f
00002F76  E8A0F8            call 0x2819
00002F79  E8E8D5            call 0x564
00002F7C  E830D5            call 0x4af
00002F7F  E9A900            jmp 0x302b
00002F82  8DBE1CFF          lea di,[bp-0xe4]
00002F86  16                push ss
00002F87  57                push di
00002F88  BE79B0            mov si,0xb079
00002F8B  E89EDF            call 0xf2c
00002F8E  BF9CBA            mov di,0xba9c
00002F91  E865EE            call 0x1df9
00002F94  BF9CBA            mov di,0xba9c
00002F97  5E                pop si
00002F98  07                pop es
00002F99  06                push es
00002F9A  57                push di
00002F9B  1E                push ds
00002F9C  56                push si
00002F9D  E882DF            call 0xf22
00002FA0  E884E6            call 0x1627
00002FA3  E857EE            call 0x1dfd
00002FA6  807E9F00          cmp byte [bp-0x61],0x0
00002FAA  7513              jnz 0x2fbf
00002FAC  BF0AB0            mov di,0xb00a
00002FAF  E847EE            call 0x1df9
00002FB2  8D7EA2            lea di,[bp-0x5e]
00002FB5  16                push ss
00002FB6  57                push di
00002FB7  E843EE            call 0x1dfd
00002FBA  BF77B0            mov di,0xb077
00002FBD  EB09              jmp 0x2fc8
00002FBF  807E9E18          cmp byte [bp-0x62],0x18
00002FC3  7206              jc 0x2fcb
00002FC5  BF63B1            mov di,0xb163
00002FC8  E82EEE            call 0x1df9
00002FCB  8DBE1CFF          lea di,[bp-0xe4]
00002FCF  E8DBE8            call 0x18ad
00002FD2  E828EE            call 0x1dfd
00002FD5  A0A0BD            mov al,[0xbda0]
00002FD8  3C06              cmp al,0x6
00002FDA  7417              jz 0x2ff3
00002FDC  3C02              cmp al,0x2
00002FDE  7519              jnz 0x2ff9
00002FE0  8E46A0            mov es,word [bp-0x60]
00002FE3  26803E1F002D      cmp byte [es:0x1f],0x2d
00002FE9  7408              jz 0x2ff3
00002FEB  26803E1F0040      cmp byte [es:0x1f],0x40
00002FF1  7506              jnz 0x2ff9
00002FF3  BF66B0            mov di,0xb066
00002FF6  E800EE            call 0x1df9
00002FF9  BF9EBB            mov di,0xbb9e
00002FFC  E8FAED            call 0x1df9
00002FFF  E8F4ED            call 0x1df6
00003002  8DBE1CFF          lea di,[bp-0xe4]
00003006  16                push ss
00003007  57                push di
00003008  8E46A0            mov es,word [bp-0x60]
0000300B  BF1E00            mov di,0x1e
0000300E  06                push es
0000300F  57                push di
00003010  E884E1            call 0x1197
00003013  E8E7ED            call 0x1dfd
00003016  E8DDED            call 0x1df6
00003019  803EBEB900        cmp byte [0xb9be],0x0
0000301E  7508              jnz 0x3028
00003020  BF25B0            mov di,0xb025
00003023  1E                push ds
00003024  57                push di
00003025  E8D5ED            call 0x1dfd
00003028  E81FF3            call 0x234a
0000302B  E8B9D6            call 0x6e7
0000302E  8D7EA2            lea di,[bp-0x5e]
00003031  16                push ss
00003032  57                push di
00003033  E8DBDF            call 0x1011
00003036  7308              jnc 0x3040
00003038  BE6FAF            mov si,0xaf6f
0000303B  BF2DA9            mov di,0xa92d
0000303E  EB22              jmp 0x3062
00003040  8DBE1CFF          lea di,[bp-0xe4]
00003044  16                push ss
00003045  57                push di
00003046  33C0              xor ax,ax
00003048  50                push ax
00003049  E80AF6            call 0x2656
0000304C  BF1DBB            mov di,0xbb1d
0000304F  1E                push ds
00003050  57                push di
00003051  E89EE7            call 0x17f2
00003054  7311              jnc 0x3067
00003056  8D76A2            lea si,[bp-0x5e]
00003059  E8A2DF            call 0xffe
0000305C  BF1DBB            mov di,0xbb1d
0000305F  BE8EA7            mov si,0xa78e
00003062  E8FFD4            call 0x564
00003065  EB2C              jmp 0x3093
00003067  E89ED9            call 0xa08
0000306A  C606BEB900        mov byte [0xb9be],0x0
0000306F  8DBE1CFE          lea di,[bp-0x1e4]
00003073  16                push ss
00003074  57                push di
00003075  8D76A2            lea si,[bp-0x5e]
00003078  E8B1DE            call 0xf2c
0000307B  BF38BD            mov di,0xbd38
0000307E  E878ED            call 0x1df9
00003081  E8C6F2            call 0x234a
00003084  E8D4D5            call 0x65b
00003087  E859D8            call 0x8e3
0000308A  E870D9            call 0x9fd
0000308D  8D76A2            lea si,[bp-0x5e]
00003090  E86BDF            call 0xffe
00003093  E898D4            call 0x52e
00003096  E84BD6            call 0x6e4
00003099  8A1669B1          mov dl,[0xb169]
0000309D  80EA41            sub dl,0x41
000030A0  B40E              mov ah,0xe
000030A2  CD21              int byte 0x21
000030A4  BA69B1            mov dx,0xb169
000030A7  B43B              mov ah,0x3b
000030A9  CD21              int byte 0x21
000030AB  E98CDD            jmp 0xe3a
000030AE  55                push bp
000030AF  8BEC              mov bp,sp
000030B1  56                push si
000030B2  83EC08            sub sp,0x8
000030B5  A0A0BD            mov al,[0xbda0]
000030B8  3C19              cmp al,0x19
000030BA  7404              jz 0x30c0
000030BC  3C0E              cmp al,0xe
000030BE  7503              jnz 0x30c3
000030C0  E977DD            jmp 0xe3a
000030C3  BF9CBA            mov di,0xba9c
000030C6  0BF6              or si,si
000030C8  750E              jnz 0x30d8
000030CA  BE9EBB            mov si,0xbb9e
000030CD  E850DE            call 0xf20
000030D0  C41670BD          les dx,word [0xbd70]
000030D4  8CC1              mov cx,es
000030D6  EB19              jmp 0x30f1
000030D8  96                xchg ax,si
000030D9  57                push di
000030DA  E854F1            call 0x2231
000030DD  5F                pop di
000030DE  1E                push ds
000030DF  06                push es
000030E0  1F                pop ds
000030E1  BE1E00            mov si,0x1e
000030E4  07                pop es
000030E5  E83ADE            call 0xf22
000030E8  BE0300            mov si,0x3
000030EB  AD                lodsw
000030EC  92                xchg ax,dx
000030ED  AD                lodsw
000030EE  91                xchg ax,cx
000030EF  06                push es
000030F0  1F                pop ds
000030F1  E8DFDE            call 0xfd3
000030F4  A364BD            mov [0xbd64],ax
000030F7  891666BD          mov [0xbd66],dx
000030FB  0BC2              or ax,dx
000030FD  74C1              jz 0x30c0
000030FF  E822DF            call 0x1024
00003102  A0A0BD            mov al,[0xbda0]
00003105  3C03              cmp al,0x3
00003107  754A              jnz 0x3153
00003109  837EFE00          cmp word [bp-0x2],0x0
0000310D  7434              jz 0x3143
0000310F  E8C6DE            call 0xfd8
00003112  E8B6DE            call 0xfcb
00003115  C446F8            les ax,word [bp-0x8]
00003118  8CC3              mov bx,es
0000311A  03C3              add ax,bx
0000311C  051200            add ax,0x12
0000311F  C41664BD          les dx,word [0xbd64]
00003123  8CC1              mov cx,es
00003125  03D0              add dx,ax
00003127  83D100            adc cx,0x0
0000312A  E8A6DE            call 0xfd3
0000312D  EB17              jmp 0x3146
0000312F  B90400            mov cx,0x4
00003132  E889DE            call 0xfbe
00003135  AD                lodsw
00003136  86E0              xchg ah,al
00003138  0BC0              or ax,ax
0000313A  757A              jnz 0x31b6
0000313C  AD                lodsw
0000313D  86E0              xchg ah,al
0000313F  91                xchg ax,cx
00003140  E98900            jmp 0x31cc
00003143  E885DE            call 0xfcb
00003146  8B4EF6            mov cx,[bp-0xa]
00003149  E896DE            call 0xfe2
0000314C  8BF9              mov di,cx
0000314E  B00A              mov al,0xa
00003150  AA                stosb
00003151  EB7E              jmp 0x31d1
00003153  7C0C              jl 0x3161
00003155  3C05              cmp al,0x5
00003157  7513              jnz 0x316c
00003159  E87CDE            call 0xfd8
0000315C  8B4EF8            mov cx,[bp-0x8]
0000315F  EB6B              jmp 0x31cc
00003161  B90300            mov cx,0x3
00003164  E874DE            call 0xfdb
00003167  294EF8            sub [bp-0x8],cx
0000316A  EBF0              jmp 0x315c
0000316C  7C25              jl 0x3193
0000316E  3C0F              cmp al,0xf
00003170  750C              jnz 0x317e
00003172  E863DE            call 0xfd8
00003175  8B46FA            mov ax,[bp-0x6]
00003178  2D0500            sub ax,0x5
0000317B  91                xchg ax,cx
0000317C  EB4E              jmp 0x31cc
0000317E  7218              jc 0x3198
00003180  3C17              cmp al,0x17
00003182  750A              jnz 0x318e
00003184  B90100            mov cx,0x1
00003187  E834DE            call 0xfbe
0000318A  AC                lodsb
0000318B  98                cbw
0000318C  EBED              jmp 0x317b
0000318E  E83ADE            call 0xfcb
00003191  EB36              jmp 0x31c9
00003193  B92000            mov cx,0x20
00003196  EB34              jmp 0x31cc
00003198  3C06              cmp al,0x6
0000319A  7508              jnz 0x31a4
0000319C  B90108            mov cx,0x801
0000319F  E840DE            call 0xfe2
000031A2  EB30              jmp 0x31d4
000031A4  3C0D              cmp al,0xd
000031A6  7513              jnz 0x31bb
000031A8  837EFE00          cmp word [bp-0x2],0x0
000031AC  7481              jz 0x312f
000031AE  B90200            mov cx,0x2
000031B1  E80ADE            call 0xfbe
000031B4  EB86              jmp 0x313c
000031B6  B91E08            mov cx,0x81e
000031B9  EB11              jmp 0x31cc
000031BB  E81ADE            call 0xfd8
000031BE  E80ADE            call 0xfcb
000031C1  C456F8            les dx,word [bp-0x8]
000031C4  8CC1              mov cx,es
000031C6  E80ADE            call 0xfd3
000031C9  8B4EF6            mov cx,[bp-0xa]
000031CC  E813DE            call 0xfe2
000031CF  8BF9              mov di,cx
000031D1  32C0              xor al,al
000031D3  AA                stosb
000031D4  B303              mov bl,0x3
000031D6  1E                push ds
000031D7  8E1E34BD          mov ds,word [0xbd34]
000031DB  33F6              xor si,si
000031DD  AC                lodsb
000031DE  3C0A              cmp al,0xa
000031E0  7501              jnz 0x31e3
000031E2  43                inc bx
000031E3  0AC0              or al,al
000031E5  75F6              jnz 0x31dd
000031E7  1F                pop ds
000031E8  A0ACB1            mov al,[0xb1ac]
000031EB  D0E8              shr al,0x0
000031ED  0408              add al,0x8
000031EF  3AC3              cmp al,bl
000031F1  7602              jna 0x31f5
000031F3  8AC3              mov al,bl
000031F5  8AD8              mov bl,al
000031F7  D0EB              shr bl,0x0
000031F9  7301              jnc 0x31fc
000031FB  40                inc ax
000031FC  8846FA            mov [bp-0x6],al
000031FF  93                xchg ax,bx
00003200  A18AAF            mov ax,[0xaf8a]
00003203  FECC              dec ah
00003205  92                xchg ax,dx
00003206  A0ACB1            mov al,[0xb1ac]
00003209  D0E8              shr al,0x0
0000320B  D0EB              shr bl,0x0
0000320D  48                dec ax
0000320E  8AE0              mov ah,al
00003210  2AC3              sub al,bl
00003212  02E3              add ah,bl
00003214  93                xchg ax,bx
00003215  BF1DBB            mov di,0xbb1d
00003218  57                push di
00003219  57                push di
0000321A  BEE6AD            mov si,0xade6
0000321D  E800DD            call 0xf20
00003220  5F                pop di
00003221  BE9CBA            mov si,0xba9c
00003224  E817DD            call 0xf3e
00003227  5E                pop si
00003228  8A04              mov al,[si]
0000322A  3C4E              cmp al,0x4e
0000322C  7204              jc 0x3232
0000322E  B04E              mov al,0x4e
00003230  8804              mov [si],al
00003232  E8ECDA            call 0xd21
00003235  A0DFA3            mov al,[0xa3df]
00003238  E88BD5            call 0x7c6
0000323B  8E0634BD          mov es,word [0xbd34]
0000323F  33FF              xor di,di
00003241  C646F602          mov byte [bp-0xa],0x2
00003245  897EFC            mov [bp-0x4],di
00003248  268A05            mov al,[es:di]
0000324B  0AC0              or al,al
0000324D  7450              jz 0x329f
0000324F  0AC0              or al,al
00003251  7455              jz 0x32a8
00003253  8A5EF6            mov bl,[bp-0xa]
00003256  3A5EFA            cmp bl,[bp-0x6]
00003259  7D4D              jnl 0x32a8
0000325B  3C09              cmp al,0x9
0000325D  7515              jnz 0x3274
0000325F  FE46FC            inc byte [bp-0x4]
00003262  B020              mov al,0x20
00003264  E8B0D5            call 0x817
00003267  8A46FC            mov al,[bp-0x4]
0000326A  A807              test al,0x7
0000326C  7412              jz 0x3280
0000326E  3C4E              cmp al,0x4e
00003270  7CED              jl 0x325f
00003272  EB0C              jmp 0x3280
00003274  807EFC4E          cmp byte [bp-0x4],0x4e
00003278  7D03              jnl 0x327d
0000327A  E89AD5            call 0x817
0000327D  FE46FC            inc byte [bp-0x4]
00003280  268B05            mov ax,[es:di]
00003283  3C0A              cmp al,0xa
00003285  7512              jnz 0x3299
00003287  FE46F6            inc byte [bp-0xa]
0000328A  80FC0D            cmp ah,0xd
0000328D  740A              jz 0x3299
0000328F  B00D              mov al,0xd
00003291  E883D5            call 0x817
00003294  32C0              xor al,al
00003296  8846FC            mov [bp-0x4],al
00003299  47                inc di
0000329A  268A05            mov al,[es:di]
0000329D  EBB0              jmp 0x324f
0000329F  E842D4            call 0x6e4
000032A2  E88CDD            call 0x1031
000032A5  E992DB            jmp 0xe3a
000032A8  06                push es
000032A9  57                push di
000032AA  268B45FF          mov ax,[es:di-0x1]
000032AE  0AE4              or ah,ah
000032B0  750B              jnz 0x32bd
000032B2  3C0A              cmp al,0xa
000032B4  7407              jz 0x32bd
000032B6  3C0D              cmp al,0xd
000032B8  7403              jz 0x32bd
000032BA  E853D5            call 0x810
000032BD  A0DEA3            mov al,[0xa3de]
000032C0  A2B3BD            mov [0xbdb3],al
000032C3  E8B5D5            call 0x87b
000032C6  BFD1A8            mov di,0xa8d1
000032C9  E8B9D5            call 0x885
000032CC  E8FBD2            call 0x5ca
000032CF  C646F603          mov byte [bp-0xa],0x3
000032D3  B00D              mov al,0xd
000032D5  E83FD5            call 0x817
000032D8  A0DFA3            mov al,[0xa3df]
000032DB  A2B3BD            mov [0xbdb3],al
000032DE  E82FD4            call 0x710
000032E1  5F                pop di
000032E2  07                pop es
000032E3  803E5BBD1B        cmp byte [0xbd5b],0x1b
000032E8  74B5              jz 0x329f
000032EA  E95BFF            jmp 0x3248
000032ED  8CC8              mov ax,cs
000032EF  48                dec ax
000032F0  8EC0              mov es,ax
000032F2  26A10300          mov ax,[es:0x3]
000032F6  2B0621BF          sub ax,[0xbf21]
000032FA  030625BF          add ax,[0xbf25]
000032FE  A3C4B9            mov [0xb9c4],ax
00003301  C3                ret
00003302  0100              add [bx+si],ax
00003304  55                push bp
00003305  8BEC              mov bp,sp
00003307  81EC0001          sub sp,0x100
0000330B  8DBE00FF          lea di,[bp-0x100]
0000330F  BEBFBD            mov si,0xbdbf
00003312  16                push ss
00003313  07                pop es
00003314  3C1B              cmp al,0x1b
00003316  741B              jz 0x3333
00003318  833E59BD01        cmp word [0xbd59],0x1
0000331D  7E14              jng 0x3333
0000331F  C606BEBD01        mov byte [0xbdbe],0x1
00003324  3C08              cmp al,0x8
00003326  7514              jnz 0x333c
00003328  AC                lodsb
00003329  0AC0              or al,al
0000332B  7453              jz 0x3380
0000332D  FE0EBFBD          dec byte [0xbdbf]
00003331  EB4D              jmp 0x3380
00003333  32C0              xor al,al
00003335  8804              mov [si],al
00003337  A2BEBD            mov [0xbdbe],al
0000333A  EB44              jmp 0x3380
0000333C  E883E0            call 0x13c2
0000333F  3C21              cmp al,0x21
00003341  7C3D              jl 0x3380
00003343  06                push es
00003344  57                push di
00003345  BF0234            mov di,0x3402
00003348  2E884501          mov [cs:di+0x1],al
0000334C  0E                push cs
0000334D  57                push di
0000334E  8DBE00FF          lea di,[bp-0x100]
00003352  E8CDDB            call 0xf22
00003355  E8A5EA            call 0x1dfd
00003358  BF5BB0            mov di,0xb05b
0000335B  E89BEA            call 0x1df9
0000335E  A1C8BC            mov ax,[0xbcc8]
00003361  48                dec ax
00003362  E8FAEE            call 0x225f
00003365  7319              jnc 0x3380
00003367  3B06C8BC          cmp ax,[0xbcc8]
0000336B  7406              jz 0x3373
0000336D  A3C8BC            mov [0xbcc8],ax
00003370  E8B4E3            call 0x1727
00003373  BF0234            mov di,0x3402
00003376  BEBFBD            mov si,0xbdbf
00003379  1E                push ds
0000337A  56                push si
0000337B  0E                push cs
0000337C  57                push di
0000337D  E87DEA            call 0x1dfd
00003380  E9B7DA            jmp 0xe3a
00003383  55                push bp
00003384  8BEC              mov bp,sp
00003386  C47E04            les di,word [bp+0x4]
00003389  BECBB9            mov si,0xb9cb
0000338C  AD                lodsw
0000338D  0AC0              or al,al
0000338F  7503              jnz 0x3394
00003391  AA                stosb
00003392  EB30              jmp 0x33c4
00003394  8826EAB0          mov [0xb0ea],ah
00003398  BEBEB0            mov si,0xb0be
0000339B  A0A0BD            mov al,[0xbda0]
0000339E  3C09              cmp al,0x9
000033A0  7208              jc 0x33aa
000033A2  3C0B              cmp al,0xb
000033A4  7412              jz 0x33b8
000033A6  3C18              cmp al,0x18
000033A8  721A              jc 0x33c4
000033AA  3C05              cmp al,0x5
000033AC  7707              ja 0x33b5
000033AE  3C02              cmp al,0x2
000033B0  7603              jna 0x33b5
000033B2  83C603            add si,0x3
000033B5  83C603            add si,0x3
000033B8  E867DB            call 0xf22
000033BB  C47E04            les di,word [bp+0x4]
000033BE  BEE9B0            mov si,0xb0e9
000033C1  E87ADB            call 0xf3e
000033C4  5D                pop bp
000033C5  C3                ret
000033C6  B101              mov cl,0x1
000033C8  E805DF            call 0x12d0
000033CB  33C9              xor cx,cx
000033CD  AC                lodsb
000033CE  D0E0              shl al,0x0
000033D0  720C              jc 0x33de
000033D2  D0E0              shl al,0x0
000033D4  50                push ax
000033D5  7204              jc 0x33db
000033D7  33C0              xor ax,ax
000033D9  EB21              jmp 0x33fc
000033DB  41                inc cx
000033DC  EB07              jmp 0x33e5
000033DE  D0E0              shl al,0x0
000033E0  50                push ax
000033E1  720D              jc 0x33f0
000033E3  B102              mov cl,0x2
000033E5  51                push cx
000033E6  B101              mov cl,0x1
000033E8  E8E5DE            call 0x12d0
000033EB  AC                lodsb
000033EC  32E4              xor ah,ah
000033EE  EB0B              jmp 0x33fb
000033F0  B304              mov bl,0x4
000033F2  53                push bx
000033F3  B102              mov cl,0x2
000033F5  E8D8DE            call 0x12d0
000033F8  AD                lodsw
000033F9  86C4              xchg al,ah
000033FB  59                pop cx
000033FC  80C104            add cl,0x4
000033FF  5B                pop bx
00003400  50                push ax
00003401  D0E3              shl bl,0x0
00003403  7303              jnc 0x3408
00003405  80C102            add cl,0x2
00003408  D0E3              shl bl,0x0
0000340A  7303              jnc 0x340f
0000340C  80C102            add cl,0x2
0000340F  E8BEDE            call 0x12d0
00003412  58                pop ax
00003413  C3                ret
00003414  C606CABC00        mov byte [0xbcca],0x0
00003419  B102              mov cl,0x2
0000341B  E8B2DE            call 0x12d0
0000341E  AD                lodsw
0000341F  86E0              xchg ah,al
00003421  50                push ax
00003422  D1E0              shl ax,0x0
00003424  D1E0              shl ax,0x0
00003426  D1E0              shl ax,0x0
00003428  7216              jc 0x3440
0000342A  D1E0              shl ax,0x0
0000342C  7205              jc 0x3433
0000342E  33C0              xor ax,ax
00003430  50                push ax
00003431  EB2C              jmp 0x345f
00003433  B101              mov cl,0x1
00003435  E898DE            call 0x12d0
00003438  AC                lodsb
00003439  32E4              xor ah,ah
0000343B  50                push ax
0000343C  B101              mov cl,0x1
0000343E  EB1C              jmp 0x345c
00003440  D1E0              shl ax,0x0
00003442  720D              jc 0x3451
00003444  B101              mov cl,0x1
00003446  E887DE            call 0x12d0
00003449  AC                lodsb
0000344A  32E4              xor ah,ah
0000344C  50                push ax
0000344D  B102              mov cl,0x2
0000344F  EB0B              jmp 0x345c
00003451  B102              mov cl,0x2
00003453  E87ADE            call 0x12d0
00003456  AD                lodsw
00003457  86E0              xchg ah,al
00003459  50                push ax
0000345A  B104              mov cl,0x4
0000345C  E871DE            call 0x12d0
0000345F  B104              mov cl,0x4
00003461  E86CDE            call 0x12d0
00003464  AD                lodsw
00003465  86E0              xchg ah,al
00003467  A3DBBC            mov [0xbcdb],ax
0000346A  AD                lodsw
0000346B  86E0              xchg ah,al
0000346D  A3D9BC            mov [0xbcd9],ax
00003470  1E                push ds
00003471  07                pop es
00003472  E84AE7            call 0x1bbf
00003475  5B                pop bx
00003476  58                pop ax
00003477  53                push bx
00003478  D1E0              shl ax,0x0
0000347A  50                push ax
0000347B  7313              jnc 0x3490
0000347D  B104              mov cl,0x4
0000347F  E84EDE            call 0x12d0
00003482  AD                lodsw
00003483  86E0              xchg ah,al
00003485  A3D7BC            mov [0xbcd7],ax
00003488  AD                lodsw
00003489  86E0              xchg ah,al
0000348B  A3D5BC            mov [0xbcd5],ax
0000348E  EB10              jmp 0x34a0
00003490  B102              mov cl,0x2
00003492  E83BDE            call 0x12d0
00003495  AD                lodsw
00003496  86E0              xchg ah,al
00003498  A3D5BC            mov [0xbcd5],ax
0000349B  33C0              xor ax,ax
0000349D  A3D7BC            mov [0xbcd7],ax
000034A0  58                pop ax
000034A1  D1E0              shl ax,0x0
000034A3  50                push ax
000034A4  7313              jnc 0x34b9
000034A6  B104              mov cl,0x4
000034A8  E825DE            call 0x12d0
000034AB  AD                lodsw
000034AC  86E0              xchg ah,al
000034AE  A3D3BC            mov [0xbcd3],ax
000034B1  AD                lodsw
000034B2  86E0              xchg ah,al
000034B4  A3D1BC            mov [0xbcd1],ax
000034B7  EB10              jmp 0x34c9
000034B9  B102              mov cl,0x2
000034BB  E812DE            call 0x12d0
000034BE  AD                lodsw
000034BF  86E0              xchg ah,al
000034C1  A3D1BC            mov [0xbcd1],ax
000034C4  33C0              xor ax,ax
000034C6  A3D3BC            mov [0xbcd3],ax
000034C9  58                pop ax
000034CA  F6C420            test ah,0x20
000034CD  740A              jz 0x34d9
000034CF  50                push ax
000034D0  B102              mov cl,0x2
000034D2  E8FBDD            call 0x12d0
000034D5  AD                lodsw
000034D6  86E0              xchg ah,al
000034D8  58                pop ax
000034D9  F6C408            test ah,0x8
000034DC  7405              jz 0x34e3
000034DE  800ECABC20        or byte [0xbcca],0x20
000034E3  F6C410            test ah,0x10
000034E6  7414              jz 0x34fc
000034E8  B101              mov cl,0x1
000034EA  E8E3DD            call 0x12d0
000034ED  AC                lodsb
000034EE  A810              test al,0x10
000034F0  7402              jz 0x34f4
000034F2  0402              add al,0x2
000034F4  0AC0              or al,al
000034F6  7404              jz 0x34fc
000034F8  91                xchg ax,cx
000034F9  E8D4DD            call 0x12d0
000034FC  58                pop ax
000034FD  C3                ret
000034FE  BF1EBB            mov di,0xbb1e
00003501  57                push di
00003502  B101              mov cl,0x1
00003504  E8C9DD            call 0x12d0
00003507  AC                lodsb
00003508  0AC0              or al,al
0000350A  5F                pop di
0000350B  7405              jz 0x3512
0000350D  8805              mov [di],al
0000350F  47                inc di
00003510  EBEF              jmp 0x3501
00003512  97                xchg ax,di
00003513  2D1EBB            sub ax,0xbb1e
00003516  A21DBB            mov [0xbb1d],al
00003519  C3                ret
0000351A  BF9DBA            mov di,0xba9d
0000351D  57                push di
0000351E  B101              mov cl,0x1
00003520  E8ADDD            call 0x12d0
00003523  AC                lodsb
00003524  0AC0              or al,al
00003526  5F                pop di
00003527  7405              jz 0x352e
00003529  8805              mov [di],al
0000352B  47                inc di
0000352C  EBEF              jmp 0x351d
0000352E  97                xchg ax,di
0000352F  2D9DBA            sub ax,0xba9d
00003532  A29CBA            mov [0xba9c],al
00003535  E875EE            call 0x23ad
00003538  52                push dx
00003539  50                push ax
0000353A  C416AFBD          les dx,word [0xbdaf]
0000353E  8CC1              mov cx,es
00003540  E89EEE            call 0x23e1
00003543  33C9              xor cx,cx
00003545  51                push cx
00003546  E8CBFE            call 0x3414
00003549  59                pop cx
0000354A  41                inc cx
0000354B  3B0E59BD          cmp cx,[0xbd59]
0000354F  76F4              jna 0x3545
00003551  0BC0              or ax,ax
00003553  750B              jnz 0x3560
00003555  BF1DBB            mov di,0xbb1d
00003558  BE9CBA            mov si,0xba9c
0000355B  E8C2D9            call 0xf20
0000355E  EB48              jmp 0x35a8
00003560  50                push ax
00003561  C416A3BD          les dx,word [0xbda3]
00003565  8CC1              mov cx,es
00003567  E877EE            call 0x23e1
0000356A  59                pop cx
0000356B  51                push cx
0000356C  E308              jcxz 0x3576
0000356E  51                push cx
0000356F  E88CFF            call 0x34fe
00003572  59                pop cx
00003573  49                dec cx
00003574  75F8              jnz 0x356e
00003576  BF1DBB            mov di,0xbb1d
00003579  BE9CBA            mov si,0xba9c
0000357C  56                push si
0000357D  57                push di
0000357E  8A1D              mov bl,[di]
00003580  32FF              xor bh,bh
00003582  C641015C          mov byte [bx+di+0x1],0x5c
00003586  FE05              inc byte [di]
00003588  E8B1D9            call 0xf3c
0000358B  5E                pop si
0000358C  5F                pop di
0000358D  E890D9            call 0xf20
00003590  C41664BD          les dx,word [0xbd64]
00003594  8CC1              mov cx,es
00003596  E848EE            call 0x23e1
00003599  59                pop cx
0000359A  E308              jcxz 0x35a4
0000359C  51                push cx
0000359D  E826FE            call 0x33c6
000035A0  59                pop cx
000035A1  49                dec cx
000035A2  75F8              jnz 0x359c
000035A4  0BC0              or ax,ax
000035A6  75B8              jnz 0x3560
000035A8  5A                pop dx
000035A9  59                pop cx
000035AA  E834EE            call 0x23e1
000035AD  E82C12            call 0x47dc
000035B0  C3                ret
000035B1  32E4              xor ah,ah
000035B3  33DB              xor bx,bx
000035B5  EB13              jmp 0x35ca
000035B7  C406D1BC          les ax,word [0xbcd1]
000035BB  8CC3              mov bx,es
000035BD  03C1              add ax,cx
000035BF  83D300            adc bx,0x0
000035C2  EB06              jmp 0x35ca
000035C4  C406D1BC          les ax,word [0xbcd1]
000035C8  8CC3              mov bx,es
000035CA  C41664BD          les dx,word [0xbd64]
000035CE  8CC1              mov cx,es
000035D0  03D0              add dx,ax
000035D2  13CB              adc cx,bx
000035D4  891664BD          mov [0xbd64],dx
000035D8  890E66BD          mov [0xbd66],cx
000035DC  C3                ret
000035DD  A364BD            mov [0xbd64],ax
000035E0  891666BD          mov [0xbd66],dx
000035E4  33C0              xor ax,ax
000035E6  BFCABC            mov di,0xbcca
000035E9  B90F00            mov cx,0xf
000035EC  1E                push ds
000035ED  07                pop es
000035EE  F3AB              rep stosw
000035F0  C3                ret
000035F1  83C20F            add dx,0xf
000035F4  83D100            adc cx,0x0
000035F7  891664BD          mov [0xbd64],dx
000035FB  890E66BD          mov [0xbd66],cx
000035FF  E8DFED            call 0x23e1
00003602  B128              mov cl,0x28
00003604  E8C9DC            call 0x12d0
00003607  3C28              cmp al,0x28
00003609  75E5              jnz 0x35f0
0000360B  AD                lodsw
0000360C  3D8E68            cmp ax,0x688e
0000360F  75DF              jnz 0x35f0
00003611  E8D0FF            call 0x35e4
00003614  AD                lodsw
00003615  BFD1BC            mov di,0xbcd1
00003618  A5                movsw
00003619  A5                movsw
0000361A  83C609            add si,0x9
0000361D  AC                lodsb
0000361E  2418              and al,0x18
00003620  A2CABC            mov [0xbcca],al
00003623  BFD9BC            mov di,0xbcd9
00003626  A5                movsw
00003627  A5                movsw
00003628  BFD5BC            mov di,0xbcd5
0000362B  A5                movsw
0000362C  A5                movsw
0000362D  BF1EBB            mov di,0xbb1e
00003630  B10D              mov cl,0xd
00003632  AC                lodsb
00003633  0AC0              or al,al
00003635  7403              jz 0x363a
00003637  AA                stosb
00003638  E2F8              loop 0x3632
0000363A  97                xchg ax,di
0000363B  2D1EBB            sub ax,0xbb1e
0000363E  A21DBB            mov [0xbb1d],al
00003641  E88C11            call 0x47d0
00003644  73AA              jnc 0x35f0
00003646  B92800            mov cx,0x28
00003649  E86BFF            call 0x35b7
0000364C  EBB1              jmp 0x35ff
0000364E  B104              mov cl,0x4
00003650  E87DDC            call 0x12d0
00003653  AD                lodsw
00003654  3D4F5A            cmp ax,0x5a4f
00003657  7597              jnz 0x35f0
00003659  AC                lodsb
0000365A  3CDE              cmp al,0xde
0000365C  7502              jnz 0x3660
0000365E  EB90              jmp 0x35f0
00003660  3CE0              cmp al,0xe0
00003662  7424              jz 0x3688
00003664  7309              jnc 0x366f
00003666  AC                lodsb
00003667  E847FF            call 0x35b1
0000366A  E874ED            call 0x23e1
0000366D  EBDF              jmp 0x364e
0000366F  3CE2              cmp al,0xe2
00003671  77F3              ja 0x3666
00003673  7435              jz 0x36aa
00003675  AC                lodsb
00003676  E838FF            call 0x35b1
00003679  2C04              sub al,0x4
0000367B  8AC8              mov cl,al
0000367D  E850DC            call 0x12d0
00003680  AD                lodsw
00003681  AD                lodsw
00003682  AD                lodsw
00003683  E82DFF            call 0x35b3
00003686  EBE2              jmp 0x366a
00003688  AC                lodsb
00003689  E825FF            call 0x35b1
0000368C  2C04              sub al,0x4
0000368E  8AC8              mov cl,al
00003690  E83DDC            call 0x12d0
00003693  83C606            add si,0x6
00003696  33DB              xor bx,bx
00003698  803800            cmp byte [bx+si],0x0
0000369B  7403              jz 0x36a0
0000369D  43                inc bx
0000369E  EBF8              jmp 0x3698
000036A0  BF9CBA            mov di,0xba9c
000036A3  93                xchg ax,bx
000036A4  AA                stosb
000036A5  91                xchg ax,cx
000036A6  F3A4              rep movsb
000036A8  EBA4              jmp 0x364e
000036AA  AC                lodsb
000036AB  E803FF            call 0x35b1
000036AE  2C04              sub al,0x4
000036B0  8AC8              mov cl,al
000036B2  E81BDC            call 0x12d0
000036B5  E82CFF            call 0x35e4
000036B8  AD                lodsw
000036B9  BFD1BC            mov di,0xbcd1
000036BC  A5                movsw
000036BD  A5                movsw
000036BE  AD                lodsw
000036BF  BFD9BC            mov di,0xbcd9
000036C2  A5                movsw
000036C3  A5                movsw
000036C4  83C606            add si,0x6
000036C7  AD                lodsw
000036C8  A2E1BC            mov [0xbce1],al
000036CB  BFD5BC            mov di,0xbcd5
000036CE  A5                movsw
000036CF  A5                movsw
000036D0  33DB              xor bx,bx
000036D2  803800            cmp byte [bx+si],0x0
000036D5  7403              jz 0x36da
000036D7  43                inc bx
000036D8  EBF8              jmp 0x36d2
000036DA  56                push si
000036DB  53                push bx
000036DC  BF1DBB            mov di,0xbb1d
000036DF  BE9CBA            mov si,0xba9c
000036E2  E83DD8            call 0xf22
000036E5  5B                pop bx
000036E6  5E                pop si
000036E7  BF1DBB            mov di,0xbb1d
000036EA  8A05              mov al,[di]
000036EC  98                cbw
000036ED  011D              add [di],bx
000036EF  47                inc di
000036F0  03F8              add di,ax
000036F2  8BCB              mov cx,bx
000036F4  F3A4              rep movsb
000036F6  E8D710            call 0x47d0
000036F9  7303              jnc 0x36fe
000036FB  E950FF            jmp 0x364e
000036FE  C3                ret
000036FF  83C208            add dx,0x8
00003702  83D100            adc cx,0x0
00003705  891664BD          mov [0xbd64],dx
00003709  890E66BD          mov [0xbd66],cx
0000370D  E8D1EC            call 0x23e1
00003710  B104              mov cl,0x4
00003712  E8BBDB            call 0x12d0
00003715  3C04              cmp al,0x4
00003717  75E5              jnz 0x36fe
00003719  E897FE            call 0x35b3
0000371C  AD                lodsw
0000371D  3D3703            cmp ax,0x337
00003720  7573              jnz 0x3795
00003722  E8BFFE            call 0x35e4
00003725  AC                lodsb
00003726  A801              test al,0x1
00003728  7405              jz 0x372f
0000372A  C606CABC20        mov byte [0xbcca],0x20
0000372F  A804              test al,0x4
00003731  7405              jz 0x3738
00003733  800ECABC04        or byte [0xbcca],0x4
00003738  A802              test al,0x2
0000373A  7422              jz 0x375e
0000373C  C40664BD          les ax,word [0xbd64]
00003740  8CC2              mov dx,es
00003742  2D0100            sub ax,0x1
00003745  83DA00            sbb dx,0x0
00003748  A3CDBC            mov [0xbccd],ax
0000374B  8916CFBC          mov [0xbccf],dx
0000374F  AC                lodsb
00003750  E85EFE            call 0x35b1
00003753  E88BEC            call 0x23e1
00003756  B101              mov cl,0x1
00003758  E875DB            call 0x12d0
0000375B  E855FE            call 0x35b3
0000375E  AC                lodsb
0000375F  E84FFE            call 0x35b1
00003762  8AC8              mov cl,al
00003764  E869DB            call 0x12d0
00003767  BF1DBB            mov di,0xbb1d
0000376A  AA                stosb
0000376B  98                cbw
0000376C  91                xchg ax,cx
0000376D  F3A4              rep movsb
0000376F  B111              mov cl,0x11
00003771  E85CDB            call 0x12d0
00003774  AD                lodsw
00003775  A2E1BC            mov [0xbce1],al
00003778  BFD9BC            mov di,0xbcd9
0000377B  A5                movsw
0000377C  A5                movsw
0000377D  BFD5BC            mov di,0xbcd5
00003780  A5                movsw
00003781  A5                movsw
00003782  BFD1BC            mov di,0xbcd1
00003785  A5                movsw
00003786  A5                movsw
00003787  E84610            call 0x47d0
0000378A  7309              jnc 0x3795
0000378C  B91100            mov cx,0x11
0000378F  E825FE            call 0x35b7
00003792  E978FF            jmp 0x370d
00003795  C3                ret
00003796  891664BD          mov [0xbd64],dx
0000379A  890E66BD          mov [0xbd66],cx
0000379E  E840EC            call 0x23e1
000037A1  B107              mov cl,0x7
000037A3  E82ADB            call 0x12d0
000037A6  0AC0              or al,al
000037A8  74EB              jz 0x3795
000037AA  AD                lodsw
000037AB  AD                lodsw
000037AC  050400            add ax,0x4
000037AF  50                push ax
000037B0  AC                lodsb
000037B1  0AC0              or al,al
000037B3  7541              jnz 0x37f6
000037B5  AD                lodsw
000037B6  A802              test al,0x2
000037B8  7429              jz 0x37e3
000037BA  B117              mov cl,0x17
000037BC  F6C410            test ah,0x10
000037BF  7401              jz 0x37c2
000037C1  41                inc cx
000037C2  E80BDB            call 0x12d0
000037C5  3C18              cmp al,0x18
000037C7  7507              jnz 0x37d0
000037C9  8A4417            mov al,[si+0x17]
000037CC  32E4              xor ah,ah
000037CE  EB02              jmp 0x37d2
000037D0  33C0              xor ax,ax
000037D2  50                push ax
000037D3  E8D7EB            call 0x23ad
000037D6  59                pop cx
000037D7  03C1              add ax,cx
000037D9  83D200            adc dx,0x0
000037DC  A370BD            mov [0xbd70],ax
000037DF  891672BD          mov [0xbd72],dx
000037E3  F6C408            test ah,0x8
000037E6  7406              jz 0x37ee
000037E8  C706A1BD0800      mov word [0xbda1],0x8
000037EE  58                pop ax
000037EF  E8C1FD            call 0x35b3
000037F2  EBAA              jmp 0x379e
000037F4  58                pop ax
000037F5  C3                ret
000037F6  3C01              cmp al,0x1
000037F8  75FA              jnz 0x37f4
000037FA  E8E7FD            call 0x35e4
000037FD  C606E7BC14        mov byte [0xbce7],0x14
00003802  8B14              mov dx,[si]
00003804  F6C202            test dl,0x2
00003807  7402              jz 0x380b
00003809  B002              mov al,0x2
0000380B  F6C630            test dh,0x30
0000380E  7402              jz 0x3812
00003810  0C40              or al,0x40
00003812  F6C640            test dh,0x40
00003815  7402              jz 0x3819
00003817  0C04              or al,0x4
00003819  03D2              add dx,dx
0000381B  7309              jnc 0x3826
0000381D  803EB9BC00        cmp byte [0xbcb9],0x0
00003822  7402              jz 0x3826
00003824  0C20              or al,0x20
00003826  A2CABC            mov [0xbcca],al
00003829  B11C              mov cl,0x1c
0000382B  E8A2DA            call 0x12d0
0000382E  BFD1BC            mov di,0xbcd1
00003831  A5                movsw
00003832  A5                movsw
00003833  A5                movsw
00003834  A5                movsw
00003835  A5                movsw
00003836  A5                movsw
00003837  AD                lodsw
00003838  A2E1BC            mov [0xbce1],al
0000383B  2418              and al,0x18
0000383D  7505              jnz 0x3844
0000383F  C606B9BC01        mov byte [0xbcb9],0x1
00003844  0806CABC          or [0xbcca],al
00003848  AD                lodsw
00003849  A5                movsw
0000384A  A5                movsw
0000384B  AD                lodsw
0000384C  A2E6BC            mov [0xbce6],al
0000384F  3C01              cmp al,0x1
00003851  770C              ja 0x385f
00003853  F606CABC04        test byte [0xbcca],0x4
00003858  7505              jnz 0x385f
0000385A  800ECABC80        or byte [0xbcca],0x80
0000385F  AD                lodsw
00003860  250F00            and ax,0xf
00003863  2D0600            sub ax,0x6
00003866  7E09              jng 0x3871
00003868  3B06BCBD          cmp ax,[0xbdbc]
0000386C  7603              jna 0x3871
0000386E  A3BCBD            mov [0xbdbc],ax
00003871  8B4402            mov ax,[si+0x2]
00003874  91                xchg ax,cx
00003875  E858DA            call 0x12d0
00003878  BF1DBB            mov di,0xbb1d
0000387B  AA                stosb
0000387C  91                xchg ax,cx
0000387D  F3A4              rep movsb
0000387F  F606CABC02        test byte [0xbcca],0x2
00003884  740A              jz 0x3890
00003886  E824EB            call 0x23ad
00003889  A3CDBC            mov [0xbccd],ax
0000388C  8916CFBC          mov [0xbccf],dx
00003890  58                pop ax
00003891  E81FFD            call 0x35b3
00003894  8916E2BC          mov [0xbce2],dx
00003898  890EE4BC          mov [0xbce4],cx
0000389C  E8310F            call 0x47d0
0000389F  737E              jnc 0x391f
000038A1  E820FD            call 0x35c4
000038A4  E9F7FE            jmp 0x379e
000038A7  83C207            add dx,0x7
000038AA  83D100            adc cx,0x0
000038AD  E831EB            call 0x23e1
000038B0  E82AFD            call 0x35dd
000038B3  B16C              mov cl,0x6c
000038B5  E818DA            call 0x12d0
000038B8  3C16              cmp al,0x16
000038BA  7263              jc 0x391f
000038BC  BF1EBB            mov di,0xbb1e
000038BF  AC                lodsb
000038C0  0AC0              or al,al
000038C2  7409              jz 0x38cd
000038C4  3C2F              cmp al,0x2f
000038C6  7502              jnz 0x38ca
000038C8  B05C              mov al,0x5c
000038CA  AA                stosb
000038CB  EBF2              jmp 0x38bf
000038CD  97                xchg ax,di
000038CE  2D1EBB            sub ax,0xbb1e
000038D1  A21DBB            mov [0xbb1d],al
000038D4  8B04              mov ax,[si]
000038D6  A3D9BC            mov [0xbcd9],ax
000038D9  8B4402            mov ax,[si+0x2]
000038DC  A3DBBC            mov [0xbcdb],ax
000038DF  8B4405            mov ax,[si+0x5]
000038E2  A3D5BC            mov [0xbcd5],ax
000038E5  8B4407            mov ax,[si+0x7]
000038E8  A3D7BC            mov [0xbcd7],ax
000038EB  8B4409            mov ax,[si+0x9]
000038EE  A3D1BC            mov [0xbcd1],ax
000038F1  8B440B            mov ax,[si+0xb]
000038F4  A3D3BC            mov [0xbcd3],ax
000038F7  8A4411            mov al,[si+0x11]
000038FA  98                cbw
000038FB  051200            add ax,0x12
000038FE  03C6              add ax,si
00003900  2DBEB1            sub ax,0xb1be
00003903  48                dec ax
00003904  E8ACFC            call 0x35b3
00003907  8B4412            mov ax,[si+0x12]
0000390A  0AC0              or al,al
0000390C  7504              jnz 0x3912
0000390E  8826CABC          mov [0xbcca],ah
00003912  E8AAE2            call 0x1bbf
00003915  E8B80E            call 0x47d0
00003918  7305              jnc 0x391f
0000391A  E8A7FC            call 0x35c4
0000391D  EB8E              jmp 0x38ad
0000391F  C3                ret
00003920  83C204            add dx,0x4
00003923  83D100            adc cx,0x0
00003926  E8B8EA            call 0x23e1
00003929  E8B1FC            call 0x35dd
0000392C  B16C              mov cl,0x6c
0000392E  E89FD9            call 0x12d0
00003931  3C15              cmp al,0x15
00003933  7244              jc 0x3979
00003935  AC                lodsb
00003936  BFD1BC            mov di,0xbcd1
00003939  A5                movsw
0000393A  A5                movsw
0000393B  A5                movsw
0000393C  A5                movsw
0000393D  BECBB1            mov si,0xb1cb
00003940  A5                movsw
00003941  A5                movsw
00003942  BF1EBB            mov di,0xbb1e
00003945  AC                lodsb
00003946  0AC0              or al,al
00003948  7403              jz 0x394d
0000394A  AA                stosb
0000394B  EBF8              jmp 0x3945
0000394D  AC                lodsb
0000394E  0AC0              or al,al
00003950  7403              jz 0x3955
00003952  AA                stosb
00003953  EBF8              jmp 0x394d
00003955  96                xchg ax,si
00003956  2DD0B1            sub ax,0xb1d0
00003959  48                dec ax
0000395A  A21DBB            mov [0xbb1d],al
0000395D  0415              add al,0x15
0000395F  8BD8              mov bx,ax
00003961  8A87BDB1          mov al,[bx-0x4e43]
00003965  32E4              xor ah,ah
00003967  03C3              add ax,bx
00003969  E847FC            call 0x35b3
0000396C  E8610E            call 0x47d0
0000396F  7308              jnc 0x3979
00003971  E850FC            call 0x35c4
00003974  E86AEA            call 0x23e1
00003977  EBB0              jmp 0x3929
00003979  C3                ret
0000397A  83C204            add dx,0x4
0000397D  83D100            adc cx,0x0
00003980  E85EEA            call 0x23e1
00003983  E857FC            call 0x35dd
00003986  B16C              mov cl,0x6c
00003988  E845D9            call 0x12d0
0000398B  0AC0              or al,al
0000398D  74EA              jz 0x3979
0000398F  AC                lodsb
00003990  3C30              cmp al,0x30
00003992  7305              jnc 0x3999
00003994  800ECABC80        or byte [0xbcca],0x80
00003999  240F              and al,0xf
0000399B  A2E6BC            mov [0xbce6],al
0000399E  BFD1BC            mov di,0xbcd1
000039A1  A5                movsw
000039A2  A5                movsw
000039A3  A5                movsw
000039A4  A5                movsw
000039A5  AD                lodsw
000039A6  A3DDBC            mov [0xbcdd],ax
000039A9  AD                lodsw
000039AA  A3DFBC            mov [0xbcdf],ax
000039AD  AD                lodsw
000039AE  2D5046            sub ax,0x4650
000039B1  AB                stosw
000039B2  AD                lodsw
000039B3  1D0000            sbb ax,0x0
000039B6  AB                stosw
000039B7  BF1EBB            mov di,0xbb1e
000039BA  AC                lodsb
000039BB  0AC0              or al,al
000039BD  7403              jz 0x39c2
000039BF  AA                stosb
000039C0  EBF8              jmp 0x39ba
000039C2  AC                lodsb
000039C3  0AC0              or al,al
000039C5  7403              jz 0x39ca
000039C7  AA                stosb
000039C8  EBF8              jmp 0x39c2
000039CA  96                xchg ax,si
000039CB  2DD0B1            sub ax,0xb1d0
000039CE  48                dec ax
000039CF  A21DBB            mov [0xbb1d],al
000039D2  0413              add al,0x13
000039D4  50                push ax
000039D5  E8E7E1            call 0x1bbf
000039D8  5B                pop bx
000039D9  BEBEB1            mov si,0xb1be
000039DC  03F3              add si,bx
000039DE  AC                lodsb
000039DF  02D8              add bl,al
000039E1  AD                lodsw
000039E2  8826E1BC          mov [0xbce1],ah
000039E6  93                xchg ax,bx
000039E7  40                inc ax
000039E8  E8C6FB            call 0x35b1
000039EB  803EE6BC0E        cmp byte [0xbce6],0xe
000039F0  7305              jnc 0x39f7
000039F2  E8DB0D            call 0x47d0
000039F5  7308              jnc 0x39ff
000039F7  E8CAFB            call 0x35c4
000039FA  E8E4E9            call 0x23e1
000039FD  EB84              jmp 0x3983
000039FF  C3                ret
00003A00  E83ED6            call 0x1041
00003A03  2D0F00            sub ax,0xf
00003A06  83DA00            sbb dx,0x0
00003A09  A364BD            mov [0xbd64],ax
00003A0C  891666BD          mov [0xbd66],dx
00003A10  92                xchg ax,dx
00003A11  91                xchg ax,cx
00003A12  E8CCE9            call 0x23e1
00003A15  B10F              mov cl,0xf
00003A17  E8B6D8            call 0x12d0
00003A1A  AD                lodsw
00003A1B  86E0              xchg ah,al
00003A1D  A3ABBD            mov [0xbdab],ax
00003A20  AD                lodsw
00003A21  86E0              xchg ah,al
00003A23  A3ADBD            mov [0xbdad],ax
00003A26  AD                lodsw
00003A27  93                xchg ax,bx
00003A28  AD                lodsw
00003A29  86E3              xchg ah,bl
00003A2B  86C7              xchg al,bh
00003A2D  291E64BD          sub [0xbd64],bx
00003A31  190666BD          sbb [0xbd66],ax
00003A35  46                inc si
00003A36  AD                lodsw
00003A37  8ADC              mov bl,ah
00003A39  AD                lodsw
00003A3A  3D4850            cmp ax,0x5048
00003A3D  7514              jnz 0x3a53
00003A3F  AD                lodsw
00003A40  3D414B            cmp ax,0x4b41
00003A43  750E              jnz 0x3a53
00003A45  F6C30F            test bl,0xf
00003A48  740C              jz 0x3a56
00003A4A  BF7AA6            mov di,0xa67a
00003A4D  BEACA6            mov si,0xa6ac
00003A50  E811CB            call 0x564
00003A53  E985C9            jmp 0x3db
00003A56  C41664BD          les dx,word [0xbd64]
00003A5A  8CC1              mov cx,es
00003A5C  E882E9            call 0x23e1
00003A5F  A1ABBD            mov ax,[0xbdab]
00003A62  0BC0              or ax,ax
00003A64  7408              jz 0x3a6e
00003A66  50                push ax
00003A67  E85CF9            call 0x33c6
00003A6A  58                pop ax
00003A6B  48                dec ax
00003A6C  75F8              jnz 0x3a66
00003A6E  E83CE9            call 0x23ad
00003A71  A3AFBD            mov [0xbdaf],ax
00003A74  8916B1BD          mov [0xbdb1],dx
00003A78  A1ADBD            mov ax,[0xbdad]
00003A7B  0BC0              or ax,ax
00003A7D  7408              jz 0x3a87
00003A7F  50                push ax
00003A80  E891F9            call 0x3414
00003A83  58                pop ax
00003A84  48                dec ax
00003A85  75F8              jnz 0x3a7f
00003A87  E823E9            call 0x23ad
00003A8A  A3A3BD            mov [0xbda3],ax
00003A8D  8916A5BD          mov [0xbda5],dx
00003A91  A1ABBD            mov ax,[0xbdab]
00003A94  0BC0              or ax,ax
00003A96  7408              jz 0x3aa0
00003A98  50                push ax
00003A99  E862FA            call 0x34fe
00003A9C  58                pop ax
00003A9D  48                dec ax
00003A9E  75F8              jnz 0x3a98
00003AA0  A1ADBD            mov ax,[0xbdad]
00003AA3  0BC0              or ax,ax
00003AA5  740A              jz 0x3ab1
00003AA7  50                push ax
00003AA8  E86FFA            call 0x351a
00003AAB  58                pop ax
00003AAC  7303              jnc 0x3ab1
00003AAE  48                dec ax
00003AAF  75F6              jnz 0x3aa7
00003AB1  C3                ret
00003AB2  B106              mov cl,0x6
00003AB4  E819D8            call 0x12d0
00003AB7  E8F3E8            call 0x23ad
00003ABA  A364BD            mov [0xbd64],ax
00003ABD  891666BD          mov [0xbd66],dx
00003AC1  F606C2B101        test byte [0xb1c2],0x1
00003AC6  741C              jz 0x3ae4
00003AC8  A370BD            mov [0xbd70],ax
00003ACB  891672BD          mov [0xbd72],dx
00003ACF  B104              mov cl,0x4
00003AD1  E8FCD7            call 0x12d0
00003AD4  AD                lodsw
00003AD5  86E0              xchg ah,al
00003AD7  93                xchg ax,bx
00003AD8  AD                lodsw
00003AD9  86E0              xchg ah,al
00003ADB  050400            add ax,0x4
00003ADE  83D300            adc bx,0x0
00003AE1  E8E6FA            call 0x35ca
00003AE4  C41664BD          les dx,word [0xbd64]
00003AE8  8CC1              mov cx,es
00003AEA  E8F4E8            call 0x23e1
00003AED  B106              mov cl,0x6
00003AEF  E8DED7            call 0x12d0
00003AF2  3C06              cmp al,0x6
00003AF4  7401              jz 0x3af7
00003AF6  C3                ret
00003AF7  AD                lodsw
00003AF8  3DFF42            cmp ax,0x42ff
00003AFB  75F9              jnz 0x3af6
00003AFD  AD                lodsw
00003AFE  3D5341            cmp ax,0x4153
00003B01  75F3              jnz 0x3af6
00003B03  AD                lodsw
00003B04  86E0              xchg ah,al
00003B06  E8AAFA            call 0x35b3
00003B09  2BD0              sub dx,ax
00003B0B  83D900            sbb cx,0x0
00003B0E  51                push cx
00003B0F  52                push dx
00003B10  91                xchg ax,cx
00003B11  E8BED7            call 0x12d2
00003B14  B00A              mov al,0xa
00003B16  E898FA            call 0x35b1
00003B19  E8C8FA            call 0x35e4
00003B1C  AC                lodsb
00003B1D  A808              test al,0x8
00003B1F  7407              jz 0x3b28
00003B21  C606E6BC00        mov byte [0xbce6],0x0
00003B26  EB09              jmp 0x3b31
00003B28  3C00              cmp al,0x0
00003B2A  750A              jnz 0x3b36
00003B2C  C606E6BC07        mov byte [0xbce6],0x7
00003B31  800ECABC80        or byte [0xbcca],0x80
00003B36  AC                lodsb
00003B37  A801              test al,0x1
00003B39  7405              jz 0x3b40
00003B3B  800ECABC20        or byte [0xbcca],0x20
00003B40  A808              test al,0x8
00003B42  7405              jz 0x3b49
00003B44  800ECABC40        or byte [0xbcca],0x40
00003B49  A820              test al,0x20
00003B4B  7405              jz 0x3b52
00003B4D  800ECABC02        or byte [0xbcca],0x2
00003B52  AC                lodsb
00003B53  AC                lodsb
00003B54  A2E1BC            mov [0xbce1],al
00003B57  AD                lodsw
00003B58  86E0              xchg ah,al
00003B5A  A3DBBC            mov [0xbcdb],ax
00003B5D  AD                lodsw
00003B5E  86E0              xchg ah,al
00003B60  A3D9BC            mov [0xbcd9],ax
00003B63  33C9              xor cx,cx
00003B65  BF1EBB            mov di,0xbb1e
00003B68  AC                lodsb
00003B69  0AC0              or al,al
00003B6B  7404              jz 0x3b71
00003B6D  AA                stosb
00003B6E  41                inc cx
00003B6F  EBF7              jmp 0x3b68
00003B71  880E1DBB          mov [0xbb1d],cl
00003B75  AD                lodsw
00003B76  86E0              xchg ah,al
00003B78  A3D7BC            mov [0xbcd7],ax
00003B7B  AD                lodsw
00003B7C  86E0              xchg ah,al
00003B7E  A3D5BC            mov [0xbcd5],ax
00003B81  AD                lodsw
00003B82  86E0              xchg ah,al
00003B84  A3D3BC            mov [0xbcd3],ax
00003B87  AD                lodsw
00003B88  86E0              xchg ah,al
00003B8A  A3D1BC            mov [0xbcd1],ax
00003B8D  AD                lodsw
00003B8E  86E0              xchg ah,al
00003B90  A3DFBC            mov [0xbcdf],ax
00003B93  AD                lodsw
00003B94  86E0              xchg ah,al
00003B96  A3DDBC            mov [0xbcdd],ax
00003B99  59                pop cx
00003B9A  5A                pop dx
00003B9B  F606CABC02        test byte [0xbcca],0x2
00003BA0  7414              jz 0x3bb6
00003BA2  8BC6              mov ax,si
00003BA4  2DBEB1            sub ax,0xb1be
00003BA7  050600            add ax,0x6
00003BAA  03C1              add ax,cx
00003BAC  83D200            adc dx,0x0
00003BAF  A3CDBC            mov [0xbccd],ax
00003BB2  8916CFBC          mov [0xbccf],dx
00003BB6  E8170C            call 0x47d0
00003BB9  7352              jnc 0x3c0d
00003BBB  E806FA            call 0x35c4
00003BBE  E923FF            jmp 0x3ae4
00003BC1  B105              mov cl,0x5
00003BC3  E80AD7            call 0x12d0
00003BC6  A1C1B1            mov ax,[0xb1c1]
00003BC9  A364BD            mov [0xbd64],ax
00003BCC  33C0              xor ax,ax
00003BCE  A366BD            mov [0xbd66],ax
00003BD1  C41664BD          les dx,word [0xbd64]
00003BD5  8CC1              mov cx,es
00003BD7  E807E8            call 0x23e1
00003BDA  B104              mov cl,0x4
00003BDC  E8F1D6            call 0x12d0
00003BDF  AD                lodsw
00003BE0  50                push ax
00003BE1  AD                lodsw
00003BE2  E8CEF9            call 0x35b3
00003BE5  59                pop cx
00003BE6  81F933C0          cmp cx,0xc033
00003BEA  7512              jnz 0x3bfe
00003BEC  E8BEE7            call 0x23ad
00003BEF  2D0400            sub ax,0x4
00003BF2  83DA00            sbb dx,0x0
00003BF5  A370BD            mov [0xbd70],ax
00003BF8  891672BD          mov [0xbd72],dx
00003BFC  EBD3              jmp 0x3bd1
00003BFE  2D0400            sub ax,0x4
00003C01  91                xchg ax,cx
00003C02  50                push ax
00003C03  E8CCD6            call 0x12d2
00003C06  91                xchg ax,cx
00003C07  58                pop ax
00003C08  3D13F8            cmp ax,0xf813
00003C0B  7501              jnz 0x3c0e
00003C0D  C3                ret
00003C0E  3D23F1            cmp ax,0xf123
00003C11  7419              jz 0x3c2c
00003C13  3D80D1            cmp ax,0xd180
00003C16  75F5              jnz 0x3c0d
00003C18  BF9CBA            mov di,0xba9c
00003C1B  49                dec cx
00003C1C  E301              jcxz 0x3c1f
00003C1E  41                inc cx
00003C1F  91                xchg ax,cx
00003C20  AA                stosb
00003C21  91                xchg ax,cx
00003C22  E3B6              jcxz 0x3bda
00003C24  49                dec cx
00003C25  F3A4              rep movsb
00003C27  B05C              mov al,0x5c
00003C29  AA                stosb
00003C2A  EBAE              jmp 0x3bda
00003C2C  51                push cx
00003C2D  E8B4F9            call 0x35e4
00003C30  59                pop cx
00003C31  AD                lodsw
00003C32  BFD9BC            mov di,0xbcd9
00003C35  A5                movsw
00003C36  A5                movsw
00003C37  AD                lodsw
00003C38  2418              and al,0x18
00003C3A  752C              jnz 0x3c68
00003C3C  46                inc si
00003C3D  BFD5BC            mov di,0xbcd5
00003C40  A5                movsw
00003C41  A5                movsw
00003C42  BFD1BC            mov di,0xbcd1
00003C45  A5                movsw
00003C46  A5                movsw
00003C47  AD                lodsw
00003C48  AD                lodsw
00003C49  83E916            sub cx,0x16
00003C4C  BF1DBB            mov di,0xbb1d
00003C4F  57                push di
00003C50  56                push si
00003C51  51                push cx
00003C52  BE9CBA            mov si,0xba9c
00003C55  E8CAD2            call 0xf22
00003C58  59                pop cx
00003C59  5E                pop si
00003C5A  5F                pop di
00003C5B  8A05              mov al,[di]
00003C5D  98                cbw
00003C5E  000D              add [di],cl
00003C60  03F8              add di,ax
00003C62  47                inc di
00003C63  E8680B            call 0x47ce
00003C66  73A5              jnc 0x3c0d
00003C68  E859F9            call 0x35c4
00003C6B  E963FF            jmp 0x3bd1
00003C6E  B107              mov cl,0x7
00003C70  E85DD6            call 0x12d0
00003C73  E837E7            call 0x23ad
00003C76  8A1EC4B1          mov bl,[0xb1c4]
00003C7A  80E308            and bl,0x8
00003C7D  881EA1BD          mov [0xbda1],bl
00003C81  F6C302            test bl,0x2
00003C84  7403              jz 0x3c89
00003C86  A370BD            mov [0xbd70],ax
00003C89  2D0700            sub ax,0x7
00003C8C  0306C2B1          add ax,[0xb1c2]
00003C90  83D200            adc dx,0x0
00003C93  A364BD            mov [0xbd64],ax
00003C96  891666BD          mov [0xbd66],dx
00003C9A  92                xchg ax,dx
00003C9B  91                xchg ax,cx
00003C9C  E842E7            call 0x23e1
00003C9F  B115              mov cl,0x15
00003CA1  E82CD6            call 0x12d0
00003CA4  3C15              cmp al,0x15
00003CA6  7401              jz 0x3ca9
00003CA8  C3                ret
00003CA9  E838F9            call 0x35e4
00003CAC  BFD1BC            mov di,0xbcd1
00003CAF  A5                movsw
00003CB0  A5                movsw
00003CB1  A5                movsw
00003CB2  A5                movsw
00003CB3  AD                lodsw
00003CB4  50                push ax
00003CB5  AD                lodsw
00003CB6  010664BD          add [0xbd64],ax
00003CBA  831666BD00        adc word [0xbd66],0x0
00003CBF  A5                movsw
00003CC0  A5                movsw
00003CC1  58                pop ax
00003CC2  AB                stosw
00003CC3  AB                stosw
00003CC4  AD                lodsw
00003CC5  AA                stosb
00003CC6  50                push ax
00003CC7  AC                lodsb
00003CC8  A2E7BC            mov [0xbce7],al
00003CCB  AD                lodsw
00003CCC  80EC30            sub ah,0x30
00003CCF  8826E6BC          mov [0xbce6],ah
00003CD3  91                xchg ax,cx
00003CD4  E8F9D5            call 0x12d0
00003CD7  BF1DBB            mov di,0xbb1d
00003CDA  AA                stosb
00003CDB  91                xchg ax,cx
00003CDC  F3A4              rep movsb
00003CDE  58                pop ax
00003CDF  2418              and al,0x18
00003CE1  F6C403            test ah,0x3
00003CE4  7402              jz 0x3ce8
00003CE6  0C40              or al,0x40
00003CE8  F6C408            test ah,0x8
00003CEB  7402              jz 0x3cef
00003CED  0C02              or al,0x2
00003CEF  80E404            and ah,0x4
00003CF2  0AC4              or al,ah
00003CF4  803EE7BC00        cmp byte [0xbce7],0x0
00003CF9  7402              jz 0x3cfd
00003CFB  0C80              or al,0x80
00003CFD  A2CABC            mov [0xbcca],al
00003D00  2402              and al,0x2
00003D02  741C              jz 0x3d20
00003D04  E8A6E6            call 0x23ad
00003D07  A3CDBC            mov [0xbccd],ax
00003D0A  8916CFBC          mov [0xbccf],dx
00003D0E  E8BDD5            call 0x12ce
00003D11  E899E6            call 0x23ad
00003D14  0306BEB1          add ax,[0xb1be]
00003D18  83D200            adc dx,0x0
00003D1B  92                xchg ax,dx
00003D1C  91                xchg ax,cx
00003D1D  E8C1E6            call 0x23e1
00003D20  E8AD0A            call 0x47d0
00003D23  7383              jnc 0x3ca8
00003D25  E89CF8            call 0x35c4
00003D28  E971FF            jmp 0x3c9c
00003D2B  C3                ret
00003D2C  B10E              mov cl,0xe
00003D2E  E89FD5            call 0x12d0
00003D31  E879E6            call 0x23ad
00003D34  91                xchg ax,cx
00003D35  BEC8B1            mov si,0xb1c8
00003D38  AD                lodsw
00003D39  2408              and al,0x8
00003D3B  A2A1BD            mov [0xbda1],al
00003D3E  AD                lodsw
00003D3F  2D0700            sub ax,0x7
00003D42  03C1              add ax,cx
00003D44  83D200            adc dx,0x0
00003D47  A364BD            mov [0xbd64],ax
00003D4A  891666BD          mov [0xbd66],dx
00003D4E  92                xchg ax,dx
00003D4F  91                xchg ax,cx
00003D50  E88EE6            call 0x23e1
00003D53  B120              mov cl,0x20
00003D55  E878D5            call 0x12d0
00003D58  3C20              cmp al,0x20
00003D5A  75CF              jnz 0x3d2b
00003D5C  E885F8            call 0x35e4
00003D5F  BEC0B1            mov si,0xb1c0
00003D62  AC                lodsb
00003D63  93                xchg ax,bx
00003D64  AD                lodsw
00003D65  92                xchg ax,dx
00003D66  AD                lodsw
00003D67  80FB74            cmp bl,0x74
00003D6A  7405              jz 0x3d71
00003D6C  E844F8            call 0x35b3
00003D6F  EBDF              jmp 0x3d50
00003D71  010664BD          add [0xbd64],ax
00003D75  831666BD00        adc word [0xbd66],0x0
00003D7A  BFD1BC            mov di,0xbcd1
00003D7D  A5                movsw
00003D7E  A5                movsw
00003D7F  A5                movsw
00003D80  A5                movsw
00003D81  46                inc si
00003D82  AD                lodsw
00003D83  93                xchg ax,bx
00003D84  AD                lodsw
00003D85  91                xchg ax,cx
00003D86  A5                movsw
00003D87  A5                movsw
00003D88  93                xchg ax,bx
00003D89  AB                stosw
00003D8A  91                xchg ax,cx
00003D8B  AB                stosw
00003D8C  AC                lodsb
00003D8D  33DB              xor bx,bx
00003D8F  A2E7BC            mov [0xbce7],al
00003D92  3C14              cmp al,0x14
00003D94  7716              ja 0x3dac
00003D96  720F              jc 0x3da7
00003D98  8BC2              mov ax,dx
00003D9A  B105              mov cl,0x5
00003D9C  D2E8              shr al,cl
00003D9E  B90100            mov cx,0x1
00003DA1  91                xchg ax,cx
00003DA2  D3E0              shl ax,cl
00003DA4  48                dec ax
00003DA5  8BD8              mov bx,ax
00003DA7  C606CABC80        mov byte [0xbcca],0x80
00003DAC  AC                lodsb
00003DAD  2C30              sub al,0x30
00003DAF  A2E6BC            mov [0xbce6],al
00003DB2  AD                lodsw
00003DB3  91                xchg ax,cx
00003DB4  AC                lodsb
00003DB5  A2E1BC            mov [0xbce1],al
00003DB8  2418              and al,0x18
00003DBA  750A              jnz 0x3dc6
00003DBC  3B1EBCBD          cmp bx,[0xbdbc]
00003DC0  7604              jna 0x3dc6
00003DC2  891EBCBD          mov [0xbdbc],bx
00003DC6  F6C203            test dl,0x3
00003DC9  7402              jz 0x3dcd
00003DCB  0C40              or al,0x40
00003DCD  F6C208            test dl,0x8
00003DD0  7400              jz 0x3dd2
00003DD2  F6C210            test dl,0x10
00003DD5  7402              jz 0x3dd9
00003DD7  0C20              or al,0x20
00003DD9  80E204            and dl,0x4
00003DDC  0AC2              or al,dl
00003DDE  0806CABC          or [0xbcca],al
00003DE2  E8EBD4            call 0x12d0
00003DE5  BF1DBB            mov di,0xbb1d
00003DE8  E8E109            call 0x47cc
00003DEB  731D              jnc 0x3e0a
00003DED  E8D4F7            call 0x35c4
00003DF0  E95DFF            jmp 0x3d50
00003DF3  33C0              xor ax,ax
00003DF5  8BD0              mov dx,ax
00003DF7  A25FBD            mov [0xbd5f],al
00003DFA  A364BD            mov [0xbd64],ax
00003DFD  891666BD          mov [0xbd66],dx
00003E01  B104              mov cl,0x4
00003E03  E8CAD4            call 0x12d0
00003E06  0BC0              or ax,ax
00003E08  7501              jnz 0x3e0b
00003E0A  C3                ret
00003E0B  E8D6F7            call 0x35e4
00003E0E  AD                lodsw
00003E0F  3D5343            cmp ax,0x4353
00003E12  7403              jz 0x3e17
00003E14  E9E500            jmp 0x3efc
00003E17  46                inc si
00003E18  AC                lodsb
00003E19  3C64              cmp al,0x64
00003E1B  7509              jnz 0x3e26
00003E1D  FE0E5FBD          dec byte [0xbd5f]
00003E21  E889E5            call 0x23ad
00003E24  EBD4              jmp 0x3dfa
00003E26  3C44              cmp al,0x44
00003E28  7526              jnz 0x3e50
00003E2A  FE065FBD          inc byte [0xbd5f]
00003E2E  E899D4            call 0x12ca
00003E31  A05FBD            mov al,[0xbd5f]
00003E34  48                dec ax
00003E35  B241              mov dl,0x41
00003E37  F6E2              mul dl
00003E39  50                push ax
00003E3A  8A4405            mov al,[si+0x5]
00003E3D  50                push ax
00003E3E  91                xchg ax,cx
00003E3F  E88ED4            call 0x12d0
00003E42  8E0632BD          mov es,word [0xbd32]
00003E46  58                pop ax
00003E47  59                pop cx
00003E48  8BF9              mov di,cx
00003E4A  AA                stosb
00003E4B  91                xchg ax,cx
00003E4C  F3A4              rep movsb
00003E4E  EBD1              jmp 0x3e21
00003E50  3C46              cmp al,0x46
00003E52  75B6              jnz 0x3e0a
00003E54  B114              mov cl,0x14
00003E56  E877D4            call 0x12d0
00003E59  BFD1BC            mov di,0xbcd1
00003E5C  A5                movsw
00003E5D  A5                movsw
00003E5E  A5                movsw
00003E5F  A5                movsw
00003E60  AD                lodsw
00003E61  AD                lodsw
00003E62  A5                movsw
00003E63  A5                movsw
00003E64  AD                lodsw
00003E65  AD                lodsw
00003E66  50                push ax
00003E67  50                push ax
00003E68  BF1EBB            mov di,0xbb1e
00003E6B  32D2              xor dl,dl
00003E6D  3A165FBD          cmp dl,[0xbd5f]
00003E71  7419              jz 0x3e8c
00003E73  1E                push ds
00003E74  8E1E32BD          mov ds,word [0xbd32]
00003E78  33F6              xor si,si
00003E7A  B041              mov al,0x41
00003E7C  F6E2              mul dl
00003E7E  03F0              add si,ax
00003E80  AC                lodsb
00003E81  98                cbw
00003E82  91                xchg ax,cx
00003E83  F3A4              rep movsb
00003E85  B05C              mov al,0x5c
00003E87  AA                stosb
00003E88  42                inc dx
00003E89  1F                pop ds
00003E8A  EBE1              jmp 0x3e6d
00003E8C  97                xchg ax,di
00003E8D  59                pop cx
00003E8E  50                push ax
00003E8F  2D1EBB            sub ax,0xbb1e
00003E92  03C1              add ax,cx
00003E94  A21DBB            mov [0xbb1d],al
00003E97  E838D4            call 0x12d2
00003E9A  5F                pop di
00003E9B  59                pop cx
00003E9C  E82F09            call 0x47ce
00003E9F  7337              jnc 0x3ed8
00003EA1  E820F7            call 0x35c4
00003EA4  E83AE5            call 0x23e1
00003EA7  E957FF            jmp 0x3e01
00003EAA  E834E5            call 0x23e1
00003EAD  E82DF7            call 0x35dd
00003EB0  B16C              mov cl,0x6c
00003EB2  E81BD4            call 0x12d0
00003EB5  BFD1BC            mov di,0xbcd1
00003EB8  92                xchg ax,dx
00003EB9  A0A0BD            mov al,[0xbda0]
00003EBC  0BD2              or dx,dx
00003EBE  7418              jz 0x3ed8
00003EC0  3C0C              cmp al,0xc
00003EC2  7404              jz 0x3ec8
00003EC4  3C07              cmp al,0x7
00003EC6  7507              jnz 0x3ecf
00003EC8  83FA1A            cmp dx,0x1a
00003ECB  7C0B              jl 0x3ed8
00003ECD  EB13              jmp 0x3ee2
00003ECF  3C02              cmp al,0x2
00003ED1  772C              ja 0x3eff
00003ED3  AD                lodsw
00003ED4  0AC0              or al,al
00003ED6  752D              jnz 0x3f05
00003ED8  C3                ret
00003ED9  AD                lodsw
00003EDA  AC                lodsb
00003EDB  3CAE              cmp al,0xae
00003EDD  751D              jnz 0x3efc
00003EDF  E9CA00            jmp 0x3fac
00003EE2  3C07              cmp al,0x7
00003EE4  75F3              jnz 0x3ed9
00003EE6  AD                lodsw
00003EE7  3D4F72            cmp ax,0x724f
00003EEA  740D              jz 0x3ef9
00003EEC  B002              mov al,0x2
00003EEE  E8C0F6            call 0x35b1
00003EF1  1E                push ds
00003EF2  07                pop es
00003EF3  AD                lodsw
00003EF4  3D4F72            cmp ax,0x724f
00003EF7  7503              jnz 0x3efc
00003EF9  E98700            jmp 0x3f83
00003EFC  E9DCC4            jmp 0x3db
00003EFF  AC                lodsb
00003F00  3C1A              cmp al,0x1a
00003F02  AD                lodsw
00003F03  EBD8              jmp 0x3edd
00003F05  AD                lodsw
00003F06  3C2D              cmp al,0x2d
00003F08  75F2              jnz 0x3efc
00003F0A  AD                lodsw
00003F0B  3C68              cmp al,0x68
00003F0D  750D              jnz 0x3f1c
00003F0F  80FC64            cmp ah,0x64
00003F12  7502              jnz 0x3f16
00003F14  B430              mov ah,0x30
00003F16  8AC4              mov al,ah
00003F18  2C30              sub al,0x30
00003F1A  EB0C              jmp 0x3f28
00003F1C  3D7A35            cmp ax,0x357a
00003F1F  7407              jz 0x3f28
00003F21  3D7A34            cmp ax,0x347a
00003F24  751A              jnz 0x3f40
00003F26  B000              mov al,0x0
00003F28  A2E6BC            mov [0xbce6],al
00003F2B  3C7A              cmp al,0x7a
00003F2D  740C              jz 0x3f3b
00003F2F  3C02              cmp al,0x2
00003F31  7208              jc 0x3f3b
00003F33  3C04              cmp al,0x4
00003F35  7209              jc 0x3f40
00003F37  3C07              cmp al,0x7
00003F39  7305              jnc 0x3f40
00003F3B  800ECABC80        or byte [0xbcca],0x80
00003F40  803EA0BD01        cmp byte [0xbda0],0x1
00003F45  7501              jnz 0x3f48
00003F47  46                inc si
00003F48  46                inc si
00003F49  B106              mov cl,0x6
00003F4B  F3A5              rep movsw
00003F4D  AD                lodsw
00003F4E  AB                stosw
00003F4F  AA                stosb
00003F50  80FC02            cmp ah,0x2
00003F53  7443              jz 0x3f98
00003F55  56                push si
00003F56  BF1DBB            mov di,0xbb1d
00003F59  E8C6CF            call 0xf22
00003F5C  5E                pop si
00003F5D  AC                lodsb
00003F5E  98                cbw
00003F5F  03F0              add si,ax
00003F61  AD                lodsw
00003F62  803EA0BD01        cmp byte [0xbda0],0x1
00003F67  7502              jnz 0x3f6b
00003F69  32E4              xor ah,ah
00003F6B  A3DDBC            mov [0xbcdd],ax
00003F6E  A0BEB1            mov al,[0xb1be]
00003F71  803ED2B101        cmp byte [0xb1d2],0x1
00003F76  7505              jnz 0x3f7d
00003F78  50                push ax
00003F79  E80BCF            call 0xe87
00003F7C  58                pop ax
00003F7D  32E4              xor ah,ah
00003F7F  40                inc ax
00003F80  40                inc ax
00003F81  EB24              jmp 0x3fa7
00003F83  AD                lodsw
00003F84  BFD5BC            mov di,0xbcd5
00003F87  A5                movsw
00003F88  A5                movsw
00003F89  A5                movsw
00003F8A  A5                movsw
00003F8B  83C603            add si,0x3
00003F8E  BFD1BC            mov di,0xbcd1
00003F91  A5                movsw
00003F92  A5                movsw
00003F93  83C607            add si,0x7
00003F96  EB39              jmp 0x3fd1
00003F98  AD                lodsw
00003F99  A3DDBC            mov [0xbcdd],ax
00003F9C  B018              mov al,0x18
00003F9E  E8E6CE            call 0xe87
00003FA1  E81BDC            call 0x1bbf
00003FA4  A1BEB1            mov ax,[0xb1be]
00003FA7  E809F6            call 0x35b3
00003FAA  EB53              jmp 0x3fff
00003FAC  AC                lodsb
00003FAD  A2E6BC            mov [0xbce6],al
00003FB0  3C07              cmp al,0x7
00003FB2  7408              jz 0x3fbc
00003FB4  3C03              cmp al,0x3
00003FB6  7709              ja 0x3fc1
00003FB8  3C01              cmp al,0x1
00003FBA  7405              jz 0x3fc1
00003FBC  800ECABC80        or byte [0xbcca],0x80
00003FC1  B107              mov cl,0x7
00003FC3  F3A5              rep movsw
00003FC5  A4                movsb
00003FC6  BED4B1            mov si,0xb1d4
00003FC9  A0A0BD            mov al,[0xbda0]
00003FCC  3C09              cmp al,0x9
00003FCE  7501              jnz 0x3fd1
00003FD0  4E                dec si
00003FD1  BF1DBB            mov di,0xbb1d
00003FD4  E849CF            call 0xf20
00003FD7  A0A0BD            mov al,[0xbda0]
00003FDA  3C0C              cmp al,0xc
00003FDC  7414              jz 0x3ff2
00003FDE  A01DBB            mov al,[0xbb1d]
00003FE1  E8CDF5            call 0x35b1
00003FE4  803EA0BD09        cmp byte [0xbda0],0x9
00003FE9  7305              jnc 0x3ff0
00003FEB  B81B00            mov ax,0x1b
00003FEE  EB0C              jmp 0x3ffc
00003FF0  7407              jz 0x3ff9
00003FF2  A1BEB1            mov ax,[0xb1be]
00003FF5  40                inc ax
00003FF6  40                inc ax
00003FF7  EB03              jmp 0x3ffc
00003FF9  B81600            mov ax,0x16
00003FFC  E8B4F5            call 0x35b3
00003FFF  E8CE07            call 0x47d0
00004002  7306              jnc 0x400a
00004004  E8BDF5            call 0x35c4
00004007  E9A0FE            jmp 0x3eaa
0000400A  C3                ret
0000400B  91                xchg ax,cx
0000400C  92                xchg ax,dx
0000400D  050400            add ax,0x4
00004010  83D200            adc dx,0x0
00004013  E8C7F5            call 0x35dd
00004016  B11F              mov cl,0x1f
00004018  E8B5D2            call 0x12d0
0000401B  0BC0              or ax,ax
0000401D  74EB              jz 0x400a
0000401F  AD                lodsw
00004020  3D60EA            cmp ax,0xea60
00004023  7403              jz 0x4028
00004025  E9D4FE            jmp 0x3efc
00004028  AD                lodsw
00004029  A3C8BC            mov [0xbcc8],ax
0000402C  0BC0              or ax,ax
0000402E  74DA              jz 0x400a
00004030  AC                lodsb
00004031  32E4              xor ah,ah
00004033  C41664BD          les dx,word [0xbd64]
00004037  8CC1              mov cx,es
00004039  03D0              add dx,ax
0000403B  83D100            adc cx,0x0
0000403E  E8A0E3            call 0x23e1
00004041  1E                push ds
00004042  07                pop es
00004043  BFCDBC            mov di,0xbccd
00004046  AB                stosw
00004047  92                xchg ax,dx
00004048  AB                stosw
00004049  BEC8B1            mov si,0xb1c8
0000404C  AD                lodsw
0000404D  50                push ax
0000404E  A0C7B1            mov al,[0xb1c7]
00004051  3C04              cmp al,0x4
00004053  7705              ja 0x405a
00004055  800ECABC80        or byte [0xbcca],0x80
0000405A  A3E6BC            mov [0xbce6],ax
0000405D  AD                lodsw
0000405E  92                xchg ax,dx
0000405F  AD                lodsw
00004060  93                xchg ax,bx
00004061  A5                movsw
00004062  A5                movsw
00004063  A5                movsw
00004064  A5                movsw
00004065  92                xchg ax,dx
00004066  AB                stosw
00004067  93                xchg ax,bx
00004068  AB                stosw
00004069  A5                movsw
0000406A  A5                movsw
0000406B  AD                lodsw
0000406C  A4                movsb
0000406D  32C9              xor cl,cl
0000406F  BEC6B1            mov si,0xb1c6
00004072  AD                lodsw
00004073  A801              test al,0x1
00004075  7402              jz 0x4079
00004077  B104              mov cl,0x4
00004079  240C              and al,0xc
0000407B  7403              jz 0x4080
0000407D  80C940            or cl,0x40
00004080  AC                lodsb
00004081  D0E8              shr al,0x0
00004083  7303              jnc 0x4088
00004085  80C920            or cl,0x20
00004088  A0DCB1            mov al,[0xb1dc]
0000408B  2418              and al,0x18
0000408D  02C1              add al,cl
0000408F  0806CABC          or [0xbcca],al
00004093  B150              mov cl,0x50
00004095  E838D2            call 0x12d0
00004098  BF1EBB            mov di,0xbb1e
0000409B  AC                lodsb
0000409C  0AC0              or al,al
0000409E  7404              jz 0x40a4
000040A0  AA                stosb
000040A1  41                inc cx
000040A2  EBF7              jmp 0x409b
000040A4  880E1DBB          mov [0xbb1d],cl
000040A8  41                inc cx
000040A9  51                push cx
000040AA  A1C8BC            mov ax,[0xbcc8]
000040AD  050600            add ax,0x6
000040B0  E800F5            call 0x35b3
000040B3  59                pop cx
000040B4  AC                lodsb
000040B5  0AC0              or al,al
000040B7  740D              jz 0x40c6
000040B9  C406CDBC          les ax,word [0xbccd]
000040BD  8CC2              mov dx,es
000040BF  03C1              add ax,cx
000040C1  83D200            adc dx,0x0
000040C4  EB04              jmp 0x40ca
000040C6  33C0              xor ax,ax
000040C8  8BD0              mov dx,ax
000040CA  5B                pop bx
000040CB  80FB02            cmp bl,0x2
000040CE  750F              jnz 0x40df
000040D0  A370BD            mov [0xbd70],ax
000040D3  891672BD          mov [0xbd72],dx
000040D7  C41664BD          les dx,word [0xbd64]
000040DB  8CC1              mov cx,es
000040DD  EB10              jmp 0x40ef
000040DF  A3CDBC            mov [0xbccd],ax
000040E2  8916CFBC          mov [0xbccf],dx
000040E6  E8E706            call 0x47d0
000040E9  7201              jc 0x40ec
000040EB  C3                ret
000040EC  E8D5F4            call 0x35c4
000040EF  E8EFE2            call 0x23e1
000040F2  E918FF            jmp 0x400d
000040F5  E849CF            call 0x1041
000040F8  FC                cld
000040F9  0BD2              or dx,dx
000040FB  750D              jnz 0x410a
000040FD  3D0008            cmp ax,0x800
00004100  7308              jnc 0x410a
00004102  0BC0              or ax,ax
00004104  743C              jz 0x4142
00004106  33C0              xor ax,ax
00004108  EB06              jmp 0x4110
0000410A  2D0008            sub ax,0x800
0000410D  83DA00            sbb dx,0x0
00004110  92                xchg ax,dx
00004111  91                xchg ax,cx
00004112  E8CCE2            call 0x23e1
00004115  A364BD            mov [0xbd64],ax
00004118  891666BD          mov [0xbd66],dx
0000411C  B90008            mov cx,0x800
0000411F  E8B0D1            call 0x12d2
00004122  03F0              add si,ax
00004124  4E                dec si
00004125  4E                dec si
00004126  FD                std
00004127  AD                lodsw
00004128  81FEBEB1          cmp si,0xb1be
0000412C  7F18              jg 0x4146
0000412E  C40664BD          les ax,word [0xbd64]
00004132  8CC2              mov dx,es
00004134  8BC8              mov cx,ax
00004136  0BCA              or cx,dx
00004138  7408              jz 0x4142
0000413A  053200            add ax,0x32
0000413D  83D200            adc dx,0x0
00004140  EBB6              jmp 0x40f8
00004142  FC                cld
00004143  E995C2            jmp 0x3db
00004146  3D0506            cmp ax,0x605
00004149  7407              jz 0x4152
0000414B  3C06              cmp al,0x6
0000414D  75D8              jnz 0x4127
0000414F  46                inc si
00004150  EBD5              jmp 0x4127
00004152  AD                lodsw
00004153  3D504B            cmp ax,0x4b50
00004156  75EE              jnz 0x4146
00004158  FC                cld
00004159  83C612            add si,0x12
0000415C  AD                lodsw
0000415D  A374BD            mov [0xbd74],ax
00004160  92                xchg ax,dx
00004161  AD                lodsw
00004162  A376BD            mov [0xbd76],ax
00004165  91                xchg ax,cx
00004166  AD                lodsw
00004167  0BC0              or ax,ax
00004169  7416              jz 0x4181
0000416B  C40664BD          les ax,word [0xbd64]
0000416F  8CC3              mov bx,es
00004171  81EEC0B1          sub si,0xb1c0
00004175  03C6              add ax,si
00004177  83D300            adc bx,0x0
0000417A  A370BD            mov [0xbd70],ax
0000417D  891E72BD          mov [0xbd72],bx
00004181  E85DE2            call 0x23e1
00004184  051C00            add ax,0x1c
00004187  83D200            adc dx,0x0
0000418A  E850F4            call 0x35dd
0000418D  B182              mov cl,0x82
0000418F  E83ED1            call 0x12d0
00004192  BFD9BC            mov di,0xbcd9
00004195  AD                lodsw
00004196  3D504B            cmp ax,0x4b50
00004199  75A7              jnz 0x4142
0000419B  AD                lodsw
0000419C  3D0102            cmp ax,0x201
0000419F  7406              jz 0x41a7
000041A1  3D0506            cmp ax,0x605
000041A4  759C              jnz 0x4142
000041A6  C3                ret
000041A7  AD                lodsw
000041A8  93                xchg ax,bx
000041A9  AD                lodsw
000041AA  AD                lodsw
000041AB  A801              test al,0x1
000041AD  7405              jz 0x41b4
000041AF  C606CABC04        mov byte [0xbcca],0x4
000041B4  91                xchg ax,cx
000041B5  AD                lodsw
000041B6  8AE1              mov ah,cl
000041B8  A3E6BC            mov [0xbce6],ax
000041BB  3C02              cmp al,0x2
000041BD  7208              jc 0x41c7
000041BF  3C06              cmp al,0x6
000041C1  7404              jz 0x41c7
000041C3  3C08              cmp al,0x8
000041C5  7505              jnz 0x41cc
000041C7  800ECABC80        or byte [0xbcca],0x80
000041CC  A5                movsw
000041CD  A5                movsw
000041CE  A5                movsw
000041CF  A5                movsw
000041D0  BFD1BC            mov di,0xbcd1
000041D3  A5                movsw
000041D4  A5                movsw
000041D5  A5                movsw
000041D6  A5                movsw
000041D7  AD                lodsw
000041D8  91                xchg ax,cx
000041D9  AD                lodsw
000041DA  92                xchg ax,dx
000041DB  03D1              add dx,cx
000041DD  AD                lodsw
000041DE  03D0              add dx,ax
000041E0  83C212            add dx,0x12
000041E3  52                push dx
000041E4  0BC0              or ax,ax
000041E6  740B              jz 0x41f3
000041E8  BFCDBC            mov di,0xbccd
000041EB  A164BD            mov ax,[0xbd64]
000041EE  AB                stosw
000041EF  A166BD            mov ax,[0xbd66]
000041F2  AB                stosw
000041F3  AD                lodsw
000041F4  AD                lodsw
000041F5  8A1ECABC          mov bl,[0xbcca]
000041F9  D1E8              shr ax,0x0
000041FB  7303              jnc 0x4200
000041FD  80CB20            or bl,0x20
00004200  AD                lodsw
00004201  A2E1BC            mov [0xbce1],al
00004204  2418              and al,0x18
00004206  02C3              add al,bl
00004208  0806CABC          or [0xbcca],al
0000420C  AD                lodsw
0000420D  AD                lodsw
0000420E  A3E2BC            mov [0xbce2],ax
00004211  AD                lodsw
00004212  A3E4BC            mov [0xbce4],ax
00004215  8AC1              mov al,cl
00004217  BF1DBB            mov di,0xbb1d
0000421A  AA                stosb
0000421B  F3A4              rep movsb
0000421D  E8BC05            call 0x47dc
00004220  58                pop ax
00004221  7306              jnc 0x4229
00004223  E88DF3            call 0x35b3
00004226  E958FF            jmp 0x4181
00004229  C3                ret
0000422A  33C9              xor cx,cx
0000422C  BA0300            mov dx,0x3
0000422F  E8AFE1            call 0x23e1
00004232  051400            add ax,0x14
00004235  83D200            adc dx,0x0
00004238  E8A2F3            call 0x35dd
0000423B  C606CABC80        mov byte [0xbcca],0x80
00004240  B114              mov cl,0x14
00004242  E88BD0            call 0x12d0
00004245  3C14              cmp al,0x14
00004247  7565              jnz 0x42ae
00004249  AC                lodsb
0000424A  3C1A              cmp al,0x1a
0000424C  7560              jnz 0x42ae
0000424E  BF1EBB            mov di,0xbb1e
00004251  57                push di
00004252  A5                movsw
00004253  A5                movsw
00004254  A5                movsw
00004255  A5                movsw
00004256  BF9DBA            mov di,0xba9d
00004259  A5                movsw
0000425A  A4                movsb
0000425B  AD                lodsw
0000425C  92                xchg ax,dx
0000425D  AC                lodsb
0000425E  32E4              xor ah,ah
00004260  8BD8              mov bx,ax
00004262  BFD1BC            mov di,0xbcd1
00004265  B90400            mov cx,0x4
00004268  92                xchg ax,dx
00004269  AB                stosw
0000426A  E2FC              loop 0x4268
0000426C  A5                movsw
0000426D  A5                movsw
0000426E  5F                pop di
0000426F  1E                push ds
00004270  07                pop es
00004271  B90900            mov cx,0x9
00004274  32C0              xor al,al
00004276  F2AE              repne scasb
00004278  B008              mov al,0x8
0000427A  2AC1              sub al,cl
0000427C  A21DBB            mov [0xbb1d],al
0000427F  B104              mov cl,0x4
00004281  BF9DBA            mov di,0xba9d
00004284  32C0              xor al,al
00004286  F2AE              repne scasb
00004288  B003              mov al,0x3
0000428A  2AC1              sub al,cl
0000428C  A29CBA            mov [0xba9c],al
0000428F  0AC0              or al,al
00004291  7411              jz 0x42a4
00004293  BF1DBB            mov di,0xbb1d
00004296  BE32B0            mov si,0xb032
00004299  57                push di
0000429A  E89FCC            call 0xf3c
0000429D  5F                pop di
0000429E  BE9CBA            mov si,0xba9c
000042A1  E89ACC            call 0xf3e
000042A4  E82905            call 0x47d0
000042A7  7305              jnc 0x42ae
000042A9  E818F3            call 0x35c4
000042AC  EB81              jmp 0x422f
000042AE  C3                ret
000042AF  2EC606D46804      mov byte [cs:0x68d4],0x4
000042B5  33C9              xor cx,cx
000042B7  8BD1              mov dx,cx
000042B9  E825E1            call 0x23e1
000042BC  B12A              mov cl,0x2a
000042BE  E80FD0            call 0x12d0
000042C1  BA2A00            mov dx,0x2a
000042C4  833EE5B100        cmp word [0xb1e5],0x0
000042C9  740A              jz 0x42d5
000042CB  C70670BD2300      mov word [0xbd70],0x23
000042D1  890E72BD          mov [0xbd72],cx
000042D5  891664BD          mov [0xbd64],dx
000042D9  890E66BD          mov [0xbd66],cx
000042DD  E801E1            call 0x23e1
000042E0  B126              mov cl,0x26
000042E2  E8EBCF            call 0x12d0
000042E5  0BC0              or ax,ax
000042E7  74C5              jz 0x42ae
000042E9  E8F8F2            call 0x35e4
000042EC  AD                lodsw
000042ED  3DDCA7            cmp ax,0xa7dc
000042F0  75BC              jnz 0x42ae
000042F2  AD                lodsw
000042F3  3DC4FD            cmp ax,0xfdc4
000042F6  75B6              jnz 0x42ae
000042F8  AD                lodsw
000042F9  80FC02            cmp ah,0x2
000042FC  7705              ja 0x4303
000042FE  C606CABC80        mov byte [0xbcca],0x80
00004303  8826E6BC          mov [0xbce6],ah
00004307  C606E1BC00        mov byte [0xbce1],0x0
0000430C  AD                lodsw
0000430D  92                xchg ax,dx
0000430E  AD                lodsw
0000430F  891664BD          mov [0xbd64],dx
00004313  A366BD            mov [0xbd66],ax
00004316  0BC2              or ax,dx
00004318  7494              jz 0x42ae
0000431A  AD                lodsw
0000431B  A3E2BC            mov [0xbce2],ax
0000431E  AD                lodsw
0000431F  A3E4BC            mov [0xbce4],ax
00004322  AD                lodsw
00004323  A3DBBC            mov [0xbcdb],ax
00004326  AD                lodsw
00004327  A3D9BC            mov [0xbcd9],ax
0000432A  AD                lodsw
0000432B  A3DDBC            mov [0xbcdd],ax
0000432E  BFD5BC            mov di,0xbcd5
00004331  A5                movsw
00004332  A5                movsw
00004333  BFD1BC            mov di,0xbcd1
00004336  A5                movsw
00004337  A5                movsw
00004338  A0DCB1            mov al,[0xb1dc]
0000433B  3C01              cmp al,0x1
0000433D  7505              jnz 0x4344
0000433F  800ECABC10        or byte [0xbcca],0x10
00004344  A1E2B1            mov ax,[0xb1e2]
00004347  0BC0              or ax,ax
00004349  7410              jz 0x435b
0000434B  E85FE0            call 0x23ad
0000434E  2D0600            sub ax,0x6
00004351  83DA00            sbb dx,0x0
00004354  A3CDBC            mov [0xbccd],ax
00004357  8916CFBC          mov [0xbccf],dx
0000435B  B114              mov cl,0x14
0000435D  E870CF            call 0x12d0
00004360  BF1EBB            mov di,0xbb1e
00004363  AC                lodsb
00004364  0AC0              or al,al
00004366  7404              jz 0x436c
00004368  41                inc cx
00004369  AA                stosb
0000436A  EBF7              jmp 0x4363
0000436C  880E1DBB          mov [0xbb1d],cl
00004370  A0D1B1            mov al,[0xb1d1]
00004373  0AC0              or al,al
00004375  741D              jz 0x4394
00004377  BF9CBA            mov di,0xba9c
0000437A  AA                stosb
0000437B  98                cbw
0000437C  91                xchg ax,cx
0000437D  49                dec cx
0000437E  51                push cx
0000437F  E850CF            call 0x12d2
00004382  59                pop cx
00004383  BF9DBA            mov di,0xba9d
00004386  F3A4              rep movsb
00004388  B02F              mov al,0x2f
0000438A  AA                stosb
0000438B  BE1DBB            mov si,0xbb1d
0000438E  E8A8CB            call 0xf39
00004391  E886CB            call 0xf1a
00004394  E84504            call 0x47dc
00004397  7201              jc 0x439a
00004399  C3                ret
0000439A  C41664BD          les dx,word [0xbd64]
0000439E  8CC1              mov cx,es
000043A0  E93AFF            jmp 0x42dd
000043A3  B8FF01            mov ax,0x1ff
000043A6  2EA39268          mov [cs:0x6892],ax
000043AA  2EA3496A          mov [cs:0x6a49],ax
000043AE  2EC606B7680E      mov byte [cs:0x68b7],0xe
000043B4  2EC706BF681392    mov word [cs:0x68bf],0x9213
000043BB  2EC706D0681F92    mov word [cs:0x68d0],0x921f
000043C2  B85E68            mov ax,0x685e
000043C5  2D3668            sub ax,0x6836
000043C8  8AE0              mov ah,al
000043CA  B0EB              mov al,0xeb
000043CC  2EA33468          mov [cs:0x6834],ax
000043D0  2EC6062A6A07      mov byte [cs:0x6a2a],0x7
000043D6  2EC6062A681F      mov byte [cs:0x682a],0x1f
000043DC  83C208            add dx,0x8
000043DF  83D100            adc cx,0x0
000043E2  891664BD          mov [0xbd64],dx
000043E6  890E66BD          mov [0xbd66],cx
000043EA  E8F4DF            call 0x23e1
000043ED  B101              mov cl,0x1
000043EF  E8DECE            call 0x12d0
000043F2  AC                lodsb
000043F3  0AC0              or al,al
000043F5  74A2              jz 0x4399
000043F7  3C13              cmp al,0x13
000043F9  731E              jnc 0x4419
000043FB  50                push ax
000043FC  B104              mov cl,0x4
000043FE  E8CFCE            call 0x12d0
00004401  58                pop ax
00004402  3C01              cmp al,0x1
00004404  7508              jnz 0x440e
00004406  A1C0B1            mov ax,[0xb1c0]
00004409  050A00            add ax,0xa
0000440C  EB04              jmp 0x4412
0000440E  AD                lodsw
0000440F  050300            add ax,0x3
00004412  33DB              xor bx,bx
00004414  E8B3F1            call 0x35ca
00004417  EBD1              jmp 0x43ea
00004419  32E4              xor ah,ah
0000441B  91                xchg ax,cx
0000441C  41                inc cx
0000441D  51                push cx
0000441E  E8B1CE            call 0x12d2
00004421  93                xchg ax,bx
00004422  59                pop cx
00004423  3BD9              cmp bx,cx
00004425  7548              jnz 0x446f
00004427  51                push cx
00004428  C40664BD          les ax,word [0xbd64]
0000442C  8CC3              mov bx,es
0000442E  41                inc cx
0000442F  03C1              add ax,cx
00004431  83D300            adc bx,0x0
00004434  53                push bx
00004435  50                push ax
00004436  91                xchg ax,cx
00004437  E879F1            call 0x35b3
0000443A  E8A7F1            call 0x35e4
0000443D  58                pop ax
0000443E  A3E2BC            mov [0xbce2],ax
00004441  58                pop ax
00004442  A3E4BC            mov [0xbce4],ax
00004445  AC                lodsb
00004446  AC                lodsb
00004447  240F              and al,0xf
00004449  A2E6BC            mov [0xbce6],al
0000444C  3C04              cmp al,0x4
0000444E  7705              ja 0x4455
00004450  C606CABC80        mov byte [0xbcca],0x80
00004455  BFD1BC            mov di,0xbcd1
00004458  A5                movsw
00004459  A5                movsw
0000445A  A5                movsw
0000445B  A5                movsw
0000445C  A5                movsw
0000445D  A5                movsw
0000445E  AC                lodsb
0000445F  A2E1BC            mov [0xbce1],al
00004462  A5                movsw
00004463  A5                movsw
00004464  BF1DBB            mov di,0xbb1d
00004467  58                pop ax
00004468  2C13              sub al,0x13
0000446A  E85F03            call 0x47cc
0000446D  7201              jc 0x4470
0000446F  C3                ret
00004470  C406D1BC          les ax,word [0xbcd1]
00004474  8CC3              mov bx,es
00004476  EB9C              jmp 0x4414
00004478  E866DF            call 0x23e1
0000447B  E85FF1            call 0x35dd
0000447E  B11D              mov cl,0x1d
00004480  E84DCE            call 0x12d0
00004483  50                push ax
00004484  AC                lodsb
00004485  3C1A              cmp al,0x1a
00004487  75E6              jnz 0x446f
00004489  AC                lodsb
0000448A  0AC0              or al,al
0000448C  7450              jz 0x44de
0000448E  2C02              sub al,0x2
00004490  A2E6BC            mov [0xbce6],al
00004493  3C03              cmp al,0x3
00004495  7208              jc 0x449f
00004497  3C06              cmp al,0x6
00004499  7404              jz 0x449f
0000449B  3C07              cmp al,0x7
0000449D  7505              jnz 0x44a4
0000449F  800ECABC80        or byte [0xbcca],0x80
000044A4  58                pop ax
000044A5  E80BF1            call 0x35b3
000044A8  1E                push ds
000044A9  07                pop es
000044AA  BF1EBB            mov di,0xbb1e
000044AD  57                push di
000044AE  B90D00            mov cx,0xd
000044B1  51                push cx
000044B2  F3A4              rep movsb
000044B4  59                pop cx
000044B5  5F                pop di
000044B6  32C0              xor al,al
000044B8  F2AE              repne scasb
000044BA  B00C              mov al,0xc
000044BC  2AC1              sub al,cl
000044BE  A21DBB            mov [0xbb1d],al
000044C1  BFD1BC            mov di,0xbcd1
000044C4  A5                movsw
000044C5  A5                movsw
000044C6  BFDBBC            mov di,0xbcdb
000044C9  A5                movsw
000044CA  AD                lodsw
000044CB  A3D9BC            mov [0xbcd9],ax
000044CE  A5                movsw
000044CF  BFD5BC            mov di,0xbcd5
000044D2  A5                movsw
000044D3  A5                movsw
000044D4  E8F902            call 0x47d0
000044D7  7396              jnc 0x446f
000044D9  E8E8F0            call 0x35c4
000044DC  EB9A              jmp 0x4478
000044DE  58                pop ax
000044DF  3C02              cmp al,0x2
000044E1  7463              jz 0x4546
000044E3  B002              mov al,0x2
000044E5  E8C9F0            call 0x35b1
000044E8  E8F6DE            call 0x23e1
000044EB  803EA0BD05        cmp byte [0xbda0],0x5
000044F0  7455              jz 0x4547
000044F2  A370BD            mov [0xbd70],ax
000044F5  891672BD          mov [0xbd72],dx
000044F9  B120              mov cl,0x20
000044FB  E8D2CD            call 0x12d0
000044FE  AC                lodsb
000044FF  3C01              cmp al,0x1
00004501  750A              jnz 0x450d
00004503  33C0              xor ax,ax
00004505  A370BD            mov [0xbd70],ax
00004508  A372BD            mov [0xbd72],ax
0000450B  EB0F              jmp 0x451c
0000450D  B120              mov cl,0x20
0000450F  E8BECD            call 0x12d0
00004512  AC                lodsb
00004513  3C01              cmp al,0x1
00004515  752F              jnz 0x4546
00004517  B020              mov al,0x20
00004519  E895F0            call 0x35b1
0000451C  33DB              xor bx,bx
0000451E  C40664BD          les ax,word [0xbd64]
00004522  8CC2              mov dx,es
00004524  8B3659BD          mov si,[0xbd59]
00004528  D1E6              shl si,0x0
0000452A  8E0636BD          mov es,word [0xbd36]
0000452E  268E07            mov es,word [es:bx]
00004531  33FF              xor di,di
00004533  052000            add ax,0x20
00004536  83D200            adc dx,0x0
00004539  BF0300            mov di,0x3
0000453C  AB                stosw
0000453D  92                xchg ax,dx
0000453E  AB                stosw
0000453F  92                xchg ax,dx
00004540  43                inc bx
00004541  43                inc bx
00004542  3BDE              cmp bx,si
00004544  7CE4              jl 0x452a
00004546  C3                ret
00004547  B108              mov cl,0x8
00004549  E884CD            call 0x12d0
0000454C  0BC0              or ax,ax
0000454E  74F6              jz 0x4546
00004550  AD                lodsw
00004551  0AE4              or ah,ah
00004553  74F1              jz 0x4546
00004555  3CFE              cmp al,0xfe
00004557  75ED              jnz 0x4546
00004559  AD                lodsw
0000455A  A35DBD            mov [0xbd5d],ax
0000455D  AD                lodsw
0000455E  A35FBD            mov [0xbd5f],ax
00004561  A0BFB1            mov al,[0xb1bf]
00004564  3C02              cmp al,0x2
00004566  7508              jnz 0x4570
00004568  8B0E5FBD          mov cx,[0xbd5f]
0000456C  0BC9              or cx,cx
0000456E  753A              jnz 0x45aa
00004570  E83ADE            call 0x23ad
00004573  50                push ax
00004574  52                push dx
00004575  91                xchg ax,cx
00004576  A0BEB1            mov al,[0xb1be]
00004579  3C01              cmp al,0x1
0000457B  751F              jnz 0x459c
0000457D  83E904            sub cx,0x4
00004580  83DA00            sbb dx,0x0
00004583  A15DBD            mov ax,[0xbd5d]
00004586  0BC0              or ax,ax
00004588  750A              jnz 0x4594
0000458A  890E70BD          mov [0xbd70],cx
0000458E  891672BD          mov [0xbd72],dx
00004592  EB08              jmp 0x459c
00004594  890ECDBC          mov [0xbccd],cx
00004598  8916CFBC          mov [0xbccf],dx
0000459C  59                pop cx
0000459D  5A                pop dx
0000459E  03165FBD          add dx,[0xbd5f]
000045A2  83D100            adc cx,0x0
000045A5  E839DE            call 0x23e1
000045A8  EB60              jmp 0x460a
000045AA  E825CD            call 0x12d2
000045AD  BF1DBB            mov di,0xbb1d
000045B0  1E                push ds
000045B1  07                pop es
000045B2  AA                stosb
000045B3  91                xchg ax,cx
000045B4  F3A4              rep movsb
000045B6  A15DBD            mov ax,[0xbd5d]
000045B9  E875DC            call 0x2231
000045BC  8BF7              mov si,di
000045BE  1E                push ds
000045BF  1E                push ds
000045C0  06                push es
000045C1  1F                pop ds
000045C2  07                pop es
000045C3  BFCABC            mov di,0xbcca
000045C6  B132              mov cl,0x32
000045C8  F3A5              rep movsw
000045CA  1F                pop ds
000045CB  A0E8BC            mov al,[0xbce8]
000045CE  02061DBB          add al,[0xbb1d]
000045D2  041F              add al,0x1f
000045D4  98                cbw
000045D5  8B3E5DBD          mov di,[0xbd5d]
000045D9  50                push ax
000045DA  57                push di
000045DB  A0E8BC            mov al,[0xbce8]
000045DE  041F              add al,0x1f
000045E0  98                cbw
000045E1  E8DABA            call 0xbe
000045E4  58                pop ax
000045E5  59                pop cx
000045E6  51                push cx
000045E7  E8BCBA            call 0xa6
000045EA  0BC0              or ax,ax
000045EC  741F              jz 0x460d
000045EE  BF1DBB            mov di,0xbb1d
000045F1  BEE8BC            mov si,0xbce8
000045F4  57                push di
000045F5  56                push si
000045F6  E843C9            call 0xf3c
000045F9  5F                pop di
000045FA  5E                pop si
000045FB  E824C9            call 0xf22
000045FE  BECABC            mov si,0xbcca
00004601  A15DBD            mov ax,[0xbd5d]
00004604  E82ADC            call 0x2231
00004607  59                pop cx
00004608  F3A4              rep movsb
0000460A  E93AFF            jmp 0x4547
0000460D  59                pop cx
0000460E  BF03A8            mov di,0xa803
00004611  BE08AA            mov si,0xaa08
00004614  E94DBF            jmp 0x564
00004617  0BC0              or ax,ax
00004619  55                push bp
0000461A  8BEC              mov bp,sp
0000461C  81ECA600          sub sp,0xa6
00004620  E88BC1            call 0x7ae
00004623  A0DAA3            mov al,[0xa3da]
00004626  A2B3BD            mov [0xbdb3],al
00004629  BA0500            mov dx,0x5
0000462C  BF70A8            mov di,0xa870
0000462F  E879C2            call 0x8ab
00004632  BA0F00            mov dx,0xf
00004635  BF7AA8            mov di,0xa87a
00004638  E870C2            call 0x8ab
0000463B  BA1A00            mov dx,0x1a
0000463E  BF75A8            mov di,0xa875
00004641  E867C2            call 0x8ab
00004644  BA2100            mov dx,0x21
00004647  BF83A8            mov di,0xa883
0000464A  E85EC2            call 0x8ab
0000464D  A0DCA3            mov al,[0xa3dc]
00004650  A2B3BD            mov [0xbdb3],al
00004653  BA0E00            mov dx,0xe
00004656  E84FC2            call 0x8a8
00004659  BA1700            mov dx,0x17
0000465C  E849C2            call 0x8a8
0000465F  BA2000            mov dx,0x20
00004662  E843C2            call 0x8a8
00004665  8D7ED0            lea di,[bp-0x30]
00004668  16                push ss
00004669  57                push di
0000466A  E884CB            call 0x11f1
0000466D  BF88A8            mov di,0xa888
00004670  E886D7            call 0x1df9
00004673  8D7EC0            lea di,[bp-0x40]
00004676  A159BD            mov ax,[0xbd59]
00004679  B104              mov cl,0x4
0000467B  16                push ss
0000467C  57                push di
0000467D  E87DCB            call 0x11fd
00004680  E87AD7            call 0x1dfd
00004683  BF93A8            mov di,0xa893
00004686  E870D7            call 0x1df9
00004689  5F                pop di
0000468A  07                pop es
0000468B  8A36C4BC          mov dh,[0xbcc4]
0000468F  B204              mov dl,0x4
00004691  FECE              dec dh
00004693  E817C2            call 0x8ad
00004696  A1C4BC            mov ax,[0xbcc4]
00004699  2D0300            sub ax,0x3
0000469C  3B0659BD          cmp ax,[0xbd59]
000046A0  7C03              jl 0x46a5
000046A2  A159BD            mov ax,[0xbd59]
000046A5  0BC0              or ax,ax
000046A7  7503              jnz 0x46ac
000046A9  E91B01            jmp 0x47c7
000046AC  48                dec ax
000046AD  8846FE            mov [bp-0x2],al
000046B0  32C0              xor al,al
000046B2  8846FF            mov [bp-0x1],al
000046B5  33D2              xor dx,dx
000046B7  40                inc ax
000046B8  8AF0              mov dh,al
000046BA  E81EC1            call 0x7db
000046BD  E850C0            call 0x710
000046C0  8A46FF            mov al,[bp-0x1]
000046C3  98                cbw
000046C4  E86BDB            call 0x2232
000046C7  897EFA            mov [bp-0x6],di
000046CA  8C46FC            mov word [bp-0x4],es
000046CD  C47EFA            les di,word [bp-0x6]
000046D0  BF1E00            mov di,0x1e
000046D3  06                push es
000046D4  57                push di
000046D5  8D76AA            lea si,[bp-0x56]
000046D8  16                push ss
000046D9  56                push si
000046DA  06                push es
000046DB  57                push di
000046DC  E8B8CA            call 0x1197
000046DF  E85EDB            call 0x2240
000046E2  7405              jz 0x46e9
000046E4  BEA5AF            mov si,0xafa5
000046E7  EB03              jmp 0x46ec
000046E9  BE6FAF            mov si,0xaf6f
000046EC  8D7EAA            lea di,[bp-0x56]
000046EF  16                push ss
000046F0  57                push di
000046F1  16                push ss
000046F2  57                push di
000046F3  E836C8            call 0xf2c
000046F6  8DBE5AFF          lea di,[bp-0xa6]
000046FA  16                push ss
000046FB  57                push di
000046FC  C47EFA            les di,word [bp-0x6]
000046FF  BF1E00            mov di,0x1e
00004702  06                push es
00004703  57                push di
00004704  E890CA            call 0x1197
00004707  E8F3D6            call 0x1dfd
0000470A  5E                pop si
0000470B  58                pop ax
0000470C  B20E              mov dl,0xe
0000470E  E80ACA            call 0x111b
00004711  BF31AF            mov di,0xaf31
00004714  E8E2D6            call 0x1df9
00004717  8DBE5AFF          lea di,[bp-0xa6]
0000471B  16                push ss
0000471C  57                push di
0000471D  C47EFA            les di,word [bp-0x6]
00004720  E8C6CA            call 0x11e9
00004723  E8D7D6            call 0x1dfd
00004726  BF31AF            mov di,0xaf31
00004729  E8CDD6            call 0x1df9
0000472C  8DBE5AFF          lea di,[bp-0xa6]
00004730  16                push ss
00004731  57                push di
00004732  C47EFA            les di,word [bp-0x6]
00004735  268B4511          mov ax,[es:di+0x11]
00004739  251F00            and ax,0x1f
0000473C  E832C3            call 0xa71
0000473F  E8BBD6            call 0x1dfd
00004742  E8ACD6            call 0x1df1
00004745  8DBE5AFF          lea di,[bp-0xa6]
00004749  16                push ss
0000474A  57                push di
0000474B  C47EFA            les di,word [bp-0x6]
0000474E  268B4511          mov ax,[es:di+0x11]
00004752  80E401            and ah,0x1
00004755  E815C3            call 0xa6d
00004758  E8A2D6            call 0x1dfd
0000475B  E893D6            call 0x1df1
0000475E  8DBE5AFF          lea di,[bp-0xa6]
00004762  16                push ss
00004763  57                push di
00004764  C47EFA            les di,word [bp-0x6]
00004767  268B4511          mov ax,[es:di+0x11]
0000476B  B109              mov cl,0x9
0000476D  D3E8              shr ax,cl
0000476F  055000            add ax,0x50
00004772  E8FCC2            call 0xa71
00004775  E885D6            call 0x1dfd
00004778  BF31AF            mov di,0xaf31
0000477B  E87BD6            call 0x1df9
0000477E  8DBE5AFF          lea di,[bp-0xa6]
00004782  16                push ss
00004783  57                push di
00004784  C47EFA            les di,word [bp-0x6]
00004787  268B450F          mov ax,[es:di+0xf]
0000478B  B10B              mov cl,0xb
0000478D  E8DFC2            call 0xa6f
00004790  E86AD6            call 0x1dfd
00004793  BF0EB0            mov di,0xb00e
00004796  E860D6            call 0x1df9
00004799  8DBE5AFF          lea di,[bp-0xa6]
0000479D  16                push ss
0000479E  57                push di
0000479F  C47EFA            les di,word [bp-0x6]
000047A2  268B4511          mov ax,[es:di+0x11]
000047A6  E8C1C2            call 0xa6a
000047A9  E851D6            call 0x1dfd
000047AC  8A46FF            mov al,[bp-0x1]
000047AF  98                cbw
000047B0  0402              add al,0x2
000047B2  8B3EC0BC          mov di,[0xbcc0]
000047B6  47                inc di
000047B7  E810C2            call 0x9ca
000047BA  8A46FF            mov al,[bp-0x1]
000047BD  FEC0              inc al
000047BF  3A46FE            cmp al,[bp-0x2]
000047C2  7F03              jg 0x47c7
000047C4  E9EBFE            jmp 0x46b2
000047C7  33C0              xor ax,ax
000047C9  E8E6DA            call 0x22b2
000047CC  AA                stosb
000047CD  91                xchg ax,cx
000047CE  F3A4              rep movsb
000047D0  C40664BD          les ax,word [0xbd64]
000047D4  A3E2BC            mov [0xbce2],ax
000047D7  8CC0              mov ax,es
000047D9  A3E4BC            mov [0xbce4],ax
000047DC  B401              mov ah,0x1
000047DE  CD16              int byte 0x16
000047E0  7407              jz 0x47e9
000047E2  E8D8C1            call 0x9bd
000047E5  3C1B              cmp al,0x1b
000047E7  74DE              jz 0x47c7
000047E9  A1CDBC            mov ax,[0xbccd]
000047EC  0B06CFBC          or ax,[0xbccf]
000047F0  7400              jz 0x47f2
000047F2  1E                push ds
000047F3  07                pop es
000047F4  E861D1            call 0x1958
000047F7  E30E              jcxz 0x4807
000047F9  AC                lodsb
000047FA  3C2F              cmp al,0x2f
000047FC  7404              jz 0x4802
000047FE  3CFF              cmp al,0xff
00004800  7502              jnz 0x4804
00004802  B05C              mov al,0x5c
00004804  AA                stosb
00004805  E2F2              loop 0x47f9
00004807  A0A0BD            mov al,[0xbda0]
0000480A  3C0A              cmp al,0xa
0000480C  7408              jz 0x4816
0000480E  3C11              cmp al,0x11
00004810  7404              jz 0x4816
00004812  3C0C              cmp al,0xc
00004814  750C              jnz 0x4822
00004816  E83FD1            call 0x1958
00004819  E307              jcxz 0x4822
0000481B  AC                lodsb
0000481C  E8A3CB            call 0x13c2
0000481F  AA                stosb
00004820  E2F9              loop 0x481b
00004822  BFE8BC            mov di,0xbce8
00004825  BE1DBB            mov si,0xbb1d
00004828  E8F5C6            call 0xf20
0000482B  A159BD            mov ax,[0xbd59]
0000482E  40                inc ax
0000482F  803E7EBD00        cmp byte [0xbd7e],0x0
00004834  757B              jnz 0x48b1
00004836  3D983A            cmp ax,0x3a98
00004839  7D6B              jnl 0x48a6
0000483B  A359BD            mov [0xbd59],ax
0000483E  50                push ax
0000483F  A3CBBC            mov [0xbccb],ax
00004842  A1CDBC            mov ax,[0xbccd]
00004845  0B06CFBC          or ax,[0xbccf]
00004849  7405              jz 0x4850
0000484B  800ECABC02        or byte [0xbcca],0x2
00004850  C406D5BC          les ax,word [0xbcd5]
00004854  8CC2              mov dx,es
00004856  01066CBD          add [0xbd6c],ax
0000485A  11166EBD          adc [0xbd6e],dx
0000485E  C406D1BC          les ax,word [0xbcd1]
00004862  8CC2              mov dx,es
00004864  010668BD          add [0xbd68],ax
00004868  11166ABD          adc [0xbd6a],dx
0000486C  A0E8BC            mov al,[0xbce8]
0000486F  041F              add al,0x1f
00004871  98                cbw
00004872  91                xchg ax,cx
00004873  58                pop ax
00004874  51                push cx
00004875  E82EB8            call 0xa6
00004878  0BC0              or ax,ax
0000487A  7425              jz 0x48a1
0000487C  BECABC            mov si,0xbcca
0000487F  A159BD            mov ax,[0xbd59]
00004882  E8ACD9            call 0x2231
00004885  59                pop cx
00004886  E851C9            call 0x11da
00004889  33C0              xor ax,ax
0000488B  A3C8BC            mov [0xbcc8],ax
0000488E  803EC1B900        cmp byte [0xb9c1],0x0
00004893  750A              jnz 0x489f
00004895  A1CBBC            mov ax,[0xbccb]
00004898  2407              and al,0x7
0000489A  7503              jnz 0x489f
0000489C  E8C2D0            call 0x1961
0000489F  F9                stc
000048A0  C3                ret
000048A1  58                pop ax
000048A2  FF0E59BD          dec word [0xbd59]
000048A6  BF03A8            mov di,0xa803
000048A9  BEC1A8            mov si,0xa8c1
000048AC  E8B5BC            call 0x564
000048AF  F8                clc
000048B0  C3                ret
000048B1  3D1E00            cmp ax,0x1e
000048B4  7703              ja 0x48b9
000048B6  E97DFF            jmp 0x4836
000048B9  803EC1B900        cmp byte [0xb9c1],0x0
000048BE  75EF              jnz 0x48af
000048C0  BF72AA            mov di,0xaa72
000048C3  BE6FAF            mov si,0xaf6f
000048C6  E88ABC            call 0x553
000048C9  EBE4              jmp 0x48af
000048CB  55                push bp
000048CC  8BEC              mov bp,sp
000048CE  33DB              xor bx,bx
000048D0  8BD3              mov dx,bx
000048D2  8E0636BD          mov es,word [0xbd36]
000048D6  8B3659BD          mov si,[0xbd59]
000048DA  268B0F            mov cx,[es:bx]
000048DD  803EA0BD05        cmp byte [0xbda0],0x5
000048E2  7513              jnz 0x48f7
000048E4  3BDE              cmp bx,si
000048E6  740F              jz 0x48f7
000048E8  8BFB              mov di,bx
000048EA  268B01            mov ax,[es:bx+di]
000048ED  3BC1              cmp ax,cx
000048EF  7303              jnc 0x48f4
000048F1  91                xchg ax,cx
000048F2  8BD3              mov dx,bx
000048F4  43                inc bx
000048F5  EBED              jmp 0x48e4
000048F7  33FF              xor di,di
000048F9  51                push cx
000048FA  57                push di
000048FB  8CC0              mov ax,es
000048FD  D1EE              shr si,0x0
000048FF  46                inc si
00004900  03C6              add ax,si
00004902  50                push ax
00004903  57                push di
00004904  A125BF            mov ax,[0xbf25]
00004907  2B46FE            sub ax,[bp-0x2]
0000490A  B104              mov cl,0x4
0000490C  8BD0              mov dx,ax
0000490E  D3C2              rol dx,cl
00004910  83E20F            and dx,0xf
00004913  D3E0              shl ax,cl
00004915  93                xchg ax,bx
00004916  0BD2              or dx,dx
00004918  7506              jnz 0x4920
0000491A  81FB00F0          cmp bx,0xf000
0000491E  7217              jc 0x4937
00004920  B900F0            mov cx,0xf000
00004923  2BD9              sub bx,cx
00004925  83DA00            sbb dx,0x0
00004928  E85000            call 0x497b
0000492B  8146FE000F        add word [bp-0x2],0xf00
00004930  8146FA000F        add word [bp-0x6],0xf00
00004935  EBDF              jmp 0x4916
00004937  8BCB              mov cx,bx
00004939  E83F00            call 0x497b
0000493C  5E                pop si
0000493D  5B                pop bx
0000493E  5F                pop di
0000493F  58                pop ax
00004940  2BC3              sub ax,bx
00004942  8E0636BD          mov es,word [0xbd36]
00004946  3B3E59BD          cmp di,[0xbd59]
0000494A  7308              jnc 0x4954
0000494C  8BDF              mov bx,di
0000494E  262901            sub [es:bx+di],ax
00004951  47                inc di
00004952  EBF2              jmp 0x4946
00004954  91                xchg ax,cx
00004955  A125BF            mov ax,[0xbf25]
00004958  2BC1              sub ax,cx
0000495A  A332BD            mov [0xbd32],ax
0000495D  93                xchg ax,bx
0000495E  A0ACB1            mov al,[0xb1ac]
00004961  8A168BAF          mov dl,[0xaf8b]
00004965  D1E2              shl dx,0x0
00004967  F6E2              mul dl
00004969  B104              mov cl,0x4
0000496B  D3E8              shr ax,cl
0000496D  40                inc ax
0000496E  03C3              add ax,bx
00004970  A325BF            mov [0xbf25],ax
00004973  A334BD            mov [0xbd34],ax
00004976  A3B2B1            mov [0xb1b2],ax
00004979  5D                pop bp
0000497A  C3                ret
0000497B  1E                push ds
0000497C  C47EF8            les di,word [bp-0x8]
0000497F  C576FC            lds si,word [bp-0x4]
00004982  E855C8            call 0x11da
00004985  1F                pop ds
00004986  C3                ret
00004987  56                push si
00004988  BF3EBC            mov di,0xbc3e
0000498B  E894C5            call 0xf22
0000498E  E848C4            call 0xdd9
00004991  5E                pop si
00004992  720B              jc 0x499f
00004994  C60400            mov byte [si],0x0
00004997  BFA1A9            mov di,0xa9a1
0000499A  BE3EBC            mov si,0xbc3e
0000499D  EB1B              jmp 0x49ba
0000499F  AC                lodsb
000049A0  4E                dec si
000049A1  0AC0              or al,al
000049A3  740F              jz 0x49b4
000049A5  98                cbw
000049A6  93                xchg ax,bx
000049A7  B85C00            mov ax,0x5c
000049AA  3800              cmp [bx+si],al
000049AC  750F              jnz 0x49bd
000049AE  8820              mov [bx+si],ah
000049B0  FE0C              dec byte [si]
000049B2  EB09              jmp 0x49bd
000049B4  BF14A7            mov di,0xa714
000049B7  BE6FAF            mov si,0xaf6f
000049BA  E8A7BB            call 0x564
000049BD  C3                ret
000049BE  AC                lodsb
000049BF  F4                hlt
000049C0  AC                lodsb
000049C1  F4                hlt
000049C2  F7F6              div si
000049C4  7AFA              jpe 0x49c0
000049C6  7AFA              jpe 0x49c2
000049C8  0DF6AC            or ax,0xacf6
000049CB  F4                hlt
000049CC  F5                cmc
000049CD  F3AC              rep lodsb
000049CF  F4                hlt
000049D0  B1F8              mov cl,0xf8
000049D2  2CF8              sub al,0xf8
000049D4  AC                lodsb
000049D5  F4                hlt
000049D6  B4F0              mov ah,0xf0
000049D8  A5                movsw
000049D9  F9                stc
000049DA  C3                ret
000049DB  F1                int1
000049DC  02F0              add dh,al
000049DE  7CEF              jl 0x49cf
000049E0  22EF              and ch,bh
000049E2  98                cbw
000049E3  ED                in ax,dx
000049E4  50                push ax
000049E5  EC                in al,dx
000049E6  A9EEF3            test ax,0xf3ee
000049E9  EB01              jmp 0x49ec
000049EB  ED                in ax,dx
000049EC  70F2              jo 0x49e0
000049EE  2EF355            cs rep push bp
000049F1  8BEC              mov bp,sp
000049F3  83EC51            sub sp,0x51
000049F6  E8B4D9            call 0x23ad
000049F9  92                xchg ax,dx
000049FA  91                xchg ax,cx
000049FB  E8F3EB            call 0x35f1
000049FE  8B0EBCBD          mov cx,[0xbdbc]
00004A02  B80100            mov ax,0x1
00004A05  D3E0              shl ax,cl
00004A07  48                dec ax
00004A08  A3BCBD            mov [0xbdbc],ax
00004A0B  803EC1B900        cmp byte [0xb9c1],0x0
00004A10  7403              jz 0x4a15
00004A12  E804FC            call 0x4619
00004A15  E8ECBB            call 0x604
00004A18  A0A0BD            mov al,[0xbda0]
00004A1B  3C18              cmp al,0x18
00004A1D  7314              jnc 0x4a33
00004A1F  3C13              cmp al,0x13
00004A21  7715              ja 0x4a38
00004A23  3C06              cmp al,0x6
00004A25  760C              jna 0x4a33
00004A27  3C0A              cmp al,0xa
00004A29  720D              jc 0x4a38
00004A2B  3C11              cmp al,0x11
00004A2D  7304              jnc 0x4a33
00004A2F  3C0E              cmp al,0xe
00004A31  7705              ja 0x4a38
00004A33  C606C2B9FF        mov byte [0xb9c2],0xff
00004A38  BFCBB9            mov di,0xb9cb
00004A3B  BE3BAF            mov si,0xaf3b
00004A3E  1E                push ds
00004A3F  57                push di
00004A40  E81BC5            call 0xf5e
00004A43  5E                pop si
00004A44  58                pop ax
00004A45  E83FFF            call 0x4987
00004A48  E867BF            call 0x9b2
00004A4B  A2C0B9            mov [0xb9c0],al
00004A4E  E8EDC3            call 0xe3e
00004A51  E879BA            call 0x4cd
00004A54  833E59BD00        cmp word [0xbd59],0x0
00004A59  7503              jnz 0x4a5e
00004A5B  E97DB9            jmp 0x3db
00004A5E  E86AFE            call 0x48cb
00004A61  E889E8            call 0x32ed
00004A64  E8D0C5            call 0x1037
00004A67  C6065FBD05        mov byte [0xbd5f],0x5
00004A6C  8B168AAF          mov dx,[0xaf8a]
00004A70  FECE              dec dh
00004A72  8A2E71AF          mov ch,[0xaf71]
00004A76  B102              mov cl,0x2
00004A78  E855BD            call 0x7d0
00004A7B  A0DCA3            mov al,[0xa3dc]
00004A7E  E845BD            call 0x7c6
00004A81  E863BC            call 0x6e7
00004A84  E8BBD2            call 0x1d42
00004A87  B80100            mov ax,0x1
00004A8A  A3C4BC            mov [0xbcc4],ax
00004A8D  A3C8BC            mov [0xbcc8],ax
00004A90  A3C0BC            mov [0xbcc0],ax
00004A93  A1C7B9            mov ax,[0xb9c7]
00004A96  40                inc ax
00004A97  3B0659BD          cmp ax,[0xbd59]
00004A9B  7E03              jng 0x4aa0
00004A9D  A159BD            mov ax,[0xbd59]
00004AA0  A3C6BC            mov [0xbcc6],ax
00004AA3  E8DBCC            call 0x1781
00004AA6  E815C6            call 0x10be
00004AA9  A0D8A3            mov al,[0xa3d8]
00004AAC  A2B3BD            mov [0xbdb3],al
00004AAF  BA2900            mov dx,0x29
00004AB2  E82ABD            call 0x7df
00004AB5  E858BC            call 0x710
00004AB8  A1BEBC            mov ax,[0xbcbe]
00004ABB  0BC0              or ax,ax
00004ABD  7428              jz 0x4ae7
00004ABF  50                push ax
00004AC0  BF19AA            mov di,0xaa19
00004AC3  E8BFBD            call 0x885
00004AC6  58                pop ax
00004AC7  8D7EE0            lea di,[bp-0x20]
00004ACA  16                push ss
00004ACB  57                push di
00004ACC  16                push ss
00004ACD  57                push di
00004ACE  E82AC7            call 0x11fb
00004AD1  E8CEBD            call 0x8a2
00004AD4  B02F              mov al,0x2f
00004AD6  E83EBD            call 0x817
00004AD9  C4067ABD          les ax,word [0xbd7a]
00004ADD  8CC3              mov bx,es
00004ADF  33C9              xor cx,cx
00004AE1  E81BC7            call 0x11ff
00004AE4  E8BBBD            call 0x8a2
00004AE7  803EC2B900        cmp byte [0xb9c2],0x0
00004AEC  740B              jz 0x4af9
00004AEE  BA3F00            mov dx,0x3f
00004AF1  E8EBBC            call 0x7df
00004AF4  B021              mov al,0x21
00004AF6  E81EBD            call 0x817
00004AF9  803E8EBC00        cmp byte [0xbc8e],0x0
00004AFE  7409              jz 0x4b09
00004B00  BA4000            mov dx,0x40
00004B03  BF23AA            mov di,0xaa23
00004B06  E8A2BD            call 0x8ab
00004B09  803EBFB900        cmp byte [0xb9bf],0x0
00004B0E  7409              jz 0x4b19
00004B10  BA4400            mov dx,0x44
00004B13  BF28AA            mov di,0xaa28
00004B16  E892BD            call 0x8ab
00004B19  E845CE            call 0x1961
00004B1C  A1BCBC            mov ax,[0xbcbc]
00004B1F  3B06C8BC          cmp ax,[0xbcc8]
00004B23  7403              jz 0x4b28
00004B25  E89DC5            call 0x10c5
00004B28  A1C4BC            mov ax,[0xbcc4]
00004B2B  2B06C0BC          sub ax,[0xbcc0]
00004B2F  7422              jz 0x4b53
00004B31  3D0100            cmp ax,0x1
00004B34  740A              jz 0x4b40
00004B36  3DFFFF            cmp ax,0xffff
00004B39  7405              jz 0x4b40
00004B3B  E843CC            call 0x1781
00004B3E  EB13              jmp 0x4b53
00004B40  E8E2BE            call 0xa25
00004B43  A1C4BC            mov ax,[0xbcc4]
00004B46  3B06C0BC          cmp ax,[0xbcc0]
00004B4A  7E03              jng 0x4b4f
00004B4C  A1C6BC            mov ax,[0xbcc6]
00004B4F  50                push ax
00004B50  E8D9D2            call 0x1e2c
00004B53  E868C5            call 0x10be
00004B56  E83ABE            call 0x993
00004B59  803EBEBD00        cmp byte [0xbdbe],0x0
00004B5E  7438              jz 0x4b98
00004B60  B10A              mov cl,0xa
00004B62  A0C7B9            mov al,[0xb9c7]
00004B65  8A26C9B9          mov ah,[0xb9c9]
00004B69  50                push ax
00004B6A  93                xchg ax,bx
00004B6B  BF2DAA            mov di,0xaa2d
00004B6E  E848C1            call 0xcb9
00004B71  59                pop cx
00004B72  B120              mov cl,0x20
00004B74  B22E              mov dl,0x2e
00004B76  80ED02            sub ch,0x2
00004B79  8AF5              mov dh,ch
00004B7B  51                push cx
00004B7C  8A3EDDA3          mov bh,[0xa3dd]
00004B80  883EB3BD          mov [0xbdb3],bh
00004B84  B80007            mov ax,0x700
00004B87  55                push bp
00004B88  CD10              int byte 0x10
00004B8A  5D                pop bp
00004B8B  5A                pop dx
00004B8C  E850BC            call 0x7df
00004B8F  BFBFBD            mov di,0xbdbf
00004B92  E8F0BC            call 0x885
00004B95  E84FBB            call 0x6e7
00004B98  32C0              xor al,al
00004B9A  A2CAB9            mov [0xb9ca],al
00004B9D  E82ABA            call 0x5ca
00004BA0  E80CB9            call 0x4af
00004BA3  A1C4BC            mov ax,[0xbcc4]
00004BA6  A3C0BC            mov [0xbcc0],ax
00004BA9  A1C8BC            mov ax,[0xbcc8]
00004BAC  A3BCBC            mov [0xbcbc],ax
00004BAF  A15BBD            mov ax,[0xbd5b]
00004BB2  0AC0              or al,al
00004BB4  7403              jz 0x4bb9
00004BB6  E90E03            jmp 0x4ec7
00004BB9  86E0              xchg ah,al
00004BBB  3C3B              cmp al,0x3b
00004BBD  7424              jz 0x4be3
00004BBF  3CEF              cmp al,0xef
00004BC1  7528              jnz 0x4beb
00004BC3  A1AFB1            mov ax,[0xb1af]
00004BC6  3A06ACB1          cmp al,[0xb1ac]
00004BCA  756A              jnz 0x4c36
00004BCC  A1ADB1            mov ax,[0xb1ad]
00004BCF  B103              mov cl,0x3
00004BD1  48                dec ax
00004BD2  D2E8              shr al,cl
00004BD4  043B              add al,0x3b
00004BD6  803EC0B900        cmp byte [0xb9c0],0x0
00004BDB  7502              jnz 0x4bdf
00004BDD  042D              add al,0x2d
00004BDF  3C3B              cmp al,0x3b
00004BE1  7505              jnz 0x4be8
00004BE3  E873E0            call 0x2c59
00004BE6  EB68              jmp 0x4c50
00004BE8  A25CBD            mov [0xbd5c],al
00004BEB  93                xchg ax,bx
00004BEC  A0C3B9            mov al,[0xb9c3]
00004BEF  80FB4B            cmp bl,0x4b
00004BF2  7507              jnz 0x4bfb
00004BF4  3C01              cmp al,0x1
00004BF6  7E6B              jng 0x4c63
00004BF8  48                dec ax
00004BF9  EB62              jmp 0x4c5d
00004BFB  80FB4D            cmp bl,0x4d
00004BFE  7507              jnz 0x4c07
00004C00  3C2C              cmp al,0x2c
00004C02  7D5F              jnl 0x4c63
00004C04  40                inc ax
00004C05  EB56              jmp 0x4c5d
00004C07  80FB73            cmp bl,0x73
00004C0A  750C              jnz 0x4c18
00004C0C  3C01              cmp al,0x1
00004C0E  7453              jz 0x4c63
00004C10  3C08              cmp al,0x8
00004C12  7E47              jng 0x4c5b
00004C14  2C08              sub al,0x8
00004C16  EB45              jmp 0x4c5d
00004C18  80FB74            cmp bl,0x74
00004C1B  750C              jnz 0x4c29
00004C1D  3C2C              cmp al,0x2c
00004C1F  7442              jz 0x4c63
00004C21  3C25              cmp al,0x25
00004C23  7D0D              jnl 0x4c32
00004C25  0408              add al,0x8
00004C27  EB34              jmp 0x4c5d
00004C29  80FB75            cmp bl,0x75
00004C2C  7524              jnz 0x4c52
00004C2E  3C2C              cmp al,0x2c
00004C30  7431              jz 0x4c63
00004C32  B02C              mov al,0x2c
00004C34  EB27              jmp 0x4c5d
00004C36  3C03              cmp al,0x3
00004C38  7C31              jl 0x4c6b
00004C3A  3A06C9B9          cmp al,[0xb9c9]
00004C3E  7D6C              jnl 0x4cac
00004C40  2D0300            sub ax,0x3
00004C43  3B0659BD          cmp ax,[0xbd59]
00004C47  7F63              jg 0x4cac
00004C49  0306C4BC          add ax,[0xbcc4]
00004C4D  A3C8BC            mov [0xbcc8],ax
00004C50  EB11              jmp 0x4c63
00004C52  80FB77            cmp bl,0x77
00004C55  750F              jnz 0x4c66
00004C57  3C01              cmp al,0x1
00004C59  7408              jz 0x4c63
00004C5B  B001              mov al,0x1
00004C5D  A2C3B9            mov [0xb9c3],al
00004C60  E81BCB            call 0x177e
00004C63  E93503            jmp 0x4f9b
00004C66  93                xchg ax,bx
00004C67  3C49              cmp al,0x49
00004C69  753D              jnz 0x4ca8
00004C6B  A1C7B9            mov ax,[0xb9c7]
00004C6E  8B16C4BC          mov dx,[0xbcc4]
00004C72  3BD0              cmp dx,ax
00004C74  7E04              jng 0x4c7a
00004C76  F7D8              neg ax
00004C78  EB64              jmp 0x4cde
00004C7A  8B1EC8BC          mov bx,[0xbcc8]
00004C7E  83FA01            cmp dx,0x1
00004C81  7405              jz 0x4c88
00004C83  BA0100            mov dx,0x1
00004C86  EB03              jmp 0x4c8b
00004C88  BB0100            mov bx,0x1
00004C8B  40                inc ax
00004C8C  3B0659BD          cmp ax,[0xbd59]
00004C90  7E03              jng 0x4c95
00004C92  A159BD            mov ax,[0xbd59]
00004C95  A3C6BC            mov [0xbcc6],ax
00004C98  3BD8              cmp bx,ax
00004C9A  7E02              jng 0x4c9e
00004C9C  8BD8              mov bx,ax
00004C9E  891EC8BC          mov [0xbcc8],bx
00004CA2  8916C4BC          mov [0xbcc4],dx
00004CA6  EBBB              jmp 0x4c63
00004CA8  3C51              cmp al,0x51
00004CAA  7540              jnz 0x4cec
00004CAC  A1C7B9            mov ax,[0xb9c7]
00004CAF  8B1659BD          mov dx,[0xbd59]
00004CB3  8B0EC6BC          mov cx,[0xbcc6]
00004CB7  8BD8              mov bx,ax
00004CB9  03D9              add bx,cx
00004CBB  3BDA              cmp bx,dx
00004CBD  7E1F              jng 0x4cde
00004CBF  92                xchg ax,dx
00004CC0  8B1EC8BC          mov bx,[0xbcc8]
00004CC4  3BC8              cmp cx,ax
00004CC6  7502              jnz 0x4cca
00004CC8  8BD8              mov bx,ax
00004CCA  A3C6BC            mov [0xbcc6],ax
00004CCD  2BC2              sub ax,dx
00004CCF  7202              jc 0x4cd3
00004CD1  7503              jnz 0x4cd6
00004CD3  B80100            mov ax,0x1
00004CD6  8BD0              mov dx,ax
00004CD8  3BD8              cmp bx,ax
00004CDA  7DC2              jnl 0x4c9e
00004CDC  EBBE              jmp 0x4c9c
00004CDE  0106C4BC          add [0xbcc4],ax
00004CE2  0106C8BC          add [0xbcc8],ax
00004CE6  0106C6BC          add [0xbcc6],ax
00004CEA  EB5A              jmp 0x4d46
00004CEC  3C47              cmp al,0x47
00004CEE  751B              jnz 0x4d0b
00004CF0  B80100            mov ax,0x1
00004CF3  A3C4BC            mov [0xbcc4],ax
00004CF6  A3C8BC            mov [0xbcc8],ax
00004CF9  A0C7B9            mov al,[0xb9c7]
00004CFC  40                inc ax
00004CFD  3B0659BD          cmp ax,[0xbd59]
00004D01  7E03              jng 0x4d06
00004D03  A159BD            mov ax,[0xbd59]
00004D06  A3C6BC            mov [0xbcc6],ax
00004D09  EB3B              jmp 0x4d46
00004D0B  3C4F              cmp al,0x4f
00004D0D  7519              jnz 0x4d28
00004D0F  A159BD            mov ax,[0xbd59]
00004D12  A3C6BC            mov [0xbcc6],ax
00004D15  A3C8BC            mov [0xbcc8],ax
00004D18  2B06C7B9          sub ax,[0xb9c7]
00004D1C  7802              js 0x4d20
00004D1E  7503              jnz 0x4d23
00004D20  B80100            mov ax,0x1
00004D23  A3C4BC            mov [0xbcc4],ax
00004D26  EB1E              jmp 0x4d46
00004D28  8B0EC8BC          mov cx,[0xbcc8]
00004D2C  3C48              cmp al,0x48
00004D2E  7507              jnz 0x4d37
00004D30  49                dec cx
00004D31  7813              js 0x4d46
00004D33  750D              jnz 0x4d42
00004D35  EB75              jmp 0x4dac
00004D37  3C50              cmp al,0x50
00004D39  750D              jnz 0x4d48
00004D3B  3B0E59BD          cmp cx,[0xbd59]
00004D3F  7D05              jnl 0x4d46
00004D41  41                inc cx
00004D42  890EC8BC          mov [0xbcc8],cx
00004D46  EB64              jmp 0x4dac
00004D48  3C52              cmp al,0x52
00004D4A  7434              jz 0x4d80
00004D4C  3CEE              cmp al,0xee
00004D4E  7555              jnz 0x4da5
00004D50  A1AFB1            mov ax,[0xb1af]
00004D53  3C03              cmp al,0x3
00004D55  7D05              jnl 0x4d5c
00004D57  A1C4BC            mov ax,[0xbcc4]
00004D5A  EB21              jmp 0x4d7d
00004D5C  3A06C9B9          cmp al,[0xb9c9]
00004D60  7F09              jg 0x4d6b
00004D62  2D0300            sub ax,0x3
00004D65  3B0659BD          cmp ax,[0xbd59]
00004D69  7C05              jl 0x4d70
00004D6B  A1C6BC            mov ax,[0xbcc6]
00004D6E  EB0D              jmp 0x4d7d
00004D70  0306C4BC          add ax,[0xbcc4]
00004D74  3B0659BD          cmp ax,[0xbd59]
00004D78  7E03              jng 0x4d7d
00004D7A  A159BD            mov ax,[0xbd59]
00004D7D  A3C8BC            mov [0xbcc8],ax
00004D80  A1C8BC            mov ax,[0xbcc8]
00004D83  50                push ax
00004D84  E8AAD4            call 0x2231
00004D87  268B4D0B          mov cx,[es:di+0xb]
00004D8B  268B550D          mov dx,[es:di+0xd]
00004D8F  268A05            mov al,[es:di]
00004D92  A801              test al,0x1
00004D94  755F              jnz 0x4df5
00004D96  40                inc ax
00004D97  FF06BEBC          inc word [0xbcbe]
00004D9B  010E7ABD          add [0xbd7a],cx
00004D9F  11167CBD          adc [0xbd7c],dx
00004DA3  EB5D              jmp 0x4e02
00004DA5  3C20              cmp al,0x20
00004DA7  7505              jnz 0x4dae
00004DA9  E8D0B8            call 0x67c
00004DAC  EB2D              jmp 0x4ddb
00004DAE  3C14              cmp al,0x14
00004DB0  752B              jnz 0x4ddd
00004DB2  BECBB9            mov si,0xb9cb
00004DB5  BF9CBA            mov di,0xba9c
00004DB8  57                push di
00004DB9  E864C1            call 0xf20
00004DBC  BF6CA9            mov di,0xa96c
00004DBF  E8F2BE            call 0xcb4
00004DC2  5F                pop di
00004DC3  E8F1D6            call 0x24b7
00004DC6  A15BBD            mov ax,[0xbd5b]
00004DC9  3C1B              cmp al,0x1b
00004DCB  740E              jz 0x4ddb
00004DCD  BE9CBA            mov si,0xba9c
00004DD0  BFCBB9            mov di,0xb9cb
00004DD3  57                push di
00004DD4  E849C1            call 0xf20
00004DD7  5E                pop si
00004DD8  E8ACFB            call 0x4987
00004DDB  EB74              jmp 0x4e51
00004DDD  3C70              cmp al,0x70
00004DDF  7505              jnz 0x4de6
00004DE1  E863BB            call 0x947
00004DE4  EB6B              jmp 0x4e51
00004DE6  3C43              cmp al,0x43
00004DE8  7529              jnz 0x4e13
00004DEA  E8E301            call 0x4fd0
00004DED  E855DA            call 0x2845
00004DF0  E8F101            call 0x4fe4
00004DF3  EB5C              jmp 0x4e51
00004DF5  48                dec ax
00004DF6  FF0EBEBC          dec word [0xbcbe]
00004DFA  290E7ABD          sub [0xbd7a],cx
00004DFE  19167CBD          sbb [0xbd7c],dx
00004E02  AA                stosb
00004E03  58                pop ax
00004E04  50                push ax
00004E05  E8BDC2            call 0x10c5
00004E08  59                pop cx
00004E09  803E5CBD52        cmp byte [0xbd5c],0x52
00004E0E  759C              jnz 0x4dac
00004E10  E928FF            jmp 0x4d3b
00004E13  3C3E              cmp al,0x3e
00004E15  7505              jnz 0x4e1c
00004E17  E806C9            call 0x1720
00004E1A  EB35              jmp 0x4e51
00004E1C  3C3F              cmp al,0x3f
00004E1E  7505              jnz 0x4e25
00004E20  E8AEB8            call 0x6d1
00004E23  EB2C              jmp 0x4e51
00004E25  3C6E              cmp al,0x6e
00004E27  7521              jnz 0x4e4a
00004E29  A0A0BD            mov al,[0xbda0]
00004E2C  3C18              cmp al,0x18
00004E2E  7314              jnc 0x4e44
00004E30  3C13              cmp al,0x13
00004E32  771D              ja 0x4e51
00004E34  3C06              cmp al,0x6
00004E36  760C              jna 0x4e44
00004E38  3C0A              cmp al,0xa
00004E3A  7215              jc 0x4e51
00004E3C  3C11              cmp al,0x11
00004E3E  7304              jnc 0x4e44
00004E40  3C0E              cmp al,0xe
00004E42  770D              ja 0x4e51
00004E44  F616C2B9          not byte [0xb9c2]
00004E48  EB07              jmp 0x4e51
00004E4A  3C71              cmp al,0x71
00004E4C  7505              jnz 0x4e53
00004E4E  E898BB            call 0x9e9
00004E51  EB2B              jmp 0x4e7e
00004E53  3C42              cmp al,0x42
00004E55  7506              jnz 0x4e5d
00004E57  8B36C8BC          mov si,[0xbcc8]
00004E5B  EB1E              jmp 0x4e7b
00004E5D  3C68              cmp al,0x68
00004E5F  7C1F              jl 0x4e80
00004E61  3C6D              cmp al,0x6d
00004E63  7F10              jg 0x4e75
00004E65  2C68              sub al,0x68
00004E67  3A065FBD          cmp al,[0xbd5f]
00004E6B  74E4              jz 0x4e51
00004E6D  A25FBD            mov [0xbd5f],al
00004E70  E8B1C8            call 0x1724
00004E73  EB09              jmp 0x4e7e
00004E75  3C6F              cmp al,0x6f
00004E77  7507              jnz 0x4e80
00004E79  33F6              xor si,si
00004E7B  E830E2            call 0x30ae
00004E7E  EB5F              jmp 0x4edf
00004E80  3C3C              cmp al,0x3c
00004E82  750B              jnz 0x4e8f
00004E84  E84901            call 0x4fd0
00004E87  E82BDA            call 0x28b5
00004E8A  E85701            call 0x4fe4
00004E8D  EB50              jmp 0x4edf
00004E8F  3C40              cmp al,0x40
00004E91  7502              jnz 0x4e95
00004E93  EBEF              jmp 0x4e84
00004E95  3C3D              cmp al,0x3d
00004E97  7518              jnz 0x4eb1
00004E99  A0BFB9            mov al,[0xb9bf]
00004E9C  50                push ax
00004E9D  C606BFB9FF        mov byte [0xb9bf],0xff
00004EA2  E82B01            call 0x4fd0
00004EA5  E8D8DE            call 0x2d80
00004EA8  E83901            call 0x4fe4
00004EAB  58                pop ax
00004EAC  A2BFB9            mov [0xb9bf],al
00004EAF  EB2E              jmp 0x4edf
00004EB1  33DB              xor bx,bx
00004EB3  3C5A              cmp al,0x5a
00004EB5  7405              jz 0x4ebc
00004EB7  3C41              cmp al,0x41
00004EB9  7506              jnz 0x4ec1
00004EBB  43                inc bx
00004EBC  E835C8            call 0x16f4
00004EBF  EB1E              jmp 0x4edf
00004EC1  3C44              cmp al,0x44
00004EC3  751A              jnz 0x4edf
00004EC5  EB1D              jmp 0x4ee4
00004EC7  3D2B4E            cmp ax,0x4e2b
00004ECA  741C              jz 0x4ee8
00004ECC  3D2D4A            cmp ax,0x4a2d
00004ECF  7417              jz 0x4ee8
00004ED1  3C1B              cmp al,0x1b
00004ED3  7507              jnz 0x4edc
00004ED5  803EBEBD00        cmp byte [0xbdbe],0x0
00004EDA  7406              jz 0x4ee2
00004EDC  E825E4            call 0x3304
00004EDF  E9BC00            jmp 0x4f9e
00004EE2  33DB              xor bx,bx
00004EE4  93                xchg ax,bx
00004EE5  E8CAD3            call 0x22b2
00004EE8  50                push ax
00004EE9  E8A7BA            call 0x993
00004EEC  58                pop ax
00004EED  2C2B              sub al,0x2b
00004EEF  BF51AA            mov di,0xaa51
00004EF2  8846FF            mov [bp-0x1],al
00004EF5  0AC0              or al,al
00004EF7  7503              jnz 0x4efc
00004EF9  83C70B            add di,0xb
00004EFC  57                push di
00004EFD  E8B4BD            call 0xcb4
00004F00  5F                pop di
00004F01  E881B9            call 0x885
00004F04  BF9CBA            mov di,0xba9c
00004F07  BE73B0            mov si,0xb073
00004F0A  57                push di
00004F0B  E812C0            call 0xf20
00004F0E  5F                pop di
00004F0F  E8A5D5            call 0x24b7
00004F12  A15BBD            mov ax,[0xbd5b]
00004F15  3C1B              cmp al,0x1b
00004F17  7503              jnz 0x4f1c
00004F19  E98200            jmp 0x4f9e
00004F1C  33C0              xor ax,ax
00004F1E  3B0659BD          cmp ax,[0xbd59]
00004F22  744F              jz 0x4f73
00004F24  50                push ax
00004F25  E80AD3            call 0x2232
00004F28  06                push es
00004F29  57                push di
00004F2A  BF1E00            mov di,0x1e
00004F2D  BE1DBB            mov si,0xbb1d
00004F30  1E                push ds
00004F31  56                push si
00004F32  8D76AF            lea si,[bp-0x51]
00004F35  16                push ss
00004F36  56                push si
00004F37  06                push es
00004F38  57                push di
00004F39  E85BC2            call 0x1197
00004F3C  E873C2            call 0x11b2
00004F3F  BF9CBA            mov di,0xba9c
00004F42  1E                push ds
00004F43  57                push di
00004F44  E8D5D4            call 0x241c
00004F47  5F                pop di
00004F48  07                pop es
00004F49  7324              jnc 0x4f6f
00004F4B  268A05            mov al,[es:di]
00004F4E  268B4D0B          mov cx,[es:di+0xb]
00004F52  268B550D          mov dx,[es:di+0xd]
00004F56  807EFF00          cmp byte [bp-0x1],0x0
00004F5A  752B              jnz 0x4f87
00004F5C  A801              test al,0x1
00004F5E  750F              jnz 0x4f6f
00004F60  FF06BEBC          inc word [0xbcbe]
00004F64  010E7ABD          add [0xbd7a],cx
00004F68  11167CBD          adc [0xbd7c],dx
00004F6C  0C01              or al,0x1
00004F6E  AA                stosb
00004F6F  58                pop ax
00004F70  40                inc ax
00004F71  EBAB              jmp 0x4f1e
00004F73  E839B5            call 0x4af
00004F76  A1C4BC            mov ax,[0xbcc4]
00004F79  3B06C6BC          cmp ax,[0xbcc6]
00004F7D  7F1C              jg 0x4f9b
00004F7F  50                push ax
00004F80  E842C1            call 0x10c5
00004F83  58                pop ax
00004F84  40                inc ax
00004F85  EBF2              jmp 0x4f79
00004F87  A801              test al,0x1
00004F89  74E4              jz 0x4f6f
00004F8B  FF0EBEBC          dec word [0xbcbe]
00004F8F  290E7ABD          sub [0xbd7a],cx
00004F93  19167CBD          sbb [0xbd7c],dx
00004F97  24FE              and al,0xfe
00004F99  EBD3              jmp 0x4f6e
00004F9B  E8F5B9            call 0x993
00004F9E  E843B7            call 0x6e4
00004FA1  A1C8BC            mov ax,[0xbcc8]
00004FA4  3B0659BD          cmp ax,[0xbd59]
00004FA8  7E06              jng 0x4fb0
00004FAA  A159BD            mov ax,[0xbd59]
00004FAD  A3C8BC            mov [0xbcc8],ax
00004FB0  3B06C6BC          cmp ax,[0xbcc6]
00004FB4  7E08              jng 0x4fbe
00004FB6  FF06C4BC          inc word [0xbcc4]
00004FBA  FF06C6BC          inc word [0xbcc6]
00004FBE  3B06C4BC          cmp ax,[0xbcc4]
00004FC2  7D08              jnl 0x4fcc
00004FC4  FF0EC4BC          dec word [0xbcc4]
00004FC8  FF0EC6BC          dec word [0xbcc6]
00004FCC  E9DAFA            jmp 0x4aa9
00004FCF  00A05FBD          add [bx+si-0x42a1],ah
00004FD3  2EA2CF50          mov [cs:0x50cf],al
00004FD7  3C05              cmp al,0x5
00004FD9  7408              jz 0x4fe3
00004FDB  C6065FBD05        mov byte [0xbd5f],0x5
00004FE0  E941C7            jmp 0x1724
00004FE3  C3                ret
00004FE4  2EA0CF50          mov al,[cs:0x50cf]
00004FE8  3A065FBD          cmp al,[0xbd5f]
00004FEC  74F5              jz 0x4fe3
00004FEE  A25FBD            mov [0xbd5f],al
00004FF1  E930C7            jmp 0x1724
00004FF4  D7                xlatb
00004FF5  13952349          adc dx,[di+0x4923]
00004FF9  C5                db 0xc5
00004FFA  C0CDF9            ror ch,byte 0xf9
00004FFD  1C10              sbb al,0x10
00004FFF  7730              ja 0x5031
00005001  DD02              fld qword [bp+si]
00005003  2AE8              sub ch,al
00005005  01B1E90E          add [bx+di+0xee9],si
00005009  58                pop ax
0000500A  DB19              fistp dword [bx+di]
0000500C  DFC3              ffreep st3
0000500E  F4                hlt
0000500F  5A                pop dx
00005010  57                push di
00005011  EF                out dx,ax
00005012  99                cwd
00005013  89FF              mov di,di
00005015  C7                db 0xc7
00005016  93                xchg ax,bx
00005017  46                inc si
00005018  5C                pop sp
00005019  42                inc dx
0000501A  F6                db 0xf6
0000501B  0DD828            or ax,0x28d8
0000501E  3E1DD9E6          ds sbb ax,0xe6d9
00005022  56                push si
00005023  06                push es
00005024  47                inc di
00005025  18ABC465          sbb [bp+di+0x65c4],ch
00005029  71DA              jno 0x5005
0000502B  7B5D              jpo 0x508a
0000502D  5B                pop bx
0000502E  A3B2CA            mov [0xcab2],ax
00005031  43                inc bx
00005032  2CEB              sub al,0xeb
00005034  6BFA4B            imul di,dx,0x4b
00005037  EA31A77DD3        jmp word 0xd37d:word 0xa731
0000503C  53                push bx
0000503D  729D              jc 0x4fdc
0000503F  90                nop
00005040  20C1              and cl,al
00005042  8F                db 0x8f
00005043  249E              and al,0x9e
00005045  7CF7              jl 0x503e
00005047  BB59D6            mov bx,0xd659
0000504A  8D2F              lea bp,[bx]
0000504C  79E4              jns 0x5032
0000504E  3D82D5            cmp ax,0xd582
00005051  C2AEFB            ret word 0xfbae
00005054  61                popa
00005055  6E                outsb
00005056  36E573            ss in ax,byte 0x73
00005059  39985E69          cmp [bx+si+0x695e],bx
0000505D  F3D437            rep aam byte 0x37
00005060  D1                db 0xd1
00005061  F5                cmc
00005062  3F                aas
00005063  0BA4C81F          or sp,[si+0x1fc8]
00005067  9C                pushf
00005068  51                push cx
00005069  B0E3              mov al,0xe3
0000506B  154C63            adc ax,0x634c
0000506E  8BBC7F11          mov di,[si+0x117f]
00005072  F8                clc
00005073  33CF              xor cx,di
00005075  78BD              js 0x5034
00005077  D208              ror byte [bx+si],cl
00005079  E229              loop 0x50a4
0000507B  48                dec ax
0000507C  B7CB              mov bh,0xcb
0000507E  87A5A63C          xchg sp,[di+0x3ca6]
00005082  6207              bound ax,[bx]
00005084  7A26              jpe 0x50ac
00005086  9BAA              wait stosb
00005088  45                inc bp
00005089  AC                lodsb
0000508A  FC                cld
0000508B  EE                out dx,al
0000508C  27                daa
0000508D  863B              xchg bh,[bp+di]
0000508F  80EC1B            sub ah,0x1b
00005092  F0                lock
00005093  50                push ax
00005094  830355            add word [bp+di],0x55
00005097  CE                into
00005098  91                xchg ax,cx
00005099  4F                dec di
0000509A  9A8E9FDCC9        call word 0xc9dc:word 0x9f8e
0000509F  854A40            test [bp+si+0x40],cx
000050A2  1481              adc al,0x81
000050A4  E0B9              loopne 0x505f
000050A6  8A67AD            mov ah,[bx-0x53]
000050A9  B62B              mov dh,0x2b
000050AB  22FE              and bh,dh
000050AD  52                push dx
000050AE  C6                db 0xc6
000050AF  97                xchg ax,di
000050B0  E7B4              out byte 0xb4,ax
000050B2  3A0A              cmp cl,[bp+si]
000050B4  761A              jna 0x50d0
000050B6  660C32            o32 or al,0x32
000050B9  8416BF88          test [0x88bf],dl
000050BD  6F                outsw
000050BE  A2B32D            mov [0x2db3],al
000050C1  0494              add al,0x94
000050C3  6C                insb
000050C4  A1384E            mov ax,[0x4e38]
000050C7  7EF2              jng 0x50bb
000050C9  DE0F              fimul word [bx]
000050CB  AF                scasw
000050CC  92                xchg ax,dx
000050CD  17                pop ss
000050CE  21F1              and cx,si
000050D0  B5BE              mov ch,0xbe
000050D2  4D                dec bp
000050D3  E100              loope 0x50d5
000050D5  2EA9BA44          cs test ax,0x44ba
000050D9  5F                pop di
000050DA  ED                in ax,dx
000050DB  41                inc cx
000050DC  35D0FD            xor ax,0xfdd0
000050DF  A809              test al,0x9
000050E1  126434            adc ah,[si+0x34]
000050E4  74B8              jz 0x509e
000050E6  A0606D            mov al,[0x6d60]
000050E9  251E6A            and ax,0x6a1e
000050EC  8C6896            mov word [bx+si-0x6a],gs
000050EF  05CC75            add ax,0x75cc
000050F2  7054              jo 0x5148
000050F4  0004              add [si],al
000050F6  0810              or [bx+si],dl
000050F8  204080            and [bx+si-0x80],al
000050FB  C00202            rol byte [bp+si],byte 0x2
000050FE  0304              add ax,[si]
00005100  050606            add ax,0x606
00005103  06                push es
00005104  0001              add [bx+di],al
00005106  0203              add al,[bp+di]
00005108  0405              add al,0x5
0000510A  06                push es
0000510B  07                pop es
0000510C  080A              or [bp+si],cl
0000510E  0C0E              or al,0xe
00005110  1014              adc [si],dl
00005112  181C              sbb [si],bl
00005114  2028              and [bx+si],ch
00005116  3038              xor [bx+si],bh
00005118  40                inc ax
00005119  50                push ax
0000511A  60                pusha
0000511B  7080              jo 0x509d
0000511D  A0C0E0            mov al,[0xe0c0]
00005120  0100              add [bx+si],ax
00005122  03A004D0          add sp,[bx+si-0x2ffc]
00005126  04E0              add al,0xe0
00005128  05F006            add ax,0x6f0
0000512B  F8                clc
0000512C  07                pop es
0000512D  FC                cld
0000512E  08FE              or dh,bh
00005130  08FF              or bh,bh
00005132  04C0              add al,0xc0
00005134  0480              add al,0x80
00005136  059006            add ax,0x690
00005139  98                cbw
0000513A  06                push es
0000513B  9C                pushf
0000513C  0000              add [bx+si],al
0000513E  0200              add al,[bx+si]
00005140  034003            add ax,[bx+si+0x3]
00005143  60                pusha
00005144  03A004D0          add sp,[bx+si-0x2ffc]
00005148  04E0              add al,0xe0
0000514A  05F006            add ax,0x6f0
0000514D  F8                clc
0000514E  06                push es
0000514F  FC                cld
00005150  04C0              add al,0xc0
00005152  0480              add al,0x80
00005154  059006            add ax,0x690
00005157  98                cbw
00005158  06                push es
00005159  9C                pushf
0000515A  0000              add [bx+si],al
0000515C  0100              add [bx+si],ax
0000515E  04A0              add al,0xa0
00005160  04D0              add al,0xd0
00005162  04E0              add al,0xe0
00005164  05F006            add ax,0x6f0
00005167  F8                clc
00005168  07                pop es
00005169  FC                cld
0000516A  08FE              or dh,bh
0000516C  08FF              or bh,bh
0000516E  04C0              add al,0xc0
00005170  0480              add al,0x80
00005172  059006            add ax,0x690
00005175  98                cbw
00005176  06                push es
00005177  9C                pushf
00005178  04B0              add al,0xb0
0000517A  0000              add [bx+si],al
0000517C  0200              add al,[bx+si]
0000517E  034003            add ax,[bx+si+0x3]
00005181  60                pusha
00005182  04A0              add al,0xa0
00005184  04D0              add al,0xd0
00005186  04E0              add al,0xe0
00005188  05F006            add ax,0x6f0
0000518B  F8                clc
0000518C  06                push es
0000518D  FC                cld
0000518E  04C0              add al,0xc0
00005190  0480              add al,0x80
00005192  059006            add ax,0x690
00005195  98                cbw
00005196  06                push es
00005197  9C                pushf
00005198  04B0              add al,0xb0
0000519A  0000              add [bx+si],al
0000519C  01800240          add [bx+si+0x4002],ax
000051A0  0320              add sp,[bx+si]
000051A2  0410              add al,0x10
000051A4  050806            add ax,0x608
000051A7  0407              add al,0x7
000051A9  0208              add cl,[bx+si]
000051AB  0108              add [bx+si],cx
000051AD  0000              add [bx+si],al
000051AF  00AE2E00          add [bp+0x2e],ch
000051B3  0002              add [bp+si],al
000051B5  0001              add [bx+di],al
000051B7  0002              add [bp+si],al
000051B9  0002              add [bp+si],al
000051BB  0004              add [si],al
000051BD  0005              add [di],al
000051BF  0004              add [si],al
000051C1  0004              add [si],al
000051C3  0008              add [bx+si],cl
000051C5  0000              add [bx+si],al
000051C7  00E0              add al,ah
000051C9  00B23000          add [bp+si+0x30],dh
000051CD  0000              add [bx+si],al
000051CF  0005              add [di],al
000051D1  0002              add [bp+si],al
000051D3  0002              add [bp+si],al
000051D5  0004              add [si],al
000051D7  0005              add [di],al
000051D9  0004              add [si],al
000051DB  0004              add [si],al
000051DD  0008              add [bx+si],cl
000051DF  0002              add [bp+si],al
000051E1  00DC              add ah,bl
000051E3  00B63200          add [bp+0x32],dh
000051E7  0000              add [bx+si],al
000051E9  0000              add [bx+si],al
000051EB  0008              add [bx+si],cl
000051ED  0008              add [bx+si],cl
000051EF  0008              add [bx+si],cl
000051F1  0009              add [bx+di],cl
000051F3  0000              add [bx+si],al
000051F5  0000              add [bx+si],al
000051F7  0000              add [bx+si],al
000051F9  0000              add [bx+si],al
000051FB  00E0              add al,ah
000051FD  00BA3400          add [bp+si+0x34],bh
00005201  0000              add [bx+si],al
00005203  0000              add [bx+si],al
00005205  0000              add [bx+si],al
00005207  0004              add [si],al
00005209  0028              add [bx+si],ch
0000520B  0010              add [bx+si],dl
0000520D  0010              add [bx+si],dl
0000520F  0004              add [si],al
00005211  0000              add [bx+si],al
00005213  002F              add [bx],ch
00005215  008200BE          add [bp+si-0x4200],al
00005219  360000            add [ss:bx+si],al
0000521C  0000              add [bx+si],al
0000521E  0000              add [bx+si],al
00005220  0000              add [bx+si],al
00005222  0200              add al,[bx+si]
00005224  05002E            add ax,0x2e00
00005227  004000            add [bx+si+0x0],al
0000522A  7400              jz 0x522c
0000522C  1800              sbb [bx+si],al
0000522E  0000              add [bx+si],al
00005230  0000              add [bx+si],al
00005232  C23800            ret word 0x38
00005235  0000              add [bx+si],al
00005237  0000              add [bx+si],al
00005239  0000              add [bx+si],al
0000523B  0000              add [bx+si],al
0000523D  0002              add [bp+si],al
0000523F  000E00CA          add [0xca00],cl
00005243  0021              add [bx+di],ah
00005245  00060000          add [0x0],al
00005249  0000              add [bx+si],al
0000524B  00C6              add dh,al
0000524D  3A00              cmp al,[bx+si]
0000524F  0000              add [bx+si],al
00005251  0000              add [bx+si],al
00005253  0000              add [bx+si],al
00005255  0000              add [bx+si],al
00005257  0000              add [bx+si],al
00005259  0000              add [bx+si],al
0000525B  00FF              add bh,bh
0000525D  0002              add [bp+si],al
0000525F  0000              add [bx+si],al
00005261  0000              add [bx+si],al
00005263  0000              add [bx+si],al
00005265  0000              add [bx+si],al
00005267  005580            add [di-0x80],dl
0000526A  3E                ds
0000526B  9BBD0075          wait mov bp,0x7500
0000526F  4F                dec di
00005270  56                push si
00005271  06                push es
00005272  56                push si
00005273  E89BBD            call 0x1011
00005276  7236              jc 0x52ae
00005278  E895B5            call 0x810
0000527B  E88AB7            call 0xa08
0000527E  5F                pop di
0000527F  57                push di
00005280  E82DB6            call 0x8b0
00005283  06                push es
00005284  BFE8A5            mov di,0xa5e8
00005287  E8FBB5            call 0x885
0000528A  E83DB3            call 0x5ca
0000528D  96                xchg ax,si
0000528E  E831C1            call 0x13c2
00005291  E883B5            call 0x817
00005294  07                pop es
00005295  3C59              cmp al,0x59
00005297  740C              jz 0x52a5
00005299  3C4E              cmp al,0x4e
0000529B  75DB              jnz 0x5278
0000529D  E860B7            call 0xa00
000052A0  B00C              mov al,0xc
000052A2  5E                pop si
000052A3  5D                pop bp
000052A4  C3                ret
000052A5  5E                pop si
000052A6  56                push si
000052A7  06                push es
000052A8  56                push si
000052A9  33D2              xor dx,dx
000052AB  E85EBD            call 0x100c
000052AE  5E                pop si
000052AF  B101              mov cl,0x1
000052B1  E8FABC            call 0xfae
000052B4  A391BD            mov [0xbd91],ax
000052B7  B80100            mov ax,0x1
000052BA  7303              jnc 0x52bf
000052BC  E9BE01            jmp 0x547d
000052BF  E820B9            call 0xbe2
000052C2  1E                push ds
000052C3  A0A0BD            mov al,[0xbda0]
000052C6  3C13              cmp al,0x13
000052C8  744C              jz 0x5316
000052CA  3C18              cmp al,0x18
000052CC  7209              jc 0x52d7
000052CE  803E9FBD00        cmp byte [0xbd9f],0x0
000052D3  741D              jz 0x52f2
000052D5  EB3F              jmp 0x5316
000052D7  8B1EC4B9          mov bx,[0xb9c4]
000052DB  3C11              cmp al,0x11
000052DD  7506              jnz 0x52e5
000052DF  81C32C6A          add bx,0x6a2c
000052E3  EB04              jmp 0x52e9
000052E5  81C30010          add bx,0x1000
000052E9  E84FBD            call 0x103b
000052EC  7304              jnc 0x52f2
000052EE  B007              mov al,0x7
000052F0  EBB0              jmp 0x52a2
000052F2  8E06B2B1          mov es,word [0xb1b2]
000052F6  33C0              xor ax,ax
000052F8  8BF8              mov di,ax
000052FA  B90080            mov cx,0x8000
000052FD  F3AB              rep stosw
000052FF  BE7FBD            mov si,0xbd7f
00005302  BF00A0            mov di,0xa000
00005305  B91800            mov cx,0x18
00005308  F3A5              rep movsw
0000530A  BF34A0            mov di,0xa034
0000530D  8CD8              mov ax,ds
0000530F  AB                stosw
00005310  8BC4              mov ax,sp
00005312  AB                stosw
00005313  E98700            jmp 0x539d
00005316  8E06B2B1          mov es,word [0xb1b2]
0000531A  803EE7BC14        cmp byte [0xbce7],0x14
0000531F  73DE              jnc 0x52ff
00005321  E8F408            call 0x5c18
00005324  1E                push ds
00005325  0E                push cs
00005326  1F                pop ds
00005327  BE2052            mov si,0x5220
0000532A  BF1E2E            mov di,0x2e1e
0000532D  B148              mov cl,0x48
0000532F  F3A5              rep movsw
00005331  1F                pop ds
00005332  BE7FBD            mov si,0xbd7f
00005335  BFDD43            mov di,0x43dd
00005338  B91600            mov cx,0x16
0000533B  F3A5              rep movsw
0000533D  BFE543            mov di,0x43e5
00005340  8CD8              mov ax,ds
00005342  AB                stosw
00005343  8BC4              mov ax,sp
00005345  AB                stosw
00005346  8CD0              mov ax,ss
00005348  AB                stosw
00005349  8CC0              mov ax,es
0000534B  054114            add ax,0x1441
0000534E  FA                cli
0000534F  8ED0              mov ss,ax
00005351  BCCEE0            mov sp,0xe0ce
00005354  FB                sti
00005355  E82808            call 0x5b80
00005358  8E1EB2B1          mov ds,word [0xb1b2]
0000535C  8CC0              mov ax,es
0000535E  054104            add ax,0x441
00005361  A3DB3F            mov [0x3fdb],ax
00005364  803EEB4300        cmp byte [0x43eb],0x0
00005369  7503              jnz 0x536e
0000536B  E86A09            call 0x5cd8
0000536E  E8F832            call 0x8669
00005371  A1E943            mov ax,[0x43e9]
00005374  C41EF143          les bx,word [0x43f1]
00005378  8CC2              mov dx,es
0000537A  FA                cli
0000537B  8ED0              mov ss,ax
0000537D  8B26E743          mov sp,[0x43e7]
00005381  FB                sti
00005382  93                xchg ax,bx
00005383  1F                pop ds
00005384  F606A1BD08        test byte [0xbda1],0x8
00005389  7405              jz 0x5390
0000538B  C6068DBD01        mov byte [0xbd8d],0x1
00005390  803E9CBD02        cmp byte [0xbd9c],0x2
00005395  7703              ja 0x539a
00005397  E9AB00            jmp 0x5445
0000539A  E99B00            jmp 0x5438
0000539D  A0A0BD            mov al,[0xbda0]
000053A0  3C0B              cmp al,0xb
000053A2  7509              jnz 0x53ad
000053A4  E86411            call 0x650b
000053A7  1F                pop ds
000053A8  33C0              xor ax,ax
000053AA  E9A300            jmp 0x5450
000053AD  26C70618A040A0    mov word [es:0xa018],0xa040
000053B4  3C0A              cmp al,0xa
000053B6  7404              jz 0x53bc
000053B8  3C02              cmp al,0x2
000053BA  7721              ja 0x53dd
000053BC  E81011            call 0x64cf
000053BF  803EA0BD0A        cmp byte [0xbda0],0xa
000053C4  7505              jnz 0x53cb
000053C6  E87205            call 0x593b
000053C9  EB03              jmp 0x53ce
000053CB  E8BC05            call 0x598a
000053CE  A114A0            mov ax,[0xa014]
000053D1  1F                pop ds
000053D2  803EA0BD01        cmp byte [0xbda0],0x1
000053D7  756C              jnz 0x5445
000053D9  32E4              xor ah,ah
000053DB  EB68              jmp 0x5445
000053DD  3C0C              cmp al,0xc
000053DF  750D              jnz 0x53ee
000053E1  E85E07            call 0x5b42
000053E4  EBE8              jmp 0x53ce
000053E6  E8E610            call 0x64cf
000053E9  E8D203            call 0x57be
000053EC  EBE0              jmp 0x53ce
000053EE  3C05              cmp al,0x5
000053F0  74F4              jz 0x53e6
000053F2  3C04              cmp al,0x4
000053F4  74F0              jz 0x53e6
000053F6  50                push ax
000053F7  E8ED10            call 0x64e7
000053FA  B8FFFF            mov ax,0xffff
000053FD  BF14A0            mov di,0xa014
00005400  AB                stosw
00005401  AB                stosw
00005402  58                pop ax
00005403  3C19              cmp al,0x19
00005405  7416              jz 0x541d
00005407  3C0E              cmp al,0xe
00005409  740D              jz 0x5418
0000540B  3C13              cmp al,0x13
0000540D  7413              jz 0x5422
0000540F  3C03              cmp al,0x3
00005411  756C              jnz 0x547f
00005413  E8C400            call 0x54da
00005416  EB12              jmp 0x542a
00005418  E86D09            call 0x5d88
0000541B  EB0D              jmp 0x542a
0000541D  E89540            call 0x94b5
00005420  EB08              jmp 0x542a
00005422  E86B4C            call 0xa090
00005425  EB03              jmp 0x542a
00005427  E84505            call 0x596f
0000542A  C40614A0          les ax,word [0xa014]
0000542E  8CC2              mov dx,es
00005430  1F                pop ds
00005431  803EA0BD13        cmp byte [0xbda0],0x13
00005436  7407              jz 0x543f
00005438  B9FFFF            mov cx,0xffff
0000543B  33D1              xor dx,cx
0000543D  33C1              xor ax,cx
0000543F  391695BD          cmp [0xbd95],dx
00005443  7509              jnz 0x544e
00005445  390693BD          cmp [0xbd93],ax
00005449  7503              jnz 0x544e
0000544B  E95AFF            jmp 0x53a8
0000544E  B004              mov al,0x4
00005450  50                push ax
00005451  803E9BBD00        cmp byte [0xbd9b],0x0
00005456  7513              jnz 0x546b
00005458  8B1E91BD          mov bx,[0xbd91]
0000545C  C40EAFBD          les cx,word [0xbdaf]
00005460  8CC2              mov dx,es
00005462  B80157            mov ax,0x5701
00005465  CD21              int byte 0x21
00005467  B43E              mov ah,0x3e
00005469  CD21              int byte 0x21
0000546B  A0A0BD            mov al,[0xbda0]
0000546E  3C18              cmp al,0x18
00005470  7307              jnc 0x5479
00005472  3C13              cmp al,0x13
00005474  7403              jz 0x5479
00005476  E8BEBB            call 0x1037
00005479  E884B5            call 0xa00
0000547C  58                pop ax
0000547D  5D                pop bp
0000547E  C3                ret
0000547F  3C11              cmp al,0x11
00005481  75A4              jnz 0x5427
00005483  E80200            call 0x5488
00005486  EBA2              jmp 0x542a
00005488  06                push es
00005489  1F                pop ds
0000548A  E88810            call 0x6515
0000548D  E8D323            call 0x7863
00005490  803E20A001        cmp byte [0xa020],0x1
00005495  7503              jnz 0x549a
00005497  E9841F            jmp 0x741e
0000549A  8CD8              mov ax,ds
0000549C  050010            add ax,0x1000
0000549F  A3AEA8            mov [0xa8ae],ax
000054A2  050008            add ax,0x800
000054A5  A3ACA8            mov [0xa8ac],ax
000054A8  05E20C            add ax,0xce2
000054AB  A3B0A8            mov [0xa8b0],ax
000054AE  05C409            add ax,0x9c4
000054B1  A3A4A8            mov [0xa8a4],ax
000054B4  05530F            add ax,0xf53
000054B7  A3A8A8            mov [0xa8a8],ax
000054BA  057102            add ax,0x271
000054BD  A3B2A8            mov [0xa8b2],ax
000054C0  05C409            add ax,0x9c4
000054C3  A3A6A8            mov [0xa8a6],ax
000054C6  05FF0F            add ax,0xfff
000054C9  A3AAA8            mov [0xa8aa],ax
000054CC  E9342D            jmp 0x8203
000054CF  B005              mov al,0x5
000054D1  98                cbw
000054D2  8B2636A0          mov sp,[0xa036]
000054D6  1F                pop ds
000054D7  E976FF            jmp 0x5450
000054DA  BE8EBC            mov si,0xbc8e
000054DD  BF40A0            mov di,0xa040
000054E0  B121              mov cl,0x21
000054E2  F3A4              rep movsb
000054E4  BE3FAE            mov si,0xae3f
000054E7  BFC09B            mov di,0x9bc0
000054EA  B13D              mov cl,0x3d
000054EC  F3A4              rep movsb
000054EE  A195BD            mov ax,[0xbd95]
000054F1  06                push es
000054F2  1F                pop ds
000054F3  F60622A001        test byte [0xa022],0x1
000054F8  7419              jz 0x5513
000054FA  A3FE9F            mov [0x9ffe],ax
000054FD  E8881C            call 0x7188
00005500  803E20A008        cmp byte [0xa020],0x8
00005505  7406              jz 0x550d
00005507  3B06FE9F          cmp ax,[0x9ffe]
0000550B  EB04              jmp 0x5511
0000550D  3A26FF9F          cmp ah,[0x9fff]
00005511  75BC              jnz 0x54cf
00005513  E8FF0F            call 0x6515
00005516  3C08              cmp al,0x8
00005518  7506              jnz 0x5520
0000551A  E86B0A            call 0x5f88
0000551D  E9B916            jmp 0x6bd9
00005520  3C01              cmp al,0x1
00005522  7503              jnz 0x5527
00005524  E9F30C            jmp 0x621a
00005527  33C0              xor ax,ax
00005529  F60622A004        test byte [0xa022],0x4
0000552E  745D              jz 0x558d
00005530  B90001            mov cx,0x100
00005533  BB224F            mov bx,0x4f22
00005536  E8A00A            call 0x5fd9
00005539  BA2250            mov dx,0x5022
0000553C  E8EB0A            call 0x602a
0000553F  BAFF00            mov dx,0xff
00005542  8BDA              mov bx,dx
00005544  8BF3              mov si,bx
00005546  8BB02250          mov si,[bx+si+0x5022]
0000554A  8A8F224F          mov cl,[bx+0x4f22]
0000554E  80F908            cmp cl,0x8
00005551  7705              ja 0x5558
00005553  BDA243            mov bp,0x43a2
00005556  EB2D              jmp 0x5585
00005558  85F2              test dx,si
0000555A  7415              jz 0x5571
0000555C  8BFE              mov di,si
0000555E  23FA              and di,dx
00005560  8895A243          mov [di+0x43a2],dl
00005564  F7C63F00          test si,0x3f
00005568  740E              jz 0x5578
0000556A  BDA245            mov bp,0x45a2
0000556D  B104              mov cl,0x4
0000556F  EB0C              jmp 0x557d
00005571  BDA249            mov bp,0x49a2
00005574  B108              mov cl,0x8
00005576  EB05              jmp 0x557d
00005578  BDA247            mov bp,0x47a2
0000557B  B106              mov cl,0x6
0000557D  D3EE              shr si,cl
0000557F  2A8F224F          sub cl,[bx+0x4f22]
00005583  F6D9              neg cl
00005585  E88C0A            call 0x6014
00005588  4B                dec bx
00005589  7DB9              jnl 0x5544
0000558B  B001              mov al,0x1
0000558D  A2605F            mov [0x5f60],al
00005590  0402              add al,0x2
00005592  A26A5F            mov [0x5f6a],al
00005595  F60622A002        test byte [0xa022],0x2
0000559A  751E              jnz 0x55ba
0000559C  C706645F0020      mov word [0x5f64],0x2000
000055A2  C706665F0010      mov word [0x5f66],0x1000
000055A8  C606625F3F        mov byte [0x5f62],0x3f
000055AD  C6066B5F06        mov byte [0x5f6b],0x6
000055B2  C706685F621C      mov word [0x5f68],0x1c62
000055B8  EB1C              jmp 0x55d6
000055BA  C706645F0010      mov word [0x5f64],0x1000
000055C0  C706665F0020      mov word [0x5f66],0x2000
000055C6  C606625F7F        mov byte [0x5f62],0x7f
000055CB  C6066B5F07        mov byte [0x5f6b],0x7
000055D0  C706685F622C      mov word [0x5f68],0x2c62
000055D6  A06A5F            mov al,[0x5f6a]
000055D9  043F              add al,0x3f
000055DB  A26C5F            mov [0x5f6c],al
000055DE  B94000            mov cx,0x40
000055E1  BB624E            mov bx,0x4e62
000055E4  E8F209            call 0x5fd9
000055E7  BAA24E            mov dx,0x4ea2
000055EA  E83D0A            call 0x602a
000055ED  BBA24D            mov bx,0x4da2
000055F0  E8E609            call 0x5fd9
000055F3  BAE24D            mov dx,0x4de2
000055F6  E8310A            call 0x602a
000055F9  BB3F00            mov bx,0x3f
000055FC  BAFF00            mov dx,0xff
000055FF  8A8FA24D          mov cl,[bx+0x4da2]
00005603  80F910            cmp cl,0x10
00005606  731D              jnc 0x5625
00005608  32E4              xor ah,ah
0000560A  8BF3              mov si,bx
0000560C  8A80E24D          mov al,[bx+si+0x4de2]
00005610  8BF0              mov si,ax
00005612  BF0100            mov di,0x1
00005615  D3E7              shl di,cl
00005617  888CA24C          mov [si+0x4ca2],cl
0000561B  889CA24B          mov [si+0x4ba2],bl
0000561F  03F7              add si,di
00005621  3BF2              cmp si,dx
00005623  76F2              jna 0x5617
00005625  8AC3              mov al,bl
00005627  02066A5F          add al,[0x5f6a]
0000562B  8BF3              mov si,bx
0000562D  8BB0A24E          mov si,[bx+si+0x4ea2]
00005631  8A8F624E          mov cl,[bx+0x4e62]
00005635  80F908            cmp cl,0x8
00005638  7705              ja 0x563f
0000563A  BDA23D            mov bp,0x3da2
0000563D  EB20              jmp 0x565f
0000563F  85F2              test dx,si
00005641  740F              jz 0x5652
00005643  8BFE              mov di,si
00005645  23FA              and di,dx
00005647  8895A23D          mov [di+0x3da2],dl
0000564B  BDA23F            mov bp,0x3fa2
0000564E  B104              mov cl,0x4
00005650  EB05              jmp 0x5657
00005652  BDA241            mov bp,0x41a2
00005655  B108              mov cl,0x8
00005657  D3EE              shr si,cl
00005659  2A8F624E          sub cl,[bx+0x4e62]
0000565D  F6D9              neg cl
0000565F  E8B409            call 0x6016
00005662  4B                dec bx
00005663  7D9A              jnl 0x55ff
00005665  32C0              xor al,al
00005667  BF620C            mov di,0xc62
0000566A  8B0E665F          mov cx,[0x5f66]
0000566E  81C14001          add cx,0x140
00005672  F3AA              rep stosb
00005674  33C0              xor ax,ax
00005676  A27652            mov [0x5276],al
00005679  E8570A            call 0x60d3
0000567C  8B3E685F          mov di,[0x5f68]
00005680  4D                dec bp
00005681  8A14              mov dl,[si]
00005683  46                inc si
00005684  32ED              xor ch,ch
00005686  EB5A              jmp 0x56e2
00005688  B106              mov cl,0x6
0000568A  B8A247            mov ax,0x47a2
0000568D  E8BE00            call 0x574e
00005690  03D8              add bx,ax
00005692  8A07              mov al,[bx]
00005694  8A8F0001          mov cl,[bx+0x100]
00005698  EB23              jmp 0x56bd
0000569A  B108              mov cl,0x8
0000569C  B8A249            mov ax,0x49a2
0000569F  EBEC              jmp 0x568d
000056A1  F6C23F            test dl,0x3f
000056A4  74E2              jz 0x5688
000056A6  B104              mov cl,0x4
000056A8  B8A245            mov ax,0x45a2
000056AB  EBE0              jmp 0x568d
000056AD  84D2              test dl,dl
000056AF  74E9              jz 0x569a
000056B1  8A87A243          mov al,[bx+0x43a2]
000056B5  3CFF              cmp al,0xff
000056B7  74E8              jz 0x56a1
000056B9  8A8FA244          mov cl,[bx+0x44a2]
000056BD  E88E00            call 0x574e
000056C0  AA                stosb
000056C1  EB7C              jmp 0x573f
000056C3  803E605F00        cmp byte [0x5f60],0x0
000056C8  75E3              jnz 0x56ad
000056CA  8AC2              mov al,dl
000056CC  B108              mov cl,0x8
000056CE  EBED              jmp 0x56bd
000056D0  B108              mov cl,0x8
000056D2  B8A241            mov ax,0x41a2
000056D5  E87600            call 0x574e
000056D8  03D8              add bx,ax
000056DA  8A07              mov al,[bx]
000056DC  8A8F0001          mov cl,[bx+0x100]
000056E0  EB3A              jmp 0x571c
000056E2  B101              mov cl,0x1
000056E4  E86700            call 0x574e
000056E7  72DA              jc 0x56c3
000056E9  8AE2              mov ah,dl
000056EB  2226625F          and ah,[0x5f62]
000056EF  A06B5F            mov al,[0x5f6b]
000056F2  8AC8              mov cl,al
000056F4  E85700            call 0x574e
000056F7  8A8FA24C          mov cl,[bx+0x4ca2]
000056FB  8A9FA24B          mov bl,[bx+0x4ba2]
000056FF  91                xchg ax,cx
00005700  D3E3              shl bx,cl
00005702  91                xchg ax,cx
00005703  0ADC              or bl,ah
00005705  891E4257          mov [0x5742],bx
00005709  E84200            call 0x574e
0000570C  84D2              test dl,dl
0000570E  74C0              jz 0x56d0
00005710  8A87A23D          mov al,[bx+0x3da2]
00005714  84C0              test al,al
00005716  782F              js 0x5747
00005718  8A8FA23E          mov cl,[bx+0x3ea2]
0000571C  E82F00            call 0x574e
0000571F  32E4              xor ah,ah
00005721  38066C5F          cmp [0x5f6c],al
00005725  750B              jnz 0x5732
00005727  8BDA              mov bx,dx
00005729  32FF              xor bh,bh
0000572B  03C3              add ax,bx
0000572D  B108              mov cl,0x8
0000572F  E81C00            call 0x574e
00005732  91                xchg ax,cx
00005733  56                push si
00005734  8BF7              mov si,di
00005736  2B364257          sub si,[0x5742]
0000573A  4E                dec si
0000573B  F3A4              rep movsb
0000573D  91                xchg ax,cx
0000573E  5E                pop si
0000573F  81FF623C          cmp di,0x3c62
00005743  729D              jc 0x56e2
00005745  EB3C              jmp 0x5783
00005747  B104              mov cl,0x4
00005749  B8A23F            mov ax,0x3fa2
0000574C  EB87              jmp 0x56d5
0000574E  2AE9              sub ch,cl
00005750  732A              jnc 0x577c
00005752  F6DD              neg ch
00005754  2ACD              sub cl,ch
00005756  D3EA              shr dx,cl
00005758  4D                dec bp
00005759  7918              jns 0x5773
0000575B  803E765201        cmp byte [0x5276],0x1
00005760  744E              jz 0x57b0
00005762  50                push ax
00005763  51                push cx
00005764  52                push dx
00005765  E86B09            call 0x60d3
00005768  5A                pop dx
00005769  59                pop cx
0000576A  58                pop ax
0000576B  4D                dec bp
0000576C  7905              jns 0x5773
0000576E  C606765201        mov byte [0x5276],0x1
00005773  8A34              mov dh,[si]
00005775  46                inc si
00005776  8ACD              mov cl,ch
00005778  B508              mov ch,0x8
0000577A  2AE9              sub ch,cl
0000577C  32FF              xor bh,bh
0000577E  D3EA              shr dx,cl
00005780  8ADA              mov bl,dl
00005782  C3                ret
00005783  51                push cx
00005784  52                push dx
00005785  56                push si
00005786  57                push di
00005787  8B0E645F          mov cx,[0x5f64]
0000578B  8B36685F          mov si,[0x5f68]
0000578F  51                push cx
00005790  56                push si
00005791  E8D714            call 0x6c6b
00005794  BF620C            mov di,0xc62
00005797  5E                pop si
00005798  58                pop ax
00005799  03F0              add si,ax
0000579B  8B0E665F          mov cx,[0x5f66]
0000579F  2BF1              sub si,cx
000057A1  81C14001          add cx,0x140
000057A5  F3A4              rep movsb
000057A7  5F                pop di
000057A8  2BF8              sub di,ax
000057AA  5E                pop si
000057AB  5A                pop dx
000057AC  59                pop cx
000057AD  E932FF            jmp 0x56e2
000057B0  58                pop ax
000057B1  8BCF              mov cx,di
000057B3  8B16685F          mov dx,[0x5f68]
000057B7  2BCA              sub cx,dx
000057B9  8BF2              mov si,dx
000057BB  E9AD14            jmp 0x6c6b
000057BE  BE3FAE            mov si,0xae3f
000057C1  BFC09B            mov di,0x9bc0
000057C4  B13D              mov cl,0x3d
000057C6  F3A4              rep movsb
000057C8  E8430D            call 0x650e
000057CB  33FF              xor di,di
000057CD  893E1EA0          mov [0xa01e],di
000057D1  893EFE9F          mov [0x9ffe],di
000057D5  893E06A0          mov [0xa006],di
000057D9  A020A0            mov al,[0xa020]
000057DC  3C02              cmp al,0x2
000057DE  732A              jnc 0x580a
000057E0  B92C2F            mov cx,0x2f2c
000057E3  BA40A0            mov dx,0xa040
000057E6  57                push di
000057E7  E8B507            call 0x5f9f
000057EA  5F                pop di
000057EB  7304              jnc 0x57f1
000057ED  E87214            call 0x6c62
000057F0  C3                ret
000057F1  8BF2              mov si,dx
000057F3  832E00A001        sub word [0xa000],0x1
000057F8  831E02A000        sbb word [0xa002],0x0
000057FD  78EE              js 0x57ed
000057FF  E87109            call 0x6173
00005802  81FE6CCF          cmp si,0xcf6c
00005806  72EB              jc 0x57f3
00005808  EBD6              jmp 0x57e0
0000580A  7447              jz 0x5853
0000580C  C70618A06CCF      mov word [0xa018],0xcf6c
00005812  2EC7063A6486FF    mov word [cs:0x643a],0xff86
00005819  2EC7064964D800    mov word [cs:0x6449],0xd8
00005820  2EC7060F641201    mov word [cs:0x640f],0x112
00005827  3C06              cmp al,0x6
00005829  750D              jnz 0x5838
0000582B  B90800            mov cx,0x8
0000582E  E8080C            call 0x6439
00005831  B310              mov bl,0x10
00005833  B8B7FD            mov ax,0xfdb7
00005836  EB05              jmp 0x583d
00005838  B888FD            mov ax,0xfd88
0000583B  B320              mov bl,0x20
0000583D  2EA39E64          mov [cs:0x649e],ax
00005841  2E881E7D64        mov [cs:0x647d],bl
00005846  2E881EC664        mov [cs:0x64c6],bl
0000584B  2E881EAB64        mov [cs:0x64ab],bl
00005850  E9E60A            jmp 0x6339
00005853  BAFE9F            mov dx,0x9ffe
00005856  B90200            mov cx,0x2
00005859  E83A07            call 0x5f96
0000585C  AD                lodsw
0000585D  3D0101            cmp ax,0x101
00005860  7368              jnc 0x58ca
00005862  D1E0              shl ax,0x0
00005864  D1E0              shl ax,0x0
00005866  91                xchg ax,cx
00005867  BA6EFB            mov dx,0xfb6e
0000586A  E82907            call 0x5f96
0000586D  C70618A06CCF      mov word [0xa018],0xcf6c
00005873  33C9              xor cx,cx
00005875  BF6CF7            mov di,0xf76c
00005878  1E                push ds
00005879  07                pop es
0000587A  8BD1              mov dx,cx
0000587C  8BDA              mov bx,dx
0000587E  D1E3              shl bx,0x0
00005880  D1E3              shl bx,0x0
00005882  33C0              xor ax,ax
00005884  48                dec ax
00005885  AB                stosw
00005886  40                inc ax
00005887  8BD8              mov bx,ax
00005889  D1EA              shr dx,0x0
0000588B  D1D3              rcl bx,0x0
0000588D  D1E3              shl bx,0x0
0000588F  8B18              mov bx,[bx+si]
00005891  40                inc ax
00005892  0BDB              or bx,bx
00005894  780A              js 0x58a0
00005896  3C08              cmp al,0x8
00005898  72EF              jc 0x5889
0000589A  93                xchg ax,bx
0000589B  AA                stosb
0000589C  93                xchg ax,bx
0000589D  AA                stosb
0000589E  EB07              jmp 0x58a7
000058A0  F7D3              not bx
000058A2  895DFE            mov [di-0x2],bx
000058A5  47                inc di
000058A6  AA                stosb
000058A7  FEC1              inc cl
000058A9  75CF              jnz 0x587a
000058AB  33C0              xor ax,ax
000058AD  A31EA0            mov [0xa01e],ax
000058B0  33C9              xor cx,cx
000058B2  E8380B            call 0x63ed
000058B5  92                xchg ax,dx
000058B6  8BC2              mov ax,dx
000058B8  32E4              xor ah,ah
000058BA  BE6CF7            mov si,0xf76c
000058BD  D1E0              shl ax,0x0
000058BF  D1E0              shl ax,0x0
000058C1  03F0              add si,ax
000058C3  AD                lodsw
000058C4  0AE4              or ah,ah
000058C6  783B              js 0x5903
000058C8  7403              jz 0x58cd
000058CA  E95D0A            jmp 0x632a
000058CD  93                xchg ax,bx
000058CE  AD                lodsw
000058CF  8AEC              mov ch,ah
000058D1  3AE9              cmp ch,cl
000058D3  770A              ja 0x58df
000058D5  86E9              xchg ch,cl
000058D7  D3EA              shr dx,cl
000058D9  2AE9              sub ch,cl
000058DB  86E9              xchg ch,cl
000058DD  EB12              jmp 0x58f1
000058DF  E80B0B            call 0x63ed
000058E2  D3EA              shr dx,cl
000058E4  8AF0              mov dh,al
000058E6  2AE9              sub ch,cl
000058E8  86E9              xchg ch,cl
000058EA  D3EA              shr dx,cl
000058EC  80E908            sub cl,0x8
000058EF  F6D9              neg cl
000058F1  93                xchg ax,bx
000058F2  51                push cx
000058F3  52                push dx
000058F4  8B3E1EA0          mov di,[0xa01e]
000058F8  E87908            call 0x6174
000058FB  893E1EA0          mov [0xa01e],di
000058FF  5A                pop dx
00005900  59                pop cx
00005901  EBB3              jmp 0x58b6
00005903  AC                lodsb
00005904  32E4              xor ah,ah
00005906  93                xchg ax,bx
00005907  E8E30A            call 0x63ed
0000590A  D3EA              shr dx,cl
0000590C  8AF0              mov dh,al
0000590E  B008              mov al,0x8
00005910  2AC1              sub al,cl
00005912  91                xchg ax,cx
00005913  D3EA              shr dx,cl
00005915  91                xchg ax,cx
00005916  FEC9              dec cl
00005918  7907              jns 0x5921
0000591A  B107              mov cl,0x7
0000591C  E8CE0A            call 0x63ed
0000591F  8AF0              mov dh,al
00005921  8BF3              mov si,bx
00005923  D1EA              shr dx,0x0
00005925  D1D6              rcl si,0x0
00005927  D1E6              shl si,0x0
00005929  8B846EFB          mov ax,[si-0x492]
0000592D  0AE4              or ah,ah
0000592F  8BD8              mov bx,ax
00005931  79E3              jns 0x5916
00005933  F7D0              not ax
00005935  0AE4              or ah,ah
00005937  7591              jnz 0x58ca
00005939  EBB7              jmp 0x58f2
0000593B  BE3FAE            mov si,0xae3f
0000593E  BFC09B            mov di,0x9bc0
00005941  B13D              mov cl,0x3d
00005943  F3A4              rep movsb
00005945  E8C60B            call 0x650e
00005948  A020A0            mov al,[0xa020]
0000594B  3C01              cmp al,0x1
0000594D  752B              jnz 0x597a
0000594F  33FF              xor di,di
00005951  893E1EA0          mov [0xa01e],di
00005955  893E06A0          mov [0xa006],di
00005959  C70618A06CCF      mov word [0xa018],0xcf6c
0000595F  B90900            mov cx,0x9
00005962  E8D40A            call 0x6439
00005965  2EC7069E6488FD    mov word [cs:0x649e],0xfd88
0000596C  E9C309            jmp 0x6332
0000596F  06                push es
00005970  1F                pop ds
00005971  C706FE9F0D0A      mov word [0x9ffe],0xa0d
00005977  E89B0B            call 0x6515
0000597A  E80B06            call 0x5f88
0000597D  A020A0            mov al,[0xa020]
00005980  3C04              cmp al,0x4
00005982  7403              jz 0x5987
00005984  E9A604            jmp 0x5e2d
00005987  E96105            jmp 0x5eeb
0000598A  BE7CAE            mov si,0xae7c
0000598D  BF0090            mov di,0x9000
00005990  33C0              xor ax,ax
00005992  8BD0              mov dx,ax
00005994  BB2000            mov bx,0x20
00005997  8A14              mov dl,[si]
00005999  46                inc si
0000599A  8BCB              mov cx,bx
0000599C  F3AA              rep stosb
0000599E  40                inc ax
0000599F  4A                dec dx
000059A0  75F8              jnz 0x599a
000059A2  D1EB              shr bx,0x0
000059A4  73F1              jnc 0x5997
000059A6  B003              mov al,0x3
000059A8  8A0C              mov cl,[si]
000059AA  E306              jcxz 0x59b2
000059AC  46                inc si
000059AD  F3AA              rep stosb
000059AF  40                inc ax
000059B0  EBF6              jmp 0x59a8
000059B2  E8590B            call 0x650e
000059B5  E8D005            call 0x5f88
000059B8  A020A0            mov al,[0xa020]
000059BB  3C7A              cmp al,0x7a
000059BD  7503              jnz 0x59c2
000059BF  E9D900            jmp 0x5a9b
000059C2  3C01              cmp al,0x1
000059C4  7412              jz 0x59d8
000059C6  2EC606D46805      mov byte [cs:0x68d4],0x5
000059CC  3C06              cmp al,0x6
000059CE  74B4              jz 0x5984
000059D0  2EC606D46804      mov byte [cs:0x68d4],0x4
000059D6  EBAC              jmp 0x5984
000059D8  B80100            mov ax,0x1
000059DB  B93A01            mov cx,0x13a
000059DE  51                push cx
000059DF  BF16E4            mov di,0xe416
000059E2  F3AB              rep stosw
000059E4  B8E604            mov ax,0x4e6
000059E7  33D2              xor dx,dx
000059E9  59                pop cx
000059EA  BF30DF            mov di,0xdf30
000059ED  BBE4ED            mov bx,0xede4
000059F0  AB                stosw
000059F1  40                inc ax
000059F2  40                inc ax
000059F3  8917              mov [bx],dx
000059F5  43                inc bx
000059F6  43                inc bx
000059F7  42                inc dx
000059F8  42                inc dx
000059F9  E2F5              loop 0x59f0
000059FB  33F6              xor si,si
000059FD  BB7402            mov bx,0x274
00005A00  B93901            mov cx,0x139
00005A03  8B8416E4          mov ax,[si-0x1bea]
00005A07  038418E4          add ax,[si-0x1be8]
00005A0B  898716E4          mov [bx-0x1bea],ax
00005A0F  89B730DF          mov [bx-0x20d0],si
00005A13  899CFEE8          mov [si-0x1702],bx
00005A17  899C00E9          mov [si-0x1700],bx
00005A1B  83C604            add si,0x4
00005A1E  43                inc bx
00005A1F  43                inc bx
00005A20  E2E1              loop 0x5a03
00005A22  33C0              xor ax,ax
00005A24  A30AA0            mov [0xa00a],ax
00005A27  A208A0            mov [0xa008],al
00005A2A  A3E2ED            mov [0xede2],ax
00005A2D  48                dec ax
00005A2E  A3FCE8            mov [0xe8fc],ax
00005A31  33FF              xor di,di
00005A33  B82020            mov ax,0x2020
00005A36  B90040            mov cx,0x4000
00005A39  F3AB              rep stosw
00005A3B  33FF              xor di,di
00005A3D  A106A0            mov ax,[0xa006]
00005A40  0BC0              or ax,ax
00005A42  7852              js 0x5a96
00005A44  0B0604A0          or ax,[0xa004]
00005A48  744C              jz 0x5a96
00005A4A  57                push di
00005A4B  E85610            call 0x6aa4
00005A4E  0AE4              or ah,ah
00005A50  7510              jnz 0x5a62
00005A52  5F                pop di
00005A53  AA                stosb
00005A54  81FF0080          cmp di,0x8000
00005A58  7203              jc 0x5a5d
00005A5A  E80912            call 0x6c66
00005A5D  B90100            mov cx,0x1
00005A60  EB29              jmp 0x5a8b
00005A62  50                push ax
00005A63  E8F710            call 0x6b5d
00005A66  40                inc ax
00005A67  59                pop cx
00005A68  5F                pop di
00005A69  8BF7              mov si,di
00005A6B  2BF0              sub si,ax
00005A6D  81E6FF7F          and si,0x7fff
00005A71  81E9FD00          sub cx,0xfd
00005A75  51                push cx
00005A76  A4                movsb
00005A77  81FF0080          cmp di,0x8000
00005A7B  7207              jc 0x5a84
00005A7D  56                push si
00005A7E  51                push cx
00005A7F  E8E411            call 0x6c66
00005A82  59                pop cx
00005A83  5E                pop si
00005A84  81E6FF7F          and si,0x7fff
00005A88  E2EC              loop 0x5a76
00005A8A  59                pop cx
00005A8B  290E04A0          sub [0xa004],cx
00005A8F  831E06A000        sbb word [0xa006],0x0
00005A94  EBA7              jmp 0x5a3d
00005A96  8BCF              mov cx,di
00005A98  E9CE11            jmp 0x6c69
00005A9B  33FF              xor di,di
00005A9D  B90040            mov cx,0x4000
00005AA0  B82020            mov ax,0x2020
00005AA3  F3AB              rep stosw
00005AA5  BE40A0            mov si,0xa040
00005AA8  33FF              xor di,di
00005AAA  B280              mov dl,0x80
00005AAC  A106A0            mov ax,[0xa006]
00005AAF  0BC0              or ax,ax
00005AB1  78E3              js 0x5a96
00005AB3  0B0604A0          or ax,[0xa004]
00005AB7  74DD              jz 0x5a96
00005AB9  D0C2              rol dl,0x0
00005ABB  7305              jnc 0x5ac2
00005ABD  E87000            call 0x5b30
00005AC0  8AF0              mov dh,al
00005AC2  E86B00            call 0x5b30
00005AC5  84F2              test dl,dh
00005AC7  741C              jz 0x5ae5
00005AC9  AA                stosb
00005ACA  81FF0080          cmp di,0x8000
00005ACE  7207              jc 0x5ad7
00005AD0  56                push si
00005AD1  52                push dx
00005AD2  E89111            call 0x6c66
00005AD5  5A                pop dx
00005AD6  5E                pop si
00005AD7  B80100            mov ax,0x1
00005ADA  290604A0          sub [0xa004],ax
00005ADE  831E06A000        sbb word [0xa006],0x0
00005AE3  EBC7              jmp 0x5aac
00005AE5  50                push ax
00005AE6  E84700            call 0x5b30
00005AE9  5B                pop bx
00005AEA  8AF8              mov bh,al
00005AEC  B104              mov cl,0x4
00005AEE  D2EF              shr bh,cl
00005AF0  83C312            add bx,0x12
00005AF3  8BCF              mov cx,di
00005AF5  56                push si
00005AF6  8BF7              mov si,di
00005AF8  81E600F0          and si,0xf000
00005AFC  81E1FF0F          and cx,0xfff
00005B00  3BD9              cmp bx,cx
00005B02  7604              jna 0x5b08
00005B04  81EE0010          sub si,0x1000
00005B08  03F3              add si,bx
00005B0A  81E6FF7F          and si,0x7fff
00005B0E  250F00            and ax,0xf
00005B11  050300            add ax,0x3
00005B14  50                push ax
00005B15  91                xchg ax,cx
00005B16  A4                movsb
00005B17  81E6FF7F          and si,0x7fff
00005B1B  81FF0080          cmp di,0x8000
00005B1F  7209              jc 0x5b2a
00005B21  56                push si
00005B22  52                push dx
00005B23  51                push cx
00005B24  E83F11            call 0x6c66
00005B27  59                pop cx
00005B28  5A                pop dx
00005B29  5E                pop si
00005B2A  E2EA              loop 0x5b16
00005B2C  58                pop ax
00005B2D  5E                pop si
00005B2E  EBAA              jmp 0x5ada
00005B30  AC                lodsb
00005B31  81FE30DF          cmp si,0xdf30
00005B35  720A              jc 0x5b41
00005B37  50                push ax
00005B38  52                push dx
00005B39  E84C04            call 0x5f88
00005B3C  5A                pop dx
00005B3D  58                pop ax
00005B3E  BE40A0            mov si,0xa040
00005B41  C3                ret
00005B42  E8C909            call 0x650e
00005B45  803E20A007        cmp byte [0xa020],0x7
00005B4A  7504              jnz 0x5b50
00005B4C  B005              mov al,0x5
00005B4E  EB02              jmp 0x5b52
00005B50  B004              mov al,0x4
00005B52  2EA2D468          mov [cs:0x68d4],al
00005B56  2EC706B35F9090    mov word [cs:0x5fb3],0x9090
00005B5D  E82804            call 0x5f88
00005B60  E9CA02            jmp 0x5e2d
00005B63  8A1C              mov bl,[si]
00005B65  32FF              xor bh,bh
00005B67  32D8              xor bl,al
00005B69  8AC4              mov al,ah
00005B6B  8AE2              mov ah,dl
00005B6D  8AD6              mov dl,dh
00005B6F  8AF7              mov dh,bh
00005B71  D1E3              shl bx,0x0
00005B73  D1E3              shl bx,0x0
00005B75  263301            xor ax,[es:bx+di]
00005B78  26335102          xor dx,[es:bx+di+0x2]
00005B7C  46                inc si
00005B7D  E2E4              loop 0x5b63
00005B7F  C3                ret
00005B80  33C0              xor ax,ax
00005B82  BFF143            mov di,0x43f1
00005B85  803E9CBD02        cmp byte [0xbd9c],0x2
00005B8A  7602              jna 0x5b8e
00005B8C  48                dec ax
00005B8D  AB                stosw
00005B8E  AB                stosw
00005B8F  BE8EBC            mov si,0xbc8e
00005B92  AC                lodsb
00005B93  98                cbw
00005B94  91                xchg ax,cx
00005B95  E343              jcxz 0x5bda
00005B97  FE06FF43          inc byte [0x43ff]
00005B9B  803E9CBD02        cmp byte [0xbd9c],0x2
00005BA0  7639              jna 0x5bdb
00005BA2  B8FFFF            mov ax,0xffff
00005BA5  8BD0              mov dx,ax
00005BA7  BFDD3F            mov di,0x3fdd
00005BAA  56                push si
00005BAB  E8B5FF            call 0x5b63
00005BAE  5E                pop si
00005BAF  A3A3BD            mov [0xbda3],ax
00005BB2  8916A5BD          mov [0xbda5],dx
00005BB6  8A4CFF            mov cl,[si-0x1]
00005BB9  33C0              xor ax,ax
00005BBB  8BD0              mov dx,ax
00005BBD  8A1C              mov bl,[si]
00005BBF  32FF              xor bh,bh
00005BC1  32C3              xor al,bl
00005BC3  03D3              add dx,bx
00005BC5  D1E3              shl bx,0x0
00005BC7  D1E3              shl bx,0x0
00005BC9  263301            xor ax,[es:bx+di]
00005BCC  26035102          add dx,[es:bx+di+0x2]
00005BD0  46                inc si
00005BD1  E2EA              loop 0x5bbd
00005BD3  A3A7BD            mov [0xbda7],ax
00005BD6  8916A9BD          mov [0xbda9],dx
00005BDA  C3                ret
00005BDB  33C0              xor ax,ax
00005BDD  A30144            mov [0x4401],ax
00005BE0  A20344            mov [0x4403],al
00005BE3  8BD8              mov bx,ax
00005BE5  8BD0              mov dx,ax
00005BE7  AC                lodsb
00005BE8  02D8              add bl,al
00005BEA  32F8              xor bh,al
00005BEC  02D0              add dl,al
00005BEE  D0C2              rol dl,0x0
00005BF0  46                inc si
00005BF1  E2F4              loop 0x5be7
00005BF3  891E0144          mov [0x4401],bx
00005BF7  88160344          mov [0x4403],dl
00005BFB  C3                ret
00005BFC  268B05            mov ax,[es:di]
00005BFF  85C0              test ax,ax
00005C01  7414              jz 0x5c17
00005C03  B104              mov cl,0x4
00005C05  8AEC              mov ch,ah
00005C07  D2ED              shr ch,cl
00005C09  B110              mov cl,0x10
00005C0B  2ACD              sub cl,ch
00005C0D  D3E0              shl ax,cl
00005C0F  80E50F            and ch,0xf
00005C12  0AC5              or al,ch
00005C14  AB                stosw
00005C15  EBE5              jmp 0x5bfc
00005C17  C3                ret
00005C18  8E06B2B1          mov es,word [0xb1b2]
00005C1C  803E8DBD00        cmp byte [0xbd8d],0x0
00005C21  7401              jz 0x5c24
00005C23  C3                ret
00005C24  B90822            mov cx,0x2208
00005C27  33C0              xor ax,ax
00005C29  8BF8              mov di,ax
00005C2B  F3AB              rep stosw
00005C2D  BEB052            mov si,0x52b0
00005C30  BA0C00            mov dx,0xc
00005C33  2E8B04            mov ax,[cs:si]
00005C36  85C0              test ax,ax
00005C38  746D              jz 0x5ca7
00005C3A  46                inc si
00005C3B  46                inc si
00005C3C  97                xchg ax,di
00005C3D  33C0              xor ax,ax
00005C3F  2E8B0C            mov cx,[cs:si]
00005C42  46                inc si
00005C43  46                inc si
00005C44  80C610            add dh,0x10
00005C47  D1E0              shl ax,0x0
00005C49  E309              jcxz 0x5c54
00005C4B  80E40F            and ah,0xf
00005C4E  0AE6              or ah,dh
00005C50  AB                stosw
00005C51  40                inc ax
00005C52  E2F7              loop 0x5c4b
00005C54  FECA              dec dl
00005C56  75E7              jnz 0x5c3f
00005C58  EBD6              jmp 0x5c30
00005C5A  33F6              xor si,si
00005C5C  3E8B1A            mov bx,[ds:bp+si]
00005C5F  85DB              test bx,bx
00005C61  74B4              jz 0x5c17
00005C63  8AEB              mov ch,bl
00005C65  80E50F            and ch,0xf
00005C68  83E3F0            and bx,0xfffffffffffffff0
00005C6B  D3EB              shr bx,cl
00005C6D  B010              mov al,0x10
00005C6F  2AC1              sub al,cl
00005C71  2AC5              sub al,ch
00005C73  51                push cx
00005C74  8AC8              mov cl,al
00005C76  B001              mov al,0x1
00005C78  D3E0              shl ax,cl
00005C7A  59                pop cx
00005C7B  803ECD3F00        cmp byte [0x3fcd],0x0
00005C80  740D              jz 0x5c8f
00005C82  D1E3              shl bx,0x0
00005C84  D1EE              shr si,0x0
00005C86  368931            mov [ss:bx+di],si
00005C89  D1EB              shr bx,0x0
00005C8B  D1E6              shl si,0x0
00005C8D  EB09              jmp 0x5c98
00005C8F  D1EE              shr si,0x0
00005C91  96                xchg ax,si
00005C92  368801            mov [ss:bx+di],al
00005C95  96                xchg ax,si
00005C96  D1E6              shl si,0x0
00005C98  87D7              xchg dx,di
00005C9A  368829            mov [ss:bx+di],ch
00005C9D  87D7              xchg dx,di
00005C9F  43                inc bx
00005CA0  48                dec ax
00005CA1  75D8              jnz 0x5c7b
00005CA3  46                inc si
00005CA4  46                inc si
00005CA5  EBB5              jmp 0x5c5c
00005CA7  BFAE2E            mov di,0x2eae
00005CAA  E84FFF            call 0x5bfc
00005CAD  BFB230            mov di,0x30b2
00005CB0  E849FF            call 0x5bfc
00005CB3  BFB632            mov di,0x32b6
00005CB6  E843FF            call 0x5bfc
00005CB9  BFBA34            mov di,0x34ba
00005CBC  E83DFF            call 0x5bfc
00005CBF  BFBE36            mov di,0x36be
00005CC2  E837FF            call 0x5bfc
00005CC5  BFC238            mov di,0x38c2
00005CC8  E831FF            call 0x5bfc
00005CCB  BFC63A            mov di,0x3ac6
00005CCE  E82BFF            call 0x5bfc
00005CD1  BFDD3F            mov di,0x3fdd
00005CD4  E81308            call 0x64ea
00005CD7  C3                ret
00005CD8  06                push es
00005CD9  16                push ss
00005CDA  07                pop es
00005CDB  33C0              xor ax,ax
00005CDD  8BF8              mov di,ax
00005CDF  B90070            mov cx,0x7000
00005CE2  F3AB              rep stosw
00005CE4  07                pop es
00005CE5  55                push bp
00005CE6  C606CD3F00        mov byte [0x3fcd],0x0
00005CEB  B108              mov cl,0x8
00005CED  BA0002            mov dx,0x200
00005CF0  BD1E2E            mov bp,0x2e1e
00005CF3  BF0000            mov di,0x0
00005CF6  E861FF            call 0x5c5a
00005CF9  BA0005            mov dx,0x500
00005CFC  BD3C2E            mov bp,0x2e3c
00005CFF  BF0003            mov di,0x300
00005D02  E855FF            call 0x5c5a
00005D05  BA0008            mov dx,0x800
00005D08  BD5A2E            mov bp,0x2e5a
00005D0B  BF0006            mov di,0x600
00005D0E  E849FF            call 0x5c5a
00005D11  BA000B            mov dx,0xb00
00005D14  BD7A2E            mov bp,0x2e7a
00005D17  BF0009            mov di,0x900
00005D1A  E83DFF            call 0x5c5a
00005D1D  BA000E            mov dx,0xe00
00005D20  BD9A2E            mov bp,0x2e9a
00005D23  BF000C            mov di,0xc00
00005D26  E831FF            call 0x5c5a
00005D29  B104              mov cl,0x4
00005D2B  BA0020            mov dx,0x2000
00005D2E  BDAE2E            mov bp,0x2eae
00005D31  BF0010            mov di,0x1000
00005D34  E823FF            call 0x5c5a
00005D37  BA0040            mov dx,0x4000
00005D3A  BDB230            mov bp,0x30b2
00005D3D  BF0030            mov di,0x3000
00005D40  E817FF            call 0x5c5a
00005D43  C606CD3F01        mov byte [0x3fcd],0x1
00005D48  BA0070            mov dx,0x7000
00005D4B  BDB632            mov bp,0x32b6
00005D4E  BF0050            mov di,0x5000
00005D51  E806FF            call 0x5c5a
00005D54  BA00A0            mov dx,0xa000
00005D57  BDBA34            mov bp,0x34ba
00005D5A  BF0080            mov di,0x8000
00005D5D  E8FAFE            call 0x5c5a
00005D60  B106              mov cl,0x6
00005D62  BA00B8            mov dx,0xb800
00005D65  BDBE36            mov bp,0x36be
00005D68  BF00B0            mov di,0xb000
00005D6B  E8ECFE            call 0x5c5a
00005D6E  BA00C8            mov dx,0xc800
00005D71  BDC238            mov bp,0x38c2
00005D74  BF00C0            mov di,0xc000
00005D77  E8E0FE            call 0x5c5a
00005D7A  BA00D8            mov dx,0xd800
00005D7D  BDC63A            mov bp,0x3ac6
00005D80  BF00D0            mov di,0xd000
00005D83  E8D4FE            call 0x5c5a
00005D86  5D                pop bp
00005D87  C3                ret
00005D88  06                push es
00005D89  1F                pop ds
00005D8A  E88807            call 0x6515
00005D8D  E8F801            call 0x5f88
00005D90  BFC07F            mov di,0x7fc0
00005D93  B82020            mov ax,0x2020
00005D96  B92000            mov cx,0x20
00005D99  F3AB              rep stosw
00005D9B  BF0096            mov di,0x9600
00005D9E  8BC1              mov ax,cx
00005DA0  B104              mov cl,0x4
00005DA2  F3AB              rep stosw
00005DA4  B104              mov cl,0x4
00005DA6  40                inc ax
00005DA7  F3AA              rep stosb
00005DA9  3C05              cmp al,0x5
00005DAB  75F7              jnz 0x5da4
00005DAD  40                inc ax
00005DAE  AA                stosb
00005DAF  AA                stosb
00005DB0  33C0              xor ax,ax
00005DB2  AB                stosw
00005DB3  B108              mov cl,0x8
00005DB5  AB                stosw
00005DB6  40                inc ax
00005DB7  E2FC              loop 0x5db5
00005DB9  AB                stosw
00005DBA  BB0200            mov bx,0x2
00005DBD  B104              mov cl,0x4
00005DBF  03C3              add ax,bx
00005DC1  AB                stosw
00005DC2  E2FB              loop 0x5dbf
00005DC4  D1E3              shl bx,0x0
00005DC6  80FB80            cmp bl,0x80
00005DC9  75F2              jnz 0x5dbd
00005DCB  A020A0            mov al,[0xa020]
00005DCE  D1E8              shr ax,0x0
00005DD0  7319              jnc 0x5deb
00005DD2  33C0              xor ax,ax
00005DD4  AA                stosb
00005DD5  B10F              mov cl,0xf
00005DD7  AA                stosb
00005DD8  40                inc ax
00005DD9  E2FC              loop 0x5dd7
00005DDB  BF8296            mov di,0x9682
00005DDE  33C0              xor ax,ax
00005DE0  AB                stosw
00005DE1  40                inc ax
00005DE2  B110              mov cl,0x10
00005DE4  AB                stosw
00005DE5  D1E0              shl ax,0x0
00005DE7  E2FB              loop 0x5de4
00005DE9  EB2B              jmp 0x5e16
00005DEB  33C0              xor ax,ax
00005DED  AB                stosw
00005DEE  AA                stosb
00005DEF  B10E              mov cl,0xe
00005DF1  AA                stosb
00005DF2  AA                stosb
00005DF3  40                inc ax
00005DF4  E2FB              loop 0x5df1
00005DF6  AA                stosb
00005DF7  BF8296            mov di,0x9682
00005DFA  BE2096            mov si,0x9620
00005DFD  B106              mov cl,0x6
00005DFF  AD                lodsw
00005E00  AB                stosw
00005E01  E2FC              loop 0x5dff
00005E03  BB0200            mov bx,0x2
00005E06  03C3              add ax,bx
00005E08  AB                stosw
00005E09  03C3              add ax,bx
00005E0B  0BC0              or ax,ax
00005E0D  7805              js 0x5e14
00005E0F  AB                stosw
00005E10  D1E3              shl bx,0x0
00005E12  EBF2              jmp 0x5e06
00005E14  48                dec ax
00005E15  AB                stosw
00005E16  803E20A002        cmp byte [0xa020],0x2
00005E1B  7609              jna 0x5e26
00005E1D  2EC7069D68EB40    mov word [cs:0x689d],0x40eb
00005E24  EB07              jmp 0x5e2d
00005E26  2EC7069D68EB60    mov word [cs:0x689d],0x60eb
00005E2D  BDFF7F            mov bp,0x7fff
00005E30  E86808            call 0x669b
00005E33  33C0              xor ax,ax
00005E35  8BF8              mov di,ax
00005E37  A31EA0            mov [0xa01e],ax
00005E3A  B90100            mov cx,0x1
00005E3D  290E04A0          sub [0xa004],cx
00005E41  831E06A000        sbb word [0xa006],0x0
00005E46  784C              js 0x5e94
00005E48  E82809            call 0x6773
00005E4B  0AE4              or ah,ah
00005E4D  7548              jnz 0x5e97
00005E4F  AA                stosb
00005E50  3BFD              cmp di,bp
00005E52  76E6              jna 0x5e3a
00005E54  E80B0E            call 0x6c62
00005E57  EBE1              jmp 0x5e3a
00005E59  770E              ja 0x5e69
00005E5B  81F98200          cmp cx,0x82
00005E5F  7C08              jl 0x5e69
00005E61  81CE0020          or si,0x2000
00005E65  81E17FFF          and cx,0xff7f
00005E69  F7D6              not si
00005E6B  03F7              add si,di
00005E6D  23F5              and si,bp
00005E6F  51                push cx
00005E70  8BD5              mov dx,bp
00005E72  2BD1              sub dx,cx
00005E74  42                inc dx
00005E75  3BF2              cmp si,dx
00005E77  7309              jnc 0x5e82
00005E79  3BFA              cmp di,dx
00005E7B  7305              jnc 0x5e82
00005E7D  F3A4              rep movsb
00005E7F  59                pop cx
00005E80  EBBB              jmp 0x5e3d
00005E82  A4                movsb
00005E83  23F5              and si,bp
00005E85  3BFD              cmp di,bp
00005E87  7607              jna 0x5e90
00005E89  56                push si
00005E8A  51                push cx
00005E8B  E8D40D            call 0x6c62
00005E8E  59                pop cx
00005E8F  5E                pop si
00005E90  E2F0              loop 0x5e82
00005E92  EBEB              jmp 0x5e7f
00005E94  E9CB0D            jmp 0x6c62
00005E97  2DFD00            sub ax,0xfd
00005E9A  8B1606A0          mov dx,[0xa006]
00005E9E  8B0E04A0          mov cx,[0xa004]
00005EA2  0BD2              or dx,dx
00005EA4  7506              jnz 0x5eac
00005EA6  3BC8              cmp cx,ax
00005EA8  7302              jnc 0x5eac
00005EAA  91                xchg ax,cx
00005EAB  40                inc ax
00005EAC  50                push ax
00005EAD  E86808            call 0x6718
00005EB0  8BF0              mov si,ax
00005EB2  59                pop cx
00005EB3  EBB4              jmp 0x5e69
00005EB5  803E20A003        cmp byte [0xa020],0x3
00005EBA  759D              jnz 0x5e59
00005EBC  83F942            cmp cx,0x42
00005EBF  7CA8              jl 0x5e69
00005EC1  81F98200          cmp cx,0x82
00005EC5  7C1A              jl 0x5ee1
00005EC7  81F9C200          cmp cx,0xc2
00005ECB  7C0A              jl 0x5ed7
00005ECD  81E9C000          sub cx,0xc0
00005ED1  81C60060          add si,0x6000
00005ED5  EB92              jmp 0x5e69
00005ED7  81E98000          sub cx,0x80
00005EDB  81C60040          add si,0x4000
00005EDF  EB88              jmp 0x5e69
00005EE1  83E940            sub cx,0x40
00005EE4  81C60020          add si,0x2000
00005EE8  E97EFF            jmp 0x5e69
00005EEB  E8AD07            call 0x669b
00005EEE  33C0              xor ax,ax
00005EF0  8BF8              mov di,ax
00005EF2  A30AA0            mov [0xa00a],ax
00005EF5  A308A0            mov [0xa008],ax
00005EF8  833E06A000        cmp word [0xa006],0x0
00005EFD  7507              jnz 0x5f06
00005EFF  833E04A000        cmp word [0xa004],0x0
00005F04  748E              jz 0x5e94
00005F06  E8680A            call 0x6971
00005F09  0BC0              or ax,ax
00005F0B  7549              jnz 0x5f56
00005F0D  833E08A008        cmp word [0xa008],0x8
00005F12  7D1C              jnl 0x5f30
00005F14  A11AA0            mov ax,[0xa01a]
00005F17  8B0E08A0          mov cx,[0xa008]
00005F1B  D3E8              shr ax,cl
00005F1D  09060AA0          or [0xa00a],ax
00005F21  B81000            mov ax,0x10
00005F24  2BC1              sub ax,cx
00005F26  91                xchg ax,cx
00005F27  E87E07            call 0x66a8
00005F2A  C70608A01000      mov word [0xa008],0x10
00005F30  A10AA0            mov ax,[0xa00a]
00005F33  B90800            mov cx,0x8
00005F36  D3E8              shr ax,cl
00005F38  D3260AA0          shl word [0xa00a],cl
00005F3C  290E08A0          sub [0xa008],cx
00005F40  AA                stosb
00005F41  832E04A001        sub word [0xa004],0x1
00005F46  831E06A000        sbb word [0xa006],0x0
00005F4B  81FF0080          cmp di,0x8000
00005F4F  72A7              jc 0x5ef8
00005F51  E8120D            call 0x6c66
00005F54  EBA2              jmp 0x5ef8
00005F56  40                inc ax
00005F57  40                inc ax
00005F58  50                push ax
00005F59  290604A0          sub [0xa004],ax
00005F5D  831E06A000        sbb word [0xa006],0x0
00005F62  E8F709            call 0x695c
00005F65  8BF7              mov si,di
00005F67  2BF0              sub si,ax
00005F69  4E                dec si
00005F6A  81E6FF7F          and si,0x7fff
00005F6E  59                pop cx
00005F6F  A4                movsb
00005F70  81E6FF7F          and si,0x7fff
00005F74  81FF0080          cmp di,0x8000
00005F78  7305              jnc 0x5f7f
00005F7A  E2F3              loop 0x5f6f
00005F7C  E979FF            jmp 0x5ef8
00005F7F  51                push cx
00005F80  56                push si
00005F81  E8E20C            call 0x6c66
00005F84  5E                pop si
00005F85  59                pop cx
00005F86  EBF2              jmp 0x5f7a
00005F88  BA40A0            mov dx,0xa040
00005F8B  B8F03E            mov ax,0x3ef0
00005F8E  E82E05            call 0x64bf
00005F91  EB0C              jmp 0x5f9f
00005F93  E82905            call 0x64bf
00005F96  290E00A0          sub [0xa000],cx
00005F9A  831E02A000        sbb word [0xa002],0x0
00005F9F  8BF2              mov si,dx
00005FA1  53                push bx
00005FA2  B43F              mov ah,0x3f
00005FA4  8B1E10A0          mov bx,[0xa010]
00005FA8  CD21              int byte 0x21
00005FAA  9C                pushf
00005FAB  803E21A019        cmp byte [0xa021],0x19
00005FB0  7513              jnz 0x5fc5
00005FB2  803E22A014        cmp byte [0xa022],0x14
00005FB7  721A              jc 0x5fd3
00005FB9  51                push cx
00005FBA  56                push si
00005FBB  1E                push ds
00005FBC  56                push si
00005FBD  51                push cx
00005FBE  E82938            call 0x97ea
00005FC1  5E                pop si
00005FC2  59                pop cx
00005FC3  EB0E              jmp 0x5fd3
00005FC5  F60622A001        test byte [0xa022],0x1
00005FCA  7407              jz 0x5fd3
00005FCC  51                push cx
00005FCD  56                push si
00005FCE  E80E01            call 0x60df
00005FD1  5E                pop si
00005FD2  59                pop cx
00005FD3  9D                popf
00005FD4  5B                pop bx
00005FD5  C3                ret
00005FD6  E9DE0F            jmp 0x6fb7
00005FD9  53                push bx
00005FDA  51                push cx
00005FDB  51                push cx
00005FDC  B90100            mov cx,0x1
00005FDF  BA600C            mov dx,0xc60
00005FE2  E8B1FF            call 0x5f96
00005FE5  72EF              jc 0x5fd6
00005FE7  AD                lodsw
00005FE8  59                pop cx
00005FE9  3BC1              cmp ax,cx
00005FEB  73E9              jnc 0x5fd6
00005FED  91                xchg ax,cx
00005FEE  50                push ax
00005FEF  41                inc cx
00005FF0  8BD6              mov dx,si
00005FF2  E8A1FF            call 0x5f96
00005FF5  72DF              jc 0x5fd6
00005FF7  5A                pop dx
00005FF8  8BFB              mov di,bx
00005FFA  33DB              xor bx,bx
00005FFC  AC                lodsb
00005FFD  8AD8              mov bl,al
00005FFF  B104              mov cl,0x4
00006001  D2EB              shr bl,cl
00006003  43                inc bx
00006004  8ACB              mov cl,bl
00006006  240F              and al,0xf
00006008  40                inc ax
00006009  2BD3              sub dx,bx
0000600B  72C9              jc 0x5fd6
0000600D  F3AA              rep stosb
0000600F  77EB              ja 0x5ffc
00006011  59                pop cx
00006012  5B                pop bx
00006013  C3                ret
00006014  8AC3              mov al,bl
00006016  BF0100            mov di,0x1
00006019  D3E7              shl di,cl
0000601B  3E8802            mov [ds:bp+si],al
0000601E  3E888A0001        mov [ds:bp+si+0x100],cl
00006023  03F7              add si,di
00006025  3BF2              cmp si,dx
00006027  76F2              jna 0x601b
00006029  C3                ret
0000602A  51                push cx
0000602B  BF620C            mov di,0xc62
0000602E  895514            mov [di+0x14],dx
00006031  895D12            mov [di+0x12],bx
00006034  894D10            mov [di+0x10],cx
00006037  51                push cx
00006038  B90800            mov cx,0x8
0000603B  33C0              xor ax,ax
0000603D  F3AB              rep stosw
0000603F  BF610C            mov di,0xc61
00006042  8BF3              mov si,bx
00006044  59                pop cx
00006045  AC                lodsb
00006046  8BD8              mov bx,ax
00006048  FE01              inc byte [bx+di]
0000604A  E2F9              loop 0x6045
0000604C  33C0              xor ax,ax
0000604E  B91000            mov cx,0x10
00006051  47                inc di
00006052  0205              add al,[di]
00006054  AA                stosb
00006055  E2FB              loop 0x6052
00006057  BF610C            mov di,0xc61
0000605A  8B7513            mov si,[di+0x13]
0000605D  8B4D11            mov cx,[di+0x11]
00006060  03F1              add si,cx
00006062  4E                dec si
00006063  FD                std
00006064  33C0              xor ax,ax
00006066  8BD8              mov bx,ax
00006068  AC                lodsb
00006069  8BE8              mov bp,ax
0000606B  3E8A1B            mov bl,[ds:bp+di]
0000606E  FECB              dec bl
00006070  3E881B            mov [ds:bp+di],bl
00006073  884117            mov [bx+di+0x17],al
00006076  E2F0              loop 0x6068
00006078  8B5D11            mov bx,[di+0x11]
0000607B  8D7116            lea si,[bx+di+0x16]
0000607E  8DB91501          lea di,[bx+di+0x115]
00006082  03FB              add di,bx
00006084  33C0              xor ax,ax
00006086  8BD0              mov dx,ax
00006088  8BC8              mov cx,ax
0000608A  03C2              add ax,dx
0000608C  8A0C              mov cl,[si]
0000608E  4E                dec si
0000608F  3AE9              cmp ch,cl
00006091  7407              jz 0x609a
00006093  8AE9              mov ch,cl
00006095  33D2              xor dx,dx
00006097  F9                stc
00006098  D3DA              rcr dx,cl
0000609A  AB                stosw
0000609B  4B                dec bx
0000609C  75EC              jnz 0x608a
0000609E  BD610C            mov bp,0xc61
000060A1  3E8B7E15          mov di,[ds:bp+0x15]
000060A5  3E8B7613          mov si,[ds:bp+0x13]
000060A9  3E8B4E11          mov cx,[ds:bp+0x11]
000060AD  BB780D            mov bx,0xd78
000060B0  FC                cld
000060B1  57                push di
000060B2  AC                lodsb
000060B3  32E4              xor ah,ah
000060B5  8BF8              mov di,ax
000060B7  3E8A03            mov al,[ds:bp+di]
000060BA  3EFE03            inc byte [ds:bp+di]
000060BD  97                xchg ax,di
000060BE  D1E7              shl di,0x0
000060C0  8B11              mov dx,[bx+di]
000060C2  51                push cx
000060C3  B91000            mov cx,0x10
000060C6  D1E2              shl dx,0x0
000060C8  D1D8              rcr ax,0x0
000060CA  E2FA              loop 0x60c6
000060CC  59                pop cx
000060CD  5F                pop di
000060CE  AB                stosw
000060CF  E2E0              loop 0x60b1
000060D1  59                pop cx
000060D2  C3                ret
000060D3  BA40A0            mov dx,0xa040
000060D6  B80008            mov ax,0x800
000060D9  E8B7FE            call 0x5f93
000060DC  8BE8              mov bp,ax
000060DE  C3                ret
000060DF  8BC8              mov cx,ax
000060E1  E344              jcxz 0x6127
000060E3  803E21A003        cmp byte [0xa021],0x3
000060E8  7433              jz 0x611d
000060EA  06                push es
000060EB  8E0634A0          mov es,word [0xa034]
000060EF  8B1E28A0          mov bx,[0xa028]
000060F3  52                push dx
000060F4  8BF2              mov si,dx
000060F6  263A1E8EBC        cmp bl,[es:0xbc8e]
000060FB  7202              jc 0x60ff
000060FD  33DB              xor bx,bx
000060FF  43                inc bx
00006100  268A978EBC        mov dl,[es:bx-0x4372]
00006105  803E21A006        cmp byte [0xa021],0x6
0000610A  7504              jnz 0x6110
0000610C  021624A0          add dl,[0xa024]
00006110  3014              xor [si],dl
00006112  46                inc si
00006113  E2E1              loop 0x60f6
00006115  891E28A0          mov [0xa028],bx
00006119  5A                pop dx
0000611A  07                pop es
0000611B  EB0A              jmp 0x6127
0000611D  50                push ax
0000611E  E85110            call 0x7172
00006121  E8E00F            call 0x7104
00006124  E2F8              loop 0x611e
00006126  58                pop ax
00006127  C3                ret
00006128  A21DA0            mov [0xa01d],al
0000612B  4E                dec si
0000612C  8804              mov [si],al
0000612E  B86EFF            mov ax,0xff6e
00006131  2BC6              sub ax,si
00006133  8BC8              mov cx,ax
00006135  8B3E1EA0          mov di,[0xa01e]
00006139  03C7              add ax,di
0000613B  3DD95B            cmp ax,0x5bd9
0000613E  7D03              jnl 0x6143
00006140  F3A4              rep movsb
00006142  C3                ret
00006143  81FE6EFF          cmp si,0xff6e
00006147  73F9              jnc 0x6142
00006149  A4                movsb
0000614A  81FFD95B          cmp di,0x5bd9
0000614E  75F3              jnz 0x6143
00006150  56                push si
00006151  E80E0B            call 0x6c62
00006154  5E                pop si
00006155  EBEC              jmp 0x6143
00006157  A21DA0            mov [0xa01d],al
0000615A  4E                dec si
0000615B  8804              mov [si],al
0000615D  B86EFF            mov ax,0xff6e
00006160  2BC6              sub ax,si
00006162  8BC8              mov cx,ax
00006164  8B3E1EA0          mov di,[0xa01e]
00006168  81FE6EFF          cmp si,0xff6e
0000616C  73D4              jnc 0x6142
0000616E  E80200            call 0x6173
00006171  EBF5              jmp 0x6168
00006173  AC                lodsb
00006174  803E07A000        cmp byte [0xa007],0x0
00006179  750F              jnz 0x618a
0000617B  B90100            mov cx,0x1
0000617E  3C90              cmp al,0x90
00006180  7521              jnz 0x61a3
00006182  A207A0            mov [0xa007],al
00006185  C3                ret
00006186  B090              mov al,0x90
00006188  EB19              jmp 0x61a3
0000618A  91                xchg ax,cx
0000618B  0BFF              or di,di
0000618D  7506              jnz 0x6195
0000618F  8A85D85B          mov al,[di+0x5bd8]
00006193  EB03              jmp 0x6198
00006195  8A45FF            mov al,[di-0x1]
00006198  32ED              xor ch,ch
0000619A  882E07A0          mov [0xa007],ch
0000619E  49                dec cx
0000619F  78E5              js 0x6186
000061A1  7503              jnz 0x61a6
000061A3  B90100            mov cx,0x1
000061A6  AA                stosb
000061A7  81FFD95B          cmp di,0x5bd9
000061AB  7509              jnz 0x61b6
000061AD  56                push si
000061AE  51                push cx
000061AF  50                push ax
000061B0  E8AF0A            call 0x6c62
000061B3  58                pop ax
000061B4  59                pop cx
000061B5  5E                pop si
000061B6  E2EE              loop 0x61a6
000061B8  C3                ret
000061B9  BFD95B            mov di,0x5bd9
000061BC  90                nop
000061BD  8B0E0AA0          mov cx,[0xa00a]
000061C1  49                dec cx
000061C2  8BD9              mov bx,cx
000061C4  D1E3              shl bx,0x0
000061C6  8D31              lea si,[bx+di]
000061C8  81E90001          sub cx,0x100
000061CC  7E35              jng 0x6203
000061CE  51                push cx
000061CF  FD                std
000061D0  810C0080          or word [si],0x8000
000061D4  4E                dec si
000061D5  4E                dec si
000061D6  E2F8              loop 0x61d0
000061D8  FC                cld
000061D9  BEDB5D            mov si,0x5ddb
000061DC  90                nop
000061DD  59                pop cx
000061DE  51                push cx
000061DF  AD                lodsw
000061E0  25FF7F            and ax,0x7fff
000061E3  3D0101            cmp ax,0x101
000061E6  7C08              jl 0x61f0
000061E8  8BD8              mov bx,ax
000061EA  D1E3              shl bx,0x0
000061EC  8121FF7F          and word [bx+di],0x7fff
000061F0  E2ED              loop 0x61df
000061F2  59                pop cx
000061F3  BEDB5D            mov si,0x5ddb
000061F6  90                nop
000061F7  AD                lodsw
000061F8  0BC0              or ax,ax
000061FA  7905              jns 0x6201
000061FC  C744FEFFFF        mov word [si-0x2],0xffff
00006201  E2F4              loop 0x61f7
00006203  B9001F            mov cx,0x1f00
00006206  B8FFFF            mov ax,0xffff
00006209  BFDB5D            mov di,0x5ddb
0000620C  90                nop
0000620D  F2AF              repne scasw
0000620F  B80020            mov ax,0x2000
00006212  7502              jnz 0x6216
00006214  2BC1              sub ax,cx
00006216  A30AA0            mov [0xa00a],ax
00006219  C3                ret
0000621A  C70618A06CCF      mov word [0xa018],0xcf6c
00006220  33FF              xor di,di
00006222  893E1EA0          mov [0xa01e],di
00006226  E8BF00            call 0x62e8
00006229  A10CA0            mov ax,[0xa00c]
0000622C  A304A0            mov [0xa004],ax
0000622F  E80302            call 0x6435
00006232  3D0001            cmp ax,0x100
00006235  7518              jnz 0x624f
00006237  E8FB01            call 0x6435
0000623A  3D0100            cmp ax,0x1
0000623D  740A              jz 0x6249
0000623F  3D0200            cmp ax,0x2
00006242  75EB              jnz 0x622f
00006244  E872FF            call 0x61b9
00006247  EBE6              jmp 0x622f
00006249  FE0608A0          inc byte [0xa008]
0000624D  EBE0              jmp 0x622f
0000624F  8BF8              mov di,ax
00006251  A30CA0            mov [0xa00c],ax
00006254  BE6EFF            mov si,0xff6e
00006257  8BDF              mov bx,di
00006259  83B9D95BFF        cmp word [bx+di+0x5bd9],0xffffffffffffffff
0000625E  750C              jnz 0x626c
00006260  4E                dec si
00006261  A01DA0            mov al,[0xa01d]
00006264  8804              mov [si],al
00006266  8B3E04A0          mov di,[0xa004]
0000626A  8BDF              mov bx,di
0000626C  81FF0101          cmp di,0x101
00006270  7214              jc 0x6286
00006272  83B9D95BFF        cmp word [bx+di+0x5bd9],0xffffffffffffffff
00006277  74E7              jz 0x6260
00006279  4E                dec si
0000627A  8A856CCF          mov al,[di-0x3094]
0000627E  8804              mov [si],al
00006280  8BB9D95B          mov di,[bx+di+0x5bd9]
00006284  EBE4              jmp 0x626a
00006286  8A856CCF          mov al,[di-0x3094]
0000628A  E89BFE            call 0x6128
0000628D  893E1EA0          mov [0xa01e],di
00006291  8B3E0AA0          mov di,[0xa00a]
00006295  81FF0020          cmp di,0x2000
00006299  7D24              jnl 0x62bf
0000629B  8BDF              mov bx,di
0000629D  A104A0            mov ax,[0xa004]
000062A0  8981D95B          mov [bx+di+0x5bd9],ax
000062A4  A01DA0            mov al,[0xa01d]
000062A7  88856CCF          mov [di-0x3094],al
000062AB  47                inc di
000062AC  81FF0020          cmp di,0x2000
000062B0  7D09              jnl 0x62bb
000062B2  8BDF              mov bx,di
000062B4  83B9D95BFF        cmp word [bx+di+0x5bd9],0xffffffffffffffff
000062B9  75F0              jnz 0x62ab
000062BB  893E0AA0          mov [0xa00a],di
000062BF  E967FF            jmp 0x6229
000062C2  A0FE9F            mov al,[0x9ffe]
000062C5  0AC0              or al,al
000062C7  741A              jz 0x62e3
000062C9  8A0E08A0          mov cl,[0xa008]
000062CD  D0E1              shl cl,0x0
000062CF  D0E1              shl cl,0x0
000062D1  D0E1              shl cl,0x0
000062D3  3AC1              cmp al,cl
000062D5  740C              jz 0x62e3
000062D7  2A0608A0          sub al,[0xa008]
000062DB  A2FE9F            mov [0x9ffe],al
000062DE  E85401            call 0x6435
000062E1  EBDF              jmp 0x62c2
000062E3  C606FE9F48        mov byte [0x9ffe],0x48
000062E8  C60608A009        mov byte [0xa008],0x9
000062ED  C7060AA00101      mov word [0xa00a],0x101
000062F3  BFD79B            mov di,0x9bd7
000062F6  90                nop
000062F7  B8FFFF            mov ax,0xffff
000062FA  FD                std
000062FB  B9001F            mov cx,0x1f00
000062FE  F3AB              rep stosw
00006300  BE6BD0            mov si,0xd06b
00006303  B1FF              mov cl,0xff
00006305  40                inc ax
00006306  880C              mov [si],cl
00006308  AB                stosw
00006309  4E                dec si
0000630A  49                dec cx
0000630B  79F9              jns 0x6306
0000630D  FC                cld
0000630E  E82401            call 0x6435
00006311  A30CA0            mov [0xa00c],ax
00006314  A21DA0            mov [0xa01d],al
00006317  8B3E1EA0          mov di,[0xa01e]
0000631B  81FFD95B          cmp di,0x5bd9
0000631F  7503              jnz 0x6324
00006321  E83E09            call 0x6c62
00006324  AA                stosb
00006325  893E1EA0          mov [0xa01e],di
00006329  C3                ret
0000632A  8B0E1EA0          mov cx,[0xa01e]
0000632E  E83809            call 0x6c69
00006331  C3                ret
00006332  2EC706F1630201    mov word [cs:0x63f1],0x102
00006339  E8ACFF            call 0x62e8
0000633C  C7060EA00002      mov word [0xa00e],0x200
00006342  A10CA0            mov ax,[0xa00c]
00006345  A304A0            mov [0xa004],ax
00006348  E8EA00            call 0x6435
0000634B  A30CA0            mov [0xa00c],ax
0000634E  BE6EFF            mov si,0xff6e
00006351  3D0001            cmp ax,0x100
00006354  74E3              jz 0x6339
00006356  7245              jc 0x639d
00006358  8BF8              mov di,ax
0000635A  8BDF              mov bx,di
0000635C  803E21A00A        cmp byte [0xa021],0xa
00006361  7505              jnz 0x6368
00006363  3D0101            cmp ax,0x101
00006366  74C2              jz 0x632a
00006368  83B9D95BFF        cmp word [bx+di+0x5bd9],0xffffffffffffffff
0000636D  7411              jz 0x6380
0000636F  4E                dec si
00006370  8A856CCF          mov al,[di-0x3094]
00006374  8804              mov [si],al
00006376  8BB9D95B          mov di,[bx+di+0x5bd9]
0000637A  81FF0020          cmp di,0x2000
0000637E  720A              jc 0x638a
00006380  4E                dec si
00006381  A01DA0            mov al,[0xa01d]
00006384  8804              mov [si],al
00006386  8B3E04A0          mov di,[0xa004]
0000638A  8BDF              mov bx,di
0000638C  83B9D95BFF        cmp word [bx+di+0x5bd9],0xffffffffffffffff
00006391  7497              jz 0x632a
00006393  81FF0101          cmp di,0x101
00006397  73CF              jnc 0x6368
00006399  8A856CCF          mov al,[di-0x3094]
0000639D  E888FD            call 0x6128
000063A0  893E1EA0          mov [0xa01e],di
000063A4  8B3E0AA0          mov di,[0xa00a]
000063A8  81FF0020          cmp di,0x2000
000063AC  7D94              jnl 0x6342
000063AE  8BDF              mov bx,di
000063B0  A104A0            mov ax,[0xa004]
000063B3  8981D95B          mov [bx+di+0x5bd9],ax
000063B7  A01DA0            mov al,[0xa01d]
000063BA  88856CCF          mov [di-0x3094],al
000063BE  47                inc di
000063BF  893E0AA0          mov [0xa00a],di
000063C3  81FF0020          cmp di,0x2000
000063C7  7D21              jnl 0x63ea
000063C9  3B3E0EA0          cmp di,[0xa00e]
000063CD  751B              jnz 0x63ea
000063CF  FE0608A0          inc byte [0xa008]
000063D3  D1260EA0          shl word [0xa00e],0x0
000063D7  803E21A00A        cmp byte [0xa021],0xa
000063DC  740C              jz 0x63ea
000063DE  A008A0            mov al,[0xa008]
000063E1  D1E0              shl ax,0x0
000063E3  D1E0              shl ax,0x0
000063E5  D1E0              shl ax,0x0
000063E7  A2FE9F            mov [0x9ffe],al
000063EA  E955FF            jmp 0x6342
000063ED  A100A0            mov ax,[0xa000]
000063F0  0B0602A0          or ax,[0xa002]
000063F4  7464              jz 0x645a
000063F6  832E00A001        sub word [0xa000],0x1
000063FB  831E00A000        sbb word [0xa000],0x0
00006400  8B3618A0          mov si,[0xa018]
00006404  81FE6CCF          cmp si,0xcf6c
00006408  7306              jnc 0x6410
0000640A  AC                lodsb
0000640B  893618A0          mov [0xa018],si
0000640F  C3                ret
00006410  53                push bx
00006411  51                push cx
00006412  52                push dx
00006413  B92C2F            mov cx,0x2f2c
00006416  BA40A0            mov dx,0xa040
00006419  E883FB            call 0x5f9f
0000641C  5A                pop dx
0000641D  59                pop cx
0000641E  5B                pop bx
0000641F  73E9              jnc 0x640a
00006421  EB37              jmp 0x645a
00006423  A008A0            mov al,[0xa008]
00006426  2806FE9F          sub [0x9ffe],al
0000642A  7509              jnz 0x6435
0000642C  D1E0              shl ax,0x0
0000642E  D1E0              shl ax,0x0
00006430  D1E0              shl ax,0x0
00006432  A2FE9F            mov [0x9ffe],al
00006435  8B0E08A0          mov cx,[0xa008]
00006439  8B161AA0          mov dx,[0xa01a]
0000643D  2AE9              sub ch,cl
0000643F  7C2B              jl 0x646c
00006441  8AD9              mov bl,cl
00006443  32FF              xor bh,bh
00006445  D1E3              shl bx,0x0
00006447  8BC2              mov ax,dx
00006449  2387DB9B          and ax,[bx-0x6425]
0000644D  EB65              jmp 0x64b4
0000644F  B92C2F            mov cx,0x2f2c
00006452  BA40A0            mov dx,0xa040
00006455  E847FB            call 0x5f9f
00006458  732B              jnc 0x6485
0000645A  58                pop ax
0000645B  E9CCFE            jmp 0x632a
0000645E  32F6              xor dh,dh
00006460  B508              mov ch,0x8
00006462  33C0              xor ax,ax
00006464  A300A0            mov [0xa000],ax
00006467  A302A0            mov [0xa002],ax
0000646A  EB2E              jmp 0x649a
0000646C  A100A0            mov ax,[0xa000]
0000646F  0B0602A0          or ax,[0xa002]
00006473  74E5              jz 0x645a
00006475  52                push dx
00006476  F6DD              neg ch
00006478  2ACD              sub cl,ch
0000647A  51                push cx
0000647B  8B3618A0          mov si,[0xa018]
0000647F  81FE6CCF          cmp si,0xcf6c
00006483  73CA              jnc 0x644f
00006485  AD                lodsw
00006486  8BD0              mov dx,ax
00006488  893618A0          mov [0xa018],si
0000648C  832E00A002        sub word [0xa000],0x2
00006491  831E02A000        sbb word [0xa002],0x0
00006496  78C6              js 0x645e
00006498  B510              mov ch,0x10
0000649A  5B                pop bx
0000649B  8ACB              mov cl,bl
0000649D  8ADF              mov bl,bh
0000649F  8BC2              mov ax,dx
000064A1  32FF              xor bh,bh
000064A3  D1E3              shl bx,0x0
000064A5  2387DB9B          and ax,[bx-0x6425]
000064A9  D3E0              shl ax,cl
000064AB  D1EB              shr bx,0x0
000064AD  8ACB              mov cl,bl
000064AF  5B                pop bx
000064B0  0BC3              or ax,bx
000064B2  2AE9              sub ch,cl
000064B4  D3EA              shr dx,cl
000064B6  89161AA0          mov [0xa01a],dx
000064BA  882E09A0          mov [0xa009],ch
000064BE  C3                ret
000064BF  BE00A0            mov si,0xa000
000064C2  837C0200          cmp word [si+0x2],0x0
000064C6  7505              jnz 0x64cd
000064C8  3904              cmp [si],ax
000064CA  7301              jnc 0x64cd
000064CC  AD                lodsw
000064CD  91                xchg ax,cx
000064CE  C3                ret
000064CF  BFFD9B            mov di,0x9bfd
000064D2  33DB              xor bx,bx
000064D4  8BC3              mov ax,bx
000064D6  B108              mov cl,0x8
000064D8  D1E8              shr ax,0x0
000064DA  7303              jnc 0x64df
000064DC  3501A0            xor ax,0xa001
000064DF  E2F7              loop 0x64d8
000064E1  AB                stosw
000064E2  FEC3              inc bl
000064E4  75EE              jnz 0x64d4
000064E6  C3                ret
000064E7  BFFD9B            mov di,0x9bfd
000064EA  33DB              xor bx,bx
000064EC  8BCB              mov cx,bx
000064EE  8BC3              mov ax,bx
000064F0  33D2              xor dx,dx
000064F2  B108              mov cl,0x8
000064F4  D1DA              rcr dx,0x0
000064F6  D1D8              rcr ax,0x0
000064F8  7307              jnc 0x6501
000064FA  352083            xor ax,0x8320
000064FD  81F2B8ED          xor dx,0xedb8
00006501  E2F1              loop 0x64f4
00006503  AB                stosw
00006504  92                xchg ax,dx
00006505  AB                stosw
00006506  FEC3              inc bl
00006508  75E4              jnz 0x64ee
0000650A  C3                ret
0000650B  58                pop ax
0000650C  50                push ax
0000650D  50                push ax
0000650E  06                push es
0000650F  1F                pop ds
00006510  33C0              xor ax,ax
00006512  A314A0            mov [0xa014],ax
00006515  A020A0            mov al,[0xa020]
00006518  0AC0              or al,al
0000651A  75EE              jnz 0x650a
0000651C  33D2              xor dx,dx
0000651E  B8F09B            mov ax,0x9bf0
00006521  E86FFA            call 0x5f93
00006524  E32C              jcxz 0x6552
00006526  3BC8              cmp cx,ax
00006528  7514              jnz 0x653e
0000652A  290604A0          sub [0xa004],ax
0000652E  831E06A000        sbb word [0xa006],0x0
00006533  7904              jns 0x6539
00006535  030E04A0          add cx,[0xa004]
00006539  E82D07            call 0x6c69
0000653C  EBDE              jmp 0x651c
0000653E  B002              mov al,0x2
00006540  E98EEF            jmp 0x54d1
00006543  B110              mov cl,0x10
00006545  2AC8              sub cl,al
00006547  8B1E1AA0          mov bx,[0xa01a]
0000654B  D3EB              shr bx,cl
0000654D  53                push bx
0000654E  91                xchg ax,cx
0000654F  E85601            call 0x66a8
00006552  58                pop ax
00006553  C3                ret
00006554  55                push bp
00006555  8BEC              mov bp,sp
00006557  83EC7E            sub sp,0x7e
0000655A  16                push ss
0000655B  07                pop es
0000655C  8D7EE0            lea di,[bp-0x20]
0000655F  B91000            mov cx,0x10
00006562  33C0              xor ax,ax
00006564  F3AB              rep stosw
00006566  8B4E0A            mov cx,[bp+0xa]
00006569  8B7608            mov si,[bp+0x8]
0000656C  AC                lodsb
0000656D  8BF8              mov di,ax
0000656F  D1E7              shl di,0x0
00006571  FF43DE            inc word [bp+di-0x22]
00006574  E2F6              loop 0x656c
00006576  1E                push ds
00006577  16                push ss
00006578  1F                pop ds
00006579  8D76E0            lea si,[bp-0x20]
0000657C  8D7E9A            lea di,[bp-0x66]
0000657F  33C0              xor ax,ax
00006581  AB                stosw
00006582  B10F              mov cl,0xf
00006584  AD                lodsw
00006585  D3E0              shl ax,cl
00006587  0345FE            add ax,[di-0x2]
0000658A  AB                stosw
0000658B  49                dec cx
0000658C  79F6              jns 0x6584
0000658E  0BC0              or ax,ax
00006590  7404              jz 0x6596
00006592  1F                pop ds
00006593  E9210A            jmp 0x6fb7
00006596  8B4E06            mov cx,[bp+0x6]
00006599  B80100            mov ax,0x1
0000659C  D3E0              shl ax,cl
0000659E  BB1000            mov bx,0x10
000065A1  2BD9              sub bx,cx
000065A3  8BCB              mov cx,bx
000065A5  894E8A            mov [bp-0x76],cx
000065A8  AF                scasw
000065A9  8D769A            lea si,[bp-0x66]
000065AC  D1E8              shr ax,0x0
000065AE  7407              jz 0x65b7
000065B0  D32C              shr word [si],cl
000065B2  46                inc si
000065B3  46                inc si
000065B4  AB                stosw
000065B5  EBF5              jmp 0x65ac
000065B7  8BCB              mov cx,bx
000065B9  B80100            mov ax,0x1
000065BC  D3E0              shl ax,cl
000065BE  D1E8              shr ax,0x0
000065C0  AB                stosw
000065C1  E2FB              loop 0x65be
000065C3  8BCB              mov cx,bx
000065C5  AD                lodsw
000065C6  1F                pop ds
000065C7  1E                push ds
000065C8  07                pop es
000065C9  D3E8              shr ax,cl
000065CB  0BC0              or ax,ax
000065CD  7419              jz 0x65e8
000065CF  BA0100            mov dx,0x1
000065D2  8B4E06            mov cx,[bp+0x6]
000065D5  D3E2              shl dx,cl
000065D7  8BCA              mov cx,dx
000065D9  2BC8              sub cx,ax
000065DB  E30B              jcxz 0x65e8
000065DD  D1E0              shl ax,0x0
000065DF  8B7E04            mov di,[bp+0x4]
000065E2  03F8              add di,ax
000065E4  33C0              xor ax,ax
000065E6  F3AB              rep stosw
000065E8  8B460A            mov ax,[bp+0xa]
000065EB  894688            mov [bp-0x78],ax
000065EE  8BCB              mov cx,bx
000065F0  49                dec cx
000065F1  B80100            mov ax,0x1
000065F4  D3E0              shl ax,cl
000065F6  894684            mov [bp-0x7c],ax
000065F9  33C0              xor ax,ax
000065FB  89468C            mov [bp-0x74],ax
000065FE  EB4B              jmp 0x664b
00006600  037608            add si,[bp+0x8]
00006603  AC                lodsb
00006604  32E4              xor ah,ah
00006606  0AC0              or al,al
00006608  743E              jz 0x6648
0000660A  8BD8              mov bx,ax
0000660C  96                xchg ax,si
0000660D  D1E6              shl si,0x0
0000660F  8B4298            mov ax,[bp+si-0x68]
00006612  8BF8              mov di,ax
00006614  0342BC            add ax,[bp+si-0x44]
00006617  894686            mov [bp-0x7a],ax
0000661A  3B5E06            cmp bx,[bp+0x6]
0000661D  7F37              jg 0x6656
0000661F  91                xchg ax,cx
00006620  2BCF              sub cx,di
00006622  7E1A              jng 0x663e
00006624  D1E7              shl di,0x0
00006626  037E04            add di,[bp+0x4]
00006629  8B468C            mov ax,[bp-0x74]
0000662C  F3AB              rep stosw
0000662E  EB0E              jmp 0x663e
00006630  81C70080          add di,0x8000
00006634  D16690            shl word [bp-0x70],0x0
00006637  E235              loop 0x666e
00006639  8B468C            mov ax,[bp-0x74]
0000663C  8905              mov [di],ax
0000663E  8B4686            mov ax,[bp-0x7a]
00006641  8BFB              mov di,bx
00006643  D1E7              shl di,0x0
00006645  894398            mov [bp+di-0x68],ax
00006648  FF468C            inc word [bp-0x74]
0000664B  8B768C            mov si,[bp-0x74]
0000664E  3B760A            cmp si,[bp+0xa]
00006651  7CAD              jl 0x6600
00006653  E951B2            jmp 0x18a7
00006656  8B4298            mov ax,[bp+si-0x68]
00006659  894690            mov [bp-0x70],ax
0000665C  8A4E8A            mov cl,[bp-0x76]
0000665F  D3E8              shr ax,cl
00006661  D1E0              shl ax,0x0
00006663  97                xchg ax,di
00006664  037E04            add di,[bp+0x4]
00006667  8BCB              mov cx,bx
00006669  2B4E06            sub cx,[bp+0x6]
0000666C  E3CB              jcxz 0x6639
0000666E  833D00            cmp word [di],0x0
00006671  7516              jnz 0x6689
00006673  8B4688            mov ax,[bp-0x78]
00006676  8905              mov [di],ax
00006678  8BF0              mov si,ax
0000667A  FF4688            inc word [bp-0x78]
0000667D  D1E6              shl si,0x0
0000667F  33C0              xor ax,ax
00006681  89840088          mov [si-0x7800],ax
00006685  89840080          mov [si-0x8000],ax
00006689  8B3D              mov di,[di]
0000668B  D1E7              shl di,0x0
0000668D  8B4690            mov ax,[bp-0x70]
00006690  854684            test [bp-0x7c],ax
00006693  749B              jz 0x6630
00006695  81C70088          add di,0x8800
00006699  EB99              jmp 0x6634
0000669B  33C0              xor ax,ax
0000669D  A31AA0            mov [0xa01a],ax
000066A0  A30EA0            mov [0xa00e],ax
000066A3  A30CA0            mov [0xa00c],ax
000066A6  B110              mov cl,0x10
000066A8  8A2E0CA0          mov ch,[0xa00c]
000066AC  8B161AA0          mov dx,[0xa01a]
000066B0  D3E2              shl dx,cl
000066B2  A10EA0            mov ax,[0xa00e]
000066B5  3ACD              cmp cl,ch
000066B7  7E2E              jng 0x66e7
000066B9  2ACD              sub cl,ch
000066BB  D3E0              shl ax,cl
000066BD  0BD0              or dx,ax
000066BF  A100A0            mov ax,[0xa000]
000066C2  0B0602A0          or ax,[0xa002]
000066C6  7432              jz 0x66fa
000066C8  FF0E00A0          dec word [0xa000]
000066CC  7904              jns 0x66d2
000066CE  FF0E02A0          dec word [0xa002]
000066D2  8B3618A0          mov si,[0xa018]
000066D6  AC                lodsb
000066D7  32E4              xor ah,ah
000066D9  81FE30DF          cmp si,0xdf30
000066DD  731F              jnc 0x66fe
000066DF  893618A0          mov [0xa018],si
000066E3  B508              mov ch,0x8
000066E5  EBCE              jmp 0x66b5
000066E7  A30EA0            mov [0xa00e],ax
000066EA  2AE9              sub ch,cl
000066EC  86E9              xchg ch,cl
000066EE  D3E8              shr ax,cl
000066F0  880E0CA0          mov [0xa00c],cl
000066F4  0BC2              or ax,dx
000066F6  A31AA0            mov [0xa01a],ax
000066F9  C3                ret
000066FA  33C0              xor ax,ax
000066FC  EBE5              jmp 0x66e3
000066FE  51                push cx
000066FF  89161AA0          mov [0xa01a],dx
00006703  B9F03E            mov cx,0x3ef0
00006706  96                xchg ax,si
00006707  BA40A0            mov dx,0xa040
0000670A  E894F8            call 0x5fa1
0000670D  8B161AA0          mov dx,[0xa01a]
00006711  59                pop cx
00006712  96                xchg ax,si
00006713  BE40A0            mov si,0xa040
00006716  EBC7              jmp 0x66df
00006718  57                push di
00006719  8A1E1BA0          mov bl,[0xa01b]
0000671D  32FF              xor bh,bh
0000671F  D1E3              shl bx,0x0
00006721  8BBFFD99          mov di,[bx-0x6603]
00006725  B80001            mov ax,0x100
00006728  83FF11            cmp di,0x11
0000672B  731B              jnc 0x6748
0000672D  8A8D0092          mov cl,[di-0x6e00]
00006731  E874FF            call 0x66a8
00006734  97                xchg ax,di
00006735  0BC0              or ax,ax
00006737  740D              jz 0x6746
00006739  48                dec ax
0000673A  50                push ax
0000673B  E805FE            call 0x6543
0000673E  59                pop cx
0000673F  BA0100            mov dx,0x1
00006742  D3E2              shl dx,cl
00006744  03C2              add ax,dx
00006746  5F                pop di
00006747  C3                ret
00006748  D1E8              shr ax,0x0
0000674A  D1E7              shl di,0x0
0000674C  85061AA0          test [0xa01a],ax
00006750  7406              jz 0x6758
00006752  8BBD0088          mov di,[di-0x7800]
00006756  EBD0              jmp 0x6728
00006758  8BBD0080          mov di,[di-0x8000]
0000675C  EBCA              jmp 0x6728
0000675E  8A856296          mov al,[di-0x699e]
00006762  32E4              xor ah,ah
00006764  0AC0              or al,al
00006766  747D              jz 0x67e5
00006768  E8D8FD            call 0x6543
0000676B  D1E7              shl di,0x0
0000676D  03858296          add ax,[di-0x697e]
00006771  5F                pop di
00006772  C3                ret
00006773  57                push di
00006774  833E1EA000        cmp word [0xa01e],0x0
00006779  743B              jz 0x67b6
0000677B  FF0E1EA0          dec word [0xa01e]
0000677F  8B1E1AA0          mov bx,[0xa01a]
00006783  B104              mov cl,0x4
00006785  D3EB              shr bx,cl
00006787  D1E3              shl bx,0x0
00006789  8BBF30DF          mov di,[bx-0x20d0]
0000678D  B81000            mov ax,0x10
00006790  81FFFE01          cmp di,0x1fe
00006794  730A              jnc 0x67a0
00006796  8A8D0090          mov cl,[di-0x7000]
0000679A  E80BFF            call 0x66a8
0000679D  97                xchg ax,di
0000679E  5F                pop di
0000679F  C3                ret
000067A0  D1E8              shr ax,0x0
000067A2  D1E7              shl di,0x0
000067A4  85061AA0          test [0xa01a],ax
000067A8  7406              jz 0x67b0
000067AA  8BBD0088          mov di,[di-0x7800]
000067AE  EBE0              jmp 0x6790
000067B0  8BBD0080          mov di,[di-0x8000]
000067B4  EBDA              jmp 0x6790
000067B6  B010              mov al,0x10
000067B8  E888FD            call 0x6543
000067BB  A31EA0            mov [0xa01e],ax
000067BE  B81392            mov ax,0x9213
000067C1  50                push ax
000067C2  B005              mov al,0x5
000067C4  50                push ax
000067C5  B80392            mov ax,0x9203
000067C8  50                push ax
000067C9  E86400            call 0x6830
000067CC  E8F500            call 0x68c4
000067CF  B81192            mov ax,0x9211
000067D2  50                push ax
000067D3  B005              mov al,0x5
000067D5  50                push ax
000067D6  B8FF91            mov ax,0x91ff
000067D9  50                push ax
000067DA  E85300            call 0x6830
000067DD  EB9C              jmp 0x677b
000067DF  81FFFF00          cmp di,0xff
000067E3  7F03              jg 0x67e8
000067E5  97                xchg ax,di
000067E6  5F                pop di
000067E7  C3                ret
000067E8  8A850095          mov al,[di-0x6b00]
000067EC  32E4              xor ah,ah
000067EE  0AC0              or al,al
000067F0  74F3              jz 0x67e5
000067F2  E84EFD            call 0x6543
000067F5  D1E7              shl di,0x0
000067F7  03852094          add ax,[di-0x6be0]
000067FB  FEC4              inc ah
000067FD  5F                pop di
000067FE  C3                ret
000067FF  81FFC001          cmp di,0x1c0
00006803  72E0              jc 0x67e5
00006805  B001              mov al,0x1
00006807  E839FD            call 0x6543
0000680A  D1E7              shl di,0x0
0000680C  81EFC001          sub di,0x1c0
00006810  03C7              add ax,di
00006812  5F                pop di
00006813  C3                ret
00006814  8A4606            mov al,[bp+0x6]
00006817  E829FD            call 0x6543
0000681A  B90001            mov cx,0x100
0000681D  BFFD99            mov di,0x99fd
00006820  F3AB              rep stosw
00006822  33C0              xor ax,ax
00006824  BF0092            mov di,0x9200
00006827  8B4E08            mov cx,[bp+0x8]
0000682A  2BCF              sub cx,di
0000682C  F3AA              rep stosb
0000682E  EB79              jmp 0x68a9
00006830  55                push bp
00006831  8BEC              mov bp,sp
00006833  83EC04            sub sp,0x4
00006836  8A4606            mov al,[bp+0x6]
00006839  E807FD            call 0x6543
0000683C  0BC0              or ax,ax
0000683E  74D4              jz 0x6814
00006840  BF0092            mov di,0x9200
00006843  03C7              add ax,di
00006845  8946FC            mov [bp-0x4],ax
00006848  3B7EFC            cmp di,[bp-0x4]
0000684B  733D              jnc 0x688a
0000684D  8A1E1BA0          mov bl,[0xa01b]
00006851  B105              mov cl,0x5
00006853  D2EB              shr bl,cl
00006855  80FB07            cmp bl,0x7
00006858  7D19              jnl 0x6873
0000685A  B103              mov cl,0x3
0000685C  53                push bx
0000685D  E848FE            call 0x66a8
00006860  58                pop ax
00006861  AA                stosb
00006862  3B7E04            cmp di,[bp+0x4]
00006865  75E1              jnz 0x6848
00006867  B002              mov al,0x2
00006869  E8D7FC            call 0x6543
0000686C  91                xchg ax,cx
0000686D  32C0              xor al,al
0000686F  F3AA              rep stosb
00006871  EBD5              jmp 0x6848
00006873  750E              jnz 0x6883
00006875  B80010            mov ax,0x1000
00006878  85061AA0          test [0xa01a],ax
0000687C  7405              jz 0x6883
0000687E  D1E8              shr ax,0x0
00006880  43                inc bx
00006881  EBF5              jmp 0x6878
00006883  8BCB              mov cx,bx
00006885  80E903            sub cl,0x3
00006888  EBD2              jmp 0x685c
0000688A  33C0              xor ax,ax
0000688C  3B7E08            cmp di,[bp+0x8]
0000688F  7303              jnc 0x6894
00006891  AA                stosb
00006892  EBF8              jmp 0x688c
00006894  8B5E08            mov bx,[bp+0x8]
00006897  B80092            mov ax,0x9200
0000689A  2BD8              sub bx,ax
0000689C  53                push bx
0000689D  50                push ax
0000689E  B80800            mov ax,0x8
000068A1  50                push ax
000068A2  B8FD99            mov ax,0x99fd
000068A5  50                push ax
000068A6  E8ABFC            call 0x6554
000068A9  E990B4            jmp 0x1d3c
000068AC  B009              mov al,0x9
000068AE  E892FC            call 0x6543
000068B1  B90020            mov cx,0x2000
000068B4  BF30DF            mov di,0xdf30
000068B7  F3AB              rep stosw
000068B9  33C0              xor ax,ax
000068BB  BF0090            mov di,0x9000
000068BE  B9FF00            mov cx,0xff
000068C1  F3AB              rep stosw
000068C3  C3                ret
000068C4  B009              mov al,0x9
000068C6  E87AFC            call 0x6543
000068C9  0BC0              or ax,ax
000068CB  74DF              jz 0x68ac
000068CD  BF0090            mov di,0x9000
000068D0  03C7              add ax,di
000068D2  50                push ax
000068D3  58                pop ax
000068D4  3BF8              cmp di,ax
000068D6  7365              jnc 0x693d
000068D8  50                push ax
000068D9  8A1E1BA0          mov bl,[0xa01b]
000068DD  32FF              xor bh,bh
000068DF  D1E3              shl bx,0x0
000068E1  8BB7FD99          mov si,[bx-0x6603]
000068E5  83FE13            cmp si,0x13
000068E8  7C1C              jl 0x6906
000068EA  B88000            mov ax,0x80
000068ED  D1E6              shl si,0x0
000068EF  85061AA0          test [0xa01a],ax
000068F3  7406              jz 0x68fb
000068F5  8BB40088          mov si,[si-0x7800]
000068F9  EB04              jmp 0x68ff
000068FB  8BB40080          mov si,[si-0x8000]
000068FF  D1E8              shr ax,0x0
00006901  83FE13            cmp si,0x13
00006904  7DE7              jnl 0x68ed
00006906  56                push si
00006907  8A8C0092          mov cl,[si-0x6e00]
0000690B  E89AFD            call 0x66a8
0000690E  58                pop ax
0000690F  3C02              cmp al,0x2
00006911  7F25              jg 0x6938
00006913  0BC0              or ax,ax
00006915  7504              jnz 0x691b
00006917  B001              mov al,0x1
00006919  EB16              jmp 0x6931
0000691B  3C01              cmp al,0x1
0000691D  750A              jnz 0x6929
0000691F  B004              mov al,0x4
00006921  E81FFC            call 0x6543
00006924  050300            add ax,0x3
00006927  EB08              jmp 0x6931
00006929  B009              mov al,0x9
0000692B  E815FC            call 0x6543
0000692E  051400            add ax,0x14
00006931  91                xchg ax,cx
00006932  33C0              xor ax,ax
00006934  F3AA              rep stosb
00006936  EB9B              jmp 0x68d3
00006938  48                dec ax
00006939  48                dec ax
0000693A  AA                stosb
0000693B  EB96              jmp 0x68d3
0000693D  B90092            mov cx,0x9200
00006940  2BCF              sub cx,di
00006942  7604              jna 0x6948
00006944  33C0              xor ax,ax
00006946  F3AA              rep stosb
00006948  B8FE01            mov ax,0x1fe
0000694B  50                push ax
0000694C  B80090            mov ax,0x9000
0000694F  50                push ax
00006950  B80C00            mov ax,0xc
00006953  50                push ax
00006954  B830DF            mov ax,0xdf30
00006957  50                push ax
00006958  E8F9FB            call 0x6554
0000695B  C3                ret
0000695C  55                push bp
0000695D  8BEC              mov bp,sp
0000695F  33C0              xor ax,ax
00006961  50                push ax
00006962  B402              mov ah,0x2
00006964  50                push ax
00006965  57                push di
00006966  BE0900            mov si,0x9
00006969  2EC606BF6A0D      mov byte [cs:0x6abf],0xd
0000696F  EB11              jmp 0x6982
00006971  55                push bp
00006972  8BEC              mov bp,sp
00006974  33C0              xor ax,ax
00006976  50                push ax
00006977  40                inc ax
00006978  50                push ax
00006979  57                push di
0000697A  33F6              xor si,si
0000697C  2EC606BF6A07      mov byte [cs:0x6abf],0x7
00006982  833E08A000        cmp word [0xa008],0x0
00006987  751B              jnz 0x69a4
00006989  A11AA0            mov ax,[0xa01a]
0000698C  8B0E08A0          mov cx,[0xa008]
00006990  09060AA0          or [0xa00a],ax
00006994  83E910            sub cx,0x10
00006997  F7D9              neg cx
00006999  56                push si
0000699A  E80BFD            call 0x66a8
0000699D  5E                pop si
0000699E  C70608A01000      mov word [0xa008],0x10
000069A4  FF0E08A0          dec word [0xa008]
000069A8  33FF              xor di,di
000069AA  D1260AA0          shl word [0xa00a],0x0
000069AE  83D700            adc di,0x0
000069B1  740F              jz 0x69c2
000069B3  8B46FC            mov ax,[bp-0x4]
000069B6  0146FE            add [bp-0x2],ax
000069B9  D166FC            shl word [bp-0x4],0x0
000069BC  46                inc si
000069BD  83FE07            cmp si,0x7
000069C0  7CC0              jl 0x6982
000069C2  0BF6              or si,si
000069C4  7439              jz 0x69ff
000069C6  A108A0            mov ax,[0xa008]
000069C9  3BC6              cmp ax,si
000069CB  7D1D              jnl 0x69ea
000069CD  A11AA0            mov ax,[0xa01a]
000069D0  8B0E08A0          mov cx,[0xa008]
000069D4  D3E8              shr ax,cl
000069D6  09060AA0          or [0xa00a],ax
000069DA  83E910            sub cx,0x10
000069DD  F7D9              neg cx
000069DF  56                push si
000069E0  E8C5FC            call 0x66a8
000069E3  5E                pop si
000069E4  C70608A01000      mov word [0xa008],0x10
000069EA  B91000            mov cx,0x10
000069ED  2BCE              sub cx,si
000069EF  8B3E0AA0          mov di,[0xa00a]
000069F3  D3EF              shr di,cl
000069F5  8BCE              mov cx,si
000069F7  D3260AA0          shl word [0xa00a],cl
000069FB  293608A0          sub [0xa008],si
000069FF  037EFE            add di,[bp-0x2]
00006A02  97                xchg ax,di
00006A03  5F                pop di
00006A04  E933A4            jmp 0xe3a
00006A07  57                push di
00006A08  BE16E4            mov si,0xe416
00006A0B  8BFE              mov di,si
00006A0D  8B841AFB          mov ax,[si-0x4e6]
00006A11  3DE604            cmp ax,0x4e6
00006A14  720B              jc 0x6a21
00006A16  89851AFB          mov [di-0x4e6],ax
00006A1A  AD                lodsw
00006A1B  40                inc ax
00006A1C  D1E8              shr ax,0x0
00006A1E  AB                stosw
00006A1F  EB02              jmp 0x6a23
00006A21  46                inc si
00006A22  46                inc si
00006A23  81FEFCE8          cmp si,0xe8fc
00006A27  72E4              jc 0x6a0d
00006A29  BE16E4            mov si,0xe416
00006A2C  BF8AE6            mov di,0xe68a
00006A2F  AD                lodsw
00006A30  93                xchg ax,bx
00006A31  AD                lodsw
00006A32  03C3              add ax,bx
00006A34  AB                stosw
00006A35  4F                dec di
00006A36  4F                dec di
00006A37  8BDF              mov bx,di
00006A39  4B                dec bx
00006A3A  4B                dec bx
00006A3B  3B07              cmp ax,[bx]
00006A3D  72FA              jc 0x6a39
00006A3F  43                inc bx
00006A40  43                inc bx
00006A41  8BCF              mov cx,di
00006A43  2BCB              sub cx,bx
00006A45  D1E9              shr cx,0x0
00006A47  FD                std
00006A48  56                push si
00006A49  57                push di
00006A4A  51                push cx
00006A4B  8BF7              mov si,di
00006A4D  4E                dec si
00006A4E  4E                dec si
00006A4F  F3A5              rep movsw
00006A51  8907              mov [bx],ax
00006A53  59                pop cx
00006A54  5F                pop di
00006A55  57                push di
00006A56  8DB51AFB          lea si,[di-0x4e6]
00006A5A  8BFE              mov di,si
00006A5C  4E                dec si
00006A5D  4E                dec si
00006A5E  F3A5              rep movsw
00006A60  5F                pop di
00006A61  5E                pop si
00006A62  8BC6              mov ax,si
00006A64  2D1AE4            sub ax,0xe41a
00006A67  89871AFB          mov [bx-0x4e6],ax
00006A6B  47                inc di
00006A6C  47                inc di
00006A6D  FC                cld
00006A6E  81FFFCE8          cmp di,0xe8fc
00006A72  72BB              jc 0x6a2f
00006A74  33F6              xor si,si
00006A76  8BBC30DF          mov di,[si-0x20d0]
00006A7A  89B5FEE8          mov [di-0x1702],si
00006A7E  81FFE604          cmp di,0x4e6
00006A82  7304              jnc 0x6a88
00006A84  89B500E9          mov [di-0x1700],si
00006A88  46                inc si
00006A89  46                inc si
00006A8A  81FEE604          cmp si,0x4e6
00006A8E  72E6              jc 0x6a76
00006A90  5F                pop di
00006A91  C3                ret
00006A92  53                push bx
00006A93  E8AC00            call 0x6b42
00006A96  8AF0              mov dh,al
00006A98  52                push dx
00006A99  E8A600            call 0x6b42
00006A9C  5A                pop dx
00006A9D  8AD0              mov dl,al
00006A9F  B110              mov cl,0x10
00006AA1  5B                pop bx
00006AA2  EB21              jmp 0x6ac5
00006AA4  BB30DF            mov bx,0xdf30
00006AA7  8BBFE404          mov di,[bx+0x4e4]
00006AAB  8B160AA0          mov dx,[0xa00a]
00006AAF  8A0E08A0          mov cl,[0xa008]
00006AB3  32ED              xor ch,ch
00006AB5  EB0C              jmp 0x6ac3
00006AB7  D1EF              shr di,0x0
00006AB9  D1E2              shl dx,0x0
00006ABB  83D700            adc di,0x0
00006ABE  D1E7              shl di,0x0
00006AC0  8B39              mov di,[bx+di]
00006AC2  49                dec cx
00006AC3  E3CD              jcxz 0x6a92
00006AC5  81FFE604          cmp di,0x4e6
00006AC9  72EC              jc 0x6ab7
00006ACB  89160AA0          mov [0xa00a],dx
00006ACF  880E08A0          mov [0xa008],cl
00006AD3  81EFE604          sub di,0x4e6
00006AD7  8BD7              mov dx,di
00006AD9  813EFAE80080      cmp word [0xe8fa],0x8000
00006ADF  7203              jc 0x6ae4
00006AE1  E823FF            call 0x6a07
00006AE4  8BB5E4ED          mov si,[di-0x121c]
00006AE8  8B8416E4          mov ax,[si-0x1bea]
00006AEC  40                inc ax
00006AED  898416E4          mov [si-0x1bea],ax
00006AF1  8D7C02            lea di,[si+0x2]
00006AF4  3B8516E4          cmp ax,[di-0x1bea]
00006AF8  763C              jna 0x6b36
00006AFA  47                inc di
00006AFB  47                inc di
00006AFC  3B8516E4          cmp ax,[di-0x1bea]
00006B00  77F8              ja 0x6afa
00006B02  4F                dec di
00006B03  4F                dec di
00006B04  878516E4          xchg ax,[di-0x1bea]
00006B08  898416E4          mov [si-0x1bea],ax
00006B0C  8B9C30DF          mov bx,[si-0x20d0]
00006B10  89BFFEE8          mov [bx-0x1702],di
00006B14  81FBE604          cmp bx,0x4e6
00006B18  7304              jnc 0x6b1e
00006B1A  89BF00E9          mov [bx-0x1700],di
00006B1E  879D30DF          xchg bx,[di-0x20d0]
00006B22  89B7FEE8          mov [bx-0x1702],si
00006B26  81FBE604          cmp bx,0x4e6
00006B2A  7304              jnc 0x6b30
00006B2C  89B700E9          mov [bx-0x1700],si
00006B30  899C30DF          mov [si-0x20d0],bx
00006B34  8BF7              mov si,di
00006B36  8BB4FEE8          mov si,[si-0x1702]
00006B3A  0BF6              or si,si
00006B3C  75AA              jnz 0x6ae8
00006B3E  92                xchg ax,dx
00006B3F  D1E8              shr ax,0x0
00006B41  C3                ret
00006B42  8B3618A0          mov si,[0xa018]
00006B46  81FE30DF          cmp si,0xdf30
00006B4A  7209              jc 0x6b55
00006B4C  B9F03E            mov cx,0x3ef0
00006B4F  BA40A0            mov dx,0xa040
00006B52  E84AF4            call 0x5f9f
00006B55  AC                lodsb
00006B56  32E4              xor ah,ah
00006B58  893618A0          mov [0xa018],si
00006B5C  C3                ret
00006B5D  B108              mov cl,0x8
00006B5F  E82400            call 0x6b86
00006B62  86C4              xchg al,ah
00006B64  97                xchg ax,di
00006B65  8A950090          mov dl,[di-0x7000]
00006B69  32F6              xor dh,dh
00006B6B  B106              mov cl,0x6
00006B6D  D3E2              shl dx,cl
00006B6F  52                push dx
00006B70  8A8D0091          mov cl,[di-0x6f00]
00006B74  32ED              xor ch,ch
00006B76  49                dec cx
00006B77  49                dec cx
00006B78  E80B00            call 0x6b86
00006B7B  0BC7              or ax,di
00006B7D  D3C0              rol ax,cl
00006B7F  253F00            and ax,0x3f
00006B82  5A                pop dx
00006B83  0BC2              or ax,dx
00006B85  C3                ret
00006B86  8B160AA0          mov dx,[0xa00a]
00006B8A  803E08A008        cmp byte [0xa008],0x8
00006B8F  7F05              jg 0x6b96
00006B91  51                push cx
00006B92  E8B800            call 0x6c4d
00006B95  59                pop cx
00006B96  8BC2              mov ax,dx
00006B98  D3E2              shl dx,cl
00006B9A  89160AA0          mov [0xa00a],dx
00006B9E  280E08A0          sub [0xa008],cl
00006BA2  BAFFFF            mov dx,0xffff
00006BA5  D3EA              shr dx,cl
00006BA7  F7D2              not dx
00006BA9  23C2              and ax,dx
00006BAB  C3                ret
00006BAC  80FD08            cmp ch,0x8
00006BAF  7405              jz 0x6bb6
00006BB1  8ACD              mov cl,ch
00006BB3  E82B02            call 0x6de1
00006BB6  8BCA              mov cx,dx
00006BB8  AD                lodsw
00006BB9  33C2              xor ax,dx
00006BBB  40                inc ax
00006BBC  7403              jz 0x6bc1
00006BBE  E98B03            jmp 0x6f4c
00006BC1  A4                movsb
00006BC2  81FF0080          cmp di,0x8000
00006BC6  7203              jc 0x6bcb
00006BC8  E82B05            call 0x70f6
00006BCB  81FE30DF          cmp si,0xdf30
00006BCF  7203              jc 0x6bd4
00006BD1  E8FD01            call 0x6dd1
00006BD4  E2EB              loop 0x6bc1
00006BD6  58                pop ax
00006BD7  EB0B              jmp 0x6be4
00006BD9  C70608A00000      mov word [0xa008],0x0
00006BDF  8BD6              mov dx,si
00006BE1  BF0000            mov di,0x0
00006BE4  B508              mov ch,0x8
00006BE6  AD                lodsw
00006BE7  92                xchg ax,dx
00006BE8  803E08A000        cmp byte [0xa008],0x0
00006BED  7573              jnz 0x6c62
00006BEF  E8C701            call 0x6db9
00006BF2  D01608A0          rcl byte [0xa008],0x0
00006BF6  E8CD02            call 0x6ec6
00006BF9  E8F901            call 0x6df5
00006BFC  84E4              test ah,ah
00006BFE  750C              jnz 0x6c0c
00006C00  AA                stosb
00006C01  81FF0080          cmp di,0x8000
00006C05  72F2              jc 0x6bf9
00006C07  E8EC04            call 0x70f6
00006C0A  EBED              jmp 0x6bf9
00006C0C  3D0001            cmp ax,0x100
00006C0F  74D7              jz 0x6be8
00006C11  2DFE00            sub ax,0xfe
00006C14  50                push ax
00006C15  E84402            call 0x6e5c
00006C18  91                xchg ax,cx
00006C19  59                pop cx
00006C1A  56                push si
00006C1B  8D75FF            lea si,[di-0x1]
00006C1E  2BF3              sub si,bx
00006C20  81E6FF7F          and si,0x7fff
00006C24  BB0080            mov bx,0x8000
00006C27  2BD9              sub bx,cx
00006C29  3BF3              cmp si,bx
00006C2B  730A              jnc 0x6c37
00006C2D  3BFB              cmp di,bx
00006C2F  7306              jnc 0x6c37
00006C31  F3A4              rep movsb
00006C33  91                xchg ax,cx
00006C34  5E                pop si
00006C35  EBC2              jmp 0x6bf9
00006C37  A4                movsb
00006C38  81E6FF7F          and si,0x7fff
00006C3C  81FF0080          cmp di,0x8000
00006C40  7304              jnc 0x6c46
00006C42  E2F3              loop 0x6c37
00006C44  EBED              jmp 0x6c33
00006C46  50                push ax
00006C47  E8AC04            call 0x70f6
00006C4A  58                pop ax
00006C4B  EBF5              jmp 0x6c42
00006C4D  52                push dx
00006C4E  E8F1FE            call 0x6b42
00006C51  5A                pop dx
00006C52  B108              mov cl,0x8
00006C54  2A0E08A0          sub cl,[0xa008]
00006C58  D3E0              shl ax,cl
00006C5A  0BD0              or dx,ax
00006C5C  800608A008        add byte [0xa008],0x8
00006C61  C3                ret
00006C62  8BCF              mov cx,di
00006C64  EB03              jmp 0x6c69
00006C66  B90080            mov cx,0x8000
00006C69  33F6              xor si,si
00006C6B  E3F4              jcxz 0x6c61
00006C6D  51                push cx
00006C6E  56                push si
00006C6F  BFFD9B            mov di,0x9bfd
00006C72  A021A0            mov al,[0xa021]
00006C75  3C03              cmp al,0x3
00006C77  741B              jz 0x6c94
00006C79  3C05              cmp al,0x5
00006C7B  767B              jna 0x6cf8
00006C7D  3C0A              cmp al,0xa
00006C7F  7477              jz 0x6cf8
00006C81  3C0C              cmp al,0xc
00006C83  750F              jnz 0x6c94
00006C85  33D2              xor dx,dx
00006C87  32E4              xor ah,ah
00006C89  AC                lodsb
00006C8A  03D0              add dx,ax
00006C8C  E2FB              loop 0x6c89
00006C8E  011614A0          add [0xa014],dx
00006C92  EB16              jmp 0x6caa
00006C94  A114A0            mov ax,[0xa014]
00006C97  8B1616A0          mov dx,[0xa016]
00006C9B  8A1C              mov bl,[si]
00006C9D  46                inc si
00006C9E  E8BB04            call 0x715c
00006CA1  E2F8              loop 0x6c9b
00006CA3  A314A0            mov [0xa014],ax
00006CA6  891616A0          mov [0xa016],dx
00006CAA  1E                push ds
00006CAB  8E1E34A0          mov ds,word [0xa034]
00006CAF  803EE5A300        cmp byte [0xa3e5],0x0
00006CB4  7411              jz 0x6cc7
00006CB6  FE0EE1A3          dec byte [0xa3e1]
00006CBA  790B              jns 0x6cc7
00006CBC  A0BABC            mov al,[0xbcba]
00006CBF  A2E1A3            mov [0xa3e1],al
00006CC2  B0DB              mov al,0xdb
00006CC4  E8509B            call 0x817
00006CC7  1F                pop ds
00006CC8  5A                pop dx
00006CC9  59                pop cx
00006CCA  803E1CA000        cmp byte [0xa01c],0x0
00006CCF  751F              jnz 0x6cf0
00006CD1  803E21A006        cmp byte [0xa021],0x6
00006CD6  7407              jz 0x6cdf
00006CD8  803E21A00D        cmp byte [0xa021],0xd
00006CDD  7507              jnz 0x6ce6
00006CDF  803E1DA000        cmp byte [0xa01d],0x0
00006CE4  752B              jnz 0x6d11
00006CE6  8B1E12A0          mov bx,[0xa012]
00006CEA  B440              mov ah,0x40
00006CEC  CD21              int byte 0x21
00006CEE  7203              jc 0x6cf3
00006CF0  8BFA              mov di,dx
00006CF2  C3                ret
00006CF3  B001              mov al,0x1
00006CF5  E9D9E7            jmp 0x54d1
00006CF8  AC                lodsb
00006CF9  8A1E14A0          mov bl,[0xa014]
00006CFD  8AE7              mov ah,bh
00006CFF  33D8              xor bx,ax
00006D01  A015A0            mov al,[0xa015]
00006D04  32E4              xor ah,ah
00006D06  D1E3              shl bx,0x0
00006D08  3301              xor ax,[bx+di]
00006D0A  A314A0            mov [0xa014],ax
00006D0D  E2E9              loop 0x6cf8
00006D0F  EB99              jmp 0x6caa
00006D11  55                push bp
00006D12  1E                push ds
00006D13  07                pop es
00006D14  BD0068            mov bp,0x6800
00006D17  3BCD              cmp cx,bp
00006D19  730F              jnc 0x6d2a
00006D1B  8BF1              mov si,cx
00006D1D  8BFD              mov di,bp
00006D1F  FD                std
00006D20  A6                cmpsb
00006D21  51                push cx
00006D22  F3A4              rep movsb
00006D24  FC                cld
00006D25  59                pop cx
00006D26  2BE9              sub bp,cx
00006D28  EB02              jmp 0x6d2c
00006D2A  33ED              xor bp,bp
00006D2C  33FF              xor di,di
00006D2E  8BF5              mov si,bp
00006D30  AC                lodsb
00006D31  3C0A              cmp al,0xa
00006D33  7420              jz 0x6d55
00006D35  E2F9              loop 0x6d30
00006D37  0BFF              or di,di
00006D39  7404              jz 0x6d3f
00006D3B  0BED              or bp,bp
00006D3D  7507              jnz 0x6d46
00006D3F  5D                pop bp
00006D40  8BCE              mov cx,si
00006D42  33D2              xor dx,dx
00006D44  EBA0              jmp 0x6ce6
00006D46  8BCE              mov cx,si
00006D48  2BCD              sub cx,bp
00006D4A  2BCF              sub cx,di
00006D4C  2BF1              sub si,cx
00006D4E  F3A4              rep movsb
00006D50  8BCF              mov cx,di
00006D52  5D                pop bp
00006D53  EBED              jmp 0x6d42
00006D55  0BED              or bp,bp
00006D57  751E              jnz 0x6d77
00006D59  4E                dec si
00006D5A  0BF6              or si,si
00006D5C  740A              jz 0x6d68
00006D5E  41                inc cx
00006D5F  51                push cx
00006D60  E8DDFF            call 0x6d40
00006D63  8BEE              mov bp,si
00006D65  59                pop cx
00006D66  EBCD              jmp 0x6d35
00006D68  46                inc si
00006D69  51                push cx
00006D6A  B90200            mov cx,0x2
00006D6D  BAFE9F            mov dx,0x9ffe
00006D70  57                push di
00006D71  E872FF            call 0x6ce6
00006D74  5F                pop di
00006D75  EBEC              jmp 0x6d63
00006D77  51                push cx
00006D78  8BCE              mov cx,si
00006D7A  8BF7              mov si,di
00006D7C  03F5              add si,bp
00006D7E  2BCE              sub cx,si
00006D80  49                dec cx
00006D81  F3A4              rep movsb
00006D83  B00D              mov al,0xd
00006D85  AA                stosb
00006D86  A4                movsb
00006D87  4D                dec bp
00006D88  EBDB              jmp 0x6d65
00006D8A  80F908            cmp cl,0x8
00006D8D  7712              ja 0x6da1
00006D8F  53                push bx
00006D90  33C0              xor ax,ax
00006D92  8BD8              mov bx,ax
00006D94  8AD9              mov bl,cl
00006D96  8A87D29B          mov al,[bx-0x642e]
00006D9A  22C2              and al,dl
00006D9C  E84200            call 0x6de1
00006D9F  5B                pop bx
00006DA0  C3                ret
00006DA1  53                push bx
00006DA2  33DB              xor bx,bx
00006DA4  8AD9              mov bl,cl
00006DA6  B108              mov cl,0x8
00006DA8  2AD9              sub bl,cl
00006DAA  E8E2FF            call 0x6d8f
00006DAD  8ACB              mov cl,bl
00006DAF  8AD8              mov bl,al
00006DB1  E8DBFF            call 0x6d8f
00006DB4  0AF8              or bh,al
00006DB6  93                xchg ax,bx
00006DB7  5B                pop bx
00006DB8  C3                ret
00006DB9  D1EA              shr dx,0x0
00006DBB  FECD              dec ch
00006DBD  7401              jz 0x6dc0
00006DBF  C3                ret
00006DC0  9C                pushf
00006DC1  81FE30DF          cmp si,0xdf30
00006DC5  7203              jc 0x6dca
00006DC7  E80700            call 0x6dd1
00006DCA  8A34              mov dh,[si]
00006DCC  46                inc si
00006DCD  B508              mov ch,0x8
00006DCF  9D                popf
00006DD0  C3                ret
00006DD1  50                push ax
00006DD2  51                push cx
00006DD3  52                push dx
00006DD4  B9F03E            mov cx,0x3ef0
00006DD7  BA40A0            mov dx,0xa040
00006DDA  E8C2F1            call 0x5f9f
00006DDD  5A                pop dx
00006DDE  59                pop cx
00006DDF  58                pop ax
00006DE0  C3                ret
00006DE1  2AE9              sub ch,cl
00006DE3  770D              ja 0x6df2
00006DE5  F6DD              neg ch
00006DE7  2ACD              sub cl,ch
00006DE9  D3EA              shr dx,cl
00006DEB  8ACD              mov cl,ch
00006DED  E8D0FF            call 0x6dc0
00006DF0  2AE9              sub ch,cl
00006DF2  D3EA              shr dx,cl
00006DF4  C3                ret
00006DF5  8ADA              mov bl,dl
00006DF7  32FF              xor bh,bh
00006DF9  D1E3              shl bx,0x0
00006DFB  8B9F90E0          mov bx,[bx-0x1f70]
00006DFF  85DB              test bx,bx
00006E01  780E              js 0x6e11
00006E03  8A8F30DF          mov cl,[bx-0x20d0]
00006E07  E8D7FF            call 0x6de1
00006E0A  93                xchg ax,bx
00006E0B  3D0901            cmp ax,0x109
00006E0E  7309              jnc 0x6e19
00006E10  C3                ret
00006E11  B890E4            mov ax,0xe490
00006E14  E82600            call 0x6e3d
00006E17  EBEE              jmp 0x6e07
00006E19  3D1D01            cmp ax,0x11d
00006E1C  741B              jz 0x6e39
00006E1E  2D0101            sub ax,0x101
00006E21  8AC8              mov cl,al
00006E23  D0E9              shr cl,0x0
00006E25  D0E9              shr cl,0x0
00006E27  49                dec cx
00006E28  250300            and ax,0x3
00006E2B  0404              add al,0x4
00006E2D  D3E0              shl ax,cl
00006E2F  050101            add ax,0x101
00006E32  93                xchg ax,bx
00006E33  E854FF            call 0x6d8a
00006E36  03C3              add ax,bx
00006E38  C3                ret
00006E39  B80002            mov ax,0x200
00006E3C  C3                ret
00006E3D  B108              mov cl,0x8
00006E3F  E89FFF            call 0x6de1
00006E42  56                push si
00006E43  96                xchg ax,si
00006E44  8AC2              mov al,dl
00006E46  32C9              xor cl,cl
00006E48  F7D3              not bx
00006E4A  FEC1              inc cl
00006E4C  D1EB              shr bx,0x0
00006E4E  D1E8              shr ax,0x0
00006E50  D1D3              rcl bx,0x0
00006E52  D1E3              shl bx,0x0
00006E54  8B18              mov bx,[bx+si]
00006E56  85DB              test bx,bx
00006E58  78EE              js 0x6e48
00006E5A  5E                pop si
00006E5B  C3                ret
00006E5C  8ADA              mov bl,dl
00006E5E  32FF              xor bh,bh
00006E60  D1E3              shl bx,0x0
00006E62  8B9F90E2          mov bx,[bx-0x1d70]
00006E66  85DB              test bx,bx
00006E68  781F              js 0x6e89
00006E6A  8A8F70E0          mov cl,[bx-0x1f90]
00006E6E  E870FF            call 0x6de1
00006E71  80FB04            cmp bl,0x4
00006E74  7212              jc 0x6e88
00006E76  93                xchg ax,bx
00006E77  8AC8              mov cl,al
00006E79  D0E9              shr cl,0x0
00006E7B  49                dec cx
00006E7C  2401              and al,0x1
00006E7E  0402              add al,0x2
00006E80  D3E0              shl ax,cl
00006E82  93                xchg ax,bx
00006E83  E804FF            call 0x6d8a
00006E86  03D8              add bx,ax
00006E88  C3                ret
00006E89  B810E9            mov ax,0xe910
00006E8C  E8AEFF            call 0x6e3d
00006E8F  EBDD              jmp 0x6e6e
00006E91  56                push si
00006E92  51                push cx
00006E93  BF30DF            mov di,0xdf30
00006E96  B99000            mov cx,0x90
00006E99  B008              mov al,0x8
00006E9B  F3AA              rep stosb
00006E9D  B170              mov cl,0x70
00006E9F  40                inc ax
00006EA0  F3AA              rep stosb
00006EA2  B118              mov cl,0x18
00006EA4  B007              mov al,0x7
00006EA6  F3AA              rep stosb
00006EA8  B108              mov cl,0x8
00006EAA  40                inc ax
00006EAB  F3AA              rep stosb
00006EAD  BF70E0            mov di,0xe070
00006EB0  B120              mov cl,0x20
00006EB2  890E18EA          mov [0xea18],cx
00006EB6  B005              mov al,0x5
00006EB8  F3AA              rep stosb
00006EBA  C7062EEA2001      mov word [0xea2e],0x120
00006EC0  E9D200            jmp 0x6f95
00006EC3  E9E6FC            jmp 0x6bac
00006EC6  B102              mov cl,0x2
00006EC8  E8BFFE            call 0x6d8a
00006ECB  48                dec ax
00006ECC  78F5              js 0x6ec3
00006ECE  57                push di
00006ECF  74C0              jz 0x6e91
00006ED1  48                dec ax
00006ED2  7578              jnz 0x6f4c
00006ED4  B105              mov cl,0x5
00006ED6  E8B1FE            call 0x6d8a
00006ED9  050101            add ax,0x101
00006EDC  A32EEA            mov [0xea2e],ax
00006EDF  B105              mov cl,0x5
00006EE1  E8A6FE            call 0x6d8a
00006EE4  40                inc ax
00006EE5  A318EA            mov [0xea18],ax
00006EE8  51                push cx
00006EE9  BF1AEA            mov di,0xea1a
00006EEC  B91300            mov cx,0x13
00006EEF  32C0              xor al,al
00006EF1  F3AA              rep stosb
00006EF3  59                pop cx
00006EF4  B104              mov cl,0x4
00006EF6  E891FE            call 0x6d8a
00006EF9  0404              add al,0x4
00006EFB  BFC09B            mov di,0x9bc0
00006EFE  8BEF              mov bp,di
00006F00  03E8              add bp,ax
00006F02  33DB              xor bx,bx
00006F04  B103              mov cl,0x3
00006F06  E881FE            call 0x6d8a
00006F09  8A1D              mov bl,[di]
00006F0B  88871AEA          mov [bx-0x15e6],al
00006F0F  47                inc di
00006F10  3BFD              cmp di,bp
00006F12  72F0              jc 0x6f04
00006F14  56                push si
00006F15  51                push cx
00006F16  BF34EA            mov di,0xea34
00006F19  BE1AEA            mov si,0xea1a
00006F1C  B81300            mov ax,0x13
00006F1F  E89A00            call 0x6fbc
00006F22  59                pop cx
00006F23  5E                pop si
00006F24  8B2E2EEA          mov bp,[0xea2e]
00006F28  032E18EA          add bp,[0xea18]
00006F2C  BF30DF            mov di,0xdf30
00006F2F  32FF              xor bh,bh
00006F31  8ADA              mov bl,dl
00006F33  D1E3              shl bx,0x0
00006F35  8B9F34EA          mov bx,[bx-0x15cc]
00006F39  8A8F1AEA          mov cl,[bx-0x15e6]
00006F3D  E8A1FE            call 0x6de1
00006F40  8AC3              mov al,bl
00006F42  3C10              cmp al,0x10
00006F44  7308              jnc 0x6f4e
00006F46  AA                stosb
00006F47  4D                dec bp
00006F48  75E5              jnz 0x6f2f
00006F4A  EB37              jmp 0x6f83
00006F4C  EB69              jmp 0x6fb7
00006F4E  770C              ja 0x6f5c
00006F50  B102              mov cl,0x2
00006F52  E835FE            call 0x6d8a
00006F55  0403              add al,0x3
00006F57  8A4DFF            mov cl,[di-0x1]
00006F5A  EB17              jmp 0x6f73
00006F5C  3C11              cmp al,0x11
00006F5E  7709              ja 0x6f69
00006F60  B103              mov cl,0x3
00006F62  E825FE            call 0x6d8a
00006F65  0403              add al,0x3
00006F67  EB08              jmp 0x6f71
00006F69  B107              mov cl,0x7
00006F6B  E81CFE            call 0x6d8a
00006F6E  050B00            add ax,0xb
00006F71  32C9              xor cl,cl
00006F73  51                push cx
00006F74  86C1              xchg al,cl
00006F76  32ED              xor ch,ch
00006F78  2BE9              sub bp,cx
00006F7A  72D0              jc 0x6f4c
00006F7C  F3AA              rep stosb
00006F7E  59                pop cx
00006F7F  85ED              test bp,bp
00006F81  75AC              jnz 0x6f2f
00006F83  56                push si
00006F84  51                push cx
00006F85  BE30DF            mov si,0xdf30
00006F88  BF70E0            mov di,0xe070
00006F8B  03362EEA          add si,[0xea2e]
00006F8F  8B0E18EA          mov cx,[0xea18]
00006F93  F3A4              rep movsb
00006F95  A12EEA            mov ax,[0xea2e]
00006F98  BE30DF            mov si,0xdf30
00006F9B  BF90E0            mov di,0xe090
00006F9E  BD90E4            mov bp,0xe490
00006FA1  E81800            call 0x6fbc
00006FA4  A118EA            mov ax,[0xea18]
00006FA7  BE70E0            mov si,0xe070
00006FAA  BF90E2            mov di,0xe290
00006FAD  BD10E9            mov bp,0xe910
00006FB0  E80900            call 0x6fbc
00006FB3  59                pop cx
00006FB4  5E                pop si
00006FB5  5F                pop di
00006FB6  C3                ret
00006FB7  B003              mov al,0x3
00006FB9  E915E5            jmp 0x54d1
00006FBC  85C0              test ax,ax
00006FBE  74F6              jz 0x6fb6
00006FC0  52                push dx
00006FC1  A3F4E9            mov [0xe9f4],ax
00006FC4  893E30EA          mov [0xea30],di
00006FC8  BFF6E9            mov di,0xe9f6
00006FCB  57                push di
00006FCC  B91000            mov cx,0x10
00006FCF  33C0              xor ax,ax
00006FD1  F3AB              rep stosw
00006FD3  5F                pop di
00006FD4  56                push si
00006FD5  8B0EF4E9          mov cx,[0xe9f4]
00006FD9  33DB              xor bx,bx
00006FDB  AC                lodsb
00006FDC  8AD8              mov bl,al
00006FDE  D1E3              shl bx,0x0
00006FE0  FF01              inc word [bx+di]
00006FE2  E2F7              loop 0x6fdb
00006FE4  BED0E9            mov si,0xe9d0
00006FE7  BB0200            mov bx,0x2
00006FEA  33C0              xor ax,ax
00006FEC  8900              mov [bx+si],ax
00006FEE  B10F              mov cl,0xf
00006FF0  0387F6E9          add ax,[bx-0x160a]
00006FF4  D1E0              shl ax,0x0
00006FF6  43                inc bx
00006FF7  43                inc bx
00006FF8  8900              mov [bx+si],ax
00006FFA  E2F4              loop 0x6ff0
00006FFC  833800            cmp word [bx+si],0x0
00006FFF  7412              jz 0x7013
00007001  BEF8E9            mov si,0xe9f8
00007004  B90F00            mov cx,0xf
00007007  33DB              xor bx,bx
00007009  AD                lodsw
0000700A  03D8              add bx,ax
0000700C  E2FB              loop 0x7009
0000700E  83FB01            cmp bx,0x1
00007011  77A4              ja 0x6fb7
00007013  5E                pop si
00007014  56                push si
00007015  8B0EF4E9          mov cx,[0xe9f4]
00007019  BF6099            mov di,0x9960
0000701C  AC                lodsb
0000701D  32E4              xor ah,ah
0000701F  85C0              test ax,ax
00007021  740E              jz 0x7031
00007023  8BD8              mov bx,ax
00007025  D1E3              shl bx,0x0
00007027  8B87D0E9          mov ax,[bx-0x1630]
0000702B  40                inc ax
0000702C  8987D0E9          mov [bx-0x1630],ax
00007030  48                dec ax
00007031  AB                stosw
00007032  E2E8              loop 0x701c
00007034  5E                pop si
00007035  56                push si
00007036  BF6099            mov di,0x9960
00007039  8B16F4E9          mov dx,[0xe9f4]
0000703D  AC                lodsb
0000703E  8AC8              mov cl,al
00007040  49                dec cx
00007041  7817              js 0x705a
00007043  7415              jz 0x705a
00007045  8B1D              mov bx,[di]
00007047  33C0              xor ax,ax
00007049  D1EB              shr bx,0x0
0000704B  D1D0              rcl ax,0x0
0000704D  E0FA              loopne 0x7049
0000704F  41                inc cx
00007050  D1EB              shr bx,0x0
00007052  D3D0              rcl ax,cl
00007054  AB                stosw
00007055  4A                dec dx
00007056  75E5              jnz 0x703d
00007058  EB07              jmp 0x7061
0000705A  47                inc di
0000705B  47                inc di
0000705C  33C9              xor cx,cx
0000705E  4A                dec dx
0000705F  75DC              jnz 0x703d
00007061  5E                pop si
00007062  8B3E30EA          mov di,[0xea30]
00007066  B90001            mov cx,0x100
00007069  33C0              xor ax,ax
0000706B  F3AB              rep stosw
0000706D  BF6099            mov di,0x9960
00007070  8B16F4E9          mov dx,[0xe9f4]
00007074  A3F4E9            mov [0xe9f4],ax
00007077  4A                dec dx
00007078  03F2              add si,dx
0000707A  03FA              add di,dx
0000707C  03FA              add di,dx
0000707E  FD                std
0000707F  AC                lodsb
00007080  84C0              test al,al
00007082  741E              jz 0x70a2
00007084  3C08              cmp al,0x8
00007086  7722              ja 0x70aa
00007088  91                xchg ax,cx
00007089  B80100            mov ax,0x1
0000708C  41                inc cx
0000708D  D3E0              shl ax,cl
0000708F  8B1D              mov bx,[di]
00007091  D1E3              shl bx,0x0
00007093  56                push si
00007094  8B3630EA          mov si,[0xea30]
00007098  8910              mov [bx+si],dx
0000709A  03D8              add bx,ax
0000709C  80FF02            cmp bh,0x2
0000709F  72F7              jc 0x7098
000070A1  5E                pop si
000070A2  4F                dec di
000070A3  4F                dec di
000070A4  4A                dec dx
000070A5  79D8              jns 0x707f
000070A7  FC                cld
000070A8  5A                pop dx
000070A9  C3                ret
000070AA  2C08              sub al,0x8
000070AC  8AC8              mov cl,al
000070AE  8B05              mov ax,[di]
000070B0  8AD8              mov bl,al
000070B2  32FF              xor bh,bh
000070B4  D1E3              shl bx,0x0
000070B6  031E30EA          add bx,[0xea30]
000070BA  B501              mov ch,0x1
000070BC  56                push si
000070BD  52                push dx
000070BE  833F00            cmp word [bx],0x0
000070C1  751A              jnz 0x70dd
000070C3  8B16F4E9          mov dx,[0xe9f4]
000070C7  8BF2              mov si,dx
000070C9  D1EA              shr dx,0x0
000070CB  F7D2              not dx
000070CD  8917              mov [bx],dx
000070CF  8306F4E904        add word [0xe9f4],0x4
000070D4  33D2              xor dx,dx
000070D6  3E8912            mov [ds:bp+si],dx
000070D9  3E895202          mov [ds:bp+si+0x2],dx
000070DD  8B1F              mov bx,[bx]
000070DF  F7D3              not bx
000070E1  D1E3              shl bx,0x0
000070E3  03DD              add bx,bp
000070E5  84E5              test ch,ah
000070E7  7402              jz 0x70eb
000070E9  43                inc bx
000070EA  43                inc bx
000070EB  D0E5              shl ch,0x0
000070ED  FEC9              dec cl
000070EF  75CD              jnz 0x70be
000070F1  5A                pop dx
000070F2  8917              mov [bx],dx
000070F4  EBAB              jmp 0x70a1
000070F6  53                push bx
000070F7  51                push cx
000070F8  52                push dx
000070F9  56                push si
000070FA  55                push bp
000070FB  E864FB            call 0x6c62
000070FE  5D                pop bp
000070FF  5E                pop si
00007100  5A                pop dx
00007101  59                pop cx
00007102  5B                pop bx
00007103  C3                ret
00007104  56                push si
00007105  A124A0            mov ax,[0xa024]
00007108  8B1626A0          mov dx,[0xa026]
0000710C  E84A00            call 0x7159
0000710F  A324A0            mov [0xa024],ax
00007112  891626A0          mov [0xa026],dx
00007116  32E4              xor ah,ah
00007118  8B362AA0          mov si,[0xa02a]
0000711C  030628A0          add ax,[0xa028]
00007120  83D600            adc si,0x0
00007123  8BF8              mov di,ax
00007125  BB0808            mov bx,0x808
00007128  F7E3              mul bx
0000712A  96                xchg ax,si
0000712B  BB0584            mov bx,0x8405
0000712E  F7E3              mul bx
00007130  03C6              add ax,si
00007132  97                xchg ax,di
00007133  F7E3              mul bx
00007135  03D7              add dx,di
00007137  050100            add ax,0x1
0000713A  83D200            adc dx,0x0
0000713D  A328A0            mov [0xa028],ax
00007140  89162AA0          mov [0xa02a],dx
00007144  8ADE              mov bl,dh
00007146  A12CA0            mov ax,[0xa02c]
00007149  8B162EA0          mov dx,[0xa02e]
0000714D  E80900            call 0x7159
00007150  A32CA0            mov [0xa02c],ax
00007153  89162EA0          mov [0xa02e],dx
00007157  5E                pop si
00007158  C3                ret
00007159  BFFD9B            mov di,0x9bfd
0000715C  32D8              xor bl,al
0000715E  32FF              xor bh,bh
00007160  8AC4              mov al,ah
00007162  8AE2              mov ah,dl
00007164  8AD6              mov dl,dh
00007166  8AF7              mov dh,bh
00007168  D1E3              shl bx,0x0
0000716A  D1E3              shl bx,0x0
0000716C  3301              xor ax,[bx+di]
0000716E  335102            xor dx,[bx+di+0x2]
00007171  C3                ret
00007172  AC                lodsb
00007173  8AD8              mov bl,al
00007175  A12CA0            mov ax,[0xa02c]
00007178  0D0200            or ax,0x2
0000717B  8BD0              mov dx,ax
0000717D  83F201            xor dx,0x1
00007180  F7E2              mul dx
00007182  32DC              xor bl,ah
00007184  885CFF            mov [si-0x1],bl
00007187  C3                ret
00007188  BF24A0            mov di,0xa024
0000718B  B87856            mov ax,0x5678
0000718E  AB                stosw
0000718F  B83412            mov ax,0x1234
00007192  AB                stosw
00007193  B88967            mov ax,0x6789
00007196  AB                stosw
00007197  B84523            mov ax,0x2345
0000719A  AB                stosw
0000719B  B89078            mov ax,0x7890
0000719E  AB                stosw
0000719F  B85634            mov ax,0x3456
000071A2  AB                stosw
000071A3  BE40A0            mov si,0xa040
000071A6  AC                lodsb
000071A7  98                cbw
000071A8  91                xchg ax,cx
000071A9  E308              jcxz 0x71b3
000071AB  AC                lodsb
000071AC  8AD8              mov bl,al
000071AE  E853FF            call 0x7104
000071B1  E2F8              loop 0x71ab
000071B3  B90C00            mov cx,0xc
000071B6  33D2              xor dx,dx
000071B8  E8DBED            call 0x5f96
000071BB  A10A00            mov ax,[0xa]
000071BE  C3                ret
000071BF  E85B01            call 0x731d
000071C2  33C0              xor ax,ax
000071C4  A3E299            mov [0x99e2],ax
000071C7  A2E699            mov [0x99e6],al
000071CA  48                dec ax
000071CB  A3E099            mov [0x99e0],ax
000071CE  832EEA9902        sub word [0x99ea],0x2
000071D3  C706E8990200      mov word [0x99e8],0x2
000071D9  A1E079            mov ax,[0x79e0]
000071DC  86E0              xchg ah,al
000071DE  A3E499            mov [0x99e4],ax
000071E1  C3                ret
000071E2  8B3EEC99          mov di,[0x99ec]
000071E6  D1E7              shl di,0x0
000071E8  D1E7              shl di,0x0
000071EA  D1AD80A8          shr word [di-0x5780],0x0
000071EE  7506              jnz 0x71f6
000071F0  C78580A80100      mov word [di-0x5780],0x1
000071F6  D1AD82A8          shr word [di-0x577e],0x0
000071FA  7506              jnz 0x7202
000071FC  C78582A80100      mov word [di-0x577e],0x1
00007202  C3                ret
00007203  58                pop ax
00007204  5B                pop bx
00007205  50                push ax
00007206  A1E099            mov ax,[0x99e0]
00007209  2B06E299          sub ax,[0x99e2]
0000720D  40                inc ax
0000720E  7415              jz 0x7225
00007210  8BC8              mov cx,ax
00007212  A1E499            mov ax,[0x99e4]
00007215  2B06E299          sub ax,[0x99e2]
00007219  40                inc ax
0000721A  F7E3              mul bx
0000721C  2D0100            sub ax,0x1
0000721F  83DA00            sbb dx,0x0
00007222  F7F1              div cx
00007224  C3                ret
00007225  A1E499            mov ax,[0x99e4]
00007228  2B06E299          sub ax,[0x99e2]
0000722C  40                inc ax
0000722D  740B              jz 0x723a
0000722F  F7E3              mul bx
00007231  2D0100            sub ax,0x1
00007234  83DA00            sbb dx,0x0
00007237  8BC2              mov ax,dx
00007239  C3                ret
0000723A  8BC3              mov ax,bx
0000723C  48                dec ax
0000723D  C3                ret
0000723E  58                pop ax
0000723F  5F                pop di
00007240  5A                pop dx
00007241  5E                pop si
00007242  50                push ax
00007243  A1E099            mov ax,[0x99e0]
00007246  8B1EE299          mov bx,[0x99e2]
0000724A  2BC3              sub ax,bx
0000724C  40                inc ax
0000724D  8BC8              mov cx,ax
0000724F  7402              jz 0x7253
00007251  F7E2              mul dx
00007253  3BD7              cmp dx,di
00007255  7405              jz 0x725c
00007257  F7F7              div di
00007259  48                dec ax
0000725A  EB03              jmp 0x725f
0000725C  B8FFFF            mov ax,0xffff
0000725F  03C3              add ax,bx
00007261  A3E099            mov [0x99e0],ax
00007264  8BC1              mov ax,cx
00007266  0BC0              or ax,ax
00007268  7404              jz 0x726e
0000726A  F7E6              mul si
0000726C  EB02              jmp 0x7270
0000726E  8BD6              mov dx,si
00007270  F7F7              div di
00007272  0106E299          add [0x99e2],ax
00007276  A1E099            mov ax,[0x99e0]
00007279  3306E299          xor ax,[0x99e2]
0000727D  D1E0              shl ax,0x0
0000727F  723D              jc 0x72be
00007281  D126E299          shl word [0x99e2],0x0
00007285  F9                stc
00007286  D116E099          rcl word [0x99e0],0x0
0000728A  A0E699            mov al,[0x99e6]
0000728D  A87F              test al,0x7f
0000728F  7523              jnz 0x72b4
00007291  FF0EEA99          dec word [0x99ea]
00007295  7909              jns 0x72a0
00007297  E8C905            call 0x7863
0000729A  FF0EEA99          dec word [0x99ea]
0000729E  7818              js 0x72b8
000072A0  8B1EE899          mov bx,[0x99e8]
000072A4  FF06E899          inc word [0x99e8]
000072A8  8A87E079          mov al,[bx+0x79e0]
000072AC  F9                stc
000072AD  D0D0              rcl al,0x0
000072AF  A2E699            mov [0x99e6],al
000072B2  EB04              jmp 0x72b8
000072B4  D026E699          shl byte [0x99e6],0x0
000072B8  D116E499          rcl word [0x99e4],0x0
000072BC  EBB8              jmp 0x7276
000072BE  F706E2990040      test word [0x99e2],0x4000
000072C4  7456              jz 0x731c
000072C6  F706E0990040      test word [0x99e0],0x4000
000072CC  754E              jnz 0x731c
000072CE  D126E299          shl word [0x99e2],0x0
000072D2  8126E299FF7F      and word [0x99e2],0x7fff
000072D8  D126E099          shl word [0x99e0],0x0
000072DC  810EE0990180      or word [0x99e0],0x8001
000072E2  8136E4990040      xor word [0x99e4],0x4000
000072E8  A0E699            mov al,[0x99e6]
000072EB  A87F              test al,0x7f
000072ED  7523              jnz 0x7312
000072EF  FF0EEA99          dec word [0x99ea]
000072F3  7909              jns 0x72fe
000072F5  E86B05            call 0x7863
000072F8  FF0EEA99          dec word [0x99ea]
000072FC  7818              js 0x7316
000072FE  8B1EE899          mov bx,[0x99e8]
00007302  FF06E899          inc word [0x99e8]
00007306  8A87E079          mov al,[bx+0x79e0]
0000730A  F9                stc
0000730B  D0D0              rcl al,0x0
0000730D  A2E699            mov [0x99e6],al
00007310  EB04              jmp 0x7316
00007312  D026E699          shl byte [0x99e6],0x0
00007316  D116E499          rcl word [0x99e4],0x0
0000731A  EBA2              jmp 0x72be
0000731C  C3                ret
0000731D  C706EE990100      mov word [0x99ee],0x1
00007323  C706F0990800      mov word [0x99f0],0x8
00007329  33C0              xor ax,ax
0000732B  A3F299            mov [0x99f2],ax
0000732E  A3EC99            mov [0x99ec],ax
00007331  40                inc ax
00007332  A3F499            mov [0x99f4],ax
00007335  A3F699            mov [0x99f6],ax
00007338  B82800            mov ax,0x28
0000733B  B90800            mov cx,0x8
0000733E  BF80A8            mov di,0xa880
00007341  1E                push ds
00007342  07                pop es
00007343  F3AB              rep stosw
00007345  BBFA99            mov bx,0x99fa
00007348  B94000            mov cx,0x40
0000734B  B80100            mov ax,0x1
0000734E  E85F00            call 0x73b0
00007351  BB80A0            mov bx,0xa080
00007354  B90001            mov cx,0x100
00007357  E85600            call 0x73b0
0000735A  BBFA9A            mov bx,0x9afa
0000735D  B94000            mov cx,0x40
00007360  33C0              xor ax,ax
00007362  E84B00            call 0x73b0
00007365  BB80A4            mov bx,0xa480
00007368  B90001            mov cx,0x100
0000736B  E84200            call 0x73b0
0000736E  BB40A0            mov bx,0xa040
00007371  B91000            mov cx,0x10
00007374  E83900            call 0x73b0
00007377  BA1000            mov dx,0x10
0000737A  BE7017            mov si,0x1770
0000737D  B81800            mov ax,0x18
00007380  33FF              xor di,di
00007382  03FA              add di,dx
00007384  0BFF              or di,di
00007386  740A              jz 0x7392
00007388  03FF              add di,di
0000738A  0101              add [bx+di],ax
0000738C  D1EF              shr di,0x0
0000738E  D1EF              shr di,0x0
00007390  EBF2              jmp 0x7384
00007392  397702            cmp [bx+0x2],si
00007395  7285              jc 0x731c
00007397  8BFA              mov di,dx
00007399  03FF              add di,di
0000739B  8BF7              mov si,di
0000739D  4F                dec di
0000739E  03FF              add di,di
000073A0  3BFE              cmp di,si
000073A2  721B              jc 0x73bf
000073A4  833901            cmp word [bx+di],0x1
000073A7  7602              jna 0x73ab
000073A9  D129              shr word [bx+di],0x0
000073AB  83EF02            sub di,0x2
000073AE  EBF0              jmp 0x73a0
000073B0  8BF9              mov di,cx
000073B2  03FF              add di,di
000073B4  8BF7              mov si,di
000073B6  03FB              add di,bx
000073B8  F3AB              rep stosw
000073BA  8BFE              mov di,si
000073BC  83EF02            sub di,0x2
000073BF  03F6              add si,si
000073C1  83EE04            sub si,0x4
000073C4  0BFF              or di,di
000073C6  7452              jz 0x741a
000073C8  8B08              mov cx,[bx+si]
000073CA  034802            add cx,[bx+si+0x2]
000073CD  8909              mov [bx+di],cx
000073CF  83EF02            sub di,0x2
000073D2  EBED              jmp 0x73c1
000073D4  03FF              add di,di
000073D6  8B01              mov ax,[bx+di]
000073D8  D1EF              shr di,0x0
000073DA  743E              jz 0x741a
000073DC  03FF              add di,di
000073DE  2901              sub [bx+di],ax
000073E0  D1EF              shr di,0x0
000073E2  EBF4              jmp 0x73d8
000073E4  8B36F899          mov si,[0x99f8]
000073E8  8BFE              mov di,si
000073EA  2BF0              sub si,ax
000073EC  7F04              jg 0x73f2
000073EE  81C6E079          add si,0x79e0
000073F2  4E                dec si
000073F3  E325              jcxz 0x741a
000073F5  8A04              mov al,[si]
000073F7  8805              mov [di],al
000073F9  47                inc di
000073FA  81FFE079          cmp di,0x79e0
000073FE  750B              jnz 0x740b
00007400  51                push cx
00007401  56                push si
00007402  8BCF              mov cx,di
00007404  E87704            call 0x787e
00007407  33FF              xor di,di
00007409  5E                pop si
0000740A  59                pop cx
0000740B  46                inc si
0000740C  81FEE079          cmp si,0x79e0
00007410  7502              jnz 0x7414
00007412  33F6              xor si,si
00007414  E2DF              loop 0x73f5
00007416  893EF899          mov [0x99f8],di
0000741A  C3                ret
0000741B  E9D001            jmp 0x75ee
0000741E  55                push bp
0000741F  8BEC              mov bp,sp
00007421  83EC06            sub sp,0x6
00007424  C706F8990000      mov word [0x99f8],0x0
0000742A  E892FD            call 0x71bf
0000742D  8B36EC99          mov si,[0x99ec]
00007431  03F6              add si,si
00007433  03F6              add si,si
00007435  81C680A8          add si,0xa880
00007439  AD                lodsw
0000743A  50                push ax
0000743B  0304              add ax,[si]
0000743D  8946FA            mov [bp-0x6],ax
00007440  93                xchg ax,bx
00007441  43                inc bx
00007442  E8C1FD            call 0x7206
00007445  5B                pop bx
00007446  3BD8              cmp bx,ax
00007448  76D1              jna 0x741b
0000744A  33F6              xor si,si
0000744C  8B3EEC99          mov di,[0x99ec]
00007450  03FF              add di,di
00007452  03FF              add di,di
00007454  8B9580A8          mov dx,[di-0x5780]
00007458  57                push di
00007459  8B7EFA            mov di,[bp-0x6]
0000745C  47                inc di
0000745D  E8E3FD            call 0x7243
00007460  5F                pop di
00007461  838580A828        add word [di-0x5780],0x28
00007466  817EFA7017        cmp word [bp-0x6],0x1770
0000746B  7203              jc 0x7470
0000746D  E872FD            call 0x71e2
00007470  D126EC99          shl word [0x99ec],0x0
00007474  8326EC9903        and word [0x99ec],0x3
00007479  8B1E82A4          mov bx,[0xa482]
0000747D  031EEE99          add bx,[0x99ee]
00007481  E882FD            call 0x7206
00007484  3B0682A4          cmp ax,[0xa482]
00007488  7267              jc 0x74f1
0000748A  8B3682A4          mov si,[0xa482]
0000748E  8B16EE99          mov dx,[0x99ee]
00007492  03D6              add dx,si
00007494  8BFA              mov di,dx
00007496  E8AAFD            call 0x7243
00007499  BF80A0            mov di,0xa080
0000749C  8B5D02            mov bx,[di+0x2]
0000749F  E864FD            call 0x7206
000074A2  33D2              xor dx,dx
000074A4  BB0200            mov bx,0x2
000074A7  92                xchg ax,dx
000074A8  03DB              add bx,bx
000074AA  8BC8              mov cx,ax
000074AC  0309              add cx,[bx+di]
000074AE  3BCA              cmp cx,dx
000074B0  7705              ja 0x74b7
000074B2  0301              add ax,[bx+di]
000074B4  83C302            add bx,0x2
000074B7  81FB0002          cmp bx,0x200
000074BB  72EB              jc 0x74a8
000074BD  8BFB              mov di,bx
000074BF  D1EB              shr bx,0x0
000074C1  81EB0001          sub bx,0x100
000074C5  895EFE            mov [bp-0x2],bx
000074C8  8B9580A0          mov dx,[di-0x5f80]
000074CC  03D0              add dx,ax
000074CE  8BF0              mov si,ax
000074D0  8B3E82A0          mov di,[0xa082]
000074D4  E86CFD            call 0x7243
000074D7  BB80A0            mov bx,0xa080
000074DA  BF0001            mov di,0x100
000074DD  037EFE            add di,[bp-0x2]
000074E0  E8F1FE            call 0x73d4
000074E3  833E82A000        cmp word [0xa082],0x0
000074E8  7409              jz 0x74f3
000074EA  8306EE9901        add word [0x99ee],0x1
000074EF  EB08              jmp 0x74f9
000074F1  EB50              jmp 0x7543
000074F3  C706EE990000      mov word [0x99ee],0x0
000074F9  8B46FE            mov ax,[bp-0x2]
000074FC  2D0800            sub ax,0x8
000074FF  7902              jns 0x7503
00007501  33C0              xor ax,ax
00007503  8946FA            mov [bp-0x6],ax
00007506  8B5EFE            mov bx,[bp-0x2]
00007509  83C308            add bx,0x8
0000750C  81FBFF00          cmp bx,0xff
00007510  7207              jc 0x7519
00007512  3DFF00            cmp ax,0xff
00007515  7368              jnc 0x757f
00007517  EB04              jmp 0x751d
00007519  3BC3              cmp ax,bx
0000751B  7362              jnc 0x757f
0000751D  BF0001            mov di,0x100
00007520  03C7              add ax,di
00007522  97                xchg ax,di
00007523  BB80A0            mov bx,0xa080
00007526  03FF              add di,di
00007528  833900            cmp word [bx+di],0x0
0000752B  740E              jz 0x753b
0000752D  8BD0              mov dx,ax
0000752F  BEE803            mov si,0x3e8
00007532  B80100            mov ax,0x1
00007535  8B7EFA            mov di,[bp-0x6]
00007538  E847FE            call 0x7382
0000753B  FF46FA            inc word [bp-0x6]
0000753E  8B46FA            mov ax,[bp-0x6]
00007541  EBC3              jmp 0x7506
00007543  BF80A4            mov di,0xa480
00007546  BB0200            mov bx,0x2
00007549  33D2              xor dx,dx
0000754B  92                xchg ax,dx
0000754C  D1E3              shl bx,0x0
0000754E  8BC8              mov cx,ax
00007550  0309              add cx,[bx+di]
00007552  3BCA              cmp cx,dx
00007554  7705              ja 0x755b
00007556  0301              add ax,[bx+di]
00007558  83C302            add bx,0x2
0000755B  81FB0002          cmp bx,0x200
0000755F  72EB              jc 0x754c
00007561  8BFB              mov di,bx
00007563  D1EB              shr bx,0x0
00007565  81EB0001          sub bx,0x100
00007569  895EFE            mov [bp-0x2],bx
0000756C  8BF0              mov si,ax
0000756E  8B9580A4          mov dx,[di-0x5b80]
00007572  03D6              add dx,si
00007574  8B3E82A4          mov di,[0xa482]
00007578  033EEE99          add di,[0x99ee]
0000757C  E8C4FC            call 0x7243
0000757F  BB80A4            mov bx,0xa480
00007582  BA0001            mov dx,0x100
00007585  BEE803            mov si,0x3e8
00007588  B80100            mov ax,0x1
0000758B  8B7EFE            mov di,[bp-0x2]
0000758E  E8F1FD            call 0x7382
00007591  8B7EFE            mov di,[bp-0x2]
00007594  81C70001          add di,0x100
00007598  03FF              add di,di
0000759A  83BD80A403        cmp word [di-0x5b80],0x3
0000759F  750D              jnz 0x75ae
000075A1  832EEE9901        sub word [0x99ee],0x1
000075A6  7F06              jg 0x75ae
000075A8  C706EE990100      mov word [0x99ee],0x1
000075AE  8A46FE            mov al,[bp-0x2]
000075B1  8B3EF899          mov di,[0x99f8]
000075B5  FF06F899          inc word [0x99f8]
000075B9  8805              mov [di],al
000075BB  813EF899E079      cmp word [0x99f8],0x79e0
000075C1  7507              jnz 0x75ca
000075C3  8B0EF899          mov cx,[0x99f8]
000075C7  E8B402            call 0x787e
000075CA  813EF299E079      cmp word [0x99f2],0x79e0
000075D0  7304              jnc 0x75d6
000075D2  FF06F299          inc word [0x99f2]
000075D6  E954FE            jmp 0x742d
000075D9  8B76FA            mov si,[bp-0x6]
000075DC  8BD6              mov dx,si
000075DE  42                inc dx
000075DF  8BFA              mov di,dx
000075E1  E85FFC            call 0x7243
000075E4  8B0EF899          mov cx,[0x99f8]
000075E8  8BE5              mov sp,bp
000075EA  5D                pop bp
000075EB  E99002            jmp 0x787e
000075EE  3946FA            cmp [bp-0x6],ax
000075F1  76E6              jna 0x75d9
000075F3  8B3EEC99          mov di,[0x99ec]
000075F7  03FF              add di,di
000075F9  03FF              add di,di
000075FB  8BB580A8          mov si,[di-0x5780]
000075FF  838582A828        add word [di-0x577e],0x28
00007604  8B56FA            mov dx,[bp-0x6]
00007607  8BFA              mov di,dx
00007609  47                inc di
0000760A  E836FC            call 0x7243
0000760D  817EFA7017        cmp word [bp-0x6],0x1770
00007612  7203              jc 0x7617
00007614  E8CBFB            call 0x71e2
00007617  F9                stc
00007618  D116EC99          rcl word [0x99ec],0x0
0000761C  8326EC9903        and word [0x99ec],0x3
00007621  A1F699            mov ax,[0x99f6]
00007624  3B06F299          cmp ax,[0x99f2]
00007628  731D              jnc 0x7647
0000762A  BB40A0            mov bx,0xa040
0000762D  BA1000            mov dx,0x10
00007630  BE7017            mov si,0x1770
00007633  B81800            mov ax,0x18
00007636  8B3EF499          mov di,[0x99f4]
0000763A  E845FD            call 0x7382
0000763D  FF06F499          inc word [0x99f4]
00007641  D126F699          shl word [0x99f6],0x0
00007645  EBDA              jmp 0x7621
00007647  8B1E42A0          mov bx,[0xa042]
0000764B  E8B8FB            call 0x7206
0000764E  BF40A0            mov di,0xa040
00007651  BB0200            mov bx,0x2
00007654  33D2              xor dx,dx
00007656  92                xchg ax,dx
00007657  D1E3              shl bx,0x0
00007659  8BC8              mov cx,ax
0000765B  0309              add cx,[bx+di]
0000765D  3BCA              cmp cx,dx
0000765F  7705              ja 0x7666
00007661  0301              add ax,[bx+di]
00007663  83C302            add bx,0x2
00007666  83FB20            cmp bx,0x20
00007669  72EC              jc 0x7657
0000766B  8BFB              mov di,bx
0000766D  D1EB              shr bx,0x0
0000766F  83EB10            sub bx,0x10
00007672  895EFC            mov [bp-0x4],bx
00007675  8BF0              mov si,ax
00007677  038540A0          add ax,[di-0x5fc0]
0000767B  8BD0              mov dx,ax
0000767D  8B3E42A0          mov di,[0xa042]
00007681  E8BFFB            call 0x7243
00007684  BB40A0            mov bx,0xa040
00007687  BA1000            mov dx,0x10
0000768A  BE7017            mov si,0x1770
0000768D  B81800            mov ax,0x18
00007690  8B7EFC            mov di,[bp-0x4]
00007693  E8ECFC            call 0x7382
00007696  8B4EFC            mov cx,[bp-0x4]
00007699  83F901            cmp cx,0x1
0000769C  7636              jna 0x76d4
0000769E  B80100            mov ax,0x1
000076A1  D1E0              shl ax,0x0
000076A3  E2FC              loop 0x76a1
000076A5  3B06F699          cmp ax,[0x99f6]
000076A9  750C              jnz 0x76b7
000076AB  D1E8              shr ax,0x0
000076AD  50                push ax
000076AE  8B1EF299          mov bx,[0x99f2]
000076B2  2BD8              sub bx,ax
000076B4  93                xchg ax,bx
000076B5  EB03              jmp 0x76ba
000076B7  D1E8              shr ax,0x0
000076B9  50                push ax
000076BA  8946FE            mov [bp-0x2],ax
000076BD  8BD8              mov bx,ax
000076BF  E844FB            call 0x7206
000076C2  8946FC            mov [bp-0x4],ax
000076C5  8BF0              mov si,ax
000076C7  40                inc ax
000076C8  8BD0              mov dx,ax
000076CA  8B7EFE            mov di,[bp-0x2]
000076CD  E873FB            call 0x7243
000076D0  58                pop ax
000076D1  0146FC            add [bp-0x4],ax
000076D4  8B1EFC9A          mov bx,[0x9afc]
000076D8  031EF099          add bx,[0x99f0]
000076DC  E827FB            call 0x7206
000076DF  3B06FC9A          cmp ax,[0x9afc]
000076E3  7267              jc 0x774c
000076E5  8B36FC9A          mov si,[0x9afc]
000076E9  8B16F099          mov dx,[0x99f0]
000076ED  03D6              add dx,si
000076EF  8BFA              mov di,dx
000076F1  E84FFB            call 0x7243
000076F4  8B1EFC99          mov bx,[0x99fc]
000076F8  E80BFB            call 0x7206
000076FB  BFFA99            mov di,0x99fa
000076FE  BB0200            mov bx,0x2
00007701  33D2              xor dx,dx
00007703  92                xchg ax,dx
00007704  D1E3              shl bx,0x0
00007706  8BC8              mov cx,ax
00007708  0309              add cx,[bx+di]
0000770A  3BCA              cmp cx,dx
0000770C  7705              ja 0x7713
0000770E  0301              add ax,[bx+di]
00007710  83C302            add bx,0x2
00007713  81FB8000          cmp bx,0x80
00007717  72EB              jc 0x7704
00007719  8BFB              mov di,bx
0000771B  D1EB              shr bx,0x0
0000771D  83EB40            sub bx,0x40
00007720  895EFE            mov [bp-0x2],bx
00007723  8BF0              mov si,ax
00007725  8B95FA99          mov dx,[di-0x6606]
00007729  03D0              add dx,ax
0000772B  8B3EFC99          mov di,[0x99fc]
0000772F  E811FB            call 0x7243
00007732  BBFA99            mov bx,0x99fa
00007735  BF4000            mov di,0x40
00007738  037EFE            add di,[bp-0x2]
0000773B  E896FC            call 0x73d4
0000773E  833EFC9900        cmp word [0x99fc],0x0
00007743  7409              jz 0x774e
00007745  8306F09908        add word [0x99f0],0x8
0000774A  EB08              jmp 0x7754
0000774C  EB4F              jmp 0x779d
0000774E  C706F0990000      mov word [0x99f0],0x0
00007754  8B46FE            mov ax,[bp-0x2]
00007757  2D0400            sub ax,0x4
0000775A  7302              jnc 0x775e
0000775C  33C0              xor ax,ax
0000775E  8946FA            mov [bp-0x6],ax
00007761  8B5EFE            mov bx,[bp-0x2]
00007764  83C304            add bx,0x4
00007767  83FB3F            cmp bx,0x3f
0000776A  7207              jc 0x7773
0000776C  3D3F00            cmp ax,0x3f
0000776F  732A              jnc 0x779b
00007771  EB04              jmp 0x7777
00007773  3BC3              cmp ax,bx
00007775  7324              jnc 0x779b
00007777  BF4000            mov di,0x40
0000777A  03F8              add di,ax
0000777C  BBFA99            mov bx,0x99fa
0000777F  03FF              add di,di
00007781  833900            cmp word [bx+di],0x0
00007784  740D              jz 0x7793
00007786  97                xchg ax,di
00007787  BA4000            mov dx,0x40
0000778A  BE7017            mov si,0x1770
0000778D  B80100            mov ax,0x1
00007790  E8EFFB            call 0x7382
00007793  FF46FA            inc word [bp-0x6]
00007796  8B46FA            mov ax,[bp-0x6]
00007799  EBC6              jmp 0x7761
0000779B  EB3B              jmp 0x77d8
0000779D  BFFA9A            mov di,0x9afa
000077A0  BB0200            mov bx,0x2
000077A3  33D2              xor dx,dx
000077A5  92                xchg ax,dx
000077A6  D1E3              shl bx,0x0
000077A8  8BC8              mov cx,ax
000077AA  0309              add cx,[bx+di]
000077AC  3BCA              cmp cx,dx
000077AE  7705              ja 0x77b5
000077B0  0301              add ax,[bx+di]
000077B2  83C302            add bx,0x2
000077B5  81FB8000          cmp bx,0x80
000077B9  72EB              jc 0x77a6
000077BB  8BFB              mov di,bx
000077BD  D1EB              shr bx,0x0
000077BF  83EB40            sub bx,0x40
000077C2  895EFE            mov [bp-0x2],bx
000077C5  8BF0              mov si,ax
000077C7  8B95FA9A          mov dx,[di-0x6506]
000077CB  03D0              add dx,ax
000077CD  8B3EFC9A          mov di,[0x9afc]
000077D1  033EF099          add di,[0x99f0]
000077D5  E86BFA            call 0x7243
000077D8  BBFA9A            mov bx,0x9afa
000077DB  BA4000            mov dx,0x40
000077DE  BE7017            mov si,0x1770
000077E1  B80800            mov ax,0x8
000077E4  8B7EFE            mov di,[bp-0x2]
000077E7  E898FB            call 0x7382
000077EA  8B76FE            mov si,[bp-0x2]
000077ED  83C640            add si,0x40
000077F0  03F6              add si,si
000077F2  83BCFA9A18        cmp word [si-0x6506],0x18
000077F7  750D              jnz 0x7806
000077F9  832EF09908        sub word [0x99f0],0x8
000077FE  7706              ja 0x7806
00007800  C706F0990100      mov word [0x99f0],0x1
00007806  8B76FE            mov si,[bp-0x2]
00007809  46                inc si
0000780A  83FE10            cmp si,0x10
0000780D  7507              jnz 0x7816
0000780F  C746FE0F03        mov word [bp-0x2],0x30f
00007814  EB24              jmp 0x783a
00007816  7222              jc 0x783a
00007818  BB1000            mov bx,0x10
0000781B  E8E8F9            call 0x7206
0000781E  96                xchg ax,si
0000781F  8B46FE            mov ax,[bp-0x2]
00007822  2D1000            sub ax,0x10
00007825  B104              mov cl,0x4
00007827  D3E0              shl ax,cl
00007829  03C6              add ax,si
0000782B  050F00            add ax,0xf
0000782E  8946FE            mov [bp-0x2],ax
00007831  8BD6              mov dx,si
00007833  42                inc dx
00007834  BF1000            mov di,0x10
00007837  E809FA            call 0x7243
0000783A  8B4EFE            mov cx,[bp-0x2]
0000783D  83C103            add cx,0x3
00007840  813EF299E079      cmp word [0x99f2],0x79e0
00007846  7312              jnc 0x785a
00007848  010EF299          add [0x99f2],cx
0000784C  813EF299E079      cmp word [0x99f2],0x79e0
00007852  7606              jna 0x785a
00007854  C706F299E079      mov word [0x99f2],0x79e0
0000785A  8B46FC            mov ax,[bp-0x4]
0000785D  E884FB            call 0x73e4
00007860  E9CAFB            jmp 0x742d
00007863  C706E8990000      mov word [0x99e8],0x0
00007869  BAE079            mov dx,0x79e0
0000786C  B90020            mov cx,0x2000
0000786F  E82FE7            call 0x5fa1
00007872  7204              jc 0x7878
00007874  A3EA99            mov [0x99ea],ax
00007877  C3                ret
00007878  B80200            mov ax,0x2
0000787B  E953DC            jmp 0x54d1
0000787E  C706F8990000      mov word [0x99f8],0x0
00007884  E9E2F3            jmp 0x6c69
00007887  B90400            mov cx,0x4
0000788A  880EF3F8          mov [0xf8f3],cl
0000788E  1E                push ds
0000788F  07                pop es
00007890  BFDEF6            mov di,0xf6de
00007893  B010              mov al,0x10
00007895  AA                stosb
00007896  48                dec ax
00007897  F3AA              rep stosb
00007899  C706F1F8C409      mov word [0xf8f1],0x9c4
0000789F  33C0              xor ax,ax
000078A1  A2F4F8            mov [0xf8f4],al
000078A4  A3F5F8            mov [0xf8f5],ax
000078A7  A3EFF8            mov [0xf8ef],ax
000078AA  A3B4A8            mov [0xa8b4],ax
000078AD  A3EBF6            mov [0xf6eb],ax
000078B0  BFE3F6            mov di,0xf6e3
000078B3  AB                stosw
000078B4  AB                stosw
000078B5  B180              mov cl,0x80
000078B7  BFEDF6            mov di,0xf6ed
000078BA  F3AB              rep stosw
000078BC  B91027            mov cx,0x2710
000078BF  8BD9              mov bx,cx
000078C1  890EF7F8          mov [0xf8f7],cx
000078C5  8E06B0A8          mov es,word [0xa8b0]
000078C9  33FF              xor di,di
000078CB  40                inc ax
000078CC  AB                stosw
000078CD  E2FC              loop 0x78cb
000078CF  48                dec ax
000078D0  A3EDF8            mov [0xf8ed],ax
000078D3  8BCB              mov cx,bx
000078D5  B8FFFF            mov ax,0xffff
000078D8  AB                stosw
000078D9  40                inc ax
000078DA  E2FC              loop 0x78d8
000078DC  8E06ACA8          mov es,word [0xa8ac]
000078E0  B8FFFF            mov ax,0xffff
000078E3  33FF              xor di,di
000078E5  B90040            mov cx,0x4000
000078E8  F3AB              rep stosw
000078EA  8E06AAA8          mov es,word [0xa8aa]
000078EE  33FF              xor di,di
000078F0  8BCB              mov cx,bx
000078F2  F3AB              rep stosw
000078F4  8BC3              mov ax,bx
000078F6  40                inc ax
000078F7  AB                stosw
000078F8  3DF77F            cmp ax,0x7ff7
000078FB  75F9              jnz 0x78f6
000078FD  B8FFFF            mov ax,0xffff
00007900  AB                stosw
00007901  1E                push ds
00007902  07                pop es
00007903  BFBEA8            mov di,0xa8be
00007906  B91027            mov cx,0x2710
00007909  F3AB              rep stosw
0000790B  B80A00            mov ax,0xa
0000790E  33D2              xor dx,dx
00007910  8BFA              mov di,dx
00007912  81FF0080          cmp di,0x8000
00007916  744C              jz 0x7964
00007918  57                push di
00007919  B91DF3            mov cx,0xf31d
0000791C  BB0100            mov bx,0x1
0000791F  E88909            call 0x82ab
00007922  53                push bx
00007923  51                push cx
00007924  B9140B            mov cx,0xb14
00007927  33DB              xor bx,bx
00007929  E8EF09            call 0x831b
0000792C  59                pop cx
0000792D  5B                pop bx
0000792E  50                push ax
0000792F  52                push dx
00007930  91                xchg ax,cx
00007931  87D3              xchg dx,bx
00007933  B9A741            mov cx,0x41a7
00007936  33DB              xor bx,bx
00007938  E8E009            call 0x831b
0000793B  5B                pop bx
0000793C  59                pop cx
0000793D  2BC1              sub ax,cx
0000793F  1BD3              sbb dx,bx
00007941  1BD3              sbb dx,bx
00007943  0BD2              or dx,dx
00007945  7806              js 0x794d
00007947  750B              jnz 0x7954
00007949  0BC0              or ax,ax
0000794B  7507              jnz 0x7954
0000794D  05FFFF            add ax,0xffff
00007950  81D2FF7F          adc dx,0x7fff
00007954  8BF0              mov si,ax
00007956  81E6FF3F          and si,0x3fff
0000795A  96                xchg ax,si
0000795B  5F                pop di
0000795C  8E06AEA8          mov es,word [0xa8ae]
00007960  AB                stosw
00007961  96                xchg ax,si
00007962  EBAE              jmp 0x7912
00007964  33C0              xor ax,ax
00007966  A3E299            mov [0x99e2],ax
00007969  A2E699            mov [0x99e6],al
0000796C  48                dec ax
0000796D  A3E099            mov [0x99e0],ax
00007970  832EEA9902        sub word [0x99ea],0x2
00007975  8B1EE899          mov bx,[0x99e8]
00007979  8306E89902        add word [0x99e8],0x2
0000797E  8B87E079          mov ax,[bx+0x79e0]
00007982  86E0              xchg ah,al
00007984  A3E499            mov [0x99e4],ax
00007987  C3                ret
00007988  8E06A4A8          mov es,word [0xa8a4]
0000798C  8BDF              mov bx,di
0000798E  26833901          cmp word [es:bx+di],0x1
00007992  7515              jnz 0x79a9
00007994  8A9DBEA8          mov bl,[di-0x5742]
00007998  32FF              xor bh,bh
0000799A  80BFDEF610        cmp byte [bx-0x922],0x10
0000799F  7204              jc 0x79a5
000079A1  B80200            mov ax,0x2
000079A4  C3                ret
000079A5  B80100            mov ax,0x1
000079A8  C3                ret
000079A9  8A95CECF          mov dl,[di-0x3032]
000079AD  80FAFF            cmp dl,0xff
000079B0  74F3              jz 0x79a5
000079B2  0AD2              or dl,dl
000079B4  741E              jz 0x79d4
000079B6  03FF              add di,di
000079B8  268B0D            mov cx,[es:di]
000079BB  8ADA              mov bl,dl
000079BD  32FF              xor bh,bh
000079BF  43                inc bx
000079C0  8BFB              mov di,bx
000079C2  03FF              add di,di
000079C4  3BF9              cmp di,cx
000079C6  720C              jc 0x79d4
000079C8  F7E7              mul di
000079CA  F7F1              div cx
000079CC  3BD9              cmp bx,cx
000079CE  7504              jnz 0x79d4
000079D0  D1EB              shr bx,0x0
000079D2  03C3              add ax,bx
000079D4  0BC0              or ax,ax
000079D6  74CD              jz 0x79a5
000079D8  C3                ret
000079D9  A0E3F6            mov al,[0xf6e3]
000079DC  32E4              xor ah,ah
000079DE  93                xchg ax,bx
000079DF  8E06AEA8          mov es,word [0xa8ae]
000079E3  03DB              add bx,bx
000079E5  268B07            mov ax,[es:bx]
000079E8  A3B6A8            mov [0xa8b6],ax
000079EB  8A1EE4F6          mov bl,[0xf6e4]
000079EF  32FF              xor bh,bh
000079F1  03D8              add bx,ax
000079F3  81E3FF3F          and bx,0x3fff
000079F7  03DB              add bx,bx
000079F9  268B07            mov ax,[es:bx]
000079FC  A3B8A8            mov [0xa8b8],ax
000079FF  8A1EE5F6          mov bl,[0xf6e5]
00007A03  32FF              xor bh,bh
00007A05  03D8              add bx,ax
00007A07  81E3FF3F          and bx,0x3fff
00007A0B  03DB              add bx,bx
00007A0D  268B07            mov ax,[es:bx]
00007A10  A3BAA8            mov [0xa8ba],ax
00007A13  8A1EE6F6          mov bl,[0xf6e6]
00007A17  32FF              xor bh,bh
00007A19  03D8              add bx,ax
00007A1B  81E3FF3F          and bx,0x3fff
00007A1F  03DB              add bx,bx
00007A21  268B07            mov ax,[es:bx]
00007A24  A3BCA8            mov [0xa8bc],ax
00007A27  33C9              xor cx,cx
00007A29  890EE7F6          mov [0xf6e7],cx
00007A2D  870EEBF6          xchg cx,[0xf6eb]
00007A31  E30F              jcxz 0x7a42
00007A33  8BD9              mov bx,cx
00007A35  8A9FECF7          mov bl,[bx-0x814]
00007A39  32FF              xor bh,bh
00007A3B  C687EDF600        mov byte [bx-0x913],0x0
00007A40  E2F1              loop 0x7a33
00007A42  C706E9F60500      mov word [0xf6e9],0x5
00007A48  8B16E9F6          mov dx,[0xf6e9]
00007A4C  4A                dec dx
00007A4D  7841              js 0x7a90
00007A4F  8BDA              mov bx,dx
00007A51  D1E3              shl bx,0x0
00007A53  8B9FB4A8          mov bx,[bx-0x574c]
00007A57  33FF              xor di,di
00007A59  8E06ACA8          mov es,word [0xa8ac]
00007A5D  03DB              add bx,bx
00007A5F  268B19            mov bx,[es:bx+di]
00007A62  83FBFF            cmp bx,0xffffffffffffffff
00007A65  74E5              jz 0x7a4c
00007A67  8A8FBEA8          mov cl,[bx-0x5742]
00007A6B  32ED              xor ch,ch
00007A6D  3ACA              cmp cl,dl
00007A6F  751A              jnz 0x7a8b
00007A71  E311              jcxz 0x7a84
00007A73  BEE3F6            mov si,0xf6e3
00007A76  8E06B2A8          mov es,word [0xa8b2]
00007A7A  8BFB              mov di,bx
00007A7C  03FF              add di,di
00007A7E  03FF              add di,di
00007A80  F3A6              repe cmpsb
00007A82  7507              jnz 0x7a8b
00007A84  8916E9F6          mov [0xf6e9],dx
00007A88  8BC3              mov ax,bx
00007A8A  C3                ret
00007A8B  BF0080            mov di,0x8000
00007A8E  EBC9              jmp 0x7a59
00007A90  B8FFFF            mov ax,0xffff
00007A93  C3                ret
00007A94  3B3EEFF8          cmp di,[0xf8ef]
00007A98  7501              jnz 0x7a9b
00007A9A  C3                ret
00007A9B  8E06B0A8          mov es,word [0xa8b0]
00007A9F  8BDF              mov bx,di
00007AA1  268B81204E        mov ax,[es:bx+di+0x4e20]
00007AA6  3B3EEDF8          cmp di,[0xf8ed]
00007AAA  7505              jnz 0x7ab1
00007AAC  A3EDF8            mov [0xf8ed],ax
00007AAF  EB19              jmp 0x7aca
00007AB1  268B31            mov si,[es:bx+di]
00007AB4  8BDE              mov bx,si
00007AB6  268980204E        mov [es:bx+si+0x4e20],ax
00007ABB  8BDF              mov bx,di
00007ABD  268B81204E        mov ax,[es:bx+di+0x4e20]
00007AC2  96                xchg ax,si
00007AC3  8BDE              mov bx,si
00007AC5  268900            mov [es:bx+si],ax
00007AC8  8BDF              mov bx,di
00007ACA  A1EFF8            mov ax,[0xf8ef]
00007ACD  268901            mov [es:bx+di],ax
00007AD0  8BD8              mov bx,ax
00007AD2  97                xchg ax,di
00007AD3  268981204E        mov [es:bx+di+0x4e20],ax
00007AD8  A3EFF8            mov [0xf8ef],ax
00007ADB  C3                ret
00007ADC  55                push bp
00007ADD  8BEC              mov bp,sp
00007ADF  8B7E04            mov di,[bp+0x4]
00007AE2  8E06A4A8          mov es,word [0xa8a4]
00007AE6  8BDF              mov bx,di
00007AE8  268B01            mov ax,[es:bx+di]
00007AEB  50                push ax
00007AEC  268A85204E        mov al,[es:di+0x4e20]
00007AF1  32E4              xor ah,ah
00007AF3  E898FE            call 0x798e
00007AF6  8B5EFE            mov bx,[bp-0x2]
00007AF9  50                push ax
00007AFA  C706E7F60100      mov word [0xf6e7],0x1
00007B00  803EF4F805        cmp byte [0xf8f4],0x5
00007B05  724C              jc 0x7b53
00007B07  B101              mov cl,0x1
00007B09  83FB04            cmp bx,0x4
00007B0C  7708              ja 0x7b16
00007B0E  803EF4F80A        cmp byte [0xf8f4],0xa
00007B13  7501              jnz 0x7b16
00007B15  41                inc cx
00007B16  D3E3              shl bx,cl
00007B18  895EFE            mov [bp-0x2],bx
00007B1B  51                push cx
00007B1C  03D8              add bx,ax
00007B1E  E8E5F6            call 0x7206
00007B21  59                pop cx
00007B22  D3E8              shr ax,cl
00007B24  8BF0              mov si,ax
00007B26  33C0              xor ax,ax
00007B28  8B5E04            mov bx,[bp+0x4]
00007B2B  83FBFF            cmp bx,0xffffffffffffffff
00007B2E  7450              jz 0x7b80
00007B30  03DB              add bx,bx
00007B32  8E06A6A8          mov es,word [0xa8a6]
00007B36  268B17            mov dx,[es:bx]
00007B39  03D0              add dx,ax
00007B3B  3BD6              cmp dx,si
00007B3D  770B              ja 0x7b4a
00007B3F  8BC2              mov ax,dx
00007B41  8E06AAA8          mov es,word [0xa8aa]
00007B45  268B1F            mov bx,[es:bx]
00007B48  EBE1              jmp 0x7b2b
00007B4A  268B17            mov dx,[es:bx]
00007B4D  D3E2              shl dx,cl
00007B4F  D3E0              shl ax,cl
00007B51  EB2F              jmp 0x7b82
00007B53  03D8              add bx,ax
00007B55  E8AEF6            call 0x7206
00007B58  8BF0              mov si,ax
00007B5A  33C0              xor ax,ax
00007B5C  8B5E04            mov bx,[bp+0x4]
00007B5F  83FBFF            cmp bx,0xffffffffffffffff
00007B62  746F              jz 0x7bd3
00007B64  03DB              add bx,bx
00007B66  8E06A6A8          mov es,word [0xa8a6]
00007B6A  268B17            mov dx,[es:bx]
00007B6D  8BC8              mov cx,ax
00007B6F  03CA              add cx,dx
00007B71  3BCE              cmp cx,si
00007B73  770D              ja 0x7b82
00007B75  8BC1              mov ax,cx
00007B77  8E06AAA8          mov es,word [0xa8aa]
00007B7B  268B1F            mov bx,[es:bx]
00007B7E  EBDF              jmp 0x7b5f
00007B80  EB51              jmp 0x7bd3
00007B82  D1EB              shr bx,0x0
00007B84  53                push bx
00007B85  8BF0              mov si,ax
00007B87  03D0              add dx,ax
00007B89  8B7EFE            mov di,[bp-0x2]
00007B8C  037EFC            add di,[bp-0x4]
00007B8F  E8B1F6            call 0x7243
00007B92  8B5E04            mov bx,[bp+0x4]
00007B95  891E90A8          mov [0xa890],bx
00007B99  8E06A4A8          mov es,word [0xa8a4]
00007B9D  8BFB              mov di,bx
00007B9F  26833901          cmp word [es:bx+di],0x1
00007BA3  7511              jnz 0x7bb6
00007BA5  8A9FBEA8          mov bl,[bx-0x5742]
00007BA9  32FF              xor bh,bh
00007BAB  80BFDEF600        cmp byte [bx-0x922],0x0
00007BB0  7404              jz 0x7bb6
00007BB2  FE8FDEF6          dec byte [bx-0x922]
00007BB6  5B                pop bx
00007BB7  891E9AA8          mov [0xa89a],bx
00007BBB  8E06A4A8          mov es,word [0xa8a4]
00007BBF  268A873075        mov al,[es:bx+0x7530]
00007BC4  32E4              xor ah,ah
00007BC6  803EF4F80A        cmp byte [0xf8f4],0xa
00007BCB  7375              jnc 0x7c42
00007BCD  FE06F4F8          inc byte [0xf8f4]
00007BD1  EB6F              jmp 0x7c42
00007BD3  8B76FE            mov si,[bp-0x2]
00007BD6  8BD6              mov dx,si
00007BD8  0356FC            add dx,[bp-0x4]
00007BDB  8BFA              mov di,dx
00007BDD  E863F6            call 0x7243
00007BE0  8B7E04            mov di,[bp+0x4]
00007BE3  8E06A4A8          mov es,word [0xa8a4]
00007BE7  8BDF              mov bx,di
00007BE9  26833901          cmp word [es:bx+di],0x1
00007BED  7511              jnz 0x7c00
00007BEF  8A9DBEA8          mov bl,[di-0x5742]
00007BF3  32FF              xor bh,bh
00007BF5  80BFDEF620        cmp byte [bx-0x922],0x20
00007BFA  7304              jnc 0x7c00
00007BFC  FE87DEF6          inc byte [bx-0x922]
00007C00  83FFFF            cmp di,0xffffffffffffffff
00007C03  7429              jz 0x7c2e
00007C05  8E06A4A8          mov es,word [0xa8a4]
00007C09  268A9D3075        mov bl,[es:di+0x7530]
00007C0E  32FF              xor bh,bh
00007C10  8B36EBF6          mov si,[0xf6eb]
00007C14  889CEDF7          mov [si-0x813],bl
00007C18  FF06EBF6          inc word [0xf6eb]
00007C1C  C687EDF601        mov byte [bx-0x913],0x1
00007C21  8E06AAA8          mov es,word [0xa8aa]
00007C25  8BDF              mov bx,di
00007C27  8BC7              mov ax,di
00007C29  268B39            mov di,[es:bx+di]
00007C2C  EBD2              jmp 0x7c00
00007C2E  A39AA8            mov [0xa89a],ax
00007C31  8B4604            mov ax,[bp+0x4]
00007C34  0D0080            or ax,0x8000
00007C37  A390A8            mov [0xa890],ax
00007C3A  C606F4F800        mov byte [0xf8f4],0x0
00007C3F  B80001            mov ax,0x100
00007C42  8BE5              mov sp,bp
00007C44  5D                pop bp
00007C45  C3                ret
00007C46  55                push bp
00007C47  8BEC              mov bp,sp
00007C49  33C0              xor ax,ax
00007C4B  33D2              xor dx,dx
00007C4D  8B7E04            mov di,[bp+0x4]
00007C50  8BF7              mov si,di
00007C52  83FEFF            cmp si,0xffffffffffffffff
00007C55  742B              jz 0x7c82
00007C57  8E06A4A8          mov es,word [0xa8a4]
00007C5B  268A9C3075        mov bl,[es:si+0x7530]
00007C60  32FF              xor bh,bh
00007C62  03F6              add si,si
00007C64  80BFEDF600        cmp byte [bx-0x913],0x0
00007C69  750E              jnz 0x7c79
00007C6B  8E06A6A8          mov es,word [0xa8a6]
00007C6F  260304            add ax,[es:si]
00007C72  26833C03          cmp word [es:si],0x3
00007C76  7301              jnc 0x7c79
00007C78  42                inc dx
00007C79  8E06AAA8          mov es,word [0xa8aa]
00007C7D  268B34            mov si,[es:si]
00007C80  EBD0              jmp 0x7c52
00007C82  50                push ax
00007C83  8BC2              mov ax,dx
00007C85  E800FD            call 0x7988
00007C88  0346FE            add ax,[bp-0x2]
00007C8B  50                push ax
00007C8C  8BD8              mov bx,ax
00007C8E  E875F5            call 0x7206
00007C91  8BF0              mov si,ax
00007C93  33C0              xor ax,ax
00007C95  8B7E04            mov di,[bp+0x4]
00007C98  83FFFF            cmp di,0xffffffffffffffff
00007C9B  747C              jz 0x7d19
00007C9D  8E06A4A8          mov es,word [0xa8a4]
00007CA1  268A9D3075        mov bl,[es:di+0x7530]
00007CA6  32FF              xor bh,bh
00007CA8  80BFEDF600        cmp byte [bx-0x913],0x0
00007CAD  7513              jnz 0x7cc2
00007CAF  8E06A6A8          mov es,word [0xa8a6]
00007CB3  8BDF              mov bx,di
00007CB5  268B11            mov dx,[es:bx+di]
00007CB8  8BC8              mov cx,ax
00007CBA  03CA              add cx,dx
00007CBC  3BCE              cmp cx,si
00007CBE  770D              ja 0x7ccd
00007CC0  8BC1              mov ax,cx
00007CC2  8E06AAA8          mov es,word [0xa8aa]
00007CC6  8BDF              mov bx,di
00007CC8  268B39            mov di,[es:bx+di]
00007CCB  EBCB              jmp 0x7c98
00007CCD  57                push di
00007CCE  8BF0              mov si,ax
00007CD0  03D0              add dx,ax
00007CD2  8B7EFC            mov di,[bp-0x4]
00007CD5  E86BF5            call 0x7243
00007CD8  8B7E04            mov di,[bp+0x4]
00007CDB  8B36E7F6          mov si,[0xf6e7]
00007CDF  03F6              add si,si
00007CE1  89BC90A8          mov [si-0x5770],di
00007CE5  58                pop ax
00007CE6  89849AA8          mov [si-0x5766],ax
00007CEA  8E06A4A8          mov es,word [0xa8a4]
00007CEE  8BDF              mov bx,di
00007CF0  26833901          cmp word [es:bx+di],0x1
00007CF4  7511              jnz 0x7d07
00007CF6  8A9DBEA8          mov bl,[di-0x5742]
00007CFA  32FF              xor bh,bh
00007CFC  80BFDEF600        cmp byte [bx-0x922],0x0
00007D01  7404              jz 0x7d07
00007D03  FE8FDEF6          dec byte [bx-0x922]
00007D07  97                xchg ax,di
00007D08  268A853075        mov al,[es:di+0x7530]
00007D0D  32E4              xor ah,ah
00007D0F  FF06E7F6          inc word [0xf6e7]
00007D13  FE06F4F8          inc byte [0xf8f4]
00007D17  EB7B              jmp 0x7d94
00007D19  8B76FE            mov si,[bp-0x2]
00007D1C  8B56FC            mov dx,[bp-0x4]
00007D1F  8BFA              mov di,dx
00007D21  E81FF5            call 0x7243
00007D24  8B7E04            mov di,[bp+0x4]
00007D27  8E06A4A8          mov es,word [0xa8a4]
00007D2B  8BDF              mov bx,di
00007D2D  26833901          cmp word [es:bx+di],0x1
00007D31  7511              jnz 0x7d44
00007D33  8A9DBEA8          mov bl,[di-0x5742]
00007D37  32FF              xor bh,bh
00007D39  80BFDEF620        cmp byte [bx-0x922],0x20
00007D3E  7304              jnc 0x7d44
00007D40  FE87DEF6          inc byte [bx-0x922]
00007D44  83FFFF            cmp di,0xffffffffffffffff
00007D47  7430              jz 0x7d79
00007D49  8E06A4A8          mov es,word [0xa8a4]
00007D4D  268A9D3075        mov bl,[es:di+0x7530]
00007D52  32FF              xor bh,bh
00007D54  80BFEDF600        cmp byte [bx-0x913],0x0
00007D59  7511              jnz 0x7d6c
00007D5B  8B36EBF6          mov si,[0xf6eb]
00007D5F  889CEDF7          mov [si-0x813],bl
00007D63  FF06EBF6          inc word [0xf6eb]
00007D67  C687EDF601        mov byte [bx-0x913],0x1
00007D6C  8E06AAA8          mov es,word [0xa8aa]
00007D70  8BDF              mov bx,di
00007D72  8BC7              mov ax,di
00007D74  268B39            mov di,[es:bx+di]
00007D77  EBCB              jmp 0x7d44
00007D79  8B36E7F6          mov si,[0xf6e7]
00007D7D  D1E6              shl si,0x0
00007D7F  89849AA8          mov [si-0x5766],ax
00007D83  8B4604            mov ax,[bp+0x4]
00007D86  0D0080            or ax,0x8000
00007D89  898490A8          mov [si-0x5770],ax
00007D8D  FF06E7F6          inc word [0xf6e7]
00007D91  B80001            mov ax,0x100
00007D94  8BE5              mov sp,bp
00007D96  5D                pop bp
00007D97  C3                ret
00007D98  BF0101            mov di,0x101
00007D9B  2B3EEBF6          sub di,[0xf6eb]
00007D9F  8BDF              mov bx,di
00007DA1  E862F4            call 0x7206
00007DA4  92                xchg ax,dx
00007DA5  33DB              xor bx,bx
00007DA7  33C0              xor ax,ax
00007DA9  BEEDF6            mov si,0xf6ed
00007DAC  803800            cmp byte [bx+si],0x0
00007DAF  7505              jnz 0x7db6
00007DB1  3BC2              cmp ax,dx
00007DB3  7307              jnc 0x7dbc
00007DB5  40                inc ax
00007DB6  FEC3              inc bl
00007DB8  75F2              jnz 0x7dac
00007DBA  B701              mov bh,0x1
00007DBC  53                push bx
00007DBD  8BF0              mov si,ax
00007DBF  40                inc ax
00007DC0  8BD0              mov dx,ax
00007DC2  E87EF4            call 0x7243
00007DC5  58                pop ax
00007DC6  C3                ret
00007DC7  8E06AAA8          mov es,word [0xa8aa]
00007DCB  8B3EF5F8          mov di,[0xf8f5]
00007DCF  03FF              add di,di
00007DD1  83C702            add di,0x2
00007DD4  81FF204E          cmp di,0x4e20
00007DD8  7502              jnz 0x7ddc
00007DDA  33FF              xor di,di
00007DDC  26833DFF          cmp word [es:di],0xffffffffffffffff
00007DE0  74EF              jz 0x7dd1
00007DE2  33F6              xor si,si
00007DE4  8B16E7F6          mov dx,[0xf6e7]
00007DE8  03D2              add dx,dx
00007DEA  3BF2              cmp si,dx
00007DEC  770F              ja 0x7dfd
00007DEE  8B8490A8          mov ax,[si-0x5770]
00007DF2  03C0              add ax,ax
00007DF4  3BC7              cmp ax,di
00007DF6  74D9              jz 0x7dd1
00007DF8  83C602            add si,0x2
00007DFB  EBED              jmp 0x7dea
00007DFD  268B35            mov si,[es:di]
00007E00  8E06A6A8          mov es,word [0xa8a6]
00007E04  268B0D            mov cx,[es:di]
00007E07  8BD1              mov dx,cx
00007E09  83FEFF            cmp si,0xffffffffffffffff
00007E0C  7417              jz 0x7e25
00007E0E  03F6              add si,si
00007E10  26390C            cmp [es:si],cx
00007E13  7303              jnc 0x7e18
00007E15  268B0C            mov cx,[es:si]
00007E18  8E06AAA8          mov es,word [0xa8aa]
00007E1C  268B34            mov si,[es:si]
00007E1F  8E06A6A8          mov es,word [0xa8a6]
00007E23  EBE4              jmp 0x7e09
00007E25  41                inc cx
00007E26  3BD1              cmp dx,cx
00007E28  737B              jnc 0x7ea5
00007E2A  8E06AAA8          mov es,word [0xa8aa]
00007E2E  268B35            mov si,[es:di]
00007E31  03F6              add si,si
00007E33  8E06A6A8          mov es,word [0xa8a6]
00007E37  268B14            mov dx,[es:si]
00007E3A  3BD1              cmp dx,cx
00007E3C  730F              jnc 0x7e4d
00007E3E  8E06AAA8          mov es,word [0xa8aa]
00007E42  26833CFF          cmp word [es:si],0xffffffffffffffff
00007E46  7405              jz 0x7e4d
00007E48  268B34            mov si,[es:si]
00007E4B  EBE4              jmp 0x7e31
00007E4D  8E06A6A8          mov es,word [0xa8a6]
00007E51  268915            mov [es:di],dx
00007E54  D1EE              shr si,0x0
00007E56  D1EF              shr di,0x0
00007E58  893EF5F8          mov [0xf8f5],di
00007E5C  8E06A4A8          mov es,word [0xa8a4]
00007E60  268A843075        mov al,[es:si+0x7530]
00007E65  2688853075        mov [es:di+0x7530],al
00007E6A  03F6              add si,si
00007E6C  03FF              add di,di
00007E6E  8E06AAA8          mov es,word [0xa8aa]
00007E72  A1F7F8            mov ax,[0xf8f7]
00007E75  268704            xchg ax,[es:si]
00007E78  268705            xchg ax,[es:di]
00007E7B  A3F7F8            mov [0xf8f7],ax
00007E7E  26833DFF          cmp word [es:di],0xffffffffffffffff
00007E82  7521              jnz 0x7ea5
00007E84  8E06A4A8          mov es,word [0xa8a4]
00007E88  268915            mov [es:di],dx
00007E8B  D1EF              shr di,0x0
00007E8D  C685CECF00        mov byte [di-0x3032],0x0
00007E92  83FA03            cmp dx,0x3
00007E95  7307              jnc 0x7e9e
00007E97  26C685204E01      mov byte [es:di+0x4e20],0x1
00007E9D  C3                ret
00007E9E  26C685204E00      mov byte [es:di+0x4e20],0x0
00007EA4  C3                ret
00007EA5  92                xchg ax,dx
00007EA6  8E06A6A8          mov es,word [0xa8a6]
00007EAA  33D2              xor dx,dx
00007EAC  F7F1              div cx
00007EAE  268905            mov [es:di],ax
00007EB1  8E06A4A8          mov es,word [0xa8a4]
00007EB5  268905            mov [es:di],ax
00007EB8  D1EF              shr di,0x0
00007EBA  3D0300            cmp ax,0x3
00007EBD  7308              jnc 0x7ec7
00007EBF  26C685204E01      mov byte [es:di+0x4e20],0x1
00007EC5  EB06              jmp 0x7ecd
00007EC7  26C685204E00      mov byte [es:di+0x4e20],0x0
00007ECD  C685CECF00        mov byte [di-0x3032],0x0
00007ED2  893EF5F8          mov [0xf8f5],di
00007ED6  8BD7              mov dx,di
00007ED8  03D2              add dx,dx
00007EDA  8E06AAA8          mov es,word [0xa8aa]
00007EDE  8BDF              mov bx,di
00007EE0  268B31            mov si,[es:bx+di]
00007EE3  83FEFF            cmp si,0xffffffffffffffff
00007EE6  7450              jz 0x7f38
00007EE8  03F6              add si,si
00007EEA  8E06A6A8          mov es,word [0xa8a6]
00007EEE  26390C            cmp [es:si],cx
00007EF1  7319              jnc 0x7f0c
00007EF3  8E06AAA8          mov es,word [0xa8aa]
00007EF7  A1F7F8            mov ax,[0xf8f7]
00007EFA  268704            xchg ax,[es:si]
00007EFD  87F2              xchg si,dx
00007EFF  268904            mov [es:si],ax
00007F02  87F2              xchg si,dx
00007F04  96                xchg ax,si
00007F05  D1E8              shr ax,0x0
00007F07  A3F7F8            mov [0xf8f7],ax
00007F0A  EBD7              jmp 0x7ee3
00007F0C  268B04            mov ax,[es:si]
00007F0F  33D2              xor dx,dx
00007F11  F7F1              div cx
00007F13  268904            mov [es:si],ax
00007F16  8E06A4A8          mov es,word [0xa8a4]
00007F1A  8BDF              mov bx,di
00007F1C  260101            add [es:bx+di],ax
00007F1F  3D0300            cmp ax,0x3
00007F22  7305              jnc 0x7f29
00007F24  26FE85204E        inc byte [es:di+0x4e20]
00007F29  FE85CECF          inc byte [di-0x3032]
00007F2D  8BD6              mov dx,si
00007F2F  8E06AAA8          mov es,word [0xa8aa]
00007F33  268B34            mov si,[es:si]
00007F36  EBAB              jmp 0x7ee3
00007F38  C3                ret
00007F39  8B1EE7F6          mov bx,[0xf6e7]
00007F3D  4B                dec bx
00007F3E  7901              jns 0x7f41
00007F40  C3                ret
00007F41  891EE7F6          mov [0xf6e7],bx
00007F45  03DB              add bx,bx
00007F47  8BB79AA8          mov si,[bx-0x5766]
00007F4B  8BBF90A8          mov di,[bx-0x5770]
00007F4F  0BFF              or di,di
00007F51  7953              jns 0x7fa6
00007F53  81E7FF7F          and di,0x7fff
00007F57  833EF7F8FF        cmp word [0xf8f7],0xffffffffffffffff
00007F5C  7509              jnz 0x7f67
00007F5E  56                push si
00007F5F  57                push di
00007F60  51                push cx
00007F61  E863FE            call 0x7dc7
00007F64  59                pop cx
00007F65  5F                pop di
00007F66  5E                pop si
00007F67  8E06AAA8          mov es,word [0xa8aa]
00007F6B  03F6              add si,si
00007F6D  A1F7F8            mov ax,[0xf8f7]
00007F70  268904            mov [es:si],ax
00007F73  8BF0              mov si,ax
00007F75  03F6              add si,si
00007F77  268B04            mov ax,[es:si]
00007F7A  A3F7F8            mov [0xf8f7],ax
00007F7D  26C704FFFF        mov word [es:si],0xffff
00007F82  D1EE              shr si,0x0
00007F84  FE85CECF          inc byte [di-0x3032]
00007F88  8E06A4A8          mov es,word [0xa8a4]
00007F8C  26FE85204E        inc byte [es:di+0x4e20]
00007F91  8BC1              mov ax,cx
00007F93  2688843075        mov [es:si+0x7530],al
00007F98  B80100            mov ax,0x1
00007F9B  8E06A6A8          mov es,word [0xa8a6]
00007F9F  8BDE              mov bx,si
00007FA1  268900            mov [es:bx+si],ax
00007FA4  EB1A              jmp 0x7fc0
00007FA6  8E06A6A8          mov es,word [0xa8a6]
00007FAA  8BDE              mov bx,si
00007FAC  26FF00            inc word [es:bx+si]
00007FAF  268B00            mov ax,[es:bx+si]
00007FB2  8E06A4A8          mov es,word [0xa8a4]
00007FB6  3D0300            cmp ax,0x3
00007FB9  7505              jnz 0x7fc0
00007FBB  26FE8D204E        dec byte [es:di+0x4e20]
00007FC0  8E06A4A8          mov es,word [0xa8a4]
00007FC4  8BDF              mov bx,di
00007FC6  26FF01            inc word [es:bx+di]
00007FC9  51                push cx
00007FCA  268B09            mov cx,[es:bx+di]
00007FCD  03C0              add ax,ax
00007FCF  8A9DCECF          mov bl,[di-0x3032]
00007FD3  32FF              xor bh,bh
00007FD5  43                inc bx
00007FD6  91                xchg ax,cx
00007FD7  33D2              xor dx,dx
00007FD9  50                push ax
00007FDA  F7F3              div bx
00007FDC  8E06A8A8          mov es,word [0xa8a8]
00007FE0  3BC8              cmp cx,ax
00007FE2  7305              jnc 0x7fe9
00007FE4  26FE0D            dec byte [es:di]
00007FE7  EB09              jmp 0x7ff2
00007FE9  26803D04          cmp byte [es:di],0x4
00007FED  7303              jnc 0x7ff2
00007FEF  26FE05            inc byte [es:di]
00007FF2  58                pop ax
00007FF3  59                pop cx
00007FF4  26803D00          cmp byte [es:di],0x0
00007FF8  7405              jz 0x7fff
00007FFA  3D401F            cmp ax,0x1f40
00007FFD  725A              jc 0x8059
00007FFF  26FE05            inc byte [es:di]
00008002  33C0              xor ax,ax
00008004  8E06A4A8          mov es,word [0xa8a4]
00008008  8BDF              mov bx,di
0000800A  268901            mov [es:bx+di],ax
0000800D  268885204E        mov [es:di+0x4e20],al
00008012  8BF7              mov si,di
00008014  83FEFF            cmp si,0xffffffffffffffff
00008017  7440              jz 0x8059
00008019  8E06A6A8          mov es,word [0xa8a6]
0000801D  8BDE              mov bx,si
0000801F  26833801          cmp word [es:bx+si],0x1
00008023  761B              jna 0x8040
00008025  26D128            shr word [es:bx+si],0x0
00008028  268B00            mov ax,[es:bx+si]
0000802B  8E06A4A8          mov es,word [0xa8a4]
0000802F  8BDF              mov bx,di
00008031  260101            add [es:bx+di],ax
00008034  3D0300            cmp ax,0x3
00008037  7315              jnc 0x804e
00008039  26FE85204E        inc byte [es:di+0x4e20]
0000803E  EB0E              jmp 0x804e
00008040  8E06A4A8          mov es,word [0xa8a4]
00008044  8BDF              mov bx,di
00008046  26FF01            inc word [es:bx+di]
00008049  26FE85204E        inc byte [es:di+0x4e20]
0000804E  8E06AAA8          mov es,word [0xa8aa]
00008052  8BDE              mov bx,si
00008054  268B30            mov si,[es:bx+si]
00008057  EBBB              jmp 0x8014
00008059  E9DDFE            jmp 0x7f39
0000805C  51                push cx
0000805D  8B3EEDF8          mov di,[0xf8ed]
00008061  8E06A4A8          mov es,word [0xa8a4]
00008065  2688853075        mov [es:di+0x7530],al
0000806A  8E06B0A8          mov es,word [0xa8b0]
0000806E  8BDF              mov bx,di
00008070  268B81204E        mov ax,[es:bx+di+0x4e20]
00008075  A3EDF8            mov [0xf8ed],ax
00008078  8B36EFF8          mov si,[0xf8ef]
0000807C  8BDE              mov bx,si
0000807E  2689B8204E        mov [es:bx+si+0x4e20],di
00008083  8BDF              mov bx,di
00008085  268931            mov [es:bx+di],si
00008088  893EEFF8          mov [0xf8ef],di
0000808C  8A85BEA8          mov al,[di-0x5742]
00008090  3CFF              cmp al,0xff
00008092  7503              jnz 0x8097
00008094  E9C300            jmp 0x815a
00008097  3C04              cmp al,0x4
00008099  750B              jnz 0x80a6
0000809B  FF0EF1F8          dec word [0xf8f1]
0000809F  7505              jnz 0x80a6
000080A1  C606F3F803        mov byte [0xf8f3],0x3
000080A6  33F6              xor si,si
000080A8  3C00              cmp al,0x0
000080AA  7451              jz 0x80fd
000080AC  1E                push ds
000080AD  8E06B2A8          mov es,word [0xa8b2]
000080B1  8E1EAEA8          mov ds,word [0xa8ae]
000080B5  8BDF              mov bx,di
000080B7  03DB              add bx,bx
000080B9  03DB              add bx,bx
000080BB  268A0F            mov cl,[es:bx]
000080BE  32ED              xor ch,ch
000080C0  8BF1              mov si,cx
000080C2  03F6              add si,si
000080C4  8B34              mov si,[si]
000080C6  3C01              cmp al,0x1
000080C8  7432              jz 0x80fc
000080CA  268A4F01          mov cl,[es:bx+0x1]
000080CE  03F1              add si,cx
000080D0  81E6FF3F          and si,0x3fff
000080D4  03F6              add si,si
000080D6  8B34              mov si,[si]
000080D8  3C02              cmp al,0x2
000080DA  7420              jz 0x80fc
000080DC  268A4F02          mov cl,[es:bx+0x2]
000080E0  03F1              add si,cx
000080E2  81E6FF3F          and si,0x3fff
000080E6  03F6              add si,si
000080E8  8B34              mov si,[si]
000080EA  3C03              cmp al,0x3
000080EC  740E              jz 0x80fc
000080EE  268A4F03          mov cl,[es:bx+0x3]
000080F2  03F1              add si,cx
000080F4  81E6FF3F          and si,0x3fff
000080F8  03F6              add si,si
000080FA  8B34              mov si,[si]
000080FC  1F                pop ds
000080FD  03F6              add si,si
000080FF  8E06ACA8          mov es,word [0xa8ac]
00008103  26393C            cmp [es:si],di
00008106  750C              jnz 0x8114
00008108  8BDF              mov bx,di
0000810A  268B810080        mov ax,[es:bx+di-0x8000]
0000810F  268904            mov [es:si],ax
00008112  EB1F              jmp 0x8133
00008114  268B34            mov si,[es:si]
00008117  03F6              add si,si
00008119  2639BC0080        cmp [es:si-0x8000],di
0000811E  7407              jz 0x8127
00008120  268BB40080        mov si,[es:si-0x8000]
00008125  EBF0              jmp 0x8117
00008127  8BDF              mov bx,di
00008129  268B810080        mov ax,[es:bx+di-0x8000]
0000812E  2689840080        mov [es:si-0x8000],ax
00008133  8E06AAA8          mov es,word [0xa8aa]
00008137  8BF7              mov si,di
00008139  03F6              add si,si
0000813B  26833CFF          cmp word [es:si],0xffffffffffffffff
0000813F  7419              jz 0x815a
00008141  268B34            mov si,[es:si]
00008144  03F6              add si,si
00008146  26833CFF          cmp word [es:si],0xffffffffffffffff
0000814A  75F5              jnz 0x8141
0000814C  A1F7F8            mov ax,[0xf8f7]
0000814F  268904            mov [es:si],ax
00008152  8BDF              mov bx,di
00008154  268B01            mov ax,[es:bx+di]
00008157  A3F7F8            mov [0xf8f7],ax
0000815A  8BF7              mov si,di
0000815C  03F6              add si,si
0000815E  8E06AAA8          mov es,word [0xa8aa]
00008162  26C704FFFF        mov word [es:si],0xffff
00008167  B80100            mov ax,0x1
0000816A  8E06A4A8          mov es,word [0xa8a4]
0000816E  268904            mov [es:si],ax
00008171  268885204E        mov [es:di+0x4e20],al
00008176  8E06A6A8          mov es,word [0xa8a6]
0000817A  268904            mov [es:si],ax
0000817D  8E06A8A8          mov es,word [0xa8a8]
00008181  26C60504          mov byte [es:di],0x4
00008185  C685CECF00        mov byte [di-0x3032],0x0
0000818A  58                pop ax
0000818B  8885BEA8          mov [di-0x5742],al
0000818F  03F6              add si,si
00008191  8BD7              mov dx,di
00008193  8E06B2A8          mov es,word [0xa8b2]
00008197  8BFE              mov di,si
00008199  BEE3F6            mov si,0xf6e3
0000819C  8BDE              mov bx,si
0000819E  A5                movsw
0000819F  A5                movsw
000081A0  8BFA              mov di,dx
000081A2  33F6              xor si,si
000081A4  3C00              cmp al,0x0
000081A6  7445              jz 0x81ed
000081A8  8E06AEA8          mov es,word [0xa8ae]
000081AC  8A0F              mov cl,[bx]
000081AE  32ED              xor ch,ch
000081B0  8BF1              mov si,cx
000081B2  03F6              add si,si
000081B4  268B34            mov si,[es:si]
000081B7  3C01              cmp al,0x1
000081B9  7432              jz 0x81ed
000081BB  8A4F01            mov cl,[bx+0x1]
000081BE  03F1              add si,cx
000081C0  81E6FF3F          and si,0x3fff
000081C4  03F6              add si,si
000081C6  268B34            mov si,[es:si]
000081C9  3C02              cmp al,0x2
000081CB  7420              jz 0x81ed
000081CD  8A4F02            mov cl,[bx+0x2]
000081D0  03F1              add si,cx
000081D2  81E6FF3F          and si,0x3fff
000081D6  03F6              add si,si
000081D8  268B34            mov si,[es:si]
000081DB  3C03              cmp al,0x3
000081DD  740E              jz 0x81ed
000081DF  8A4F03            mov cl,[bx+0x3]
000081E2  03F1              add si,cx
000081E4  81E6FF3F          and si,0x3fff
000081E8  03F6              add si,si
000081EA  268B34            mov si,[es:si]
000081ED  03F6              add si,si
000081EF  8E06ACA8          mov es,word [0xa8ac]
000081F3  268B04            mov ax,[es:si]
000081F6  26893C            mov [es:si],di
000081F9  8BDF              mov bx,di
000081FB  2689810080        mov [es:bx+di-0x8000],ax
00008200  8BC7              mov ax,di
00008202  C3                ret
00008203  55                push bp
00008204  8BEC              mov bp,sp
00008206  50                push ax
00008207  50                push ax
00008208  E87CF6            call 0x7887
0000820B  E8CBF7            call 0x79d9
0000820E  97                xchg ax,di
0000820F  83FFFF            cmp di,0xffffffffffffffff
00008212  7408              jz 0x821c
00008214  8AA5BEA8          mov ah,[di-0x5742]
00008218  FEC4              inc ah
0000821A  EB02              jmp 0x821e
0000821C  B400              mov ah,0x0
0000821E  A0F3F8            mov al,[0xf8f3]
00008221  40                inc ax
00008222  8946FE            mov [bp-0x2],ax
00008225  83FFFF            cmp di,0xffffffffffffffff
00008228  7508              jnz 0x8232
0000822A  E86BFB            call 0x7d98
0000822D  8946FC            mov [bp-0x4],ax
00008230  EB24              jmp 0x8256
00008232  57                push di
00008233  833EEBF600        cmp word [0xf6eb],0x0
00008238  7405              jz 0x823f
0000823A  E809FA            call 0x7c46
0000823D  EB03              jmp 0x8242
0000823F  E89AF8            call 0x7adc
00008242  5F                pop di
00008243  8946FC            mov [bp-0x4],ax
00008246  3D0001            cmp ax,0x100
00008249  7405              jz 0x8250
0000824B  E846F8            call 0x7a94
0000824E  EB06              jmp 0x8256
00008250  E8F5F7            call 0x7a48
00008253  97                xchg ax,di
00008254  EBCF              jmp 0x8225
00008256  8B4EFC            mov cx,[bp-0x4]
00008259  81F90001          cmp cx,0x100
0000825D  7442              jz 0x82a1
0000825F  E8D7FC            call 0x7f39
00008262  8B4EFE            mov cx,[bp-0x2]
00008265  3ACD              cmp cl,ch
00008267  760C              jna 0x8275
00008269  FE4EFE            dec byte [bp-0x2]
0000826C  49                dec cx
0000826D  8B46FC            mov ax,[bp-0x4]
00008270  E8E9FD            call 0x805c
00008273  EBED              jmp 0x8262
00008275  8A46FC            mov al,[bp-0x4]
00008278  FF06F899          inc word [0x99f8]
0000827C  8B3EF899          mov di,[0x99f8]
00008280  8845FF            mov [di-0x1],al
00008283  81FFE079          cmp di,0x79e0
00008287  7507              jnz 0x8290
00008289  50                push ax
0000828A  8BCF              mov cx,di
0000828C  E8EFF5            call 0x787e
0000828F  58                pop ax
00008290  BEE3F6            mov si,0xf6e3
00008293  8604              xchg al,[si]
00008295  864401            xchg al,[si+0x1]
00008298  864402            xchg al,[si+0x2]
0000829B  864403            xchg al,[si+0x3]
0000829E  E96AFF            jmp 0x820b
000082A1  8B0EF899          mov cx,[0x99f8]
000082A5  8BE5              mov sp,bp
000082A7  5D                pop bp
000082A8  E9D3F5            jmp 0x787e
000082AB  55                push bp
000082AC  33ED              xor bp,bp
000082AE  0BD2              or dx,dx
000082B0  7908              jns 0x82ba
000082B2  45                inc bp
000082B3  F7D8              neg ax
000082B5  83D200            adc dx,0x0
000082B8  F7DA              neg dx
000082BA  0BDB              or bx,bx
000082BC  7438              jz 0x82f6
000082BE  790C              jns 0x82cc
000082C0  83C502            add bp,0x2
000082C3  F7D9              neg cx
000082C5  83D300            adc bx,0x0
000082C8  F7DB              neg bx
000082CA  742A              jz 0x82f6
000082CC  55                push bp
000082CD  8BF1              mov si,cx
000082CF  8BFB              mov di,bx
000082D1  33DB              xor bx,bx
000082D3  8BCA              mov cx,dx
000082D5  8BD0              mov dx,ax
000082D7  33C0              xor ax,ax
000082D9  BD1000            mov bp,0x10
000082DC  D1E0              shl ax,0x0
000082DE  D1D2              rcl dx,0x0
000082E0  D1D1              rcl cx,0x0
000082E2  D1D3              rcl bx,0x0
000082E4  40                inc ax
000082E5  2BCE              sub cx,si
000082E7  1BDF              sbb bx,di
000082E9  7305              jnc 0x82f0
000082EB  48                dec ax
000082EC  03CE              add cx,si
000082EE  13DF              adc bx,di
000082F0  4D                dec bp
000082F1  75E9              jnz 0x82dc
000082F3  5D                pop bp
000082F4  EB0D              jmp 0x8303
000082F6  93                xchg ax,bx
000082F7  92                xchg ax,dx
000082F8  F7F1              div cx
000082FA  93                xchg ax,bx
000082FB  F7F1              div cx
000082FD  8BCA              mov cx,dx
000082FF  8BD3              mov dx,bx
00008301  33DB              xor bx,bx
00008303  D1ED              shr bp,0x0
00008305  7308              jnc 0x830f
00008307  F7D9              neg cx
00008309  83D300            adc bx,0x0
0000830C  F7DB              neg bx
0000830E  45                inc bp
0000830F  4D                dec bp
00008310  7507              jnz 0x8319
00008312  F7D8              neg ax
00008314  83D200            adc dx,0x0
00008317  F7DA              neg dx
00008319  5D                pop bp
0000831A  C3                ret
0000831B  8BF0              mov si,ax
0000831D  8BFA              mov di,dx
0000831F  F7E1              mul cx
00008321  50                push ax
00008322  52                push dx
00008323  96                xchg ax,si
00008324  F7E3              mul bx
00008326  97                xchg ax,di
00008327  F7E1              mul cx
00008329  5A                pop dx
0000832A  03D0              add dx,ax
0000832C  03D7              add dx,di
0000832E  58                pop ax
0000832F  C3                ret
00008330  57                push di
00008331  52                push dx
00008332  B006              mov al,0x6
00008334  E89500            call 0x83cc
00008337  95                xchg ax,bp
00008338  8AAE00B8          mov ch,[bp-0x4800]
0000833C  E8C100            call 0x8400
0000833F  D1E5              shl bp,0x0
00008341  8A9E00B0          mov bl,[bp-0x5000]
00008345  32FF              xor bh,bh
00008347  8BCB              mov cx,bx
00008349  8BF3              mov si,bx
0000834B  D1E6              shl si,0x0
0000834D  8B841E2A          mov ax,[si+0x2a1e]
00008351  88260420          mov [0x2004],ah
00008355  8BF8              mov di,ax
00008357  FEC0              inc al
00008359  750D              jnz 0x8368
0000835B  B81E2A            mov ax,0x2a1e
0000835E  50                push ax
0000835F  B81E2D            mov ax,0x2d1e
00008362  50                push ax
00008363  E83400            call 0x839a
00008366  EBDF              jmp 0x8347
00008368  81E7FF00          and di,0xff
0000836C  8A951E2D          mov dl,[di+0x2d1e]
00008370  88971E2C          mov [bx+0x2c1e],dl
00008374  32F6              xor dh,dh
00008376  FE851E2D          inc byte [di+0x2d1e]
0000837A  8BFA              mov di,dx
0000837C  D1E7              shl di,0x0
0000837E  8AFE              mov bh,dh
00008380  8B951E2A          mov dx,[di+0x2a1e]
00008384  8ADE              mov bl,dh
00008386  888F1E2C          mov [bx+0x2c1e],cl
0000838A  89941E2A          mov [si+0x2a1e],dx
0000838E  89851E2A          mov [di+0x2a1e],ax
00008392  5A                pop dx
00008393  5F                pop di
00008394  C606052007        mov byte [0x2005],0x7
00008399  C3                ret
0000839A  8BEC              mov bp,sp
0000839C  57                push di
0000839D  06                push es
0000839E  1E                push ds
0000839F  07                pop es
000083A0  B80800            mov ax,0x8
000083A3  8B7E04            mov di,[bp+0x4]
000083A6  2B06D23F          sub ax,[0x3fd2]
000083AA  B92000            mov cx,0x20
000083AD  AA                stosb
000083AE  47                inc di
000083AF  E2FC              loop 0x83ad
000083B1  0306D23F          add ax,[0x3fd2]
000083B5  48                dec ax
000083B6  75EE              jnz 0x83a6
000083B8  8B7E02            mov di,[bp+0x2]
000083BB  B90800            mov cx,0x8
000083BE  2C20              sub al,0x20
000083C0  AA                stosb
000083C1  E2FB              loop 0x83be
000083C3  B17C              mov cl,0x7c
000083C5  F3AB              rep stosw
000083C7  07                pop es
000083C8  5F                pop di
000083C9  C20400            ret word 0x4
000083CC  50                push ax
000083CD  8B1EF543          mov bx,[0x43f5]
000083D1  8A0EF843          mov cl,[0x43f8]
000083D5  8B07              mov ax,[bx]
000083D7  86E0              xchg ah,al
000083D9  8A5F02            mov bl,[bx+0x2]
000083DC  8AF8              mov bh,al
000083DE  D3E3              shl bx,cl
000083E0  D3E0              shl ax,cl
000083E2  8AC7              mov al,bh
000083E4  59                pop cx
000083E5  D3E8              shr ax,cl
000083E7  C3                ret
000083E8  8B1EF543          mov bx,[0x43f5]
000083EC  8A0EF843          mov cl,[0x43f8]
000083F0  8B07              mov ax,[bx]
000083F2  86E0              xchg ah,al
000083F4  8A5F02            mov bl,[bx+0x2]
000083F7  8AF8              mov bh,al
000083F9  D3E3              shl bx,cl
000083FB  D3E0              shl ax,cl
000083FD  8AC7              mov al,bh
000083FF  C3                ret
00008400  8B36F543          mov si,[0x43f5]
00008404  022EF843          add ch,[0x43f8]
00008408  8ACD              mov cl,ch
0000840A  80E507            and ch,0x7
0000840D  882EF843          mov [0x43f8],ch
00008411  D0E9              shr cl,0x0
00008413  D0E9              shr cl,0x0
00008415  D0E9              shr cl,0x0
00008417  32ED              xor ch,ch
00008419  03F1              add si,cx
0000841B  8936F543          mov [0x43f5],si
0000841F  81FEF41F          cmp si,0x1ff4
00008423  7208              jc 0x842d
00008425  C606CC3F00        mov byte [0x3fcc],0x0
0000842A  E80901            call 0x8536
0000842D  C3                ret
0000842E  56                push si
0000842F  57                push di
00008430  803EF94300        cmp byte [0x43f9],0x0
00008435  7535              jnz 0x846c
00008437  8BCF              mov cx,di
00008439  8B16D43F          mov dx,[0x3fd4]
0000843D  8B1EEF43          mov bx,[0x43ef]
00008441  1E                push ds
00008442  B440              mov ah,0x40
00008444  8E1EDB3F          mov ds,word [0x3fdb]
00008448  3BCA              cmp cx,dx
0000844A  771B              ja 0x8467
0000844C  51                push cx
0000844D  8BCA              mov cx,dx
0000844F  F7D9              neg cx
00008451  CD21              int byte 0x21
00008453  B440              mov ah,0x40
00008455  59                pop cx
00008456  33D2              xor dx,dx
00008458  EB0F              jmp 0x8469
0000845A  260204            add al,[es:si]
0000845D  80D400            adc ah,0x0
00008460  D1C0              rol ax,0x0
00008462  46                inc si
00008463  E2F5              loop 0x845a
00008465  EB3F              jmp 0x84a6
00008467  2BCA              sub cx,dx
00008469  CD21              int byte 0x21
0000846B  1F                pop ds
0000846C  8BCF              mov cx,di
0000846E  2B0ED43F          sub cx,[0x3fd4]
00008472  8B36D43F          mov si,[0x3fd4]
00008476  A1F143            mov ax,[0x43f1]
00008479  803EFA4302        cmp byte [0x43fa],0x2
0000847E  76DA              jna 0x845a
00008480  8B16F343          mov dx,[0x43f3]
00008484  BFDD3F            mov di,0x3fdd
00008487  268A1C            mov bl,[es:si]
0000848A  32FF              xor bh,bh
0000848C  32D8              xor bl,al
0000848E  8AC4              mov al,ah
00008490  8AE2              mov ah,dl
00008492  8AD6              mov dl,dh
00008494  8AF7              mov dh,bh
00008496  D1E3              shl bx,0x0
00008498  D1E3              shl bx,0x0
0000849A  3301              xor ax,[bx+di]
0000849C  335102            xor dx,[bx+di+0x2]
0000849F  46                inc si
000084A0  E2E5              loop 0x8487
000084A2  8916F343          mov [0x43f3],dx
000084A6  A3F143            mov [0x43f1],ax
000084A9  8936D43F          mov [0x3fd4],si
000084AD  1E                push ds
000084AE  8E1EE543          mov ds,word [0x43e5]
000084B2  803EE5A300        cmp byte [0xa3e5],0x0
000084B7  7411              jz 0x84ca
000084B9  FE0EE1A3          dec byte [0xa3e1]
000084BD  790B              jns 0x84ca
000084BF  A0BABC            mov al,[0xbcba]
000084C2  A2E1A3            mov [0xa3e1],al
000084C5  B0DB              mov al,0xdb
000084C7  E84D83            call 0x817
000084CA  1F                pop ds
000084CB  5F                pop di
000084CC  5E                pop si
000084CD  C3                ret
000084CE  F606FF4301        test byte [0x43ff],0x1
000084D3  7449              jz 0x851e
000084D5  803EFA4302        cmp byte [0x43fa],0x2
000084DA  7643              jna 0x851f
000084DC  55                push bp
000084DD  A10144            mov ax,[0x4401]
000084E0  8B1E0344          mov bx,[0x4403]
000084E4  8B160544          mov dx,[0x4405]
000084E8  8B2E0744          mov bp,[0x4407]
000084EC  053412            add ax,0x1234
000084EF  8BF8              mov di,ax
000084F1  81E7FE01          and di,0x1fe
000084F5  D1E7              shl di,0x0
000084F7  339DDD3F          xor bx,[di+0x3fdd]
000084FB  2B95DF3F          sub dx,[di+0x3fdf]
000084FF  33C2              xor ax,dx
00008501  D1CD              ror bp,0x0
00008503  33EB              xor bp,bx
00008505  D1CD              ror bp,0x0
00008507  33C5              xor ax,bp
00008509  3024              xor [si],ah
0000850B  46                inc si
0000850C  E2DE              loop 0x84ec
0000850E  A30144            mov [0x4401],ax
00008511  891E0344          mov [0x4403],bx
00008515  89160544          mov [0x4405],dx
00008519  892E0744          mov [0x4407],bp
0000851D  5D                pop bp
0000851E  C3                ret
0000851F  A10144            mov ax,[0x4401]
00008522  8A1E0344          mov bl,[0x4403]
00008526  02E3              add ah,bl
00008528  02C4              add al,ah
0000852A  2804              sub [si],al
0000852C  E2F8              loop 0x8526
0000852E  A30144            mov [0x4401],ax
00008531  881E0344          mov [0x4403],bl
00008535  C3                ret
00008536  53                push bx
00008537  52                push dx
00008538  56                push si
00008539  57                push di
0000853A  06                push es
0000853B  BF0000            mov di,0x0
0000853E  B80020            mov ax,0x2000
00008541  803ECC3F00        cmp byte [0x3fcc],0x0
00008546  7409              jz 0x8551
00008548  33D2              xor dx,dx
0000854A  8BCA              mov cx,dx
0000854C  A3F543            mov [0x43f5],ax
0000854F  EB11              jmp 0x8562
00008551  2B06F543          sub ax,[0x43f5]
00008555  8BC8              mov cx,ax
00008557  1E                push ds
00008558  07                pop es
00008559  8BF7              mov si,di
0000855B  0336F543          add si,[0x43f5]
0000855F  F3A4              rep movsb
00008561  92                xchg ax,dx
00008562  870EF543          xchg cx,[0x43f5]
00008566  8B1EED43          mov bx,[0x43ed]
0000856A  83C200            add dx,0x0
0000856D  B43F              mov ah,0x3f
0000856F  CD21              int byte 0x21
00008571  8BF2              mov si,dx
00008573  E858FF            call 0x84ce
00008576  07                pop es
00008577  5F                pop di
00008578  5E                pop si
00008579  5A                pop dx
0000857A  5B                pop bx
0000857B  C3                ret
0000857C  33DB              xor bx,bx
0000857E  889F1E22          mov [bx+0x221e],bl
00008582  889F1E28          mov [bx+0x281e],bl
00008586  889F1E24          mov [bx+0x241e],bl
0000858A  889F1E25          mov [bx+0x251e],bl
0000858E  8ACB              mov cl,bl
00008590  F6D9              neg cl
00008592  888F1E2C          mov [bx+0x2c1e],cl
00008596  8BF3              mov si,bx
00008598  8BC3              mov ax,bx
0000859A  86E0              xchg ah,al
0000859C  89801E20          mov [bx+si+0x201e],ax
000085A0  89801E26          mov [bx+si+0x261e],ax
000085A4  8AE1              mov ah,cl
000085A6  89801E2A          mov [bx+si+0x2a1e],ax
000085AA  FEC3              inc bl
000085AC  75D0              jnz 0x857e
000085AE  B81E26            mov ax,0x261e
000085B1  50                push ax
000085B2  B81E29            mov ax,0x291e
000085B5  50                push ax
000085B6  E8E1FD            call 0x839a
000085B9  C3                ret
000085BA  803EFA4301        cmp byte [0x43fa],0x1
000085BF  750E              jnz 0x85cf
000085C1  C706D03F2800      mov word [0x3fd0],0x28
000085C7  C706D23F0000      mov word [0x3fd2],0x0
000085CD  EB0C              jmp 0x85db
000085CF  C706D03F4000      mov word [0x3fd0],0x40
000085D5  C706D23F0100      mov word [0x3fd2],0x1
000085DB  1E                push ds
000085DC  07                pop es
000085DD  BFCC3C            mov di,0x3ccc
000085E0  B91B00            mov cx,0x1b
000085E3  B80400            mov ax,0x4
000085E6  F3AB              rep stosw
000085E8  B006              mov al,0x6
000085EA  B165              mov cl,0x65
000085EC  F3AB              rep stosw
000085EE  B107              mov cl,0x7
000085F0  B80050            mov ax,0x5000
000085F3  F3AB              rep stosw
000085F5  B480              mov ah,0x80
000085F7  B114              mov cl,0x14
000085F9  F3AB              rep stosw
000085FB  B4B0              mov ah,0xb0
000085FD  B114              mov cl,0x14
000085FF  F3AB              rep stosw
00008601  B4C0              mov ah,0xc0
00008603  B10C              mov cl,0xc
00008605  F3AB              rep stosw
00008607  B4D0              mov ah,0xd0
00008609  B145              mov cl,0x45
0000860B  F3AB              rep stosw
0000860D  B107              mov cl,0x7
0000860F  B470              mov ah,0x70
00008611  F3AB              rep stosw
00008613  B4A0              mov ah,0xa0
00008615  B114              mov cl,0x14
00008617  F3AB              rep stosw
00008619  B4B8              mov ah,0xb8
0000861B  B114              mov cl,0x14
0000861D  F3AB              rep stosw
0000861F  B4C8              mov ah,0xc8
00008621  B10C              mov cl,0xc
00008623  F3AB              rep stosw
00008625  B4D8              mov ah,0xd8
00008627  B145              mov cl,0x45
00008629  F3AB              rep stosw
0000862B  C3                ret
0000862C  803EEB4300        cmp byte [0x43eb],0x0
00008631  7517              jnz 0x864a
00008633  1E                push ds
00008634  07                pop es
00008635  BF0020            mov di,0x2000
00008638  B90F07            mov cx,0x70f
0000863B  33C0              xor ax,ax
0000863D  F3AB              rep stosw
0000863F  B435              mov ah,0x35
00008641  A30620            mov [0x2006],ax
00008644  B88080            mov ax,0x8080
00008647  A30220            mov [0x2002],ax
0000864A  33C0              xor ax,ax
0000864C  803EFA4302        cmp byte [0x43fa],0x2
00008651  7604              jna 0x8657
00008653  48                dec ax
00008654  A3F343            mov [0x43f3],ax
00008657  A3F143            mov [0x43f1],ax
0000865A  33C0              xor ax,ax
0000865C  A20120            mov [0x2001],al
0000865F  A2F743            mov [0x43f7],al
00008662  A2F843            mov [0x43f8],al
00008665  A3F543            mov [0x43f5],ax
00008668  C3                ret
00008669  55                push bp
0000866A  E84DFF            call 0x85ba
0000866D  E8BCFF            call 0x862c
00008670  8E06DB3F          mov es,word [0x3fdb]
00008674  803EEB4300        cmp byte [0x43eb],0x0
00008679  7516              jnz 0x8691
0000867B  E8FEFE            call 0x857c
0000867E  C706CA3C0120      mov word [0x3cca],0x2001
00008684  A3D43F            mov [0x3fd4],ax
00008687  A2D63F            mov [0x3fd6],al
0000868A  8BF8              mov di,ax
0000868C  B90080            mov cx,0x8000
0000868F  F3AB              rep stosw
00008691  832EE14301        sub word [0x43e1],0x1
00008696  831EE34300        sbb word [0x43e3],0x0
0000869B  832EDD4301        sub word [0x43dd],0x1
000086A0  831EDF4300        sbb word [0x43df],0x0
000086A5  7902              jns 0x86a9
000086A7  5D                pop bp
000086A8  C3                ret
000086A9  A3F543            mov [0x43f5],ax
000086AC  40                inc ax
000086AD  A2CC3F            mov [0x3fcc],al
000086B0  E883FE            call 0x8536
000086B3  33C0              xor ax,ax
000086B5  803EEB4300        cmp byte [0x43eb],0x0
000086BA  7403              jz 0x86bf
000086BC  A1D43F            mov ax,[0x3fd4]
000086BF  8BF8              mov di,ax
000086C1  E86CFC            call 0x8330
000086C4  FE060520          inc byte [0x2005]
000086C8  8B160220          mov dx,[0x2002]
000086CC  803E012001        cmp byte [0x2001],0x1
000086D1  7413              jz 0x86e6
000086D3  FE0E0520          dec byte [0x2005]
000086D7  7903              jns 0x86dc
000086D9  E854FC            call 0x8330
000086DC  D0260420          shl byte [0x2004],0x0
000086E0  7307              jnc 0x86e9
000086E2  3AF2              cmp dh,dl
000086E4  7716              ja 0x86fc
000086E6  E9B003            jmp 0x8a99
000086E9  FE0E0520          dec byte [0x2005]
000086ED  7903              jns 0x86f2
000086EF  E83EFC            call 0x8330
000086F2  D0260420          shl byte [0x2004],0x0
000086F6  7307              jnc 0x86ff
000086F8  3AF2              cmp dh,dl
000086FA  77EA              ja 0x86e6
000086FC  E99201            jmp 0x8891
000086FF  E9B300            jmp 0x87b5
00008702  C606F74300        mov byte [0x43f7],0x0
00008707  E8C2FC            call 0x83cc
0000870A  8BD8              mov bx,ax
0000870C  368AAF0040        mov ch,[ss:bx+0x4000]
00008711  E8ECFC            call 0x8400
00008714  368A8F0030        mov cl,[ss:bx+0x3000]
00008719  32ED              xor ch,ch
0000871B  83C105            add cx,0x5
0000871E  51                push cx
0000871F  E8C6FC            call 0x83e8
00008722  B50F              mov ch,0xf
00008724  93                xchg ax,bx
00008725  E8D8FC            call 0x8400
00008728  F9                stc
00008729  D1DB              rcr bx,0x0
0000872B  8BF3              mov si,bx
0000872D  59                pop cx
0000872E  E92B03            jmp 0x8a5c
00008731  2C0A              sub al,0xa
00008733  3C04              cmp al,0x4
00008735  74CB              jz 0x8702
00008737  8AD8              mov bl,al
00008739  32FF              xor bh,bh
0000873B  883EF743          mov [0x43f7],bh
0000873F  53                push bx
00008740  8B2E1C20          mov bp,[0x201c]
00008744  2BEB              sub bp,bx
00008746  4D                dec bp
00008747  83E503            and bp,0x3
0000874A  D1E5              shl bp,0x0
0000874C  3E8BAE1420        mov bp,[ds:bp+0x2014]
00008751  B004              mov al,0x4
00008753  E876FC            call 0x83cc
00008756  8BD8              mov bx,ax
00008758  368AAF0020        mov ch,[ss:bx+0x2000]
0000875D  E8A0FC            call 0x8400
00008760  368A8F0010        mov cl,[ss:bx+0x1000]
00008765  32ED              xor ch,ch
00008767  5B                pop bx
00008768  80F9FF            cmp cl,0xff
0000876B  750C              jnz 0x8779
0000876D  85DB              test bx,bx
0000876F  7508              jnz 0x8779
00008771  8036D63F01        xor byte [0x3fd6],0x1
00008776  E94FFF            jmp 0x86c8
00008779  80C102            add cl,0x2
0000877C  8BF5              mov si,bp
0000877E  81FE0101          cmp si,0x101
00008782  F5                cmc
00008783  83D100            adc cx,0x0
00008786  3B36CA3C          cmp si,[0x3cca]
0000878A  F5                cmc
0000878B  83D100            adc cx,0x0
0000878E  E9B702            jmp 0x8a48
00008791  8A0EF843          mov cl,[0x43f8]
00008795  41                inc cx
00008796  80F908            cmp cl,0x8
00008799  F5                cmc
0000879A  8316F54300        adc word [0x43f5],0x0
0000879F  80E107            and cl,0x7
000087A2  880EF843          mov [0x43f8],cl
000087A6  D1E0              shl ax,0x0
000087A8  7315              jnc 0x87bf
000087AA  8B361020          mov si,[0x2010]
000087AE  8B0E1220          mov cx,[0x2012]
000087B2  E9A702            jmp 0x8a5c
000087B5  E830FC            call 0x83e8
000087B8  803EF74302        cmp byte [0x43f7],0x2
000087BD  74D2              jz 0x8791
000087BF  8ADC              mov bl,ah
000087C1  32FF              xor bh,bh
000087C3  803ED63F01        cmp byte [0x3fd6],0x1
000087C8  7517              jnz 0x87e1
000087CA  833E0A2025        cmp word [0x200a],0x25
000087CF  7308              jnc 0x87d9
000087D1  BE0008            mov si,0x800
000087D4  B80006            mov ax,0x600
000087D7  EB1C              jmp 0x87f5
000087D9  BE000B            mov si,0xb00
000087DC  B80009            mov ax,0x900
000087DF  EB14              jmp 0x87f5
000087E1  833E0A2025        cmp word [0x200a],0x25
000087E6  7208              jc 0x87f0
000087E8  BE0005            mov si,0x500
000087EB  B80003            mov ax,0x300
000087EE  EB05              jmp 0x87f5
000087F0  BE0002            mov si,0x200
000087F3  33C0              xor ax,ax
000087F5  368A08            mov cl,[ss:bx+si]
000087F8  020EF843          add cl,[0x43f8]
000087FC  8B36F543          mov si,[0x43f5]
00008800  80F908            cmp cl,0x8
00008803  F5                cmc
00008804  83D600            adc si,0x0
00008807  80E107            and cl,0x7
0000880A  880EF843          mov [0x43f8],cl
0000880E  8936F543          mov [0x43f5],si
00008812  03D8              add bx,ax
00008814  368A07            mov al,[ss:bx]
00008817  3C09              cmp al,0x9
00008819  721E              jc 0x8839
0000881B  81FEF41F          cmp si,0x1ff4
0000881F  720A              jc 0x882b
00008821  C606CC3F00        mov byte [0x3fcc],0x0
00008826  50                push ax
00008827  E80CFD            call 0x8536
0000882A  58                pop ax
0000882B  3C09              cmp al,0x9
0000882D  7603              jna 0x8832
0000882F  E9FFFE            jmp 0x8731
00008832  FE06F743          inc byte [0x43f7]
00008836  E971FF            jmp 0x87aa
00008839  32E4              xor ah,ah
0000883B  8826F743          mov [0x43f7],ah
0000883F  8BD8              mov bx,ax
00008841  031E0A20          add bx,[0x200a]
00008845  B104              mov cl,0x4
00008847  8BF3              mov si,bx
00008849  D3EE              shr si,cl
0000884B  2BDE              sub bx,si
0000884D  891E0A20          mov [0x200a],bx
00008851  50                push ax
00008852  B006              mov al,0x6
00008854  E875FB            call 0x83cc
00008857  93                xchg ax,bx
00008858  368AAF00B8        mov ch,[ss:bx-0x4800]
0000885D  E8A0FB            call 0x8400
00008860  D1E3              shl bx,0x0
00008862  368BB700B0        mov si,[ss:bx-0x5000]
00008867  8A9C1E24          mov bl,[si+0x241e]
0000886B  32FF              xor bh,bh
0000886D  8BC3              mov ax,bx
0000886F  4E                dec si
00008870  7810              js 0x8882
00008872  FE8F1E25          dec byte [bx+0x251e]
00008876  869C1E24          xchg bl,[si+0x241e]
0000887A  FE871E25          inc byte [bx+0x251e]
0000887E  889C1F24          mov [si+0x241f],bl
00008882  59                pop cx
00008883  41                inc cx
00008884  41                inc cx
00008885  40                inc ax
00008886  8BF0              mov si,ax
00008888  E9BD01            jmp 0x8a48
0000888B  B690              mov dh,0x90
0000888D  D0EA              shr dl,0x0
0000888F  EB0C              jmp 0x889d
00008891  B004              mov al,0x4
00008893  E836FB            call 0x83cc
00008896  8BD8              mov bx,ax
00008898  80C610            add dh,0x10
0000889B  72EE              jc 0x888b
0000889D  89160220          mov [0x2002],dx
000088A1  A10C20            mov ax,[0x200c]
000088A4  A3D73F            mov [0x3fd7],ax
000088A7  3D7A00            cmp ax,0x7a
000088AA  7344              jnc 0x88f0
000088AC  3B06D03F          cmp ax,[0x3fd0]
000088B0  7208              jc 0x88ba
000088B2  BE0020            mov si,0x2000
000088B5  BD0010            mov bp,0x1000
000088B8  EB3C              jmp 0x88f6
000088BA  BE000E            mov si,0xe00
000088BD  BD000C            mov bp,0xc00
000088C0  D3EB              shr bx,cl
000088C2  85DB              test bx,bx
000088C4  7530              jnz 0x88f6
000088C6  8B1EF543          mov bx,[0x43f5]
000088CA  8A0EF843          mov cl,[0x43f8]
000088CE  8B4701            mov ax,[bx+0x1]
000088D1  86E0              xchg ah,al
000088D3  8AE9              mov ch,cl
000088D5  D3E0              shl ax,cl
000088D7  8ACC              mov cl,ah
000088D9  8306F54302        add word [0x43f5],0x2
000088DE  813EF543F41F      cmp word [0x43f5],0x1ff4
000088E4  721B              jc 0x8901
000088E6  C606CC3F00        mov byte [0x3fcc],0x0
000088EB  E848FC            call 0x8536
000088EE  EB11              jmp 0x8901
000088F0  BE0040            mov si,0x4000
000088F3  BD0030            mov bp,0x3000
000088F6  368A28            mov ch,[ss:bx+si]
000088F9  E804FB            call 0x8400
000088FC  03EB              add bp,bx
000088FE  8A4E00            mov cl,[bp+0x0]
00008901  32ED              xor ch,ch
00008903  A10C20            mov ax,[0x200c]
00008906  03C1              add ax,cx
00008908  8BF0              mov si,ax
0000890A  51                push cx
0000890B  B105              mov cl,0x5
0000890D  D3EE              shr si,cl
0000890F  2BC6              sub ax,si
00008911  A30C20            mov [0x200c],ax
00008914  E8D1FA            call 0x83e8
00008917  8BD8              mov bx,ax
00008919  A10820            mov ax,[0x2008]
0000891C  B103              mov cl,0x3
0000891E  80FC28            cmp ah,0x28
00008921  7608              jna 0x892b
00008923  BD00B8            mov bp,0xb800
00008926  BA00B0            mov dx,0xb000
00008929  EB15              jmp 0x8940
0000892B  80FC06            cmp ah,0x6
0000892E  7608              jna 0x8938
00008930  BD00A0            mov bp,0xa000
00008933  BA0080            mov dx,0x8000
00008936  EB06              jmp 0x893e
00008938  BD0070            mov bp,0x7000
0000893B  BA0050            mov dx,0x5000
0000893E  FEC9              dec cl
00008940  D0E1              shl cl,0x0
00008942  D3EB              shr bx,cl
00008944  03EB              add bp,bx
00008946  D1E3              shl bx,0x0
00008948  03DA              add bx,dx
0000894A  368B1F            mov bx,[ss:bx]
0000894D  03C3              add ax,bx
0000894F  2AC4              sub al,ah
00008951  80DC00            sbb ah,0x0
00008954  A30820            mov [0x2008],ax
00008957  8A6E00            mov ch,[bp+0x0]
0000895A  022EF843          add ch,[0x43f8]
0000895E  8B36F543          mov si,[0x43f5]
00008962  8ACD              mov cl,ch
00008964  80E507            and ch,0x7
00008967  882EF843          mov [0x43f8],ch
0000896B  D0E9              shr cl,0x0
0000896D  D0E9              shr cl,0x0
0000896F  D0E9              shr cl,0x0
00008971  32ED              xor ch,ch
00008973  03F1              add si,cx
00008975  8936F543          mov [0x43f5],si
00008979  D1E3              shl bx,0x0
0000897B  8BF3              mov si,bx
0000897D  8B971E26          mov dx,[bx+0x261e]
00008981  8BEA              mov bp,dx
00008983  FEC2              inc dl
00008985  750F              jnz 0x8996
00008987  55                push bp
00008988  BA1E26            mov dx,0x261e
0000898B  52                push dx
0000898C  BA1E29            mov dx,0x291e
0000898F  52                push dx
00008990  E807FA            call 0x839a
00008993  5D                pop bp
00008994  EBE7              jmp 0x897d
00008996  81E5FF00          and bp,0xff
0000899A  8BDD              mov bx,bp
0000899C  3E8A9E1E29        mov bl,[ds:bp+0x291e]
000089A1  3EFE861E29        inc byte [ds:bp+0x291e]
000089A6  D1E3              shl bx,0x0
000089A8  8BAF1E26          mov bp,[bx+0x261e]
000089AC  89AC1E26          mov [si+0x261e],bp
000089B0  89971E26          mov [bx+0x261e],dx
000089B4  8B36F543          mov si,[0x43f5]
000089B8  8A0EF843          mov cl,[0x43f8]
000089BC  8B04              mov ax,[si]
000089BE  86C4              xchg al,ah
000089C0  D3E0              shl ax,cl
000089C2  8AD4              mov dl,ah
000089C4  80C107            add cl,0x7
000089C7  8AE9              mov ch,cl
000089C9  80E507            and ch,0x7
000089CC  882EF843          mov [0x43f8],ch
000089D0  D0E9              shr cl,0x0
000089D2  D0E9              shr cl,0x0
000089D4  D0E9              shr cl,0x0
000089D6  32ED              xor ch,ch
000089D8  03F1              add si,cx
000089DA  8936F543          mov [0x43f5],si
000089DE  D1EA              shr dx,0x0
000089E0  59                pop cx
000089E1  8B1E0E20          mov bx,[0x200e]
000089E5  8BEB              mov bp,bx
000089E7  83F901            cmp cx,0x1
000089EA  7421              jz 0x8a0d
000089EC  83F904            cmp cx,0x4
000089EF  741C              jz 0x8a0d
000089F1  8BF1              mov si,cx
000089F3  3916CA3C          cmp [0x3cca],dx
000089F7  83D600            adc si,0x0
000089FA  7508              jnz 0x8a04
000089FC  43                inc bx
000089FD  2ADF              sub bl,bh
000089FF  80DF00            sbb bh,0x0
00008A02  EB05              jmp 0x8a09
00008A04  83EB01            sub bx,0x1
00008A07  7204              jc 0x8a0d
00008A09  891E0E20          mov [0x200e],bx
00008A0D  833ECE3F01        cmp word [0x3fce],0x1
00008A12  7417              jz 0x8a2b
00008A14  BB007F            mov bx,0x7f00
00008A17  81FDB000          cmp bp,0xb0
00008A1B  7711              ja 0x8a2e
00008A1D  803E07202A        cmp byte [0x2007],0x2a
00008A22  7207              jc 0x8a2b
00008A24  833ED73F40        cmp word [0x3fd7],0x40
00008A29  7203              jc 0x8a2e
00008A2B  BB0120            mov bx,0x2001
00008A2E  8BF2              mov si,dx
00008A30  83C103            add cx,0x3
00008A33  3B36CA3C          cmp si,[0x3cca]
00008A37  F5                cmc
00008A38  83D100            adc cx,0x0
00008A3B  891ECA3C          mov [0x3cca],bx
00008A3F  81FE0001          cmp si,0x100
00008A43  7703              ja 0x8a48
00008A45  83C108            add cx,0x8
00008A48  8B1E1C20          mov bx,[0x201c]
00008A4C  D1E3              shl bx,0x0
00008A4E  89B71420          mov [bx+0x2014],si
00008A52  D1EB              shr bx,0x0
00008A54  43                inc bx
00008A55  83E303            and bx,0x3
00008A58  891E1C20          mov [0x201c],bx
00008A5C  C70600200000      mov word [0x2000],0x0
00008A62  890E1220          mov [0x2012],cx
00008A66  89361020          mov [0x2010],si
00008A6A  8B1ED43F          mov bx,[0x3fd4]
00008A6E  2BDF              sub bx,di
00008A70  4B                dec bx
00008A71  81FB1001          cmp bx,0x110
00008A75  7705              ja 0x8a7c
00008A77  51                push cx
00008A78  E8B3F9            call 0x842e
00008A7B  59                pop cx
00008A7C  F7DE              neg si
00008A7E  03F7              add si,di
00008A80  1E                push ds
00008A81  06                push es
00008A82  1F                pop ds
00008A83  8BC1              mov ax,cx
00008A85  F3A4              rep movsb
00008A87  1F                pop ds
00008A88  2906E143          sub [0x43e1],ax
00008A8C  831EE34300        sbb word [0x43e3],0x0
00008A91  7903              jns 0x8a96
00008A93  E9B800            jmp 0x8b4e
00008A96  E92FFC            jmp 0x86c8
00008A99  A00720            mov al,[0x2007]
00008A9C  32E4              xor ah,ah
00008A9E  24FE              and al,0xfe
00008AA0  8BD8              mov bx,ax
00008AA2  8BB7CC3E          mov si,[bx+0x3ecc]
00008AA6  8BAFCC3D          mov bp,[bx+0x3dcc]
00008AAA  8A87CC3C          mov al,[bx+0x3ccc]
00008AAE  E81BF9            call 0x83cc
00008AB1  8BD8              mov bx,ax
00008AB3  368A28            mov ch,[ss:bx+si]
00008AB6  E847F9            call 0x8400
00008AB9  D1E3              shl bx,0x0
00008ABB  03EB              add bp,bx
00008ABD  8B7600            mov si,[bp+0x0]
00008AC0  803E012001        cmp byte [0x2001],0x1
00008AC5  746D              jz 0x8b34
00008AC7  803E002010        cmp byte [0x2000],0x10
00008ACC  736B              jnc 0x8b39
00008ACE  FE060020          inc byte [0x2000]
00008AD2  A10620            mov ax,[0x2006]
00008AD5  03C6              add ax,si
00008AD7  2AC4              sub al,ah
00008AD9  80DC00            sbb ah,0x0
00008ADC  A30620            mov [0x2006],ax
00008ADF  80C210            add dl,0x10
00008AE2  7304              jnc 0x8ae8
00008AE4  B290              mov dl,0x90
00008AE6  D0EE              shr dh,0x0
00008AE8  89160220          mov [0x2002],dx
00008AEC  D1E6              shl si,0x0
00008AEE  8B841E20          mov ax,[si+0x201e]
00008AF2  8B1ED43F          mov bx,[0x3fd4]
00008AF6  2BDF              sub bx,di
00008AF8  4B                dec bx
00008AF9  81FB1001          cmp bx,0x110
00008AFD  7248              jc 0x8b47
00008AFF  268825            mov [es:di],ah
00008B02  47                inc di
00008B03  8BD8              mov bx,ax
00008B05  FEC0              inc al
00008B07  32FF              xor bh,bh
00008B09  3CA1              cmp al,0xa1
00008B0B  774A              ja 0x8b57
00008B0D  FE871E23          inc byte [bx+0x231e]
00008B11  8A9F1E23          mov bl,[bx+0x231e]
00008B15  FECB              dec bl
00008B17  D1E3              shl bx,0x0
00008B19  8B971E20          mov dx,[bx+0x201e]
00008B1D  89941E20          mov [si+0x201e],dx
00008B21  89871E20          mov [bx+0x201e],ax
00008B25  832EE14301        sub word [0x43e1],0x1
00008B2A  831EE34300        sbb word [0x43e3],0x0
00008B2F  781D              js 0x8b4e
00008B31  E994FB            jmp 0x86c8
00008B34  4E                dec si
00008B35  799B              jns 0x8ad2
00008B37  EB2F              jmp 0x8b68
00008B39  803E052000        cmp byte [0x2005],0x0
00008B3E  758E              jnz 0x8ace
00008B40  C606012001        mov byte [0x2001],0x1
00008B45  EB87              jmp 0x8ace
00008B47  50                push ax
00008B48  E8E3F8            call 0x842e
00008B4B  58                pop ax
00008B4C  EBB1              jmp 0x8aff
00008B4E  E8DDF8            call 0x842e
00008B51  893ED93F          mov [0x3fd9],di
00008B55  5D                pop bp
00008B56  C3                ret
00008B57  B81E20            mov ax,0x201e
00008B5A  50                push ax
00008B5B  B81E23            mov ax,0x231e
00008B5E  50                push ax
00008B5F  E838F8            call 0x839a
00008B62  8B841E20          mov ax,[si+0x201e]
00008B66  EB9B              jmp 0x8b03
00008B68  E87DF8            call 0x83e8
00008B6B  8BD8              mov bx,ax
00008B6D  B501              mov ch,0x1
00008B6F  E88EF8            call 0x8400
00008B72  D1E3              shl bx,0x0
00008B74  7309              jnc 0x8b7f
00008B76  C70600200000      mov word [0x2000],0x0
00008B7C  E949FB            jmp 0x86c8
00008B7F  BD0300            mov bp,0x3
00008B82  D1E3              shl bx,0x0
00008B84  83D500            adc bp,0x0
00008B87  B106              mov cl,0x6
00008B89  D3EB              shr bx,cl
00008B8B  368AAF00B8        mov ch,[ss:bx-0x4800]
00008B90  FEC5              inc ch
00008B92  E86BF8            call 0x8400
00008B95  D1E3              shl bx,0x0
00008B97  368AB700B0        mov dh,[ss:bx-0x5000]
00008B9C  8B36F543          mov si,[0x43f5]
00008BA0  8B04              mov ax,[si]
00008BA2  86C4              xchg al,ah
00008BA4  8A0EF843          mov cl,[0x43f8]
00008BA8  D3E0              shl ax,cl
00008BAA  8AD4              mov dl,ah
00008BAC  52                push dx
00008BAD  B505              mov ch,0x5
00008BAF  E84EF8            call 0x8400
00008BB2  5A                pop dx
00008BB3  D1EA              shr dx,0x0
00008BB5  D1EA              shr dx,0x0
00008BB7  D1EA              shr dx,0x0
00008BB9  8BF2              mov si,dx
00008BBB  8BCD              mov cx,bp
00008BBD  E9AAFE            jmp 0x8a6a
00008BC0  1E                push ds
00008BC1  07                pop es
00008BC2  BF2C01            mov di,0x12c
00008BC5  B98600            mov cx,0x86
00008BC8  33C0              xor ax,ax
00008BCA  F3AB              rep stosw
00008BCC  BF3401            mov di,0x134
00008BCF  B105              mov cl,0x5
00008BD1  40                inc ax
00008BD2  FEC4              inc ah
00008BD4  AB                stosw
00008BD5  AB                stosw
00008BD6  E2F9              loop 0x8bd1
00008BD8  33C0              xor ax,ax
00008BDA  B105              mov cl,0x5
00008BDC  AB                stosw
00008BDD  47                inc di
00008BDE  47                inc di
00008BDF  40                inc ax
00008BE0  E2FA              loop 0x8bdc
00008BE2  48                dec ax
00008BE3  B10D              mov cl,0xd
00008BE5  BB0200            mov bx,0x2
00008BE8  BE0200            mov si,0x2
00008BEB  03C3              add ax,bx
00008BED  AB                stosw
00008BEE  47                inc di
00008BEF  47                inc di
00008BF0  4E                dec si
00008BF1  75F8              jnz 0x8beb
00008BF3  03DB              add bx,bx
00008BF5  E2F1              loop 0x8be8
00008BF7  03C3              add ax,bx
00008BF9  AB                stosw
00008BFA  83C704            add di,0x4
00008BFD  B80100            mov ax,0x1
00008C00  AB                stosw
00008C01  B80080            mov ax,0x8000
00008C04  AB                stosw
00008C05  B80100            mov ax,0x1
00008C08  B10F              mov cl,0xf
00008C0A  AB                stosw
00008C0B  47                inc di
00008C0C  47                inc di
00008C0D  40                inc ax
00008C0E  E2FA              loop 0x8c0a
00008C10  BF0C02            mov di,0x20c
00008C13  33C0              xor ax,ax
00008C15  B110              mov cl,0x10
00008C17  40                inc ax
00008C18  FEC4              inc ah
00008C1A  AB                stosw
00008C1B  E2FA              loop 0x8c17
00008C1D  B106              mov cl,0x6
00008C1F  F3AB              rep stosw
00008C21  C3                ret
00008C22  B82E00            mov ax,0x2e
00008C25  F7260203          mul word [0x302]
00008C29  8BF0              mov si,ax
00008C2B  8BBC5802          mov di,[si+0x258]
00008C2F  89BC5A02          mov [si+0x25a],di
00008C33  8B8C5602          mov cx,[si+0x256]
00008C37  898C5802          mov [si+0x258],cx
00008C3B  8B845C02          mov ax,[si+0x25c]
00008C3F  FF847402          inc word [si+0x274]
00008C43  8BD8              mov bx,ax
00008C45  2B845402          sub ax,[si+0x254]
00008C49  89845602          mov [si+0x256],ax
00008C4D  F7A44C02          mul word [si+0x24c]
00008C51  93                xchg ax,bx
00008C52  89845402          mov [si+0x254],ax
00008C56  F7A44A02          mul word [si+0x24a]
00008C5A  03D8              add bx,ax
00008C5C  8B844E02          mov ax,[si+0x24e]
00008C60  F7E1              mul cx
00008C62  03D8              add bx,ax
00008C64  8B845002          mov ax,[si+0x250]
00008C68  F7E7              mul di
00008C6A  03D8              add bx,ax
00008C6C  8B845202          mov ax,[si+0x252]
00008C70  F7264802          mul word [0x248]
00008C74  03D8              add bx,ax
00008C76  B103              mov cl,0x3
00008C78  8B847602          mov ax,[si+0x276]
00008C7C  D3E0              shl ax,cl
00008C7E  03C3              add ax,bx
00008C80  D3E8              shr ax,cl
00008C82  5B                pop bx
00008C83  32E4              xor ah,ah
00008C85  5F                pop di
00008C86  2BC7              sub ax,di
00008C88  53                push bx
00008C89  97                xchg ax,di
00008C8A  98                cbw
00008C8B  D3E0              shl ax,cl
00008C8D  8BD8              mov bx,ax
00008C8F  99                cwd
00008C90  33C2              xor ax,dx
00008C92  2BC2              sub ax,dx
00008C94  01845E02          add [si+0x25e],ax
00008C98  8BC3              mov ax,bx
00008C9A  2B845402          sub ax,[si+0x254]
00008C9E  99                cwd
00008C9F  33C2              xor ax,dx
00008CA1  2BC2              sub ax,dx
00008CA3  01846002          add [si+0x260],ax
00008CA7  8BC3              mov ax,bx
00008CA9  03845402          add ax,[si+0x254]
00008CAD  99                cwd
00008CAE  33C2              xor ax,dx
00008CB0  2BC2              sub ax,dx
00008CB2  01846202          add [si+0x262],ax
00008CB6  8BC3              mov ax,bx
00008CB8  2B845602          sub ax,[si+0x256]
00008CBC  99                cwd
00008CBD  33C2              xor ax,dx
00008CBF  2BC2              sub ax,dx
00008CC1  01846402          add [si+0x264],ax
00008CC5  8BC3              mov ax,bx
00008CC7  03845602          add ax,[si+0x256]
00008CCB  99                cwd
00008CCC  33C2              xor ax,dx
00008CCE  2BC2              sub ax,dx
00008CD0  01846602          add [si+0x266],ax
00008CD4  8BC3              mov ax,bx
00008CD6  2B845802          sub ax,[si+0x258]
00008CDA  99                cwd
00008CDB  33C2              xor ax,dx
00008CDD  2BC2              sub ax,dx
00008CDF  01846802          add [si+0x268],ax
00008CE3  8BC3              mov ax,bx
00008CE5  03845802          add ax,[si+0x258]
00008CE9  99                cwd
00008CEA  33C2              xor ax,dx
00008CEC  2BC2              sub ax,dx
00008CEE  01846A02          add [si+0x26a],ax
00008CF2  8BC3              mov ax,bx
00008CF4  2B845A02          sub ax,[si+0x25a]
00008CF8  99                cwd
00008CF9  33C2              xor ax,dx
00008CFB  2BC2              sub ax,dx
00008CFD  01846C02          add [si+0x26c],ax
00008D01  8BC3              mov ax,bx
00008D03  03845A02          add ax,[si+0x25a]
00008D07  99                cwd
00008D08  33C2              xor ax,dx
00008D0A  2BC2              sub ax,dx
00008D0C  01846E02          add [si+0x26e],ax
00008D10  8BC3              mov ax,bx
00008D12  2B064802          sub ax,[0x248]
00008D16  99                cwd
00008D17  33C2              xor ax,dx
00008D19  2BC2              sub ax,dx
00008D1B  01847002          add [si+0x270],ax
00008D1F  8BC3              mov ax,bx
00008D21  03064802          add ax,[0x248]
00008D25  99                cwd
00008D26  33C2              xor ax,dx
00008D28  2BC2              sub ax,dx
00008D2A  01847202          add [si+0x272],ax
00008D2E  8BC7              mov ax,di
00008D30  2B847602          sub ax,[si+0x276]
00008D34  98                cbw
00008D35  A34802            mov [0x248],ax
00008D38  89845C02          mov [si+0x25c],ax
00008D3C  89BC7602          mov [si+0x276],di
00008D40  8B847402          mov ax,[si+0x274]
00008D44  251F00            and ax,0x1f
00008D47  754D              jnz 0x8d96
00008D49  8B945E02          mov dx,[si+0x25e]
00008D4D  89845E02          mov [si+0x25e],ax
00008D51  8BD8              mov bx,ax
00008D53  83C302            add bx,0x2
00008D56  39905E02          cmp [bx+si+0x25e],dx
00008D5A  7306              jnc 0x8d62
00008D5C  8B905E02          mov dx,[bx+si+0x25e]
00008D60  8BC3              mov ax,bx
00008D62  C7805E020000      mov word [bx+si+0x25e],0x0
00008D68  83FB14            cmp bx,0x14
00008D6B  72E6              jc 0x8d53
00008D6D  0BC0              or ax,ax
00008D6F  7425              jz 0x8d96
00008D71  D1E8              shr ax,0x0
00008D73  8BD8              mov bx,ax
00008D75  4B                dec bx
00008D76  D1EB              shr bx,0x0
00008D78  03DB              add bx,bx
00008D7A  D1E8              shr ax,0x0
00008D7C  730D              jnc 0x8d8b
00008D7E  83B84A02F0        cmp word [bx+si+0x24a],0xfffffffffffffff0
00008D83  7C11              jl 0x8d96
00008D85  FF884A02          dec word [bx+si+0x24a]
00008D89  EB0B              jmp 0x8d96
00008D8B  83B84A0210        cmp word [bx+si+0x24a],0x10
00008D90  7D04              jnl 0x8d96
00008D92  FF804A02          inc word [bx+si+0x24a]
00008D96  8BC7              mov ax,di
00008D98  C3                ret
00008D99  55                push bp
00008D9A  8BEC              mov bp,sp
00008D9C  83EC40            sub sp,0x40
00008D9F  33C0              xor ax,ax
00008DA1  16                push ss
00008DA2  B91000            mov cx,0x10
00008DA5  07                pop es
00008DA6  8D7EE0            lea di,[bp-0x20]
00008DA9  F3AB              rep stosw
00008DAB  8B5E04            mov bx,[bp+0x4]
00008DAE  8B7E06            mov di,[bp+0x6]
00008DB1  4B                dec bx
00008DB2  780F              js 0x8dc3
00008DB4  368B01            mov ax,[ss:bx+di]
00008DB7  250F00            and ax,0xf
00008DBA  8BF0              mov si,ax
00008DBC  03F6              add si,si
00008DBE  FF42E0            inc word [bp+si-0x20]
00008DC1  EBEB              jmp 0x8dae
00008DC3  33C0              xor ax,ax
00008DC5  8B7E08            mov di,[bp+0x8]
00008DC8  8BD0              mov dx,ax
00008DCA  8946E0            mov [bp-0x20],ax
00008DCD  8946C0            mov [bp-0x40],ax
00008DD0  894502            mov [di+0x2],ax
00008DD3  894522            mov [di+0x22],ax
00008DD6  BE0200            mov si,0x2
00008DD9  B90E00            mov cx,0xe
00008DDC  0342E0            add ax,[bp+si-0x20]
00008DDF  83D200            adc dx,0x0
00008DE2  03C0              add ax,ax
00008DE4  D1D2              rcl dx,0x0
00008DE6  0BD2              or dx,dx
00008DE8  7506              jnz 0x8df0
00008DEA  8BD8              mov bx,ax
00008DEC  D3E0              shl ax,cl
00008DEE  7303              jnc 0x8df3
00008DF0  B8FFFF            mov ax,0xffff
00008DF3  87DE              xchg bx,si
00008DF5  894102            mov [bx+di+0x2],ax
00008DF8  56                push si
00008DF9  8BF3              mov si,bx
00008DFB  8B4120            mov ax,[bx+di+0x20]
00008DFE  0342DE            add ax,[bp+si-0x22]
00008E01  8942C0            mov [bp+si-0x40],ax
00008E04  894122            mov [bx+di+0x22],ax
00008E07  58                pop ax
00008E08  83C602            add si,0x2
00008E0B  49                dec cx
00008E0C  79CE              jns 0x8ddc
00008E0E  33DB              xor bx,bx
00008E10  8B4E04            mov cx,[bp+0x4]
00008E13  8B7E06            mov di,[bp+0x6]
00008E16  368A01            mov al,[ss:bx+di]
00008E19  0AC0              or al,al
00008E1B  7417              jz 0x8e34
00008E1D  8BF0              mov si,ax
00008E1F  83E60F            and si,0xf
00008E22  03F6              add si,si
00008E24  8B52C0            mov dx,[bp+si-0x40]
00008E27  8B7E08            mov di,[bp+0x8]
00008E2A  FF42C0            inc word [bp+si-0x40]
00008E2D  03D2              add dx,dx
00008E2F  03FA              add di,dx
00008E31  895D42            mov [di+0x42],bx
00008E34  43                inc bx
00008E35  3BD9              cmp bx,cx
00008E37  75DA              jnz 0x8e13
00008E39  8B7E08            mov di,[bp+0x8]
00008E3C  890D              mov [di],cx
00008E3E  8BE5              mov sp,bp
00008E40  5D                pop bp
00008E41  C20600            ret word 0x6
00008E44  A118A0            mov ax,[0xa018]
00008E47  050500            add ax,0x5
00008E4A  3B066214          cmp ax,[0x1462]
00008E4E  771B              ja 0x8e6b
00008E50  833E060300        cmp word [0x306],0x0
00008E55  7415              jz 0x8e6c
00008E57  B84402            mov ax,0x244
00008E5A  F7260203          mul word [0x302]
00008E5E  05A007            add ax,0x7a0
00008E61  8BF8              mov di,ax
00008E63  E85202            call 0x90b8
00008E66  3D0001            cmp ax,0x100
00008E69  740D              jz 0x8e78
00008E6B  C3                ret
00008E6C  BFB010            mov di,0x10b0
00008E6F  E84602            call 0x90b8
00008E72  3D0301            cmp ax,0x103
00008E75  7401              jz 0x8e78
00008E77  C3                ret
00008E78  55                push bp
00008E79  8BEC              mov bp,sp
00008E7B  81EC1B04          sub sp,0x41b
00008E7F  813E18A017DF      cmp word [0xa018],0xdf17
00008E85  7603              jna 0x8e8a
00008E87  E87203            call 0x91fc
00008E8A  8B3618A0          mov si,[0xa018]
00008E8E  8A24              mov ah,[si]
00008E90  8A4401            mov al,[si+0x1]
00008E93  8B0E0403          mov cx,[0x304]
00008E97  D3E0              shl ax,cl
00008E99  33D2              xor dx,dx
00008E9B  D1E0              shl ax,0x0
00008E9D  D1D2              rcl dx,0x0
00008E9F  89160603          mov [0x306],dx
00008EA3  D1E0              shl ax,0x0
00008EA5  8BD8              mov bx,ax
00008EA7  720C              jc 0x8eb5
00008EA9  1E                push ds
00008EAA  BF3203            mov di,0x332
00008EAD  07                pop es
00008EAE  33C0              xor ax,ax
00008EB0  B90202            mov cx,0x202
00008EB3  F3AB              rep stosw
00008EB5  8306040302        add word [0x304],0x2
00008EBA  A10403            mov ax,[0x304]
00008EBD  8326040307        and word [0x304],0x7
00008EC2  B103              mov cl,0x3
00008EC4  D3E8              shr ax,cl
00008EC6  010618A0          add [0xa018],ax
00008ECA  833E060300        cmp word [0x306],0x0
00008ECF  7434              jz 0x8f05
00008ED1  8BC3              mov ax,bx
00008ED3  B10E              mov cl,0xe
00008ED5  D3E8              shr ax,cl
00008ED7  40                inc ax
00008ED8  A33607            mov [0x736],ax
00008EDB  3B060203          cmp ax,[0x302]
00008EDF  7706              ja 0x8ee7
00008EE1  C70602030000      mov word [0x302],0x0
00008EE7  8306040302        add word [0x304],0x2
00008EEC  A10403            mov ax,[0x304]
00008EEF  8326040307        and word [0x304],0x7
00008EF4  B103              mov cl,0x3
00008EF6  D3E8              shr ax,cl
00008EF8  010618A0          add [0xa018],ax
00008EFC  B80101            mov ax,0x101
00008EFF  F7263607          mul word [0x736]
00008F03  EB03              jmp 0x8f08
00008F05  B87601            mov ax,0x176
00008F08  8986E7FB          mov [bp-0x419],ax
00008F0C  33FF              xor di,di
00008F0E  8B3618A0          mov si,[0xa018]
00008F12  8A24              mov ah,[si]
00008F14  8A4401            mov al,[si+0x1]
00008F17  B90C00            mov cx,0xc
00008F1A  2B0E0403          sub cx,[0x304]
00008F1E  D3E8              shr ax,cl
00008F20  250F00            and ax,0xf
00008F23  8843ED            mov [bp+di-0x13],al
00008F26  8306040304        add word [0x304],0x4
00008F2B  A10403            mov ax,[0x304]
00008F2E  8326040307        and word [0x304],0x7
00008F33  B103              mov cl,0x3
00008F35  D3E8              shr ax,cl
00008F37  010618A0          add [0xa018],ax
00008F3B  47                inc di
00008F3C  83FF13            cmp di,0x13
00008F3F  75CD              jnz 0x8f0e
00008F41  B83807            mov ax,0x738
00008F44  50                push ax
00008F45  8D7EED            lea di,[bp-0x13]
00008F48  57                push di
00008F49  B81300            mov ax,0x13
00008F4C  50                push ax
00008F4D  E849FE            call 0x8d99
00008F50  33F6              xor si,si
00008F52  89B6E5FB          mov [bp-0x41b],si
00008F56  3BB6E7FB          cmp si,[bp-0x419]
00008F5A  7203              jc 0x8f5f
00008F5C  E9DB00            jmp 0x903a
00008F5F  813E18A02BDF      cmp word [0xa018],0xdf2b
00008F65  7603              jna 0x8f6a
00008F67  E89202            call 0x91fc
00008F6A  BF3807            mov di,0x738
00008F6D  E84801            call 0x90b8
00008F70  3D1000            cmp ax,0x10
00008F73  7311              jnc 0x8f86
00008F75  8BB6E5FB          mov si,[bp-0x41b]
00008F79  02843203          add al,[si+0x332]
00008F7D  240F              and al,0xf
00008F7F  8882E9FB          mov [bp+si-0x417],al
00008F83  46                inc si
00008F84  EBCC              jmp 0x8f52
00008F86  7547              jnz 0x8fcf
00008F88  8B3618A0          mov si,[0xa018]
00008F8C  8A24              mov ah,[si]
00008F8E  8A4401            mov al,[si+0x1]
00008F91  B90E00            mov cx,0xe
00008F94  2B0E0403          sub cx,[0x304]
00008F98  D3E8              shr ax,cl
00008F9A  250300            and ax,0x3
00008F9D  050300            add ax,0x3
00008FA0  8BD0              mov dx,ax
00008FA2  8306040302        add word [0x304],0x2
00008FA7  A10403            mov ax,[0x304]
00008FAA  8326040307        and word [0x304],0x7
00008FAF  B103              mov cl,0x3
00008FB1  D3E8              shr ax,cl
00008FB3  010618A0          add [0xa018],ax
00008FB7  8BB6E5FB          mov si,[bp-0x41b]
00008FBB  3BB6E7FB          cmp si,[bp-0x419]
00008FBF  7379              jnc 0x903a
00008FC1  8A82E8FB          mov al,[bp+si-0x418]
00008FC5  8882E9FB          mov [bp+si-0x417],al
00008FC9  46                inc si
00008FCA  4A                dec dx
00008FCB  75EE              jnz 0x8fbb
00008FCD  EB83              jmp 0x8f52
00008FCF  3D1100            cmp ax,0x11
00008FD2  7521              jnz 0x8ff5
00008FD4  8B3618A0          mov si,[0xa018]
00008FD8  8A24              mov ah,[si]
00008FDA  8A4401            mov al,[si+0x1]
00008FDD  B90D00            mov cx,0xd
00008FE0  2B0E0403          sub cx,[0x304]
00008FE4  D3E8              shr ax,cl
00008FE6  250700            and ax,0x7
00008FE9  8BD0              mov dx,ax
00008FEB  B103              mov cl,0x3
00008FED  03D1              add dx,cx
00008FEF  010E0403          add [0x304],cx
00008FF3  EB21              jmp 0x9016
00008FF5  8B3618A0          mov si,[0xa018]
00008FF9  8A24              mov ah,[si]
00008FFB  8A4401            mov al,[si+0x1]
00008FFE  B90900            mov cx,0x9
00009001  2B0E0403          sub cx,[0x304]
00009005  D3E8              shr ax,cl
00009007  257F00            and ax,0x7f
0000900A  050B00            add ax,0xb
0000900D  8BD0              mov dx,ax
0000900F  B103              mov cl,0x3
00009011  8306040307        add word [0x304],0x7
00009016  A10403            mov ax,[0x304]
00009019  8326040307        and word [0x304],0x7
0000901E  D3E8              shr ax,cl
00009020  010618A0          add [0xa018],ax
00009024  8BB6E5FB          mov si,[bp-0x41b]
00009028  C682E9FB00        mov byte [bp+si-0x417],0x0
0000902D  46                inc si
0000902E  4A                dec dx
0000902F  7503              jnz 0x9034
00009031  E91EFF            jmp 0x8f52
00009034  3BB6E7FB          cmp si,[bp-0x419]
00009038  72EE              jc 0x9028
0000903A  833E060300        cmp word [0x306],0x0
0000903F  7431              jz 0x9072
00009041  C786E7FB0000      mov word [bp-0x419],0x0
00009047  8B86E7FB          mov ax,[bp-0x419]
0000904B  3B063607          cmp ax,[0x736]
0000904F  7351              jnc 0x90a2
00009051  8BD8              mov bx,ax
00009053  B90101            mov cx,0x101
00009056  F7E1              mul cx
00009058  8DBEE9FB          lea di,[bp-0x417]
0000905C  03F8              add di,ax
0000905E  B84402            mov ax,0x244
00009061  F7E3              mul bx
00009063  05A007            add ax,0x7a0
00009066  50                push ax
00009067  57                push di
00009068  51                push cx
00009069  E82DFD            call 0x8d99
0000906C  FF86E7FB          inc word [bp-0x419]
00009070  EBD5              jmp 0x9047
00009072  B8B010            mov ax,0x10b0
00009075  50                push ax
00009076  8DBEE9FB          lea di,[bp-0x417]
0000907A  57                push di
0000907B  B82A01            mov ax,0x12a
0000907E  50                push ax
0000907F  E817FD            call 0x8d99
00009082  B84613            mov ax,0x1346
00009085  50                push ax
00009086  8DBE13FD          lea di,[bp-0x2ed]
0000908A  57                push di
0000908B  B83000            mov ax,0x30
0000908E  50                push ax
0000908F  E807FD            call 0x8d99
00009092  B8E813            mov ax,0x13e8
00009095  50                push ax
00009096  8DBE43FD          lea di,[bp-0x2bd]
0000909A  57                push di
0000909B  B81C00            mov ax,0x1c
0000909E  50                push ax
0000909F  E8F7FC            call 0x8d99
000090A2  1E                push ds
000090A3  16                push ss
000090A4  1E                push ds
000090A5  8DB6E9FB          lea si,[bp-0x417]
000090A9  07                pop es
000090AA  BF3203            mov di,0x332
000090AD  1F                pop ds
000090AE  B90202            mov cx,0x202
000090B1  F3A5              rep movsw
000090B3  1F                pop ds
000090B4  8BE5              mov sp,bp
000090B6  5D                pop bp
000090B7  C3                ret
000090B8  8B3618A0          mov si,[0xa018]
000090BC  8B04              mov ax,[si]
000090BE  8AD0              mov dl,al
000090C0  8A4402            mov al,[si+0x2]
000090C3  B90800            mov cx,0x8
000090C6  2B0E0403          sub cx,[0x304]
000090CA  D3E8              shr ax,cl
000090CC  F7D9              neg cx
000090CE  83C110            add cx,0x10
000090D1  D3E2              shl dx,cl
000090D3  0BC2              or ax,dx
000090D5  25FEFF            and ax,0xfffe
000090D8  8BD0              mov dx,ax
000090DA  3B4512            cmp ax,[di+0x12]
000090DD  734B              jnc 0x912a
000090DF  3B450A            cmp ax,[di+0xa]
000090E2  7323              jnc 0x9107
000090E4  3B4506            cmp ax,[di+0x6]
000090E7  730F              jnc 0x90f8
000090E9  3B4504            cmp ax,[di+0x4]
000090EC  7305              jnc 0x90f3
000090EE  BB0100            mov bx,0x1
000090F1  EB76              jmp 0x9169
000090F3  BB0200            mov bx,0x2
000090F6  EB71              jmp 0x9169
000090F8  3B4508            cmp ax,[di+0x8]
000090FB  7305              jnc 0x9102
000090FD  BB0300            mov bx,0x3
00009100  EB67              jmp 0x9169
00009102  BB0400            mov bx,0x4
00009105  EB62              jmp 0x9169
00009107  3B450E            cmp ax,[di+0xe]
0000910A  730F              jnc 0x911b
0000910C  3B450C            cmp ax,[di+0xc]
0000910F  7305              jnc 0x9116
00009111  BB0500            mov bx,0x5
00009114  EB53              jmp 0x9169
00009116  BB0600            mov bx,0x6
00009119  EB4E              jmp 0x9169
0000911B  3B4510            cmp ax,[di+0x10]
0000911E  7305              jnc 0x9125
00009120  BB0700            mov bx,0x7
00009123  EB44              jmp 0x9169
00009125  BB0800            mov bx,0x8
00009128  EB3F              jmp 0x9169
0000912A  3B451A            cmp ax,[di+0x1a]
0000912D  7323              jnc 0x9152
0000912F  3B4516            cmp ax,[di+0x16]
00009132  730F              jnc 0x9143
00009134  3B4514            cmp ax,[di+0x14]
00009137  7305              jnc 0x913e
00009139  BB0900            mov bx,0x9
0000913C  EB2B              jmp 0x9169
0000913E  BB0A00            mov bx,0xa
00009141  EB26              jmp 0x9169
00009143  3B4518            cmp ax,[di+0x18]
00009146  7305              jnc 0x914d
00009148  BB0B00            mov bx,0xb
0000914B  EB1C              jmp 0x9169
0000914D  BB0C00            mov bx,0xc
00009150  EB17              jmp 0x9169
00009152  3B451E            cmp ax,[di+0x1e]
00009155  730F              jnc 0x9166
00009157  3B451C            cmp ax,[di+0x1c]
0000915A  7305              jnc 0x9161
0000915C  BB0D00            mov bx,0xd
0000915F  EB08              jmp 0x9169
00009161  BB0E00            mov bx,0xe
00009164  EB03              jmp 0x9169
00009166  BB0F00            mov bx,0xf
00009169  011E0403          add [0x304],bx
0000916D  A10403            mov ax,[0x304]
00009170  8326040307        and word [0x304],0x7
00009175  B103              mov cl,0x3
00009177  D3E8              shr ax,cl
00009179  010618A0          add [0xa018],ax
0000917D  B91000            mov cx,0x10
00009180  2BCB              sub cx,bx
00009182  03DB              add bx,bx
00009184  8BC2              mov ax,dx
00009186  2B01              sub ax,[bx+di]
00009188  D3E8              shr ax,cl
0000918A  034122            add ax,[bx+di+0x22]
0000918D  3B05              cmp ax,[di]
0000918F  7202              jc 0x9193
00009191  33C0              xor ax,ax
00009193  8BD8              mov bx,ax
00009195  03DB              add bx,bx
00009197  8B4142            mov ax,[bx+di+0x42]
0000919A  C3                ret
0000919B  33C0              xor ax,ax
0000919D  A30403            mov [0x304],ax
000091A0  A39615            mov [0x1596],ax
000091A3  39060EA0          cmp [0xa00e],ax
000091A7  753F              jnz 0x91e8
000091A9  A34802            mov [0x248],ax
000091AC  A30C03            mov [0x30c],ax
000091AF  A3A215            mov [0x15a2],ax
000091B2  A30203            mov [0x302],ax
000091B5  A36414            mov [0x1464],ax
000091B8  A36814            mov [0x1468],ax
000091BB  A36A14            mov [0x146a],ax
000091BE  A36614            mov [0x1466],ax
000091C1  A38215            mov [0x1582],ax
000091C4  A38415            mov [0x1584],ax
000091C7  C43E9615          les di,word [0x1596]
000091CB  B90080            mov cx,0x8000
000091CE  F3AB              rep stosw
000091D0  1E                push ds
000091D1  07                pop es
000091D2  BF4A02            mov di,0x24a
000091D5  B15C              mov cl,0x5c
000091D7  F3AB              rep stosw
000091D9  BF8615            mov di,0x1586
000091DC  B108              mov cl,0x8
000091DE  F3AB              rep stosw
000091E0  BF3203            mov di,0x332
000091E3  B90202            mov cx,0x202
000091E6  F3AB              rep stosw
000091E8  C3                ret
000091E9  1E                push ds
000091EA  B840A0            mov ax,0xa040
000091ED  A318A0            mov [0xa018],ax
000091F0  A36214            mov [0x1462],ax
000091F3  50                push ax
000091F4  B8F03E            mov ax,0x3ef0
000091F7  50                push ax
000091F8  E8A905            call 0x97a4
000091FB  C3                ret
000091FC  B91000            mov cx,0x10
000091FF  1E                push ds
00009200  BF40A0            mov di,0xa040
00009203  07                pop es
00009204  BE10DF            mov si,0xdf10
00009207  F3A5              rep movsw
00009209  B9D03E            mov cx,0x3ed0
0000920C  1E                push ds
0000920D  BF60A0            mov di,0xa060
00009210  57                push di
00009211  290E18A0          sub [0xa018],cx
00009215  51                push cx
00009216  E88B05            call 0x97a4
00009219  0BC0              or ax,ax
0000921B  7407              jz 0x9224
0000921D  0560A0            add ax,0xa060
00009220  A36214            mov [0x1462],ax
00009223  C3                ret
00009224  A118A0            mov ax,[0xa018]
00009227  A36214            mov [0x1462],ax
0000922A  C3                ret
0000922B  C43E9615          les di,word [0x1596]
0000922F  A18415            mov ax,[0x1584]
00009232  03F8              add di,ax
00009234  06                push es
00009235  57                push di
00009236  3B068215          cmp ax,[0x1582]
0000923A  7223              jc 0x925f
0000923C  0BC0              or ax,ax
0000923E  750D              jnz 0x924d
00009240  B80080            mov ax,0x8000
00009243  01068415          add [0x1584],ax
00009247  50                push ax
00009248  E8CB05            call 0x9816
0000924B  EBDE              jmp 0x922b
0000924D  F7D8              neg ax
0000924F  50                push ax
00009250  E8C305            call 0x9816
00009253  C43E9615          les di,word [0x1596]
00009257  06                push es
00009258  57                push di
00009259  FF368215          push word [0x1582]
0000925D  EB07              jmp 0x9266
0000925F  8B1E8215          mov bx,[0x1582]
00009263  2BD8              sub bx,ax
00009265  53                push bx
00009266  E8AD05            call 0x9816
00009269  A18215            mov ax,[0x1582]
0000926C  A38415            mov [0x1584],ax
0000926F  1E                push ds
00009270  8E1E34A0          mov ds,word [0xa034]
00009274  803EE5A300        cmp byte [0xa3e5],0x0
00009279  7411              jz 0x928c
0000927B  FE0EE1A3          dec byte [0xa3e1]
0000927F  790B              jns 0x928c
00009281  A0BABC            mov al,[0xbcba]
00009284  A2E1A3            mov [0xa3e1],al
00009287  B0DB              mov al,0xdb
00009289  E88B75            call 0x817
0000928C  1F                pop ds
0000928D  C3                ret
0000928E  B409              mov ah,0x9
00009290  FF1E87BD          call word far [0xbd87]
00009294  0BC0              or ax,ax
00009296  7502              jnz 0x929a
00009298  33D2              xor dx,dx
0000929A  89168BBD          mov [0xbd8b],dx
0000929E  C3                ret
0000929F  8B168BBD          mov dx,[0xbd8b]
000092A3  0BD2              or dx,dx
000092A5  7406              jz 0x92ad
000092A7  B40A              mov ah,0xa
000092A9  FF1E87BD          call word far [0xbd87]
000092AD  C3                ret
000092AE  B81043            mov ax,0x4310
000092B1  CD2F              int byte 0x2f
000092B3  891E87BD          mov [0xbd87],bx
000092B7  8C0689BD          mov word [0xbd89],es
000092BB  C3                ret
000092BC  B80043            mov ax,0x4300
000092BF  CD2F              int byte 0x2f
000092C1  3C80              cmp al,0x80
000092C3  7402              jz 0x92c7
000092C5  32C0              xor al,al
000092C7  C3                ret
000092C8  8B360C03          mov si,[0x30c]
000092CC  46                inc si
000092CD  3B361EA0          cmp si,[0xa01e]
000092D1  7602              jna 0x92d5
000092D3  33F6              xor si,si
000092D5  89360C03          mov [0x30c],si
000092D9  8B360C03          mov si,[0x30c]
000092DD  03F6              add si,si
000092DF  8E841003          mov es,word [si+0x310]
000092E3  1E                push ds
000092E4  C5369615          lds si,word [0x1596]
000092E8  33FF              xor di,di
000092EA  B90080            mov cx,0x8000
000092ED  F3A5              rep movsw
000092EF  1F                pop ds
000092F0  8C069815          mov word [0x1598],es
000092F4  A11EA0            mov ax,[0xa01e]
000092F7  3B061AA0          cmp ax,[0xa01a]
000092FB  73CA              jnc 0x92c7
000092FD  A10CA0            mov ax,[0xa00c]
00009300  0BC0              or ax,ax
00009302  744D              jz 0x9351
00009304  BE3802            mov si,0x238
00009307  89440A            mov [si+0xa],ax
0000930A  33C0              xor ax,ax
0000930C  C744020100        mov word [si+0x2],0x1
00009311  8904              mov [si],ax
00009313  894404            mov [si+0x4],ax
00009316  89440C            mov [si+0xc],ax
00009319  C4069615          les ax,word [0x1596]
0000931D  894406            mov [si+0x6],ax
00009320  8C4408            mov word [si+0x8],es
00009323  A1A215            mov ax,[0x15a2]
00009326  89440E            mov [si+0xe],ax
00009329  B40B              mov ah,0xb
0000932B  FF1E08A0          call word far [0xa008]
0000932F  833EA21500        cmp word [0x15a2],0x0
00009334  751E              jnz 0x9354
00009336  BE3802            mov si,0x238
00009339  C7040004          mov word [si],0x400
0000933D  C744020000        mov word [si+0x2],0x0
00009342  A11AA0            mov ax,[0xa01a]
00009345  40                inc ax
00009346  89440E            mov [si+0xe],ax
00009349  B40B              mov ah,0xb
0000934B  FF1E08A0          call word far [0xa008]
0000934F  EB03              jmp 0x9354
00009351  E963DC            jmp 0x6fb7
00009354  A1A215            mov ax,[0x15a2]
00009357  40                inc ax
00009358  3B061AA0          cmp ax,[0xa01a]
0000935C  7602              jna 0x9360
0000935E  33C0              xor ax,ax
00009360  A3A215            mov [0x15a2],ax
00009363  C3                ret
00009364  A16C14            mov ax,[0x146c]
00009367  8B166E14          mov dx,[0x146e]
0000936B  A36814            mov [0x1468],ax
0000936E  89166A14          mov [0x146a],dx
00009372  8B366414          mov si,[0x1464]
00009376  83E603            and si,0x3
00009379  03F6              add si,si
0000937B  03F6              add si,si
0000937D  89848615          mov [si+0x1586],ax
00009381  89948815          mov [si+0x1588],dx
00009385  FF066414          inc word [0x1464]
00009389  8B0E7014          mov cx,[0x1470]
0000938D  890E6614          mov [0x1466],cx
00009391  EB0C              jmp 0x939f
00009393  90                nop
00009394  A16C14            mov ax,[0x146c]
00009397  8B166E14          mov dx,[0x146e]
0000939B  8B0E7014          mov cx,[0x1470]
0000939F  0BD2              or dx,dx
000093A1  7544              jnz 0x93e7
000093A3  1E                push ds
000093A4  C43E9615          les di,word [0x1596]
000093A8  033E8215          add di,[0x1582]
000093AC  8BF7              mov si,di
000093AE  2BF0              sub si,ax
000093B0  8E1E9815          mov ds,word [0x1598]
000093B4  8BC7              mov ax,di
000093B6  03C1              add ax,cx
000093B8  7628              jna 0x93e2
000093BA  F3A4              rep movsb
000093BC  1F                pop ds
000093BD  893E8215          mov [0x1582],di
000093C1  A17014            mov ax,[0x1470]
000093C4  C3                ret
000093C5  A4                movsb
000093C6  0BFF              or di,di
000093C8  7518              jnz 0x93e2
000093CA  1F                pop ds
000093CB  833E1AA000        cmp word [0xa01a],0x0
000093D0  7409              jz 0x93db
000093D2  51                push cx
000093D3  56                push si
000093D4  E8F1FE            call 0x92c8
000093D7  5E                pop si
000093D8  59                pop cx
000093D9  33FF              xor di,di
000093DB  1E                push ds
000093DC  8E1E9815          mov ds,word [0x1598]
000093E0  EBD8              jmp 0x93ba
000093E2  49                dec cx
000093E3  79E0              jns 0x93c5
000093E5  EBD5              jmp 0x93bc
000093E7  39161AA0          cmp [0xa01a],dx
000093EB  7D02              jnl 0x93ef
000093ED  EBB4              jmp 0x93a3
000093EF  A16C14            mov ax,[0x146c]
000093F2  8B166E14          mov dx,[0x146e]
000093F6  2B068215          sub ax,[0x1582]
000093FA  83DA00            sbb dx,0x0
000093FD  2D0100            sub ax,0x1
00009400  83DA00            sbb dx,0x0
00009403  8BD8              mov bx,ax
00009405  40                inc ax
00009406  7404              jz 0x940c
00009408  3BC1              cmp ax,cx
0000940A  7206              jc 0x9412
0000940C  3B161EA0          cmp dx,[0xa01e]
00009410  7264              jc 0x9476
00009412  A1A215            mov ax,[0x15a2]
00009415  2BC2              sub ax,dx
00009417  7E06              jng 0x941f
00009419  48                dec ax
0000941A  EB07              jmp 0x9423
0000941C  E998DB            jmp 0x6fb7
0000941F  03061AA0          add ax,[0xa01a]
00009423  8BD3              mov dx,bx
00009425  F7D2              not dx
00009427  8BC8              mov cx,ax
00009429  A10CA0            mov ax,[0xa00c]
0000942C  0BC0              or ax,ax
0000942E  74EC              jz 0x941c
00009430  BE3802            mov si,0x238
00009433  894404            mov [si+0x4],ax
00009436  A17014            mov ax,[0x1470]
00009439  40                inc ax
0000943A  D1E8              shr ax,0x0
0000943C  03C0              add ax,ax
0000943E  8904              mov [si],ax
00009440  33C0              xor ax,ax
00009442  894402            mov [si+0x2],ax
00009445  89440A            mov [si+0xa],ax
00009448  C7440C7214        mov word [si+0xc],0x1472
0000944D  8C5C0E            mov word [si+0xe],ds
00009450  895406            mov [si+0x6],dx
00009453  894C08            mov [si+0x8],cx
00009456  B40B              mov ah,0xb
00009458  FF1E08A0          call word far [0xa008]
0000945C  8B0E7014          mov cx,[0x1470]
00009460  C43E9615          les di,word [0x1596]
00009464  033E8215          add di,[0x1582]
00009468  BE7214            mov si,0x1472
0000946B  8BC7              mov ax,di
0000946D  03C1              add ax,cx
0000946F  761F              jna 0x9490
00009471  F3A4              rep movsb
00009473  E947FF            jmp 0x93bd
00009476  A10C03            mov ax,[0x30c]
00009479  2BC2              sub ax,dx
0000947B  7E19              jng 0x9496
0000947D  48                dec ax
0000947E  EB1A              jmp 0x949a
00009480  51                push cx
00009481  56                push si
00009482  E843FE            call 0x92c8
00009485  5E                pop si
00009486  59                pop cx
00009487  33FF              xor di,di
00009489  EBE6              jmp 0x9471
0000948B  A4                movsb
0000948C  0BFF              or di,di
0000948E  74F0              jz 0x9480
00009490  49                dec cx
00009491  79F8              jns 0x948b
00009493  E927FF            jmp 0x93bd
00009496  03061EA0          add ax,[0xa01e]
0000949A  1E                push ds
0000949B  07                pop es
0000949C  8B0E7014          mov cx,[0x1470]
000094A0  BF7214            mov di,0x1472
000094A3  8BF3              mov si,bx
000094A5  8BD8              mov bx,ax
000094A7  03DB              add bx,bx
000094A9  F7D6              not si
000094AB  1E                push ds
000094AC  8E9F1003          mov ds,word [bx+0x310]
000094B0  F3A4              rep movsb
000094B2  1F                pop ds
000094B3  EBA7              jmp 0x945c
000094B5  8E06B2B1          mov es,word [0xb1b2]
000094B9  BE8EBC            mov si,0xbc8e
000094BC  BF40A0            mov di,0xa040
000094BF  B92100            mov cx,0x21
000094C2  F3A4              rep movsb
000094C4  A1BCBD            mov ax,[0xbdbc]
000094C7  0E                push cs
000094C8  1F                pop ds
000094C9  BEF450            mov si,0x50f4
000094CC  BF0000            mov di,0x0
000094CF  B196              mov cl,0x96
000094D1  F3A5              rep movsw
000094D3  06                push es
000094D4  1F                pop ds
000094D5  A31AA0            mov [0xa01a],ax
000094D8  E8E5F6            call 0x8bc0
000094DB  803E22A014        cmp byte [0xa022],0x14
000094E0  7206              jc 0x94e8
000094E2  BE40A0            mov si,0xa040
000094E5  E8CF06            call 0x9bb7
000094E8  E82AD0            call 0x6515
000094EB  8CD8              mov ax,ds
000094ED  050010            add ax,0x1000
000094F0  BF1003            mov di,0x310
000094F3  8B161EA0          mov dx,[0xa01e]
000094F7  803E0EA000        cmp byte [0xa00e],0x0
000094FC  7508              jnz 0x9506
000094FE  A39815            mov [0x1598],ax
00009501  EB03              jmp 0x9506
00009503  050010            add ax,0x1000
00009506  AB                stosw
00009507  4A                dec dx
00009508  79F9              jns 0x9503
0000950A  E88EFC            call 0x919b
0000950D  E8D9FC            call 0x91e9
00009510  803E0EA000        cmp byte [0xa00e],0x0
00009515  7503              jnz 0x951a
00009517  E85EF9            call 0x8e78
0000951A  B80100            mov ax,0x1
0000951D  290604A0          sub [0xa004],ax
00009521  831E06A000        sbb word [0xa006],0x0
00009526  7907              jns 0x952f
00009528  E819F9            call 0x8e44
0000952B  E8FDFC            call 0x922b
0000952E  C3                ret
0000952F  813E18A012DF      cmp word [0xa018],0xdf12
00009535  7603              jna 0x953a
00009537  E8C2FC            call 0x91fc
0000953A  A18415            mov ax,[0x1584]
0000953D  2B068215          sub ax,[0x1582]
00009541  7408              jz 0x954b
00009543  3D0E01            cmp ax,0x10e
00009546  7303              jnc 0x954b
00009548  E8E0FC            call 0x922b
0000954B  833E060300        cmp word [0x306],0x0
00009550  744C              jz 0x959e
00009552  B84402            mov ax,0x244
00009555  F7260203          mul word [0x302]
00009559  05A007            add ax,0x7a0
0000955C  8BF8              mov di,ax
0000955E  E857FB            call 0x90b8
00009561  3D0001            cmp ax,0x100
00009564  7433              jz 0x9599
00009566  50                push ax
00009567  E8B8F6            call 0x8c22
0000956A  FF060203          inc word [0x302]
0000956E  8B160203          mov dx,[0x302]
00009572  3B163607          cmp dx,[0x736]
00009576  7506              jnz 0x957e
00009578  C70602030000      mov word [0x302],0x0
0000957E  C43E9615          les di,word [0x1596]
00009582  8B3E8215          mov di,[0x1582]
00009586  AA                stosb
00009587  FF068215          inc word [0x1582]
0000958B  758D              jnz 0x951a
0000958D  833E1AA000        cmp word [0xa01a],0x0
00009592  7486              jz 0x951a
00009594  E831FD            call 0x92c8
00009597  EB81              jmp 0x951a
00009599  E8DCF8            call 0x8e78
0000959C  EB91              jmp 0x952f
0000959E  BFB010            mov di,0x10b0
000095A1  E814FB            call 0x90b8
000095A4  3D0001            cmp ax,0x100
000095A7  72D5              jc 0x957e
000095A9  3D0D01            cmp ax,0x10d
000095AC  7705              ja 0x95b3
000095AE  74E9              jz 0x9599
000095B0  E9EA00            jmp 0x969d
000095B3  2D0E01            sub ax,0x10e
000095B6  8BF0              mov si,ax
000095B8  8A841001          mov al,[si+0x110]
000095BC  050300            add ax,0x3
000095BF  A37014            mov [0x1470],ax
000095C2  33DB              xor bx,bx
000095C4  8A9C2C01          mov bl,[si+0x12c]
000095C8  0BDB              or bx,bx
000095CA  742D              jz 0x95f9
000095CC  8B3618A0          mov si,[0xa018]
000095D0  8A24              mov ah,[si]
000095D2  8A4401            mov al,[si+0x1]
000095D5  8A0E0403          mov cl,[0x304]
000095D9  D3E0              shl ax,cl
000095DB  B110              mov cl,0x10
000095DD  2ACB              sub cl,bl
000095DF  D3E8              shr ax,cl
000095E1  01067014          add [0x1470],ax
000095E5  011E0403          add [0x304],bx
000095E9  A10403            mov ax,[0x304]
000095EC  8326040307        and word [0x304],0x7
000095F1  B103              mov cl,0x3
000095F3  D3E8              shr ax,cl
000095F5  010618A0          add [0xa018],ax
000095F9  BF4613            mov di,0x1346
000095FC  E8B9FA            call 0x90b8
000095FF  8BF0              mov si,ax
00009601  33DB              xor bx,bx
00009603  8A9C0802          mov bl,[si+0x208]
00009607  03F6              add si,si
00009609  03F6              add si,si
0000960B  8B844801          mov ax,[si+0x148]
0000960F  40                inc ax
00009610  A36C14            mov [0x146c],ax
00009613  8B844A01          mov ax,[si+0x14a]
00009617  A36E14            mov [0x146e],ax
0000961A  0BDB              or bx,bx
0000961C  7440              jz 0x965e
0000961E  8B3618A0          mov si,[0xa018]
00009622  8B04              mov ax,[si]
00009624  8AD0              mov dl,al
00009626  8A4402            mov al,[si+0x2]
00009629  B90800            mov cx,0x8
0000962C  2B0E0403          sub cx,[0x304]
00009630  D3E8              shr ax,cl
00009632  F7D9              neg cx
00009634  83C110            add cx,0x10
00009637  D3E2              shl dx,cl
00009639  0BC2              or ax,dx
0000963B  B110              mov cl,0x10
0000963D  2ACB              sub cl,bl
0000963F  D3E8              shr ax,cl
00009641  01066C14          add [0x146c],ax
00009645  83166E1400        adc word [0x146e],0x0
0000964A  011E0403          add [0x304],bx
0000964E  A10403            mov ax,[0x304]
00009651  8326040307        and word [0x304],0x7
00009656  B103              mov cl,0x3
00009658  D3E8              shr ax,cl
0000965A  010618A0          add [0xa018],ax
0000965E  A16E14            mov ax,[0x146e]
00009661  3D0400            cmp ax,0x4
00009664  721A              jc 0x9680
00009666  8306701402        add word [0x1470],0x2
0000966B  EB23              jmp 0x9690
0000966D  A16614            mov ax,[0x1466]
00009670  A37014            mov [0x1470],ax
00009673  C4066814          les ax,word [0x1468]
00009677  A36C14            mov [0x146c],ax
0000967A  8C066E14          mov word [0x146e],es
0000967E  EB10              jmp 0x9690
00009680  0BC0              or ax,ax
00009682  7508              jnz 0x968c
00009684  813E6C140020      cmp word [0x146c],0x2000
0000968A  7204              jc 0x9690
0000968C  FF067014          inc word [0x1470]
00009690  E8D1FC            call 0x9364
00009693  893E8215          mov [0x1582],di
00009697  A17014            mov ax,[0x1470]
0000969A  E980FE            jmp 0x951d
0000969D  3D0001            cmp ax,0x100
000096A0  74CB              jz 0x966d
000096A2  2D0501            sub ax,0x105
000096A5  7203              jc 0x96aa
000096A7  E9AE00            jmp 0x9758
000096AA  050500            add ax,0x5
000096AD  8B366414          mov si,[0x1464]
000096B1  2BF0              sub si,ax
000096B3  83E603            and si,0x3
000096B6  03F6              add si,si
000096B8  03F6              add si,si
000096BA  C4848615          les ax,word [si+0x1586]
000096BE  A36C14            mov [0x146c],ax
000096C1  8C066E14          mov word [0x146e],es
000096C5  BFE813            mov di,0x13e8
000096C8  E8EDF9            call 0x90b8
000096CB  8BF0              mov si,ax
000096CD  8A841001          mov al,[si+0x110]
000096D1  050200            add ax,0x2
000096D4  A37014            mov [0x1470],ax
000096D7  33DB              xor bx,bx
000096D9  8A9C2C01          mov bl,[si+0x12c]
000096DD  0BDB              or bx,bx
000096DF  744A              jz 0x972b
000096E1  8B3618A0          mov si,[0xa018]
000096E5  8B04              mov ax,[si]
000096E7  8AD0              mov dl,al
000096E9  8A4402            mov al,[si+0x2]
000096EC  B90800            mov cx,0x8
000096EF  2B0E0403          sub cx,[0x304]
000096F3  D3E8              shr ax,cl
000096F5  F7D9              neg cx
000096F7  83C110            add cx,0x10
000096FA  D3E2              shl dx,cl
000096FC  0BC2              or ax,dx
000096FE  8B3618A0          mov si,[0xa018]
00009702  8A24              mov ah,[si]
00009704  8A4401            mov al,[si+0x1]
00009707  8A0E0403          mov cl,[0x304]
0000970B  D3E0              shl ax,cl
0000970D  B110              mov cl,0x10
0000970F  2ACB              sub cl,bl
00009711  D3E8              shr ax,cl
00009713  01067014          add [0x1470],ax
00009717  011E0403          add [0x304],bx
0000971B  A10403            mov ax,[0x304]
0000971E  8326040307        and word [0x304],0x7
00009723  B103              mov cl,0x3
00009725  D3E8              shr ax,cl
00009727  010618A0          add [0xa018],ax
0000972B  A16E14            mov ax,[0x146e]
0000972E  3D0400            cmp ax,0x4
00009731  7317              jnc 0x974a
00009733  0BC0              or ax,ax
00009735  751E              jnz 0x9755
00009737  813E6C140020      cmp word [0x146c],0x2000
0000973D  7316              jnc 0x9755
0000973F  813E6C140101      cmp word [0x146c],0x101
00009745  730B              jnc 0x9752
00009747  E946FF            jmp 0x9690
0000974A  8306701403        add word [0x1470],0x3
0000974F  E93EFF            jmp 0x9690
00009752  E937FF            jmp 0x968c
00009755  E90EFF            jmp 0x9666
00009758  8BF0              mov si,ax
0000975A  33DB              xor bx,bx
0000975C  8A840001          mov al,[si+0x100]
00009760  40                inc ax
00009761  A36C14            mov [0x146c],ax
00009764  8A9C0801          mov bl,[si+0x108]
00009768  8B3618A0          mov si,[0xa018]
0000976C  8A24              mov ah,[si]
0000976E  8A4401            mov al,[si+0x1]
00009771  8A0E0403          mov cl,[0x304]
00009775  D3E0              shl ax,cl
00009777  B110              mov cl,0x10
00009779  C7066E140000      mov word [0x146e],0x0
0000977F  2ACB              sub cl,bl
00009781  D3E8              shr ax,cl
00009783  01066C14          add [0x146c],ax
00009787  011E0403          add [0x304],bx
0000978B  A10403            mov ax,[0x304]
0000978E  8326040307        and word [0x304],0x7
00009793  B103              mov cl,0x3
00009795  D3E8              shr ax,cl
00009797  010618A0          add [0xa018],ax
0000979B  C70670140200      mov word [0x1470],0x2
000097A1  E9ECFE            jmp 0x9690
000097A4  55                push bp
000097A5  8BEC              mov bp,sp
000097A7  8B4E04            mov cx,[bp+0x4]
000097AA  833E02A000        cmp word [0xa002],0x0
000097AF  750A              jnz 0x97bb
000097B1  3B0E00A0          cmp cx,[0xa000]
000097B5  7604              jna 0x97bb
000097B7  8B0E00A0          mov cx,[0xa000]
000097BB  8B1E10A0          mov bx,[0xa010]
000097BF  1E                push ds
000097C0  C55606            lds dx,word [bp+0x6]
000097C3  B43F              mov ah,0x3f
000097C5  CD21              int byte 0x21
000097C7  1F                pop ds
000097C8  0BC0              or ax,ax
000097CA  7418              jz 0x97e4
000097CC  01069A15          add [0x159a],ax
000097D0  83169C1500        adc word [0x159c],0x0
000097D5  290600A0          sub [0xa000],ax
000097D9  831E02A000        sbb word [0xa002],0x0
000097DE  1E                push ds
000097DF  52                push dx
000097E0  50                push ax
000097E1  E80600            call 0x97ea
000097E4  8BE5              mov sp,bp
000097E6  5D                pop bp
000097E7  C20600            ret word 0x6
000097EA  55                push bp
000097EB  8BEC              mov bp,sp
000097ED  803E22A014        cmp byte [0xa022],0x14
000097F2  721C              jc 0x9810
000097F4  B80000            mov ax,0x0
000097F7  3B4604            cmp ax,[bp+0x4]
000097FA  7311              jnc 0x980d
000097FC  C47E06            les di,word [bp+0x6]
000097FF  03F8              add di,ax
00009801  50                push ax
00009802  06                push es
00009803  57                push di
00009804  E8B101            call 0x99b8
00009807  58                pop ax
00009808  051000            add ax,0x10
0000980B  EBEA              jmp 0x97f7
0000980D  8B4604            mov ax,[bp+0x4]
00009810  8BE5              mov sp,bp
00009812  5D                pop bp
00009813  C20600            ret word 0x6
00009816  55                push bp
00009817  8BEC              mov bp,sp
00009819  8B4E04            mov cx,[bp+0x4]
0000981C  803E1CA000        cmp byte [0xa01c],0x0
00009821  7512              jnz 0x9835
00009823  8B1E12A0          mov bx,[0xa012]
00009827  1E                push ds
00009828  C55606            lds dx,word [bp+0x6]
0000982B  B440              mov ah,0x40
0000982D  CD21              int byte 0x21
0000982F  1F                pop ds
00009830  894604            mov [bp+0x4],ax
00009833  8BC8              mov cx,ax
00009835  29069E15          sub [0x159e],ax
00009839  831EA01500        sbb word [0x15a0],0x0
0000983E  C47E06            les di,word [bp+0x6]
00009841  E85104            call 0x9c95
00009844  8B4604            mov ax,[bp+0x4]
00009847  8BE5              mov sp,bp
00009849  5D                pop bp
0000984A  C20600            ret word 0x6
0000984D  55                push bp
0000984E  8BEC              mov bp,sp
00009850  83EC16            sub sp,0x16
00009853  C47E04            les di,word [bp+0x4]
00009856  268B05            mov ax,[es:di]
00009859  330624A0          xor ax,[0xa024]
0000985D  8946FC            mov [bp-0x4],ax
00009860  268B4502          mov ax,[es:di+0x2]
00009864  330626A0          xor ax,[0xa026]
00009868  8946FE            mov [bp-0x2],ax
0000986B  268B4504          mov ax,[es:di+0x4]
0000986F  330628A0          xor ax,[0xa028]
00009873  8946F8            mov [bp-0x8],ax
00009876  268B4506          mov ax,[es:di+0x6]
0000987A  33062AA0          xor ax,[0xa02a]
0000987E  8946FA            mov [bp-0x6],ax
00009881  268B4508          mov ax,[es:di+0x8]
00009885  33062CA0          xor ax,[0xa02c]
00009889  8946F4            mov [bp-0xc],ax
0000988C  268B450A          mov ax,[es:di+0xa]
00009890  33062EA0          xor ax,[0xa02e]
00009894  8946F6            mov [bp-0xa],ax
00009897  268B450C          mov ax,[es:di+0xc]
0000989B  330630A0          xor ax,[0xa030]
0000989F  8946F0            mov [bp-0x10],ax
000098A2  268B450E          mov ax,[es:di+0xe]
000098A6  330632A0          xor ax,[0xa032]
000098AA  8946F2            mov [bp-0xe],ax
000098AD  C746EE0000        mov word [bp-0x12],0x0
000098B2  E9C100            jmp 0x9976
000098B5  8B46F0            mov ax,[bp-0x10]
000098B8  8B56F2            mov dx,[bp-0xe]
000098BB  8BFA              mov di,dx
000098BD  B10B              mov cl,0xb
000098BF  8BD8              mov bx,ax
000098C1  D3E0              shl ax,cl
000098C3  D3E2              shl dx,cl
000098C5  B105              mov cl,0x5
000098C7  D3EB              shr bx,cl
000098C9  D3EF              shr di,cl
000098CB  0BD3              or dx,bx
000098CD  0BC7              or ax,di
000098CF  0346F4            add ax,[bp-0xc]
000098D2  1356F6            adc dx,[bp-0xa]
000098D5  8B76EE            mov si,[bp-0x12]
000098D8  83E603            and si,0x3
000098DB  03F6              add si,si
000098DD  03F6              add si,si
000098DF  338424A0          xor ax,[si-0x5fdc]
000098E3  339426A0          xor dx,[si-0x5fda]
000098E7  33DB              xor bx,bx
000098E9  8AD8              mov bl,al
000098EB  8A8F30DF          mov cl,[bx-0x20d0]
000098EF  8ADC              mov bl,ah
000098F1  8AAF30DF          mov ch,[bx-0x20d0]
000098F5  8ADA              mov bl,dl
000098F7  8A8730DF          mov al,[bx-0x20d0]
000098FB  8ADE              mov bl,dh
000098FD  8AA730DF          mov ah,[bx-0x20d0]
00009901  334EFC            xor cx,[bp-0x4]
00009904  3346FE            xor ax,[bp-0x2]
00009907  894EEA            mov [bp-0x16],cx
0000990A  8946EC            mov [bp-0x14],ax
0000990D  8B46F4            mov ax,[bp-0xc]
00009910  8B56F6            mov dx,[bp-0xa]
00009913  92                xchg ax,dx
00009914  D1E2              shl dx,0x0
00009916  D1D0              rcl ax,0x0
00009918  83D200            adc dx,0x0
0000991B  3346F0            xor ax,[bp-0x10]
0000991E  3356F2            xor dx,[bp-0xe]
00009921  038424A0          add ax,[si-0x5fdc]
00009925  139426A0          adc dx,[si-0x5fda]
00009929  33DB              xor bx,bx
0000992B  8AD8              mov bl,al
0000992D  8A8F30DF          mov cl,[bx-0x20d0]
00009931  8ADC              mov bl,ah
00009933  8AAF30DF          mov ch,[bx-0x20d0]
00009937  8ADA              mov bl,dl
00009939  8A8730DF          mov al,[bx-0x20d0]
0000993D  8ADE              mov bl,dh
0000993F  8AA730DF          mov ah,[bx-0x20d0]
00009943  334EF8            xor cx,[bp-0x8]
00009946  3346FA            xor ax,[bp-0x6]
00009949  8B56F4            mov dx,[bp-0xc]
0000994C  8956FC            mov [bp-0x4],dx
0000994F  8B56F6            mov dx,[bp-0xa]
00009952  8956FE            mov [bp-0x2],dx
00009955  8B56F0            mov dx,[bp-0x10]
00009958  8956F8            mov [bp-0x8],dx
0000995B  8B56F2            mov dx,[bp-0xe]
0000995E  8956FA            mov [bp-0x6],dx
00009961  8B56EA            mov dx,[bp-0x16]
00009964  8956F4            mov [bp-0xc],dx
00009967  8B56EC            mov dx,[bp-0x14]
0000996A  8956F6            mov [bp-0xa],dx
0000996D  894EF0            mov [bp-0x10],cx
00009970  8946F2            mov [bp-0xe],ax
00009973  FF46EE            inc word [bp-0x12]
00009976  837EEE20          cmp word [bp-0x12],0x20
0000997A  7303              jnc 0x997f
0000997C  E936FF            jmp 0x98b5
0000997F  C47E04            les di,word [bp+0x4]
00009982  06                push es
00009983  57                push di
00009984  BE24A0            mov si,0xa024
00009987  AD                lodsw
00009988  3346F4            xor ax,[bp-0xc]
0000998B  AB                stosw
0000998C  AD                lodsw
0000998D  3346F6            xor ax,[bp-0xa]
00009990  AB                stosw
00009991  AD                lodsw
00009992  3346F0            xor ax,[bp-0x10]
00009995  AB                stosw
00009996  AD                lodsw
00009997  3346F2            xor ax,[bp-0xe]
0000999A  AB                stosw
0000999B  AD                lodsw
0000999C  3346FC            xor ax,[bp-0x4]
0000999F  AB                stosw
000099A0  AD                lodsw
000099A1  3346FE            xor ax,[bp-0x2]
000099A4  AB                stosw
000099A5  AD                lodsw
000099A6  3346F8            xor ax,[bp-0x8]
000099A9  AB                stosw
000099AA  AD                lodsw
000099AB  3346FA            xor ax,[bp-0x6]
000099AE  AB                stosw
000099AF  E88501            call 0x9b37
000099B2  8BE5              mov sp,bp
000099B4  5D                pop bp
000099B5  C20400            ret word 0x4
000099B8  55                push bp
000099B9  8BEC              mov bp,sp
000099BB  83EC28            sub sp,0x28
000099BE  C47E04            les di,word [bp+0x4]
000099C1  268B05            mov ax,[es:di]
000099C4  8946D8            mov [bp-0x28],ax
000099C7  330624A0          xor ax,[0xa024]
000099CB  8946FC            mov [bp-0x4],ax
000099CE  268B4502          mov ax,[es:di+0x2]
000099D2  8946DA            mov [bp-0x26],ax
000099D5  330626A0          xor ax,[0xa026]
000099D9  8946FE            mov [bp-0x2],ax
000099DC  268B4504          mov ax,[es:di+0x4]
000099E0  8946DC            mov [bp-0x24],ax
000099E3  330628A0          xor ax,[0xa028]
000099E7  8946F8            mov [bp-0x8],ax
000099EA  268B4506          mov ax,[es:di+0x6]
000099EE  8946DE            mov [bp-0x22],ax
000099F1  33062AA0          xor ax,[0xa02a]
000099F5  8946FA            mov [bp-0x6],ax
000099F8  268B4508          mov ax,[es:di+0x8]
000099FC  8946E0            mov [bp-0x20],ax
000099FF  33062CA0          xor ax,[0xa02c]
00009A03  8946F4            mov [bp-0xc],ax
00009A06  268B450A          mov ax,[es:di+0xa]
00009A0A  8946E2            mov [bp-0x1e],ax
00009A0D  33062EA0          xor ax,[0xa02e]
00009A11  8946F6            mov [bp-0xa],ax
00009A14  268B450C          mov ax,[es:di+0xc]
00009A18  8946E4            mov [bp-0x1c],ax
00009A1B  330630A0          xor ax,[0xa030]
00009A1F  8946F0            mov [bp-0x10],ax
00009A22  268B450E          mov ax,[es:di+0xe]
00009A26  8946E6            mov [bp-0x1a],ax
00009A29  330632A0          xor ax,[0xa032]
00009A2D  8946F2            mov [bp-0xe],ax
00009A30  C746EE1F00        mov word [bp-0x12],0x1f
00009A35  8B46F0            mov ax,[bp-0x10]
00009A38  8B56F2            mov dx,[bp-0xe]
00009A3B  8BFA              mov di,dx
00009A3D  B10B              mov cl,0xb
00009A3F  8BD8              mov bx,ax
00009A41  D3E0              shl ax,cl
00009A43  D3E2              shl dx,cl
00009A45  B105              mov cl,0x5
00009A47  D3EB              shr bx,cl
00009A49  D3EF              shr di,cl
00009A4B  0BD3              or dx,bx
00009A4D  0BC7              or ax,di
00009A4F  0346F4            add ax,[bp-0xc]
00009A52  1356F6            adc dx,[bp-0xa]
00009A55  8B76EE            mov si,[bp-0x12]
00009A58  83E603            and si,0x3
00009A5B  03F6              add si,si
00009A5D  03F6              add si,si
00009A5F  338424A0          xor ax,[si-0x5fdc]
00009A63  339426A0          xor dx,[si-0x5fda]
00009A67  33DB              xor bx,bx
00009A69  8AD8              mov bl,al
00009A6B  8A8F30DF          mov cl,[bx-0x20d0]
00009A6F  8ADC              mov bl,ah
00009A71  8AAF30DF          mov ch,[bx-0x20d0]
00009A75  8ADA              mov bl,dl
00009A77  8A8730DF          mov al,[bx-0x20d0]
00009A7B  8ADE              mov bl,dh
00009A7D  8AA730DF          mov ah,[bx-0x20d0]
00009A81  334EFC            xor cx,[bp-0x4]
00009A84  3346FE            xor ax,[bp-0x2]
00009A87  894EEA            mov [bp-0x16],cx
00009A8A  8946EC            mov [bp-0x14],ax
00009A8D  8B46F4            mov ax,[bp-0xc]
00009A90  8B56F6            mov dx,[bp-0xa]
00009A93  92                xchg ax,dx
00009A94  D1E2              shl dx,0x0
00009A96  D1D0              rcl ax,0x0
00009A98  83D200            adc dx,0x0
00009A9B  3346F0            xor ax,[bp-0x10]
00009A9E  3356F2            xor dx,[bp-0xe]
00009AA1  038424A0          add ax,[si-0x5fdc]
00009AA5  139426A0          adc dx,[si-0x5fda]
00009AA9  33DB              xor bx,bx
00009AAB  8AD8              mov bl,al
00009AAD  8A8F30DF          mov cl,[bx-0x20d0]
00009AB1  8ADC              mov bl,ah
00009AB3  8AAF30DF          mov ch,[bx-0x20d0]
00009AB7  8ADA              mov bl,dl
00009AB9  8A8730DF          mov al,[bx-0x20d0]
00009ABD  8ADE              mov bl,dh
00009ABF  8AA730DF          mov ah,[bx-0x20d0]
00009AC3  334EF8            xor cx,[bp-0x8]
00009AC6  3346FA            xor ax,[bp-0x6]
00009AC9  8B56F4            mov dx,[bp-0xc]
00009ACC  8956FC            mov [bp-0x4],dx
00009ACF  8B56F6            mov dx,[bp-0xa]
00009AD2  8956FE            mov [bp-0x2],dx
00009AD5  8B56F0            mov dx,[bp-0x10]
00009AD8  8956F8            mov [bp-0x8],dx
00009ADB  8B56F2            mov dx,[bp-0xe]
00009ADE  8956FA            mov [bp-0x6],dx
00009AE1  8B56EA            mov dx,[bp-0x16]
00009AE4  8956F4            mov [bp-0xc],dx
00009AE7  8B56EC            mov dx,[bp-0x14]
00009AEA  8956F6            mov [bp-0xa],dx
00009AED  894EF0            mov [bp-0x10],cx
00009AF0  8946F2            mov [bp-0xe],ax
00009AF3  FF4EEE            dec word [bp-0x12]
00009AF6  7803              js 0x9afb
00009AF8  E93AFF            jmp 0x9a35
00009AFB  C47E04            les di,word [bp+0x4]
00009AFE  BE24A0            mov si,0xa024
00009B01  AD                lodsw
00009B02  3346F4            xor ax,[bp-0xc]
00009B05  AB                stosw
00009B06  AD                lodsw
00009B07  3346F6            xor ax,[bp-0xa]
00009B0A  AB                stosw
00009B0B  AD                lodsw
00009B0C  3346F0            xor ax,[bp-0x10]
00009B0F  AB                stosw
00009B10  AD                lodsw
00009B11  3346F2            xor ax,[bp-0xe]
00009B14  AB                stosw
00009B15  AD                lodsw
00009B16  3346FC            xor ax,[bp-0x4]
00009B19  AB                stosw
00009B1A  AD                lodsw
00009B1B  3346FE            xor ax,[bp-0x2]
00009B1E  AB                stosw
00009B1F  AD                lodsw
00009B20  3346F8            xor ax,[bp-0x8]
00009B23  AB                stosw
00009B24  AD                lodsw
00009B25  3346FA            xor ax,[bp-0x6]
00009B28  AB                stosw
00009B29  8D7ED8            lea di,[bp-0x28]
00009B2C  16                push ss
00009B2D  57                push di
00009B2E  E80600            call 0x9b37
00009B31  8BE5              mov sp,bp
00009B33  5D                pop bp
00009B34  C20400            ret word 0x4
00009B37  5B                pop bx
00009B38  5F                pop di
00009B39  07                pop es
00009B3A  53                push bx
00009B3B  33DB              xor bx,bx
00009B3D  EB72              jmp 0x9bb1
00009B3F  33C0              xor ax,ax
00009B41  268A01            mov al,[es:bx+di]
00009B44  8BF0              mov si,ax
00009B46  03F6              add si,si
00009B48  03F6              add si,si
00009B4A  8B84FD9B          mov ax,[si-0x6403]
00009B4E  310624A0          xor [0xa024],ax
00009B52  8B84FF9B          mov ax,[si-0x6401]
00009B56  310626A0          xor [0xa026],ax
00009B5A  33C0              xor ax,ax
00009B5C  268A4101          mov al,[es:bx+di+0x1]
00009B60  8BF0              mov si,ax
00009B62  03F6              add si,si
00009B64  03F6              add si,si
00009B66  8B84FD9B          mov ax,[si-0x6403]
00009B6A  310628A0          xor [0xa028],ax
00009B6E  8B84FF9B          mov ax,[si-0x6401]
00009B72  31062AA0          xor [0xa02a],ax
00009B76  33C0              xor ax,ax
00009B78  268A4102          mov al,[es:bx+di+0x2]
00009B7C  8BF0              mov si,ax
00009B7E  03F6              add si,si
00009B80  03F6              add si,si
00009B82  8B84FD9B          mov ax,[si-0x6403]
00009B86  31062CA0          xor [0xa02c],ax
00009B8A  8B84FF9B          mov ax,[si-0x6401]
00009B8E  31062EA0          xor [0xa02e],ax
00009B92  33C0              xor ax,ax
00009B94  268A4103          mov al,[es:bx+di+0x3]
00009B98  8BF0              mov si,ax
00009B9A  03F6              add si,si
00009B9C  03F6              add si,si
00009B9E  8B84FD9B          mov ax,[si-0x6403]
00009BA2  310630A0          xor [0xa030],ax
00009BA6  8B84FF9B          mov ax,[si-0x6401]
00009BAA  310632A0          xor [0xa032],ax
00009BAE  83C304            add bx,0x4
00009BB1  83FB10            cmp bx,0x10
00009BB4  7289              jc 0x9b3f
00009BB6  C3                ret
00009BB7  55                push bp
00009BB8  8BEC              mov bp,sp
00009BBA  81EC0E01          sub sp,0x10e
00009BBE  B879B8            mov ax,0xb879
00009BC1  A324A0            mov [0xa024],ax
00009BC4  B8A3D3            mov ax,0xd3a3
00009BC7  A326A0            mov [0xa026],ax
00009BCA  B8F712            mov ax,0x12f7
00009BCD  A328A0            mov [0xa028],ax
00009BD0  B86D3F            mov ax,0x3f6d
00009BD3  A32AA0            mov [0xa02a],ax
00009BD6  B835A2            mov ax,0xa235
00009BD9  A32CA0            mov [0xa02c],ax
00009BDC  B81575            mov ax,0x7515
00009BDF  A32EA0            mov [0xa02e],ax
00009BE2  B823F1            mov ax,0xf123
00009BE5  A330A0            mov [0xa030],ax
00009BE8  B8E7A4            mov ax,0xa4e7
00009BEB  A332A0            mov [0xa032],ax
00009BEE  16                push ss
00009BEF  07                pop es
00009BF0  8DBEF8FE          lea di,[bp-0x108]
00009BF4  AC                lodsb
00009BF5  98                cbw
00009BF6  8946FE            mov [bp-0x2],ax
00009BF9  8BC8              mov cx,ax
00009BFB  F3A4              rep movsb
00009BFD  B501              mov ch,0x1
00009BFF  2BC8              sub cx,ax
00009C01  33C0              xor ax,ax
00009C03  F3AA              rep stosb
00009C05  1E                push ds
00009C06  07                pop es
00009C07  BE0000            mov si,0x0
00009C0A  BF30DF            mov di,0xdf30
00009C0D  B180              mov cl,0x80
00009C0F  F3A5              rep movsw
00009C11  33DB              xor bx,bx
00009C13  C746FA0000        mov word [bp-0x6],0x0
00009C18  33FF              xor di,di
00009C1A  EB52              jmp 0x9c6e
00009C1C  33C0              xor ax,ax
00009C1E  8A83F9FE          mov al,[bp+di-0x107]
00009C22  0246FA            add al,[bp-0x6]
00009C25  8BF0              mov si,ax
00009C27  03F6              add si,si
00009C29  03F6              add si,si
00009C2B  8A84FD9B          mov al,[si-0x6403]
00009C2F  8846F8            mov [bp-0x8],al
00009C32  8A83F8FE          mov al,[bp+di-0x108]
00009C36  2A46FA            sub al,[bp-0x6]
00009C39  8BF0              mov si,ax
00009C3B  03F6              add si,si
00009C3D  03F6              add si,si
00009C3F  8A9CFD9B          mov bl,[si-0x6403]
00009C43  C746FC0100        mov word [bp-0x4],0x1
00009C48  EB1C              jmp 0x9c66
00009C4A  8A8730DF          mov al,[bx-0x20d0]
00009C4E  8BF3              mov si,bx
00009C50  03F7              add si,di
00009C52  0376FC            add si,[bp-0x4]
00009C55  81E6FF00          and si,0xff
00009C59  868430DF          xchg al,[si-0x20d0]
00009C5D  888730DF          mov [bx-0x20d0],al
00009C61  FEC3              inc bl
00009C63  FF46FC            inc word [bp-0x4]
00009C66  3A5EF8            cmp bl,[bp-0x8]
00009C69  75DF              jnz 0x9c4a
00009C6B  83C702            add di,0x2
00009C6E  3B7EFE            cmp di,[bp-0x2]
00009C71  72A9              jc 0x9c1c
00009C73  FE46FA            inc byte [bp-0x6]
00009C76  75A0              jnz 0x9c18
00009C78  33DB              xor bx,bx
00009C7A  3B5EFE            cmp bx,[bp-0x2]
00009C7D  7312              jnc 0x9c91
00009C7F  8DBEF8FE          lea di,[bp-0x108]
00009C83  03FB              add di,bx
00009C85  53                push bx
00009C86  16                push ss
00009C87  57                push di
00009C88  E8C2FB            call 0x984d
00009C8B  5B                pop bx
00009C8C  83C310            add bx,0x10
00009C8F  EBE9              jmp 0x9c7a
00009C91  8BE5              mov sp,bp
00009C93  5D                pop bp
00009C94  C3                ret
00009C95  A114A0            mov ax,[0xa014]
00009C98  8B1616A0          mov dx,[0xa016]
00009C9C  8BF7              mov si,di
00009C9E  BFFD9B            mov di,0x9bfd
00009CA1  268A1C            mov bl,[es:si]
00009CA4  46                inc si
00009CA5  E8B4D4            call 0x715c
00009CA8  E2F7              loop 0x9ca1
00009CAA  A314A0            mov [0xa014],ax
00009CAD  891616A0          mov [0xa016],dx
00009CB1  C3                ret
00009CB2  33C0              xor ax,ax
00009CB4  A36414            mov [0x1464],ax
00009CB7  1E                push ds
00009CB8  07                pop es
00009CB9  BF8615            mov di,0x1586
00009CBC  B90800            mov cx,0x8
00009CBF  F3AB              rep stosw
00009CC1  A30403            mov [0x304],ax
00009CC4  A39615            mov [0x1596],ax
00009CC7  39060EA0          cmp [0xa00e],ax
00009CCB  7524              jnz 0x9cf1
00009CCD  A3A415            mov [0x15a4],ax
00009CD0  A30C03            mov [0x30c],ax
00009CD3  A3A215            mov [0x15a2],ax
00009CD6  A36414            mov [0x1464],ax
00009CD9  A38215            mov [0x1582],ax
00009CDC  A38415            mov [0x1584],ax
00009CDF  C43E9615          les di,word [0x1596]
00009CE3  B90080            mov cx,0x8000
00009CE6  F3AB              rep stosw
00009CE8  1E                push ds
00009CE9  07                pop es
00009CEA  BF8615            mov di,0x1586
00009CED  B108              mov cl,0x8
00009CEF  F3AB              rep stosw
00009CF1  C3                ret
00009CF2  33C0              xor ax,ax
00009CF4  50                push ax
00009CF5  57                push di
00009CF6  8BDF              mov bx,di
00009CF8  89B9DE17          mov [bx+di+0x17de],di
00009CFC  4F                dec di
00009CFD  79F7              jns 0x9cf6
00009CFF  E80100            call 0x9d03
00009D02  C3                ret
00009D03  55                push bp
00009D04  8BEC              mov bp,sp
00009D06  8B7606            mov si,[bp+0x6]
00009D09  8B7E04            mov di,[bp+0x4]
00009D0C  8A95A615          mov dl,[di+0x15a6]
00009D10  3894A615          cmp [si+0x15a6],dl
00009D14  7E03              jng 0x9d19
00009D16  46                inc si
00009D17  EBF7              jmp 0x9d10
00009D19  3895A615          cmp [di+0x15a6],dl
00009D1D  7D03              jnl 0x9d22
00009D1F  4F                dec di
00009D20  EBF7              jmp 0x9d19
00009D22  3BF7              cmp si,di
00009D24  7F24              jg 0x9d4a
00009D26  8A84A615          mov al,[si+0x15a6]
00009D2A  8685A615          xchg al,[di+0x15a6]
00009D2E  8884A615          mov [si+0x15a6],al
00009D32  8BDE              mov bx,si
00009D34  8B80DE17          mov ax,[bx+si+0x17de]
00009D38  8BDF              mov bx,di
00009D3A  8781DE17          xchg ax,[bx+di+0x17de]
00009D3E  8BDE              mov bx,si
00009D40  8980DE17          mov [bx+si+0x17de],ax
00009D44  4F                dec di
00009D45  46                inc si
00009D46  3BF7              cmp si,di
00009D48  7CC6              jl 0x9d10
00009D4A  397E06            cmp [bp+0x6],di
00009D4D  7D39              jnl 0x9d88
00009D4F  4F                dec di
00009D50  397E06            cmp [bp+0x6],di
00009D53  7D0C              jnl 0x9d61
00009D55  47                inc di
00009D56  56                push si
00009D57  FF7606            push word [bp+0x6]
00009D5A  57                push di
00009D5B  E8A5FF            call 0x9d03
00009D5E  5E                pop si
00009D5F  EB27              jmp 0x9d88
00009D61  8B5E06            mov bx,[bp+0x6]
00009D64  8A87A615          mov al,[bx+0x15a6]
00009D68  3A85A715          cmp al,[di+0x15a7]
00009D6C  7D1A              jnl 0x9d88
00009D6E  8685A715          xchg al,[di+0x15a7]
00009D72  8887A615          mov [bx+0x15a6],al
00009D76  03DB              add bx,bx
00009D78  8B87DE17          mov ax,[bx+0x17de]
00009D7C  03FF              add di,di
00009D7E  8785E017          xchg ax,[di+0x17e0]
00009D82  D1EF              shr di,0x0
00009D84  8987DE17          mov [bx+0x17de],ax
00009D88  8B7E04            mov di,[bp+0x4]
00009D8B  3BFE              cmp di,si
00009D8D  7E31              jng 0x9dc0
00009D8F  4F                dec di
00009D90  3BF7              cmp si,di
00009D92  7D08              jnl 0x9d9c
00009D94  47                inc di
00009D95  56                push si
00009D96  57                push di
00009D97  E869FF            call 0x9d03
00009D9A  EB24              jmp 0x9dc0
00009D9C  8A84A615          mov al,[si+0x15a6]
00009DA0  3A85A715          cmp al,[di+0x15a7]
00009DA4  7D1A              jnl 0x9dc0
00009DA6  8685A715          xchg al,[di+0x15a7]
00009DAA  8884A615          mov [si+0x15a6],al
00009DAE  8BDE              mov bx,si
00009DB0  8B80DE17          mov ax,[bx+si+0x17de]
00009DB4  8BDF              mov bx,di
00009DB6  8781E017          xchg ax,[bx+di+0x17e0]
00009DBA  8BDE              mov bx,si
00009DBC  8980DE17          mov [bx+si+0x17de],ax
00009DC0  8BE5              mov sp,bp
00009DC2  5D                pop bp
00009DC3  C20400            ret word 0x4
00009DC6  83F910            cmp cx,0x10
00009DC9  720A              jc 0x9dd5
00009DCB  8BC2              mov ax,dx
00009DCD  33D2              xor dx,dx
00009DCF  83E910            sub cx,0x10
00009DD2  D3E8              shr ax,cl
00009DD4  C3                ret
00009DD5  8BDA              mov bx,dx
00009DD7  D3EA              shr dx,cl
00009DD9  D3E8              shr ax,cl
00009DDB  83E910            sub cx,0x10
00009DDE  F7D9              neg cx
00009DE0  D3E3              shl bx,cl
00009DE2  0BC3              or ax,bx
00009DE4  C3                ret
00009DE5  98                cbw
00009DE6  01060403          add [0x304],ax
00009DEA  A10403            mov ax,[0x304]
00009DED  832604031F        and word [0x304],0x1f
00009DF2  B105              mov cl,0x5
00009DF4  D3E8              shr ax,cl
00009DF6  03C0              add ax,ax
00009DF8  03C0              add ax,ax
00009DFA  010618A0          add [0xa018],ax
00009DFE  813E18A028DF      cmp word [0xa018],0xdf28
00009E04  751E              jnz 0x9e24
00009E06  8B3618A0          mov si,[0xa018]
00009E0A  BF40A0            mov di,0xa040
00009E0D  893E18A0          mov [0xa018],di
00009E11  1E                push ds
00009E12  07                pop es
00009E13  B9E83E            mov cx,0x3ee8
00009E16  A5                movsw
00009E17  A5                movsw
00009E18  A5                movsw
00009E19  A5                movsw
00009E1A  8B1E10A0          mov bx,[0xa010]
00009E1E  8BD7              mov dx,di
00009E20  B43F              mov ah,0x3f
00009E22  CD21              int byte 0x21
00009E24  8B3618A0          mov si,[0xa018]
00009E28  8B04              mov ax,[si]
00009E2A  8B5402            mov dx,[si+0x2]
00009E2D  8B0E0403          mov cx,[0x304]
00009E31  83F910            cmp cx,0x10
00009E34  720B              jc 0x9e41
00009E36  8BD0              mov dx,ax
00009E38  33C0              xor ax,ax
00009E3A  83E910            sub cx,0x10
00009E3D  D3E2              shl dx,cl
00009E3F  EB0F              jmp 0x9e50
00009E41  8BD8              mov bx,ax
00009E43  D3E0              shl ax,cl
00009E45  D3E2              shl dx,cl
00009E47  83E910            sub cx,0x10
00009E4A  F7D9              neg cx
00009E4C  D3EB              shr bx,cl
00009E4E  0BD3              or dx,bx
00009E50  A30803            mov [0x308],ax
00009E53  89160A03          mov [0x30a],dx
00009E57  8B4404            mov ax,[si+0x4]
00009E5A  8B5406            mov dx,[si+0x6]
00009E5D  B92000            mov cx,0x20
00009E60  2B0E0403          sub cx,[0x304]
00009E64  E85FFF            call 0x9dc6
00009E67  01060803          add [0x308],ax
00009E6B  11160A03          adc [0x30a],dx
00009E6F  C3                ret
00009E70  55                push bp
00009E71  8BEC              mov bp,sp
00009E73  83EC04            sub sp,0x4
00009E76  1E                push ds
00009E77  07                pop es
00009E78  8B4E08            mov cx,[bp+0x8]
00009E7B  41                inc cx
00009E7C  BFA615            mov di,0x15a6
00009E7F  8B7606            mov si,[bp+0x6]
00009E82  F3A4              rep movsb
00009E84  8B7E08            mov di,[bp+0x8]
00009E87  0BFF              or di,di
00009E89  7506              jnz 0x9e91
00009E8B  893EDE17          mov [0x17de],di
00009E8F  EB03              jmp 0x9e94
00009E91  E85EFE            call 0x9cf2
00009E94  8B5E08            mov bx,[bp+0x8]
00009E97  BEA615            mov si,0x15a6
00009E9A  C6400100          mov byte [bx+si+0x1],0x0
00009E9E  803C00            cmp byte [si],0x0
00009EA1  7403              jz 0x9ea6
00009EA3  46                inc si
00009EA4  EBF8              jmp 0x9e9e
00009EA6  81EEA615          sub si,0x15a6
00009EAA  83FE02            cmp si,0x2
00009EAD  7311              jnc 0x9ec0
00009EAF  8B1EDE17          mov bx,[0x17de]
00009EB3  03DB              add bx,bx
00009EB5  8B7E04            mov di,[bp+0x4]
00009EB8  C7010100          mov word [bx+di],0x1
00009EBC  0BF6              or si,si
00009EBE  7401              jz 0x9ec1
00009EC0  4E                dec si
00009EC1  B80100            mov ax,0x1
00009EC4  8B4E0A            mov cx,[bp+0xa]
00009EC7  D3E0              shl ax,cl
00009EC9  8946FC            mov [bp-0x4],ax
00009ECC  33D2              xor dx,dx
00009ECE  3B56FC            cmp dx,[bp-0x4]
00009ED1  733B              jnc 0x9f0e
00009ED3  8B4E0A            mov cx,[bp+0xa]
00009ED6  2A8CA615          sub cl,[si+0x15a6]
00009EDA  B80100            mov ax,0x1
00009EDD  D3E0              shl ax,cl
00009EDF  8946FE            mov [bp-0x2],ax
00009EE2  8BD8              mov bx,ax
00009EE4  03C2              add ax,dx
00009EE6  3B46FC            cmp ax,[bp-0x4]
00009EE9  7603              jna 0x9eee
00009EEB  F9                stc
00009EEC  EB21              jmp 0x9f0f
00009EEE  03F6              add si,si
00009EF0  8B84DE17          mov ax,[si+0x17de]
00009EF4  D1EE              shr si,0x0
00009EF6  03DB              add bx,bx
00009EF8  8B7E04            mov di,[bp+0x4]
00009EFB  03FA              add di,dx
00009EFD  03FA              add di,dx
00009EFF  83EB02            sub bx,0x2
00009F02  7804              js 0x9f08
00009F04  8901              mov [bx+di],ax
00009F06  EBF7              jmp 0x9eff
00009F08  0356FE            add dx,[bp-0x2]
00009F0B  4E                dec si
00009F0C  79C0              jns 0x9ece
00009F0E  F8                clc
00009F0F  8BE5              mov sp,bp
00009F11  5D                pop bp
00009F12  C20800            ret word 0x8
00009F15  55                push bp
00009F16  8BEC              mov bp,sp
00009F18  83EC08            sub sp,0x8
00009F1B  1E                push ds
00009F1C  07                pop es
00009F1D  8B7E06            mov di,[bp+0x6]
00009F20  8B4E04            mov cx,[bp+0x4]
00009F23  33C0              xor ax,ax
00009F25  F3AA              rep stosb
00009F27  8B7E08            mov di,[bp+0x8]
00009F2A  BB0100            mov bx,0x1
00009F2D  8B4E0A            mov cx,[bp+0xa]
00009F30  D3E3              shl bx,cl
00009F32  8BCB              mov cx,bx
00009F34  F3AB              rep stosw
00009F36  A10A03            mov ax,[0x30a]
00009F39  8BD0              mov dx,ax
00009F3B  250700            and ax,0x7
00009F3E  03C0              add ax,ax
00009F40  8946FA            mov [bp-0x6],ax
00009F43  B103              mov cl,0x3
00009F45  8BC2              mov ax,dx
00009F47  D3E8              shr ax,cl
00009F49  250F00            and ax,0xf
00009F4C  8946F8            mov [bp-0x8],ax
00009F4F  8BC2              mov ax,dx
00009F51  B107              mov cl,0x7
00009F53  D3E8              shr ax,cl
00009F55  3B4604            cmp ax,[bp+0x4]
00009F58  7603              jna 0x9f5d
00009F5A  8B4604            mov ax,[bp+0x4]
00009F5D  8946FC            mov [bp-0x4],ax
00009F60  A10803            mov ax,[0x308]
00009F63  03C0              add ax,ax
00009F65  8356FA00          adc word [bp-0x6],0x0
00009F69  B81100            mov ax,0x11
00009F6C  E877FE            call 0x9de6
00009F6F  33FF              xor di,di
00009F71  3B7EFA            cmp di,[bp-0x6]
00009F74  7716              ja 0x9f8c
00009F76  A10A03            mov ax,[0x30a]
00009F79  B10D              mov cl,0xd
00009F7B  D3E8              shr ax,cl
00009F7D  8885161A          mov [di+0x1a16],al
00009F81  B80300            mov ax,0x3
00009F84  57                push di
00009F85  E85EFE            call 0x9de6
00009F88  5F                pop di
00009F89  47                inc di
00009F8A  EBE5              jmp 0x9f71
00009F8C  B80700            mov ax,0x7
00009F8F  50                push ax
00009F90  FF76FA            push word [bp-0x6]
00009F93  BE161A            mov si,0x1a16
00009F96  56                push si
00009F97  FF7608            push word [bp+0x8]
00009F9A  E8D3FE            call 0x9e70
00009F9D  7303              jnc 0x9fa2
00009F9F  E9AE00            jmp 0xa050
00009FA2  33DB              xor bx,bx
00009FA4  895EFE            mov [bp-0x2],bx
00009FA7  3B5EFC            cmp bx,[bp-0x4]
00009FAA  7760              ja 0xa00c
00009FAC  8B7E08            mov di,[bp+0x8]
00009FAF  8B1E0A03          mov bx,[0x30a]
00009FB3  B109              mov cl,0x9
00009FB5  D3EB              shr bx,cl
00009FB7  03DB              add bx,bx
00009FB9  8B39              mov di,[bx+di]
00009FBB  3B7EFA            cmp di,[bp-0x6]
00009FBE  7319              jnc 0x9fd9
00009FC0  8B5EFE            mov bx,[bp-0x2]
00009FC3  FF46FE            inc word [bp-0x2]
00009FC6  8BC7              mov ax,di
00009FC8  8B7606            mov si,[bp+0x6]
00009FCB  8800              mov [bx+si],al
00009FCD  8A85161A          mov al,[di+0x1a16]
00009FD1  E811FE            call 0x9de5
00009FD4  8B5EFE            mov bx,[bp-0x2]
00009FD7  EBCE              jmp 0x9fa7
00009FD9  8A85161A          mov al,[di+0x1a16]
00009FDD  E805FE            call 0x9de5
00009FE0  A10A03            mov ax,[0x30a]
00009FE3  B10C              mov cl,0xc
00009FE5  D3E8              shr ax,cl
00009FE7  050400            add ax,0x4
00009FEA  50                push ax
00009FEB  B004              mov al,0x4
00009FED  E8F5FD            call 0x9de5
00009FF0  58                pop ax
00009FF1  8B5EFE            mov bx,[bp-0x2]
00009FF4  8B7606            mov si,[bp+0x6]
00009FF7  0BC0              or ax,ax
00009FF9  740C              jz 0xa007
00009FFB  3B5EFC            cmp bx,[bp-0x4]
00009FFE  770C              ja 0xa00c
0000A000  48                dec ax
0000A001  C60000            mov byte [bx+si],0x0
0000A004  43                inc bx
0000A005  EBF0              jmp 0x9ff7
0000A007  895EFE            mov [bp-0x2],bx
0000A00A  EB9B              jmp 0x9fa7
0000A00C  837EFA00          cmp word [bp-0x6],0x0
0000A010  741B              jz 0xa02d
0000A012  8B7606            mov si,[bp+0x6]
0000A015  BB0100            mov bx,0x1
0000A018  8B4EFA            mov cx,[bp-0x6]
0000A01B  3B5EFC            cmp bx,[bp-0x4]
0000A01E  770D              ja 0xa02d
0000A020  8A00              mov al,[bx+si]
0000A022  98                cbw
0000A023  0240FF            add al,[bx+si-0x1]
0000A026  F6F1              div cl
0000A028  8820              mov [bx+si],ah
0000A02A  43                inc bx
0000A02B  EBEE              jmp 0xa01b
0000A02D  33DB              xor bx,bx
0000A02F  8B46F8            mov ax,[bp-0x8]
0000A032  3B5EFC            cmp bx,[bp-0x4]
0000A035  770A              ja 0xa041
0000A037  803800            cmp byte [bx+si],0x0
0000A03A  7402              jz 0xa03e
0000A03C  0000              add [bx+si],al
0000A03E  43                inc bx
0000A03F  EBF1              jmp 0xa032
0000A041  FF760A            push word [bp+0xa]
0000A044  FF76FC            push word [bp-0x4]
0000A047  FF7606            push word [bp+0x6]
0000A04A  FF7608            push word [bp+0x8]
0000A04D  E820FE            call 0x9e70
0000A050  8BE5              mov sp,bp
0000A052  5D                pop bp
0000A053  C20800            ret word 0x8
0000A056  B80B00            mov ax,0xb
0000A059  50                push ax
0000A05A  B8261A            mov ax,0x1a26
0000A05D  50                push ax
0000A05E  B8262A            mov ax,0x2a26
0000A061  50                push ax
0000A062  B81A01            mov ax,0x11a
0000A065  50                push ax
0000A066  E8ACFE            call 0x9f15
0000A069  7224              jc 0xa08f
0000A06B  B80B00            mov ax,0xb
0000A06E  50                push ax
0000A06F  B8422B            mov ax,0x2b42
0000A072  50                push ax
0000A073  B8423B            mov ax,0x3b42
0000A076  50                push ax
0000A077  B8FF00            mov ax,0xff
0000A07A  50                push ax
0000A07B  E897FE            call 0x9f15
0000A07E  720F              jc 0xa08f
0000A080  A10A03            mov ax,[0x30a]
0000A083  D1E8              shr ax,0x0
0000A085  A3A415            mov [0x15a4],ax
0000A088  B80F00            mov ax,0xf
0000A08B  E858FD            call 0x9de6
0000A08E  F8                clc
0000A08F  C3                ret
0000A090  8E06B2B1          mov es,word [0xb1b2]
0000A094  A1BCBD            mov ax,[0xbdbc]
0000A097  06                push es
0000A098  1F                pop ds
0000A099  A31AA0            mov [0xa01a],ax
0000A09C  8CD8              mov ax,ds
0000A09E  050010            add ax,0x1000
0000A0A1  BF1003            mov di,0x310
0000A0A4  8B161EA0          mov dx,[0xa01e]
0000A0A8  803E0EA000        cmp byte [0xa00e],0x0
0000A0AD  7508              jnz 0xa0b7
0000A0AF  A39815            mov [0x1598],ax
0000A0B2  EB03              jmp 0xa0b7
0000A0B4  050010            add ax,0x1000
0000A0B7  AB                stosw
0000A0B8  4A                dec dx
0000A0B9  79F9              jns 0xa0b4
0000A0BB  803E20A000        cmp byte [0xa020],0x0
0000A0C0  757A              jnz 0xa13c
0000A0C2  E8EDFB            call 0x9cb2
0000A0C5  A106A0            mov ax,[0xa006]
0000A0C8  0B0604A0          or ax,[0xa004]
0000A0CC  746B              jz 0xa139
0000A0CE  E818F1            call 0x91e9
0000A0D1  833E06A000        cmp word [0xa006],0x0
0000A0D6  750A              jnz 0xa0e2
0000A0D8  8B0E04A0          mov cx,[0xa004]
0000A0DC  81F9F03E          cmp cx,0x3ef0
0000A0E0  7603              jna 0xa0e5
0000A0E2  B9F03E            mov cx,0x3ef0
0000A0E5  290E04A0          sub [0xa004],cx
0000A0E9  831E06A000        sbb word [0xa006],0x0
0000A0EE  A18415            mov ax,[0x1584]
0000A0F1  2B068215          sub ax,[0x1582]
0000A0F5  7409              jz 0xa100
0000A0F7  3BC1              cmp ax,cx
0000A0F9  7705              ja 0xa100
0000A0FB  51                push cx
0000A0FC  E82CF1            call 0x922b
0000A0FF  59                pop cx
0000A100  BE40A0            mov si,0xa040
0000A103  C43E9615          les di,word [0x1596]
0000A107  8B3E8215          mov di,[0x1582]
0000A10B  833E1AA000        cmp word [0xa01a],0x0
0000A110  741F              jz 0xa131
0000A112  8BC7              mov ax,di
0000A114  03C1              add ax,cx
0000A116  7319              jnc 0xa131
0000A118  8BD9              mov bx,cx
0000A11A  8BCF              mov cx,di
0000A11C  2BD9              sub bx,cx
0000A11E  F7D9              neg cx
0000A120  F3A4              rep movsb
0000A122  893E8215          mov [0x1582],di
0000A126  56                push si
0000A127  53                push bx
0000A128  E89DF1            call 0x92c8
0000A12B  59                pop cx
0000A12C  5E                pop si
0000A12D  C43E9615          les di,word [0x1596]
0000A131  010E8215          add [0x1582],cx
0000A135  F3A4              rep movsb
0000A137  EB8C              jmp 0xa0c5
0000A139  E9EFF0            jmp 0x922b
0000A13C  E873FB            call 0x9cb2
0000A13F  E8A7F0            call 0x91e9
0000A142  A140A0            mov ax,[0xa040]
0000A145  A30803            mov [0x308],ax
0000A148  A142A0            mov ax,[0xa042]
0000A14B  A30A03            mov [0x30a],ax
0000A14E  B80100            mov ax,0x1
0000A151  290604A0          sub [0xa004],ax
0000A155  831E06A000        sbb word [0xa006],0x0
0000A15A  7903              jns 0xa15f
0000A15C  E9CCF0            jmp 0x922b
0000A15F  A18415            mov ax,[0x1584]
0000A162  2B068215          sub ax,[0x1582]
0000A166  7408              jz 0xa170
0000A168  3D0301            cmp ax,0x103
0000A16B  7703              ja 0xa170
0000A16D  E8BBF0            call 0x922b
0000A170  833EA41500        cmp word [0x15a4],0x0
0000A175  7505              jnz 0xa17c
0000A177  E8DCFE            call 0xa056
0000A17A  72E0              jc 0xa15c
0000A17C  8B3E0A03          mov di,[0x30a]
0000A180  B105              mov cl,0x5
0000A182  D3EF              shr di,cl
0000A184  03FF              add di,di
0000A186  8BBD261A          mov di,[di+0x1a26]
0000A18A  8A85262A          mov al,[di+0x2a26]
0000A18E  57                push di
0000A18F  E853FC            call 0x9de5
0000A192  FF0EA415          dec word [0x15a4]
0000A196  58                pop ax
0000A197  3DFF00            cmp ax,0xff
0000A19A  771D              ja 0xa1b9
0000A19C  C43E9615          les di,word [0x1596]
0000A1A0  033E8215          add di,[0x1582]
0000A1A4  268805            mov [es:di],al
0000A1A7  FF068215          inc word [0x1582]
0000A1AB  750A              jnz 0xa1b7
0000A1AD  833E1AA000        cmp word [0xa01a],0x0
0000A1B2  7403              jz 0xa1b7
0000A1B4  E811F1            call 0x92c8
0000A1B7  EB95              jmp 0xa14e
0000A1B9  3D0301            cmp ax,0x103
0000A1BC  7769              ja 0xa227
0000A1BE  BB0200            mov bx,0x2
0000A1C1  2D0101            sub ax,0x101
0000A1C4  7E01              jng 0xa1c7
0000A1C6  43                inc bx
0000A1C7  40                inc ax
0000A1C8  8B3E6414          mov di,[0x1464]
0000A1CC  2BF8              sub di,ax
0000A1CE  83E703            and di,0x3
0000A1D1  03FF              add di,di
0000A1D3  03FF              add di,di
0000A1D5  8B958615          mov dx,[di+0x1586]
0000A1D9  89166C14          mov [0x146c],dx
0000A1DD  8B958815          mov dx,[di+0x1588]
0000A1E1  89166E14          mov [0x146e],dx
0000A1E5  8B3E6414          mov di,[0x1464]
0000A1E9  2BF8              sub di,ax
0000A1EB  83E703            and di,0x3
0000A1EE  03FF              add di,di
0000A1F0  03FF              add di,di
0000A1F2  8BF7              mov si,di
0000A1F4  83C604            add si,0x4
0000A1F7  83E60C            and si,0xc
0000A1FA  8B948615          mov dx,[si+0x1586]
0000A1FE  89958615          mov [di+0x1586],dx
0000A202  8B948815          mov dx,[si+0x1588]
0000A206  89958815          mov [di+0x1588],dx
0000A20A  48                dec ax
0000A20B  79D8              jns 0xa1e5
0000A20D  8B3E6414          mov di,[0x1464]
0000A211  03FF              add di,di
0000A213  03FF              add di,di
0000A215  8B166C14          mov dx,[0x146c]
0000A219  89958615          mov [di+0x1586],dx
0000A21D  8B166E14          mov dx,[0x146e]
0000A221  89958815          mov [di+0x1588],dx
0000A225  EB7E              jmp 0xa2a5
0000A227  2D0501            sub ax,0x105
0000A22A  770C              ja 0xa238
0000A22C  40                inc ax
0000A22D  A36C14            mov [0x146c],ax
0000A230  C7066E140000      mov word [0x146e],0x0
0000A236  EB37              jmp 0xa26f
0000A238  8BF0              mov si,ax
0000A23A  B92000            mov cx,0x20
0000A23D  2BC8              sub cx,ax
0000A23F  A10803            mov ax,[0x308]
0000A242  8B160A03          mov dx,[0x30a]
0000A246  E87DFB            call 0x9dc6
0000A249  8BCE              mov cx,si
0000A24B  BB0100            mov bx,0x1
0000A24E  83F910            cmp cx,0x10
0000A251  7309              jnc 0xa25c
0000A253  D3E3              shl bx,cl
0000A255  03C3              add ax,bx
0000A257  83D200            adc dx,0x0
0000A25A  EB07              jmp 0xa263
0000A25C  83E910            sub cx,0x10
0000A25F  D3E3              shl bx,cl
0000A261  03D3              add dx,bx
0000A263  A36C14            mov [0x146c],ax
0000A266  89166E14          mov [0x146e],dx
0000A26A  8BC6              mov ax,si
0000A26C  E877FB            call 0x9de6
0000A26F  8B3E6414          mov di,[0x1464]
0000A273  47                inc di
0000A274  83E703            and di,0x3
0000A277  893E6414          mov [0x1464],di
0000A27B  03FF              add di,di
0000A27D  03FF              add di,di
0000A27F  A16C14            mov ax,[0x146c]
0000A282  8B166E14          mov dx,[0x146e]
0000A286  89858615          mov [di+0x1586],ax
0000A28A  89958815          mov [di+0x1588],dx
0000A28E  BB0200            mov bx,0x2
0000A291  0BD2              or dx,dx
0000A293  7505              jnz 0xa29a
0000A295  3DFF1F            cmp ax,0x1fff
0000A298  7605              jna 0xa29f
0000A29A  83C302            add bx,0x2
0000A29D  EB06              jmp 0xa2a5
0000A29F  3DFF00            cmp ax,0xff
0000A2A2  7601              jna 0xa2a5
0000A2A4  43                inc bx
0000A2A5  8B3E0A03          mov di,[0x30a]
0000A2A9  B105              mov cl,0x5
0000A2AB  D3EF              shr di,cl
0000A2AD  03FF              add di,di
0000A2AF  8BBD422B          mov di,[di+0x2b42]
0000A2B3  8A85423B          mov al,[di+0x3b42]
0000A2B7  03FB              add di,bx
0000A2B9  893E7014          mov [0x1470],di
0000A2BD  E825FB            call 0x9de5
0000A2C0  83066C1401        add word [0x146c],0x1
0000A2C5  83166E1400        adc word [0x146e],0x0
0000A2CA  E8C7F0            call 0x9394
0000A2CD  893E8215          mov [0x1582],di
0000A2D1  A17014            mov ax,[0x1470]
0000A2D4  E97AFE            jmp 0xa151
0000A2D7  0030              add [bx+si],dh
0000A2D9  4F                dec di
0000A2DA  1E                push ds
0000A2DB  1F                pop ds
0000A2DC  1B30              sbb si,[bx+si]
0000A2DE  7071              jo 0xa351
0000A2E0  7070              jo 0xa352
0000A2E2  0F07              sysret
0000A2E4  0F70033F          pshufw mm0,[bp+di],0x3f
0000A2E8  3F                aas
0000A2E9  3F                aas
0000A2EA  034152            add ax,[bx+di+0x52]
0000A2ED  58                pop ax
0000A2EE  034C5A            add cx,[si+0x5a]
0000A2F1  48                dec ax
0000A2F2  035A49            add bx,[bp+si+0x49]
0000A2F5  50                push ax
0000A2F6  034152            add ax,[bx+di+0x52]
0000A2F9  43                inc bx
0000A2FA  035041            add dx,[bx+si+0x41]
0000A2FD  4B                dec bx
0000A2FE  034152            add ax,[bx+di+0x52]
0000A301  4A                dec dx
0000A302  03454C            add ax,[di+0x4c]
0000A305  49                dec cx
0000A306  034348            add ax,[bp+di+0x48]
0000A309  5A                pop dx
0000A30A  034859            add cx,[bx+si+0x59]
0000A30D  50                push ax
0000A30E  035A4F            add bx,[bp+si+0x4f]
0000A311  4F                dec di
0000A312  035146            add dx,[bx+di+0x46]
0000A315  43                inc bx
0000A316  034253            add ax,[bp+si+0x53]
0000A319  41                inc cx
0000A31A  034253            add ax,[bp+si+0x53]
0000A31D  4E                dec si
0000A31E  035351            add dx,[bp+di+0x51]
0000A321  5A                pop dx
0000A322  034C49            add cx,[si+0x49]
0000A325  4D                dec bp
0000A326  034850            add cx,[bx+si+0x50]
0000A329  4B                dec bx
0000A32A  0320              add sp,[bx+si]
0000A32C  48                dec ax
0000A32D  41                inc cx
0000A32E  0320              add sp,[bx+si]
0000A330  4C                dec sp
0000A331  47                inc di
0000A332  034143            add ax,[bx+di+0x43]
0000A335  45                inc bp
0000A336  035A45            add bx,[bp+si+0x45]
0000A339  54                push sp
0000A33A  03424F            add ax,[bp+si+0x4f]
0000A33D  41                inc cx
0000A33E  035041            add dx,[bx+si+0x41]
0000A341  48                dec ax
0000A342  034152            add ax,[bx+di+0x52]
0000A345  4B                dec bx
0000A346  035241            add dx,[bp+si+0x41]
0000A349  52                push dx
0000A34A  035241            add dx,[bp+si+0x41]
0000A34D  3200              xor al,[bx+si]
0000A34F  0000              add [bx+si],al
0000A351  0000              add [bx+si],al
0000A353  0000              add [bx+si],al
0000A355  0000              add [bx+si],al
0000A357  0000              add [bx+si],al
0000A359  0000              add [bx+si],al
0000A35B  0000              add [bx+si],al
0000A35D  0000              add [bx+si],al
0000A35F  0000              add [bx+si],al
0000A361  0000              add [bx+si],al
0000A363  0000              add [bx+si],al
0000A365  0000              add [bx+si],al
0000A367  0000              add [bx+si],al
0000A369  0000              add [bx+si],al
0000A36B  0000              add [bx+si],al
0000A36D  0000              add [bx+si],al
0000A36F  0000              add [bx+si],al
0000A371  0000              add [bx+si],al
0000A373  0000              add [bx+si],al
0000A375  0000              add [bx+si],al
0000A377  0000              add [bx+si],al
0000A379  0000              add [bx+si],al
0000A37B  0000              add [bx+si],al
0000A37D  0000              add [bx+si],al
0000A37F  0000              add [bx+si],al
0000A381  3030              xor [bx+si],dh
0000A383  3030              xor [bx+si],dh
0000A385  3030              xor [bx+si],dh
0000A387  3030              xor [bx+si],dh
0000A389  3030              xor [bx+si],dh
0000A38B  3020              xor [bx+si],ah
0000A38D  6561              gs popa
0000A38F  726F              jc 0xa400
0000A391  6974736E64        imul si,[si+0x73],0x646e
0000A396  750A              jnz 0xa3a2
0000A398  66686C67632E      push dword 0x2e63676c
0000A39E  2C79              sub al,0x79
0000A3A0  7770              ja 0xa412
0000A3A2  224776            and al,[bx+0x76]
0000A3A5  6B62306D          imul sp,[bp+si+0x30],0x6d
0000A3A9  54                push sp
0000A3AA  3650              ss push ax
0000A3AC  335559            xor dx,[di+0x59]
0000A3AF  3435              xor al,0x35
0000A3B1  2F                das
0000A3B2  37                aaa
0000A3B3  394632            cmp [bp+0x32],ax
0000A3B6  4C                dec sp
0000A3B7  4D                dec bp
0000A3B8  D4F5              aam byte 0xf5
0000A3BA  4A                dec dx
0000A3BB  0023              add [bp+di],ah
0000A3BD  10C3              adc bl,al
0000A3BF  110640A7          adc [0xa740],ax
0000A3C3  1E                push ds
0000A3C4  50                push ax
0000A3C5  E34E              jcxz 0xa415
0000A3C7  8E6028            mov fs,word [bx+si+0x28]
0000A3CA  90                nop
0000A3CB  95                xchg ax,bp
0000A3CC  7635              jna 0xa403
0000A3CE  ED                in ax,dx
0000A3CF  A6                cmpsb
0000A3D0  106E05            adc [bp+0x5],ch
0000A3D3  7BE8              jpo 0xa3bd
0000A3D5  34E2              xor al,0xe2
0000A3D7  32EF              xor ch,bh
0000A3D9  005C00            add [si+0x0],bl
0000A3DC  8400              test [bx+si],al
0000A3DE  C11005            rcl word [bx+si],byte 0x5
0000A3E1  700E              jo 0xa3f1
0000A3E3  3E023E2190        add bh,[ds:0x9021]
0000A3E8  02890056          add cl,[bx+di+0x5600]
0000A3EC  005708            add [bx+0x8],dl
0000A3EF  46                inc si
0000A3F0  BEF495            mov si,0x95f4
0000A3F3  C519              lds bx,word [bx+di]
0000A3F5  058028            add ax,0x2880
0000A3F8  E60E              out byte 0xe,al
0000A3FA  72E6              jc 0xa3e2
0000A3FC  E4BD              in al,byte 0xbd
0000A3FE  4F                dec di
0000A3FF  0E                push cs
0000A400  0570E8            add ax,0xe870
0000A403  349A              xor al,0x9a
0000A405  E360              jcxz 0xa467
0000A407  E027              loopne 0xa430
0000A409  0E                push cs
0000A40A  D118              rcr word [bx+si],0x0
0000A40C  031E1127          add bx,[0x2711]
0000A410  1902              sbb [bp+si],ax
0000A412  70E9              jo 0xa3fd
0000A414  7E02              jng 0xa418
0000A416  31E7              xor di,sp
0000A418  231E9E5B          and bx,[0x5b9e]
0000A41C  7400              jz 0xa41e
0000A41E  5C                pop sp
0000A41F  00E6              add dh,ah
0000A421  4A                dec dx
0000A422  00C5              add ch,al
0000A424  8900              mov [bx+si],ax
0000A426  56                push si
0000A427  00A71CAE          add [bx-0x51e4],ah
0000A42B  1002              adc [bp+si],al
0000A42D  890E7286          mov [0x8672],cx
0000A431  00640A            add [si+0xa],ah
0000A434  7105              jno 0xa43b
0000A436  6B31E2            imul si,[bx+di],0xffffffffffffffe2
0000A439  AE                scasb
0000A43A  123E1E6E          adc bh,[0x6e1e]
0000A43E  50                push ax
0000A43F  E8E112            call 0xb723
0000A442  7103              jno 0xa447
0000A444  1E                push ds
0000A445  257613            and ax,0x1376
0000A448  056E4F            add ax,0x4f6e
0000A44B  B430              mov ah,0x30
0000A44D  31E2              xor dx,sp
0000A44F  57                push di
0000A450  63265480          arpl [0x8054],sp
0000A454  C11B31            rcr word [bp+di],byte 0x31
0000A457  C1                db 0xc1
0000A458  30640E            xor [si+0xe],ah
0000A45B  9E                sahf
0000A45C  1E                push ds
0000A45D  22EB              and ch,bl
0000A45F  51                push cx
0000A460  E7E4              out byte 0xe4,ax
0000A462  94                xchg ax,sp
0000A463  E3E9              jcxz 0xa44e
0000A465  E40F              in al,byte 0xf
0000A467  0E                push cs
0000A468  0102              add [bp+si],ax
0000A46A  A6                cmpsb
0000A46B  E043              loopne 0xa4b0
0000A46D  00E3              add bl,ah
0000A46F  280ED103          sub [0x3d1],cl
0000A473  12E3              adc ah,bl
0000A475  E019              loopne 0xa490
0000A477  B260              mov dl,0x60
0000A479  C4                db 0xc4
0000A47A  E1E1              loope 0xa45d
0000A47C  4E                dec si
0000A47D  758E              jnz 0xa40d
0000A47F  2029              and [bx+di],ch
0000A481  3177BB            xor [bx-0x45],si
0000A484  FC                cld
0000A485  FA                cli
0000A486  F7F6              div si
0000A488  EE                out dx,al
0000A489  F9                stc
0000A48A  E50F              in ax,byte 0xf
0000A48C  4E                dec si
0000A48D  C3                ret
0000A48E  2581E5            and ax,0xe581
0000A491  0FEAEC            pminsw mm5,mm4
0000A494  2E3E01EB          ds add bx,bp
0000A498  4B                dec bx
0000A499  AE                scasb
0000A49A  1E                push ds
0000A49B  4E                dec si
0000A49C  AE                scasb
0000A49D  4F                dec di
0000A49E  21634E            and [bp+di+0x4e],sp
0000A4A1  B2E5              mov dl,0xe5
0000A4A3  0F                db 0x0f
0000A4A4  3EEF              ds out dx,ax
0000A4A6  8F                db 0x8f
0000A4A7  1F                pop ds
0000A4A8  1BEA              sbb bp,dx
0000A4AA  143E              adc al,0x3e
0000A4AC  210EAE4F          and [0x4fae],cx
0000A4B0  DE62E8            fisub word [bp+si-0x18]
0000A4B3  EC                in al,dx
0000A4B4  40                inc ax
0000A4B5  F6ABFBAB          imul byte [bp+di-0x5405]
0000A4B9  05AC13            add ax,0x13ac
0000A4BC  AC                lodsb
0000A4BD  19ACCEAA          sbb [si-0x5532],bp
0000A4C1  D8AAF1AA          fsubr dword [bp+si-0x550f]
0000A4C5  0AAB29AB          or ch,[bp+di-0x54d7]
0000A4C9  47                inc di
0000A4CA  AB                stosw
0000A4CB  60                pusha
0000A4CC  AB                stosw
0000A4CD  6F                outsw
0000A4CE  AF                scasw
0000A4CF  78AB              js 0xa47c
0000A4D1  80ABA0ABC0        sub byte [bp+di-0x5460],0xc0
0000A4D6  AB                stosw
0000A4D7  9BAD              wait lodsw
0000A4D9  6F                outsw
0000A4DA  AF                scasw
0000A4DB  BCADD2            mov sp,0xd2ad
0000A4DE  AD                lodsw
0000A4DF  7CAD              jl 0xa48e
0000A4E1  06                push es
0000A4E2  205669            and [bp+0x69],dl
0000A4E5  657720            gs ja 0xa508
0000A4E8  3220              xor ah,[bx+si]
0000A4EA  61                popa
0000A4EB  6C                insb
0000A4EC  7265              jc 0xa553
0000A4EE  61                popa
0000A4EF  647920            fs jns 0xa512
0000A4F2  657869            gs js 0xa55e
0000A4F5  7374              jnc 0xa56b
0000A4F7  732E              jnc 0xa527
0000A4F9  44                inc sp
0000A4FA  6F                outsw
0000A4FB  20796F            and [bx+di+0x6f],bh
0000A4FE  7520              jnz 0xa520
0000A500  7769              ja 0xa56b
0000A502  7368              jnc 0xa56c
0000A504  20746F            and [si+0x6f],dh
0000A507  207772            and [bx+0x72],dh
0000A50A  6974652069        imul si,[si+0x65],0x6920
0000A50F  7420              jz 0xa531
0000A511  6F                outsw
0000A512  7665              jna 0xa579
0000A514  7228              jc 0xa53e
0000A516  59                pop cx
0000A517  2F                das
0000A518  4E                dec si
0000A519  293F              sub [bx],di
0000A51B  104578            adc [di+0x78],al
0000A51E  7472              jz 0xa592
0000A520  61                popa
0000A521  637469            arpl [si+0x69],si
0000A524  6E                outsb
0000A525  67206669          and [esi+0x69],ah
0000A529  6C                insb
0000A52A  65200D            and [gs:di],cl
0000A52D  54                push sp
0000A52E  657374            gs jnc 0xa5a5
0000A531  696E672066        imul bp,[bp+0x67],0x6620
0000A536  696C652012        imul bp,[si+0x65],0x1220
0000A53B  207061            and [bx+si+0x61],dh
0000A53E  7468              jz 0xa5a8
0000A540  206372            and [bp+di+0x72],ah
0000A543  6561              gs popa
0000A545  7465              jz 0xa5ac
0000A547  206572            and [di+0x72],ah
0000A54A  726F              jc 0xa5bb
0000A54C  722C              jc 0xa57a
0000A54E  0D0A65            or ax,0x650a
0000A551  2D6D61            sub ax,0x616d
0000A554  696C3A2070        imul bp,[si+0x3a],0x7020
0000A559  6F                outsw
0000A55A  7374              jnc 0xa5d0
0000A55C  6D                insw
0000A55D  61                popa
0000A55E  7374              jnc 0xa5d4
0000A560  657240            gs jc 0xa5a3
0000A563  736B              jnc 0xa5d0
0000A565  692D6661          imul bp,[di],0x6166
0000A569  63746F            arpl [si+0x6f],si
0000A56C  7279              jc 0xa5e7
0000A56E  2E757A            {pn} jnz 0xa5eb
0000A571  68676F            push word 0x6f67
0000A574  726F              jc 0xa5e5
0000A576  642E7561          {pn} jnz 0xa5db
0000A57A  314C47            xor [si+0x47],cx
0000A57D  41                inc cx
0000A57E  56                push si
0000A57F  49                dec cx
0000A580  45                inc bp
0000A581  57                push di
0000A582  206361            and [bp+di+0x61],ah
0000A585  6E                outsb
0000A586  27                daa
0000A587  7420              jz 0xa5a9
0000A589  7265              jc 0xa5f0
0000A58B  61                popa
0000A58C  6420636F          and [fs:bp+di+0x6f],ah
0000A590  6E                outsb
0000A591  7465              jz 0xa5f8
0000A593  6E                outsb
0000A594  7420              jz 0xa5b6
0000A596  6F                outsw
0000A597  6620656E          o32 and [di+0x6e],ah
0000A59B  637279            arpl [bp+si+0x79],si
0000A59E  7074              jo 0xa614
0000A5A0  65642C6D          fs sub al,0x6d
0000A5A4  756C              jnz 0xa612
0000A5A6  7469              jz 0xa611
0000A5A8  7061              jo 0xa60b
0000A5AA  7274              jc 0xa620
0000A5AC  17                pop ss
0000A5AD  6F                outsw
0000A5AE  7220              jc 0xa5d0
0000A5B0  7365              jnc 0xa617
0000A5B2  637572            arpl [di+0x72],si
0000A5B5  6564204850        and [fs:bx+si+0x50],cl
0000A5BA  4B                dec bx
0000A5BB  2D6172            sub ax,0x7261
0000A5BE  636869            arpl [bx+si+0x69],bp
0000A5C1  7665              jna 0xa628
0000A5C3  730E              jnc 0xa5d3
0000A5C5  204261            and [bp+si+0x61],al
0000A5C8  64207061          and [fs:bx+si+0x61],dh
0000A5CC  7373              jnc 0xa641
0000A5CE  776F              ja 0xa63f
0000A5D0  7264              jc 0xa636
0000A5D2  200B              and [bp+di],cl
0000A5D4  204352            and [bp+di+0x52],al
0000A5D7  43                inc bx
0000A5D8  206572            and [di+0x72],ah
0000A5DB  726F              jc 0xa64c
0000A5DD  7220              jc 0xa5ff
0000A5DF  0820              or [bx+si],ah
0000A5E1  43                inc bx
0000A5E2  52                push dx
0000A5E3  43                inc bx
0000A5E4  204F4B            and [bx+0x4b],cl
0000A5E7  200C              and [si],cl
0000A5E9  204361            and [bp+di+0x61],al
0000A5EC  6E                outsb
0000A5ED  27                daa
0000A5EE  7420              jz 0xa610
0000A5F0  7265              jc 0xa657
0000A5F2  61                popa
0000A5F3  64200D            and [fs:di],cl
0000A5F6  204361            and [bp+di+0x61],al
0000A5F9  6E                outsb
0000A5FA  27                daa
0000A5FB  7420              jz 0xa61d
0000A5FD  7772              ja 0xa671
0000A5FF  6974652010        imul si,[si+0x65],0x1020
0000A604  204461            and [si+0x61],al
0000A607  6D                insw
0000A608  61                popa
0000A609  676520696E        and [gs:ecx+0x6e],ch
0000A60E  206669            and [bp+0x69],ah
0000A611  6C                insb
0000A612  652023            and [gs:bp+di],ah
0000A615  50                push ax
0000A616  6C                insb
0000A617  6561              gs popa
0000A619  7365              jnc 0xa680
0000A61B  206465            and [si+0x65],ah
0000A61E  66696E6520656E76  imul ebp,[bp+0x65],0x766e6520
0000A626  69726F6E6D        imul si,[bp+si+0x6f],0x6d6e
0000A62B  656E              gs outsb
0000A62D  7420              jz 0xa64f
0000A62F  6E                outsb
0000A630  61                popa
0000A631  6D                insw
0000A632  65205445          and [gs:si+0x45],dl
0000A636  4D                dec bp
0000A637  50                push ax
0000A638  0B20              or sp,[bx+si]
0000A63A  66696C6573206672  imul ebp,[si+0x65],0x72662073
0000A642  6F                outsw
0000A643  6D                insw
0000A644  0420              add al,0x20
0000A646  6F                outsw
0000A647  6620064361        o32 and [0x6143],al
0000A64C  6E                outsb
0000A64D  63656C            arpl [di+0x6c],sp
0000A650  0A4361            or al,[bp+di+0x61]
0000A653  6E                outsb
0000A654  27                daa
0000A655  7420              jz 0xa677
0000A657  7669              jna 0xa6c2
0000A659  65770D            gs ja 0xa669
0000A65C  43                inc bx
0000A65D  61                popa
0000A65E  6E                outsb
0000A65F  27                daa
0000A660  7420              jz 0xa682
0000A662  657874            gs js 0xa6d9
0000A665  7261              jc 0xa6c8
0000A667  63740A            arpl [si+0xa],si
0000A66A  43                inc bx
0000A66B  61                popa
0000A66C  6E                outsb
0000A66D  27                daa
0000A66E  7420              jz 0xa690
0000A670  7465              jz 0xa6d7
0000A672  7374              jnc 0xa6e8
0000A674  196469            sbb [si+0x69],sp
0000A677  7265              jc 0xa6de
0000A679  63746F            arpl [si+0x6f],si
0000A67C  7279              jc 0xa6f7
0000A67E  206F72            and [bx+0x72],ch
0000A681  20766F            and [bp+0x6f],dh
0000A684  6C                insb
0000A685  756D              jnz 0xa6f4
0000A687  65206C61          and [gs:si+0x61],ch
0000A68B  62656C            bound sp,[di+0x6c]
0000A68E  2F                das
0000A68F  6E                outsb
0000A690  6F                outsw
0000A691  7420              jz 0xa6b3
0000A693  666F              outsd
0000A695  756E              jnz 0xa705
0000A697  64206F6E          and [fs:bx+0x6e],ch
0000A69B  207061            and [bx+si+0x61],dh
0000A69E  7468              jz 0xa708
0000A6A0  2E204D6F          and [cs:di+0x6f],cl
0000A6A4  64696679204C      imul sp,[fs:bp+0x79],0x4c20
0000A6AA  47                inc di
0000A6AB  41                inc cx
0000A6AC  56                push si
0000A6AD  49                dec cx
0000A6AE  45                inc bp
0000A6AF  57                push di
0000A6B0  2E43              cs inc bx
0000A6B2  46                inc si
0000A6B3  47                inc di
0000A6B4  20706C            and [bx+si+0x6c],dh
0000A6B7  6561              gs popa
0000A6B9  7365              jnc 0xa720
0000A6BB  2E2E2E2B4665      sub ax,[cs:bp+0x65]
0000A6C1  61                popa
0000A6C2  7475              jz 0xa739
0000A6C4  7265              jc 0xa72b
0000A6C6  206973            and [bx+di+0x73],ch
0000A6C9  206E6F            and [bp+0x6f],ch
0000A6CC  7420              jz 0xa6ee
0000A6CE  696D706C65        imul bp,[di+0x70],0x656c
0000A6D3  6D                insw
0000A6D4  656E              gs outsb
0000A6D6  7465              jz 0xa73d
0000A6D8  6420666F          and [fs:bp+0x6f],ah
0000A6DC  7220              jc 0xa6fe
0000A6DE  7468              jz 0xa748
0000A6E0  6973206172        imul si,[bp+di+0x20],0x7261
0000A6E5  637479            arpl [si+0x79],si
0000A6E8  7065              jo 0xa74f
0000A6EA  185761            sbb [bx+0x61],dl
0000A6ED  697420666F        imul si,[si+0x20],0x6f66
0000A6F2  7220              jc 0xa714
0000A6F4  667574            jnz 0xa76b
0000A6F7  7572              jnz 0xa76b
0000A6F9  65207265          and [gs:bp+si+0x65],dh
0000A6FD  6C                insb
0000A6FE  6561              gs popa
0000A700  7365              jnc 0xa767
0000A702  7316              jnc 0xa71a
0000A704  4E                dec si
0000A705  6F                outsw
0000A706  7420              jz 0xa728
0000A708  656E              gs outsb
0000A70A  6F                outsw
0000A70B  7567              jnz 0xa774
0000A70D  68206D            push word 0x6d20
0000A710  656D              gs insw
0000A712  6F                outsw
0000A713  7279              jc 0xa78e
0000A715  20666F            and [bp+0x6f],ah
0000A718  7220              jc 0xa73a
0000A71A  0F70726F67        pshufw mm6,[bp+si+0x6f],0x67
0000A71F  7261              jc 0xa782
0000A721  6D                insw
0000A722  207275            and [bp+si+0x75],dh
0000A725  6E                outsb
0000A726  6E                outsb
0000A727  696E670E50        imul bp,[bp+0x67],0x500e
0000A72C  7265              jc 0xa793
0000A72E  7373              jnc 0xa7a3
0000A730  20616E            and [bx+di+0x6e],ah
0000A733  7920              jns 0xa755
0000A735  6B657920          imul sp,[di+0x79],0x20
0000A739  06                push es
0000A73A  54                push sp
0000A73B  6D                insw
0000A73C  7044              jo 0xa782
0000A73E  4F                dec di
0000A73F  53                push bx
0000A740  06                push es
0000A741  51                push cx
0000A742  7569              jnz 0xa7ad
0000A744  7420              jz 0xa766
0000A746  2007              and [bx],al
0000A748  204572            and [di+0x72],al
0000A74B  726F              jc 0xa7bc
0000A74D  7220              jc 0xa76f
0000A74F  07                pop es
0000A750  746F              jz 0xa7c1
0000A752  206578            and [di+0x78],ah
0000A755  6974084172        imul si,[si+0x8],0x7241
0000A75A  636869            arpl [bx+si+0x69],bp
0000A75D  7665              jna 0xa7c4
0000A75F  3A05              cmp al,[di]
0000A761  46                inc si
0000A762  696C652009        imul bp,[si+0x65],0x920
0000A767  4F                dec di
0000A768  7269              jc 0xa7d3
0000A76A  67696E616C20      imul bp,[esi+0x61],0x206c
0000A770  044E              add al,0x4e
0000A772  61                popa
0000A773  6D                insw
0000A774  650444            gs add al,0x44
0000A777  61                popa
0000A778  7465              jz 0xa7df
0000A77A  084F72            or [bx+0x72],cl
0000A77D  696753697A        imul sp,[bx+0x53],0x7a69
0000A782  650454            gs add al,0x54
0000A785  696D650A20        imul bp,[di+0x65],0x200a
0000A78A  627974            bound di,[bx+di+0x74]
0000A78D  657320            gs jnc 0xa7b0
0000A790  696E200820        imul bp,[bp+0x20],0x2008
0000A795  66696C6528732924  imul ebp,[si+0x65],0x24297328
0000A79D  2D7369            sub ax,0x6973
0000A7A0  7A65              jpe 0xa807
0000A7A2  205261            and [bp+si+0x61],dl
0000A7A5  7469              jz 0xa810
0000A7A7  6F                outsw
0000A7A8  2020              and [bx+si],ah
0000A7AA  44                inc sp
0000A7AB  61                popa
0000A7AC  7465              jz 0xa813
0000A7AE  2020              and [bx+si],ah
0000A7B0  205469            and [si+0x69],dl
0000A7B3  6D                insw
0000A7B4  652020            and [gs:bx+si],ah
0000A7B7  43                inc bx
0000A7B8  50                push ax
0000A7B9  41                inc cx
0000A7BA  58                pop ax
0000A7BB  45                inc bp
0000A7BC  204E61            and [bp+0x61],cl
0000A7BF  6D                insw
0000A7C0  650F616C6C        punpcklwd mm5,[gs:si+0x6c]
0000A7C5  206669            and [bp+0x69],ah
0000A7C8  6C                insb
0000A7C9  65206E61          and [gs:bp+0x61],ch
0000A7CD  6D                insw
0000A7CE  65732E            gs jnc 0xa7ff
0000A7D1  0B746F            or si,[si+0x6f]
0000A7D4  20636F            and [bp+di+0x6f],ah
0000A7D7  6E                outsb
0000A7D8  7469              jz 0xa843
0000A7DA  6E                outsb
0000A7DB  7565              jnz 0xa842
0000A7DD  1120              adc [bx+si],sp
0000A7DF  45                inc bp
0000A7E0  6E                outsb
0000A7E1  7465              jz 0xa848
0000A7E3  7220              jc 0xa805
0000A7E5  7061              jo 0xa848
0000A7E7  7373              jnc 0xa85c
0000A7E9  776F              ja 0xa85a
0000A7EB  7264              jc 0xa851
0000A7ED  3A20              cmp ah,[bx+si]
0000A7EF  0E                push cs
0000A7F0  57                push di
0000A7F1  61                popa
0000A7F2  697420706C        imul si,[si+0x20],0x6c70
0000A7F7  6561              gs popa
0000A7F9  7365              jnc 0xa860
0000A7FB  2E2E2E0D4E6F      cs or ax,0x6f4e
0000A801  206672            and [bp+0x72],ah
0000A804  6565207370        and [gs:bp+di+0x70],dh
0000A809  61                popa
0000A80A  636512            arpl [di+0x12],sp
0000A80D  666F              outsd
0000A80F  7220              jc 0xa831
0000A811  7465              jz 0xa878
0000A813  6D                insw
0000A814  706F              jo 0xa885
0000A816  7261              jc 0xa879
0000A818  7279              jc 0xa893
0000A81A  206669            and [bp+0x69],ah
0000A81D  6C                insb
0000A81E  650D4361          gs or ax,0x6143
0000A822  6E                outsb
0000A823  27                daa
0000A824  7420              jz 0xa846
0000A826  657874            gs js 0xa89d
0000A829  7261              jc 0xa88c
0000A82B  637429            arpl [si+0x29],si
0000A82E  4E                dec si
0000A82F  6F                outsw
0000A830  7420              jz 0xa852
0000A832  656E              gs outsb
0000A834  6F                outsw
0000A835  7567              jnz 0xa89e
0000A837  68206D            push word 0x6d20
0000A83A  656D              gs insw
0000A83C  6F                outsw
0000A83D  7279              jc 0xa8b8
0000A83F  206F72            and [bx+0x72],ch
0000A842  207061            and [bx+si+0x61],dh
0000A845  7373              jnc 0xa8ba
0000A847  776F              ja 0xa8b8
0000A849  7264              jc 0xa8af
0000A84B  206E6F            and [bp+0x6f],ch
0000A84E  7420              jz 0xa870
0000A850  636F72            arpl [bx+0x72],bp
0000A853  7265              jc 0xa8ba
0000A855  637414            arpl [si+0x14],si
0000A858  20456E            and [di+0x6e],al
0000A85B  7465              jz 0xa8c2
0000A85D  7220              jc 0xa87f
0000A85F  657874            gs js 0xa8d6
0000A862  7261              jc 0xa8c5
0000A864  637420            arpl [si+0x20],si
0000A867  7061              jo 0xa8ca
0000A869  7468              jz 0xa8d3
0000A86B  20162045          and [0x4520],dl
0000A86F  6E                outsb
0000A870  7465              jz 0xa8d7
0000A872  7220              jc 0xa894
0000A874  7465              jz 0xa8db
0000A876  6D                insw
0000A877  706F              jo 0xa8e8
0000A879  7261              jc 0xa8dc
0000A87B  7279              jc 0xa8f6
0000A87D  207061            and [bx+si+0x61],dh
0000A880  7468              jz 0xa8ea
0000A882  201D              and [di],bl
0000A884  3C45              cmp al,0x45
0000A886  6E                outsb
0000A887  7465              jz 0xa8ee
0000A889  723E              jc 0xa8c9
0000A88B  20666F            and [bp+0x6f],ah
0000A88E  7220              jc 0xa8b0
0000A890  637572            arpl [di+0x72],si
0000A893  7265              jc 0xa8fa
0000A895  6E                outsb
0000A896  7420              jz 0xa8b8
0000A898  646972656374      imul si,[fs:bp+si+0x65],0x7463
0000A89E  6F                outsw
0000A89F  7279              jc 0xa91a
0000A8A1  0C49              or al,0x49
0000A8A3  6E                outsb
0000A8A4  7661              jna 0xa907
0000A8A6  6C                insb
0000A8A7  6964207061        imul sp,[si+0x20],0x6170
0000A8AC  7468              jz 0xa916
0000A8AE  2420              and al,0x20
0000A8B0  4C                dec sp
0000A8B1  7961              jns 0xa914
0000A8B3  706B              jo 0xa920
0000A8B5  6F                outsw
0000A8B6  204765            and [bx+0x65],al
0000A8B9  6F                outsw
0000A8BA  7267              jc 0xa923
0000A8BC  65204172          and [gs:bx+di+0x72],al
0000A8C0  636869            arpl [bx+si+0x69],bp
0000A8C3  7665              jna 0xa92a
0000A8C5  205669            and [bp+0x69],dl
0000A8C8  657765            gs ja 0xa930
0000A8CB  7220              jc 0xa8ed
0000A8CD  7636              jna 0xa905
0000A8CF  2E3039            xor [cs:bx+di],bh
0000A8D2  201C              and [si],bl
0000A8D4  55                push bp
0000A8D5  7361              jnc 0xa938
0000A8D7  67653A20          cmp ah,[gs:eax]
0000A8DB  4C                dec sp
0000A8DC  47                inc di
0000A8DD  41                inc cx
0000A8DE  56                push si
0000A8DF  49                dec cx
0000A8E0  45                inc bp
0000A8E1  57                push di
0000A8E2  2020              and [bx+si],ah
0000A8E4  66696C656E616D65  imul ebp,[si+0x65],0x656d616e
0000A8EC  2E657874          gs js 0xa964
0000A8F0  0E                push cs
0000A8F1  46                inc si
0000A8F2  696C65206E        imul bp,[si+0x65],0x6e20
0000A8F7  6F                outsw
0000A8F8  7420              jz 0xa91a
0000A8FA  666F              outsd
0000A8FC  756E              jnz 0xa96c
0000A8FE  64085265          or [fs:bp+si+0x65],dl
0000A902  61                popa
0000A903  64696E672010      imul bp,[fs:bp+0x67],0x1020
0000A909  66756C            jnz 0xa978
0000A90C  6C                insb
0000A90D  207061            and [bx+si+0x61],dh
0000A910  7468              jz 0xa97a
0000A912  206E61            and [bp+0x61],ch
0000A915  6D                insw
0000A916  65732E            gs jnc 0xa947
0000A919  095365            or [bp+di+0x65],dx
0000A91C  6C                insb
0000A91D  65637465          arpl [gs:si+0x65],si
0000A921  643A04            cmp al,[fs:si]
0000A924  50                push ax
0000A925  61                popa
0000A926  7373              jnc 0xa99b
0000A928  0450              add al,0x50
0000A92A  61                popa
0000A92B  7468              jz 0xa995
0000A92D  0820              or [bx+si],ah
0000A92F  53                push bx
0000A930  6561              gs popa
0000A932  7263              jc 0xa997
0000A934  68201A            push word 0x1a20
0000A937  20456E            and [di+0x6e],al
0000A93A  7465              jz 0xa9a1
0000A93C  7220              jc 0xa95e
0000A93E  6D                insw
0000A93F  61                popa
0000A940  736B              jnc 0xa9ad
0000A942  20746F            and [si+0x6f],dh
0000A945  207365            and [bp+di+0x65],dh
0000A948  61                popa
0000A949  7263              jc 0xa9ae
0000A94B  682066            push word 0x6620
0000A94E  6F                outsw
0000A94F  7220              jc 0xa971
0000A951  0A20              or ah,[bx+si]
0000A953  55                push bp
0000A954  6E                outsb
0000A955  7365              jnc 0xa9bc
0000A957  6C                insb
0000A958  65637420          arpl [gs:si+0x20],si
0000A95C  0820              or [bx+si],ah
0000A95E  53                push bx
0000A95F  656C              gs insb
0000A961  65637420          arpl [gs:si+0x20],si
0000A965  0C20              or al,0x20
0000A967  44                inc sp
0000A968  4F                dec di
0000A969  53                push bx
0000A96A  207769            and [bx+0x69],dh
0000A96D  6E                outsb
0000A96E  646F              fs outsw
0000A970  7720              ja 0xa992
0000A972  1820              sbb [bx+si],ah
0000A974  52                push dx
0000A975  65676973746572    imul si,[gs:ebx+0x74],0x7265
0000A97C  207468            and [si+0x68],dh
0000A97F  6973207072        imul si,[bp+di+0x20],0x7270
0000A984  6F                outsw
0000A985  677261            a32 jc 0xa9e9
0000A988  6D                insw
0000A989  2120              and [bx+si],sp
0000A98B  1E                push ds
0000A98C  4D                dec bp
0000A98D  61                popa
0000A98E  7920              jns 0xa9b0
0000A990  3139              xor [bx+di],di
0000A992  3938              cmp [bx+si],di
0000A994  2028              and [bx+si],ch
0000A996  6329              arpl [bx+di],bp
0000A998  206279            and [bp+si+0x79],ah
0000A99B  204765            and [bx+0x65],al
0000A99E  6F                outsw
0000A99F  7267              jc 0xaa08
0000A9A1  65204C79          and [gs:si+0x79],cl
0000A9A5  61                popa
0000A9A6  706B              jo 0xaa13
0000A9A8  6F                outsw
0000A9A9  280D              sub [di],cl
0000A9AB  756E              jnz 0xaa1b
0000A9AD  7265              jc 0xaa14
0000A9AF  676973746572      imul si,[ebx+0x74],0x7265
0000A9B5  656429062053      sub [fs:0x5320],ax
0000A9BB  65722E            gs jc 0xa9ec
0000A9BE  4E                dec si
0000A9BF  0E                push cs
0000A9C0  52                push dx
0000A9C1  65676973746572    imul si,[gs:ebx+0x74],0x7265
0000A9C8  656420746F        and [fs:si+0x6f],dh
0000A9CD  2009              and [bx+di],cl
0000A9CF  4D                dec bp
0000A9D0  4F                dec di
0000A9D1  56                push si
0000A9D2  45                inc bp
0000A9D3  4D                dec bp
0000A9D4  45                inc bp
0000A9D5  4E                dec si
0000A9D6  54                push sp
0000A9D7  3A18              cmp bl,[bx+si]
0000A9D9  182C              sbb [si],ch
0000A9DB  1920              sbb [bx+si],sp
0000A9DD  2020              and [bx+si],ah
0000A9DF  2020              and [bx+si],ah
0000A9E1  2020              and [bx+si],ah
0000A9E3  2D204C            sub ax,0x4c20
0000A9E6  696E652055        imul bp,[bp+0x65],0x5520
0000A9EB  702F              jo 0xaa1c
0000A9ED  44                inc sp
0000A9EE  6F                outsw
0000A9EF  776E              ja 0xaa5f
0000A9F1  185067            sbb [bx+si+0x67],dl
0000A9F4  55                push bp
0000A9F5  702C              jo 0xaa23
0000A9F7  50                push ax
0000A9F8  6744              a32 inc sp
0000A9FA  6E                outsb
0000A9FB  202D              and [di],ch
0000A9FD  205061            and [bx+si+0x61],dl
0000AA00  6765205570        and [gs:ebp+0x70],dl
0000AA05  2F                das
0000AA06  44                inc sp
0000AA07  6F                outsw
0000AA08  776E              ja 0xaa78
0000AA0A  1E                push ds
0000AA0B  48                dec ax
0000AA0C  6F                outsw
0000AA0D  6D                insw
0000AA0E  652C45            gs sub al,0x45
0000AA11  6E                outsb
0000AA12  642020            and [fs:bx+si],ah
0000AA15  2D2054            sub ax,0x5420
0000AA18  6F                outsw
0000AA19  702F              jo 0xaa4a
0000AA1B  42                inc dx
0000AA1C  6F                outsw
0000AA1D  7474              jz 0xaa93
0000AA1F  6F                outsw
0000AA20  6D                insw
0000AA21  206F66            and [bx+0x66],ch
0000AA24  206C69            and [si+0x69],ch
0000AA27  7374              jnc 0xaa9d
0000AA29  1D1A20            sbb ax,0x201a
0000AA2C  2C20              sub al,0x20
0000AA2E  1B20              sbb sp,[bx+si]
0000AA30  2020              and [bx+si],ah
0000AA32  2020              and [bx+si],ah
0000AA34  2D2052            sub ax,0x5220
0000AA37  696768742F        imul sp,[bx+0x68],0x2f74
0000AA3C  4C                dec sp
0000AA3D  65667420          gs jz 0xaa61
0000AA41  3120              xor [bx+si],sp
0000AA43  636861            arpl [bx+si+0x61],bp
0000AA46  7218              jc 0xaa60
0000AA48  43                inc bx
0000AA49  7472              jz 0xaabd
0000AA4B  6C                insb
0000AA4C  2D201A            sub ax,0x1a20
0000AA4F  2020              and [bx+si],ah
0000AA51  202D              and [di],ch
0000AA53  205269            and [bp+si+0x69],dl
0000AA56  67687420          a32 push word 0x2074
0000AA5A  3820              cmp [bx+si],ah
0000AA5C  636861            arpl [bx+si+0x61],bp
0000AA5F  7217              jc 0xaa78
0000AA61  43                inc bx
0000AA62  7472              jz 0xaad6
0000AA64  6C                insb
0000AA65  2D201B            sub ax,0x1b20
0000AA68  2020              and [bx+si],ah
0000AA6A  202D              and [di],ch
0000AA6C  204C65            and [si+0x65],cl
0000AA6F  667420            jz 0xaa92
0000AA72  3820              cmp [bx+si],ah
0000AA74  636861            arpl [bx+si+0x61],bp
0000AA77  7207              jc 0xaa80
0000AA79  53                push bx
0000AA7A  45                inc bp
0000AA7B  4C                dec sp
0000AA7C  45                inc bp
0000AA7D  43                inc bx
0000AA7E  54                push sp
0000AA7F  3A1F              cmp bl,[bx]
0000AA81  49                dec cx
0000AA82  6E                outsb
0000AA83  7320              jnc 0xaaa5
0000AA85  2020              and [bx+si],ah
0000AA87  2020              and [bx+si],ah
0000AA89  202D              and [di],ch
0000AA8B  205365            and [bp+di+0x65],dl
0000AA8E  6C                insb
0000AA8F  6563742F          arpl [gs:si+0x2f],si
0000AA93  756E              jnz 0xab03
0000AA95  7365              jnc 0xaafc
0000AA97  6C                insb
0000AA98  65637420          arpl [gs:si+0x20],si
0000AA9C  66696C651F477265  imul ebp,[si+0x65],0x6572471f
0000AAA4  7920              jns 0xaac6
0000AAA6  2B20              sub sp,[bx+si]
0000AAA8  2020              and [bx+si],ah
0000AAAA  2D2053            sub ax,0x5320
0000AAAD  656C              gs insb
0000AAAF  65637420          arpl [gs:si+0x20],si
0000AAB3  66696C6573206279  imul ebp,[si+0x65],0x79622073
0000AABB  206D61            and [di+0x61],ch
0000AABE  736B              jnc 0xab2b
0000AAC0  214772            and [bx+0x72],ax
0000AAC3  657920            gs jns 0xaae6
0000AAC6  2D2020            sub ax,0x2020
0000AAC9  202D              and [di],ch
0000AACB  20556E            and [di+0x6e],dl
0000AACE  7365              jnc 0xab35
0000AAD0  6C                insb
0000AAD1  65637420          arpl [gs:si+0x20],si
0000AAD5  66696C6573206279  imul ebp,[si+0x65],0x79622073
0000AADD  206D61            and [di+0x61],ch
0000AAE0  736B              jnc 0xab4d
0000AAE2  05534F            add ax,0x4f53
0000AAE5  52                push dx
0000AAE6  54                push sp
0000AAE7  3A0D              cmp cl,[di]
0000AAE9  20416C            and [bx+di+0x6c],al
0000AAEC  742B              jz 0xab19
0000AAEE  46                inc si
0000AAEF  3120              xor [bx+si],sp
0000AAF1  2D2062            sub ax,0x6220
0000AAF4  7920              jns 0xab16
0000AAF6  044E              add al,0x4e
0000AAF8  61                popa
0000AAF9  6D                insw
0000AAFA  65094578          or [gs:di+0x78],ax
0000AAFE  7465              jz 0xab65
0000AB00  6E                outsb
0000AB01  7369              jnc 0xab6c
0000AB03  6F                outsw
0000AB04  6E                outsb
0000AB05  0D4F72            or ax,0x724f
0000AB08  6967696E61        imul sp,[bx+0x69],0x616e
0000AB0D  6C                insb
0000AB0E  207369            and [bp+di+0x69],dh
0000AB11  7A65              jpe 0xab78
0000AB13  055261            add ax,0x6152
0000AB16  7469              jz 0xab81
0000AB18  6F                outsw
0000AB19  094461            or [si+0x61],ax
0000AB1C  7465              jz 0xab83
0000AB1E  2F                das
0000AB1F  54                push sp
0000AB20  696D65E720        imul bp,[di+0x65],0x20e7
0000AB25  41                inc cx
0000AB26  6C                insb
0000AB27  742B              jz 0xab54
0000AB29  46                inc si
0000AB2A  36202D            and [ss:di],ch
0000AB2D  2020              and [bx+si],ah
0000AB2F  2020              and [bx+si],ah
0000AB31  55                push bp
0000AB32  6E                outsb
0000AB33  736F              jnc 0xaba4
0000AB35  7274              jc 0xabab
0000AB37  65640D0A20        fs or ax,0x200a
0000AB3C  46                inc si
0000AB3D  362020            and [ss:bx+si],ah
0000AB40  2020              and [bx+si],ah
0000AB42  202D              and [di],ch
0000AB44  204578            and [di+0x78],al
0000AB47  7472              jz 0xabbb
0000AB49  61                popa
0000AB4A  637420            arpl [si+0x20],si
0000AB4D  7365              jnc 0xabb4
0000AB4F  6C                insb
0000AB50  65637465          arpl [gs:si+0x65],si
0000AB54  64206669          and [fs:bp+0x69],ah
0000AB58  6C                insb
0000AB59  65730D            gs jnc 0xab69
0000AB5C  0A20              or ah,[bx+si]
0000AB5E  46                inc si
0000AB5F  3220              xor ah,[bx+si]
0000AB61  2020              and [bx+si],ah
0000AB63  2020              and [bx+si],ah
0000AB65  2D2045            sub ax,0x4520
0000AB68  7874              js 0xabde
0000AB6A  7261              jc 0xabcd
0000AB6C  637420            arpl [si+0x20],si
0000AB6F  746F              jz 0xabe0
0000AB71  2E2E2E0D0A20      cs or ax,0x200a
0000AB77  46                inc si
0000AB78  3320              xor sp,[bx+si]
0000AB7A  2020              and [bx+si],ah
0000AB7C  2020              and [bx+si],ah
0000AB7E  2D2056            sub ax,0x5620
0000AB81  6965772068        imul sp,[di+0x77],0x6820
0000AB86  6967686C69        imul sp,[bx+0x68],0x696c
0000AB8B  67687465          a32 push word 0x6574
0000AB8F  64206669          and [fs:bp+0x69],ah
0000AB93  6C                insb
0000AB94  650D0A20          gs or ax,0x200a
0000AB98  46                inc si
0000AB99  3420              xor al,0x20
0000AB9B  2020              and [bx+si],ah
0000AB9D  2020              and [bx+si],ah
0000AB9F  2D2054            sub ax,0x5420
0000ABA2  6F                outsw
0000ABA3  67676C            a32 insb
0000ABA6  65206675          and [gs:bp+0x75],ah
0000ABAA  6C                insb
0000ABAB  6C                insb
0000ABAC  207061            and [bx+si+0x61],dh
0000ABAF  7468              jz 0xac19
0000ABB1  206E61            and [bp+0x61],ch
0000ABB4  6D                insw
0000ABB5  650D0A20          gs or ax,0x200a
0000ABB9  46                inc si
0000ABBA  352020            xor ax,0x2020
0000ABBD  2020              and [bx+si],ah
0000ABBF  202D              and [di],ch
0000ABC1  205365            and [bp+di+0x65],dl
0000ABC4  7420              jz 0xabe6
0000ABC6  7061              jo 0xac29
0000ABC8  7373              jnc 0xac3d
0000ABCA  776F              ja 0xac3b
0000ABCC  7264              jc 0xac32
0000ABCE  0D0A20            or ax,0x200a
0000ABD1  46                inc si
0000ABD2  37                aaa
0000ABD3  2020              and [bx+si],ah
0000ABD5  2020              and [bx+si],ah
0000ABD7  202D              and [di],ch
0000ABD9  205365            and [bp+di+0x65],dl
0000ABDC  61                popa
0000ABDD  7263              jc 0xac42
0000ABDF  682066            push word 0x6620
0000ABE2  6F                outsw
0000ABE3  7220              jc 0xac05
0000ABE5  66696C656E616D65  imul ebp,[si+0x65],0x656d616e
0000ABED  0D0A20            or ax,0x200a
0000ABF0  46                inc si
0000ABF1  3820              cmp [bx+si],ah
0000ABF3  2020              and [bx+si],ah
0000ABF5  2020              and [bx+si],ah
0000ABF7  2D2056            sub ax,0x5620
0000ABFA  6965772066        imul sp,[di+0x77],0x6620
0000ABFF  696C652063        imul bp,[si+0x65],0x6320
0000AC04  6F                outsw
0000AC05  6D                insw
0000AC06  6D                insw
0000AC07  656E              gs outsb
0000AC09  7473              jz 0xac7e
0000AC0B  7020              jo 0xac2d
0000AC0D  46                inc si
0000AC0E  3920              cmp [bx+si],sp
0000AC10  2020              and [bx+si],ah
0000AC12  2020              and [bx+si],ah
0000AC14  2D2054            sub ax,0x5420
0000AC17  657374            gs jnc 0xac8e
0000AC1A  207365            and [bp+di+0x65],dh
0000AC1D  6C                insb
0000AC1E  65637465          arpl [gs:si+0x65],si
0000AC22  64206669          and [fs:bp+0x69],ah
0000AC26  6C                insb
0000AC27  65730D            gs jnc 0xac37
0000AC2A  0A20              or ah,[bx+si]
0000AC2C  41                inc cx
0000AC2D  6C                insb
0000AC2E  742B              jz 0xac5b
0000AC30  46                inc si
0000AC31  3820              cmp [bx+si],ah
0000AC33  2D2056            sub ax,0x5620
0000AC36  6965772061        imul sp,[di+0x77],0x6120
0000AC3B  7263              jc 0xaca0
0000AC3D  686976            push word 0x7669
0000AC40  6520636F          and [gs:bp+di+0x6f],ah
0000AC44  6D                insw
0000AC45  6D                insw
0000AC46  656E              gs outsb
0000AC48  7473              jz 0xacbd
0000AC4A  0D0A53            or ax,0x530a
0000AC4D  686966            push word 0x6669
0000AC50  742B              jz 0xac7d
0000AC52  46                inc si
0000AC53  37                aaa
0000AC54  2D2043            sub ax,0x4320
0000AC57  6F                outsw
0000AC58  6E                outsb
0000AC59  7469              jz 0xacc4
0000AC5B  6E                outsb
0000AC5C  7565              jnz 0xacc3
0000AC5E  207365            and [bp+di+0x65],dh
0000AC61  61                popa
0000AC62  7263              jc 0xacc7
0000AC64  680D0A            push word 0xa0d
0000AC67  204631            and [bp+0x31],al
0000AC6A  3020              xor [bx+si],ah
0000AC6C  2020              and [bx+si],ah
0000AC6E  202D              and [di],ch
0000AC70  205175            and [bx+di+0x75],dl
0000AC73  697420746F        imul si,[si+0x20],0x6f74
0000AC78  20444F            and [si+0x4f],al
0000AC7B  53                push bx
0000AC7C  1E                push ds
0000AC7D  41                inc cx
0000AC7E  6C                insb
0000AC7F  742B              jz 0xacac
0000AC81  46                inc si
0000AC82  37                aaa
0000AC83  2020              and [bx+si],ah
0000AC85  202D              and [di],ch
0000AC87  204275            and [bp+si+0x75],al
0000AC8A  696C742D69        imul bp,[si+0x74],0x692d
0000AC8F  6E                outsb
0000AC90  206578            and [di+0x78],ah
0000AC93  7472              jz 0xad07
0000AC95  61                popa
0000AC96  637469            arpl [si+0x69],si
0000AC99  6F                outsw
0000AC9A  6E                outsb
0000AC9B  20416C            and [bx+di+0x6c],al
0000AC9E  742B              jz 0xaccb
0000ACA0  46                inc si
0000ACA1  3920              cmp [bx+si],sp
0000ACA3  2020              and [bx+si],ah
0000ACA5  2D2054            sub ax,0x5420
0000ACA8  6F                outsw
0000ACA9  67676C            a32 insb
0000ACAC  65207365          and [gs:bp+di+0x65],dh
0000ACB0  6C                insb
0000ACB1  65637465          arpl [gs:si+0x65],si
0000ACB5  64206669          and [fs:bp+0x69],ah
0000ACB9  6C                insb
0000ACBA  657315            gs jnc 0xacd2
0000ACBD  41                inc cx
0000ACBE  6C                insb
0000ACBF  742B              jz 0xacec
0000ACC1  44                inc sp
0000ACC2  2020              and [bx+si],ah
0000ACC4  2020              and [bx+si],ah
0000ACC6  2D2046            sub ax,0x4620
0000ACC9  7265              jc 0xad30
0000ACCB  65207370          and [gs:bp+di+0x70],dh
0000ACCF  61                popa
0000ACD0  636513            arpl [di+0x13],sp
0000ACD3  41                inc cx
0000ACD4  6C                insb
0000ACD5  742B              jz 0xad02
0000ACD7  54                push sp
0000ACD8  2020              and [bx+si],ah
0000ACDA  2020              and [bx+si],ah
0000ACDC  2D2054            sub ax,0x5420
0000ACDF  656D              gs insw
0000ACE1  7020              jo 0xad03
0000ACE3  6469720B636F      imul si,[fs:bp+si+0xb],0x6f63
0000ACE9  6D                insw
0000ACEA  6D                insw
0000ACEB  656E              gs outsb
0000ACED  7420              jz 0xad0f
0000ACEF  746F              jz 0xad60
0000ACF1  2002              and [bp+si],al
0000ACF3  46                inc si
0000ACF4  330A              xor cx,[bp+si]
0000ACF6  54                push sp
0000ACF7  686520            push word 0x2065
0000ACFA  66696C6520220422  imul ebp,[si+0x65],0x22042220
0000AD02  206973            and [bx+di+0x73],ch
0000AD05  2461              and al,0x61
0000AD07  6E                outsb
0000AD08  206172            and [bx+di+0x72],ah
0000AD0B  636869            arpl [bx+si+0x69],bp
0000AD0E  7665              jna 0xad75
0000AD10  206F66            and [bx+0x66],ch
0000AD13  20756E            and [di+0x6e],dh
0000AD16  6B6E6F77          imul bp,[bp+0x6f],0x77
0000AD1A  6E                outsb
0000AD1B  207061            and [bx+si+0x61],dh
0000AD1E  636B69            arpl [bp+di+0x69],bp
0000AD21  6E                outsb
0000AD22  67206D65          and [ebp+0x65],ch
0000AD26  7468              jz 0xad90
0000AD28  6F                outsw
0000AD29  641450            fs adc al,0x50
0000AD2C  7265              jc 0xad93
0000AD2E  7373              jnc 0xada3
0000AD30  204633            and [bp+0x33],al
0000AD33  20746F            and [si+0x6f],dh
0000AD36  207669            and [bp+0x69],dh
0000AD39  657720            gs ja 0xad5c
0000AD3C  69742E1011        imul si,[si+0x2e],0x1110
0000AD41  1200              adc al,[bx+si]
0000AD43  0807              or [bx],al
0000AD45  09060A05          or [0x50a],ax
0000AD49  0B04              or ax,[si]
0000AD4B  0C03              or al,0x3
0000AD4D  0D020E            or ax,0xe02
0000AD50  010F              add [bx],cx
0000AD52  0103              add [bp+di],ax
0000AD54  07                pop es
0000AD55  0F                db 0x0f
0000AD56  1F                pop ds
0000AD57  3F                aas
0000AD58  7FFF              jg 0xad59
0000AD5A  0000              add [bx+si],al
0000AD5C  0100              add [bx+si],ax
0000AD5E  0300              add ax,[bx+si]
0000AD60  07                pop es
0000AD61  000F              add [bx],cl
0000AD63  001F              add [bx],bl
0000AD65  003F              add [bx],bh
0000AD67  007F00            add [bx+0x0],bh
0000AD6A  FF00              inc word [bx+si]
0000AD6C  FF01              inc word [bx+di]
0000AD6E  FF03              inc word [bp+di]
0000AD70  FF07              inc word [bx]
0000AD72  FF0F              dec word [bx]
0000AD74  FF1F              call word far [bx]
0000AD76  FF                db 0xff
0000AD77  3F                aas
0000AD78  FF                db 0xff
0000AD79  7FFF              jg 0xad7a
0000AD7B  FF01              inc word [bx+di]
0000AD7D  0308              add cx,[bx+si]
0000AD7F  0C18              or al,0x18
0000AD81  1020              adc [bx+si],ah
0000AD83  304030            xor [bx+si+0x30],al
0000AD86  3010              xor [bx+si],dl
0000AD88  00064E61          add [0x614e],al
0000AD8C  6D                insw
0000AD8D  652020            and [gs:bx+si],ah
0000AD90  06                push es
0000AD91  45                inc bp
0000AD92  7874              js 0xae08
0000AD94  2020              and [bx+si],ah
0000AD96  20065369          and [0x6953],al
0000AD9A  7A65              jpe 0xae01
0000AD9C  2020              and [bx+si],ah
0000AD9E  06                push es
0000AD9F  52                push dx
0000ADA0  61                popa
0000ADA1  7469              jz 0xae0c
0000ADA3  6F                outsw
0000ADA4  20065469          and [0x6954],al
0000ADA8  6D                insw
0000ADA9  652020            and [gs:bx+si],ah
0000ADAC  06                push es
0000ADAD  55                push bp
0000ADAE  6E                outsb
0000ADAF  736F              jnc 0xae20
0000ADB1  7274              jc 0xae27
0000ADB3  06                push es
0000ADB4  49                dec cx
0000ADB5  6E                outsb
0000ADB6  7445              jz 0xadfd
0000ADB8  7874              js 0xae2e
0000ADBA  06                push es
0000ADBB  41                inc cx
0000ADBC  43                inc bx
0000ADBD  6F                outsw
0000ADBE  6D                insw
0000ADBF  7473              jz 0xae34
0000ADC1  06                push es
0000ADC2  54                push sp
0000ADC3  6F                outsw
0000ADC4  6753              a32 push bx
0000ADC6  656C              gs insb
0000ADC8  06                push es
0000ADC9  48                dec ax
0000ADCA  656C              gs insb
0000ADCC  7020              jo 0xadee
0000ADCE  20064578          and [0x7845],al
0000ADD2  7472              jz 0xae46
0000ADD4  54                push sp
0000ADD5  6F                outsw
0000ADD6  06                push es
0000ADD7  56                push si
0000ADD8  6965772020        imul sp,[di+0x77],0x2020
0000ADDD  06                push es
0000ADDE  46                inc si
0000ADDF  756C              jnz 0xae4d
0000ADE1  50                push ax
0000ADE2  61                popa
0000ADE3  7406              jz 0xadeb
0000ADE5  50                push ax
0000ADE6  61                popa
0000ADE7  7377              jnc 0xae60
0000ADE9  7264              jc 0xae4f
0000ADEB  06                push es
0000ADEC  45                inc bp
0000ADED  7874              js 0xae63
0000ADEF  7220              jc 0xae11
0000ADF1  20065365          and [0x6553],al
0000ADF5  61                popa
0000ADF6  7263              jc 0xae5b
0000ADF8  680643            push word 0x4306
0000ADFB  6F                outsw
0000ADFC  6D                insw
0000ADFD  6E                outsb
0000ADFE  7473              jz 0xae73
0000AE00  06                push es
0000AE01  54                push sp
0000AE02  657374            gs jnc 0xae79
0000AE05  2020              and [bx+si],ah
0000AE07  152046            adc ax,0x4620
0000AE0A  7265              jc 0xae71
0000AE0C  65207370          and [gs:bp+di+0x70],dh
0000AE10  61                popa
0000AE11  636520            arpl [di+0x20],sp
0000AE14  696E206279        imul bp,[bp+0x20],0x7962
0000AE19  7465              jz 0xae80
0000AE1B  7320              jnc 0xae3d
0000AE1D  084472            or [si+0x72],al
0000AE20  6976652041        imul si,[bp+0x65],0x4120
0000AE25  3A0A              cmp cl,[bp+si]
0000AE27  7770              ja 0xae99
0000AE29  7669              jna 0xae94
0000AE2B  65772E            gs ja 0xae5c
0000AE2E  657865            gs js 0xae96
0000AE31  01B30761          add [bp+di+0x6107],si
0000AE35  7278              jc 0xaeaf
0000AE37  2E657865          gs js 0xaea0
0000AE3B  0454              add al,0x54
0000AE3D  45                inc bp
0000AE3E  4D                dec bp
0000AE3F  50                push ax
0000AE40  07                pop es
0000AE41  6C                insb
0000AE42  68612E            push word 0x2e61
0000AE45  657865            gs js 0xaead
0000AE48  0458              add al,0x58
0000AE4A  59                pop cx
0000AE4B  8920              mov [bx+si],sp
0000AE4D  0B706B            or si,[bx+si+0x6b]
0000AE50  756E              jnz 0xaec0
0000AE52  7A69              jpe 0xaebd
0000AE54  702E              jo 0xae84
0000AE56  657865            gs js 0xaebe
0000AE59  000B              add [bp+di],cl
0000AE5B  706B              jo 0xaec8
0000AE5D  756E              jnz 0xaecd
0000AE5F  7061              jo 0xaec2
0000AE61  6B2E657865        imul bp,[0x7865],0x65
0000AE66  0007              add [bx],al
0000AE68  7061              jo 0xaecb
0000AE6A  6B2E657865        imul bp,[0x7865],0x65
0000AE6F  0000              add [bx+si],al
0000AE71  1800              sbb [bx+si],al
0000AE73  B80761            mov ax,0x6107
0000AE76  726A              jc 0xaee2
0000AE78  2E657865          gs js 0xaee1
0000AE7C  2D2D0D            sub ax,0xd2d
0000AE7F  0A00              or al,[bx+si]
0000AE81  07                pop es
0000AE82  656C              gs insb
0000AE84  692E65786540      imul bp,[0x7865],0x4065
0000AE8A  005000            add [bx+si+0x0],dl
0000AE8D  0009              add [bx+di],cl
0000AE8F  636861            arpl [bx+si+0x61],bp
0000AE92  7263              jc 0xaef7
0000AE94  2E657865          gs js 0xaefd
0000AE98  0220              add ah,[bx+si]
0000AE9A  40                inc ax
0000AE9B  096879            or [bx+si+0x79],bp
0000AE9E  7065              jo 0xaf05
0000AEA0  722E              jc 0xaed0
0000AEA2  657865            gs js 0xaf0a
0000AEA5  022E5C07          add ch,[0x75c]
0000AEA9  7A6F              jpe 0xaf1a
0000AEAB  6F                outsw
0000AEAC  2E657865          gs js 0xaf15
0000AEB0  045E              add al,0x5e
0000AEB2  43                inc bx
0000AEB3  0D0A07            or ax,0x70a
0000AEB6  7166              jno 0xaf1e
0000AEB8  632E6578          arpl [0x7865],bp
0000AEBC  65042E            gs add al,0x2e
0000AEBF  7E7E              jng 0xaf3f
0000AEC1  7E09              jng 0xaecc
0000AEC3  627361            bound si,[bp+di+0x61]
0000AEC6  7263              jc 0xaf2b
0000AEC8  2E657865          gs js 0xaf31
0000AECC  020D              add cl,[di]
0000AECE  0A07              or al,[bx]
0000AED0  627332            bound si,[bp+di+0x32]
0000AED3  2E657865          gs js 0xaf3c
0000AED7  53                push bx
0000AED8  6F                outsw
0000AED9  636861            arpl [bx+si+0x61],bp
0000AEDC  07                pop es
0000AEDD  7371              jnc 0xaf50
0000AEDF  7A2E              jpe 0xaf0f
0000AEE1  657865            gs js 0xaf49
0000AEE4  0450              add al,0x50
0000AEE6  41                inc cx
0000AEE7  54                push sp
0000AEE8  48                dec ax
0000AEE9  096C69            or [si+0x69],bp
0000AEEC  6D                insw
0000AEED  69742E6578        imul si,[si+0x2e],0x7865
0000AEF2  650225            add ah,[gs:di]
0000AEF5  2009              and [bx+di],cl
0000AEF7  687061            push word 0x6170
0000AEFA  636B2E            arpl [bp+di+0x2e],bp
0000AEFD  657865            gs js 0xaf65
0000AF00  022D              add ch,[di]
0000AF02  7306              jnc 0xaf0a
0000AF04  68612E            push word 0x2e61
0000AF07  657865            gs js 0xaf6f
0000AF0A  032D              add bp,[di]
0000AF0C  6A77              push word 0x77
0000AF0E  013A              add [bp+si],di
0000AF10  0C61              or al,0x61
0000AF12  7268              jc 0xaf7c
0000AF14  61                popa
0000AF15  6E                outsb
0000AF16  67656C            gs a32 insb
0000AF19  2E657865          gs js 0xaf82
0000AF1D  07                pop es
0000AF1E  61                popa
0000AF1F  63652E            arpl [di+0x2e],sp
0000AF22  657865            gs js 0xaf8a
0000AF25  043E              add al,0x3e
0000AF27  6E                outsb
0000AF28  756C              jnz 0xaf96
0000AF2A  07                pop es
0000AF2B  7A65              jpe 0xaf92
0000AF2D  742E              jz 0xaf5d
0000AF2F  657865            gs js 0xaf97
0000AF32  012E022D          add [0x2d02],bp
0000AF36  6707              a32 pop es
0000AF38  626F61            bound bp,[bx+0x61]
0000AF3B  2E657865          gs js 0xafa4
0000AF3F  0000              add [bx+si],al
0000AF41  0000              add [bx+si],al
0000AF43  0007              add [bx],al
0000AF45  7061              jo 0xafa8
0000AF47  682E65            push word 0x652e
0000AF4A  7865              js 0xafb1
0000AF4C  022D              add ch,[di]
0000AF4E  7001              jo 0xaf51
0000AF50  2D0971            sub ax,0x7109
0000AF53  7561              jnz 0xafb6
0000AF55  726B              jc 0xafc2
0000AF57  2E657865          gs js 0xafc0
0000AF5B  012A              add [bp+si],bp
0000AF5D  0007              add [bx],al
0000AF5F  7261              jc 0xafc2
0000AF61  722E              jc 0xaf91
0000AF63  657865            gs js 0xafcb
0000AF66  032D              add bp,[di]
0000AF68  2D2000            sub ax,0x20
0000AF6B  07                pop es
0000AF6C  7261              jc 0xafcf
0000AF6E  722E              jc 0xaf9e
0000AF70  657865            gs js 0xafd8
0000AF73  032A              add bp,[bp+si]
0000AF75  2E2A01            sub al,[cs:bx+di]
0000AF78  2003              and [bp+di],al
0000AF7A  2F                das
0000AF7B  6320              arpl [bx+si],sp
0000AF7D  0000              add [bx+si],al
0000AF7F  000A              add [bp+si],cl
0000AF81  5C                pop sp
0000AF82  7E28              jng 0xafac
0000AF84  2129              and [bx+di],bp
0000AF86  7E2E              jng 0xafb6
0000AF88  286129            sub [bx+di+0x29],ah
0000AF8B  1F                pop ds
0000AF8C  001E001F          add [0x1f00],bl
0000AF90  001E001F          add [0x1f00],bl
0000AF94  001F              add [bx],bl
0000AF96  001E001F          add [0x1f00],bl
0000AF9A  001E001F          add [0x1f00],bl
0000AF9E  001F              add [bx],bl
0000AFA0  001D              add [di],bl
0000AFA2  0008              add [bx+si],cl
0000AFA4  7420              jz 0xafc6
0000AFA6  2F                das
0000AFA7  7061              jo 0xb00a
0000AFA9  7468              jz 0xb013
0000AFAB  2005              and [di],al
0000AFAD  7420              jz 0xafcf
0000AFAF  2D7020            sub ax,0x2070
0000AFB2  032D              add bp,[di]
0000AFB4  7420              jz 0xafd6
0000AFB6  0478              add al,0x78
0000AFB8  644E              fs dec si
0000AFBA  2002              and [bp+si],al
0000AFBC  7420              jz 0xafde
0000AFBE  022F              add ch,[bx]
0000AFC0  54                push sp
0000AFC1  022D              add ch,[di]
0000AFC3  7702              ja 0xafc7
0000AFC5  2D620B            sub ax,0xb62
0000AFC8  4C                dec sp
0000AFC9  47                inc di
0000AFCA  41                inc cx
0000AFCB  56                push si
0000AFCC  49                dec cx
0000AFCD  45                inc bp
0000AFCE  57                push di
0000AFCF  2E43              cs inc bx
0000AFD1  46                inc si
0000AFD2  47                inc di
0000AFD3  09504B            or [bx+si+0x4b],dx
0000AFD6  5A                pop dx
0000AFD7  49                dec cx
0000AFD8  50                push ax
0000AFD9  2E45              cs inc bp
0000AFDB  58                pop ax
0000AFDC  45                inc bp
0000AFDD  0B2D              or bp,[di]
0000AFDF  6A66              push word 0x66
0000AFE1  202D              and [di],ch
0000AFE3  214020            and [bx+si+0x20],ax
0000AFE6  2D2D20            sub ax,0x202d
0000AFE9  03413A            add ax,[bx+di+0x3a]
0000AFEC  2002              and [bp+si],al
0000AFEE  2029              and [bx+di],ch
0000AFF0  06                push es
0000AFF1  41                inc cx
0000AFF2  52                push dx
0000AFF3  4A                dec dx
0000AFF4  4F                dec di
0000AFF5  4C                dec sp
0000AFF6  44                inc sp
0000AFF7  07                pop es
0000AFF8  43                inc bx
0000AFF9  4F                dec di
0000AFFA  4D                dec bp
0000AFFB  53                push bx
0000AFFC  50                push ax
0000AFFD  45                inc bp
0000AFFE  43                inc bx
0000AFFF  207820            and [bx+si+0x20],bh
0000B002  2D7031            sub ax,0x3170
0000B005  206520            and [di+0x20],ah
0000B008  2D6A66            sub ax,0x666a
0000B00B  202D              and [di],ch
0000B00D  64202D            and [fs:di],ch
0000B010  65206520          and [gs:di+0x20],ah
0000B014  2F                das
0000B015  7061              jo 0xb078
0000B017  7468              jz 0xb081
0000B019  202D              and [di],ch
0000B01B  44                inc sp
0000B01C  202D              and [di],ch
0000B01E  7870              js 0xb090
0000B020  207864            and [bx+si+0x64],bh
0000B023  2F                das
0000B024  206520            and [di+0x20],ah
0000B027  2F                das
0000B028  7020              jo 0xb04a
0000B02A  7864              js 0xb090
0000B02C  202D              and [di],ch
0000B02E  7863              js 0xb093
0000B030  2020              and [bx+si],ah
0000B032  64202D            and [fs:di],ch
0000B035  7020              jo 0xb057
0000B037  2D6420            sub ax,0x2064
0000B03A  2D6470            sub ax,0x7064
0000B03D  202D              and [di],ch
0000B03F  7872              js 0xb0b3
0000B041  6320              arpl [bx+si],sp
0000B043  64656C            gs insb
0000B046  206520            and [di+0x20],ah
0000B049  2D7320            sub ax,0x2073
0000B04C  7861              js 0xb0af
0000B04E  206561            and [di+0x61],ah
0000B051  207864            and [bx+si+0x64],bh
0000B054  2020              and [bx+si],ah
0000B056  7820              js 0xb078
0000B058  2D6466            sub ax,0x6664
0000B05B  202D              and [di],ch
0000B05D  7820              js 0xb07f
0000B05F  032F              add bp,[bx]
0000B061  673D052D          a32 cmp ax,0x2d05
0000B065  7374              jnc 0xb0db
0000B067  6420433A          and [fs:bp+di+0x3a],al
0000B06B  5C                pop sp
