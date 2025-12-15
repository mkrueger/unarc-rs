; ============================================================================
; LGAVIEW.COM - Extrahierte SQZ Dekompressionsroutinen
; ============================================================================
; Diese Datei enthält die aus LGAVIEW.COM (SQZ Viewer) extrahierten
; Dekompressionsroutinen. LGAVIEW ist ein DOS-basierter Viewer für
; verschiedene Archivformate, einschließlich SQZ.
;
; Quell-Datei: LGAVIEW.COM (45.164 Bytes)
; Disassembliert mit: ndisasm -b 16
; ============================================================================

; ============================================================================
; ABSCHNITT 1: LZHUF/LZARI Dekompression (Methode 1-4)
; Offset: 0x5A22 - 0x5B41 (Zeilen 10293-10395)
; ============================================================================
; Diese Routine dekomprimiert mit dem LZHUF-Algorithmus:
; - 32KB Sliding Window (0x8000)
; - Huffman-codierte Literale und Längen/Distanzen
; - Window wird initial mit 0x20 (Leerzeichen) gefüllt

LZHUF_DECOMPRESS:
00005A22  33C0              xor ax,ax                  ; ax = 0
00005A24  A30AA0            mov [0xa00a],ax            ; Initialisierung
00005A27  A208A0            mov [0xa008],al
00005A2A  A3E2ED            mov [0xede2],ax
00005A2D  48                dec ax                     ; ax = 0xFFFF
00005A2E  A3FCE8            mov [0xe8fc],ax
00005A31  33FF              xor di,di                  ; di = Output-Position = 0
00005A33  B82020            mov ax,0x2020              ; Leerzeichen (Space)
00005A36  B90040            mov cx,0x4000              ; 16384 Words = 32768 Bytes
00005A39  F3AB              rep stosw                  ; Fülle Window mit Spaces

LZHUF_MAIN_LOOP:
00005A3B  33FF              xor di,di                  ; Reset output position
00005A3D  A106A0            mov ax,[0xa006]            ; Prüfe verbleibende Bytes (high word)
00005A40  0BC0              or ax,ax
00005A42  7852              js 0x5a96                  ; Wenn negativ -> fertig
00005A44  0B0604A0          or ax,[0xa004]             ; Prüfe verbleibende Bytes (low word)
00005A48  744C              jz 0x5a96                  ; Wenn 0 -> fertig

00005A4A  57                push di
00005A4B  E85610            call 0x6aa4                ; Hole nächstes Symbol (Huffman decode)
00005A4E  0AE4              or ah,ah
00005A50  7510              jnz 0x5a62                 ; Wenn ah != 0 -> Match

; --- Literal-Byte (kein Match) ---
00005A52  5F                pop di
00005A53  AA                stosb                      ; Speichere Literal
00005A54  81FF0080          cmp di,0x8000              ; Window voll?
00005A58  7203              jc 0x5a5d                  ; Nein -> weiter
00005A5A  E80912            call 0x6c66                ; Flush Window
00005A5D  B90100            mov cx,0x1
00005A60  EB29              jmp 0x5a8b                 ; Aktualisiere Zähler

; --- Match gefunden ---
00005A62  50                push ax
00005A63  E8F710            call 0x6b5d                ; Hole Offset/Distanz
00005A66  40                inc ax                     ; Distanz + 1
00005A67  59                pop cx                     ; cx = ursprüngliches ax (enthält Länge)
00005A68  5F                pop di                     ; Restore output position
00005A69  8BF7              mov si,di                  ; si = di (aktuelle Position)
00005A6B  2BF0              sub si,ax                  ; si = Position - Distanz
00005A6D  81E6FF7F          and si,0x7fff              ; Maske für 32KB Window

; Längen-Berechnung
00005A71  81E9FD00          sub cx,0xfd                ; cx = cx - 253 (Längen-Basis)
00005A75  51                push cx                    ; Speichere Länge

