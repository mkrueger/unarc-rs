# unarc-rs
A library for rust which supports reading of ARC, ARJ, ZOO, SQ/SQ2/QQQ, SQZ and HYP files.
This library was written for my bbs project as part of the file analyzation.
This library contains outdated and/or incomplete compression algorithm implementations. 

(In case I overlook  the issues/PRs here contact me on https://github.com/mkrueger/icy_board or per mail)

# arc
Supported compression methods:

* Unpacked
* Packed
* Squeezed
* Crunched
* Squashed

Not supported: Crushed & Distilled

ARC was #1 in the BBS scene before "the patent thing" and ZIP overtook.

Currently it's enough for me - I tried to find a LZW implementation that works but they need some tweaks to work with ARC.
Unfortunately the ARC implementations I found were GPL/LGPL and I want a MIT/Apache library so I can't just port these over.

# ARJ

Supported compression methods:

* STORE
* Method 1-3
* Method 4 (fastest)

Notes: That should cover all compressiom methods

This library was written for my bbs project as part of the file analyzation.
ARJ was popular in the BBS scene in the 90' before RAR showed up.

All advanced ARJ features are not supported like multiple archives, password protection etc.
The scope is limited to what I need. Feel free to add features you need.
(In case I overlook  the issues/PRs here contact me on https://github.com/mkrueger/icy_board or per mail)

# Zoo
Compression method 0, 1 & 2 are supported - should cover all methods.

# SQZ - Squeeze It
Only method 0 (Store) supported
It's hard do find infos for that but I suppose they use Squeeze compression as ARC does in method 1

# SQ/SQ2
I wrongly assumed that SQZ == SQ - after implementing squeeze for arc I recognized my error so I threw in the old SQ format even if it's 1 file only.
I added support for the SQ2 format as well. These both don't have a real extension it's either Q as 2nd char or .SQ/.SQ2/.QQQ

# HYP - Hyper
Only method 0 (Store) supported
It's hard do find infos for that - does anyone know which compression Hyper uses?

# LICENSE

MIT or Apache-2.0 but I don't really care :)