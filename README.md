# unarc-rs
A library for rust which supports reading of ARC files.

Supported compression methods:

* Unpacked
* Packed

Not supported: Currently the rest

I want to support at least Squeezed, Crunched and Squashed - these are the classic ones.

This library was written for my bbs project as part of the file analyzation.
ARC was #1 in the BBS scene before "the patent thing" and ZIP overtook.

Currently it's enough for me - I tried to find a LZW implementation that works but they need some tweaks to work with ARC.
Unfortunately the ARC implementations I found were GPL/LGPL and I want a MIT/Apache library so I can't just port these over.

(In case I overlook  the issues/PRs here contact me on https://github.com/mkrueger/icy_board or per mail)

# LICENSE

MIT or Apache-2.0 but I don't really care :)