; --- Copy-Schleife ---
COPY_MATCH_LOOP:
00005A76  A4                movsb                      ; Kopiere Byte
00005A77  81FF0080          cmp di,0x8000              ; Window voll?
00005A7B  7207              jc 0x5a84                  ; Nein -> weiter
00005A7D  56                push si
00005A7E  51                push cx
00005A7F  E8E411            call 0x6c66                ; Flush Window
00005A82  59                pop cx
00005A83  5E                pop si
00005A84  81E6FF7F          and si,0x7fff              ; Wrap-around im Window
00005A88  E2EC              loop 0x5a76                ; Nächstes Byte
00005A8A  59                pop cx                     ; Restore Länge

; --- Aktualisiere Eingabe-Zähler ---
00005A8B  290E04A0          sub [0xa004],cx            ; Verringere remaining bytes
00005A8F  831E06A000        sbb word [0xa006],0x0
00005A94  EBA7              jmp 0x5a3d                 ; Hauptschleife

; --- Ende erreicht ---
00005A96  8BCF              mov cx,di                  ; cx = finale Position
00005A98  E9CE11            jmp 0x6c69                 ; Schreibe Rest und beende

; ============================================================================
; ABSCHNITT 2: LZ77 mit Bit-Flags (Methode ähnlich LZSS)
; Offset: 0x5A9B - 0x5B41 (Zeilen 10396-10473)
; ============================================================================
; Alternative LZ-Dekompression mit expliziten Bit-Flags

LZ77_BITFLAG_DECOMPRESS:
00005A9B  33FF              xor di,di                  ; di = 0
00005A9D  B90040            mov cx,0x4000              ; 16384 Words
00005AA0  B82020            mov ax,0x2020              ; Leerzeichen
00005AA3  F3AB              rep stosw                  ; Fülle Window mit Spaces
00005AA5  BE40A0            mov si,0xa040              ; Eingabe-Puffer
00005AA8  33FF              xor di,di                  ; Output = 0
00005AAA  B280              mov dl,0x80                ; Bit-Maske für Flags

LZ77_BITFLAG_LOOP:
00005AAC  A106A0            mov ax,[0xa006]
00005AAF  0BC0              or ax,ax
00005AB1  78E3              js 0x5a96                  ; Ende wenn negativ
00005AB3  0B0604A0          or ax,[0xa004]
00005AB7  74DD              jz 0x5a96                  ; Ende wenn 0

; Hole nächstes Flag-Bit
00005AB9  D0C2              rol dl,0x0                 ; Rotiere Bit-Maske
00005ABB  7305              jnc 0x5ac2                 ; Wenn kein Carry -> kein neues Byte
00005ABD  E87000            call 0x5b30                ; Hole nächstes Flag-Byte
00005AC0  8AF0              mov dh,al                  ; dh = Flags

; Hole Daten-Byte
00005AC2  E86B00            call 0x5b30                ; Hole nächstes Byte
00005AC5  84F2              test dl,dh                 ; Prüfe Flag-Bit
00005AC7  741C              jz 0x5ae5                  ; Wenn 0 -> Match

; --- Literal-Byte ---
00005AC9  AA                stosb                      ; Speichere Literal
00005ACA  81FF0080          cmp di,0x8000
00005ACE  7207              jc 0x5ad7
00005AD0  56                push si
00005AD1  52                push dx
00005AD2  E89111            call 0x6c66                ; Flush Window
00005AD5  5A                pop dx
00005AD6  5E                pop si
00005AD7  B80100            mov ax,0x1
00005ADA  290604A0          sub [0xa004],ax
00005ADE  831E06A000        sbb word [0xa006],0x0
00005AE3  EBC7              jmp 0x5aac                 ; Hauptschleife

; --- Match decodieren ---
00005AE5  50                push ax                    ; Speichere erstes Byte
00005AE6  E84700            call 0x5b30                ; Hole zweites Byte
00005AE9  5B                pop bx                     ; bx = erstes Byte
00005AEA  8AF8              mov bh,al                  ; bh = zweites Byte
00005AEC  B104              mov cl,0x4
00005AEE  D2EF              shr bh,cl                  ; bh >>= 4 (obere 4 Bits)
00005AF0  83C312            add bx,0x12                ; Distanz = bl + 18
00005AF3  8BCF              mov cx,di                  ; cx = aktuelle Position
00005AF5  56                push si

