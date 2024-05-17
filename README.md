# unarc-rs
A library for rust which supports reading of ARC, ZOO, SQZ and HYP files.
This library was written for my bbs project as part of the file analyzation.
This library contains outdated and/or incomplete compression algorithm implementations. 

(In case I overlook  the issues/PRs here contact me on https://github.com/mkrueger/icy_board or per mail)

# arc
Supported compression methods:

* Unpacked
* Packed

Not supported: Currently the rest

I want to support at least Squeezed, Crunched and Squashed - these are the classic ones.
ARC was #1 in the BBS scene before "the patent thing" and ZIP overtook.

Currently it's enough for me - I tried to find a LZW implementation that works but they need some tweaks to work with ARC.
Unfortunately the ARC implementations I found were GPL/LGPL and I want a MIT/Apache library so I can't just port these over.

# Zoo
Compression method 0, 1 & 2 are supported - should cover all methods.

# SQZ - Squeeze It
Only method 0 (Store) supported
It's hard do find infos for that but I suppose they use Squeeze compression as ARC does in method 1

# HYP - Hyper
Only method 0 (Store) supported
It's hard do find infos for that - does anyone know which compression Hyper uses?

# LICENSE

MIT or Apache-2.0 but I don't really care :)