; Berechne Source-Position
00005AF6  8BF7              mov si,di
00005AF8  81E600F0          and si,0xf000              ; si = di & 0xF000 (4KB-Block)
00005AFC  81E1FF0F          and cx,0xfff               ; cx = di & 0x0FFF
00005B00  3BD9              cmp bx,cx                  ; Vergleiche Distanz
00005B02  7604              jna 0x5b08
00005B04  81EE0010          sub si,0x1000              ; Vorheriger Block
00005B08  03F3              add si,bx                  ; si = Source-Position
00005B0A  81E6FF7F          and si,0x7fff              ; Wrap-around

; Berechne Länge
00005B0E  250F00            and ax,0xf                 ; ax = untere 4 Bits
00005B11  050300            add ax,0x3                 ; Länge = (ax & 0x0F) + 3
00005B14  50                push ax

; Copy-Schleife
00005B15  91                xchg ax,cx
LZ77_COPY_LOOP:
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

00005B2C  58                pop ax                     ; Restore Länge
00005B2D  5E                pop si
00005B2E  EBAA              jmp 0x5ada                 ; Aktualisiere Zähler

; --- Helper: Hole nächstes Byte ---
GET_NEXT_BYTE:
00005B30  AC                lodsb                      ; al = [si++]
00005B31  81FE30DF          cmp si,0xdf30              ; Puffer-Ende?
00005B35  720A              jc 0x5b41                  ; Nein -> fertig
00005B37  50                push ax
00005B38  52                push dx
00005B39  E84C04            call 0x5f88                ; Lade nächsten Block
00005B3C  5A                pop dx
00005B3D  58                pop ax
00005B3E  BE40A0            mov si,0xa040              ; Reset Puffer-Zeiger
00005B41  C3                ret

; ============================================================================
; ABSCHNITT 3: Huffman-Dekodierung Hauptroutine
; Offset: 0x6AA4 - 0x6B41 (Zeilen 11900-11985)
; ============================================================================
; Diese Routine dekodiert ein Symbol aus dem Huffman-Baum.
; Der Baum wird adaptiv aktualisiert (wie LZARI/LZHUF).
;
; Eingabe: Bit-Puffer in [0xa00a], Bit-Zähler in [0xa008]
; Ausgabe: AX = dekodiertes Symbol
;          - Low byte: Wenn < 256 -> Literal
;          - High byte: Wenn != 0 -> Match (Low byte ist Teil der Länge)

HUFFMAN_DECODE:
00006AA4  BB30DF            mov bx,0xdf30              ; Huffman-Baum Basis
00006AA7  8BBFE404          mov di,[bx+0x4e4]          ; Hole Wurzel des Baums
00006AAB  8B160AA0          mov dx,[0xa00a]            ; Lade Bit-Puffer
00006AAF  8A0E08A0          mov cl,[0xa008]            ; Lade Bit-Zähler
00006AB3  32ED              xor ch,ch

HUFFMAN_DECODE_LOOP:
00006AB5  EB0C              jmp 0x6ac3                 ; Springe zur Prüfung

; Navigiere im Baum
HUFFMAN_TRAVERSE:
00006AB7  D1EF              shr di,0x0                 ; di >>= 1
00006AB9  D1E2              shl dx,0x0                 ; Schiebe nächstes Bit raus
00006ABB  83D700            adc di,0x0                 ; di += Carry (0 oder 1)
00006ABE  D1E7              shl di,0x0                 ; di <<= 1 (Index * 2)
00006AC0  8B39              mov di,[bx+di]             ; Hole Kind-Knoten
00006AC2  49                dec cx                     ; Bit-Zähler--

; Prüfe ob Blatt erreicht
00006AC3  E3CD              jcxz 0x6a92                ; Wenn keine Bits mehr -> nachladen
00006AC5  81FFE604          cmp di,0x4e6               ; Ist es ein Blatt?
00006AC9  72EC              jc 0x6ab7                  ; Nein -> weiter traversieren

; Blatt gefunden - Symbol dekodiert
00006ACB  89160AA0          mov [0xa00a],dx            ; Speichere restlichen Bit-Puffer
00006ACF  880E08A0          mov [0xa008],cl            ; Speichere restlichen Bit-Zähler
00006AD3  81EFE604          sub di,0x4e6               ; Symbol = di - 0x4E6
00006AD7  8BD7              mov dx,di                  ; dx = Symbol

; Prüfe ob Baum-Update nötig
00006AD9  813EFAE80080      cmp word [0xe8fa],0x8000   ; Frequenz-Überlauf?
00006ADF  7203              jc 0x6ae4                  ; Nein -> weiter
00006AE1  E823FF            call 0x6a07                ; Skaliere Frequenzen herunter

; Aktualisiere Frequenz des Symbols (adaptive Huffman)
00006AE4  8BB5E4ED          mov si,[di-0x121c]         ; Hole Baum-Position
00006AE8  8B8416E4          mov ax,[si-0x1bea]         ; Hole Frequenz
00006AEC  40                inc ax                     ; Frequenz++
00006AED  898416E4          mov [si-0x1bea],ax         ; Speichere
00006AF1  8D7C02            lea di,[si+0x2]            ; Nächster Knoten

; Baum-Reorganisation (Aufwärts-Propagation)
HUFFMAN_UPDATE_LOOP:
00006AF4  3B8516E4          cmp ax,[di-0x1bea]         ; Vergleiche Frequenzen
00006AF8  763C              jna 0x6b36                 ; Wenn <= -> fertig
00006AFA  47                inc di
00006AFB  47                inc di
00006AFC  3B8516E4          cmp ax,[di-0x1bea]
00006B00  77F8              ja 0x6afa                  ; Suche Position zum Einfügen
00006B02  4F                dec di
00006B03  4F                dec di

; Tausche Knoten
00006B04  878516E4          xchg ax,[di-0x1bea]        ; Tausche Frequenzen
00006B08  898416E4          mov [si-0x1bea],ax
00006B0C  8B9C30DF          mov bx,[si-0x20d0]         ; Hole Kind-Pointer
00006B10  89BFFEE8          mov [bx-0x1702],di         ; Update Parent-Link
00006B14  81FBE604          cmp bx,0x4e6
00006B18  7304              jnc 0x6b1e
00006B1A  89BF00E9          mov [bx-0x1700],di
00006B1E  879D30DF          xchg bx,[di-0x20d0]        ; Tausche Kind-Pointer
00006B22  89B7FEE8          mov [bx-0x1702],si
00006B26  81FBE604          cmp bx,0x4e6
00006B2A  7304              jnc 0x6b30
00006B2C  89B700E9          mov [bx-0x1700],si
00006B30  899C30DF          mov [si-0x20d0],bx
00006B34  8BF7              mov si,di                  ; Weiter aufwärts

; Propagiere zum Parent
00006B36  8BB4FEE8          mov si,[si-0x1702]         ; Hole Parent
00006B3A  0BF6              or si,si                   ; Wurzel erreicht?
00006B3C  75AA              jnz 0x6ae8                 ; Nein -> weiter updaten

; Fertig - Symbol in AX zurückgeben
00006B3E  92                xchg ax,dx                 ; ax = Symbol
00006B3F  D1E8              shr ax,0x0                 ; ax >>= 1 (Index korrigieren)
00006B41  C3                ret

; ============================================================================
; ABSCHNITT 4: Distanz-Dekodierung
; Offset: 0x6B5D - 0x6B85 (Zeilen 11992-12017)
; ============================================================================
; Dekodiert die Distanz für einen LZ-Match.
; Verwendet eine Tabelle mit Basis-Werten und Extra-Bits.

DECODE_DISTANCE:
00006B5D  B108              mov cl,0x8                 ; Hole 8 Bits für Distanz-Index
00006B5F  E82400            call 0x6b86                ; Hole Bits
00006B62  86C4              xchg al,ah                 ; Tausche Bytes
00006B64  97                xchg ax,di                 ; di = Distanz-Index
00006B65  8A950090          mov dl,[di-0x7000]         ; Hole Distanz-Basis (hohe 6 Bits)
00006B69  32F6              xor dh,dh
00006B6B  B106              mov cl,0x6
00006B6D  D3E2              shl dx,cl                  ; dx = Basis << 6
00006B6F  52                push dx
00006B70  8A8D0091          mov cl,[di-0x6f00]         ; Hole Extra-Bit-Anzahl
00006B74  32ED              xor ch,ch
00006B76  49                dec cx
00006B77  49                dec cx                     ; Extra-Bits - 2
00006B78  E80B00            call 0x6b86                ; Hole Extra-Bits
00006B7B  0BC7              or ax,di                   ; Kombiniere
00006B7D  D3C0              rol ax,cl                  ; Rotiere
00006B7F  253F00            and ax,0x3f                ; Maske 6 Bits
00006B82  5A                pop dx
00006B83  0BC2              or ax,dx                   ; Kombiniere mit Basis
00006B85  C3                ret

; Helper: Hole Bits aus dem Eingabe-Strom
GET_BITS_HELPER:
00006B86  8B160AA0          mov dx,[0xa00a]            ; Lade Bit-Puffer
00006B8A  803E08A008        cmp byte [0xa008],0x8      ; Genug Bits?
00006B8F  7F05              jg 0x6b96                  ; Ja -> weiter
00006B91  51                push cx
00006B92  E8B800            call 0x6c4d                ; Lade mehr Bits
00006B95  59                pop cx
00006B96  8BC2              mov ax,dx                  ; ax = Bit-Puffer
00006B98  D3E2              shl dx,cl                  ; Schiebe verbrauchte Bits raus

; ============================================================================
; ABSCHNITT 5: LZW-Dekompression (ähnlich Compress/GIF)
; Offset: 0x621A - 0x62C1 (Zeilen 11210-11283)
; ============================================================================

LZW_DECOMPRESS:
0000621A  C70618A06CCF      mov word [0xa018],0xcf6c   ; Initialisiere Puffer-Zeiger
00006220  33FF              xor di,di                  ; di = 0
00006222  893E1EA0          mov [0xa01e],di            ; Output-Position = 0
00006226  E8BF00            call 0x62e8                ; Initialisiere LZW-Tabelle
00006229  A10CA0            mov ax,[0xa00c]            ; Lade aktuellen Code
0000622C  A304A0            mov [0xa004],ax            ; Speichere als "previous code"
0000622F  E80302            call 0x6435                ; Hole nächsten Code
00006232  3D0001            cmp ax,0x100               ; Spezialcode?
00006235  7518              jnz 0x624f                 ; Nein -> dekodiere
00006237  E8FB01            call 0x6435                ; Hole Sub-Code
0000623A  3D0100            cmp ax,0x1
0000623D  740A              jz 0x6249                  ; 1 = Erhöhe Bitbreite
0000623F  3D0200            cmp ax,0x2
00006242  75EB              jnz 0x622f                 ; Anderer Code -> weiter
00006244  E872FF            call 0x61b9                ; 2 = Reset Tabelle
00006247  EBE6              jmp 0x622f
00006249  FE0608A0          inc byte [0xa008]          ; Erhöhe Code-Bitbreite
0000624D  EBE0              jmp 0x622f

; --- Dekodiere Code zu String ---
0000624F  8BF8              mov di,ax
00006251  A30CA0            mov [0xa00c],ax            ; Speichere aktuellen Code
00006254  BE6EFF            mov si,0xff6e              ; Stack-Puffer
00006257  8BDF              mov bx,di
00006259  83B9D95BFF        cmp word [bx+di+0x5bd9],0xffffffffffffffff
0000625E  750C              jnz 0x626c

; Special case: Code noch nicht in Tabelle
00006260  4E                dec si
00006261  A01DA0            mov al,[0xa01d]            ; Letztes Zeichen
00006264  8804              mov [si],al                ; Auf Stack
00006266  8B3E04A0          mov di,[0xa004]            ; Hole previous code
0000626A  8BDF              mov bx,di

; --- String-Rekonstruktion ---
DECODE_STRING_LOOP:
0000626C  81FF0101          cmp di,0x101               ; Wurzel erreicht?
00006270  7214              jc 0x6286                  ; Ja -> ausgeben
00006272  83B9D95BFF        cmp word [bx+di+0x5bd9],0xffffffffffffffff
00006277  74E7              jz 0x6260                  ; Ungültiger Eintrag
00006279  4E                dec si
0000627A  8A856CCF          mov al,[di-0x3094]         ; Hole Zeichen
0000627E  8804              mov [si],al                ; Auf Stack
00006280  8BB9D95B          mov di,[bx+di+0x5bd9]      ; Hole Parent-Code
00006284  EBE4              jmp 0x626a

; --- Ausgabe String ---
00006286  8A856CCF          mov al,[di-0x3094]         ; Wurzel-Zeichen
0000628A  E89BFE            call 0x6128                ; Gib String aus
0000628D  893E1EA0          mov [0xa01e],di            ; Aktualisiere Position

; --- Füge neuen Eintrag zur Tabelle ---
00006291  8B3E0AA0          mov di,[0xa00a]            ; Nächster freier Slot
00006295  81FF0020          cmp di,0x2000              ; Tabelle voll?
00006299  7D24              jnl 0x62bf                 ; Ja -> überspringe
0000629B  8BDF              mov bx,di
0000629D  A104A0            mov ax,[0xa004]            ; Previous code
000062A0  8981D95B          mov [bx+di+0x5bd9],ax      ; Parent = previous
000062A4  A01DA0            mov al,[0xa01d]            ; Erstes Zeichen von current
000062A7  88856CCF          mov [di-0x3094],al         ; Suffix = erstes Zeichen
000062AB  47                inc di
000062AC  81FF0020          cmp di,0x2000
000062B0  7D09              jnl 0x62bb
000062B2  8BDF              mov bx,di
000062B4  83B9D95BFF        cmp word [bx+di+0x5bd9],0xffffffffffffffff
000062B9  75F0              jnz 0x62ab
000062BB  893E0AA0          mov [0xa00a],di            ; Aktualisiere next slot
000062BF  E967FF            jmp 0x6229                 ; Hauptschleife

; ============================================================================
; ABSCHNITT 6: Huffman-Tabellen Initialisierung
; Offset: 0x59D8 - 0x5A21 (Zeilen 10263-10292)
; ============================================================================

INIT_HUFFMAN_TABLES:
000059D8  B80100            mov ax,0x1                 ; Startwert = 1
000059DB  B93A01            mov cx,0x13a               ; 314 Einträge
000059DE  51                push cx
000059DF  BF16E4            mov di,0xe416              ; Frequenz-Tabelle
000059E2  F3AB              rep stosw                  ; Initialisiere mit 1
000059E4  B8E604            mov ax,0x4e6               ; Erster Node-Wert
000059E7  33D2              xor dx,dx                  ; dx = 0
000059E9  59                pop cx
000059EA  BF30DF            mov di,0xdf30              ; Sortierte Tabelle
000059ED  BBE4ED            mov bx,0xede4              ; Parent-Pointer

; Initialisiere Nodes
INIT_NODES_LOOP:
000059F0  AB                stosw                      ; Speichere Node-Wert
000059F1  40                inc ax
000059F2  40                inc ax                     ; ax += 2
000059F3  8917              mov [bx],dx                ; Parent = Index
000059F5  43                inc bx
000059F6  43                inc bx
000059F7  42                inc dx
000059F8  42                inc dx
000059F9  E2F5              loop 0x59f0

; Baue Huffman-Baum
000059FB  33F6              xor si,si                  ; si = 0
000059FD  BB7402            mov bx,0x274               ; Ausgabe-Position
00005A00  B93901            mov cx,0x139               ; 313 interne Nodes

BUILD_TREE_LOOP:
00005A03  8B8416E4          mov ax,[si-0x1bea]         ; Frequenz[si]
00005A07  038418E4          add ax,[si-0x1be8]         ; + Frequenz[si+2]
00005A0B  898716E4          mov [bx-0x1bea],ax         ; Neue Frequenz
00005A0F  89B730DF          mov [bx-0x20d0],si         ; Left Child = si
00005A13  899CFEE8          mov [si-0x1702],bx         ; Parent[left] = bx
00005A17  899C00E9          mov [si-0x1700],bx         ; Parent[right] = bx
00005A1B  83C604            add si,0x4                 ; si += 4
00005A1E  43                inc bx
00005A1F  43                inc bx
00005A20  E2E1              loop 0x5a03

; ============================================================================
; ABSCHNITT 7: Bit-Lese-Routine
; Offset: 0x574E (Zeilen 10547-10561)
; ============================================================================
; Liest CL Bits aus dem Eingabe-Strom
; Eingabe: CL = Anzahl Bits
; Ausgabe: BX = gelesener Wert, DX aktualisiert

READ_BITS:
0000574E  2AE9              sub ch,cl                  ; ch -= bits to read
00005750  732A              jnc 0x577c                 ; Genug Bits vorhanden
00005752  F6DD              neg ch                     ; ch = fehlende Bits
00005754  2ACD              sub cl,ch                  ; cl = verfügbare Bits
00005756  D3EA              shr dx,cl                  ; Extrahiere verfügbare Bits
00005758  4D                dec bp                     ; Dekrement Byte-Zähler
00005759  7918              jns 0x5773                 ; Noch Bytes da
0000575B  803E765201        cmp byte [0x5276],0x1      ; EOF Flag?
00005760  744E              jz 0x57b0                  ; Ja -> Ende
00005762  50                push ax
00005763  51                push cx
00005764  52                push dx
00005765  E86B09            call 0x60d3                ; Lade mehr Daten
00005768  5A                pop dx
00005769  59                pop cx
0000576A  58                pop ax
0000576B  4D                dec bp
0000576C  7905              jns 0x5773
0000576E  C606765201        mov byte [0x5276],0x1      ; Setze EOF Flag
00005773  8A34              mov dh,[si]                ; Hole nächstes Byte
00005775  46                inc si
00005776  8ACD              mov cl,ch
00005778  B508              mov ch,0x8                 ; 8 neue Bits
0000577A  2AE9              sub ch,cl                  ; Verbleibende Bits
0000577C  32FF              xor bh,bh
0000577E  D3EA              shr dx,cl                  ; Extrahiere Bits
00005780  8ADA              mov bl,dl                  ; Ergebnis in BL
00005782  C3                ret

; ============================================================================
; ABSCHNITT 8: Method 3/4 spezifische Dekompression (erweiterte Distanzen)
; Offset: 0x5EB5 - 0x5EE8 (Zeilen 10818-10851)
; ============================================================================
; Diese Routine behandelt größere Distanzen für Methode 3/4

METHOD34_DISTANCE:
00005EB5  803E20A003        cmp byte [0xa020],0x3      ; Methode 3?
00005EBA  759D              jnz 0x5e59                 ; Nein -> normale Behandlung
00005EBC  83F942            cmp cx,0x42                ; Länge >= 66?
00005EBF  7CA8              jl 0x5e69                  ; Nein -> Standard
00005EC1  81F98200          cmp cx,0x82                ; Länge >= 130?
00005EC5  7C1A              jl 0x5ee1
00005EC7  81F9C200          cmp cx,0xc2                ; Länge >= 194?
00005ECB  7C0A              jl 0x5ed7

; Distanz-Bereich 3 (Länge >= 194)
00005ECD  81E9C000          sub cx,0xc0                ; cx -= 192
00005ED1  81C60060          add si,0x6000              ; Distanz += 24576
00005ED5  EB92              jmp 0x5e69

; Distanz-Bereich 2 (Länge 130-193)
00005ED7  81E98000          sub cx,0x80                ; cx -= 128
00005EDB  81C60040          add si,0x4000              ; Distanz += 16384
00005EDF  EB88              jmp 0x5e69

; Distanz-Bereich 1 (Länge 66-129)
00005EE1  83E940            sub cx,0x40                ; cx -= 64
00005EE4  81C60020          add si,0x2000              ; Distanz += 8192
00005EE8  E97EFF            jmp 0x5e69

; ============================================================================
; ABSCHNITT 9: Methode 4 Dekompression (ähnlich LZSS aber anders)
; Offset: 0x5EEB - 0x5F86 (Zeilen 10852-10943)
; ============================================================================

METHOD4_DECOMPRESS:
00005EEB  E8AD07            call 0x669b                ; Initialisierung
00005EEE  33C0              xor ax,ax
00005EF0  8BF8              mov di,ax                  ; di = 0
00005EF2  A30AA0            mov [0xa00a],ax
00005EF5  A308A0            mov [0xa008],ax

METHOD4_MAIN_LOOP:
00005EF8  833E06A000        cmp word [0xa006],0x0
00005EFD  7507              jnz 0x5f06
00005EFF  833E04A000        cmp word [0xa004],0x0
00005F04  748E              jz 0x5e94                  ; Ende wenn 0

00005F06  E8680A            call 0x6971                ; Dekodiere nächstes Symbol
00005F09  0BC0              or ax,ax
00005F0B  7549              jnz 0x5f56                 ; Match gefunden

; Literal: Sammle genug Bits für ein Byte
00005F0D  833E08A008        cmp word [0xa008],0x8
00005F12  7D1C              jnl 0x5f30
00005F14  A11AA0            mov ax,[0xa01a]
00005F17  8B0E08A0          mov cx,[0xa008]
00005F1B  D3E8              shr ax,cl
00005F1D  09060AA0          or [0xa00a],ax
00005F21  B81000            mov ax,0x10
00005F24  2BC1              sub ax,cx
00005F26  91                xchg ax,cx
00005F27  E87E07            call 0x66a8                ; Hole mehr Bits
00005F2A  C70608A01000      mov word [0xa008],0x10

; Ausgabe Literal
00005F30  A10AA0            mov ax,[0xa00a]
00005F33  B90800            mov cx,0x8
00005F36  D3E8              shr ax,cl                  ; Extrahiere oberes Byte
00005F38  D3260AA0          shl word [0xa00a],cl       ; Schiebe Rest hoch
00005F3C  290E08A0          sub [0xa008],cx
00005F40  AA                stosb                      ; Speichere Literal
00005F41  832E04A001        sub word [0xa004],0x1
00005F46  831E06A000        sbb word [0xa006],0x0
00005F4B  81FF0080          cmp di,0x8000
00005F4F  72A7              jc 0x5ef8
00005F51  E8120D            call 0x6c66
00005F54  EBA2              jmp 0x5ef8

; Match: Dekodiere Länge und Distanz
00005F56  40                inc ax
00005F57  40                inc ax                     ; Länge + 2
00005F58  50                push ax
00005F59  290604A0          sub [0xa004],ax
00005F5D  831E06A000        sbb word [0xa006],0x0
00005F62  E8F709            call 0x695c                ; Hole Distanz
00005F65  8BF7              mov si,di
00005F67  2BF0              sub si,ax
00005F69  4E                dec si
00005F6A  81E6FF7F          and si,0x7fff
00005F6E  59                pop cx

; Copy-Schleife
METHOD4_COPY:
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

; ============================================================================
; WICHTIGE KONSTANTEN UND VARIABLEN
; ============================================================================
;
; 0xa000-0xa002: Verbleibende komprimierte Bytes (32-bit)
; 0xa004-0xa006: Verbleibende dekomprimierte Bytes (32-bit)
; 0xa008:        Aktuelle Bit-Position / Code-Bitbreite
; 0xa00a:        Nächster freier LZW-Slot / Bit-Puffer
; 0xa00c:        Aktueller LZW-Code
; 0xa01a:        Bit-Puffer für Eingabe
; 0xa01d:        Letztes ausgegebenes Zeichen
; 0xa01e:        Aktuelle Ausgabe-Position
; 0xa018:        Eingabe-Puffer-Zeiger
; 0xa020:        Kompressionsmethode (1-4, etc.)
; 0xa040:        Eingabe-Puffer Start
; 0x5bd9:        LZW Dictionary (8KB)
; 0xcf6c:        Eingabe-Puffer Ende (sekundär)
;
; WINDOW: 32KB ab Offset 0 (0x0000-0x7FFF), gefüllt mit 0x20 (Space)
; WINDOW_SIZE = 0x8000 (32768)
; WINDOW_MASK = 0x7FFF
;
; ============================================================================
; VERGLEICH MIT SQZ.EXE
; ============================================================================
;
; SQZ.EXE len_base Tabelle (Offset 0x10C98):
;   [3,4,5,6,7,8,10,12,14,16,20,24,28,32,40,48,56,64,80,96,112,128,160,192,224,256,320,384]
;
; SQZ.EXE len_extra Tabelle (Offset 0x10CD8):
;   [0,0,0,0,1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4,5,5,5,5,6,6,6,6]
;
; LGAVIEW verwendet ähnliche Strukturen, aber die Tabellen sind
; dynamisch in den Huffman-Baum eingebettet.
;
; Der Wert 0xFD (253) bei Offset 0x5A71 entspricht dem len_base-Offset,
; das in SQZ.EXE bei +256 beginnt (Symbole 256-511 sind Längen).
;
; ============================================================================
