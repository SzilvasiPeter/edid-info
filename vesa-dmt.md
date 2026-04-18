39899 Balentine Drive, Suite 125 Phone: 510 651 5122
Newark, CA 94560 Fax: 510 651 5127

# VESA and Industry Standards and Guidelines for Computer Display Monitor Timing (DMT)

Version 1.0, Rev. 13
February 8, 2013

This document includes all current VESA Monitor Timing Standards & Guidelines. Guidelines are subjected to the same VESA review and approval process as Standards, but are designated as Guidelines to ease concerns on the part of some members that VESA is 'endorsing' these timing standards. Guideline designations are typically used for lower resolutions or lower refresh rates that are in common industry use in lower-performance systems. For reference, this document also includes a number of industry-standard timings (de-facto standards) for the computer industry.

This document is the primary means of distribution for all VESA Monitor Timing Standards and Guidelines. The standards and guidelines covered by this document are outlined on the following page.

## Table of Contents

Intellectual Property .......................................................................................................................................... 5
Trademarks ........................................................................................................................................................ 5
Patents ............................................................................................................................................................... 5
Support .............................................................................................................................................................. 5
1. DMT Standards and Guidelines Summary ................................................................................................ 7
2. DMT Standard Codes & IDs Summary ................................................................................................... 10
3. DMT Video Timing Parameter Definitions: ............................................................................................ 14
3.1 DMT Video Timing Parameter Definitions - Positive H & Positive V Syncs: ................................ 14
3.2 DMT Video Timing Parameter Definitions - Positive H & Negative V Syncs: .............................. 14
3.3 DMT Video Timing Parameter Definitions - Negative H & Negative V Syncs: ............................. 14
3.4 DMT Video Timing Parameter Definitions - Negative H & Positive V Syncs: .............................. 15
3.5 DMT Video Timing Parameter Definitions - Total Frame Timing: ................................................. 16
4. DMT Timing Specifications .................................................................................................................... 17
Timing Specifications for 640x350 at 85 Hz ................................................................................................... 18
Timing Specifications for 640x400 at 85 Hz ................................................................................................... 19
Timing Specifications for 720x400 at 85 Hz ................................................................................................... 20
Timing Specifications for 640x480 at 60, 72, 75 & 85 Hz ........................................................................ 21-24
Timing Specifications for 800x600 at 56, 60, 72, 75 & 85 Hz .................................................................. 25-29
Timing Specifications for 800x600 at 120 Hz CVT (Reduced Blanking) ....................................................... 30
Timing Specifications for 848x480 at 60 Hz ................................................................................................... 31
Timing Specifications for 1024x768 at 43 (Int.), 60, 70, 75, & 85 Hz ...................................................... 32-36
Timing Specifications for 1024x768 at 120 Hz CVT (Reduced Blanking) ..................................................... 37
Timing Specifications for 1152x864 at 75 Hz ................................................................................................. 38
Timing Specifications for 1280x720 at 60 Hz ................................................................................................. 39
Timing Specifications for 1280x768 at 60 Hz CVT (Reduced Blanking) ....................................................... 40
Timing Specifications for 1280x768 at 60, 75& 85 Hz ............................................................................. 41-43
Timing Specifications for 1280x768 at 120 Hz CVT (Reduced Blanking) ..................................................... 44
Timing Specifications for 1280x800 at 60 Hz CVT (Reduced Blanking) ....................................................... 45
Timing Specifications for 1280x800 at 60, 75 & 85 Hz ............................................................................ 46-48
Timing Specifications for 1280x800 at 120 Hz CVT (Reduced Blanking) ..................................................... 49
Timing Specifications for 1280x960 at 60 & 85 Hz .................................................................................. 50-51
Timing Specifications for 1280x960 at 120 Hz CVT (Reduced Blanking) ..................................................... 52
Timing Specifications for 1280x1024 at 60, 75 & 85 Hz .......................................................................... 53-55
Timing Specifications for 1280x1024 at 120 Hz CVT (Reduced Blanking) ................................................... 56
Timing Specifications for 1360x768 at 60 Hz ................................................................................................. 57
Timing Specifications for 1360x768 at 120 Hz CVT (Reduced Blanking) ..................................................... 58
Timing Specifications for 1366x768 at 60 Hz (Normal Blanking) .................................................................. 59
Timing Specifications for 1366x768 at 60 Hz (Reduced Blanking) ................................................................ 60
Timing Specifications for 1400x1050 at 60 Hz CVT (Reduced Blanking) ..................................................... 61
Timing Specifications for 1400x1050 at 60, 75 & 85 Hz .......................................................................... 62-64
Timing Specifications for 1400x1050 at 120 Hz CVT (Reduced Blanking) ................................................... 65
Timing Specifications for 1440x900 at 60 Hz CVT (Reduced Blanking) ....................................................... 66
Timing Specifications for 1440x900 at 60, 75 & 85 Hz ............................................................................ 67-69
Timing Specifications for 1440x900 at 120 Hz CVT (Reduced Blanking) ..................................................... 70
Timing Specifications for 1600x900 at 60 Hz (Reduced Blanking) ................................................................ 71
Timing Specifications for 1600x1200 at 60, 65, 70, 75, & 85 Hz ............................................................. 72-76
Timing Specifications for 1600x1200 at 120 Hz (Reduced Blanking) ............................................................ 77
Timing Specifications for 1680x1050 at 60 Hz CVT (Reduced Blanking) ..................................................... 78
Timing Specifications for 1680x1050 at 60, 75 & 85 Hz .......................................................................... 79-81
Timing Specifications for 1680x1050 at 120 Hz CVT (Reduced Blanking) ................................................... 82
Timing Specifications for 1792x1344 at 60 & 75 Hz ................................................................................ 83-84
Timing Specifications for 1792x1344 at 120 Hz CVT (Reduced Blanking) ................................................... 85
Timing Specifications for 1856x1392 at 60 & 75 Hz ................................................................................ 86-87
Timing Specifications for 1856x1392 at 120 Hz CVT (Reduced Blanking) ................................................... 88
Timing Specifications for 1920x1080 at 60 Hz .............................................................................................. 89
Timing Specifications for 1920x1200 at 60 Hz CVT (Reduced Blanking) ..................................................... 90
Timing Specifications for 1920x1200 at 60, 75 & 85 Hz .......................................................................... 91-93
Timing Specifications for 1920x1200 at 120 Hz CVT (Reduced Blanking) ................................................... 94
Timing Specifications for 1920x1440 at 60 & 75 Hz ................................................................................ 95-96
Timing Specifications for 1920x1440 at 120 Hz CVT (Reduced Blanking) ................................................... 97
Timing Specifications for 2048x1152 at 60 Hz (Reduced Blanking) .............................................................. 98
Timing Specifications for 2560x1600 at 60 Hz CVT (Reduced Blanking) ..................................................... 99
Timing Specifications for 2560x1600 at 60, 75 & 85 Hz ...................................................................... 100-102
Timing Specifications for 2560x1600 at 120 Hz CVT (Reduced Blanking) ................................................. 103
Timing Specifications for 4096x2160 at 60 Hz CVT (Reduced Blanking v2) .............................................. 104
Timing Specifications for 4096x2160 at 59.94 Hz CVT (Reduced Blanking v2) ......................................... 105

## Tables

Table 1-1: Summary of Display Monitor Timings – Standards and Guidelines ............................................... 7
Table 2-1: Summary of DMT ID, Std. 2 Byte & CVT 3 Byte Codes ............................................................ 10

## Intellectual Property

© Copyright 1994 –2013 Video Electronics Standards Association. All other rights reserved. While every precaution has been taken in the preparation of this standard, VESA and its contributors assume no responsibility for errors or omissions, and make no warranties, expressed or implied, of functionality or suitability for any purpose.

## Trademarks

All trademarks used in this document are property of their respective owners. VESA is a trademark of the Video Electronics Standards Association.

## Patents

VESA draws attention to the fact that it is claimed that compliance with this specification may involve the use of a patent or other intellectual property right (collectively “IPR”). VESA takes no position concerning the evidence, validity, and scope of this IPR.

Attention is drawn to the possibility that some of the elements of this VESA Standard may be the subject of IPR other than any identified above. VESA shall not be held responsible for identifying any or all such IPR, and has made no inquiry into the possible existence of any such IPR. THIS SPECIFICATION IS BEING OFFERED WITHOUT ANY WARRANTY WHATSOEVER, AND IN PARTICULAR, ANY WARRANTY OF NON-INFRINGEMENT IS EXPRESSLY DISCLAIMED. ANY IMPLEMENTATION OF THIS SPECIFICATION SHALL BE MADE ENTIRELY AT THE IMPLEMENTER’S OWN RISK, AND NEITHER VESA, NOR ANY OF ITS MEMBERS OR SUBMITTERS, SHALL HAVE ANY LIABILITY WHATSOEVER TO ANY IMPLEMENTER OR THIRD PARTY FOR ANY DAMAGES OF ANY NATURE WHATSOEVER DIRECTLY OR INDIRECTLY ARISING FROM THE IMPLEMENTATION OF THIS SPECIFICATION.

## Support

If you have a product that incorporates any of the standards in this document, you should ask the company that manufactured your product for assistance. If you are a display or controller manufacturer, VESA can assist you with any clarifications you may require. All comments or reported errors should be submitted in writing to VESA using one of the following methods

Fax: 510 651 5127, Technical Support
Email: support@vesa.org
Mail: Video Electronics Standards Association 39899 Balentine Drive, Suite 125 Newark, CA 94560

## Revision History

Version 1.0 Revision 0.0 Sept. 12, 1994 Initial Release of the Standard
Version 1.0 Revision 0.1 Oct. 10, 1994 Fixed sync polarity of 1024x768 @ 60 & 70 Hz. Removed page numbers so new timings could be added.
Version 1.0 Revision 0.2 Nov. 4, 1994 Added notes & comments to clarify timing of interlaced modes.
Version 1.0 Revision 0.3 Feb. 16, 1995 Fixed miscellaneous typos
Version 1.0 Revision 0.4 May 4, 1995 Added EDID IDs for DDC, fixed 1024x768 interlace vertical times.
Version 1.0 Revision 0.5 June 14, 1995 Added BIOS mode #s, fixed miscellaneous typos
Version 1.0 Revision 0.6 April 10, 1996 Added new modes from VDMTPROP V1.0, R0.6 passed in March 1996 (85 Hz stds, 1152x864@75, 1280x960@60).
Version 1.0 Revision 0.6a Sept. 8, 1996 Reformatted to Word 6 for electronic distribution
Version 1.0 Revision 0.7 Dec. 18, 1996 Added new modes from VDMTREV V1.0, R0.8 passed in Dec. 1996 (1280x1024@60, 1600x1200@60, 65, 70, 75, 85)
Version 1.0 Revision 0.8 July 22, 1998 Added 1792x1344, 1856X1392 & 1920x1440 all @60, 75 Hz. Corrected EDID code for 1600x1200@85 Hz.
Version 1.0 Revision 0.9 Aug. 21, 2003 Added 848x480@60 Hz, CVT 1280x768 timings, 1360x768@60 Hz, CVT 1400x1050 timings, & CVT 1920x1200 timings based on US & Japan workgroup requests.
Version 1.0 Revision 10 July 14, 2004 Added CVT 1.30MA (1440x900) & CVT 1.76MA (1680x1050) formats.
Version 1.0 Revision 11 May 1, 2007 Added several DMT CVT Reduced Blanking Timings, 1280x800@60/75/85 Hz timings, 2560x1600@60/75/85 Hz and DMT IDs.
Version 1.0 Revision 12 Nov. 17, 2008 Added timing definitions for 1280x720 @ 60Hz, 1366x768 @ 60 Hz (Normal & Reduced Blanking), 1600x900 @ 60 Hz (Reduced Blanking), 1920x1080 @ 60 Hz and 2048x1152 @60 Hz (Reduced Blanking). Updated Tables 1-1 and 2-1.
Version 1.0 Revision 13 Feb. 8, 2013 Added timing definitions for 4096x2160 @ 60Hz (Reduced Blanking v2) and4096x2160 @ 59.94Hz (Reduced Blanking v2). Updated Tables 1-1 and 2-1.

# 1. DMT Standards and Guidelines Summary

Table 1-1 contains a summary of display monitor timings (DMT) that are defined in this standard. All DMTs listed in Table 1-1 are non-interlaced video timing modes, unless otherwise specified using the symbol “(Int.)”. The symbol “(Int.)” means that this DMT is interlaced. All DMTs listed in Table 1-1 include normal video blanking, unless otherwise specified using the symbol “(RB)”. The symbol “(RB)” means that this DMT includes Reduced Blanking. Complete timing specifications for these DMTs are defined in Section 4.

Table 1-1: Summary of Display Monitor Timings – Standards and Guidelines

| Pixel Format | Refresh Rate | Horizontal Frequency | Pixel Frequency | Standard Type | Original Document | Date |
| ------------ | ------------ | -------------------- | --------------- | ------------- | ----------------- | ---- |
| 640 x 350 | 85 Hz | 37.9 kHz | 31.500 MHz | VESA Standard | VDMTPROP | 3/1/96 |
| 640 x 400 | 85 Hz | 37.9 kHz | 31.500 MHz | VESA Standard | VDMTPROP | 3/1/96 |
| 720 x 400 | 85 Hz | 37.9 kHz | 35.500 MHz | VESA Standard | VDMTPROP | 3/1/96 |
| 640 x 480 | 60 Hz | 31.5 kHz | 25.175 MHz | Industry Standard | n/a | n/a |
| 640 x 480 | 72 Hz | 37.9 kHz | 31.500 MHz | VESA Standard | VS901101 | 12/2/92 |
| 640 x 480 | 75 Hz | 37.5 kHz | 31.500 MHz | VESA Standard | VDMT75HZ | 10/4/93 |
| 640 x 480 | 85 Hz | 43.3 kHz | 36.000 MHz | VESA Standard | VDMTPROP | 3/1/96 |
| 800 x 600 | 56 Hz | 35.2 kHz | 36.000 MHz | VESA Guidelines | VG900601 | 8/6/90 |
| 800 x 600 | 60 Hz | 37.9 kHz | 40.000 MHz | VESA Guidelines | VG900602 | 8/6/90 |
| 800 x 600 | 72 Hz | 48.1 kHz | 50.000 MHz | VESA Standard | VS900603A | 8/6/90 |
| 800 x 600 | 75 Hz | 46.9 kHz | 49.500 MHz | VESA Standard | VDMT75HZ | 10/4/93 |
| 800 x 600 | 85 Hz | 53.7 kHz | 56.250 MHz | VESA Standard | VDMTPROP | 3/1/96 |
| 800 x 600 | 120 Hz (RB) | 76.3 kHz | 73.250 MHz | CVT Red. Blanking | n/a | 5/1/07 |
| 848 x 480 | 60 Hz | 31.0 kHz | 33.750 MHz | VESA Standard | AddDMT | 3/4/03 |
| 1024 x 768 | 43 Hz (Int.) | 35.5 kHz | 44.900 MHz | Industry Standard | n/a | n/a |
| 1024 x 768 | 60 Hz | 48.4 kHz | 65.000 MHz | VESA Guidelines | VG901101A | 9/10/91 |
| 1024 x 768 | 70 Hz | 56.5 kHz | 75.000 MHz | VESA Standard | VS910801-2 | 8/9/91 |
| 1024 x 768 | 75 Hz | 60.0 kHz | 78.750 MHz | VESA Standard | VDMT75HZ | 10/4/93 |
| 1024 x 768 | 85 Hz | 68.7 kHz | 94.500 MHz | VESA Standard | VDMTPROP | 3/1/96 |
| 1024 x 768 | 120 Hz (RB) | 97.6 kHz | 115.500 MHz | CVT Red. Blanking | n/a | 5/1/07 |
| 1152 x 864 | 75 Hz | 67.5 kHz | 108.000 MHz | VESA Standard | VDMTPROP | 3/1/96 |
| 1280 x 720 | 60 Hz | 45.0 kHz | 74.250 MHz | CEA Standard | CEA-861 | TBD |
| 1280 x 768 | 60 Hz(RB) | 47.4 kHz | 68.250 MHz | CVT Red. Blanking | AddDMT | 3/4/03 |
| 1280 x 768 | 60 Hz | 47.8 kHz | 79.500 MHz | CVT | AddDMT | 3/4/03 |
| 1280 x 768 | 75 Hz | 60.3 kHz | 102.250 MHz | CVT | AddDMT | 3/4/03 |
| 1280 x 768 | 85 Hz | 68.6 kHz | 117.500 MHz | CVT | AddDMT | 3/4/03 |
| 1280 x 768 | 120 Hz (RB) | 97.4 kHz | 140.250 MHz | CVT Red. Blanking | n/a | 5/1/07 |
| 1280 x 800 | 60 Hz(RB) | 49.3 kHz | 71.000 MHz | CVT Red. Blanking | CVT1.02MA-R | 5/1/07 |
| 1280 x 800 | 60 Hz | 49.7 kHz | 83.500 MHz | CVT | CVT 1.02MA | 5/1/07 |
| 1280 x 800 | 75 Hz | 62.8 kHz | 106.500 MHz | CVT | CVT 1.02MA | 5/1/07 |
| 1280 x 800 | 85 Hz | 71.6 kHz | 122.500 MHz | CVT | CVT 1.02MA | 5/1/07 |
| 1280 x 800 | 120 Hz (RB) | 101.6 kHz | 146.250 MHz | CVT Red. Blanking | n/a | 5/1/07 |
| 1280 x 960 | 60 Hz | 60.0 kHz | 108.000 MHz | VESA Standard | VDMTPROP | 3/1/96 |
| 1280 x 960 | 85 Hz | 85.9 kHz | 148.500 MHz | VESA Standard | VDMTPROP | 3/1/96 |
| 1280 x 960 | 120 Hz (RB) | 121.9 kHz | 175.500 MHz | CVT Red. Blanking | n/a | 5/1/07 |
| 1280 x 1024 | 60 Hz | 64.0 kHz | 108.000 MHz | VESA Standard | VDMTREV | 12/18/96 |
| 1280 x 1024 | 75 Hz | 80.0 kHz | 135.000 MHz | VESA Standard | VDMT75HZ | 10/4/93 |
| 1280 x 1024 | 85 Hz | 91.1 kHz | 157.500 MHz | VESA Standard | VDMTPROP | 3/1/96 |
| 1280 x 1024 | 120 Hz (RB) | 130.0 kHz | 187.250 MHz | CVT Red. Blanking | n/a | 5/1/07 |
| 1360 x 768 | 60 Hz | 47.7 kHz | 85.500 MHz | VESA Standard | AddDMT | 3/4/03 |
| 1360 x 768 | 120 Hz (RB) | 97.5 kHz | 148.250 MHz | CVT Red. Blanking | n/a | 5/1/07 |
| 1366 x 768 | 60 Hz | 47.7 kHz | 85.500 MHz | VESA Standard | DMT Update | 11/30/07 |
| 1366 x 768 | 60 Hz (RB) | 48.0 kHz | 72.000 MHz | VESA Standard | VDMTREV | 11/17/08 |
| 1400 x 1050 | 60 Hz(RB) | 64.7 kHz | 101.000 MHz | CVT Red. Blanking | AddDMT | 5/13/03 |
| 1400 x 1050 | 60 Hz | 65.3 kHz | 121.750 MHz | CVT | AddDMT | 3/4/03 |
| 1400 x 1050 | 75 Hz | 82.3 kHz | 156.000 MHz | CVT | AddDMT | 3/4/03 |
| 1400 x 1050 | 85 Hz | 93.9 kHz | 179.500 MHz | CVT | AddDMT | 3/4/03 |
| 1400 x 1050 | 120 Hz (RB) | 133.3 kHz | 208.000 MHz | CVT Red. Blanking | n/a | 5/1/07 |
| 1440 x 900 | 60 Hz(RB) | 55.5 kHz | 88.750 MHz | CVT Red. Blanking | CVT1.30MA-R | 7/14/04 |
| 1440 x 900 | 60 Hz | 55.9 kHz | 106.500 MHz | CVT | CVT 1.30MA | 7/14/04 |
| 1440 x 900 | 75 Hz | 70.6 kHz | 136.750 MHz | CVT | CVT 1.30MA | 7/14/04 |
| 1440 x 900 | 85 Hz | 80.4 kHz | 157.000 MHz | CVT | CVT 1.30MA | 7/14/04 |
| 1440 x 900 | 120 Hz (RB) | 114.2 kHz | 182.750 MHz | CVT Red. Blanking | n/a | 5/1/07 |
| 1600 x 900 | 60 Hz (RB) | 60.0 kHz | 108.000 MHz | VESA Standard | VDMTREV | 11/17/08 |
| 1600 x 1200 | 60 Hz | 75.0 kHz | 162.000 MHz | VESA Standard | VDMTREV | 12/18/96 |
| 1600 x 1200 | 65 Hz | 81.3 kHz | 175.500 MHz | VESA Standard | VDMTREV | 12/18/96 |
| 1600 x 1200 | 70 Hz | 87. 5 kHz | 189.000 MHz | VESA Standard | VDMTREV | 12/18/96 |
| 1600 x 1200 | 75 Hz | 93.8 kHz | 202.500 MHz | VESA Standard | VDMTREV | 12/18/96 |
| 1600 x 1200 | 85 Hz | 106.3 kHz | 229.500 MHz | VESA Standard | VDMTREV | 12/18/96 |
| 1600 x 1200 | 120 Hz (RB) | 152.4 kHz | 268.250 MHz | CVT Red. Blanking | n/a | 5/1/07 |
| 1680 x 1050 | 60 Hz(RB) | 64.7 kHz | 119.000 MHz | CVT Red. Blanking | CVT1.76MA-R | 7/14/04 |
| 1680 x 1050 | 60 Hz | 65.3 kHz | 146.250 MHz | CVT | CVT 1.76MA | 7/14/04 |
| 1680 x 1050 | 75 Hz | 82.3 kHz | 187.000 MHz | CVT | CVT 1.76MA | 7/14/04 |
| 1680 x 1050 | 85 Hz | 93.9 kHz | 214.750 MHz | CVT | CVT 1.76MA | 7/14/04 |
| 1680 x 1050 | 120 Hz (RB) | 133.4 kHz | 245.500 MHz | CVT Red. Blanking | n/a | 5/1/07 |
| 1792 x 1344 | 60 Hz | 83.6 kHz | 204.750 MHz | VESA Standard | VDMTREV | 9/17/98 |
| 1792 x 1344 | 75 Hz | 106.3 kHz | 261.000 MHz | VESA Standard | VDMTREV | 9/17/98 |
| 1792 x 1344 | 120 Hz (RB) | 170.7 kHz | 333.250 MHz | CVT Red. Blanking | n/a | 5/1/07 |
| 1856 x 1392 | 60 Hz | 86.3 kHz | 218.250 MHz | VESA Standard | VDMTREV | 9/17/98 |
| 1856 x 1392 | 75 Hz | 112.5 kHz | 288.000 MHz | VESA Standard | VDMTREV | 9/17/98 |
| 1856 x 1392 | 120 Hz (RB) | 176.8 kHz | 356.500 MHz | CVT Red. Blanking | n/a | 5/1/07 |
| 1920 x 1080 | 60 Hz | 67.5 kHz | 148.500 MHz | CEA Standard | CEA-861 | TBD |
| 1920 x 1200 | 60 Hz(RB) | 74.0 kHz | 154.000 MHz | CVT Red. Blanking | AddDMT | 3/4/03 |
| 1920 x 1200 | 60 Hz | 74.6 kHz | 193.250 MHz | CVT | AddDMT | 3/4/03 |
| 1920 x 1200 | 75 Hz | 94.0 kHz | 245.250 MHz | CVT | AddDMT | 3/4/03 |
| 1920 x 1200 | 85 Hz | 107.2 kHz | 281.250 MHz | CVT | AddDMT | 3/4/03 |
| 1920 x 1200 | 120 Hz (RB) | 152.4 kHz | 317.000 MHz | CVT Red. Blanking | n/a | 5/1/07 |
| 1920 x 1440 | 60 Hz | 90.0 kHz | 234.000 MHz | VESA Standard | VDMTREV | 9/17/98 |
| 1920 x 1440 | 75 Hz | 112.5 kHz | 297.000 MHz | VESA Standard | VDMTREV | 9/17/98 |
| 1920 x 1440 | 120 Hz (RB) | 182.9 kHz | 380.500 MHz | CVT Red. Blanking | n/a | 5/1/ |
| 2048 x 1152 | 60 Hz (RB) | 70.992 kHz | 156.750 MHz | CVT Red. Blanking | VDMTREV | 11/17/08 |
| 2560 x 1600 | 60 Hz (RB) | 98.7 kHz | 268.500 MHz | CVT Red. Blanking | CVT4.10MA-R | 5/1/07 |
| 2560 x 1600 | 60 Hz | 99.5 kHz | 348.500 MHz | CVT | CVT 4.10MA | 5/1/07 |
| 2560 x 1600 | 75 Hz | 125.4 kHz | 443.250 MHz | CVT | CVT 4.10MA | 5/1/07 |
| 2560 x 1600 | 85 Hz | 142.9 kHz | 505.250 MHz | CVT | CVT 4.10MA | 5/1/07 |
| 2560 x 1600 | 120 Hz (RB) | 203.2 kHz | 552.750 MHz | CVT Red. Blanking | n/a | 5/1/07 |
| 4096 x 2160 | 60 Hz (RB) | 133.320 kHz | 556.744 MHz | CVT Red. Blanking V2 | n/a | 2/8/13 |
| 4096 x 2160 | 59.94 Hz (RB) | 133.187 kHz | 556.188 MHz | CVT Red. Blanking V2 | n/a | 2/8/13 |

# 2. DMT Standard Codes & ID Summary

Table 2-1 includes a list of DMT ID codes, Standard (Std.) Timing 2 byte codes and Coordinated Video Timing (CVT) 3 byte codes. A display may use these codes to indicate support for the associated DMT. Refer to the latest version of VESA’s Enhanced Extended Display Identification (E-EDID) Standard for an explanation of how to derive the Std.   byte codes and the CVT 3 byte codes. The letters “n/a” (not applicable) indicates that a Std. 2 byte code and/or a CVT 3 byte code (DMT is not CVT-compliant) cannot be created.

Table 2-1:  Summary of DMT ID, Std. 2 Byte & CVT 3 Byte Codes

| Pixel Format | Refresh Rate | DMT ID Codes | Std. 2 Byte Codes | CVT 3 Byte Codes |
| ------------ | ------------ | ------------ | ----------------- | ---------------- |
| 640 x 350 | 85 Hz | 01h | n/a | n/a |
| 640 x 400 | 85 Hz | 02h | (31, 19)h | n/a |
| 720 x 400 | 85 Hz | 03h | n/a | n/a |
| 640 x 480 | 60 Hz | 04h |(31, 40)h | n/a |
| 640 x 480 | 72 Hz | 05h | (31, 4C)h | n/a |
| 640 x 480 | 75 Hz | 06h | (31, 4F)h | n/a |
| 640 x 480 | 85 Hz | 07h | (31, 59)h | n/a |
| 800 x 600 | 56 Hz | 08h | n/a | n/a |
| 800 x 600 | 60 Hz | 09h | (45, 40)h | n/a |
| 800 x 600 | 72 Hz | 0Ah | (45, 4C)h | n/a |
| 800 x 600 | 75 Hz | 0Bh | (45, 4F)h | n/a |
| 800 x 600 | 85 Hz | 0Ch | (45, 59)h | n/a |
| 800 x 600 | 120 Hz (RB) | 0Dh | n/a | n/a |
| 848 x 480 | 60 Hz | 0Eh | n/a | n/a |
| 1024 x 768 | 43 Hz (Int.) | 0Fh | n/a | n/a |
| 1024 x 768 | 60 Hz | 10h | (61, 40)h | n/a |
| 1024 x 768 | 70 Hz | 11h | (61, 4A)h | n/a |
| 1024 x 768 | 75 Hz | 12h | (61, 4F)h | n/a |
| 1024 x 768 | 85 Hz | 13h | (61, 59)h | n/a |
| 1024 x 768 | 120 Hz (RB) | 14h | n/a | n/a |
| 1152 x 864 | 75 Hz | 15h | (71, 4F)h | n/a |
| 1280 x 720 | 60 Hz | 55h | (81, C0)h | n/a |
| 1280 x 768 | 60 Hz(RB) | 16h | n/a | (7F, 1C, 21)h |
| 1280 x 768 | 60 Hz | 17h | n/a | (7F, 1C, 28)h |
| 1280 x 768 | 75 Hz | 18h | n/a | (7F, 1C, 44)h |
| 1280 x 768 | 85 Hz | 19h | n/a | (7F, 1C, 62)h |
| 1280 x 768 | 120 Hz (RB) | 1Ah | n/a | n/a |
| 1280 x 800 | 60 Hz (RB) | 1Bh | n/a | (8F, 18, 21)h |
| 1280 x 800 | 60 Hz | 1Ch | (81, 00)h | (8F, 18, 28)h |
| 1280 x 800 | 75 Hz | 1Dh | (81, 0F)h | (8F, 18, 44)h |
| 1280 x 800 | 85 Hz | 1Eh | (81, 19)h | (8F, 18, 62)h |
| 1280 x 800 | 120 Hz (RB) | 1Fh | n/a | n/a |
| 1280 x 960 | 60 Hz | 20h | (81, 40)h | n/a |
| 1280 x 960 | 85 Hz | 21h | (81, 59)h | n/a |
| 1280 x 960 | 120 Hz (RB) | 22h | n/a | n/a |
| 1280 x 1024 | 60 Hz | 23h | (81, 80)h | n/a |
| 1280 x 1024 | 75 Hz | 24h | (81, 8F)h | n/a |
| 1280 x 1024 | 85 Hz | 25h | (81, 99)h | n/a |
| 1280 x 1024 | 120 Hz (RB) | 26h | n/a | n/a |
| 1360 x 768 | 60 Hz | 27h | n/a | n/a |
| 1360 x 768 | 120 Hz (RB) | 28h | n/a | n/a |
| 1366 x 768 | 60 Hz | 51h | n/a | n/a |
| 1366 x 768 | 60 Hz(RB) | 56h | n/a | n/a |
| 1400 x 1050 | 60 Hz(RB) | 29h | n/a | (0C, 20, 21)h |
| 1400 x 1050 | 60 Hz | 2Ah | (90, 40)h | (0C, 20, 28)h |
| 1400 x 1050 | 75 Hz | 2Bh | (90, 4F)h | (0C, 20, 44)h |
| 1400 x 1050 | 85 Hz | 2Ch | (90, 59)h | (0C, 20, 62)h |
| 1400 x 1050 | 120 Hz (RB) | 2Dh | n/a | n/a |
| 1440 x 900 | 60 Hz(RB) | 2Eh | n/a | (C1, 18, 21)h |
| 1440 x 900 | 60 Hz | 2Fh | (95, 00)h | (C1, 18, 28)h |
| 1440 x 900 | 75 Hz | 30h | (95, 0F)h | (C1, 18, 44)h |
| 1440 x 900 | 85 Hz | 31h | (95, 19)h | (C1, 18, 68)h |
| 1440 x 900 | 120 Hz (RB) | 32h | n/a | n/a |
| 1600 x 900 | 60 Hz (RB) | 53h | (A9, C0)h | n/a |
| 1600 x 1200 | 60 Hz | 33h | (A9, 40)h | n/a |
| 1600 x 1200 | 65 Hz | 34h | (A9, 45)h | n/a |
| 1600 x 1200 | 70 Hz | 35h | (A9, 4A)h | n/a |
| 1600 x 1200 | 75 Hz | 36h | (A9, 4F)h | n/a |
| 1600 x 1200 | 85 Hz | 37h | (A9, 59)h | n/a |
| 1600 x 1200 | 120 Hz (RB) | 38h | n/a | n/a |
| 1680 x 1050 | 60 Hz(RB) | 39h | n/a | (0C, 28, 21)h |
| 1680 x 1050 | 60 Hz | 3Ah | (B3, 00)h | (0C, 28, 28)h |
| 1680 x 1050 | 75 Hz | 3Bh | (B3, 0F)h | (0C, 28, 44)h |
| 1680 x 1050 | 85 Hz | 3Ch | (B3, 19)h | (0C, 28, 68)h |
| 1680 x 1050 | 120 Hz (RB) | 3Dh | n/a | n/a |
| 1792 x 1344 | 60 Hz | 3Eh | (C1, 40)h | n/a |
| 1792 x 1344 | 75 Hz | 3Fh | (C1, 4F)h | n/a |
| 1792 x 1344 | 120 Hz (RB) | 40h | n/a | n/a |
| 1856 x 1392 | 60 Hz | 41h | (C9, 40)h | n/a |
| 1856 x 1392 | 75 Hz | 42h | (C9, 4F)h | n/a |
| 1856 x 1392 | 120 Hz (RB) | 43h | n/a | n/a |
| 1920 x 1080 | 60 Hz | 52h | (D1, C0)h | n/a |
| 1920 x 1200 | 60 Hz(RB) | 44h | n/a | (57, 28, 21)h |
| 1920 x 1200 | 60 Hz | 45h | (D1, 00)h | (57, 28, 28)h |
| 1920 x 1200 | 75 Hz | 46h | (D1, 0F)h | (57, 28, 44)h |
| 1920 x 1200 | 85 Hz | 47h | (D1, 19)h | (57, 28, 62)h |
| 1920 x 1200 | 120 Hz (RB) | 48h | n/a | n/a |
| 1920 x 1440 | 60 Hz | 49h | (D1, 40)h | n/a |
| 1920 x 1440 | 75 Hz | 4Ah | (D1, 4F)h | n/a |
| 1920 x 1440 | 120 Hz (RB) | 4Bh | n/a | n/a |
| 2048 x 1152 | 60 Hz (RB) | 54h | (E1, C0)h | n/a |
| 2560 x 1600 | 60 Hz (RB) | 4Ch | n/a | (1F, 38, 21)h |
| 2560 x 1600 | 60 Hz | 4Dh | n/a | (1F, 38, 28)h |
| 2560 x 1600 | 75 Hz | 4Eh | n/a | (1F, 38, 44)h |
| 2560 x 1600 | 85 Hz | 4Fh | n/a | (1F, 38, 62)h |
| 2560 x 1600 | 120 Hz (RB) | 50h | n/a | n/a |
| 4096 x 2160 | 60 Hz (RB) | 57h | n/a | n/a |
| 4096 x 2160 | 59.94 Hz (RB) | 58h | n/a | n/a |

Notes for Table 2-1:

1. The CVT 3 Byte Codes listed in Table 2-1 are unique and are assigned to one video timing mode that was generated using CVT formulas. A source may decode the CVT 3 Byte Code and determine the number of vertical lines, the aspect ratio, the number of horizontal pixels (calculated), the preferred vertical refresh rate, a single supported refresh rate and the blanking style. For example, a source can decode the CVT 3 Byte Code, (7F, 1C, 44)h, with the following results: the number of vertical lines is 768, the aspect ratio is 15 : 9 AR, the number of horizontal pixels (calculated) is 1280, the preferred vertical refresh rate is 75 Hz, the supported vertical refresh rate is 75 Hz and the blanking style is standard (CRT style). Refer to VESA E-EDID Standard, Rel. A, Rev. 2 for an explanation on how to derive a CVT 3 Byte Code from video timing mode parameters.
2. A display (receiver) manufacturer may use the CVT 3 Byte Code to indicate support for a fixed pixel format and one or more vertical refresh rates.
3. For example, a display may contain a CVT 3 Byte Code which indicates support for 1280 x 768 and support for 50 Hz, 60 Hz, 75 Hz & 85 Hz vertical refresh rates with 60 Hz being the preferred vertical refresh rate. In this case the CVT 3 Byte code would be (7F, 1C, 3E)h. When the source decodes the CVT 3 Byte code, (7F, 1C, 3E)h, it knows that the display supports 1280 x 768, along with 50 Hz, 60Hz, 75 Hz & 85 Hz vertical refresh rates with 60 Hz being the preferred vertical refresh rate. The source should output 1280 x 768 at 60 Hz (standard CRT style blanking). The source also knows that the 60 Hz (reduced blanking) is not supported in the display. Refer to E-EDID Standard Rel. A, Rev. 2 for an explanation on how to derive a CVT 3 Byte Code from the video timing mode parameters.

# 3. DMT Video Timing Parameter Definitions

Section 3 includes a list of drawings that define the video timing parameters for all DMTs defined in this standard. There are four drawings based on the possible combinations of positive and negative horizontal and vertical syncs.

## 3.1  DMT Video Timing Parameter Definitions - Positive H & Positive V Syncs

Definition of Terms
|     | "Active" Video |     | Blanking |     |
| --- | -------------- | --- | -------- | --- |
Video
Back Top / Left "Addressable" Video Bottom / Right Front Back
| Sync         |             |        | Sync  |       |
| ------------ | ----------- | ------ | ----- | ----- |
| Porch Border | (Addr Time) | Border | Porch | Porch |
HSync
VSync
|     | Blank Start |     | Blank Time |     |
| --- | ----------- | --- | ---------- | --- |
Sync Start Sync
Time

3.2  DMT Video Timing Parameter Definitions - Positive H & Negative V Syncs

Definition of Terms
|     | "Active" Video |     | Blanking |     |
| --- | -------------- | --- | -------- | --- |
Video
Back Top / Left "Addressable" Video Bottom / Right Front Back
| Sync         |             |        | Sync  |       |
| ------------ | ----------- | ------ | ----- | ----- |
| Porch Border | (Addr Time) | Border | Porch | Porch |
HSync
VSync
|     | Blank Start |     | Blank Time |     |
| --- | ----------- | --- | ---------- | --- |
Sync
Sync Start
Time

3.3  DMT Video Timing Parameter Definitions - Negative H & Negative V Syncs

Definition of Terms
|     | "Active" Video |     | Blanking |     |
| --- | -------------- | --- | -------- | --- |
Video
Back Top / Left "Addressable" Video Bottom / Right Front Back
| Sync         |             |        | Sync  |       |
| ------------ | ----------- | ------ | ----- | ----- |
| Porch Border | (Addr Time) | Border | Porch | Porch |
HSync
VSync
|     | Blank Start |     | Blank Time |     |
| --- | ----------- | --- | ---------- | --- |
Sync
Sync Start Time


3.4  DMT Video Timing Parameter Definitions - Negative H & Positive V Syncs

Definition of Terms
|     | "Active" Video |     | Blanking |     |
| --- | -------------- | --- | -------- | --- |
Video
Back Top / Left "Addressable" Video Bottom / Right Front Back
| Sync         |             |        | Sync  |       |
| ------------ | ----------- | ------ | ----- | ----- |
| Porch Border | (Addr Time) | Border | Porch | Porch |
HSync
VSync
|     | Blank Start |     | Blank Time |     |
| --- | ----------- | --- | ---------- | --- |
Sync
Sync Start Time


3.5 DMT Video Timing Parameter Definitions - Total Frame Timing


4. DMT Timing Specifications
Section 4 includes a list of detailed timing parameters for all DMTs defined in this standard.


VESA MONITOR TIMING STANDARD
Adopted:  3/1/96
Resolution:  640 x 350 at 85 Hz (non-interlaced)
EDID ID:  DMT ID: 01h; Std. 2 Byte Code: n/a; CVT 3 Byte Code: n/a
Method:  *** NOT CVT COMPLIANT ***

Detailed Timing Parameters
| Timing Name   | = 640 x 350 @ 85Hz; |           |             |         |
| ------------- | ------------------- | --------- | ----------- | ------- |
| Hor Pixels    | = 640;              | // Pixels |             |         |
| Ver Pixels    | = 350;              | // Lines  |             |         |
| Hor Frequency | = 37.861;           | // kHz    | = 26.4 usec | / line  |
| Ver Frequency | = 85.080;           | // Hz     | = 11.8 msec | / frame |
31.500;
| Pixel Clock       | =                | // MHz    | = 31.7 nsec       | ± 0.5%       |
| ----------------- | ---------------- | --------- | ----------------- | ------------ |
| Character Width   | = 8;             | // Pixels | = 254.0 nsec      |              |
| Scan Type         | = NONINTERLACED; |           | // H Phase        | = 3.8 %      |
| Hor Sync Polarity | = POSITIVE;      | // HBlank | = 23.1% of HTotal |              |
| Ver Sync Polarity | = NEGATIVE;      | // VBlank | = 21.3% of VTotal |              |
| Hor Total Time    | = 26.413;        | // (usec) | = 104 chars       | = 832 Pixels |
| Hor Addr Time     | = 20.317;        | // (usec) | = 80 chars        | = 640 Pixels |
| Hor Blank Start   | = 20.317;        | // (usec) | = 80 chars        | = 640 Pixels |
| Hor Blank Time    | = 6.095;         | // (usec) | = 24 chars        | = 192 Pixels |
| Hor Sync Start    | = 21.333;        | // (usec) | = 84 chars        | = 672 Pixels |
| // H Right Border | = 0.000;         | // (usec) | = 0 chars         | = 0 Pixels   |
| // H Front Porch  | = 1.016;         | // (usec) | = 4 chars         | = 32 Pixels  |
| Hor Sync Time     | = 2.032;         | // (usec) | = 8 chars         | = 64 Pixels  |
| // H Back Porch   | = 3.048;         | // (usec) | = 12 chars        | = 96 Pixels  |
| // H Left Border  | = 0.000;         | // (usec) | = 0 chars         | = 0 Pixels   |
Ver Total Time = 11.754; // (msec) = 445 lines HT – (1.06xHA)
| Ver Addr Time   | = 9.244;  | // (msec) | = 350 lines | = 4.88 |
| --------------- | --------- | --------- | ----------- | ------ |
| Ver Blank Start | = 9.244;  | // (msec) | = 350 lines |        |
| Ver Blank Time  | = 2.509;  | // (msec) | = 95 lines  |        |
| Ver Sync Start  | = 10.090; | // (msec) | = 382 lines |        |
0
| // V Bottom Border | = 0.000; | // (msec) | = lines    |     |
| ------------------ | -------- | --------- | ---------- | --- |
| // V Front Porch   | = 0.845; | // (msec) | = 32 lines |     |
| Ver Sync Time      | = 0.079; | // (msec) | = 3 lines  |     |
| // V Back Porch    | = 1.585; | // (msec) | = 60 lines |     |
| // V Top Border    | = 0.000; | // (msec) | = 0 lines  |     |

Definition of Terms: Refer to section 3.2.


VESA MONITOR TIMING STANDARD
Adopted:  3/1/96
Resolution:  640 x 400 at 85 Hz (non-interlaced)
EDID ID:  DMT ID: 02h; Std. 2 Byte Code: (31, 19)h; CVT 3 Byte Code: n/a
Method:  *** NOT CVT COMPLIANT ***

Detailed Timing Parameters
| Timing Name | = 640 x 400 @ 85Hz; |     |     |     |
| ----------- | ------------------- | --- | --- | --- |
640;
| Hor Pixels    | =         | // Pixels |             |         |
| ------------- | --------- | --------- | ----------- | ------- |
| Ver Pixels    | = 400;    | // Lines  |             |         |
| Hor Frequency | = 37.861; | // kHz    | = 26.4 usec | / line  |
| Ver Frequency | = 85.080; | // Hz     | = 11.8 msec | / frame |
31.500;
| Pixel Clock       | =                | // MHz    | = 31.7 nsec       | ± 0.5%       |
| ----------------- | ---------------- | --------- | ----------------- | ------------ |
| Character Width   | = 8;             | // Pixels | = 254.0 nsec      |              |
| Scan Type         | = NONINTERLACED; |           | // H Phase        | = 3.8 %      |
| Hor Sync Polarity | = NEGATIVE;      | // HBlank | = 23.1% of HTotal |              |
| Ver Sync Polarity | = POSITIVE;      | // VBlank | = 10.1% of VTotal |              |
| Hor Total Time    | = 26.413;        | // (usec) | = 104 chars       | = 832 Pixels |
| Hor Addr Time     | = 20.317;        | // (usec) | = 80 chars        | = 640 Pixels |
| Hor Blank Start   | = 20.317;        | // (usec) | = 80 chars        | = 640 Pixels |
| Hor Blank Time    | = 6.095;         | // (usec) | = 24 chars        | = 192 Pixels |
| Hor Sync Start    | = 21.333;        | // (usec) | = 84 chars        | = 672 Pixels |
| // H Right Border | = 0.000;         | // (usec) | = 0 chars         | = 0 Pixels   |
| // H Front Porch  | = 1.016;         | // (usec) | = 4 chars         | = 32 Pixels  |
| Hor Sync Time     | = 2.032;         | // (usec) | = 8 chars         | = 64 Pixels  |
| // H Back Porch   | = 3.048;         | // (usec) | = 12 chars        | = 96 Pixels  |
| // H Left Border  | = 0.000;         | // (usec) | = 0 chars         | = 0 Pixels   |
Ver Total Time = 11.754; // (msec) = 445 lines HT – (1.06xHA)
| Ver Addr Time      | = 10.565; | // (msec) | = 400 lines | = 4.88 |
| ------------------ | --------- | --------- | ----------- | ------ |
| Ver Blank Start    | = 10.565; | // (msec) | = 400 lines |        |
| Ver Blank Time     | = 1.189;  | // (msec) | = 45 lines  |        |
| Ver Sync Start     | = 10.591; | // (msec) | = 401 lines |        |
| // V Bottom Border | = 0.000;  | // (msec) | = 0 lines   |        |
| // V Front Porch   | = 0.026;  | // (msec) | = 1 lines   |        |
| Ver Sync Time      | = 0.079;  | // (msec) | = 3 lines   |        |
| // V Back Porch    | = 1.083;  | // (msec) | = 41 lines  |        |
| // V Top Border    | = 0.000;  | // (msec) | = 0 lines   |        |

Definition of Terms: Refer to section 3.4.
VESA MONITOR TIMING STANDARD
Adopted:  3/1/96
Resolution:  720 x 400 at 85 Hz (non-interlaced)
EDID ID:  DMT ID: 03h; Std. 2 Byte Code: n/a; CVT 3 Byte Code: n/a
Method:  *** NOT CVT COMPLIANT ***

Detailed Timing Parameters
| Timing Name       | = 720 x 400 @ 85Hz; |           |                   |              |
| ----------------- | ------------------- | --------- | ----------------- | ------------ |
| Hor Pixels        | = 720;              | // Pixels |                   |              |
| Ver Pixels        | = 400;              | // Lines  |                   |              |
| Hor Frequency     | = 37.927;           | // kHz    | = 26.4 usec       | / line       |
| Ver Frequency     | = 85.039;           | // Hz     | = 11.8 msec       | / frame      |
| Pixel Clock       | = 35.500;           | // MHz    | = 28.2 nsec       | ± 0.5%       |
| Character Width   | = 9;                | // Pixels | = 253.5 nsec      |              |
| Scan Type         | = NONINTERLACED;    |           | // H Phase        | = 3.8 %      |
| Hor Sync Polarity | = NEGATIVE;         | // HBlank | = 23.1% of HTotal |              |
| Ver Sync Polarity | = POSITIVE;         | // VBlank | = 10.3% of VTotal |              |
| Hor Total Time    | = 26.366;           | // (usec) | = 104 chars       | = 936 Pixels |
| Hor Addr Time     | = 20.282;           | // (usec) | = 80 chars        | = 720 Pixels |
| Hor Blank Start   | = 20.282;           | // (usec) | = 80 chars        | = 720 Pixels |
| Hor Blank Time    | = 6.085;            | // (usec) | = 24 chars        | = 216 Pixels |
| Hor Sync Start    | = 21.296;           | // (usec) | = 84 chars        | = 756 Pixels |
| // H Right Border | = 0.000;            | // (usec) | = 0 chars         | = 0 Pixels   |
| // H Front Porch  | = 1.014;            | // (usec) | = 4 chars         | = 36 Pixels  |
| Hor Sync Time     | = 2.028;            | // (usec) | = 8 chars         | = 72 Pixels  |
| // H Back Porch   | = 3.042;            | // (usec) | = 12 chars        | = 108 Pixels |
| // H Left Border  | = 0.000;            | // (usec) | = 0 chars         | = 0 Pixels   |
Ver Total Time = 11.759; // (msec) = 446 lines HT – (1.06xHA)
| Ver Addr Time      | = 10.546; | // (msec) | = 400 lines | = 4.87 |
| ------------------ | --------- | --------- | ----------- | ------ |
| Ver Blank Start    | = 10.546; | // (msec) | = 400 lines |        |
| Ver Blank Time     | = 1.213;  | // (msec) | = 46 lines  |        |
| Ver Sync Start     | = 10.573; | // (msec) | = 401 lines |        |
| // V Bottom Border | = 0.000;  | // (msec) | = 0 lines   |        |
| // V Front Porch   | = 0.026;  | // (msec) | = 1 lines   |        |
| Ver Sync Time      | = 0.079;  | // (msec) | = 3 lines   |        |
| // V Back Porch    | = 1.107;  | // (msec) | = 42 lines  |        |
| // V Top Border    | = 0.000;  | // (msec) | = 0 lines   |        |

Definition of Terms: Refer to section 3.4.


VESA MONITOR TIMING STANDARD
Adopted:  n/a    ** For Reference Only - Not a VESA Standard **
Resolution:  640 x 480 at 60 Hz (non-interlaced)
EDID ID:  DMT ID: 04h; Std. 2 Byte Code: (31, 40)h; CVT 3 Byte Code: n/a
BIOS Modes:  11h, 12h, 101h, 110h, 111h, & 112h (1, 4, 8, 15, 16, & 24 bpp)
Method:  *** NOT CVT COMPLIANT ***

Detailed Timing Parameters
640 x 480 @ 60Hz;
| Timing Name       | =                |           |                   |         |     |
| ----------------- | ---------------- | --------- | ----------------- | ------- | --- |
| Hor Pixels        | = 640;           | // Pixels |                   |         |     |
| Ver Pixels        | = 480;           | // Lines  |                   |         |     |
| Hor Frequency     | = 31.469;        | // kHz    | = 31.8 usec       | / line  |     |
| Ver Frequency     | = 59.940;        | // Hz     | = 16.7 msec       | / frame |     |
| Pixel Clock       | = 25.175;        | // MHz    | = 39.7 nsec       | ± 0.5%  |     |
| Character Width   | = 8;             | // Pixels | = 317.8 nsec      |         |     |
| Scan Type         | = NONINTERLACED; |           | // H Phase        | = 2.0 % |     |
| Hor Sync Polarity | = NEGATIVE;      | // HBlank | = 18.0% of HTotal |         |     |
NEGATIVE;
| Ver Sync Polarity | =         | // VBlank | = 5.5% of VTotal |              |     |
| ----------------- | --------- | --------- | ---------------- | ------------ | --- |
| Hor Total Time    | = 31.778; | // (usec) | = 100 chars      | = 800 Pixels |     |
| Hor Addr Time     | = 25.422; | // (usec) | = 80 chars       | = 640 Pixels |     |
| Hor Blank Start   | = 25.740; | // (usec) | = 81 chars       | = 648 Pixels |     |
| Hor Blank Time    | = 5.720;  | // (usec) | = 18 chars       | = 144 Pixels |     |
| Hor Sync Start    | = 26.058; | // (usec) | = 82 chars       | = 656 Pixels |     |
| // H Right Border | = 0.318;  | // (usec) | = 1 chars        | = 8 Pixels   |     |
| // H Front Porch  | = 0.318;  | // (usec) | = 1 chars        | = 8 Pixels   |     |
| Hor Sync Time     | = 3.813;  | // (usec) | = 12 chars       | = 96 Pixels  |     |
| // H Back Porch   | = 1.589;  | // (usec) | = 5 chars        | = 40 Pixels  |     |
| // H Left Border  | = 0.318;  | // (usec) | = 1 chars        | = 8 Pixels   |     |
Ver Total Time = 16.683; // (msec) = 525 lines HT – (1.06xHA)
| Ver Addr Time      | = 15.253; | // (msec) | = 480 lines | = 4.83 |     |
| ------------------ | --------- | --------- | ----------- | ------ | --- |
| Ver Blank Start    | = 15.507; | // (msec) | = 488 lines |        |     |
| Ver Blank Time     | = 0.922;  | // (msec) | = 29 lines  |        |     |
| Ver Sync Start     | = 15.571; | // (msec) | = 490 lines |        |     |
| // V Bottom Border | = 0.254;  | // (msec) | = 8 lines   |        |     |
| // V Front Porch   | = 0.064;  | // (msec) | = 2 lines   |        |     |
| Ver Sync Time      | = 0.064;  | // (msec) | = 2 lines   |        |     |
| // V Back Porch    | = 0.794;  | // (msec) | = 25 lines  |        |     |
8
| // V Top Border | = 0.254; | // (msec) | = lines |     |     |
| --------------- | -------- | --------- | ------- | --- | --- |

Definition of Terms: Refer to section 3.3.


VESA MONITOR TIMING STANDARD
Adopted:  11/11/90 (VESA #901101)
Resolution:  640 x 480 at 72 Hz (non-interlaced)
EDID ID:  DMT ID: 05h; Std. 2 Byte Code: (31, 4C)h; CVT 3 Byte Code: n/a
BIOS Modes:  11h, 12h, 101h, 110h, 111h, & 112h (1, 4, 8, 15, 16, & 24 bpp)
Method:  *** NOT CVT COMPLIANT ***

Detailed Timing Parameters
640 x 480 @ 72Hz;
| Timing Name       | =                |           |                   |         |     |
| ----------------- | ---------------- | --------- | ----------------- | ------- | --- |
| Hor Pixels        | = 640;           | // Pixels |                   |         |     |
| Ver Pixels        | = 480;           | // Lines  |                   |         |     |
| Hor Frequency     | = 37.861;        | // kHz    | = 26.4 usec       | / line  |     |
| Ver Frequency     | = 72.809;        | // Hz     | = 13.7 msec       | / frame |     |
| Pixel Clock       | = 31.500;        | // MHz    | = 31.7 nsec       | ± 0.5%  |     |
| Character Width   | = 8;             | // Pixels | = 254.0 nsec      |         |     |
| Scan Type         | = NONINTERLACED; |           | // H Phase        | = 6.3 % |     |
| Hor Sync Polarity | = NEGATIVE;      | // HBlank | = 21.2% of HTotal |         |     |
NEGATIVE;
| Ver Sync Polarity | =         | // VBlank | = 4.6% of VTotal |              |     |
| ----------------- | --------- | --------- | ---------------- | ------------ | --- |
| Hor Total Time    | = 26.413; | // (usec) | = 104 chars      | = 832 Pixels |     |
| Hor Addr Time     | = 20.317; | // (usec) | = 80 chars       | = 640 Pixels |     |
| Hor Blank Start   | = 20.571; | // (usec) | = 81 chars       | = 648 Pixels |     |
| Hor Blank Time    | = 5.587;  | // (usec) | = 22 chars       | = 176 Pixels |     |
| Hor Sync Start    | = 21.079; | // (usec) | = 83 chars       | = 664 Pixels |     |
| // H Right Border | = 0.254;  | // (usec) | = 1 chars        | = 8 Pixels   |     |
| // H Front Porch  | = 0.508;  | // (usec) | = 2 chars        | = 16 Pixels  |     |
| Hor Sync Time     | = 1.270;  | // (usec) | = 5 chars        | = 40 Pixels  |     |
| // H Back Porch   | = 3.810;  | // (usec) | = 15 chars       | = 120 Pixels |     |
| // H Left Border  | = 0.254;  | // (usec) | = 1 chars        | = 8 Pixels   |     |
Ver Total Time = 13.735; // (msec) = 520 lines HT – (1.06xHA)
| Ver Addr Time      | = 12.678; | // (msec) | = 480 lines | = 4.88 |     |
| ------------------ | --------- | --------- | ----------- | ------ | --- |
| Ver Blank Start    | = 12.889; | // (msec) | = 488 lines |        |     |
| Ver Blank Time     | = 0.634;  | // (msec) | = 24 lines  |        |     |
| Ver Sync Start     | = 12.916; | // (msec) | = 489 lines |        |     |
| // V Bottom Border | = 0.211;  | // (msec) | = 8 lines   |        |     |
| // V Front Porch   | = 0.026;  | // (msec) | = 1 lines   |        |     |
| Ver Sync Time      | = 0.079;  | // (msec) | = 3 lines   |        |     |
| // V Back Porch    | = 0.528;  | // (msec) | = 20 lines  |        |     |
8
| // V Top Border | = 0.211; | // (msec) | = lines |     |     |
| --------------- | -------- | --------- | ------- | --- | --- |

Definition of Terms: Refer to section 3.3.

VESA MONITOR TIMING STANDARD
Adopted:  10/4/93
Resolution:  640 x 480 at 75 Hz (non-interlaced)
EDID ID:  DMT ID: 06h; Std. 2 Byte Code: (31, 4F)h; CVT 3 Byte Code: n/a
BIOS Modes:  11h, 12h, 101h, 110h, 111h, & 112h (1, 4, 8, 15, 16, & 24 bpp)
Method:  *** NOT CVT COMPLIANT ***

Detailed Timing Parameters
640 x 480 @ 75Hz;
| Timing Name       | =                |           |                   |         |     |
| ----------------- | ---------------- | --------- | ----------------- | ------- | --- |
| Hor Pixels        | = 640;           | // Pixels |                   |         |     |
| Ver Pixels        | = 480;           | // Lines  |                   |         |     |
| Hor Frequency     | = 37.500;        | // kHz    | = 26.7 usec       | / line  |     |
| Ver Frequency     | = 75.000;        | // Hz     | = 13.3 msec       | / frame |     |
| Pixel Clock       | = 31.500;        | // MHz    | = 31.7 nsec       | ± 0.5%  |     |
| Character Width   | = 8;             | // Pixels | = 254.0 nsec      |         |     |
| Scan Type         | = NONINTERLACED; |           | // H Phase        | = 6.2 % |     |
| Hor Sync Polarity | = NEGATIVE;      | // HBlank | = 23.8% of HTotal |         |     |
NEGATIVE;
| Ver Sync Polarity | =         | // VBlank | = 4.0% of VTotal |              |     |
| ----------------- | --------- | --------- | ---------------- | ------------ | --- |
| Hor Total Time    | = 26.667; | // (usec) | = 105 chars      | = 840 Pixels |     |
| Hor Addr Time     | = 20.317; | // (usec) | = 80 chars       | = 640 Pixels |     |
| Hor Blank Start   | = 20.317; | // (usec) | = 80 chars       | = 640 Pixels |     |
| Hor Blank Time    | = 6.349;  | // (usec) | = 25 chars       | = 200 Pixels |     |
| Hor Sync Start    | = 20.825; | // (usec) | = 82 chars       | = 656 Pixels |     |
| // H Right Border | = 0.000;  | // (usec) | = 0 chars        | = 0 Pixels   |     |
| // H Front Porch  | = 0.508;  | // (usec) | = 2 chars        | = 16 Pixels  |     |
| Hor Sync Time     | = 2.032;  | // (usec) | = 8 chars        | = 64 Pixels  |     |
| // H Back Porch   | = 3.810;  | // (usec) | = 15 chars       | = 120 Pixels |     |
| // H Left Border  | = 0.000;  | // (usec) | = 0 chars        | = 0 Pixels   |     |
Ver Total Time = 13.333; // (msec) = 500 lines HT – (1.06xHA)
| Ver Addr Time      | = 12.800; | // (msec) | = 480 lines | = 5.13 |     |
| ------------------ | --------- | --------- | ----------- | ------ | --- |
| Ver Blank Start    | = 12.800; | // (msec) | = 480 lines |        |     |
| Ver Blank Time     | = 0.533;  | // (msec) | = 20 lines  |        |     |
| Ver Sync Start     | = 12.827; | // (msec) | = 481 lines |        |     |
| // V Bottom Border | = 0.000;  | // (msec) | = 0 lines   |        |     |
| // V Front Porch   | = 0.027;  | // (msec) | = 1 lines   |        |     |
| Ver Sync Time      | = 0.080;  | // (msec) | = 3 lines   |        |     |
| // V Back Porch    | = 0.427;  | // (msec) | = 16 lines  |        |     |
0
| // V Top Border | = 0.000; | // (msec) | = lines |     |     |
| --------------- | -------- | --------- | ------- | --- | --- |

Definition of Terms: Refer to section 3.3.

VESA MONITOR TIMING STANDARD
Adopted:  3/1/96
Resolution:  640 x 480 at 85 Hz (non-interlaced)
EDID ID:  DMT ID: 07h; Std. 2 Byte Code: (31, 59)h; CVT 3 Byte Code: n/a
*** NOT CVT COMPLIANT ***
Method:

Detailed Timing Parameters
| Timing Name       | = 640 x 480 @ 85Hz; |           |                   |              |
| ----------------- | ------------------- | --------- | ----------------- | ------------ |
| Hor Pixels        | = 640;              | // Pixels |                   |              |
| Ver Pixels        | = 480;              | // Lines  |                   |              |
| Hor Frequency     | = 43.269;           | // kHz    | = 23.1 usec       | / line       |
| Ver Frequency     | = 85.008;           | // Hz     | = 11.8 msec       | / frame      |
| Pixel Clock       | = 36.000;           | // MHz    | = 27.8 nsec       | ± 0.5%       |
| Character Width   | = 8;                | // Pixels | = 222.2 nsec      |              |
| Scan Type         | = NONINTERLACED;    |           | // H Phase        | = 1.4 %      |
| Hor Sync Polarity | = NEGATIVE;         | // HBlank | = 23.1% of HTotal |              |
| Ver Sync Polarity | = NEGATIVE;         | // VBlank | = 5.7% of VTotal  |              |
| Hor Total Time    | = 23.111;           | // (usec) | = 104 chars       | = 832 Pixels |
| Hor Addr Time     | = 17.778;           | // (usec) | = 80 chars        | = 640 Pixels |
| Hor Blank Start   | = 17.778;           | // (usec) | = 80 chars        | = 640 Pixels |
| Hor Blank Time    | = 5.333;            | // (usec) | = 24 chars        | = 192 Pixels |
| Hor Sync Start    | = 19.333;           | // (usec) | = 87 chars        | = 696 Pixels |
| // H Right Border | = 0.000;            | // (usec) | = 0 chars         | = 0 Pixels   |
| // H Front Porch  | = 1.556;            | // (usec) | = 7 chars         | = 56 Pixels  |
| Hor Sync Time     | = 1.556;            | // (usec) | = 7 chars         | = 56 Pixels  |
| // H Back Porch   | = 2.222;            | // (usec) | = 10 chars        | = 80 Pixels  |
| // H Left Border  | = 0.000;            | // (usec) | = 0 chars         | = 0 Pixels   |
Ver Total Time = 11.764; // (msec) = 509 lines HT – (1.06xHA)
| Ver Addr Time      | = 11.093; | // (msec) | = 480 lines | = 4.27 |
| ------------------ | --------- | --------- | ----------- | ------ |
| Ver Blank Start    | = 11.093; | // (msec) | = 480 lines |        |
| Ver Blank Time     | = 0.670;  | // (msec) | = 29 lines  |        |
| Ver Sync Start     | = 11.116; | // (msec) | = 481 lines |        |
| // V Bottom Border | = 0.000;  | // (msec) | = 0 lines   |        |
| // V Front Porch   | = 0.023;  | // (msec) | = 1 lines   |        |
| Ver Sync Time      | = 0.069;  | // (msec) | = 3 lines   |        |
25
| // V Back Porch | = 0.578; | // (msec) | = lines   |     |
| --------------- | -------- | --------- | --------- | --- |
| // V Top Border | = 0.000; | // (msec) | = 0 lines |     |

Definition of Terms: Refer to section 3.3.

VESA MONITOR TIMING STANDARD
Adopted:  8/7/90 (VESA #900601)
Resolution:  800 x 600 at 56 Hz (non-interlaced)
EDID ID:  DMT ID: 08h; Std. 2 Byte Code: n/a; CVT 3 Byte Code: n/a
BIOS Modes:  102h, 103h, 113h, 114h, & 115h (4, 8, 15, 16, & 24 bpp)
Method:  *** NOT CVT COMPLIANT ***

Detailed Timing Parameters
800 x 600 @ 56Hz;
| Timing Name       | =                |           |                   |         |     |
| ----------------- | ---------------- | --------- | ----------------- | ------- | --- |
| Hor Pixels        | = 800;           | // Pixels |                   |         |     |
| Ver Pixels        | = 600;           | // Lines  |                   |         |     |
| Hor Frequency     | = 35.156;        | // kHz    | = 28.4 usec       | / line  |     |
| Ver Frequency     | = 56.250;        | // Hz     | = 17.8 msec       | / frame |     |
| Pixel Clock       | = 36.000;        | // MHz    | = 27.8 nsec       | ± 0.5%  |     |
| Character Width   | = 8;             | // Pixels | = 222.2 nsec      |         |     |
| Scan Type         | = NONINTERLACED; |           | // H Phase        | = 5.1 % |     |
| Hor Sync Polarity | = POSITIVE;      | // HBlank | = 21.9% of HTotal |         |     |
POSITIVE;
| Ver Sync Polarity | =         | // VBlank | = 4.0% of VTotal |               |     |
| ----------------- | --------- | --------- | ---------------- | ------------- | --- |
| Hor Total Time    | = 28.444; | // (usec) | = 128 chars      | = 1024 Pixels |     |
| Hor Addr Time     | = 22.222; | // (usec) | = 100 chars      | = 800 Pixels  |     |
| Hor Blank Start   | = 22.222; | // (usec) | = 100 chars      | = 800 Pixels  |     |
| Hor Blank Time    | = 6.222;  | // (usec) | = 28 chars       | = 224 Pixels  |     |
| Hor Sync Start    | = 22.889; | // (usec) | = 103 chars      | = 824 Pixels  |     |
| // H Right Border | = 0.000;  | // (usec) | = 0 chars        | = 0 Pixels    |     |
| // H Front Porch  | = 0.667;  | // (usec) | = 3 chars        | = 24 Pixels   |     |
| Hor Sync Time     | = 2.000;  | // (usec) | = 9 chars        | = 72 Pixels   |     |
| // H Back Porch   | = 3.556;  | // (usec) | = 16 chars       | = 128 Pixels  |     |
| // H Left Border  | = 0.000;  | // (usec) | = 0 chars        | = 0 Pixels    |     |
Ver Total Time = 17.778; // (msec) = 625 lines HT – (1.06xHA)
| Ver Addr Time      | = 17.067; | // (msec) | = 600 lines | = 4.89 |     |
| ------------------ | --------- | --------- | ----------- | ------ | --- |
| Ver Blank Start    | = 17.067; | // (msec) | = 600 lines |        |     |
| Ver Blank Time     | = 0.711;  | // (msec) | = 25 lines  |        |     |
| Ver Sync Start     | = 17.095; | // (msec) | = 601 lines |        |     |
| // V Bottom Border | = 0.000;  | // (msec) | = 0 lines   |        |     |
| // V Front Porch   | = 0.028;  | // (msec) | = 1 lines   |        |     |
| Ver Sync Time      | = 0.057;  | // (msec) | = 2 lines   |        |     |
| // V Back Porch    | = 0.626;  | // (msec) | = 22 lines  |        |     |
0
| // V Top Border | = 0.000; | // (msec) | = lines |     |     |
| --------------- | -------- | --------- | ------- | --- | --- |

Definition of Terms: Refer to section 3.1.

VESA MONITOR TIMING STANDARD
Adopted:  8/7/90 (VESA #900602)
Resolution:  800 x 600 at 60 Hz (non-interlaced)
EDID ID:  DMT ID: 09h; Std. 2 Byte Code: (45, 40)h; CVT 3 Byte Code: n/a
BIOS Modes:  102h, 103h, 113h, 114h, & 115h (4, 8, 15, 16, & 24 bpp)
Method:  *** NOT CVT COMPLIANT ***

Detailed Timing Parameters
| Timing Name   | = 800 x 600 @ 60Hz; |           |             |         |
| ------------- | ------------------- | --------- | ----------- | ------- |
| Hor Pixels    | = 800;              | // Pixels |             |         |
| Ver Pixels    | = 600;              | // Lines  |             |         |
| Hor Frequency | = 37.879;           | // kHz    | = 26.4 usec | / line  |
| Ver Frequency | = 60.317;           | // Hz     | = 16.6 msec | / frame |
| Pixel Clock   | = 40.000;           | // MHz    | = 25.0 nsec | ± 0.5%  |
8;
| Character Width   | =                | // Pixels | = 200.0 nsec      |               |
| ----------------- | ---------------- | --------- | ----------------- | ------------- |
| Scan Type         | = NONINTERLACED; |           | // H Phase        | = 2.3 %       |
| Hor Sync Polarity | = POSITIVE;      | // HBlank | = 24.2% of HTotal |               |
| Ver Sync Polarity | = POSITIVE;      | // VBlank | = 4.5% of VTotal  |               |
| Hor Total Time    | = 26.400;        | // (usec) | = 132 chars       | = 1056 Pixels |
| Hor Addr Time     | = 20.000;        | // (usec) | = 100 chars       | = 800 Pixels  |
| Hor Blank Start   | = 20.000;        | // (usec) | = 100 chars       | = 800 Pixels  |
| Hor Blank Time    | = 6.400;         | // (usec) | = 32 chars        | = 256 Pixels  |
| Hor Sync Start    | = 21.000;        | // (usec) | = 105 chars       | = 840 Pixels  |
0
| // H Right Border | = 0.000; | // (usec) | = chars    | = 0 Pixels   |
| ----------------- | -------- | --------- | ---------- | ------------ |
| // H Front Porch  | = 1.000; | // (usec) | = 5 chars  | = 40 Pixels  |
| Hor Sync Time     | = 3.200; | // (usec) | = 16 chars | = 128 Pixels |
| // H Back Porch   | = 2.200; | // (usec) | = 11 chars | = 88 Pixels  |
| // H Left Border  | = 0.000; | // (usec) | = 0 chars  | = 0 Pixels   |
Ver Total Time = 16.579; // (msec) = 628 lines HT – (1.06xHA)
| Ver Addr Time      | = 15.840; | // (msec) | = 600 lines | = 5.2 |
| ------------------ | --------- | --------- | ----------- | ----- |
| Ver Blank Start    | = 15.840; | // (msec) | = 600 lines |       |
| Ver Blank Time     | = 0.739;  | // (msec) | = 28 lines  |       |
| Ver Sync Start     | = 15.866; | // (msec) | = 601 lines |       |
| // V Bottom Border | = 0.000;  | // (msec) | = 0 lines   |       |
1
| // V Front Porch | = 0.026; | // (msec) | = lines    |     |
| ---------------- | -------- | --------- | ---------- | --- |
| Ver Sync Time    | = 0.106; | // (msec) | = 4 lines  |     |
| // V Back Porch  | = 0.607; | // (msec) | = 23 lines |     |
| // V Top Border  | = 0.000; | // (msec) | = 0 lines  |     |

Definition of Terms: Refer to section 3.1.

VESA MONITOR TIMING STANDARD
Adopted:  9/10/91 (VESA #900603A)
Resolution:  800 x 600 at 72 Hz (non-interlaced)
EDID ID:  DMT ID: 0Ah; Std. 2 Byte Code: (45, 4C)h; CVT 3 Byte Code: n/a
BIOS Modes:  102h, 103h, 113h, 114h, & 115h (4, 8, 15, 16, & 24 bpp)
Method:  *** NOT CVT COMPLIANT ***

Detailed Timing Parameters
800 x 600 @ 72Hz;
| Timing Name       | =                |           |                   |         |     |
| ----------------- | ---------------- | --------- | ----------------- | ------- | --- |
| Hor Pixels        | = 800;           | // Pixels |                   |         |     |
| Ver Pixels        | = 600;           | // Lines  |                   |         |     |
| Hor Frequency     | = 48.077;        | // kHz    | = 20.8 usec       | / line  |     |
| Ver Frequency     | = 72.188;        | // Hz     | = 13.9 msec       | / frame |     |
| Pixel Clock       | = 50.000;        | // MHz    | = 20.0 nsec       | ± 0.5%  |     |
| Character Width   | = 8;             | // Pixels | = 160.0 nsec      |         |     |
| Scan Type         | = NONINTERLACED; |           | // H Phase        | = 0.4 % |     |
| Hor Sync Polarity | = POSITIVE;      | // HBlank | = 23.1% of HTotal |         |     |
POSITIVE;
| Ver Sync Polarity | =         | // VBlank | = 9.9% of VTotal |               |     |
| ----------------- | --------- | --------- | ---------------- | ------------- | --- |
| Hor Total Time    | = 20.800; | // (usec) | = 130 chars      | = 1040 Pixels |     |
| Hor Addr Time     | = 16.000; | // (usec) | = 100 chars      | = 800 Pixels  |     |
| Hor Blank Start   | = 16.000; | // (usec) | = 100 chars      | = 800 Pixels  |     |
| Hor Blank Time    | = 4.800;  | // (usec) | = 30 chars       | = 240 Pixels  |     |
| Hor Sync Start    | = 17.120; | // (usec) | = 107 chars      | = 856 Pixels  |     |
| // H Right Border | = 0.000;  | // (usec) | = 0 chars        | = 0 Pixels    |     |
| // H Front Porch  | = 1.120;  | // (usec) | = 7 chars        | = 56 Pixels   |     |
| Hor Sync Time     | = 2.400;  | // (usec) | = 15 chars       | = 120 Pixels  |     |
| // H Back Porch   | = 1.280;  | // (usec) | = 8 chars        | = 64 Pixels   |     |
| // H Left Border  | = 0.000;  | // (usec) | = 0 chars        | = 0 Pixels    |     |
Ver Total Time = 13.853; // (msec) = 666 lines HT – (1.06xHA)
| Ver Addr Time      | = 12.480; | // (msec) | = 600 lines | = 3.84 |     |
| ------------------ | --------- | --------- | ----------- | ------ | --- |
| Ver Blank Start    | = 12.480; | // (msec) | = 600 lines |        |     |
| Ver Blank Time     | = 1.373;  | // (msec) | = 66 lines  |        |     |
| Ver Sync Start     | = 13.250; | // (msec) | = 637 lines |        |     |
| // V Bottom Border | = 0.000;  | // (msec) | = 0 lines   |        |     |
| // V Front Porch   | = 0.770;  | // (msec) | = 37 lines  |        |     |
| Ver Sync Time      | = 0.125;  | // (msec) | = 6 lines   |        |     |
| // V Back Porch    | = 0.478;  | // (msec) | = 23 lines  |        |     |
0
| // V Top Border | = 0.000; | // (msec) | = lines |     |     |
| --------------- | -------- | --------- | ------- | --- | --- |

Definition of Terms: Refer to section 3.1.

VESA MONITOR TIMING STANDARD
Adopted:  10/4/93
Resolution:  800 x 600 at 75 Hz (non-interlaced)
EDID ID:  DMT ID: 0Bh; Std. 2 Byte Code: (45, 4F)h; CVT 3 Byte Code: n/a
BIOS Modes:  102h, 103h, 113h, 114h, & 115h (4, 8, 15, 16, & 24 bpp)
Method:  *** NOT CVT COMPLIANT ***

Detailed Timing Parameters
800 x 600 @ 75Hz;
| Timing Name       | =                |           |                   |         |     |
| ----------------- | ---------------- | --------- | ----------------- | ------- | --- |
| Hor Pixels        | = 800;           | // Pixels |                   |         |     |
| Ver Pixels        | = 600;           | // Lines  |                   |         |     |
| Hor Frequency     | = 46.875;        | // kHz    | = 21.3 usec       | / line  |     |
| Ver Frequency     | = 75.000;        | // Hz     | = 13.3 msec       | / frame |     |
| Pixel Clock       | = 49.500;        | // MHz    | = 20.2 nsec       | ± 0.5%  |     |
| Character Width   | = 8;             | // Pixels | = 161.6 nsec      |         |     |
| Scan Type         | = NONINTERLACED; |           | // H Phase        | = 6.8 % |     |
| Hor Sync Polarity | = POSITIVE;      | // HBlank | = 24.2% of HTotal |         |     |
POSITIVE;
| Ver Sync Polarity | =         | // VBlank | = 4.0% of VTotal |               |     |
| ----------------- | --------- | --------- | ---------------- | ------------- | --- |
| Hor Total Time    | = 21.333; | // (usec) | = 132 chars      | = 1056 Pixels |     |
| Hor Addr Time     | = 16.162; | // (usec) | = 100 chars      | = 800 Pixels  |     |
| Hor Blank Start   | = 16.162; | // (usec) | = 100 chars      | = 800 Pixels  |     |
| Hor Blank Time    | = 5.172;  | // (usec) | = 32 chars       | = 256 Pixels  |     |
| Hor Sync Start    | = 16.485; | // (usec) | = 102 chars      | = 816 Pixels  |     |
| // H Right Border | = 0.000;  | // (usec) | = 0 chars        | = 0 Pixels    |     |
| // H Front Porch  | = 0.323;  | // (usec) | = 2 chars        | = 16 Pixels   |     |
| Hor Sync Time     | = 1.616;  | // (usec) | = 10 chars       | = 80 Pixels   |     |
| // H Back Porch   | = 3.232;  | // (usec) | = 20 chars       | = 160 Pixels  |     |
| // H Left Border  | = 0.000;  | // (usec) | = 0 chars        | = 0 Pixels    |     |
Ver Total Time = 13.333; // (msec) = 625 lines HT – (1.06xHA)
| Ver Addr Time      | = 12.800; | // (msec) | = 600 lines | = 4.2 |     |
| ------------------ | --------- | --------- | ----------- | ----- | --- |
| Ver Blank Start    | = 12.800; | // (msec) | = 600 lines |       |     |
| Ver Blank Time     | = 0.533;  | // (msec) | = 25 lines  |       |     |
| Ver Sync Start     | = 12.821; | // (msec) | = 601 lines |       |     |
| // V Bottom Border | = 0.000;  | // (msec) | = 0 lines   |       |     |
| // V Front Porch   | = 0.021;  | // (msec) | = 1 lines   |       |     |
| Ver Sync Time      | = 0.064;  | // (msec) | = 3 lines   |       |     |
| // V Back Porch    | = 0.448;  | // (msec) | = 21 lines  |       |     |
0
| // V Top Border | = 0.000; | // (msec) | = lines |     |     |
| --------------- | -------- | --------- | ------- | --- | --- |

Definition of Terms: Refer to section 3.1.

VESA MONITOR TIMING STANDARD

Adopted:  3/1/96
Resolution:  800 x 600 at 85 Hz (non-interlaced)
EDID ID:  DMT ID: 0Ch; Std. 2 Byte Code: (45, 59)h; CVT 3 Byte Code: n/a
Method:  *** NOT CVT COMPLIANT ***

Detailed Timing Parameters
| Timing Name   | = 800 x 600 @ 85Hz; |           |             |         |
| ------------- | ------------------- | --------- | ----------- | ------- |
| Hor Pixels    | = 800;              | // Pixels |             |         |
| Ver Pixels    | = 600;              | // Lines  |             |         |
| Hor Frequency | = 53.674;           | // kHz    | = 18.6 usec | / line  |
| Ver Frequency | = 85.061;           | // Hz     | = 11.8 msec | / frame |
| Pixel Clock   | = 56.250;           | // MHz    | = 17.8 nsec | ± 0.5%  |
8;
| Character Width   | =                | // Pixels | = 142.2 nsec      |               |
| ----------------- | ---------------- | --------- | ----------------- | ------------- |
| Scan Type         | = NONINTERLACED; |           | // H Phase        | = 5.7 %       |
| Hor Sync Polarity | = POSITIVE;      | // HBlank | = 23.7% of HTotal |               |
| Ver Sync Polarity | = POSITIVE;      | // VBlank | = 4.9% of VTotal  |               |
| Hor Total Time    | = 18.631;        | // (usec) | = 131 chars       | = 1048 Pixels |
| Hor Addr Time     | = 14.222;        | // (usec) | = 100 chars       | = 800 Pixels  |
| Hor Blank Start   | = 14.222;        | // (usec) | = 100 chars       | = 800 Pixels  |
| Hor Blank Time    | = 4.409;         | // (usec) | = 31 chars        | = 248 Pixels  |
| Hor Sync Start    | = 14.791;        | // (usec) | = 104 chars       | = 832 Pixels  |
| // H Right Border | = 0.000;         | // (usec) | = 0 chars         | = 0 Pixels    |
| // H Front Porch  | = 0.569;         | // (usec) | = 4 chars         | = 32 Pixels   |
| Hor Sync Time     | = 1.138;         | // (usec) | = 8 chars         | = 64 Pixels   |
| // H Back Porch   | = 2.702;         | // (usec) | = 19 chars        | = 152 Pixels  |
| // H Left Border  | = 0.000;         | // (usec) | = 0 chars         | = 0 Pixels    |
Ver Total Time = 11.756; // (msec) = 631 lines HT – (1.06xHA)
| Ver Addr Time      | = 11.179; | // (msec) | = 600 lines | = 3.56 |
| ------------------ | --------- | --------- | ----------- | ------ |
| Ver Blank Start    | = 11.179; | // (msec) | = 600 lines |        |
| Ver Blank Time     | = 0.578;  | // (msec) | = 31 lines  |        |
| Ver Sync Start     | = 11.197; | // (msec) | = 601 lines |        |
| // V Bottom Border | = 0.000;  | // (msec) | = 0 lines   |        |
1
| // V Front Porch | = 0.019; | // (msec) | = lines    |     |
| ---------------- | -------- | --------- | ---------- | --- |
| Ver Sync Time    | = 0.056; | // (msec) | = 3 lines  |     |
| // V Back Porch  | = 0.503; | // (msec) | = 27 lines |     |
| // V Top Border  | = 0.000; | // (msec) | = 0 lines  |     |

Definition of Terms: Refer to section 3.1.

VESA MONITOR TIMING STANDARD
Adopted:  5/1/07
Resolution:  800 x 600 at 120 Hz (non-interlaced) REDUCED BLANKING
EDID ID:  DMT ID: 0Dh; Std. 2 Byte Code: n/a; CVT 3 Byte Code: n/a
Method:  Generated using CVT (Reduced Blanking) Formula

Detailed Timing Parameters
| Timing Name       | = 800 x 600 @ 120Hz CVT (Reduced Blanking); |           |            |           |        |
| ----------------- | ------------------------------------------- | --------- | ---------- | --------- | ------ |
| Hor Pixels        | = 800;                                      | // Pixels |            |           |        |
| Ver Pixels        | = 600;                                      | // Lines  |            |           |        |
| Hor Frequency     | = 76.302;                                   | // kHz    | = 13.1     | usec /    | line   |
| Ver Frequency     | = 119.972;                                  | // Hz     | = 8.3      | msec /    | frame  |
| Pixel Clock       | = 73.250;                                   | // MHz    | = 13.7     | nsec      | ± 0.5% |
| Character Width   | = 8;                                        | // Pixels | = 109.2    | nsec      |        |
| Scan Type         | = NONINTERLACED;                            |           | // H Phase | =         | 1.7 %  |
| Hor Sync Polarity | = POSITIVE;                                 | // HBlank | = 16.7%    | of HTotal |        |
NEGATIVE
| Ver Sync Polarity | =         | // VBlank | = 5.7% | of VTotal  |            |
| ----------------- | --------- | --------- | ------ | ---------- | ---------- |
| Hor Total Time    | = 13.106; | // (usec) | = 120  | chars =    | 960 Pixels |
| Hor Addr Time     | = 10.922; | // (usec) | = 100  | chars =    | 800 Pixels |
| Hor Blank Start   | = 10.922; | // (usec) | = 100  | chars =    | 800 Pixels |
| Hor Blank Time    | = 2.184;  | // (usec) | =      | 20 chars = | 160 Pixels |
| Hor Sync Start    | = 11.577; | // (usec) | = 106  | chars =    | 848 Pixels |
| // H Right Border | = 0.000;  | // (usec) | =      | 0 chars =  | 0 Pixels   |
6
| // H Front Porch | = 0.655; | // (usec) | =     | chars =   | 48 Pixels      |
| ---------------- | -------- | --------- | ----- | --------- | -------------- |
| Hor Sync Time    | = 0.437; | // (usec) | =     | 4 chars = | 32 Pixels      |
| // H Back Porch  | = 1.092; | // (usec) | = 10  | chars =   | 80 Pixels      |
| // H Left Border | = 0.000; | // (usec) | =     | 0 chars = | 0 Pixels       |
| Ver Total Time   | = 8.335; | // (msec) | = 636 | lines     | HT – (1.06xHA) |
| Ver Addr Time    | = 7.863; | // (msec) | = 600 | lines     | = 1.53         |
| Ver Blank Start  | = 7.863; | // (msec) | = 600 | lines     |                |
| Ver Blank Time   | = 0.472; | // (msec) | =     | 36 lines  |                |
| Ver Sync Start   | = 7.903; | // (msec) | = 603 | lines     |                |
0
| // V Bottom Border | = 0.000; | // (msec) | =    | lines   |     |
| ------------------ | -------- | --------- | ---- | ------- | --- |
| // V Front Porch   | = 0.039; | // (msec) | =    | 3 lines |     |
| Ver Sync Time      | = 0.052; | // (msec) | =    | 4 lines |     |
| // V Back Porch    | = 0.380; | // (msec) | = 29 | lines   |     |
| // V Top Border    | = 0.000; | // (msec) | =    | 0 lines |     |

Definition of Terms: Refer to section 3.2.

VESA MONITOR TIMING STANDARD
Adopted:  8/21/03
Resolution:  848 x 480 at 60 Hz (non-interlaced)
EDID ID:  DMT ID: 0Eh; Std. 2 Byte Code: n/a; CVT 3 Byte Code: n/a
Method:  *** NOT CVT COMPLIANT ***

Detailed Timing Parameters
| Timing Name | = 848 x 480 @ 60Hz; |     |     |     |     |
| ----------- | ------------------- | --- | --- | --- | --- |
848;
| Hor Pixels        | =                | // Pixels |            |           |        |
| ----------------- | ---------------- | --------- | ---------- | --------- | ------ |
| Ver Pixels        | = 480;           | // Lines  |            |           |        |
| Hor Frequency     | = 31.020;        | // kHz    | = 32.2     | usec /    | line   |
| Ver Frequency     | = 60.000;        | // Hz     | = 16.7     | msec /    | frame  |
| Pixel Clock       | = 33.750;        | // MHz    | = 29.6     | nsec      | ± 0.5% |
| Character Width   | = 8;             | // Pixels | = 237.0    | nsec      |        |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =         | 4.4 %  |
| Hor Sync Polarity | = POSITIVE;      | // HBlank | = 22.1%    | of HTotal |        |
POSITIVE;
| Ver Sync Polarity | =         | // VBlank | = 7.2% | of VTotal  |             |
| ----------------- | --------- | --------- | ------ | ---------- | ----------- |
| Hor Total Time    | = 32.237; | // (usec) | = 136  | chars =    | 1088 Pixels |
| Hor Addr Time     | = 25.126; | // (usec) | = 106  | chars =    | 848 Pixels  |
| Hor Blank Start   | = 25.126; | // (usec) | = 106  | chars =    | 848 Pixels  |
| Hor Blank Time    | = 7.111;  | // (usec) | =      | 30 chars = | 240 Pixels  |
| Hor Sync Start    | = 25.600; | // (usec) | = 108  | chars =    | 864 Pixels  |
| // H Right Border | = 0.000;  | // (usec) | =      | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.474;  | // (usec) | =      | 2 chars =  | 16 Pixels   |
| Hor Sync Time     | = 3.319;  | // (usec) | =      | 14 chars = | 112 Pixels  |
| // H Back Porch   | = 3.319;  | // (usec) | =      | 14 chars = | 112 Pixels  |
| // H Left Border  | = 0.000;  | // (usec) | =      | 0 chars =  | 0 Pixels    |
Ver Total Time = 16.667; // (msec) = 517 lines HT – (1.06xHA)
| Ver Addr Time      | = 15.474; | // (msec) | = 480 | lines    | = 5.6 |
| ------------------ | --------- | --------- | ----- | -------- | ----- |
| Ver Blank Start    | = 15.474; | // (msec) | = 480 | lines    |       |
| Ver Blank Time     | = 1.193;  | // (msec) | =     | 37 lines |       |
| Ver Sync Start     | = 15.667; | // (msec) | = 486 | lines    |       |
| // V Bottom Border | = 0.000;  | // (msec) | =     | 0 lines  |       |
| // V Front Porch   | = 0.193;  | // (msec) | =     | 6 lines  |       |
| Ver Sync Time      | = 0.258;  | // (msec) | =     | 8 lines  |       |
23
| // V Back Porch | = 0.741; | // (msec) | =   | lines   |     |
| --------------- | -------- | --------- | --- | ------- | --- |
| // V Top Border | = 0.000; | // (msec) | =   | 0 lines |     |

Definition of Terms: Refer to section 3.1.

VESA MONITOR TIMING STANDARD
Adopted:  n/a      ** For Reference Only - Not a VESA Standard **
Resolution:  1024 x 768 at 43 Hz (interlaced)
EDID ID:  DMT ID: 0Fh; Std. 2 Byte Code: n/a; CVT 3 Byte Code: n/a
BIOS Modes:  104h, 105h, 116h, 117h, & 118h (4, 8, 15, 16, & 24 bpp)
Method:  *** NOT CVT COMPLIANT ***

Detailed Timing Parameters
1024 x 768 @ 43Hz (Interlaced);
| Timing Name   | =         |           |        |        |      |
| ------------- | --------- | --------- | ------ | ------ | ---- |
| Hor Pixels    | = 1024;   | // Pixels |        |        |      |
| Ver Pixels    | = 768;    | // Lines  |        |        |      |
| Hor Frequency | = 35.522; | // kHz    | = 28.2 | usec / | line |
field
| Ver Frequency | = 86.957; | // Hz | = 11.5 | msec / |     |
| ------------- | --------- | ----- | ------ | ------ | --- |
44.900;
| Pixel Clock       | =             | // MHz    | = 22.3  | nsec        | ± 0.5%      |
| ----------------- | ------------- | --------- | ------- | ----------- | ----------- |
| Character Width   | = 8;          | // Pixels | = 178.2 | nsec        |             |
| Scan Type         | = INTERLACED; |           |         |             |             |
| Hor Sync Polarity | = POSITIVE;   | // HBlank | = 19.0% | of HTotal   |             |
| Ver Sync Polarity | = POSITIVE;   | // VBlank | = 5.9%  | of VTotal   |             |
| Hor Total Time    | = 28.151;     | // (usec) | =       | 158 chars = | 1264 Pixels |
| Hor Addr Time     | = 22.806;     | // (usec) | =       | 128 chars = | 1024 Pixels |
Hor Blank Start = 22.806; // (usec) = 128 chars = 1024 Pixels
| Hor Blank Time    | = 5.345;  | // (usec) | =   | 30 chars =  | 240 Pixels  |
| ----------------- | --------- | --------- | --- | ----------- | ----------- |
| Hor Sync Start    | = 22.984; | // (usec) | =   | 129 chars = | 1032 Pixels |
| // H Right Border | = 0.000;  | // (usec) | =   | 0 chars =   | 0 Pixels    |
| // H Front Porch  | = 0.178;  | // (usec) | =   | 1 chars =   | 8 Pixels    |
| Hor Sync Time     | = 3.920;  | // (usec) | =   | 22 chars =  | 176 Pixels  |
7
| // H Back Porch    | = 1.247;  | // (usec) | =   | chars =               | 56 Pixels |
| ------------------ | --------- | --------- | --- | --------------------- | --------- |
| // H Left Border   | = 0.000;  | // (usec) | =   | 0 chars =             | 0 Pixels  |
| Ver Total Time     | = 23.000; | // (msec) | =   | 817 lines (Per Frame) |           |
| Ver Addr Time      | = 21.620; | // (msec) | =   | 768 lines (Per Frame) |           |
| Ver Blank Start    | = 21.620; | // (msec) | =   | 768 lines (Per Frame) |           |
| Ver Blank Time     | = 0.676;  | // (msec) | =   | 24 lines (Per Field)  |           |
| Ver Sync Start     | = 21.620; | // (msec) | =   | 768 lines (Per Frame) |           |
| // V Bottom Border | = 0.000;  | // (msec) | =   | 0 lines (Odd Field)   |           |
| // V Front Porch   | = 0.000;  | // (msec) | =   | 0 lines (Odd Field)   |           |
| Ver Sync Time      | = 0.113;  | // (msec) | =   | 4 lines (Both Fields) |           |
| // V Back Porch    | = 0.563;  | // (msec) | =   | 20 lines (Odd Field)  |           |
| // V Top Border    | = 0.000;  | // (msec) | =   | 0 lines (Odd Field)   |           |

Definition of Terms: Refer to section 3.1.

VESA MONITOR TIMING STANDARD
Adopted:  9/10/91 (VESA #901101A)
Resolution:  1024 x 768 at 60 Hz (non-interlaced)
EDID ID:  DMT ID: 10h; Std. 2 Byte Code: (61, 40)h; CVT 3 Byte Code: n/a
BIOS Modes:  104h, 105h, 116h, 117h, & 118h (4, 8, 15, 16, & 24 bpp)
Method:  *** NOT CVT COMPLIANT ***

Detailed Timing Parameters
| Timing Name   | = 1024 x 768 @ 60Hz; |           |             |         |
| ------------- | -------------------- | --------- | ----------- | ------- |
| Hor Pixels    | = 1024;              | // Pixels |             |         |
| Ver Pixels    | = 768;               | // Lines  |             |         |
| Hor Frequency | = 48.363;            | // kHz    | = 20.7 usec | / line  |
| Ver Frequency | = 60.004;            | // Hz     | = 16.7 msec | / frame |
| Pixel Clock   | = 65.000;            | // MHz    | = 15.4 nsec | ± 0.5%  |
8;
| Character Width   | =                | // Pixels | = 123.1 nsec      |               |
| ----------------- | ---------------- | --------- | ----------------- | ------------- |
| Scan Type         | = NONINTERLACED; |           | // H Phase        | = 5.1 %       |
| Hor Sync Polarity | = NEGATIVE;      | // HBlank | = 23.8% of HTotal |               |
| Ver Sync Polarity | = NEGATIVE;      | // VBlank | = 4.7% of VTotal  |               |
| Hor Total Time    | = 20.677;        | // (usec) | = 168 chars       | = 1344 Pixels |
| Hor Addr Time     | = 15.754;        | // (usec) | = 128 chars       | = 1024 Pixels |
Hor Blank Start = 15.754; // (usec) = 128 chars = 1024 Pixels
| Hor Blank Time | = 4.923;  | // (usec) | = 40 chars  | = 320 Pixels  |
| -------------- | --------- | --------- | ----------- | ------------- |
| Hor Sync Start | = 16.123; | // (usec) | = 131 chars | = 1048 Pixels |
0
| // H Right Border | = 0.000; | // (usec) | = chars    | = 0 Pixels   |
| ----------------- | -------- | --------- | ---------- | ------------ |
| // H Front Porch  | = 0.369; | // (usec) | = 3 chars  | = 24 Pixels  |
| Hor Sync Time     | = 2.092; | // (usec) | = 17 chars | = 136 Pixels |
| // H Back Porch   | = 2.462; | // (usec) | = 20 chars | = 160 Pixels |
| // H Left Border  | = 0.000; | // (usec) | = 0 chars  | = 0 Pixels   |
Ver Total Time = 16.666; // (msec) = 806 lines HT – (1.06xHA)
| Ver Addr Time      | = 15.880; | // (msec) | = 768 lines | = 3.98 |
| ------------------ | --------- | --------- | ----------- | ------ |
| Ver Blank Start    | = 15.880; | // (msec) | = 768 lines |        |
| Ver Blank Time     | = 0.786;  | // (msec) | = 38 lines  |        |
| Ver Sync Start     | = 15.942; | // (msec) | = 771 lines |        |
| // V Bottom Border | = 0.000;  | // (msec) | = 0 lines   |        |
3
| // V Front Porch | = 0.062; | // (msec) | = lines    |     |
| ---------------- | -------- | --------- | ---------- | --- |
| Ver Sync Time    | = 0.124; | // (msec) | = 6 lines  |     |
| // V Back Porch  | = 0.600; | // (msec) | = 29 lines |     |
| // V Top Border  | = 0.000; | // (msec) | = 0 lines  |     |

Definition of Terms: Refer to section 3.3.

VESA MONITOR TIMING STANDARD
Adopted:  8/9/91 (VESA #910801-2)
Resolution:  1024 x 768 at 70 Hz (non-interlaced)
EDID ID:  DMT ID: 11h; Std. 2 Byte Code: (61, 4A)h; CVT 3 Byte Code: n/a
BIOS Modes:  104h, 105h, 116h, 117h, & 118h (4, 8, 15, 16, & 24 bpp)
Method:  *** NOT CVT COMPLIANT ***

Detailed Timing Parameters
1024 x 768 @ 70Hz;
| Timing Name       | =                |           |                   |         |     |
| ----------------- | ---------------- | --------- | ----------------- | ------- | --- |
| Hor Pixels        | = 1024;          | // Pixels |                   |         |     |
| Ver Pixels        | = 768;           | // Lines  |                   |         |     |
| Hor Frequency     | = 56.476;        | // kHz    | = 17.7 usec       | / line  |     |
| Ver Frequency     | = 70.069;        | // Hz     | = 14.3 msec       | / frame |     |
| Pixel Clock       | = 75.000;        | // MHz    | = 13.3 nsec       | ± 0.5%  |     |
| Character Width   | = 8;             | // Pixels | = 106.7 nsec      |         |     |
| Scan Type         | = NONINTERLACED; |           | // H Phase        | = 4.5 % |     |
| Hor Sync Polarity | = NEGATIVE;      | // HBlank | = 22.9% of HTotal |         |     |
NEGATIVE;
| Ver Sync Polarity | =         | // VBlank | = 4.7% of VTotal |               |     |
| ----------------- | --------- | --------- | ---------------- | ------------- | --- |
| Hor Total Time    | = 17.707; | // (usec) | = 166 chars      | = 1328 Pixels |     |
| Hor Addr Time     | = 13.653; | // (usec) | = 128 chars      | = 1024 Pixels |     |
Hor Blank Start = 13.653; // (usec) = 128 chars = 1024 Pixels
| Hor Blank Time    | = 4.053;  | // (usec) | = 38 chars  | = 304 Pixels  |     |
| ----------------- | --------- | --------- | ----------- | ------------- | --- |
| Hor Sync Start    | = 13.973; | // (usec) | = 131 chars | = 1048 Pixels |     |
| // H Right Border | = 0.000;  | // (usec) | = 0 chars   | = 0 Pixels    |     |
| // H Front Porch  | = 0.320;  | // (usec) | = 3 chars   | = 24 Pixels   |     |
| Hor Sync Time     | = 1.813;  | // (usec) | = 17 chars  | = 136 Pixels  |     |
| // H Back Porch   | = 1.920;  | // (usec) | = 18 chars  | = 144 Pixels  |     |
| // H Left Border  | = 0.000;  | // (usec) | = 0 chars   | = 0 Pixels    |     |
Ver Total Time = 14.272; // (msec) = 806 lines HT – (1.06xHA)
| Ver Addr Time      | = 13.599; | // (msec) | = 768 lines | = 3.23 |     |
| ------------------ | --------- | --------- | ----------- | ------ | --- |
| Ver Blank Start    | = 13.599; | // (msec) | = 768 lines |        |     |
| Ver Blank Time     | = 0.673;  | // (msec) | = 38 lines  |        |     |
| Ver Sync Start     | = 13.652; | // (msec) | = 771 lines |        |     |
| // V Bottom Border | = 0.000;  | // (msec) | = 0 lines   |        |     |
| // V Front Porch   | = 0.053;  | // (msec) | = 3 lines   |        |     |
| Ver Sync Time      | = 0.106;  | // (msec) | = 6 lines   |        |     |
| // V Back Porch    | = 0.513;  | // (msec) | = 29 lines  |        |     |
0
| // V Top Border | = 0.000; | // (msec) | = lines |     |     |
| --------------- | -------- | --------- | ------- | --- | --- |

Definition of Terms: Refer to section 3.3.

VESA MONITOR TIMING STANDARD
Adopted:  10/4/93
Resolution:  1024 x 768 at 75 Hz (non-interlaced)
EDID ID:  DMT ID: 12h; Std. 2 Byte Code: (61, 4F)h; CVT 3 Byte Code: n/a
BIOS Modes:  104h, 105h, 116h, 117h, & 118h (4, 8, 15, 16, & 24 bpp)
Method:  *** NOT CVT COMPLIANT ***

Detailed Timing Parameters
1024 x 768 @ 75Hz;
| Timing Name       | =                |           |                   |               |
| ----------------- | ---------------- | --------- | ----------------- | ------------- |
| Hor Pixels        | = 1024;          | // Pixels |                   |               |
| Ver Pixels        | = 768;           | // Lines  |                   |               |
| Hor Frequency     | = 60.023;        | // kHz    | = 16.7 usec       | / line        |
| Ver Frequency     | = 75.029;        | // Hz     | = 13.3 msec       | / frame       |
| Pixel Clock       | = 78.750;        | // MHz    | = 12.7 nsec       | ± 0.5%        |
| Character Width   | = 8;             | // Pixels | = 101.6 nsec      |               |
| Scan Type         | = NONINTERLACED; |           | // H Phase        | = 6.1 %       |
| Hor Sync Polarity | = POSITIVE;      | // HBlank | = 22.0% of HTotal |               |
| Ver Sync Polarity | = POSITIVE;      | // VBlank | = 4.0% of VTotal  |               |
| Hor Total Time    | = 16.660;        | // (usec) | = 164 chars       | = 1312 Pixels |
| Hor Addr Time     | = 13.003;        | // (usec) | = 128 chars       | = 1024 Pixels |
Hor Blank Start = 13.003; // (usec) = 128 chars = 1024 Pixels
| Hor Blank Time    | = 3.657;  | // (usec) | = 36 chars  | = 288 Pixels  |
| ----------------- | --------- | --------- | ----------- | ------------- |
| Hor Sync Start    | = 13.206; | // (usec) | = 130 chars | = 1040 Pixels |
| // H Right Border | = 0.000;  | // (usec) | = 0 chars   | = 0 Pixels    |
| // H Front Porch  | = 0.203;  | // (usec) | = 2 chars   | = 16 Pixels   |
| Hor Sync Time     | = 1.219;  | // (usec) | = 12 chars  | = 96 Pixels   |
| // H Back Porch   | = 2.235;  | // (usec) | = 22 chars  | = 176 Pixels  |
| // H Left Border  | = 0.000;  | // (usec) | = 0 chars   | = 0 Pixels    |
Ver Total Time = 13.328; // (msec) = 800 lines HT – (1.06xHA)
| Ver Addr Time      | = 12.795; | // (msec) | = 768 lines | = 2.88 |
| ------------------ | --------- | --------- | ----------- | ------ |
| Ver Blank Start    | = 12.795; | // (msec) | = 768 lines |        |
| Ver Blank Time     | = 0.533;  | // (msec) | = 32 lines  |        |
| Ver Sync Start     | = 12.812; | // (msec) | = 769 lines |        |
| // V Bottom Border | = 0.000;  | // (msec) | = 0 lines   |        |
| // V Front Porch   | = 0.017;  | // (msec) | = 1 lines   |        |
| Ver Sync Time      | = 0.050;  | // (msec) | = 3 lines   |        |
| // V Back Porch    | = 0.466;  | // (msec) | = 28 lines  |        |
| // V Top Border    | = 0.000;  | // (msec) | = 0 lines   |        |

Definition of Terms: Refer to section 3.1.

VESA MONITOR TIMING STANDARD
Adopted:  3/1/96
Resolution:  1024 x 768 at 85 Hz (non-interlaced)
EDID ID:  DMT ID: 13h; Std. 2 Byte Code: (61, 59)h; CVT 3 Byte Code: n/a
Method:  *** NOT CVT COMPLIANT ***

Detailed Timing Parameters
| Timing Name   | = 1024 x 768 @ 85Hz; |           |             |         |
| ------------- | -------------------- | --------- | ----------- | ------- |
| Hor Pixels    | = 1024;              | // Pixels |             |         |
| Ver Pixels    | = 768;               | // Lines  |             |         |
| Hor Frequency | = 68.677;            | // kHz    | = 14.6 usec | / line  |
| Ver Frequency | = 84.997;            | // Hz     | = 11.8 msec | / frame |
94.500;
| Pixel Clock       | =                | // MHz    | = 10.6 nsec       | ± 0.5%        |
| ----------------- | ---------------- | --------- | ----------------- | ------------- |
| Character Width   | = 8;             | // Pixels | = 84.7 nsec       |               |
| Scan Type         | = NONINTERLACED; |           | // H Phase        | = 5.8 %       |
| Hor Sync Polarity | = POSITIVE;      | // HBlank | = 25.6% of HTotal |               |
| Ver Sync Polarity | = POSITIVE;      | // VBlank | = 5.0% of VTotal  |               |
| Hor Total Time    | = 14.561;        | // (usec) | = 172 chars       | = 1376 Pixels |
| Hor Addr Time     | = 10.836;        | // (usec) | = 128 chars       | = 1024 Pixels |
Hor Blank Start = 10.836; // (usec) = 128 chars = 1024 Pixels
| Hor Blank Time    | = 3.725;  | // (usec) | = 44 chars  | = 352 Pixels  |
| ----------------- | --------- | --------- | ----------- | ------------- |
| Hor Sync Start    | = 11.344; | // (usec) | = 134 chars | = 1072 Pixels |
| // H Right Border | = 0.000;  | // (usec) | = 0 chars   | = 0 Pixels    |
| // H Front Porch  | = 0.508;  | // (usec) | = 6 chars   | = 48 Pixels   |
| Hor Sync Time     | = 1.016;  | // (usec) | = 12 chars  | = 96 Pixels   |
| // H Back Porch   | = 2.201;  | // (usec) | = 26 chars  | = 208 Pixels  |
| // H Left Border  | = 0.000;  | // (usec) | = 0 chars   | = 0 Pixels    |
Ver Total Time = 11.765; // (msec) = 808 lines HT – (1.06xHA)
| Ver Addr Time   | = 11.183; | // (msec) | = 768 lines | = 3.07 |
| --------------- | --------- | --------- | ----------- | ------ |
| Ver Blank Start | = 11.183; | // (msec) | = 768 lines |        |
| Ver Blank Time  | = 0.582;  | // (msec) | = 40 lines  |        |
| Ver Sync Start  | = 11.197; | // (msec) | = 769 lines |        |
0
| // V Bottom Border | = 0.000; | // (msec) | = lines    |     |
| ------------------ | -------- | --------- | ---------- | --- |
| // V Front Porch   | = 0.015; | // (msec) | = 1 lines  |     |
| Ver Sync Time      | = 0.044; | // (msec) | = 3 lines  |     |
| // V Back Porch    | = 0.524; | // (msec) | = 36 lines |     |
| // V Top Border    | = 0.000; | // (msec) | = 0 lines  |     |

Definition of Terms: Refer to section 3.1.

VESA MONITOR TIMING STANDARD
Adopted:  5/1/07
Resolution:  1024 x 768 at 120 Hz (non-interlaced) REDUCED BLANKING
EDID ID:  DMT ID: 14h; Std. 2 Byte Code: n/a; CVT 3 Byte Code: n/a
Method:  Generated using CVT (Reduced Blanking) Formula

Detailed Timing Parameters
| Timing Name | = 1024 x 768 @ 120Hz CVT (Reduced Blanking); |           |     |     |     |
| ----------- | -------------------------------------------- | --------- | --- | --- | --- |
| Hor Pixels  | = 1024;                                      | // Pixels |     |     |     |
768;
| Ver Pixels    | =          | // Lines |        |        |        |
| ------------- | ---------- | -------- | ------ | ------ | ------ |
| Hor Frequency | = 97.551;  | // kHz   | = 10.3 | usec / | line   |
| Ver Frequency | = 119.989; | // Hz    | = 8.3  | msec / | frame  |
| Pixel Clock   | = 115.500; | // MHz   | = 8.7  | nsec   | ± 0.5% |
8;
| Character Width   | =                | // Pixels | = 69.3     | nsec       |             |
| ----------------- | ---------------- | --------- | ---------- | ---------- | ----------- |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =          | 1.4 %       |
| Hor Sync Polarity | = POSITIVE;      | // HBlank | = 13.5%    | of HTotal  |             |
| Ver Sync Polarity | = NEGATIVE       | // VBlank | = 5.5%     | of VTotal  |             |
| Hor Total Time    | = 10.251;        | // (usec) | = 148      | chars =    | 1184 Pixels |
| Hor Addr Time     | = 8.866;         | // (usec) | = 128      | chars =    | 1024 Pixels |
| Hor Blank Start   | = 8.866;         | // (usec) | = 128      | chars =    | 1024 Pixels |
| Hor Blank Time    | = 1.385;         | // (usec) | =          | 20 chars = | 160 Pixels  |
| Hor Sync Start    | = 9.281;         | // (usec) | = 134      | chars =    | 1072 Pixels |
| // H Right Border | = 0.000;         | // (usec) | =          | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.416;         | // (usec) | =          | 6 chars =  | 48 Pixels   |
| Hor Sync Time     | = 0.277;         | // (usec) | =          | 4 chars =  | 32 Pixels   |
10
| // H Back Porch    | = 0.693; | // (usec) | =     | chars =   | 80 Pixels      |
| ------------------ | -------- | --------- | ----- | --------- | -------------- |
| // H Left Border   | = 0.000; | // (usec) | =     | 0 chars = | 0 Pixels       |
| Ver Total Time     | = 8.334; | // (msec) | = 813 | lines     | HT – (1.06xHA) |
| Ver Addr Time      | = 7.873; | // (msec) | = 768 | lines     | = 0.85         |
| Ver Blank Start    | = 7.873; | // (msec) | = 768 | lines     |                |
| Ver Blank Time     | = 0.461; | // (msec) | =     | 45 lines  |                |
| Ver Sync Start     | = 7.904; | // (msec) | = 771 | lines     |                |
| // V Bottom Border | = 0.000; | // (msec) | =     | 0 lines   |                |
| // V Front Porch   | = 0.031; | // (msec) | =     | 3 lines   |                |
| Ver Sync Time      | = 0.041; | // (msec) | =     | 4 lines   |                |
| // V Back Porch    | = 0.390; | // (msec) | = 38  | lines     |                |
| // V Top Border    | = 0.000; | // (msec) | =     | 0 lines   |                |

Definition of Terms: Refer to section 3.2.

VESA MONITOR TIMING STANDARD
Adopted:  3/1/96
Resolution:  1152 x 864 at 75 Hz (non-interlaced)
EDID ID:  DMT ID: 15h; Std. 2 Byte Code: (71, 4F)h; CVT 3 Byte Code: n/a
Method:  *** NOT CVT COMPLIANT ***

Detailed Timing Parameters
| Timing Name   | = 1152 x 864 @ 75Hz; |           |             |         |
| ------------- | -------------------- | --------- | ----------- | ------- |
| Hor Pixels    | = 1152;              | // Pixels |             |         |
| Ver Pixels    | = 864;               | // Lines  |             |         |
| Hor Frequency | = 67.500;            | // kHz    | = 14.8 usec | / line  |
| Ver Frequency | = 75.000;            | // Hz     | = 13.3 msec | / frame |
108.000;
| Pixel Clock       | =                | // MHz    | = 9.3 nsec        | ± 0.5%        |
| ----------------- | ---------------- | --------- | ----------------- | ------------- |
| Character Width   | = 8;             | // Pixels | = 74.1 nsec       |               |
| Scan Type         | = NONINTERLACED; |           | // H Phase        | = 6.0 %       |
| Hor Sync Polarity | = POSITIVE;      | // HBlank | = 28.0% of HTotal |               |
| Ver Sync Polarity | = POSITIVE;      | // VBlank | = 4.0% of VTotal  |               |
| Hor Total Time    | = 14.815;        | // (usec) | = 200 chars       | = 1600 Pixels |
| Hor Addr Time     | = 10.667;        | // (usec) | = 144 chars       | = 1152 Pixels |
Hor Blank Start = 10.667; // (usec) = 144 chars = 1152 Pixels
| Hor Blank Time    | = 4.148;  | // (usec) | = 56 chars  | = 448 Pixels  |
| ----------------- | --------- | --------- | ----------- | ------------- |
| Hor Sync Start    | = 11.259; | // (usec) | = 152 chars | = 1216 Pixels |
| // H Right Border | = 0.000;  | // (usec) | = 0 chars   | = 0 Pixels    |
| // H Front Porch  | = 0.593;  | // (usec) | = 8 chars   | = 64 Pixels   |
| Hor Sync Time     | = 1.185;  | // (usec) | = 16 chars  | = 128 Pixels  |
| // H Back Porch   | = 2.370;  | // (usec) | = 32 chars  | = 256 Pixels  |
| // H Left Border  | = 0.000;  | // (usec) | = 0 chars   | = 0 Pixels    |
Ver Total Time = 13.333; // (msec) = 900 lines HT – (1.06xHA)
| Ver Addr Time   | = 12.800; | // (msec) | = 864 lines | = 3.51 |
| --------------- | --------- | --------- | ----------- | ------ |
| Ver Blank Start | = 12.800; | // (msec) | = 864 lines |        |
| Ver Blank Time  | = 0.533;  | // (msec) | = 36 lines  |        |
| Ver Sync Start  | = 12.815; | // (msec) | = 865 lines |        |
0
| // V Bottom Border | = 0.000; | // (msec) | = lines    |     |
| ------------------ | -------- | --------- | ---------- | --- |
| // V Front Porch   | = 0.015; | // (msec) | = 1 lines  |     |
| Ver Sync Time      | = 0.044; | // (msec) | = 3 lines  |     |
| // V Back Porch    | = 0.474; | // (msec) | = 32 lines |     |
| // V Top Border    | = 0.000; | // (msec) | = 0 lines  |     |

Definition of Terms: Refer to section 3.1.
VESA MONITOR TIMING STANDARD
Adopted:  11/17/08
Resolution:  1280 x 720 at 60 Hz (non-interlaced)
EDID ID:  DMT ID: 55h; Std. 2 Byte Code: 81h, C0h; CVT 3 Byte Code: n/a
Method:  *** NOT CVT COMPLIANT ***
                           Per CEA-861 --- 720p (Code 4) Timing Definitions

Detailed Timing Parameters
| Timing Name | = 1280 x 720 @ 60Hz; |     |     |     |     |
| ----------- | -------------------- | --- | --- | --- | --- |
1280;
| Hor Pixels        | =                | // Pixels |            |           |        |
| ----------------- | ---------------- | --------- | ---------- | --------- | ------ |
| Ver Pixels        | = 720;           | // Lines  |            |           |        |
| Hor Frequency     | = 45.000;        | // KHz    | = 22.2     | usec /    | line   |
| Ver Frequency     | = 60.000;        | // Hz     | = 16.7     | msec /    | frame  |
| Pixel Clock       | = 74.250;        | // MHz    | = 13.5     | nsec      | ± 0.5% |
| Character Width   | = 1;             | // Pixels | = 13.5     | nsec      |        |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =         | 3.3 %  |
| Hor Sync Polarity | = POSITIVE;      | // HBlank | = 22.4%    | of HTotal |        |
| Ver Sync Polarity | = POSITIVE;      | // VBlank | = 4.0%     | of VTotal |        |
Hor Total Time = 22.222; // (usec) = 1650 chars = 1650 Pixels
| Hor Addr Time | = 17.239; | // (usec) | = 1280 | chars = | 1280 Pixels |
| ------------- | --------- | --------- | ------ | ------- | ----------- |
Hor Blank Start = 17.239; // (usec) = 1280 chars = 1280 Pixels
| Hor Blank Time | = 4.983; | // (usec) | = 370 | chars = | 370 Pixels |
| -------------- | -------- | --------- | ----- | ------- | ---------- |
Hor Sync Start = 18.721; // (usec) = 1390 chars = 1390 Pixels
0
| // H Right Border | = 0.000; | // (usec) | =     | chars = | 0 Pixels   |
| ----------------- | -------- | --------- | ----- | ------- | ---------- |
| // H Front Porch  | = 1.481; | // (usec) | = 110 | chars = | 110 Pixels |
40
| Hor Sync Time    | = 0.539; | // (usec) | =     | chars =   | 40 Pixels  |
| ---------------- | -------- | --------- | ----- | --------- | ---------- |
| // H Back Porch  | = 2.963; | // (usec) | = 220 | chars =   | 220 Pixels |
| // H Left Border | = 0.000; | // (usec) | =     | 0 chars = | 0 Pixels   |
Ver Total Time = 16.667; // (msec) = 750 lines HT – (1.06xHA)
| Ver Addr Time      | = 16.000; | // (msec) | = 720 | lines    | = 3.95 |
| ------------------ | --------- | --------- | ----- | -------- | ------ |
| Ver Blank Start    | = 16.000; | // (msec) | = 720 | lines    |        |
| Ver Blank Time     | = 0.667;  | // (msec) | =     | 30 lines |        |
| Ver Sync Start     | = 16.111; | // (msec) | = 725 | lines    |        |
| // V Bottom Border | = 0.000;  | // (msec) | =     | 0 lines  |        |
| // V Front Porch   | = 0.111;  | // (msec) | =     | 5 lines  |        |
| Ver Sync Time      | = 0.111;  | // (msec) | =     | 5 lines  |        |
| // V Back Porch    | = 0.444;  | // (msec) | =     | 20 lines |        |
| // V Top Border    | = 0.000;  | // (msec) | =     | 0 lines  |        |

Definition of Terms: Refer to Section 3.1
VESA MONITOR TIMING STANDARD
Adopted:  8/21/03
Resolution:  1280 x 768 at 60 Hz (non-interlaced) REDUCED BLANKING
EDID ID:  DMT ID: 16h; Std. 2 Byte Code: n/a; CVT 3 Byte Code: (7F, 1C, 21)h
Method:  CVT Reduced Blanking

Detailed Timing Parameters
| Timing Name | = 1280 x 768 @ 60Hz CVT (Reduced Blanking); |           |     |     |     |
| ----------- | ------------------------------------------- | --------- | --- | --- | --- |
| Hor Pixels  | = 1280;                                     | // Pixels |     |     |     |
768;
| Ver Pixels    | =         | // Lines |        |        |        |
| ------------- | --------- | -------- | ------ | ------ | ------ |
| Hor Frequency | = 47.396; | // kHz   | = 21.1 | usec / | line   |
| Ver Frequency | = 59.995; | // Hz    | = 16.7 | msec / | frame  |
| Pixel Clock   | = 68.250; | // MHz   | = 14.7 | nsec   | ± 0.5% |
8;
| Character Width   | =                | // Pixels | = 117.2    | nsec      |             |
| ----------------- | ---------------- | --------- | ---------- | --------- | ----------- |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =         | 1.1 %       |
| Hor Sync Polarity | = POSITIVE;      | // HBlank | = 11.1%    | of HTotal |             |
| Ver Sync Polarity | = NEGATIVE       | // VBlank | = 2.8%     | of VTotal |             |
| Hor Total Time    | = 21.099;        | // (usec) | = 180      | chars =   | 1440 Pixels |
| Hor Addr Time     | = 18.755;        | // (usec) | = 160      | chars =   | 1280 Pixels |
Hor Blank Start = 18.755; // (usec) = 160 chars = 1280 Pixels
| Hor Blank Time    | = 2.344;  | // (usec) | =     | 20 chars = | 160 Pixels  |
| ----------------- | --------- | --------- | ----- | ---------- | ----------- |
| Hor Sync Start    | = 19.458; | // (usec) | = 166 | chars =    | 1328 Pixels |
| // H Right Border | = 0.000;  | // (usec) | =     | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.703;  | // (usec) | =     | 6 chars =  | 48 Pixels   |
| Hor Sync Time     | = 0.469;  | // (usec) | =     | 4 chars =  | 32 Pixels   |
10
| // H Back Porch  | = 1.172; | // (usec) | =   | chars =   | 80 Pixels |
| ---------------- | -------- | --------- | --- | --------- | --------- |
| // H Left Border | = 0.000; | // (usec) | =   | 0 chars = | 0 Pixels  |
Ver Total Time = 16.668; // (msec) = 790 lines HT – (1.06xHA)
| Ver Addr Time      | = 16.204; | // (msec) | = 768 | lines    | = 1.22 |
| ------------------ | --------- | --------- | ----- | -------- | ------ |
| Ver Blank Start    | = 16.204; | // (msec) | = 768 | lines    |        |
| Ver Blank Time     | = 0.464;  | // (msec) | =     | 22 lines |        |
| Ver Sync Start     | = 16.267; | // (msec) | = 771 | lines    |        |
| // V Bottom Border | = 0.000;  | // (msec) | =     | 0 lines  |        |
| // V Front Porch   | = 0.063;  | // (msec) | =     | 3 lines  |        |
| Ver Sync Time      | = 0.148;  | // (msec) | =     | 7 lines  |        |
| // V Back Porch    | = 0.253;  | // (msec) | = 12  | lines    |        |
| // V Top Border    | = 0.000;  | // (msec) | =     | 0 lines  |        |

Definition of Terms: Refer to section 3.2.

VESA MONITOR TIMING STANDARD
Adopted:  8/21/03
Resolution:  1280 x 768 at 60 Hz (non-interlaced)
EDID ID:  DMT ID: 17h; Std. 2 Byte Code: n/a; CVT 3 Byte Code: (7F, 1C, 28)h
Method:  CVT Compliant

Detailed Timing Parameters
| Timing Name       | = 1280 x 768 @ 60Hz; |           |            |           |             |
| ----------------- | -------------------- | --------- | ---------- | --------- | ----------- |
| Hor Pixels        | = 1280;              | // Pixels |            |           |             |
| Ver Pixels        | = 768;               | // Lines  |            |           |             |
| Hor Frequency     | = 47.776;            | // kHz    | = 20.9     | usec /    | line        |
| Ver Frequency     | = 59.870;            | // Hz     | = 16.7     | msec /    | frame       |
| Pixel Clock       | = 79.500;            | // MHz    | = 12.6     | nsec      | ± 0.5%      |
| Character Width   | = 8;                 | // Pixels | = 100.6    | nsec      |             |
| Scan Type         | = NONINTERLACED;     |           | // H Phase | =         | 3.8 %       |
| Hor Sync Polarity | = NEGATIVE           | // HBlank | = 23.1%    | of HTotal |             |
| Ver Sync Polarity | = POSITIVE;          | // VBlank | = 3.8%     | of VTotal |             |
| Hor Total Time    | = 20.931;            | // (usec) | = 208      | chars =   | 1664 Pixels |
| Hor Addr Time     | = 16.101;            | // (usec) | = 160      | chars =   | 1280 Pixels |
Hor Blank Start = 16.101; // (usec) = 160 chars = 1280 Pixels
| Hor Blank Time    | = 4.830;  | // (usec) | =     | 48 chars = | 384 Pixels  |
| ----------------- | --------- | --------- | ----- | ---------- | ----------- |
| Hor Sync Start    | = 16.906; | // (usec) | = 168 | chars =    | 1344 Pixels |
| // H Right Border | = 0.000;  | // (usec) | =     | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.805;  | // (usec) | =     | 8 chars =  | 64 Pixels   |
| Hor Sync Time     | = 1.610;  | // (usec) | = 16  | chars =    | 128 Pixels  |
| // H Back Porch   | = 2.415;  | // (usec) | = 24  | chars =    | 192 Pixels  |
| // H Left Border  | = 0.000;  | // (usec) | =     | 0 chars =  | 0 Pixels    |
Ver Total Time = 16.703; // (msec) = 798 lines HT – (1.06xHA)
| Ver Addr Time      | = 16.075; | // (msec) | = 768 | lines    | = 3.86 |
| ------------------ | --------- | --------- | ----- | -------- | ------ |
| Ver Blank Start    | = 16.075; | // (msec) | = 768 | lines    |        |
| Ver Blank Time     | = 0.628;  | // (msec) | =     | 30 lines |        |
| Ver Sync Start     | = 16.138; | // (msec) | = 771 | lines    |        |
| // V Bottom Border | = 0.000;  | // (msec) | =     | 0 lines  |        |
| // V Front Porch   | = 0.063;  | // (msec) | =     | 3 lines  |        |
| Ver Sync Time      | = 0.147;  | // (msec) | =     | 7 lines  |        |
| // V Back Porch    | = 0.419;  | // (msec) | = 20  | lines    |        |
| // V Top Border    | = 0.000;  | // (msec) | =     | 0 lines  |        |

Definition of Terms: Refer to section 3.4.

VESA MONITOR TIMING STANDARD
Adopted:  8/21/03
Resolution:  1280 x 768 at 75 Hz (non-interlaced)
EDID ID:  DMT ID: 18h; Std. 2 Byte Code: n/a; CVT 3 Byte Code: (7F, 1C, 44)h
Method:  CVT Compliant

Detailed Timing Parameters
| Timing Name       | = 1280 x 768 @ 75Hz; |           |            |           |             |
| ----------------- | -------------------- | --------- | ---------- | --------- | ----------- |
| Hor Pixels        | = 1280;              | // Pixels |            |           |             |
| Ver Pixels        | = 768;               | // Lines  |            |           |             |
| Hor Frequency     | = 60.289;            | // KHz    | = 16.6     | usec /    | line        |
| Ver Frequency     | = 74.893;            | // Hz     | = 13.4     | msec /    | frame       |
| Pixel Clock       | = 102.250;           | // MHz    | = 9.8      | nsec      | ± 0.5%      |
| Character Width   | = 8;                 | // Pixels | = 78.2     | nsec      |             |
| Scan Type         | = NONINTERLACED;     |           | // H Phase | =         | 3.8 %       |
| Hor Sync Polarity | = NEGATIVE           | // HBlank | = 24.5%    | of HTotal |             |
| Ver Sync Polarity | = POSITIVE;          | // VBlank | = 4.6%     | of VTotal |             |
| Hor Total Time    | = 16.587;            | // (usec) | = 212      | chars =   | 1696 Pixels |
| Hor Addr Time     | = 12.518;            | // (usec) | = 160      | chars =   | 1280 Pixels |
Hor Blank Start = 12.518; // (usec) = 160 chars = 1280 Pixels
| Hor Blank Time    | = 4.068;  | // (usec) | =     | 52 chars = | 416 Pixels  |
| ----------------- | --------- | --------- | ----- | ---------- | ----------- |
| Hor Sync Start    | = 13.301; | // (usec) | = 170 | chars =    | 1360 Pixels |
| // H Right Border | = 0.000;  | // (usec) | =     | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.782;  | // (usec) | = 10  | chars =    | 80 Pixels   |
| Hor Sync Time     | = 1.252;  | // (usec) | = 16  | chars =    | 128 Pixels  |
| // H Back Porch   | = 2.034;  | // (usec) | = 26  | chars =    | 208 Pixels  |
| // H Left Border  | = 0.000;  | // (usec) | =     | 0 chars =  | 0 Pixels    |
Ver Total Time = 13.352; // (msec) = 805 lines HT – (1.06xHA)
| Ver Addr Time      | = 12.739; | // (msec) | = 768 | lines    | = 3.32 |
| ------------------ | --------- | --------- | ----- | -------- | ------ |
| Ver Blank Start    | = 12.739; | // (msec) | = 768 | lines    |        |
| Ver Blank Time     | = 0.614;  | // (msec) | =     | 37 lines |        |
| Ver Sync Start     | = 12.788; | // (msec) | = 771 | lines    |        |
| // V Bottom Border | = 0.000;  | // (msec) | =     | 0 lines  |        |
| // V Front Porch   | = 0.050;  | // (msec) | =     | 3 lines  |        |
| Ver Sync Time      | = 0.116;  | // (msec) | =     | 7 lines  |        |
| // V Back Porch    | = 0.448;  | // (msec) | = 27  | lines    |        |
| // V Top Border    | = 0.000;  | // (msec) | =     | 0 lines  |        |

Definition of Terms: Refer to section 3.4.

VESA MONITOR TIMING STANDARD
Adopted:  8/21/03
Resolution:  1280 x 768 at 85 Hz (non-interlaced)
EDID ID:  DMT ID: 19h; Std. 2 Byte Code: n/a; CVT 3 Byte Code: (7F, 1C, 62)h
Method:  CVT Compliant

Detailed Timing Parameters
| Timing Name       | = 1280 x 768 @ 85Hz; |           |            |           |             |
| ----------------- | -------------------- | --------- | ---------- | --------- | ----------- |
| Hor Pixels        | = 1280;              | // Pixels |            |           |             |
| Ver Pixels        | = 768;               | // Lines  |            |           |             |
| Hor Frequency     | = 68.633;            | // kHz    | = 14.6     | usec /    | line        |
| Ver Frequency     | = 84.837;            | // Hz     | = 11.8     | msec /    | frame       |
| Pixel Clock       | = 117.500;           | // MHz    | = 8.5      | nsec      | ± 0.5%      |
| Character Width   | = 8;                 | // Pixels | = 68.1     | nsec      |             |
| Scan Type         | = NONINTERLACED;     |           | // H Phase | =         | 4.0 %       |
| Hor Sync Polarity | = NEGATIVE           | // HBlank | = 25.2%    | of HTotal |             |
| Ver Sync Polarity | = POSITIVE;          | // VBlank | = 5.1%     | of VTotal |             |
| Hor Total Time    | = 14.570;            | // (usec) | = 214      | chars =   | 1712 Pixels |
| Hor Addr Time     | = 10.894;            | // (usec) | = 160      | chars =   | 1280 Pixels |
Hor Blank Start = 10.894; // (usec) = 160 chars = 1280 Pixels
| Hor Blank Time    | = 3.677;  | // (usec) | =     | 54 chars = | 432 Pixels  |
| ----------------- | --------- | --------- | ----- | ---------- | ----------- |
| Hor Sync Start    | = 11.574; | // (usec) | = 170 | chars =    | 1360 Pixels |
| // H Right Border | = 0.000;  | // (usec) | =     | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.681;  | // (usec) | = 10  | chars =    | 80 Pixels   |
| Hor Sync Time     | = 1.157;  | // (usec) | = 17  | chars =    | 136 Pixels  |
| // H Back Porch   | = 1.838;  | // (usec) | = 27  | chars =    | 216 Pixels  |
| // H Left Border  | = 0.000;  | // (usec) | =     | 0 chars =  | 0 Pixels    |
Ver Total Time = 11.787; // (msec) = 809 lines HT – (1.06xHA)
| Ver Addr Time      | = 11.190; | // (msec) | = 768 | lines    | = 3.02 |
| ------------------ | --------- | --------- | ----- | -------- | ------ |
| Ver Blank Start    | = 11.190; | // (msec) | = 768 | lines    |        |
| Ver Blank Time     | = 0.597;  | // (msec) | =     | 41 lines |        |
| Ver Sync Start     | = 11.234; | // (msec) | = 771 | lines    |        |
| // V Bottom Border | = 0.000;  | // (msec) | =     | 0 lines  |        |
| // V Front Porch   | = 0.044;  | // (msec) | =     | 3 lines  |        |
| Ver Sync Time      | = 0.102;  | // (msec) | =     | 7 lines  |        |
| // V Back Porch    | = 0.452;  | // (msec) | = 31  | lines    |        |
| // V Top Border    | = 0.000;  | // (msec) | =     | 0 lines  |        |

Definition of Terms: Refer to section 3.4.

VESA MONITOR TIMING STANDARD
Adopted:  5/1/07
Resolution:  1280 x 768 at 120 Hz (non-interlaced) REDUCED BLANKING
EDID ID:  DMT ID: 1Ah; Std. 2 Byte Code: n/a; CVT 3 Byte Code: n/a
Method:  Generated using CVT (Reduced Blanking) Formula

Detailed Timing Parameters
| Timing Name | = 1280 x 768 @ 120Hz CVT (Reduced Blanking); |           |     |     |     |
| ----------- | -------------------------------------------- | --------- | --- | --- | --- |
| Hor Pixels  | = 1280;                                      | // Pixels |     |     |     |
768;
| Ver Pixels    | =          | // Lines |        |        |        |
| ------------- | ---------- | -------- | ------ | ------ | ------ |
| Hor Frequency | = 97.396;  | // kHz   | = 10.3 | usec / | line   |
| Ver Frequency | = 119.798; | // Hz    | = 8.3  | msec / | frame  |
| Pixel Clock   | = 140.250; | // MHz   | = 7.1  | nsec   | ± 0.5% |
8;
| Character Width   | =                | // Pixels | = 57.0     | nsec       |             |
| ----------------- | ---------------- | --------- | ---------- | ---------- | ----------- |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =          | 1.1 %       |
| Hor Sync Polarity | = POSITIVE;      | // HBlank | = 11.1%    | of HTotal  |             |
| Ver Sync Polarity | = NEGATIVE       | // VBlank | = 5.5%     | of VTotal  |             |
| Hor Total Time    | = 10.267;        | // (usec) | = 180      | chars =    | 1440 Pixels |
| Hor Addr Time     | = 9.127;         | // (usec) | = 160      | chars =    | 1280 Pixels |
| Hor Blank Start   | = 9.127;         | // (usec) | = 160      | chars =    | 1280 Pixels |
| Hor Blank Time    | = 1.141;         | // (usec) | =          | 20 chars = | 160 Pixels  |
| Hor Sync Start    | = 9.469;         | // (usec) | = 166      | chars =    | 1328 Pixels |
| // H Right Border | = 0.000;         | // (usec) | =          | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.342;         | // (usec) | =          | 6 chars =  | 48 Pixels   |
| Hor Sync Time     | = 0.228;         | // (usec) | =          | 4 chars =  | 32 Pixels   |
10
| // H Back Porch    | = 0.570; | // (usec) | =     | chars =   | 80 Pixels      |
| ------------------ | -------- | --------- | ----- | --------- | -------------- |
| // H Left Border   | = 0.000; | // (usec) | =     | 0 chars = | 0 Pixels       |
| Ver Total Time     | = 8.347; | // (msec) | = 813 | lines     | HT – (1.06xHA) |
| Ver Addr Time      | = 7.885; | // (msec) | = 768 | lines     | = 0.59         |
| Ver Blank Start    | = 7.885; | // (msec) | = 768 | lines     |                |
| Ver Blank Time     | = 0.462; | // (msec) | =     | 45 lines  |                |
| Ver Sync Start     | = 7.916; | // (msec) | = 771 | lines     |                |
| // V Bottom Border | = 0.000; | // (msec) | =     | 0 lines   |                |
| // V Front Porch   | = 0.031; | // (msec) | =     | 3 lines   |                |
| Ver Sync Time      | = 0.072; | // (msec) | =     | 7 lines   |                |
| // V Back Porch    | = 0.359; | // (msec) | = 35  | lines     |                |
| // V Top Border    | = 0.000; | // (msec) | =     | 0 lines   |                |

Definition of Terms: Refer to section 3.2.

VESA MONITOR TIMING STANDARD
Adopted:  5/1/07
Resolution:  1280 x 800 at 60 Hz (non-interlaced) REDUCED BLANKING
EDID ID:  DMT ID: 1Bh; Std. 2 Byte Code: n/a; CVT 3 Byte Code: (8F, 18, 21)h
Method:  CVT Reduced Blanking

Detailed Timing Parameters
| Timing Name   | = 1280 x 800 @ 60Hz CVT (Reduced Blanking); |           |        |        |        |     |
| ------------- | ------------------------------------------- | --------- | ------ | ------ | ------ | --- |
| Hor Pixels    | = 1280;                                     | // Pixels |        |        |        |     |
| Ver Pixels    | = 800;                                      | // Lines  |        |        |        |     |
| Hor Frequency | = 49.306;                                   | // kHz    | = 20.3 | usec / | line   |     |
| Ver Frequency | = 59.910;                                   | // Hz     | = 16.7 | msec / | frame  |     |
| Pixel Clock   | = 71.000;                                   | // MHz    | = 14.1 | nsec   | ± 0.5% |     |
8;
| Character Width   | =                | // Pixels | = 112.7    | nsec      |             |     |
| ----------------- | ---------------- | --------- | ---------- | --------- | ----------- | --- |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =         | 1.1 %       |     |
| Hor Sync Polarity | = POSITIVE;      | // HBlank | = 11.1%    | of HTotal |             |     |
| Ver Sync Polarity | = NEGATIVE;      | // VBlank | = 2.8%     | of VTotal |             |     |
| Hor Total Time    | = 20.282;        | // (usec) | = 180      | chars =   | 1440 Pixels |     |
| Hor Addr Time     | = 18.028;        | // (usec) | = 160      | chars =   | 1280 Pixels |     |
Hor Blank Start = 18.028; // (usec) = 160 chars = 1280 Pixels
| Hor Blank Time    | = 2.254;  | // (usec) | =     | 20 chars = | 160 Pixels  |     |
| ----------------- | --------- | --------- | ----- | ---------- | ----------- | --- |
| Hor Sync Start    | = 18.704; | // (usec) | = 166 | chars =    | 1328 Pixels |     |
| // H Right Border | = 0.000;  | // (usec) | =     | 0 chars =  | 0 Pixels    |     |
6
| // H Front Porch | = 0.676; | // (usec) | =   | chars =    | 48 Pixels |     |
| ---------------- | -------- | --------- | --- | ---------- | --------- | --- |
| Hor Sync Time    | = 0.451; | // (usec) | =   | 4 chars =  | 32 Pixels |     |
| // H Back Porch  | = 1.127; | // (usec) | =   | 10 chars = | 80 Pixels |     |
| // H Left Border | = 0.000; | // (usec) | =   | 0 chars =  | 0 Pixels  |     |
Ver Total Time = 16.692; // (msec) = 823 lines HT – (1.06xHA)
| Ver Addr Time      | = 16.225; | // (msec) | = 800 | lines    | = 1.17 |     |
| ------------------ | --------- | --------- | ----- | -------- | ------ | --- |
| Ver Blank Start    | = 16.225; | // (msec) | = 800 | lines    |        |     |
| Ver Blank Time     | = 0.466;  | // (msec) | =     | 23 lines |        |     |
| Ver Sync Start     | = 16.286; | // (msec) | = 803 | lines    |        |     |
| // V Bottom Border | = 0.000;  | // (msec) | =     | 0 lines  |        |     |
3
| // V Front Porch | = 0.061; | // (msec) | =   | lines    |     |     |
| ---------------- | -------- | --------- | --- | -------- | --- | --- |
| Ver Sync Time    | = 0.122; | // (msec) | =   | 6 lines  |     |     |
| // V Back Porch  | = 0.284; | // (msec) | =   | 14 lines |     |     |
| // V Top Border  | = 0.000; | // (msec) | =   | 0 lines  |     |     |

Definition of Terms: Refer to section 3.2.

VESA MONITOR TIMING STANDARD
Adopted:  5/1/07
Resolution:  1280 x 800 at 60 Hz (non-interlaced)
EDID ID:  DMT ID: 1Ch; Std. 2 Byte Code: (81, 00)h; CVT 3 Byte Code: (8F, 18, 28)h
Method:  CVT Compliant

Detailed Timing Parameters
| Timing Name       | = 1280 x 800 @ 60Hz; |           |            |           |             |
| ----------------- | -------------------- | --------- | ---------- | --------- | ----------- |
| Hor Pixels        | = 1280;              | // Pixels |            |           |             |
| Ver Pixels        | = 800;               | // Lines  |            |           |             |
| Hor Frequency     | = 49.702;            | // kHz    | = 20.1     | usec /    | line        |
| Ver Frequency     | = 59.810;            | // Hz     | = 16.7     | msec /    | frame       |
| Pixel Clock       | = 83.500;            | // MHz    | = 12.0     | nsec      | ± 0.5%      |
| Character Width   | = 8;                 | // Pixels | = 95.8     | nsec      |             |
| Scan Type         | = NONINTERLACED;     |           | // H Phase | =         | 3.8 %       |
| Hor Sync Polarity | = NEGATIVE;          | // HBlank | = 23.8%    | of HTotal |             |
| Ver Sync Polarity | = POSITIVE;          | // VBlank | = 3.7%     | of VTotal |             |
| Hor Total Time    | = 20.120;            | // (usec) | = 210      | chars =   | 1680 Pixels |
| Hor Addr Time     | = 15.329;            | // (usec) | = 160      | chars =   | 1280 Pixels |
Hor Blank Start = 15.329; // (usec) = 160 chars = 1280 Pixels
| Hor Blank Time    | = 4.790;  | // (usec) | =     | 50 chars = | 400 Pixels  |
| ----------------- | --------- | --------- | ----- | ---------- | ----------- |
| Hor Sync Start    | = 16.192; | // (usec) | = 169 | chars =    | 1352 Pixels |
| // H Right Border | = 0.000;  | // (usec) | =     | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.862;  | // (usec) | =     | 9 chars =  | 72 Pixels   |
16
| Hor Sync Time    | = 1.533; | // (usec) | =   | chars =    | 128 Pixels |
| ---------------- | -------- | --------- | --- | ---------- | ---------- |
| // H Back Porch  | = 2.395; | // (usec) | =   | 25 chars = | 200 Pixels |
| // H Left Border | = 0.000; | // (usec) | =   | 0 chars =  | 0 Pixels   |
Ver Total Time = 16.720; // (msec) = 831 lines HT – (1.06xHA)
| Ver Addr Time      | = 16.096; | // (msec) | = 800 | lines    | = 3.87 |
| ------------------ | --------- | --------- | ----- | -------- | ------ |
| Ver Blank Start    | = 16.096; | // (msec) | = 800 | lines    |        |
| Ver Blank Time     | = 0.624;  | // (msec) | =     | 31 lines |        |
| Ver Sync Start     | = 16.156; | // (msec) | = 803 | lines    |        |
| // V Bottom Border | = 0.000;  | // (msec) | =     | 0 lines  |        |
| // V Front Porch   | = 0.060;  | // (msec) | =     | 3 lines  |        |
6
| Ver Sync Time   | = 0.121; | // (msec) | =   | lines    |     |
| --------------- | -------- | --------- | --- | -------- | --- |
| // V Back Porch | = 0.443; | // (msec) | =   | 22 lines |     |
| // V Top Border | = 0.000; | // (msec) | =   | 0 lines  |     |

Definition of Terms: Refer to section 3.4.

VESA MONITOR TIMING STANDARD
Adopted:  5/1/07
Resolution:  1280 x 800 at 75 Hz (non-interlaced)
EDID ID:  DMT ID: 1Dh; Std. 2 Byte Code: (81, 0F)h; CVT 3 Byte Code: (8F, 18, 44)h
Method:  CVT Compliant

Detailed Timing Parameters
| Timing Name | = 1280 x 800 @ 75Hz; |           |     |     |     |
| ----------- | -------------------- | --------- | --- | --- | --- |
| Hor Pixels  | = 1280;              | // Pixels |     |     |     |
800;
| Ver Pixels        | =                | // Lines  |            |           |        |
| ----------------- | ---------------- | --------- | ---------- | --------- | ------ |
| Hor Frequency     | = 62.795;        | // kHz    | = 15.9     | usec /    | line   |
| Ver Frequency     | = 74.934;        | // Hz     | = 13.3     | msec /    | frame  |
| Pixel Clock       | = 106.500;       | // MHz    | = 9.4      | nsec      | ± 0.5% |
| Character Width   | = 8;             | // Pixels | = 75.1     | nsec      |        |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =         | 3.8 %  |
| Hor Sync Polarity | = NEGATIVE;      | // HBlank | = 24.5%    | of HTotal |        |
POSITIVE;
| Ver Sync Polarity | =         | // VBlank | = 4.5% | of VTotal |             |
| ----------------- | --------- | --------- | ------ | --------- | ----------- |
| Hor Total Time    | = 15.925; | // (usec) | = 212  | chars =   | 1696 Pixels |
| Hor Addr Time     | = 12.019; | // (usec) | = 160  | chars =   | 1280 Pixels |
Hor Blank Start = 12.019; // (usec) = 160 chars = 1280 Pixels
| Hor Blank Time    | = 3.906;  | // (usec) | =     | 52 chars = | 416 Pixels  |
| ----------------- | --------- | --------- | ----- | ---------- | ----------- |
| Hor Sync Start    | = 12.770; | // (usec) | = 170 | chars =    | 1360 Pixels |
| // H Right Border | = 0.000;  | // (usec) | =     | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.751;  | // (usec) | =     | 10 chars = | 80 Pixels   |
| Hor Sync Time     | = 1.202;  | // (usec) | =     | 16 chars = | 128 Pixels  |
| // H Back Porch   | = 1.953;  | // (usec) | =     | 26 chars = | 208 Pixels  |
| // H Left Border  | = 0.000;  | // (usec) | =     | 0 chars =  | 0 Pixels    |
Ver Total Time = 13.345; // (msec) = 838 lines HT – (1.06xHA)
| Ver Addr Time      | = 12.740; | // (msec) | = 800 | lines    | = 3.18 |
| ------------------ | --------- | --------- | ----- | -------- | ------ |
| Ver Blank Start    | = 12.740; | // (msec) | = 800 | lines    |        |
| Ver Blank Time     | = 0.605;  | // (msec) | =     | 38 lines |        |
| Ver Sync Start     | = 12.788; | // (msec) | = 803 | lines    |        |
| // V Bottom Border | = 0.000;  | // (msec) | =     | 0 lines  |        |
| // V Front Porch   | = 0.048;  | // (msec) | =     | 3 lines  |        |
| Ver Sync Time      | = 0.096;  | // (msec) | =     | 6 lines  |        |
| // V Back Porch    | = 0.462;  | // (msec) | =     | 29 lines |        |
| // V Top Border    | = 0.000;  | // (msec) | =     | 0 lines  |        |

Definition of Terms: Refer to section 3.4.

VESA MONITOR TIMING STANDARD
Adopted:  5/1/07
Resolution:  1280 x 800 at 85 Hz (non-interlaced)
EDID ID:  DMT ID: 1Eh; Std. 2 Byte Code: (81, 19)h; CVT 3 Byte Code: (8F, 18, 62)h
Method:  CVT Compliant

Detailed Timing Parameters
| Timing Name | = 1280 x 800 @ 85Hz; |           |     |     |     |
| ----------- | -------------------- | --------- | --- | --- | --- |
| Hor Pixels  | = 1280;              | // Pixels |     |     |     |
800;
| Ver Pixels        | =                | // Lines  |            |           |        |
| ----------------- | ---------------- | --------- | ---------- | --------- | ------ |
| Hor Frequency     | = 71.554;        | // kHz    | = 14.0     | usec /    | line   |
| Ver Frequency     | = 84.880;        | // Hz     | = 11.8     | msec /    | frame  |
| Pixel Clock       | = 122.500;       | // MHz    | = 8.2      | nsec      | ± 0.5% |
| Character Width   | = 8;             | // Pixels | = 65.3     | nsec      |        |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =         | 4.0 %  |
| Hor Sync Polarity | = NEGATIVE;      | // HBlank | = 25.2%    | of HTotal |        |
POSITIVE;
| Ver Sync Polarity | =         | // VBlank | = 5.1% | of VTotal |             |
| ----------------- | --------- | --------- | ------ | --------- | ----------- |
| Hor Total Time    | = 13.976; | // (usec) | = 214  | chars =   | 1712 Pixels |
| Hor Addr Time     | = 10.449; | // (usec) | = 160  | chars =   | 1280 Pixels |
Hor Blank Start = 10.449; // (usec) = 160 chars = 1280 Pixels
| Hor Blank Time    | = 3.527;  | // (usec) | =     | 54 chars = | 432 Pixels  |
| ----------------- | --------- | --------- | ----- | ---------- | ----------- |
| Hor Sync Start    | = 11.102; | // (usec) | = 170 | chars =    | 1360 Pixels |
| // H Right Border | = 0.000;  | // (usec) | =     | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.653;  | // (usec) | =     | 10 chars = | 80 Pixels   |
| Hor Sync Time     | = 1.110;  | // (usec) | =     | 17 chars = | 136 Pixels  |
| // H Back Porch   | = 1.763;  | // (usec) | =     | 27 chars = | 216 Pixels  |
| // H Left Border  | = 0.000;  | // (usec) | =     | 0 chars =  | 0 Pixels    |
Ver Total Time = 11.781; // (msec) = 843 lines HT – (1.06xHA)
| Ver Addr Time      | = 11.180; | // (msec) | = 800 | lines    | = 2.9 |
| ------------------ | --------- | --------- | ----- | -------- | ----- |
| Ver Blank Start    | = 11.180; | // (msec) | = 800 | lines    |       |
| Ver Blank Time     | = 0.601;  | // (msec) | =     | 43 lines |       |
| Ver Sync Start     | = 11.222; | // (msec) | = 803 | lines    |       |
| // V Bottom Border | = 0.000;  | // (msec) | =     | 0 lines  |       |
| // V Front Porch   | = 0.042;  | // (msec) | =     | 3 lines  |       |
| Ver Sync Time      | = 0.084;  | // (msec) | =     | 6 lines  |       |
| // V Back Porch    | = 0.475;  | // (msec) | =     | 34 lines |       |
| // V Top Border    | = 0.000;  | // (msec) | =     | 0 lines  |       |

Definition of Terms: Refer to section 3.4.

VESA MONITOR TIMING STANDARD
Adopted:  5/1/07
Resolution:  1280 x 800 at 120 Hz (non-interlaced) REDUCED BLANKING
EDID ID:  DMT ID: 1Fh; Std. 2 Byte Code: n/a; CVT 3 Byte Code: n/a
Method:  Generated using CVT (Reduced Blanking) Formula

Detailed Timing Parameters
| Timing Name | = 1280 x 800 @ 120Hz CVT (Reduced Blanking); |           |     |     |     |
| ----------- | -------------------------------------------- | --------- | --- | --- | --- |
| Hor Pixels  | = 1280;                                      | // Pixels |     |     |     |
800;
| Ver Pixels    | =          | // Lines |       |        |        |
| ------------- | ---------- | -------- | ----- | ------ | ------ |
| Hor Frequency | = 101.563; | // kHz   | = 9.8 | usec / | line   |
| Ver Frequency | = 119.909; | // Hz    | = 8.3 | msec / | frame  |
| Pixel Clock   | = 146.250; | // MHz   | = 6.8 | nsec   | ± 0.5% |
8;
| Character Width   | =                | // Pixels | = 54.7     | nsec       |             |
| ----------------- | ---------------- | --------- | ---------- | ---------- | ----------- |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =          | 1.1 %       |
| Hor Sync Polarity | = POSITIVE;      | // HBlank | = 11.1%    | of HTotal  |             |
| Ver Sync Polarity | = NEGATIVE       | // VBlank | = 5.5%     | of VTotal  |             |
| Hor Total Time    | = 9.846;         | // (usec) | = 180      | chars =    | 1440 Pixels |
| Hor Addr Time     | = 8.752;         | // (usec) | = 160      | chars =    | 1280 Pixels |
| Hor Blank Start   | = 8.752;         | // (usec) | = 160      | chars =    | 1280 Pixels |
| Hor Blank Time    | = 1.094;         | // (usec) | =          | 20 chars = | 160 Pixels  |
| Hor Sync Start    | = 9.080;         | // (usec) | = 166      | chars =    | 1328 Pixels |
| // H Right Border | = 0.000;         | // (usec) | =          | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.328;         | // (usec) | =          | 6 chars =  | 48 Pixels   |
| Hor Sync Time     | = 0.219;         | // (usec) | =          | 4 chars =  | 32 Pixels   |
10
| // H Back Porch    | = 0.547; | // (usec) | =     | chars =   | 80 Pixels      |
| ------------------ | -------- | --------- | ----- | --------- | -------------- |
| // H Left Border   | = 0.000; | // (usec) | =     | 0 chars = | 0 Pixels       |
| Ver Total Time     | = 8.340; | // (msec) | = 847 | lines     | HT – (1.06xHA) |
| Ver Addr Time      | = 7.877; | // (msec) | = 800 | lines     | = 0.57         |
| Ver Blank Start    | = 7.877; | // (msec) | = 800 | lines     |                |
| Ver Blank Time     | = 0.463; | // (msec) | =     | 47 lines  |                |
| Ver Sync Start     | = 7.906; | // (msec) | = 803 | lines     |                |
| // V Bottom Border | = 0.000; | // (msec) | =     | 0 lines   |                |
| // V Front Porch   | = 0.030; | // (msec) | =     | 3 lines   |                |
| Ver Sync Time      | = 0.059; | // (msec) | =     | 6 lines   |                |
| // V Back Porch    | = 0.374; | // (msec) | = 38  | lines     |                |
| // V Top Border    | = 0.000; | // (msec) | =     | 0 lines   |                |

Definition of Terms: Refer to section 3.2.

VESA MONITOR TIMING STANDARD
Adopted:  3/1/96
Resolution:  1280 x 960 at 60 Hz (non-interlaced)
EDID ID:  DMT ID: 20h; Std. 2 Byte Code: (81, 40)h; CVT 3 Byte Code: n/a
Method:  *** NOT CVT COMPLIANT ***

Detailed Timing Parameters
| Timing Name   | = 1280 x 960 @ 60Hz; |           |             |         |
| ------------- | -------------------- | --------- | ----------- | ------- |
| Hor Pixels    | = 1280;              | // Pixels |             |         |
| Ver Pixels    | = 960;               | // Lines  |             |         |
| Hor Frequency | = 60.000;            | // kHz    | = 16.7 usec | / line  |
| Ver Frequency | = 60.000;            | // Hz     | = 16.7 msec | / frame |
108.000;
| Pixel Clock       | =                | // MHz    | = 9.3 nsec        | ± 0.5%        |
| ----------------- | ---------------- | --------- | ----------------- | ------------- |
| Character Width   | = 8;             | // Pixels | = 74.1 nsec       |               |
| Scan Type         | = NONINTERLACED; |           | // H Phase        | = 6.0 %       |
| Hor Sync Polarity | = POSITIVE;      | // HBlank | = 28.9% of HTotal |               |
| Ver Sync Polarity | = POSITIVE;      | // VBlank | = 4.0% of VTotal  |               |
| Hor Total Time    | = 16.667;        | // (usec) | = 225 chars       | = 1800 Pixels |
| Hor Addr Time     | = 11.852;        | // (usec) | = 160 chars       | = 1280 Pixels |
Hor Blank Start = 11.852; // (usec) = 160 chars = 1280 Pixels
| Hor Blank Time    | = 4.815;  | // (usec) | = 65 chars  | = 520 Pixels  |
| ----------------- | --------- | --------- | ----------- | ------------- |
| Hor Sync Start    | = 12.741; | // (usec) | = 172 chars | = 1376 Pixels |
| // H Right Border | = 0.000;  | // (usec) | = 0 chars   | = 0 Pixels    |
| // H Front Porch  | = 0.889;  | // (usec) | = 12 chars  | = 96 Pixels   |
| Hor Sync Time     | = 1.037;  | // (usec) | = 14 chars  | = 112 Pixels  |
| // H Back Porch   | = 2.889;  | // (usec) | = 39 chars  | = 312 Pixels  |
| // H Left Border  | = 0.000;  | // (usec) | = 0 chars   | = 0 Pixels    |
Ver Total Time = 16.667; // (msec) = 1000 lines HT – (1.06xHA)
| Ver Addr Time   | = 16.000; | // (msec) | = 960 lines | = 4.1 |
| --------------- | --------- | --------- | ----------- | ----- |
| Ver Blank Start | = 16.000; | // (msec) | = 960 lines |       |
| Ver Blank Time  | = 0.667;  | // (msec) | = 40 lines  |       |
| Ver Sync Start  | = 16.017; | // (msec) | = 961 lines |       |
0
| // V Bottom Border | = 0.000; | // (msec) | = lines    |     |
| ------------------ | -------- | --------- | ---------- | --- |
| // V Front Porch   | = 0.017; | // (msec) | = 1 lines  |     |
| Ver Sync Time      | = 0.050; | // (msec) | = 3 lines  |     |
| // V Back Porch    | = 0.600; | // (msec) | = 36 lines |     |
| // V Top Border    | = 0.000; | // (msec) | = 0 lines  |     |

Definition of Terms: Refer to section 3.1.

VESA MONITOR TIMING STANDARD
Adopted:  3/1/96
Resolution:  1280 x 960 at 85 Hz (non-interlaced)
EDID ID:  DMT ID: 21h; Std. 2 Byte Code: (81, 59)h; CVT 3 Byte Code: n/a
Method:  *** NOT CVT COMPLIANT ***

Detailed Timing Parameters
| Timing Name       | = 1280 x 960 @ 85Hz; |           |                   |               |
| ----------------- | -------------------- | --------- | ----------------- | ------------- |
| Hor Pixels        | = 1280;              | // Pixels |                   |               |
| Ver Pixels        | = 960;               | // Lines  |                   |               |
| Hor Frequency     | = 85.938;            | // kHz    | = 11.6 usec       | / line        |
| Ver Frequency     | = 85.002;            | // Hz     | = 11.8 msec       | / frame       |
| Pixel Clock       | = 148.500;           | // MHz    | = 6.7 nsec        | ± 0.5%        |
| Character Width   | = 8;                 | // Pixels | = 53.9 nsec       |               |
| Scan Type         | = NONINTERLACED;     |           | // H Phase        | = 4.6 %       |
| Hor Sync Polarity | = POSITIVE;          | // HBlank | = 25.9% of HTotal |               |
| Ver Sync Polarity | = POSITIVE;          | // VBlank | = 5.0% of VTotal  |               |
| Hor Total Time    | = 11.636;            | // (usec) | = 216 chars       | = 1728 Pixels |
| Hor Addr Time     | = 8.620;             | // (usec) | = 160 chars       | = 1280 Pixels |
| Hor Blank Start   | = 8.620;             | // (usec) | = 160 chars       | = 1280 Pixels |
| Hor Blank Time    | = 3.017;             | // (usec) | = 56 chars        | = 448 Pixels  |
| Hor Sync Start    | = 9.051;             | // (usec) | = 168 chars       | = 1344 Pixels |
| // H Right Border | = 0.000;             | // (usec) | = 0 chars         | = 0 Pixels    |
| // H Front Porch  | = 0.431;             | // (usec) | = 8 chars         | = 64 Pixels   |
| Hor Sync Time     | = 1.077;             | // (usec) | = 20 chars        | = 160 Pixels  |
| // H Back Porch   | = 1.508;             | // (usec) | = 28 chars        | = 224 Pixels  |
| // H Left Border  | = 0.000;             | // (usec) | = 0 chars         | = 0 Pixels    |
Ver Total Time = 11.764; // (msec) = 1011 lines HT – (1.06xHA)
| Ver Addr Time      | = 11.171; | // (msec) | = 960 lines | = 2.5 |
| ------------------ | --------- | --------- | ----------- | ----- |
| Ver Blank Start    | = 11.171; | // (msec) | = 960 lines |       |
| Ver Blank Time     | = 0.593;  | // (msec) | = 51 lines  |       |
| Ver Sync Start     | = 11.183; | // (msec) | = 961 lines |       |
| // V Bottom Border | = 0.000;  | // (msec) | = 0 lines   |       |
| // V Front Porch   | = 0.012;  | // (msec) | = 1 lines   |       |
| Ver Sync Time      | = 0.035;  | // (msec) | = 3 lines   |       |
47
| // V Back Porch | = 0.547; | // (msec) | = lines   |     |
| --------------- | -------- | --------- | --------- | --- |
| // V Top Border | = 0.000; | // (msec) | = 0 lines |     |

Definition of Terms: Refer to section 3.1.

VESA MONITOR TIMING STANDARD
Adopted:  5/1/07
Resolution:  1280 x 960 at 120 Hz (non-interlaced) REDUCED BLANKING
EDID ID:  DMT ID: 22h; Std. 2 Byte Code: n/a; CVT 3 Byte Code: n/a
Method:  Generated using CVT (Reduced Blanking) Formula

Detailed Timing Parameters
| Timing Name | = 1280 x 960 @ 120Hz CVT (Reduced Blanking); |           |     |     |     |
| ----------- | -------------------------------------------- | --------- | --- | --- | --- |
| Hor Pixels  | = 1280;                                      | // Pixels |     |     |     |
960;
| Ver Pixels    | =          | // Lines |       |        |        |
| ------------- | ---------- | -------- | ----- | ------ | ------ |
| Hor Frequency | = 121.875; | // kHz   | = 8.2 | usec / | line   |
| Ver Frequency | = 119.838; | // Hz    | = 8.3 | msec / | frame  |
| Pixel Clock   | = 175.500; | // MHz   | = 5.7 | nsec   | ± 0.5% |
8;
| Character Width   | =                | // Pixels | = 45.6     | nsec       |             |
| ----------------- | ---------------- | --------- | ---------- | ---------- | ----------- |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =          | 1.1 %       |
| Hor Sync Polarity | = POSITIVE;      | // HBlank | = 11.1%    | of HTotal  |             |
| Ver Sync Polarity | = NEGATIVE       | // VBlank | = 5.6%     | of VTotal  |             |
| Hor Total Time    | = 8.205;         | // (usec) | = 180      | chars =    | 1440 Pixels |
| Hor Addr Time     | = 7.293;         | // (usec) | = 160      | chars =    | 1280 Pixels |
| Hor Blank Start   | = 7.293;         | // (usec) | = 160      | chars =    | 1280 Pixels |
| Hor Blank Time    | = 0.912;         | // (usec) | =          | 20 chars = | 160 Pixels  |
| Hor Sync Start    | = 7.567;         | // (usec) | = 166      | chars =    | 1328 Pixels |
| // H Right Border | = 0.000;         | // (usec) | =          | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.274;         | // (usec) | =          | 6 chars =  | 48 Pixels   |
| Hor Sync Time     | = 0.182;         | // (usec) | =          | 4 chars =  | 32 Pixels   |
10
| // H Back Porch  | = 0.456; | // (usec) | =   | chars =   | 80 Pixels |
| ---------------- | -------- | --------- | --- | --------- | --------- |
| // H Left Border | = 0.000; | // (usec) | =   | 0 chars = | 0 Pixels  |
Ver Total Time = 8.345; // (msec) = 1017 lines HT – (1.06xHA)
| Ver Addr Time      | = 7.877; | // (msec) | = 960 | lines    | = 0.47 |
| ------------------ | -------- | --------- | ----- | -------- | ------ |
| Ver Blank Start    | = 7.877; | // (msec) | = 960 | lines    |        |
| Ver Blank Time     | = 0.468; | // (msec) | =     | 57 lines |        |
| Ver Sync Start     | = 7.902; | // (msec) | = 963 | lines    |        |
| // V Bottom Border | = 0.000; | // (msec) | =     | 0 lines  |        |
| // V Front Porch   | = 0.025; | // (msec) | =     | 3 lines  |        |
| Ver Sync Time      | = 0.033; | // (msec) | =     | 4 lines  |        |
| // V Back Porch    | = 0.410; | // (msec) | = 50  | lines    |        |
| // V Top Border    | = 0.000; | // (msec) | =     | 0 lines  |        |

Definition of Terms: Refer to section 3.2.

VESA MONITOR TIMING STANDARD
Adopted:  12/18/96
Resolution:  1280 x 1024 at 60 Hz (non-interlaced)
EDID ID:  DMT ID: 23h; Std. 2 Byte Code: (81, 80)h; CVT 3 Byte Code: n/a
BIOS Modes:  106h, 107h, 119h, 11Ah, & 11Bh (4, 8, 15, 16, & 24 bpp)
Method:  *** NOT CVT COMPLIANT ***

Detailed Timing Parameters
| Timing Name       | = 1280 x 1024 @ 60Hz; |           |                   |               |     |
| ----------------- | --------------------- | --------- | ----------------- | ------------- | --- |
| Hor Pixels        | = 1280;               | // Pixels |                   |               |     |
| Ver Pixels        | = 1024;               | // Lines  |                   |               |     |
| Hor Frequency     | = 63.981;             | // kHz    | = 15.6 usec       | / line        |     |
| Ver Frequency     | = 60.020;             | // Hz     | = 16.7 msec       | / frame       |     |
| Pixel Clock       | = 108.000;            | // MHz    | = 9.3 nsec        | ± 0.5%        |     |
| Character Width   | = 8;                  | // Pixels | = 74.1 nsec       |               |     |
| Scan Type         | = NONINTERLACED;      |           | // H Phase        | = 5.9 %       |     |
| Hor Sync Polarity | = POSITIVE;           | // HBlank | = 24.2% of HTotal |               |     |
| Ver Sync Polarity | = POSITIVE;           | // VBlank | = 3.9% of VTotal  |               |     |
| Hor Total Time    | = 15.630;             | // (usec) | = 211 chars       | = 1688 Pixels |     |
| Hor Addr Time     | = 11.852;             | // (usec) | = 160 chars       | = 1280 Pixels |     |
Hor Blank Start = 11.852; // (usec) = 160 chars = 1280 Pixels
| Hor Blank Time    | = 3.778;  | // (usec) | = 51 chars  | = 408 Pixels  |     |
| ----------------- | --------- | --------- | ----------- | ------------- | --- |
| Hor Sync Start    | = 12.296; | // (usec) | = 166 chars | = 1328 Pixels |     |
| // H Right Border | = 0.000;  | // (usec) | = 0 chars   | = 0 Pixels    |     |
| // H Front Porch  | = 0.444;  | // (usec) | = 6 chars   | = 48 Pixels   |     |
| Hor Sync Time     | = 1.037;  | // (usec) | = 14 chars  | = 112 Pixels  |     |
| // H Back Porch   | = 2.296;  | // (usec) | = 31 chars  | = 248 Pixels  |     |
| // H Left Border  | = 0.000;  | // (usec) | = 0 chars   | = 0 Pixels    |     |
Ver Total Time = 16.661; // (msec) = 1066 lines HT – (1.06xHA)
| Ver Addr Time      | = 16.005; | // (msec) | = 1024 lines | = 3.07 |     |
| ------------------ | --------- | --------- | ------------ | ------ | --- |
| Ver Blank Start    | = 16.005; | // (msec) | = 1024 lines |        |     |
| Ver Blank Time     | = 0.656;  | // (msec) | = 42 lines   |        |     |
| Ver Sync Start     | = 16.020; | // (msec) | = 1025 lines |        |     |
| // V Bottom Border | = 0.000;  | // (msec) | = 0 lines    |        |     |
| // V Front Porch   | = 0.016;  | // (msec) | = 1 lines    |        |     |
| Ver Sync Time      | = 0.047;  | // (msec) | = 3 lines    |        |     |
| // V Back Porch    | = 0.594;  | // (msec) | = 38 lines   |        |     |
0
| // V Top Border | = 0.000; | // (msec) | = lines |     |     |
| --------------- | -------- | --------- | ------- | --- | --- |

Definition of Terms: Refer to section 3.1.

VESA MONITOR TIMING STANDARD
Adopted:  10/4/93
Resolution:  1280 x 1024 at 75 Hz (non-interlaced)
EDID ID:  DMT ID: 24h; Std. 2 Byte Code: (81, 8F)h; CVT 3 Byte Code: n/a
BIOS Modes:  106h, 107h, 119h, 11Ah, & 11Bh (4, 8, 15, 16, & 24 bpp)
Method:  *** NOT CVT COMPLIANT ***

Detailed Timing Parameters
1280 x 1024 @ 75Hz;
| Timing Name       | =                |           |                   |               |
| ----------------- | ---------------- | --------- | ----------------- | ------------- |
| Hor Pixels        | = 1280;          | // Pixels |                   |               |
| Ver Pixels        | = 1024;          | // Lines  |                   |               |
| Hor Frequency     | = 79.976;        | // kHz    | = 12.5 usec       | / line        |
| Ver Frequency     | = 75.025;        | // Hz     | = 13.3 msec       | / frame       |
| Pixel Clock       | = 135.000;       | // MHz    | = 7.4 nsec        | ± 0.5%        |
| Character Width   | = 8;             | // Pixels | = 59.3 nsec       |               |
| Scan Type         | = NONINTERLACED; |           | // H Phase        | = 6.9 %       |
| Hor Sync Polarity | = POSITIVE;      | // HBlank | = 24.2% of HTotal |               |
| Ver Sync Polarity | = POSITIVE;      | // VBlank | = 3.9% of VTotal  |               |
| Hor Total Time    | = 12.504;        | // (usec) | = 211 chars       | = 1688 Pixels |
| Hor Addr Time     | = 9.481;         | // (usec) | = 160 chars       | = 1280 Pixels |
| Hor Blank Start   | = 9.481;         | // (usec) | = 160 chars       | = 1280 Pixels |
| Hor Blank Time    | = 3.022;         | // (usec) | = 51 chars        | = 408 Pixels  |
| Hor Sync Start    | = 9.600;         | // (usec) | = 162 chars       | = 1296 Pixels |
| // H Right Border | = 0.000;         | // (usec) | = 0 chars         | = 0 Pixels    |
| // H Front Porch  | = 0.119;         | // (usec) | = 2 chars         | = 16 Pixels   |
| Hor Sync Time     | = 1.067;         | // (usec) | = 18 chars        | = 144 Pixels  |
| // H Back Porch   | = 1.837;         | // (usec) | = 31 chars        | = 248 Pixels  |
| // H Left Border  | = 0.000;         | // (usec) | = 0 chars         | = 0 Pixels    |
Ver Total Time = 13.329; // (msec) = 1066 lines HT – (1.06xHA)
| Ver Addr Time      | = 12.804; | // (msec) | = 1024 lines | = 2.45 |
| ------------------ | --------- | --------- | ------------ | ------ |
| Ver Blank Start    | = 12.804; | // (msec) | = 1024 lines |        |
| Ver Blank Time     | = 0.525;  | // (msec) | = 42 lines   |        |
| Ver Sync Start     | = 12.816; | // (msec) | = 1025 lines |        |
| // V Bottom Border | = 0.000;  | // (msec) | = 0 lines    |        |
| // V Front Porch   | = 0.013;  | // (msec) | = 1 lines    |        |
| Ver Sync Time      | = 0.038;  | // (msec) | = 3 lines    |        |
| // V Back Porch    | = 0.475;  | // (msec) | = 38 lines   |        |
| // V Top Border    | = 0.000;  | // (msec) | = 0 lines    |        |

Definition of Terms: Refer to section 3.1.

VESA MONITOR TIMING STANDARD
Adopted:  3/1/96
Resolution:  1280 x 1024 at 85 Hz (non-interlaced)
EDID ID:  DMT ID: 25h; Std. 2 Byte Code: (81, 99)h; CVT 3 Byte Code: n/a
Method:  *** NOT CVT COMPLIANT ***

Detailed Timing Parameters
| Timing Name   | = 1280 x 1024 @ 85Hz; |           |             |         |
| ------------- | --------------------- | --------- | ----------- | ------- |
| Hor Pixels    | = 1280;               | // Pixels |             |         |
| Ver Pixels    | = 1024;               | // Lines  |             |         |
| Hor Frequency | = 91.146;             | // kHz    | = 11.0 usec | / line  |
| Ver Frequency | = 85.024;             | // Hz     | = 11.8 msec | / frame |
157.500;
| Pixel Clock       | =                | // MHz    | = 6.3 nsec        | ± 0.5%        |
| ----------------- | ---------------- | --------- | ----------------- | ------------- |
| Character Width   | = 8;             | // Pixels | = 50.8 nsec       |               |
| Scan Type         | = NONINTERLACED; |           | // H Phase        | = 4.6 %       |
| Hor Sync Polarity | = POSITIVE;      | // HBlank | = 25.9% of HTotal |               |
| Ver Sync Polarity | = POSITIVE;      | // VBlank | = 4.5% of VTotal  |               |
| Hor Total Time    | = 10.971;        | // (usec) | = 216 chars       | = 1728 Pixels |
| Hor Addr Time     | = 8.127;         | // (usec) | = 160 chars       | = 1280 Pixels |
| Hor Blank Start   | = 8.127;         | // (usec) | = 160 chars       | = 1280 Pixels |
| Hor Blank Time    | = 2.844;         | // (usec) | = 56 chars        | = 448 Pixels  |
| Hor Sync Start    | = 8.533;         | // (usec) | = 168 chars       | = 1344 Pixels |
| // H Right Border | = 0.000;         | // (usec) | = 0 chars         | = 0 Pixels    |
| // H Front Porch  | = 0.406;         | // (usec) | = 8 chars         | = 64 Pixels   |
| Hor Sync Time     | = 1.016;         | // (usec) | = 20 chars        | = 160 Pixels  |
| // H Back Porch   | = 1.422;         | // (usec) | = 28 chars        | = 224 Pixels  |
| // H Left Border  | = 0.000;         | // (usec) | = 0 chars         | = 0 Pixels    |
Ver Total Time = 11.761; // (msec) = 1072 lines HT – (1.06xHA)
| Ver Addr Time   | = 11.235; | // (msec) | = 1024 lines | = 2.36 |
| --------------- | --------- | --------- | ------------ | ------ |
| Ver Blank Start | = 11.235; | // (msec) | = 1024 lines |        |
| Ver Blank Time  | = 0.527;  | // (msec) | = 48 lines   |        |
| Ver Sync Start  | = 11.246; | // (msec) | = 1025 lines |        |
0
| // V Bottom Border | = 0.000; | // (msec) | = lines    |     |
| ------------------ | -------- | --------- | ---------- | --- |
| // V Front Porch   | = 0.011; | // (msec) | = 1 lines  |     |
| Ver Sync Time      | = 0.033; | // (msec) | = 3 lines  |     |
| // V Back Porch    | = 0.483; | // (msec) | = 44 lines |     |
| // V Top Border    | = 0.000; | // (msec) | = 0 lines  |     |

Definition of Terms: Refer to section 3.1.

VESA MONITOR TIMING STANDARD
Adopted:  5/1/07
Resolution:  1280 x 1024 at 120 Hz (non-interlaced) REDUCED BLANKING
EDID ID:  DMT ID: 26h; Std. 2 Byte Code: n/a; CVT 3 Byte Code: n/a
Method:  Generated using CVT (Reduced Blanking) Formula

Detailed Timing Parameters
| Timing Name | = 1280 x 1024 @ 120Hz CVT (Reduced Blanking); |           |     |     |     |
| ----------- | --------------------------------------------- | --------- | --- | --- | --- |
| Hor Pixels  | = 1280;                                       | // Pixels |     |     |     |
1024;
| Ver Pixels    | =          | // Lines |       |        |        |
| ------------- | ---------- | -------- | ----- | ------ | ------ |
| Hor Frequency | = 130.035; | // kHz   | = 7.7 | usec / | line   |
| Ver Frequency | = 119.958; | // Hz    | = 8.3 | msec / | frame  |
| Pixel Clock   | = 187.250; | // MHz   | = 5.3 | nsec   | ± 0.5% |
8;
| Character Width   | =                | // Pixels | = 42.7     | nsec       |             |
| ----------------- | ---------------- | --------- | ---------- | ---------- | ----------- |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =          | 1.1 %       |
| Hor Sync Polarity | = POSITIVE;      | // HBlank | = 11.1%    | of HTotal  |             |
| Ver Sync Polarity | = NEGATIVE       | // VBlank | = 5.5%     | of VTotal  |             |
| Hor Total Time    | = 7.690;         | // (usec) | = 180      | chars =    | 1440 Pixels |
| Hor Addr Time     | = 6.836;         | // (usec) | = 160      | chars =    | 1280 Pixels |
| Hor Blank Start   | = 6.836;         | // (usec) | = 160      | chars =    | 1280 Pixels |
| Hor Blank Time    | = 0.854;         | // (usec) | =          | 20 chars = | 160 Pixels  |
| Hor Sync Start    | = 7.092;         | // (usec) | = 166      | chars =    | 1328 Pixels |
| // H Right Border | = 0.000;         | // (usec) | =          | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.256;         | // (usec) | =          | 6 chars =  | 48 Pixels   |
| Hor Sync Time     | = 0.171;         | // (usec) | =          | 4 chars =  | 32 Pixels   |
10
| // H Back Porch  | = 0.427; | // (usec) | =   | chars =   | 80 Pixels |
| ---------------- | -------- | --------- | --- | --------- | --------- |
| // H Left Border | = 0.000; | // (usec) | =   | 0 chars = | 0 Pixels  |
Ver Total Time = 8.336; // (msec) = 1084 lines HT – (1.06xHA)
| Ver Addr Time      | = 7.875; | // (msec) | = 1024 | lines    | = 0.44 |
| ------------------ | -------- | --------- | ------ | -------- | ------ |
| Ver Blank Start    | = 7.875; | // (msec) | = 1024 | lines    |        |
| Ver Blank Time     | = 0.461; | // (msec) | =      | 60 lines |        |
| Ver Sync Start     | = 7.898; | // (msec) | = 1027 | lines    |        |
| // V Bottom Border | = 0.000; | // (msec) | =      | 0 lines  |        |
| // V Front Porch   | = 0.023; | // (msec) | =      | 3 lines  |        |
| Ver Sync Time      | = 0.054; | // (msec) | =      | 7 lines  |        |
| // V Back Porch    | = 0.385; | // (msec) | = 50   | lines    |        |
| // V Top Border    | = 0.000; | // (msec) | =      | 0 lines  |        |

Definition of Terms: Refer to section 3.2.

VESA MONITOR TIMING STANDARD
Adopted:  8/21/03
Resolution:  1360 x 768 at 60 Hz (non-interlaced)
EDID ID:  DMT ID: 27h; Std. 2 Byte Code: n/a; CVT 3 Byte Code: n/a
Method:  *** NOT CVT COMPLIANT ***

Detailed Timing Parameters
| Timing Name       | = 1360 x 768 @ 60Hz; |           |            |           |             |
| ----------------- | -------------------- | --------- | ---------- | --------- | ----------- |
| Hor Pixels        | = 1360;              | // Pixels |            |           |             |
| Ver Pixels        | = 768;               | // Lines  |            |           |             |
| Hor Frequency     | = 47.712;            | // kHz    | = 21.0     | usec /    | line        |
| Ver Frequency     | = 60.015;            | // Hz     | = 16.7     | msec /    | frame       |
| Pixel Clock       | = 85.500;            | // MHz    | = 11.7     | nsec      | ± 0.5%      |
| Character Width   | = 8;                 | // Pixels | = 93.6     | nsec      |             |
| Scan Type         | = NONINTERLACED;     |           | // H Phase | =         | 5.4 %       |
| Hor Sync Polarity | = POSITIVE;          | // HBlank | = 24.1%    | of HTotal |             |
| Ver Sync Polarity | = POSITIVE;          | // VBlank | = 3.4%     | of VTotal |             |
| Hor Total Time    | = 20.959;            | // (usec) | = 224      | chars =   | 1792 Pixels |
| Hor Addr Time     | = 15.906;            | // (usec) | = 170      | chars =   | 1360 Pixels |
Hor Blank Start = 15.906; // (usec) = 170 chars = 1360 Pixels
| Hor Blank Time    | = 5.053;  | // (usec) | =     | 54 chars = | 432 Pixels  |
| ----------------- | --------- | --------- | ----- | ---------- | ----------- |
| Hor Sync Start    | = 16.655; | // (usec) | = 178 | chars =    | 1424 Pixels |
| // H Right Border | = 0.000;  | // (usec) | =     | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.749;  | // (usec) | =     | 8 chars =  | 64 Pixels   |
| Hor Sync Time     | = 1.310;  | // (usec) | = 14  | chars =    | 112 Pixels  |
| // H Back Porch   | = 2.994;  | // (usec) | = 32  | chars =    | 256 Pixels  |
| // H Left Border  | = 0.000;  | // (usec) | =     | 0 chars =  | 0 Pixels    |
Ver Total Time = 16.662; // (msec) = 795 lines HT – (1.06xHA)
| Ver Addr Time      | = 16.097; | // (msec) | = 768 | lines    | = 4.1 |
| ------------------ | --------- | --------- | ----- | -------- | ----- |
| Ver Blank Start    | = 16.097; | // (msec) | = 768 | lines    |       |
| Ver Blank Time     | = 0.566;  | // (msec) | =     | 27 lines |       |
| Ver Sync Start     | = 16.159; | // (msec) | = 771 | lines    |       |
| // V Bottom Border | = 0.000;  | // (msec) | =     | 0 lines  |       |
| // V Front Porch   | = 0.063;  | // (msec) | =     | 3 lines  |       |
| Ver Sync Time      | = 0.126;  | // (msec) | =     | 6 lines  |       |
| // V Back Porch    | = 0.377;  | // (msec) | = 18  | lines    |       |
| // V Top Border    | = 0.000;  | // (msec) | =     | 0 lines  |       |

Definition of Terms: Refer to section 3.1.

VESA MONITOR TIMING STANDARD
Adopted:  5/1/07
Resolution:  1360 x 768 at 120 Hz (non-interlaced) REDUCED BLANKING
EDID ID:  DMT ID: 28h; Std. 2 Byte Code: n/a; CVT 3 Byte Code: n/a
Method:  Generated using CVT (Reduced Blanking) Formula

Detailed Timing Parameters
| Timing Name | = 1360 x 768 @ 120Hz CVT (Reduced Blanking); |           |     |     |     |
| ----------- | -------------------------------------------- | --------- | --- | --- | --- |
| Hor Pixels  | = 1360;                                      | // Pixels |     |     |     |
768;
| Ver Pixels    | =          | // Lines |        |        |        |
| ------------- | ---------- | -------- | ------ | ------ | ------ |
| Hor Frequency | = 97.533;  | // kHz   | = 10.3 | usec / | line   |
| Ver Frequency | = 119.967; | // Hz    | = 8.3  | msec / | frame  |
| Pixel Clock   | = 148.250; | // MHz   | = 6.7  | nsec   | ± 0.5% |
8;
| Character Width   | =                | // Pixels | = 54.0     | nsec       |             |
| ----------------- | ---------------- | --------- | ---------- | ---------- | ----------- |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =          | 1.1 %       |
| Hor Sync Polarity | = POSITIVE;      | // HBlank | = 10.5%    | of HTotal  |             |
| Ver Sync Polarity | = NEGATIVE       | // VBlank | = 5.5%     | of VTotal  |             |
| Hor Total Time    | = 10.253;        | // (usec) | = 190      | chars =    | 1520 Pixels |
| Hor Addr Time     | = 9.174;         | // (usec) | = 170      | chars =    | 1360 Pixels |
| Hor Blank Start   | = 9.174;         | // (usec) | = 170      | chars =    | 1360 Pixels |
| Hor Blank Time    | = 1.079;         | // (usec) | =          | 20 chars = | 160 Pixels  |
| Hor Sync Start    | = 9.497;         | // (usec) | = 176      | chars =    | 1408 Pixels |
| // H Right Border | = 0.000;         | // (usec) | =          | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.324;         | // (usec) | =          | 6 chars =  | 48 Pixels   |
| Hor Sync Time     | = 0.216;         | // (usec) | =          | 4 chars =  | 32 Pixels   |
10
| // H Back Porch    | = 0.540; | // (usec) | =     | chars =   | 80 Pixels      |
| ------------------ | -------- | --------- | ----- | --------- | -------------- |
| // H Left Border   | = 0.000; | // (usec) | =     | 0 chars = | 0 Pixels       |
| Ver Total Time     | = 8.336; | // (msec) | = 813 | lines     | HT – (1.06xHA) |
| Ver Addr Time      | = 7.874; | // (msec) | = 768 | lines     | = 0.53         |
| Ver Blank Start    | = 7.874; | // (msec) | = 768 | lines     |                |
| Ver Blank Time     | = 0.461; | // (msec) | =     | 45 lines  |                |
| Ver Sync Start     | = 7.905; | // (msec) | = 771 | lines     |                |
| // V Bottom Border | = 0.000; | // (msec) | =     | 0 lines   |                |
| // V Front Porch   | = 0.031; | // (msec) | =     | 3 lines   |                |
| Ver Sync Time      | = 0.051; | // (msec) | =     | 5 lines   |                |
| // V Back Porch    | = 0.379; | // (msec) | = 37  | lines     |                |
| // V Top Border    | = 0.000; | // (msec) | =     | 0 lines   |                |

Definition of Terms: Refer to section 3.2.

VESA MONITOR TIMING STANDARD
Adopted:  11/17/08
Resolution:  1366 x 768 at 60 Hz (non-interlaced) NORMAL BLANKING
EDID ID:  DMT ID: 51h; Std. 2 Byte Code: n/a; CVT 3 Byte Code: n/a
Method:  *** NOT CVT COMPLIANT ***

Detailed Timing Parameters
| Timing Name     | = 1366 x 768 @ 60Hz; |           |        |        |        |
| --------------- | -------------------- | --------- | ------ | ------ | ------ |
| Hor Pixels      | = 1366;              | // Pixels |        |        |        |
| Ver Pixels      | = 768;               | // Lines  |        |        |        |
| Hor Frequency   | = 47.712;            | // KHz    | = 21.0 | usec / | line   |
| Ver Frequency   | = 59.790;            | // Hz     | = 16.7 | msec / | frame  |
| Pixel Clock     | = 85.500;            | // MHz    | = 11.7 | nsec   | ± 0.5% |
| Character Width | = 1;                 | // Pixels | = 11.7 | nsec   |        |
NONINTERLACED;
| Scan Type         | =           |           | // H Phase | =         | 4.0 % |
| ----------------- | ----------- | --------- | ---------- | --------- | ----- |
| Hor Sync Polarity | = POSITIVE; | // HBlank | = 23.8%    | of HTotal |       |
| Ver Sync Polarity | = POSITIVE; | // VBlank | = 3.8%     | of VTotal |       |
Hor Total Time = 20.959; // (usec) = 1792 chars = 1792 Pixels
| Hor Addr Time | = 15.977; | // (usec) | = 1366 | chars = | 1366 Pixels |
| ------------- | --------- | --------- | ------ | ------- | ----------- |
Hor Blank Start = 15.977; // (usec) = 1366 chars = 1366 Pixels
| Hor Blank Time | = 4.982; | // (usec) | = 426 | chars = | 426 Pixels |
| -------------- | -------- | --------- | ----- | ------- | ---------- |
Hor Sync Start = 16.795; // (usec) = 1436 chars = 1436 Pixels
| // H Right Border | = 0.000; | // (usec) | =     | 0 chars =  | 0 Pixels   |
| ----------------- | -------- | --------- | ----- | ---------- | ---------- |
| // H Front Porch  | = 0.819; | // (usec) | =     | 70 chars = | 70 Pixels  |
| Hor Sync Time     | = 1.673; | // (usec) | = 143 | chars =    | 143 Pixels |
| // H Back Porch   | = 2.491; | // (usec) | = 213 | chars =    | 213 Pixels |
| // H Left Border  | = 0.000; | // (usec) | =     | 0 chars =  | 0 Pixels   |
Ver Total Time = 16.725; // (msec) = 798 lines HT – (1.06xHA)
| Ver Addr Time      | = 16.097; | // (msec) | = 768 | lines    | = 4.02 |
| ------------------ | --------- | --------- | ----- | -------- | ------ |
| Ver Blank Start    | = 16.097; | // (msec) | = 768 | lines    |        |
| Ver Blank Time     | = 0.629;  | // (msec) | =     | 30 lines |        |
| Ver Sync Start     | = 16.159; | // (msec) | = 771 | lines    |        |
| // V Bottom Border | = 0.000;  | // (msec) | =     | 0 lines  |        |
| // V Front Porch   | = 0.063;  | // (msec) | =     | 3 lines  |        |
| Ver Sync Time      | = 0.063;  | // (msec) | =     | 3 lines  |        |
| // V Back Porch    | = 0.503;  | // (msec) | =     | 24 lines |        |
| // V Top Border    | = 0.000;  | // (msec) | =     | 0 lines  |        |

Definition of Terms: Refer to Section 3.1
VESA MONITOR TIMING STANDARD
Adopted:  11/17/08
Resolution:  1366 x 768 at 60 Hz (non-interlaced) REDUCED BLANKING
EDID ID:  DMT ID: 56h; Std. 2 Byte Code: n/a; CVT 3 Byte Code: n/a
Method:  *** NOT CVT COMPLIANT ***

Detailed Timing Parameters
| Timing Name     | = 1366 x 768 @ 60Hz; |           |            |        |        |
| --------------- | -------------------- | --------- | ---------- | ------ | ------ |
| Hor Pixels      | = 1366;              | // Pixels |            |        |        |
| Ver Pixels      | = 768;               | // Lines  |            |        |        |
| Hor Frequency   | = 48.000;            | // KHz    | = 20.8     | usec / | line   |
| Ver Frequency   | = 60.000;            | // Hz     | = 16.7     | msec / | frame  |
| Pixel Clock     | = 72.000;            | // MHz    | = 13.9     | nsec   | ± 0.5% |
| Character Width | = 1;                 | // Pixels | = 13.9     | nsec   |        |
| Scan Type       | = NONINTERLACED;     |           | // H Phase | =      | 1.7 %  |
POSITIVE;
| Hor Sync Polarity | =           | // HBlank | = 8.9% | of HTotal |     |
| ----------------- | ----------- | --------- | ------ | --------- | --- |
| Ver Sync Polarity | = POSITIVE; | // VBlank | = 4.0% | of VTotal |     |
Hor Total Time = 20.833; // (usec) = 1500 chars = 1500 Pixels
| Hor Addr Time | = 18.972; | // (usec) | = 1366 | chars = | 1366 Pixels |
| ------------- | --------- | --------- | ------ | ------- | ----------- |
Hor Blank Start = 18.972; // (usec) = 1366 chars = 1366 Pixels
| Hor Blank Time | = 1.861; | // (usec) | = 134 | chars = | 134 Pixels |
| -------------- | -------- | --------- | ----- | ------- | ---------- |
Hor Sync Start = 19.167; // (usec) = 1380 chars = 1380 Pixels
| // H Right Border | = 0.000; | // (usec) | =   | 0 chars =  | 0 Pixels  |
| ----------------- | -------- | --------- | --- | ---------- | --------- |
| // H Front Porch  | = 0.194; | // (usec) | =   | 14 chars = | 14 Pixels |
| Hor Sync Time     | = 0.778; | // (usec) | =   | 56 chars = | 56 Pixels |
| // H Back Porch   | = 0.889; | // (usec) | =   | 64 chars = | 64 Pixels |
| // H Left Border  | = 0.000; | // (usec) | =   | 0 chars =  | 0 Pixels  |
Ver Total Time = 16.667; // (msec) = 800 lines HT – (1.06xHA)
| Ver Addr Time   | = 16.000; | // (msec) | = 768 | lines    | = 0.72 |
| --------------- | --------- | --------- | ----- | -------- | ------ |
| Ver Blank Start | = 16.000; | // (msec) | = 768 | lines    |        |
| Ver Blank Time  | = 0.667;  | // (msec) | =     | 32 lines |        |
| Ver Sync Start  | = 16.021; | // (msec) | = 769 | lines    |        |
0
| // V Bottom Border | = 0.000; | // (msec) | =   | lines   |     |
| ------------------ | -------- | --------- | --- | ------- | --- |
| // V Front Porch   | = 0.021; | // (msec) | =   | 1 lines |     |
3
| Ver Sync Time   | = 0.063; | // (msec) | =   | lines    |     |
| --------------- | -------- | --------- | --- | -------- | --- |
| // V Back Porch | = 0.583; | // (msec) | =   | 28 lines |     |
0
| // V Top Border | = 0.000; | // (msec) | =   | lines |     |
| --------------- | -------- | --------- | --- | ----- | --- |

Definition of Terms: Refer to Section 3.1
VESA MONITOR TIMING STANDARD
Adopted:  8/21/03
Resolution:  1400 x 1050 at 60 Hz (non-interlaced) REDUCED BLANKING
EDID ID:  DMT ID: 29h; Std. 2 Byte Code: n/a; CVT 3 Byte Code: (0C, 20, 21)h
Method:  CVT Reduced Blanking

Detailed Timing Parameters
| Timing Name | = 1400 x 1050 @ 60Hz CVT (Reduced Blanking); |           |     |     |     |
| ----------- | -------------------------------------------- | --------- | --- | --- | --- |
| Hor Pixels  | = 1400;                                      | // Pixels |     |     |     |
1050;
| Ver Pixels    | =          | // Lines |        |        |        |
| ------------- | ---------- | -------- | ------ | ------ | ------ |
| Hor Frequency | = 64.744;  | // kHz   | = 15.4 | usec / | line   |
| Ver Frequency | = 59.948;  | // Hz    | = 16.7 | msec / | frame  |
| Pixel Clock   | = 101.000; | // MHz   | = 9.9  | nsec   | ± 0.5% |
8;
| Character Width   | =                | // Pixels | = 79.2     | nsec      |             |
| ----------------- | ---------------- | --------- | ---------- | --------- | ----------- |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =         | 1.0 %       |
| Hor Sync Polarity | = POSITIVE;      | // HBlank | = 10.3%    | of HTotal |             |
| Ver Sync Polarity | = NEGATIVE       | // VBlank | = 2.8%     | of VTotal |             |
| Hor Total Time    | = 15.446;        | // (usec) | = 195      | chars =   | 1560 Pixels |
| Hor Addr Time     | = 13.861;        | // (usec) | = 175      | chars =   | 1400 Pixels |
Hor Blank Start = 13.861; // (usec) = 175 chars = 1400 Pixels
| Hor Blank Time    | = 1.584;  | // (usec) | =     | 20 chars = | 160 Pixels  |
| ----------------- | --------- | --------- | ----- | ---------- | ----------- |
| Hor Sync Start    | = 14.337; | // (usec) | = 181 | chars =    | 1448 Pixels |
| // H Right Border | = 0.000;  | // (usec) | =     | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.475;  | // (usec) | =     | 6 chars =  | 48 Pixels   |
| Hor Sync Time     | = 0.317;  | // (usec) | =     | 4 chars =  | 32 Pixels   |
10
| // H Back Porch  | = 0.792; | // (usec) | =   | chars =   | 80 Pixels |
| ---------------- | -------- | --------- | --- | --------- | --------- |
| // H Left Border | = 0.000; | // (usec) | =   | 0 chars = | 0 Pixels  |
Ver Total Time = 16.681; // (msec) = 1080 lines HT – (1.06xHA)
| Ver Addr Time      | = 16.218; | // (msec) | = 1050 | lines    | = 0.75 |
| ------------------ | --------- | --------- | ------ | -------- | ------ |
| Ver Blank Start    | = 16.218; | // (msec) | = 1050 | lines    |        |
| Ver Blank Time     | = 0.463;  | // (msec) | =      | 30 lines |        |
| Ver Sync Start     | = 16.264; | // (msec) | = 1053 | lines    |        |
| // V Bottom Border | = 0.000;  | // (msec) | =      | 0 lines  |        |
| // V Front Porch   | = 0.046;  | // (msec) | =      | 3 lines  |        |
| Ver Sync Time      | = 0.062;  | // (msec) | =      | 4 lines  |        |
| // V Back Porch    | = 0.355;  | // (msec) | = 23   | lines    |        |
| // V Top Border    | = 0.000;  | // (msec) | =      | 0 lines  |        |

Definition of Terms: Refer to section 3.2.

VESA MONITOR TIMING STANDARD
Adopted:  8/21/03
Resolution:  1400 x 1050 at 60 Hz (non-interlaced)
EDID ID:  DMT ID: 2Ah; Std. 2 Byte Code: (90, 40)h; CVT 3 Byte Code: (0C, 20, 28)h
Method:  CVT Compliant

Detailed Timing Parameters
| Timing Name       | = 1400 x 1050 @ 60Hz;; |           |            |           |             |
| ----------------- | ---------------------- | --------- | ---------- | --------- | ----------- |
| Hor Pixels        | = 1400;                | // Pixels |            |           |             |
| Ver Pixels        | = 1050;                | // Lines  |            |           |             |
| Hor Frequency     | = 65.317;              | // kHz    | = 15.3     | usec /    | line        |
| Ver Frequency     | = 59.978;              | // Hz     | = 16.7     | msec /    | frame       |
| Pixel Clock       | = 121.750;             | // MHz    | = 8.2      | nsec      | ± 0.5%      |
| Character Width   | = 8;                   | // Pixels | = 65.7     | nsec      |             |
| Scan Type         | = NONINTERLACED;       |           | // H Phase | =         | 3.9 %       |
| Hor Sync Polarity | = NEGATIVE             | // HBlank | = 24.9%    | of HTotal |             |
| Ver Sync Polarity | = POSITIVE;            | // VBlank | = 3.6%     | of VTotal |             |
| Hor Total Time    | = 15.310;              | // (usec) | = 233      | chars =   | 1864 Pixels |
| Hor Addr Time     | = 11.499;              | // (usec) | = 175      | chars =   | 1400 Pixels |
Hor Blank Start = 11.499; // (usec) = 175 chars = 1400 Pixels
| Hor Blank Time    | = 3.811;  | // (usec) | =     | 58 chars = | 464 Pixels  |
| ----------------- | --------- | --------- | ----- | ---------- | ----------- |
| Hor Sync Start    | = 12.222; | // (usec) | = 186 | chars =    | 1488 Pixels |
| // H Right Border | = 0.000;  | // (usec) | =     | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.723;  | // (usec) | = 11  | chars =    | 88 Pixels   |
| Hor Sync Time     | = 1.183;  | // (usec) | = 18  | chars =    | 144 Pixels  |
| // H Back Porch   | = 1.906;  | // (usec) | = 29  | chars =    | 232 Pixels  |
| // H Left Border  | = 0.000;  | // (usec) | =     | 0 chars =  | 0 Pixels    |
Ver Total Time = 16.673; // (msec) = 1089 lines HT – (1.06xHA)
| Ver Addr Time      | = 16.076; | // (msec) | = 1050 | lines    | = 3.12 |
| ------------------ | --------- | --------- | ------ | -------- | ------ |
| Ver Blank Start    | = 16.076; | // (msec) | = 1050 | lines    |        |
| Ver Blank Time     | = 0.597;  | // (msec) | =      | 39 lines |        |
| Ver Sync Start     | = 16.121; | // (msec) | = 1053 | lines    |        |
| // V Bottom Border | = 0.000;  | // (msec) | =      | 0 lines  |        |
| // V Front Porch   | = 0.046;  | // (msec) | =      | 3 lines  |        |
| Ver Sync Time      | = 0.061;  | // (msec) | =      | 4 lines  |        |
| // V Back Porch    | = 0.490;  | // (msec) | = 32   | lines    |        |
| // V Top Border    | = 0.000;  | // (msec) | =      | 0 lines  |        |

Definition of Terms: Refer to section 3.4.

VESA MONITOR TIMING STANDARD
Adopted:  8/21/03
Resolution:  1400 x 1050 at 75 Hz (non-interlaced)
EDID ID:  DMT ID: 2Bh; Std. 2 Byte Code: (90, 4F)h; CVT 3 Byte Code: (0C, 20, 44)h
Method:  CVT Compliant

Detailed Timing Parameters
| Timing Name       | = 1400 x 1050 @ 75Hz; |           |            |            |             |
| ----------------- | --------------------- | --------- | ---------- | ---------- | ----------- |
| Hor Pixels        | = 1400;               | // Pixels |            |            |             |
| Ver Pixels        | = 1050;               | // Lines  |            |            |             |
| Hor Frequency     | = 82.278;             | // kHz    | = 12.2     | usec /     | line        |
| Ver Frequency     | = 74.867;             | // Hz     | = 13.4     | msec /     | frame       |
| Pixel Clock       | = 156.000;            | // MHz    | = 6.4      | nsec       | ± 0.5%      |
| Character Width   | = 8;                  | // Pixels | = 51.3     | nsec       |             |
| Scan Type         | = NONINTERLACED;      |           | // H Phase | =          | 3.8 %       |
| Hor Sync Polarity | = NEGATIVE            | // HBlank | = 26.2%    | of HTotal  |             |
| Ver Sync Polarity | = POSITIVE;           | // VBlank | = 4.5%     | of VTotal  |             |
| Hor Total Time    | = 12.154;             | // (usec) | = 237      | chars =    | 1896 Pixels |
| Hor Addr Time     | = 8.974;              | // (usec) | = 175      | chars =    | 1400 Pixels |
| Hor Blank Start   | = 8.974;              | // (usec) | = 175      | chars =    | 1400 Pixels |
| Hor Blank Time    | = 3.179;              | // (usec) | =          | 62 chars = | 496 Pixels  |
| Hor Sync Start    | = 9.641;              | // (usec) | = 188      | chars =    | 1504 Pixels |
| // H Right Border | = 0.000;              | // (usec) | =          | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.667;              | // (usec) | = 13       | chars =    | 104 Pixels  |
| Hor Sync Time     | = 0.923;              | // (usec) | = 18       | chars =    | 144 Pixels  |
| // H Back Porch   | = 1.590;              | // (usec) | = 31       | chars =    | 248 Pixels  |
| // H Left Border  | = 0.000;              | // (usec) | =          | 0 chars =  | 0 Pixels    |
Ver Total Time = 13.357; // (msec) = 1099 lines HT – (1.06xHA)
| Ver Addr Time      | = 12.762; | // (msec) | = 1050 | lines    | = 2.64 |
| ------------------ | --------- | --------- | ------ | -------- | ------ |
| Ver Blank Start    | = 12.762; | // (msec) | = 1050 | lines    |        |
| Ver Blank Time     | = 0.596;  | // (msec) | =      | 49 lines |        |
| Ver Sync Start     | = 12.798; | // (msec) | = 1053 | lines    |        |
| // V Bottom Border | = 0.000;  | // (msec) | =      | 0 lines  |        |
| // V Front Porch   | = 0.036;  | // (msec) | =      | 3 lines  |        |
| Ver Sync Time      | = 0.049;  | // (msec) | =      | 4 lines  |        |
| // V Back Porch    | = 0.510;  | // (msec) | = 42   | lines    |        |
| // V Top Border    | = 0.000;  | // (msec) | =      | 0 lines  |        |

Definition of Terms: Refer to section 3.4.

VESA MONITOR TIMING STANDARD
Adopted:  8/21/03
Resolution:  1400 x 1050 at 85 Hz (non-interlaced)
EDID ID:  DMT ID: 2Ch; Std. 2 Byte Code: (90, 59)h; CVT 3 Byte Code: (0C, 20, 62)h
Method:  CVT Compliant

Detailed Timing Parameters
| Timing Name       | = 1400 x 1050 @ 85Hz; |           |            |            |             |
| ----------------- | --------------------- | --------- | ---------- | ---------- | ----------- |
| Hor Pixels        | = 1400;               | // Pixels |            |            |             |
| Ver Pixels        | = 1050;               | // Lines  |            |            |             |
| Hor Frequency     | = 93.881;             | // kHz    | = 10.7     | usec /     | line        |
| Ver Frequency     | = 84.960;             | // Hz     | = 11.8     | msec /     | frame       |
| Pixel Clock       | = 179.500;            | // MHz    | = 5.6      | nsec       | ± 0.5%      |
| Character Width   | = 8;                  | // Pixels | = 44.6     | nsec       |             |
| Scan Type         | = NONINTERLACED;      |           | // H Phase | =          | 4.0 %       |
| Hor Sync Polarity | = NEGATIVE            | // HBlank | = 26.8%    | of HTotal  |             |
| Ver Sync Polarity | = POSITIVE;           | // VBlank | = 5.0%     | of VTotal  |             |
| Hor Total Time    | = 10.652;             | // (usec) | = 239      | chars =    | 1912 Pixels |
| Hor Addr Time     | = 7.799;              | // (usec) | = 175      | chars =    | 1400 Pixels |
| Hor Blank Start   | = 7.799;              | // (usec) | = 175      | chars =    | 1400 Pixels |
| Hor Blank Time    | = 2.852;              | // (usec) | =          | 64 chars = | 512 Pixels  |
| Hor Sync Start    | = 8.379;              | // (usec) | = 188      | chars =    | 1504 Pixels |
| // H Right Border | = 0.000;              | // (usec) | =          | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.579;              | // (usec) | = 13       | chars =    | 104 Pixels  |
| Hor Sync Time     | = 0.847;              | // (usec) | = 19       | chars =    | 152 Pixels  |
| // H Back Porch   | = 1.426;              | // (usec) | = 32       | chars =    | 256 Pixels  |
| // H Left Border  | = 0.000;              | // (usec) | =          | 0 chars =  | 0 Pixels    |
Ver Total Time = 11.770; // (msec) = 1105 lines HT – (1.06xHA)
| Ver Addr Time      | = 11.184; | // (msec) | = 1050 | lines    | = 2.38 |
| ------------------ | --------- | --------- | ------ | -------- | ------ |
| Ver Blank Start    | = 11.184; | // (msec) | = 1050 | lines    |        |
| Ver Blank Time     | = 0.586;  | // (msec) | =      | 55 lines |        |
| Ver Sync Start     | = 11.216; | // (msec) | = 1053 | lines    |        |
| // V Bottom Border | = 0.000;  | // (msec) | =      | 0 lines  |        |
| // V Front Porch   | = 0.032;  | // (msec) | =      | 3 lines  |        |
| Ver Sync Time      | = 0.043;  | // (msec) | =      | 4 lines  |        |
| // V Back Porch    | = 0.511;  | // (msec) | = 48   | lines    |        |
| // V Top Border    | = 0.000;  | // (msec) | =      | 0 lines  |        |

Definition of Terms: Refer to section 3.4.

VESA MONITOR TIMING STANDARD
Adopted:  5/1/07
Resolution:  1400 x 1050 at 120 Hz (non-interlaced) REDUCED BLANKING
EDID ID:  DMT ID: 2Dh; Std. 2 Byte Code: n/a; CVT 3 Byte Code: n/a
Method:  Generated using CVT (Reduced Blanking) Formula

Detailed Timing Parameters
| Timing Name       | = 1400 x 1050 @ 120Hz CVT (Reduced Blanking); |           |            |            |             |
| ----------------- | --------------------------------------------- | --------- | ---------- | ---------- | ----------- |
| Hor Pixels        | = 1400;                                       | // Pixels |            |            |             |
| Ver Pixels        | = 1050;                                       | // Lines  |            |            |             |
| Hor Frequency     | = 133.333;                                    | // kHz    | = 7.5      | usec /     | line        |
| Ver Frequency     | = 119.904;                                    | // Hz     | = 8.3      | msec /     | frame       |
| Pixel Clock       | = 208.000;                                    | // MHz    | = 4.8      | nsec       | ± 0.5%      |
| Character Width   | = 8;                                          | // Pixels | = 38.5     | nsec       |             |
| Scan Type         | = NONINTERLACED;                              |           | // H Phase | =          | 1.0 %       |
| Hor Sync Polarity | = POSITIVE;                                   | // HBlank | = 10.3%    | of HTotal  |             |
| Ver Sync Polarity | = NEGATIVE                                    | // VBlank | = 5.6%     | of VTotal  |             |
| Hor Total Time    | = 7.500;                                      | // (usec) | = 195      | chars =    | 1560 Pixels |
| Hor Addr Time     | = 6.731;                                      | // (usec) | = 175      | chars =    | 1400 Pixels |
| Hor Blank Start   | = 6.731;                                      | // (usec) | = 175      | chars =    | 1400 Pixels |
| Hor Blank Time    | = 0.769;                                      | // (usec) | =          | 20 chars = | 160 Pixels  |
| Hor Sync Start    | = 6.962;                                      | // (usec) | = 181      | chars =    | 1448 Pixels |
| // H Right Border | = 0.000;                                      | // (usec) | =          | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.231;                                      | // (usec) | =          | 6 chars =  | 48 Pixels   |
| Hor Sync Time     | = 0.154;                                      | // (usec) | =          | 4 chars =  | 32 Pixels   |
| // H Back Porch   | = 0.385;                                      | // (usec) | = 10       | chars =    | 80 Pixels   |
| // H Left Border  | = 0.000;                                      | // (usec) | =          | 0 chars =  | 0 Pixels    |
Ver Total Time = 8.340; // (msec) = 1112 lines HT – (1.06xHA)
| Ver Addr Time      | = 7.875; | // (msec) | = 1050 | lines    | = 0.37 |
| ------------------ | -------- | --------- | ------ | -------- | ------ |
| Ver Blank Start    | = 7.875; | // (msec) | = 1050 | lines    |        |
| Ver Blank Time     | = 0.465; | // (msec) | =      | 62 lines |        |
| Ver Sync Start     | = 7.898; | // (msec) | = 1053 | lines    |        |
| // V Bottom Border | = 0.000; | // (msec) | =      | 0 lines  |        |
| // V Front Porch   | = 0.023; | // (msec) | =      | 3 lines  |        |
| Ver Sync Time      | = 0.030; | // (msec) | =      | 4 lines  |        |
| // V Back Porch    | = 0.413; | // (msec) | = 55   | lines    |        |
| // V Top Border    | = 0.000; | // (msec) | =      | 0 lines  |        |

Definition of Terms: Refer to section 3.2.

VESA MONITOR TIMING STANDARD
Adopted:  10/24/04
Resolution:  1440 x 900 at 60 Hz (non-interlaced) REDUCED BLANKING
EDID ID:  DMT ID: 2Eh; Std. 2 Byte Code: n/a; CVT 3 Byte Code: (C1, 18, 21)h
Method:  CVT Reduced Blanking

Detailed Timing Parameters
| Timing Name | = 1440 x 900 @ 60Hz CVT (Reduced Blanking); |     |     |     |     |
| ----------- | ------------------------------------------- | --- | --- | --- | --- |
1440;
| Hor Pixels        | =                | // Pixels |            |           |        |
| ----------------- | ---------------- | --------- | ---------- | --------- | ------ |
| Ver Pixels        | = 900;           | // Lines  |            |           |        |
| Hor Frequency     | = 55.469;        | // kHz    | = 18.0     | usec /    | line   |
| Ver Frequency     | = 59.901;        | // Hz     | = 16.7     | msec /    | frame  |
| Pixel Clock       | = 88.750;        | // MHz    | = 11.3     | nsec      | ± 0.5% |
| Character Width   | = 8;             | // Pixels | = 90.1     | nsec      |        |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =         | 1.0 %  |
| Hor Sync Polarity | = POSITIVE;      | // HBlank | = 10.0%    | of HTotal |        |
NEGATIVE
| Ver Sync Polarity | =         | // VBlank | = 2.8% | of VTotal |             |
| ----------------- | --------- | --------- | ------ | --------- | ----------- |
| Hor Total Time    | = 18.028; | // (usec) | = 200  | chars =   | 1600 Pixels |
| Hor Addr Time     | = 16.225; | // (usec) | = 180  | chars =   | 1440 Pixels |
Hor Blank Start = 16.225; // (usec) = 180 chars = 1440 Pixels
| Hor Blank Time    | = 1.803;  | // (usec) | =     | 20 chars = | 160 Pixels  |
| ----------------- | --------- | --------- | ----- | ---------- | ----------- |
| Hor Sync Start    | = 16.766; | // (usec) | = 186 | chars =    | 1488 Pixels |
| // H Right Border | = 0.000;  | // (usec) | =     | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.541;  | // (usec) | =     | 6 chars =  | 48 Pixels   |
| Hor Sync Time     | = 0.361;  | // (usec) | =     | 4 chars =  | 32 Pixels   |
| // H Back Porch   | = 0.901;  | // (usec) | =     | 10 chars = | 80 Pixels   |
| // H Left Border  | = 0.000;  | // (usec) | =     | 0 chars =  | 0 Pixels    |
Ver Total Time = 16.694; // (msec) = 926 lines HT – (1.06xHA)
| Ver Addr Time      | = 16.225; | // (msec) | = 900 | lines    | = 0.83 |
| ------------------ | --------- | --------- | ----- | -------- | ------ |
| Ver Blank Start    | = 16.225; | // (msec) | = 900 | lines    |        |
| Ver Blank Time     | = 0.469;  | // (msec) | =     | 26 lines |        |
| Ver Sync Start     | = 16.279; | // (msec) | = 903 | lines    |        |
| // V Bottom Border | = 0.000;  | // (msec) | =     | 0 lines  |        |
| // V Front Porch   | = 0.054;  | // (msec) | =     | 3 lines  |        |
| Ver Sync Time      | = 0.108;  | // (msec) | =     | 6 lines  |        |
17
| // V Back Porch | = 0.306; | // (msec) | =   | lines   |     |
| --------------- | -------- | --------- | --- | ------- | --- |
| // V Top Border | = 0.000; | // (msec) | =   | 0 lines |     |

Definition of Terms: Refer to section 3.2.

VESA MONITOR TIMING STANDARD
Adopted:  10/24/04
Resolution:  1440 x 900 at 60 Hz (non-interlaced)
EDID ID:  DMT ID: 2Fh; Std. 2 Byte Code: (95, 00)h; CVT 3 Byte Code: (C1, 18, 28)h
Method:  CVT Compliant

Detailed Timing Parameters
| Timing Name   | = 1440 x 900 @ 60Hz; |           |        |        |       |
| ------------- | -------------------- | --------- | ------ | ------ | ----- |
| Hor Pixels    | = 1440;              | // Pixels |        |        |       |
| Ver Pixels    | = 900;               | // Lines  |        |        |       |
| Hor Frequency | = 55.935;            | // kHz    | = 17.9 | usec / | line  |
| Ver Frequency | = 59.887;            | // Hz     | = 16.7 | msec / | frame |
106.500;
| Pixel Clock       | =                | // MHz    | = 9.4      | nsec      | ± 0.5%      |
| ----------------- | ---------------- | --------- | ---------- | --------- | ----------- |
| Character Width   | = 8;             | // Pixels | = 75.1     | nsec      |             |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =         | 4.0 %       |
| Hor Sync Polarity | = NEGATIVE       | // HBlank | = 24.4%    | of HTotal |             |
| Ver Sync Polarity | = POSITIVE;      | // VBlank | = 3.6%     | of VTotal |             |
| Hor Total Time    | = 17.878;        | // (usec) | = 238      | chars =   | 1904 Pixels |
| Hor Addr Time     | = 13.521;        | // (usec) | = 180      | chars =   | 1440 Pixels |
Hor Blank Start = 13.521; // (usec) = 180 chars = 1440 Pixels
| Hor Blank Time    | = 4.357;  | // (usec) | =     | 58 chars = | 464 Pixels  |
| ----------------- | --------- | --------- | ----- | ---------- | ----------- |
| Hor Sync Start    | = 14.272; | // (usec) | = 190 | chars =    | 1520 Pixels |
| // H Right Border | = 0.000;  | // (usec) | =     | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.751;  | // (usec) | =     | 10 chars = | 80 Pixels   |
| Hor Sync Time     | = 1.427;  | // (usec) | =     | 19 chars = | 152 Pixels  |
29
| // H Back Porch  | = 2.178; | // (usec) | =   | chars =   | 232 Pixels |
| ---------------- | -------- | --------- | --- | --------- | ---------- |
| // H Left Border | = 0.000; | // (usec) | =   | 0 chars = | 0 Pixels   |
Ver Total Time = 16.698; // (msec) = 934 lines HT – (1.06xHA)
| Ver Addr Time      | = 16.090; | // (msec) | = 900 | lines    | = 3.55 |
| ------------------ | --------- | --------- | ----- | -------- | ------ |
| Ver Blank Start    | = 16.090; | // (msec) | = 900 | lines    |        |
| Ver Blank Time     | = 0.608;  | // (msec) | =     | 34 lines |        |
| Ver Sync Start     | = 16.144; | // (msec) | = 903 | lines    |        |
| // V Bottom Border | = 0.000;  | // (msec) | =     | 0 lines  |        |
3
| // V Front Porch | = 0.054; | // (msec) | =   | lines    |     |
| ---------------- | -------- | --------- | --- | -------- | --- |
| Ver Sync Time    | = 0.107; | // (msec) | =   | 6 lines  |     |
| // V Back Porch  | = 0.447; | // (msec) | =   | 25 lines |     |
| // V Top Border  | = 0.000; | // (msec) | =   | 0 lines  |     |

Definition of Terms: Refer to section 3.4.

VESA MONITOR TIMING STANDARD
Adopted:  10/24/04
Resolution:  1440 x 900 at 75 Hz (non-interlaced)
EDID ID:  DMT ID: 30h; Std. 2 Byte Code: (95, 0F)h; CVT 3 Byte Code: (C1, 18, 44)h
Method:  CVT Compliant

Detailed Timing Parameters
| Timing Name   | = 1440 x 900 @ 75Hz; |           |        |        |       |
| ------------- | -------------------- | --------- | ------ | ------ | ----- |
| Hor Pixels    | = 1440;              | // Pixels |        |        |       |
| Ver Pixels    | = 900;               | // Lines  |        |        |       |
| Hor Frequency | = 70.635;            | // kHz    | = 14.2 | usec / | line  |
| Ver Frequency | = 74.984;            | // Hz     | = 13.3 | msec / | frame |
136.750;
| Pixel Clock       | =                | // MHz    | = 7.3      | nsec      | ± 0.5%      |
| ----------------- | ---------------- | --------- | ---------- | --------- | ----------- |
| Character Width   | = 8;             | // Pixels | = 58.5     | nsec      |             |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =         | 3.9 %       |
| Hor Sync Polarity | = NEGATIVE       | // HBlank | = 25.6%    | of HTotal |             |
| Ver Sync Polarity | = POSITIVE;      | // VBlank | = 4.5%     | of VTotal |             |
| Hor Total Time    | = 14.157;        | // (usec) | = 242      | chars =   | 1936 Pixels |
| Hor Addr Time     | = 10.530;        | // (usec) | = 180      | chars =   | 1440 Pixels |
Hor Blank Start = 10.530; // (usec) = 180 chars = 1440 Pixels
| Hor Blank Time    | = 3.627;  | // (usec) | =     | 62 chars = | 496 Pixels  |
| ----------------- | --------- | --------- | ----- | ---------- | ----------- |
| Hor Sync Start    | = 11.232; | // (usec) | = 192 | chars =    | 1536 Pixels |
| // H Right Border | = 0.000;  | // (usec) | =     | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.702;  | // (usec) | =     | 12 chars = | 96 Pixels   |
| Hor Sync Time     | = 1.112;  | // (usec) | =     | 19 chars = | 152 Pixels  |
31
| // H Back Porch  | = 1.814; | // (usec) | =   | chars =   | 248 Pixels |
| ---------------- | -------- | --------- | --- | --------- | ---------- |
| // H Left Border | = 0.000; | // (usec) | =   | 0 chars = | 0 Pixels   |
Ver Total Time = 13.336; // (msec) = 942 lines HT – (1.06xHA)
| Ver Addr Time      | = 12.741; | // (msec) | = 900 | lines    | = 3 |
| ------------------ | --------- | --------- | ----- | -------- | --- |
| Ver Blank Start    | = 12.741; | // (msec) | = 900 | lines    |     |
| Ver Blank Time     | = 0.595;  | // (msec) | =     | 42 lines |     |
| Ver Sync Start     | = 12.784; | // (msec) | = 903 | lines    |     |
| // V Bottom Border | = 0.000;  | // (msec) | =     | 0 lines  |     |
3
| // V Front Porch | = 0.042; | // (msec) | =   | lines    |     |
| ---------------- | -------- | --------- | --- | -------- | --- |
| Ver Sync Time    | = 0.085; | // (msec) | =   | 6 lines  |     |
| // V Back Porch  | = 0.467; | // (msec) | =   | 33 lines |     |
| // V Top Border  | = 0.000; | // (msec) | =   | 0 lines  |     |

Definition of Terms: Refer to section 3.4.

VESA MONITOR TIMING STANDARD
Adopted:  10/24/04
Resolution:  1440 x 900 at 85 Hz (non-interlaced)
EDID ID:  DMT ID: 31h; Std. 2 Byte Code: (95, 19)h; CVT 3 Byte Code: (C1, 18, 68)h
Method:  CVT Compliant

Detailed Timing Parameters
| Timing Name   | = 1440 x 900 @ 85Hz; |           |        |        |       |
| ------------- | -------------------- | --------- | ------ | ------ | ----- |
| Hor Pixels    | = 1440;              | // Pixels |        |        |       |
| Ver Pixels    | = 900;               | // Lines  |        |        |       |
| Hor Frequency | = 80.430;            | // kHz    | = 12.4 | usec / | line  |
| Ver Frequency | = 84.842;            | // Hz     | = 11.8 | msec / | frame |
157.000;
| Pixel Clock       | =                | // MHz    | = 6.4      | nsec       | ± 0.5%      |
| ----------------- | ---------------- | --------- | ---------- | ---------- | ----------- |
| Character Width   | = 8;             | // Pixels | = 51.0     | nsec       |             |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =          | 3.9 %       |
| Hor Sync Polarity | = NEGATIVE       | // HBlank | = 26.2%    | of HTotal  |             |
| Ver Sync Polarity | = POSITIVE;      | // VBlank | = 5.1%     | of VTotal  |             |
| Hor Total Time    | = 12.433;        | // (usec) | = 244      | chars =    | 1952 Pixels |
| Hor Addr Time     | = 9.172;         | // (usec) | = 180      | chars =    | 1440 Pixels |
| Hor Blank Start   | = 9.172;         | // (usec) | = 180      | chars =    | 1440 Pixels |
| Hor Blank Time    | = 3.261;         | // (usec) | =          | 64 chars = | 512 Pixels  |
| Hor Sync Start    | = 9.834;         | // (usec) | = 193      | chars =    | 1544 Pixels |
| // H Right Border | = 0.000;         | // (usec) | =          | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.662;         | // (usec) | =          | 13 chars = | 104 Pixels  |
| Hor Sync Time     | = 0.968;         | // (usec) | =          | 19 chars = | 152 Pixels  |
32
| // H Back Porch  | = 1.631; | // (usec) | =   | chars =   | 256 Pixels |
| ---------------- | -------- | --------- | --- | --------- | ---------- |
| // H Left Border | = 0.000; | // (usec) | =   | 0 chars = | 0 Pixels   |
Ver Total Time = 11.787; // (msec) = 948 lines HT – (1.06xHA)
| Ver Addr Time      | = 11.190; | // (msec) | = 900 | lines    | = 2.71 |
| ------------------ | --------- | --------- | ----- | -------- | ------ |
| Ver Blank Start    | = 11.190; | // (msec) | = 900 | lines    |        |
| Ver Blank Time     | = 0.597;  | // (msec) | =     | 48 lines |        |
| Ver Sync Start     | = 11.227; | // (msec) | = 903 | lines    |        |
| // V Bottom Border | = 0.000;  | // (msec) | =     | 0 lines  |        |
3
| // V Front Porch | = 0.037; | // (msec) | =   | lines    |     |
| ---------------- | -------- | --------- | --- | -------- | --- |
| Ver Sync Time    | = 0.075; | // (msec) | =   | 6 lines  |     |
| // V Back Porch  | = 0.485; | // (msec) | =   | 39 lines |     |
| // V Top Border  | = 0.000; | // (msec) | =   | 0 lines  |     |

Definition of Terms: Refer to section 3.4.

VESA MONITOR TIMING STANDARD
Adopted:  5/1/07
Resolution:  1440 x 900 at 120 Hz (non-interlaced) REDUCED BLANKING
EDID ID:  DMT ID: 32h; Std. 2 Byte Code: n/a; CVT 3 Byte Code: n/a
Method:  Generated using CVT (Reduced Blanking) Formula

Detailed Timing Parameters
| Timing Name | = 1440 x 900 @ 120Hz CVT (Reduced Blanking); |           |     |     |     |
| ----------- | -------------------------------------------- | --------- | --- | --- | --- |
| Hor Pixels  | = 1440;                                      | // Pixels |     |     |     |
900;
| Ver Pixels    | =          | // Lines |       |        |        |
| ------------- | ---------- | -------- | ----- | ------ | ------ |
| Hor Frequency | = 114.219; | // kHz   | = 8.8 | usec / | line   |
| Ver Frequency | = 119.852; | // Hz    | = 8.3 | msec / | frame  |
| Pixel Clock   | = 182.750; | // MHz   | = 5.5 | nsec   | ± 0.5% |
8;
| Character Width   | =                | // Pixels | = 43.8     | nsec       |             |
| ----------------- | ---------------- | --------- | ---------- | ---------- | ----------- |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =          | 1.0 %       |
| Hor Sync Polarity | = POSITIVE;      | // HBlank | = 10.0%    | of HTotal  |             |
| Ver Sync Polarity | = NEGATIVE       | // VBlank | = 5.6%     | of VTotal  |             |
| Hor Total Time    | = 8.755;         | // (usec) | = 200      | chars =    | 1600 Pixels |
| Hor Addr Time     | = 7.880;         | // (usec) | = 180      | chars =    | 1440 Pixels |
| Hor Blank Start   | = 7.880;         | // (usec) | = 180      | chars =    | 1440 Pixels |
| Hor Blank Time    | = 0.876;         | // (usec) | =          | 20 chars = | 160 Pixels  |
| Hor Sync Start    | = 8.142;         | // (usec) | = 186      | chars =    | 1488 Pixels |
| // H Right Border | = 0.000;         | // (usec) | =          | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.263;         | // (usec) | =          | 6 chars =  | 48 Pixels   |
| Hor Sync Time     | = 0.175;         | // (usec) | =          | 4 chars =  | 32 Pixels   |
10
| // H Back Porch    | = 0.438; | // (usec) | =     | chars =   | 80 Pixels      |
| ------------------ | -------- | --------- | ----- | --------- | -------------- |
| // H Left Border   | = 0.000; | // (usec) | =     | 0 chars = | 0 Pixels       |
| Ver Total Time     | = 8.344; | // (msec) | = 953 | lines     | HT – (1.06xHA) |
| Ver Addr Time      | = 7.880; | // (msec) | = 900 | lines     | = 0.4          |
| Ver Blank Start    | = 7.880; | // (msec) | = 900 | lines     |                |
| Ver Blank Time     | = 0.464; | // (msec) | =     | 53 lines  |                |
| Ver Sync Start     | = 7.906; | // (msec) | = 903 | lines     |                |
| // V Bottom Border | = 0.000; | // (msec) | =     | 0 lines   |                |
| // V Front Porch   | = 0.026; | // (msec) | =     | 3 lines   |                |
| Ver Sync Time      | = 0.053; | // (msec) | =     | 6 lines   |                |
| // V Back Porch    | = 0.385; | // (msec) | = 44  | lines     |                |
| // V Top Border    | = 0.000; | // (msec) | =     | 0 lines   |                |

Definition of Terms: Refer to section 3.2.
VESA MONITOR TIMING STANDARD
Adopted:  11/17/08
Resolution:  1600 x 900 at 60 Hz (non-interlaced) REDUCED BLANKING
EDID ID:  DMT ID: 53h; Std. 2 Byte Code: A9h, C0h; CVT 3 Byte Code: n/a
Method:  *** NOT CVT COMPLIANT ***

Detailed Timing Parameters
| Timing Name     | = 1600 x 900 @ 60Hz; |           |            |        |        |
| --------------- | -------------------- | --------- | ---------- | ------ | ------ |
| Hor Pixels      | = 1600;              | // Pixels |            |        |        |
| Ver Pixels      | = 900;               | // Lines  |            |        |        |
| Hor Frequency   | = 60.000;            | // KHz    | = 16.7     | usec / | line   |
| Ver Frequency   | = 60.000;            | // Hz     | = 16.7     | msec / | frame  |
| Pixel Clock     | = 108.000;           | // MHz    | = 9.3      | nsec   | ± 0.5% |
| Character Width | = 8;                 | // Pixels | = 74.1     | nsec   |        |
| Scan Type       | = NONINTERLACED;     |           | // H Phase | =      | 2.0 %  |
POSITIVE;
| Hor Sync Polarity | =           | // HBlank | = 11.1% | of HTotal |             |
| ----------------- | ----------- | --------- | ------- | --------- | ----------- |
| Ver Sync Polarity | = POSITIVE; | // VBlank | = 10.0% | of VTotal |             |
| Hor Total Time    | = 16.667;   | // (usec) | = 225   | chars =   | 1800 Pixels |
| Hor Addr Time     | = 14.815;   | // (usec) | = 200   | chars =   | 1600 Pixels |
Hor Blank Start = 14.815; // (usec) = 200 chars = 1600 Pixels
| Hor Blank Time    | = 1.852;  | // (usec) | =     | 25 chars = | 200 Pixels  |
| ----------------- | --------- | --------- | ----- | ---------- | ----------- |
| Hor Sync Start    | = 15.037; | // (usec) | = 203 | chars =    | 1624 Pixels |
| // H Right Border | = 0.000;  | // (usec) | =     | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.222;  | // (usec) | =     | 3 chars =  | 24 Pixels   |
| Hor Sync Time     | = 0.741;  | // (usec) | =     | 10 chars = | 80 Pixels   |
| // H Back Porch   | = 0.889;  | // (usec) | =     | 12 chars = | 96 Pixels   |
| // H Left Border  | = 0.000;  | // (usec) | =     | 0 chars =  | 0 Pixels    |
Ver Total Time = 16.667; // (msec) = 1000 lines HT – (1.06xHA)
| Ver Addr Time   | = 15.000; | // (msec) | = 900 | lines | = 0.96 |
| --------------- | --------- | --------- | ----- | ----- | ------ |
| Ver Blank Start | = 15.000; | // (msec) | = 900 | lines |        |
| Ver Blank Time  | = 1.667;  | // (msec) | = 100 | lines |        |
| Ver Sync Start  | = 15.017; | // (msec) | = 901 | lines |        |
0
| // V Bottom Border | = 0.000; | // (msec) | =   | lines   |     |
| ------------------ | -------- | --------- | --- | ------- | --- |
| // V Front Porch   | = 0.017; | // (msec) | =   | 1 lines |     |
3
| Ver Sync Time   | = 0.050; | // (msec) | =   | lines    |     |
| --------------- | -------- | --------- | --- | -------- | --- |
| // V Back Porch | = 1.600; | // (msec) | =   | 96 lines |     |
0
| // V Top Border | = 0.000; | // (msec) | =   | lines |     |
| --------------- | -------- | --------- | --- | ----- | --- |

Definition of Terms: Refer to Section 3.1

VESA MONITOR TIMING STANDARD

Adopted:  12/18/96
Resolution:  1600 x 1200 at 60 Hz (non-interlaced)
EDID ID:  DMT ID: 33h; Std. 2 Byte Code: (A9, 40)h; CVT 3 Byte Code: n/a
Method:  *** NOT CVT COMPLIANT ***

Detailed Timing Parameters
| Timing Name       | = 1600 x 1200 @ 60Hz; |           |                   |               |
| ----------------- | --------------------- | --------- | ----------------- | ------------- |
| Hor Pixels        | = 1600;               | // Pixels |                   |               |
| Ver Pixels        | = 1200;               | // Lines  |                   |               |
| Hor Frequency     | = 75.000;             | // kHz    | = 13.3 usec       | / line        |
| Ver Frequency     | = 60.000;             | // Hz     | = 16.7 msec       | / frame       |
| Pixel Clock       | = 162.000;            | // MHz    | = 6.2 nsec        | ± 0.5%        |
| Character Width   | = 8;                  | // Pixels | = 49.4 nsec       |               |
| Scan Type         | = NONINTERLACED;      |           | // H Phase        | = 5.6 %       |
| Hor Sync Polarity | = POSITIVE;           | // HBlank | = 25.9% of HTotal |               |
| Ver Sync Polarity | = POSITIVE;           | // VBlank | = 4.0% of VTotal  |               |
| Hor Total Time    | = 13.333;             | // (usec) | = 270 chars       | = 2160 Pixels |
| Hor Addr Time     | = 9.877;              | // (usec) | = 200 chars       | = 1600 Pixels |
| Hor Blank Start   | = 9.877;              | // (usec) | = 200 chars       | = 1600 Pixels |
| Hor Blank Time    | = 3.457;              | // (usec) | = 70 chars        | = 560 Pixels  |
| Hor Sync Start    | = 10.272;             | // (usec) | = 208 chars       | = 1664 Pixels |
| // H Right Border | = 0.000;              | // (usec) | = 0 chars         | = 0 Pixels    |
| // H Front Porch  | = 0.395;              | // (usec) | = 8 chars         | = 64 Pixels   |
| Hor Sync Time     | = 1.185;              | // (usec) | = 24 chars        | = 192 Pixels  |
38
| // H Back Porch  | = 1.877; | // (usec) | = chars   | = 304 Pixels |
| ---------------- | -------- | --------- | --------- | ------------ |
| // H Left Border | = 0.000; | // (usec) | = 0 chars | = 0 Pixels   |
Ver Total Time = 16.667; // (msec) = 1250 lines HT – (1.06xHA)
| Ver Addr Time      | = 16.000; | // (msec) | = 1200 lines | = 2.86 |
| ------------------ | --------- | --------- | ------------ | ------ |
| Ver Blank Start    | = 16.000; | // (msec) | = 1200 lines |        |
| Ver Blank Time     | = 0.667;  | // (msec) | = 50 lines   |        |
| Ver Sync Start     | = 16.013; | // (msec) | = 1201 lines |        |
| // V Bottom Border | = 0.000;  | // (msec) | = 0 lines    |        |
| // V Front Porch   | = 0.013;  | // (msec) | = 1 lines    |        |
| Ver Sync Time      | = 0.040;  | // (msec) | = 3 lines    |        |
| // V Back Porch    | = 0.613;  | // (msec) | = 46 lines   |        |
| // V Top Border    | = 0.000;  | // (msec) | = 0 lines    |        |

Definition of Terms: Refer to section 3.1.

VESA MONITOR TIMING STANDARD
Adopted:  12/18/96
Resolution:  1600 x 1200 at 65 Hz (non-interlaced)
EDID ID:  DMT ID: 34h; Std. 2 Byte Code: (A9, 45)h; CVT 3 Byte Code: n/a
Method:  *** NOT CVT COMPLIANT ***

Detailed Timing Parameters
| Timing Name   | = 1600 x 1200 @ 65Hz; |           |             |         |
| ------------- | --------------------- | --------- | ----------- | ------- |
| Hor Pixels    | = 1600;               | // Pixels |             |         |
| Ver Pixels    | = 1200;               | // Lines  |             |         |
| Hor Frequency | = 81.250;             | // kHz    | = 12.3 usec | / line  |
| Ver Frequency | = 65.000;             | // Hz     | = 15.4 msec | / frame |
175.500;
| Pixel Clock       | =                | // MHz    | = 5.7 nsec        | ± 0.5%        |
| ----------------- | ---------------- | --------- | ----------------- | ------------- |
| Character Width   | = 8;             | // Pixels | = 45.6 nsec       |               |
| Scan Type         | = NONINTERLACED; |           | // H Phase        | = 5.6 %       |
| Hor Sync Polarity | = POSITIVE;      | // HBlank | = 25.9% of HTotal |               |
| Ver Sync Polarity | = POSITIVE;      | // VBlank | = 4.0% of VTotal  |               |
| Hor Total Time    | = 12.308;        | // (usec) | = 270 chars       | = 2160 Pixels |
| Hor Addr Time     | = 9.117;         | // (usec) | = 200 chars       | = 1600 Pixels |
| Hor Blank Start   | = 9.117;         | // (usec) | = 200 chars       | = 1600 Pixels |
| Hor Blank Time    | = 3.191;         | // (usec) | = 70 chars        | = 560 Pixels  |
| Hor Sync Start    | = 9.481;         | // (usec) | = 208 chars       | = 1664 Pixels |
| // H Right Border | = 0.000;         | // (usec) | = 0 chars         | = 0 Pixels    |
| // H Front Porch  | = 0.365;         | // (usec) | = 8 chars         | = 64 Pixels   |
| Hor Sync Time     | = 1.094;         | // (usec) | = 24 chars        | = 192 Pixels  |
38
| // H Back Porch  | = 1.732; | // (usec) | = chars   | = 304 Pixels |
| ---------------- | -------- | --------- | --------- | ------------ |
| // H Left Border | = 0.000; | // (usec) | = 0 chars | = 0 Pixels   |
Ver Total Time = 15.385; // (msec) = 1250 lines HT – (1.06xHA)
| Ver Addr Time      | = 14.769; | // (msec) | = 1200 lines | = 2.64 |
| ------------------ | --------- | --------- | ------------ | ------ |
| Ver Blank Start    | = 14.769; | // (msec) | = 1200 lines |        |
| Ver Blank Time     | = 0.615;  | // (msec) | = 50 lines   |        |
| Ver Sync Start     | = 14.782; | // (msec) | = 1201 lines |        |
| // V Bottom Border | = 0.000;  | // (msec) | = 0 lines    |        |
1
| // V Front Porch | = 0.012; | // (msec) | = lines    |     |
| ---------------- | -------- | --------- | ---------- | --- |
| Ver Sync Time    | = 0.037; | // (msec) | = 3 lines  |     |
| // V Back Porch  | = 0.566; | // (msec) | = 46 lines |     |
| // V Top Border  | = 0.000; | // (msec) | = 0 lines  |     |

Definition of Terms: Refer to section 3.1.

VESA MONITOR TIMING STANDARD
Adopted:  12/18/96
Resolution:  1600 x 1200 at 70 Hz (non-interlaced)
EDID ID:  DMT ID: 35h; Std. 2 Byte Code: (A9, 4A)h; CVT 3 Byte Code: n/a
Method:  *** NOT CVT COMPLIANT ***

Detailed Timing Parameters
| Timing Name   | = 1600 x 1200 @ 70Hz; |           |             |         |
| ------------- | --------------------- | --------- | ----------- | ------- |
| Hor Pixels    | = 1600;               | // Pixels |             |         |
| Ver Pixels    | = 1200;               | // Lines  |             |         |
| Hor Frequency | = 87.500;             | // kHz    | = 11.4 usec | / line  |
| Ver Frequency | = 70.000;             | // Hz     | = 14.3 msec | / frame |
189.000;
| Pixel Clock       | =                | // MHz    | = 5.3 nsec        | ± 0.5%        |
| ----------------- | ---------------- | --------- | ----------------- | ------------- |
| Character Width   | = 8;             | // Pixels | = 42.3 nsec       |               |
| Scan Type         | = NONINTERLACED; |           | // H Phase        | = 5.6 %       |
| Hor Sync Polarity | = POSITIVE;      | // HBlank | = 25.9% of HTotal |               |
| Ver Sync Polarity | = POSITIVE;      | // VBlank | = 4.0% of VTotal  |               |
| Hor Total Time    | = 11.429;        | // (usec) | = 270 chars       | = 2160 Pixels |
| Hor Addr Time     | = 8.466;         | // (usec) | = 200 chars       | = 1600 Pixels |
| Hor Blank Start   | = 8.466;         | // (usec) | = 200 chars       | = 1600 Pixels |
| Hor Blank Time    | = 2.963;         | // (usec) | = 70 chars        | = 560 Pixels  |
| Hor Sync Start    | = 8.804;         | // (usec) | = 208 chars       | = 1664 Pixels |
| // H Right Border | = 0.000;         | // (usec) | = 0 chars         | = 0 Pixels    |
| // H Front Porch  | = 0.339;         | // (usec) | = 8 chars         | = 64 Pixels   |
| Hor Sync Time     | = 1.016;         | // (usec) | = 24 chars        | = 192 Pixels  |
38
| // H Back Porch  | = 1.608; | // (usec) | = chars   | = 304 Pixels |
| ---------------- | -------- | --------- | --------- | ------------ |
| // H Left Border | = 0.000; | // (usec) | = 0 chars | = 0 Pixels   |
Ver Total Time = 14.286; // (msec) = 1250 lines HT – (1.06xHA)
| Ver Addr Time      | = 13.714; | // (msec) | = 1200 lines | = 2.46 |
| ------------------ | --------- | --------- | ------------ | ------ |
| Ver Blank Start    | = 13.714; | // (msec) | = 1200 lines |        |
| Ver Blank Time     | = 0.571;  | // (msec) | = 50 lines   |        |
| Ver Sync Start     | = 13.726; | // (msec) | = 1201 lines |        |
| // V Bottom Border | = 0.000;  | // (msec) | = 0 lines    |        |
1
| // V Front Porch | = 0.011; | // (msec) | = lines    |     |
| ---------------- | -------- | --------- | ---------- | --- |
| Ver Sync Time    | = 0.034; | // (msec) | = 3 lines  |     |
| // V Back Porch  | = 0.526; | // (msec) | = 46 lines |     |
| // V Top Border  | = 0.000; | // (msec) | = 0 lines  |     |

Definition of Terms: Refer to section 3.1.

VESA MONITOR TIMING STANDARD
Adopted:  12/18/96
Resolution:  1600 x 1200 at 75 Hz (non-interlaced)
EDID ID:  DMT ID: 36h; Std. 2 Byte Code: (A9, 4F)h; CVT 3 Byte Code: n/a
Method:  *** NOT CVT COMPLIANT ***

Detailed Timing Parameters
| Timing Name | = 1600 x 1200 @ 75Hz; |     |     |     |
| ----------- | --------------------- | --- | --- | --- |
1600;
| Hor Pixels        | =                | // Pixels |                   |         |
| ----------------- | ---------------- | --------- | ----------------- | ------- |
| Ver Pixels        | = 1200;          | // Lines  |                   |         |
| Hor Frequency     | = 93.750;        | // kHz    | = 10.7 usec       | / line  |
| Ver Frequency     | = 75.000;        | // Hz     | = 13.3 msec       | / frame |
| Pixel Clock       | = 202.500;       | // MHz    | = 4.9 nsec        | ± 0.5%  |
| Character Width   | = 8;             | // Pixels | = 39.5 nsec       |         |
| Scan Type         | = NONINTERLACED; |           | // H Phase        | = 5.6 % |
| Hor Sync Polarity | = POSITIVE;      | // HBlank | = 25.9% of HTotal |         |
POSITIVE;
| Ver Sync Polarity | =         | // VBlank | = 4.0% of VTotal |               |
| ----------------- | --------- | --------- | ---------------- | ------------- |
| Hor Total Time    | = 10.667; | // (usec) | = 270 chars      | = 2160 Pixels |
| Hor Addr Time     | = 7.901;  | // (usec) | = 200 chars      | = 1600 Pixels |
| Hor Blank Start   | = 7.901;  | // (usec) | = 200 chars      | = 1600 Pixels |
| Hor Blank Time    | = 2.765;  | // (usec) | = 70 chars       | = 560 Pixels  |
| Hor Sync Start    | = 8.217;  | // (usec) | = 208 chars      | = 1664 Pixels |
| // H Right Border | = 0.000;  | // (usec) | = 0 chars        | = 0 Pixels    |
| // H Front Porch  | = 0.316;  | // (usec) | = 8 chars        | = 64 Pixels   |
| Hor Sync Time     | = 0.948;  | // (usec) | = 24 chars       | = 192 Pixels  |
| // H Back Porch   | = 1.501;  | // (usec) | = 38 chars       | = 304 Pixels  |
| // H Left Border  | = 0.000;  | // (usec) | = 0 chars        | = 0 Pixels    |
Ver Total Time = 13.333; // (msec) = 1250 lines HT – (1.06xHA)
| Ver Addr Time      | = 12.800; | // (msec) | = 1200 lines | = 2.29 |
| ------------------ | --------- | --------- | ------------ | ------ |
| Ver Blank Start    | = 12.800; | // (msec) | = 1200 lines |        |
| Ver Blank Time     | = 0.533;  | // (msec) | = 50 lines   |        |
| Ver Sync Start     | = 12.811; | // (msec) | = 1201 lines |        |
| // V Bottom Border | = 0.000;  | // (msec) | = 0 lines    |        |
| // V Front Porch   | = 0.011;  | // (msec) | = 1 lines    |        |
| Ver Sync Time      | = 0.032;  | // (msec) | = 3 lines    |        |
46
| // V Back Porch | = 0.491; | // (msec) | = lines   |     |
| --------------- | -------- | --------- | --------- | --- |
| // V Top Border | = 0.000; | // (msec) | = 0 lines |     |

Definition of Terms: Refer to section 3.1.

VESA MONITOR TIMING STANDARD
Adopted:  12/18/96
Resolution:  1600 x 1200 at 85 Hz (non-interlaced)
EDID ID:  DMT ID: 37h; Std. 2 Byte Code: (A9, 59)h; CVT 3 Byte Code: n/a
Method:  *** NOT CVT COMPLIANT ***

Detailed Timing Parameters
| Timing Name   | = 1600 x 1200 @ 85Hz; |           |             |         |
| ------------- | --------------------- | --------- | ----------- | ------- |
| Hor Pixels    | = 1600;               | // Pixels |             |         |
| Ver Pixels    | = 1200;               | // Lines  |             |         |
| Hor Frequency | = 106.250;            | // kHz    | = 9.4 usec  | / line  |
| Ver Frequency | = 85.000;             | // Hz     | = 11.8 msec | / frame |
229.500;
| Pixel Clock       | =                | // MHz    | = 4.4 nsec        | ± 0.5%        |
| ----------------- | ---------------- | --------- | ----------------- | ------------- |
| Character Width   | = 8;             | // Pixels | = 34.9 nsec       |               |
| Scan Type         | = NONINTERLACED; |           | // H Phase        | = 5.6 %       |
| Hor Sync Polarity | = POSITIVE;      | // HBlank | = 25.9% of HTotal |               |
| Ver Sync Polarity | = POSITIVE;      | // VBlank | = 4.0% of VTotal  |               |
| Hor Total Time    | = 9.412;         | // (usec) | = 270 chars       | = 2160 Pixels |
| Hor Addr Time     | = 6.972;         | // (usec) | = 200 chars       | = 1600 Pixels |
| Hor Blank Start   | = 6.972;         | // (usec) | = 200 chars       | = 1600 Pixels |
| Hor Blank Time    | = 2.440;         | // (usec) | = 70 chars        | = 560 Pixels  |
| Hor Sync Start    | = 7.251;         | // (usec) | = 208 chars       | = 1664 Pixels |
| // H Right Border | = 0.000;         | // (usec) | = 0 chars         | = 0 Pixels    |
| // H Front Porch  | = 0.279;         | // (usec) | = 8 chars         | = 64 Pixels   |
| Hor Sync Time     | = 0.837;         | // (usec) | = 24 chars        | = 192 Pixels  |
38
| // H Back Porch  | = 1.325; | // (usec) | = chars   | = 304 Pixels |
| ---------------- | -------- | --------- | --------- | ------------ |
| // H Left Border | = 0.000; | // (usec) | = 0 chars | = 0 Pixels   |
Ver Total Time = 11.765; // (msec) = 1250 lines HT – (1.06xHA)
| Ver Addr Time      | = 11.294; | // (msec) | = 1200 lines | = 2.02 |
| ------------------ | --------- | --------- | ------------ | ------ |
| Ver Blank Start    | = 11.294; | // (msec) | = 1200 lines |        |
| Ver Blank Time     | = 0.471;  | // (msec) | = 50 lines   |        |
| Ver Sync Start     | = 11.304; | // (msec) | = 1201 lines |        |
| // V Bottom Border | = 0.000;  | // (msec) | = 0 lines    |        |
1
| // V Front Porch | = 0.009; | // (msec) | = lines    |     |
| ---------------- | -------- | --------- | ---------- | --- |
| Ver Sync Time    | = 0.028; | // (msec) | = 3 lines  |     |
| // V Back Porch  | = 0.433; | // (msec) | = 46 lines |     |
| // V Top Border  | = 0.000; | // (msec) | = 0 lines  |     |

Definition of Terms: Refer to section 3.1.

VESA MONITOR TIMING STANDARD
Adopted:  5/1/07
Resolution:  1600 x 1200 at 120 Hz (non-interlaced) REDUCED BLANKING
EDID ID:  DMT ID: 38h; Std. 2 Byte Code: n/a; CVT 3 Byte Code: n/a
Method:  Generated using CVT (Reduced Blanking) Formula

Detailed Timing Parameters
| Timing Name | = 1600 x 1200 @ 120Hz CVT (Reduced Blanking); |           |     |     |     |
| ----------- | --------------------------------------------- | --------- | --- | --- | --- |
| Hor Pixels  | = 1600;                                       | // Pixels |     |     |     |
1200;
| Ver Pixels    | =          | // Lines |       |        |        |
| ------------- | ---------- | -------- | ----- | ------ | ------ |
| Hor Frequency | = 152.415; | // kHz   | = 6.6 | usec / | line   |
| Ver Frequency | = 119.917; | // Hz    | = 8.3 | msec / | frame  |
| Pixel Clock   | = 268.250; | // MHz   | = 3.7 | nsec   | ± 0.5% |
8;
| Character Width   | =                | // Pixels | = 29.8     | nsec       |             |
| ----------------- | ---------------- | --------- | ---------- | ---------- | ----------- |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =          | 0.9 %       |
| Hor Sync Polarity | = POSITIVE;      | // HBlank | = 9.1%     | of HTotal  |             |
| Ver Sync Polarity | = NEGATIVE       | // VBlank | = 5.6%     | of VTotal  |             |
| Hor Total Time    | = 6.561;         | // (usec) | = 220      | chars =    | 1760 Pixels |
| Hor Addr Time     | = 5.965;         | // (usec) | = 200      | chars =    | 1600 Pixels |
| Hor Blank Start   | = 5.965;         | // (usec) | = 200      | chars =    | 1600 Pixels |
| Hor Blank Time    | = 0.596;         | // (usec) | =          | 20 chars = | 160 Pixels  |
| Hor Sync Start    | = 6.144;         | // (usec) | = 206      | chars =    | 1648 Pixels |
| // H Right Border | = 0.000;         | // (usec) | =          | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.179;         | // (usec) | =          | 6 chars =  | 48 Pixels   |
| Hor Sync Time     | = 0.119;         | // (usec) | =          | 4 chars =  | 32 Pixels   |
10
| // H Back Porch  | = 0.298; | // (usec) | =   | chars =   | 80 Pixels |
| ---------------- | -------- | --------- | --- | --------- | --------- |
| // H Left Border | = 0.000; | // (usec) | =   | 0 chars = | 0 Pixels  |
Ver Total Time = 8.339; // (msec) = 1271 lines HT – (1.06xHA)
| Ver Addr Time      | = 7.873; | // (msec) | = 1200 | lines    | = 0.24 |
| ------------------ | -------- | --------- | ------ | -------- | ------ |
| Ver Blank Start    | = 7.873; | // (msec) | = 1200 | lines    |        |
| Ver Blank Time     | = 0.466; | // (msec) | =      | 71 lines |        |
| Ver Sync Start     | = 7.893; | // (msec) | = 1203 | lines    |        |
| // V Bottom Border | = 0.000; | // (msec) | =      | 0 lines  |        |
| // V Front Porch   | = 0.020; | // (msec) | =      | 3 lines  |        |
| Ver Sync Time      | = 0.026; | // (msec) | =      | 4 lines  |        |
| // V Back Porch    | = 0.420; | // (msec) | = 64   | lines    |        |
| // V Top Border    | = 0.000; | // (msec) | =      | 0 lines  |        |

Definition of Terms: Refer to section 3.2.

VESA MONITOR TIMING STANDARD
Adopted:  10/24/04
Resolution:  1680 x 1050 at 60 Hz (non-interlaced) REDUCED BLANKING
EDID ID:  DMT ID: 39h; Std. 2 Byte Code: n/a; CVT 3 Byte Code: (0C, 28, 21)h
Method:  CVT Reduced Blanking

Detailed Timing Parameters
| Timing Name | = 1680 x 1050 @ 60Hz CVT (Reduced Blanking); |     |     |     |     |
| ----------- | -------------------------------------------- | --- | --- | --- | --- |
1680;
| Hor Pixels        | =                | // Pixels |            |           |        |
| ----------------- | ---------------- | --------- | ---------- | --------- | ------ |
| Ver Pixels        | = 1050;          | // Lines  |            |           |        |
| Hor Frequency     | = 64.674;        | // kHz    | = 15.5     | usec /    | line   |
| Ver Frequency     | = 59.883;        | // Hz     | = 16.7     | msec /    | frame  |
| Pixel Clock       | = 119.000;       | // MHz    | = 8.4      | nsec      | ± 0.5% |
| Character Width   | = 8;             | // Pixels | = 67.2     | nsec      |        |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =         | 0.9 %  |
| Hor Sync Polarity | = POSITIVE;      | // HBlank | = 8.7%     | of HTotal |        |
NEGATIVE
| Ver Sync Polarity | =         | // VBlank | = 2.8% | of VTotal |             |
| ----------------- | --------- | --------- | ------ | --------- | ----------- |
| Hor Total Time    | = 15.462; | // (usec) | = 230  | chars =   | 1840 Pixels |
| Hor Addr Time     | = 14.118; | // (usec) | = 210  | chars =   | 1680 Pixels |
Hor Blank Start = 14.118; // (usec) = 210 chars = 1680 Pixels
| Hor Blank Time    | = 1.345;  | // (usec) | =     | 20 chars = | 160 Pixels  |
| ----------------- | --------- | --------- | ----- | ---------- | ----------- |
| Hor Sync Start    | = 14.521; | // (usec) | = 216 | chars =    | 1728 Pixels |
| // H Right Border | = 0.000;  | // (usec) | =     | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.403;  | // (usec) | =     | 6 chars =  | 48 Pixels   |
| Hor Sync Time     | = 0.269;  | // (usec) | =     | 4 chars =  | 32 Pixels   |
| // H Back Porch   | = 0.672;  | // (usec) | =     | 10 chars = | 80 Pixels   |
| // H Left Border  | = 0.000;  | // (usec) | =     | 0 chars =  | 0 Pixels    |
Ver Total Time = 16.699; // (msec) = 1080 lines HT – (1.06xHA)
| Ver Addr Time      | = 16.235; | // (msec) | = 1050 | lines    | = 0.5 |
| ------------------ | --------- | --------- | ------ | -------- | ----- |
| Ver Blank Start    | = 16.235; | // (msec) | = 1050 | lines    |       |
| Ver Blank Time     | = 0.464;  | // (msec) | =      | 30 lines |       |
| Ver Sync Start     | = 16.282; | // (msec) | = 1053 | lines    |       |
| // V Bottom Border | = 0.000;  | // (msec) | =      | 0 lines  |       |
| // V Front Porch   | = 0.046;  | // (msec) | =      | 3 lines  |       |
| Ver Sync Time      | = 0.093;  | // (msec) | =      | 6 lines  |       |
21
| // V Back Porch | = 0.325; | // (msec) | =   | lines   |     |
| --------------- | -------- | --------- | --- | ------- | --- |
| // V Top Border | = 0.000; | // (msec) | =   | 0 lines |     |

Definition of Terms: Refer to section 3.2.

VESA MONITOR TIMING STANDARD
Adopted:  10/24/04
Resolution:  1680 x 1050 at 60 Hz (non-interlaced)
EDID ID:  DMT ID: 3Ah; Std. 2 Byte Code: (B3, 00)h; CVT 3 Byte Code: (0C, 28, 28)h
Method:  CVT Compliant

Detailed Timing Parameters
| Timing Name   | = 1680 x 1050 @ 60Hz; |           |        |        |       |
| ------------- | --------------------- | --------- | ------ | ------ | ----- |
| Hor Pixels    | = 1680;               | // Pixels |        |        |       |
| Ver Pixels    | = 1050;               | // Lines  |        |        |       |
| Hor Frequency | = 65.290;             | // kHz    | = 15.3 | usec / | line  |
| Ver Frequency | = 59.954;             | // Hz     | = 16.7 | msec / | frame |
146.250;
| Pixel Clock       | =                | // MHz    | = 6.8      | nsec      | ± 0.5%      |
| ----------------- | ---------------- | --------- | ---------- | --------- | ----------- |
| Character Width   | = 8;             | // Pixels | = 54.7     | nsec      |             |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =         | 3.9 %       |
| Hor Sync Polarity | = NEGATIVE       | // HBlank | = 25.0%    | of HTotal |             |
| Ver Sync Polarity | = POSITIVE;      | // VBlank | = 3.6%     | of VTotal |             |
| Hor Total Time    | = 15.316;        | // (usec) | = 280      | chars =   | 2240 Pixels |
| Hor Addr Time     | = 11.487;        | // (usec) | = 210      | chars =   | 1680 Pixels |
Hor Blank Start = 11.487; // (usec) = 210 chars = 1680 Pixels
| Hor Blank Time    | = 3.829;  | // (usec) | =     | 70 chars = | 560 Pixels  |
| ----------------- | --------- | --------- | ----- | ---------- | ----------- |
| Hor Sync Start    | = 12.198; | // (usec) | = 223 | chars =    | 1784 Pixels |
| // H Right Border | = 0.000;  | // (usec) | =     | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.711;  | // (usec) | =     | 13 chars = | 104 Pixels  |
| Hor Sync Time     | = 1.203;  | // (usec) | =     | 22 chars = | 176 Pixels  |
35
| // H Back Porch  | = 1.915; | // (usec) | =   | chars =   | 280 Pixels |
| ---------------- | -------- | --------- | --- | --------- | ---------- |
| // H Left Border | = 0.000; | // (usec) | =   | 0 chars = | 0 Pixels   |
Ver Total Time = 16.679; // (msec) = 1089 lines HT – (1.06xHA)
| Ver Addr Time      | = 16.082; | // (msec) | = 1050 | lines    | = 3.14 |
| ------------------ | --------- | --------- | ------ | -------- | ------ |
| Ver Blank Start    | = 16.082; | // (msec) | = 1050 | lines    |        |
| Ver Blank Time     | = 0.597;  | // (msec) | =      | 39 lines |        |
| Ver Sync Start     | = 16.128; | // (msec) | = 1053 | lines    |        |
| // V Bottom Border | = 0.000;  | // (msec) | =      | 0 lines  |        |
3
| // V Front Porch | = 0.046; | // (msec) | =   | lines    |     |
| ---------------- | -------- | --------- | --- | -------- | --- |
| Ver Sync Time    | = 0.092; | // (msec) | =   | 6 lines  |     |
| // V Back Porch  | = 0.459; | // (msec) | =   | 30 lines |     |
| // V Top Border  | = 0.000; | // (msec) | =   | 0 lines  |     |

Definition of Terms: Refer to section 3.4.

VESA MONITOR TIMING STANDARD
Adopted:  10/24/04
Resolution:  1680 x 1050 at 75 Hz (non-interlaced)
EDID ID:  DMT ID: 3Bh; Std. 2 Byte Code: (B3, 0F)h; CVT 3 Byte Code: (0C, 28, 44)h
Method:  CVT Compliant

Detailed Timing Parameters
| Timing Name   | = 1680 x 1050 @ 75Hz; |           |        |        |       |
| ------------- | --------------------- | --------- | ------ | ------ | ----- |
| Hor Pixels    | = 1680;               | // Pixels |        |        |       |
| Ver Pixels    | = 1050;               | // Lines  |        |        |       |
| Hor Frequency | = 82.306;             | // kHz    | = 12.1 | usec / | line  |
| Ver Frequency | = 74.892;             | // Hz     | = 13.4 | msec / | frame |
187.000;
| Pixel Clock       | =                | // MHz    | = 5.3      | nsec       | ± 0.5%      |
| ----------------- | ---------------- | --------- | ---------- | ---------- | ----------- |
| Character Width   | = 8;             | // Pixels | = 42.8     | nsec       |             |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =          | 3.9 %       |
| Hor Sync Polarity | = NEGATIVE       | // HBlank | = 26.1%    | of HTotal  |             |
| Ver Sync Polarity | = POSITIVE;      | // VBlank | = 4.5%     | of VTotal  |             |
| Hor Total Time    | = 12.150;        | // (usec) | = 284      | chars =    | 2272 Pixels |
| Hor Addr Time     | = 8.984;         | // (usec) | = 210      | chars =    | 1680 Pixels |
| Hor Blank Start   | = 8.984;         | // (usec) | = 210      | chars =    | 1680 Pixels |
| Hor Blank Time    | = 3.166;         | // (usec) | =          | 74 chars = | 592 Pixels  |
| Hor Sync Start    | = 9.626;         | // (usec) | = 225      | chars =    | 1800 Pixels |
| // H Right Border | = 0.000;         | // (usec) | =          | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.642;         | // (usec) | =          | 15 chars = | 120 Pixels  |
| Hor Sync Time     | = 0.941;         | // (usec) | =          | 22 chars = | 176 Pixels  |
37
| // H Back Porch  | = 1.583; | // (usec) | =   | chars =   | 296 Pixels |
| ---------------- | -------- | --------- | --- | --------- | ---------- |
| // H Left Border | = 0.000; | // (usec) | =   | 0 chars = | 0 Pixels   |
Ver Total Time = 13.353; // (msec) = 1099 lines HT – (1.06xHA)
| Ver Addr Time      | = 12.757; | // (msec) | = 1050 | lines    | = 2.63 |
| ------------------ | --------- | --------- | ------ | -------- | ------ |
| Ver Blank Start    | = 12.757; | // (msec) | = 1050 | lines    |        |
| Ver Blank Time     | = 0.595;  | // (msec) | =      | 49 lines |        |
| Ver Sync Start     | = 12.794; | // (msec) | = 1053 | lines    |        |
| // V Bottom Border | = 0.000;  | // (msec) | =      | 0 lines  |        |
3
| // V Front Porch | = 0.036; | // (msec) | =   | lines    |     |
| ---------------- | -------- | --------- | --- | -------- | --- |
| Ver Sync Time    | = 0.073; | // (msec) | =   | 6 lines  |     |
| // V Back Porch  | = 0.486; | // (msec) | =   | 40 lines |     |
| // V Top Border  | = 0.000; | // (msec) | =   | 0 lines  |     |

Definition of Terms: Refer to section 3.4.

VESA MONITOR TIMING STANDARD
Adopted:  10/24/04
Resolution:  1680 x 1050 at 85 Hz (non-interlaced)
EDID ID:  DMT ID: 3Ch; Std. 2 Byte Code: (B3, 19)h; CVT 3 Byte Code: (0C, 28, 68)h
Method:  CVT Compliant

Detailed Timing Parameters
| Timing Name   | = 1680 x 1050 @ 85Hz; |           |        |        |       |
| ------------- | --------------------- | --------- | ------ | ------ | ----- |
| Hor Pixels    | = 1680;               | // Pixels |        |        |       |
| Ver Pixels    | = 1050;               | // Lines  |        |        |       |
| Hor Frequency | = 93.859;             | // kHz    | = 10.7 | usec / | line  |
| Ver Frequency | = 84.941;             | // Hz     | = 11.8 | msec / | frame |
214.750;
| Pixel Clock       | =                | // MHz    | = 4.7      | nsec       | ± 0.5%      |
| ----------------- | ---------------- | --------- | ---------- | ---------- | ----------- |
| Character Width   | = 8;             | // Pixels | = 37.3     | nsec       |             |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =          | 3.8 %       |
| Hor Sync Polarity | = NEGATIVE       | // HBlank | = 26.6%    | of HTotal  |             |
| Ver Sync Polarity | = POSITIVE;      | // VBlank | = 5.0%     | of VTotal  |             |
| Hor Total Time    | = 10.654;        | // (usec) | = 286      | chars =    | 2288 Pixels |
| Hor Addr Time     | = 7.823;         | // (usec) | = 210      | chars =    | 1680 Pixels |
| Hor Blank Start   | = 7.823;         | // (usec) | = 210      | chars =    | 1680 Pixels |
| Hor Blank Time    | = 2.831;         | // (usec) | =          | 76 chars = | 608 Pixels  |
| Hor Sync Start    | = 8.419;         | // (usec) | = 226      | chars =    | 1808 Pixels |
| // H Right Border | = 0.000;         | // (usec) | =          | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.596;         | // (usec) | =          | 16 chars = | 128 Pixels  |
| Hor Sync Time     | = 0.820;         | // (usec) | =          | 22 chars = | 176 Pixels  |
38
| // H Back Porch  | = 1.416; | // (usec) | =   | chars =   | 304 Pixels |
| ---------------- | -------- | --------- | --- | --------- | ---------- |
| // H Left Border | = 0.000; | // (usec) | =   | 0 chars = | 0 Pixels   |
Ver Total Time = 11.773; // (msec) = 1105 lines HT – (1.06xHA)
| Ver Addr Time      | = 11.187; | // (msec) | = 1050 | lines    | = 2.36 |
| ------------------ | --------- | --------- | ------ | -------- | ------ |
| Ver Blank Start    | = 11.187; | // (msec) | = 1050 | lines    |        |
| Ver Blank Time     | = 0.586;  | // (msec) | =      | 55 lines |        |
| Ver Sync Start     | = 11.219; | // (msec) | = 1053 | lines    |        |
| // V Bottom Border | = 0.000;  | // (msec) | =      | 0 lines  |        |
3
| // V Front Porch | = 0.032; | // (msec) | =   | lines    |     |
| ---------------- | -------- | --------- | --- | -------- | --- |
| Ver Sync Time    | = 0.064; | // (msec) | =   | 6 lines  |     |
| // V Back Porch  | = 0.490; | // (msec) | =   | 46 lines |     |
| // V Top Border  | = 0.000; | // (msec) | =   | 0 lines  |     |

Definition of Terms: Refer to section 3.4.

VESA MONITOR TIMING STANDARD
Adopted:  5/1/07
Resolution:  1680 x 1050 at 120 Hz (non-interlaced) REDUCED BLANKING
EDID ID:  DMT ID: 3Dh; Std. 2 Byte Code: n/a; CVT 3 Byte Code: n/a
Method:  Generated using CVT (Reduced Blanking) Formula

Detailed Timing Parameters
| Timing Name | = 1680 x 1050 @ 120Hz CVT (Reduced Blanking); |           |     |     |     |
| ----------- | --------------------------------------------- | --------- | --- | --- | --- |
| Hor Pixels  | = 1680;                                       | // Pixels |     |     |     |
1050;
| Ver Pixels    | =          | // Lines |       |        |        |
| ------------- | ---------- | -------- | ----- | ------ | ------ |
| Hor Frequency | = 133.424; | // kHz   | = 7.5 | usec / | line   |
| Ver Frequency | = 119.986; | // Hz    | = 8.3 | msec / | frame  |
| Pixel Clock   | = 245.500; | // MHz   | = 4.1 | nsec   | ± 0.5% |
8;
| Character Width   | =                | // Pixels | = 32.6     | nsec       |             |
| ----------------- | ---------------- | --------- | ---------- | ---------- | ----------- |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =          | 0.9 %       |
| Hor Sync Polarity | = POSITIVE;      | // HBlank | = 8.7%     | of HTotal  |             |
| Ver Sync Polarity | = NEGATIVE       | // VBlank | = 5.6%     | of VTotal  |             |
| Hor Total Time    | = 7.495;         | // (usec) | = 230      | chars =    | 1840 Pixels |
| Hor Addr Time     | = 6.843;         | // (usec) | = 210      | chars =    | 1680 Pixels |
| Hor Blank Start   | = 6.843;         | // (usec) | = 210      | chars =    | 1680 Pixels |
| Hor Blank Time    | = 0.652;         | // (usec) | =          | 20 chars = | 160 Pixels  |
| Hor Sync Start    | = 7.039;         | // (usec) | = 216      | chars =    | 1728 Pixels |
| // H Right Border | = 0.000;         | // (usec) | =          | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.196;         | // (usec) | =          | 6 chars =  | 48 Pixels   |
| Hor Sync Time     | = 0.130;         | // (usec) | =          | 4 chars =  | 32 Pixels   |
10
| // H Back Porch  | = 0.326; | // (usec) | =   | chars =   | 80 Pixels |
| ---------------- | -------- | --------- | --- | --------- | --------- |
| // H Left Border | = 0.000; | // (usec) | =   | 0 chars = | 0 Pixels  |
Ver Total Time = 8.334; // (msec) = 1112 lines HT – (1.06xHA)
| Ver Addr Time      | = 7.870; | // (msec) | = 1050 | lines    | = 0.24 |
| ------------------ | -------- | --------- | ------ | -------- | ------ |
| Ver Blank Start    | = 7.870; | // (msec) | = 1050 | lines    |        |
| Ver Blank Time     | = 0.465; | // (msec) | =      | 62 lines |        |
| Ver Sync Start     | = 7.892; | // (msec) | = 1053 | lines    |        |
| // V Bottom Border | = 0.000; | // (msec) | =      | 0 lines  |        |
| // V Front Porch   | = 0.022; | // (msec) | =      | 3 lines  |        |
| Ver Sync Time      | = 0.045; | // (msec) | =      | 6 lines  |        |
| // V Back Porch    | = 0.397; | // (msec) | = 53   | lines    |        |
| // V Top Border    | = 0.000; | // (msec) | =      | 0 lines  |        |

Definition of Terms: Refer to section 3.2.

VESA MONITOR TIMING STANDARD
Adopted:  9/17/98
Resolution:  1792 x 1344 at 60 Hz (non-interlaced)
EDID ID:  DMT ID: 3Eh; Std. 2 Byte Code: (C1, 40)h; CVT 3 Byte Code: n/a
Method:  *** NOT CVT COMPLIANT ***

Detailed Timing Parameters
| Timing Name   | = 1792 x 1344 @ 60 Hz |           |        |        |        |     |
| ------------- | --------------------- | --------- | ------ | ------ | ------ | --- |
| Hor Pixels    | = 1792;               | // Pixels |        |        |        |     |
| Ver Pixels    | = 1344;               | // Lines  |        |        |        |     |
| Hor Frequency | = 83.640;             | // kHz    | = 12.0 | usec / | line   |     |
| Ver Frequency | = 60.000;             | // Hz     | = 16.7 | msec / | frame  |     |
| Pixel Clock   | = 204.750;            | // MHz    | = 4.9  | nsec   | ± 0.5% |     |
8;
| Character Width   | =                | // Pixels | = 39.1     | nsec       |             |     |
| ----------------- | ---------------- | --------- | ---------- | ---------- | ----------- | --- |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =          | 4.1 %       |     |
| Hor Sync Polarity | = NEGATIVE;      | // HBlank | = 26.8%    | of HTotal  |             |     |
| Ver Sync Polarity | = POSITIVE;      | // VBlank | = 3.6%     | of VTotal  |             |     |
| Hor Total Time    | = 11.956;        | // (usec) | = 306      | chars =    | 2448 Pixels |     |
| Hor Addr Time     | = 8.752;         | // (usec) | = 224      | chars =    | 1792 Pixels |     |
| Hor Blank Start   | = 8.752;         | // (usec) | = 224      | chars =    | 1792 Pixels |     |
| Hor Blank Time    | = 3.204;         | // (usec) | =          | 82 chars = | 656 Pixels  |     |
| Hor Sync Start    | = 9.377;         | // (usec) | = 240      | chars =    | 1920 Pixels |     |
| // H Right Border | = 0.000;         | // (usec) | =          | 0 chars =  | 0 Pixels    |     |
16
| // H Front Porch | = 0.625; | // (usec) | =   | chars =    | 128 Pixels |     |
| ---------------- | -------- | --------- | --- | ---------- | ---------- | --- |
| Hor Sync Time    | = 0.977; | // (usec) | =   | 25 chars = | 200 Pixels |     |
| // H Back Porch  | = 1.602; | // (usec) | =   | 41 chars = | 328 Pixels |     |
| // H Left Border | = 0.000; | // (usec) | =   | 0 chars =  | 0 Pixels   |     |
Ver Total Time = 16.667; // (msec) = 1394 lines HT – (1.06xHA)
| Ver Addr Time      | = 16.069; | // (msec) | = 1344 | lines    | = 2.68 |     |
| ------------------ | --------- | --------- | ------ | -------- | ------ | --- |
| Ver Blank Start    | = 16.069; | // (msec) | = 1344 | lines    |        |     |
| Ver Blank Time     | = 0.598;  | // (msec) | =      | 50 lines |        |     |
| Ver Sync Start     | = 16.081; | // (msec) | = 1345 | lines    |        |     |
| // V Bottom Border | = 0.000;  | // (msec) | =      | 0 lines  |        |     |
1
| // V Front Porch | = 0.012; | // (msec) | =   | lines    |     |     |
| ---------------- | -------- | --------- | --- | -------- | --- | --- |
| Ver Sync Time    | = 0.036; | // (msec) | =   | 3 lines  |     |     |
| // V Back Porch  | = 0.550; | // (msec) | =   | 46 lines |     |     |
| // V Top Border  | = 0.000; | // (msec) | =   | 0 lines  |     |     |

Definition of Terms: Refer to section 3.4.

VESA MONITOR TIMING STANDARD
Adopted:  9/17/98
Resolution:  1792 x 1344 at 75 Hz (non-interlaced)
EDID ID:  DMT ID: 3Fh; Std. 2 Byte Code: (C1, 4F)h; CVT 3 Byte Code: n/a
Method:  *** NOT CVT COMPLIANT ***

Detailed Timing Parameters
| Timing Name | = 1792 x 1344 @ 75Hz; |           |     |     |     |
| ----------- | --------------------- | --------- | --- | --- | --- |
| Hor Pixels  | = 1792;               | // Pixels |     |     |     |
1344;
| Ver Pixels        | =                | // Lines  |            |           |        |
| ----------------- | ---------------- | --------- | ---------- | --------- | ------ |
| Hor Frequency     | = 106.270;       | // kHz    | = 9.4      | usec /    | line   |
| Ver Frequency     | = 74.997;        | // Hz     | = 13.3     | msec /    | frame  |
| Pixel Clock       | = 261.000;       | // MHz    | = 3.8      | nsec      | ± 0.5% |
| Character Width   | = 8;             | // Pixels | = 30.7     | nsec      |        |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =         | 5.2 %  |
| Hor Sync Polarity | = NEGATIVE;      | // HBlank | = 27.0%    | of HTotal |        |
POSITIVE;
| Ver Sync Polarity | =        | // VBlank | = 5.2% | of VTotal  |             |
| ----------------- | -------- | --------- | ------ | ---------- | ----------- |
| Hor Total Time    | = 9.410; | // (usec) | = 307  | chars =    | 2456 Pixels |
| Hor Addr Time     | = 6.866; | // (usec) | = 224  | chars =    | 1792 Pixels |
| Hor Blank Start   | = 6.866; | // (usec) | = 224  | chars =    | 1792 Pixels |
| Hor Blank Time    | = 2.544; | // (usec) | =      | 83 chars = | 664 Pixels  |
| Hor Sync Start    | = 7.234; | // (usec) | = 236  | chars =    | 1888 Pixels |
| // H Right Border | = 0.000; | // (usec) | =      | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.368; | // (usec) | =      | 12 chars = | 96 Pixels   |
| Hor Sync Time     | = 0.828; | // (usec) | =      | 27 chars = | 216 Pixels  |
| // H Back Porch   | = 1.349; | // (usec) | =      | 44 chars = | 352 Pixels  |
| // H Left Border  | = 0.000; | // (usec) | =      | 0 chars =  | 0 Pixels    |
Ver Total Time = 13.334; // (msec) = 1417 lines HT – (1.06xHA)
| Ver Addr Time      | = 12.647; | // (msec) | = 1344 | lines    | = 2.13 |
| ------------------ | --------- | --------- | ------ | -------- | ------ |
| Ver Blank Start    | = 12.647; | // (msec) | = 1344 | lines    |        |
| Ver Blank Time     | = 0.687;  | // (msec) | =      | 73 lines |        |
| Ver Sync Start     | = 12.656; | // (msec) | = 1345 | lines    |        |
| // V Bottom Border | = 0.000;  | // (msec) | =      | 0 lines  |        |
| // V Front Porch   | = 0.009;  | // (msec) | =      | 1 lines  |        |
| Ver Sync Time      | = 0.028;  | // (msec) | =      | 3 lines  |        |
| // V Back Porch    | = 0.649;  | // (msec) | =      | 69 lines |        |
| // V Top Border    | = 0.000;  | // (msec) | =      | 0 lines  |        |

Definition of Terms: Refer to section 3.4.

VESA MONITOR TIMING STANDARD
Adopted:  5/1/07
Resolution:  1792 x 1344 at 120 Hz (non-interlaced) REDUCED BLANKING
EDID ID:  DMT ID: 40h; Std. 2 Byte Code: n/a; CVT 3 Byte Code: n/a
Method:  Generated using CVT (Reduced Blanking) Formula

Detailed Timing Parameters
| Timing Name       | = 1792 x 1344 @ 120Hz CVT (Reduced Blanking); |           |            |            |             |
| ----------------- | --------------------------------------------- | --------- | ---------- | ---------- | ----------- |
| Hor Pixels        | = 1792;                                       | // Pixels |            |            |             |
| Ver Pixels        | = 1344;                                       | // Lines  |            |            |             |
| Hor Frequency     | = 170.722;                                    | // kHz    | = 5.9      | usec /     | line        |
| Ver Frequency     | = 119.974;                                    | // Hz     | = 8.3      | msec /     | frame       |
| Pixel Clock       | = 333.250;                                    | // MHz    | = 3.0      | nsec       | ± 0.5%      |
| Character Width   | = 8;                                          | // Pixels | = 24.0     | nsec       |             |
| Scan Type         | = NONINTERLACED;                              |           | // H Phase | =          | 0.8 %       |
| Hor Sync Polarity | = POSITIVE;                                   | // HBlank | = 8.2%     | of HTotal  |             |
| Ver Sync Polarity | = NEGATIVE                                    | // VBlank | = 5.6%     | of VTotal  |             |
| Hor Total Time    | = 5.857;                                      | // (usec) | = 244      | chars =    | 1952 Pixels |
| Hor Addr Time     | = 5.377;                                      | // (usec) | = 224      | chars =    | 1792 Pixels |
| Hor Blank Start   | = 5.377;                                      | // (usec) | = 224      | chars =    | 1792 Pixels |
| Hor Blank Time    | = 0.480;                                      | // (usec) | =          | 20 chars = | 160 Pixels  |
| Hor Sync Start    | = 5.521;                                      | // (usec) | = 230      | chars =    | 1840 Pixels |
| // H Right Border | = 0.000;                                      | // (usec) | =          | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.144;                                      | // (usec) | =          | 6 chars =  | 48 Pixels   |
| Hor Sync Time     | = 0.096;                                      | // (usec) | =          | 4 chars =  | 32 Pixels   |
| // H Back Porch   | = 0.240;                                      | // (usec) | = 10       | chars =    | 80 Pixels   |
| // H Left Border  | = 0.000;                                      | // (usec) | =          | 0 chars =  | 0 Pixels    |
Ver Total Time = 8.335; // (msec) = 1423 lines HT – (1.06xHA)
| Ver Addr Time      | = 7.872; | // (msec) | = 1344 | lines    | = 0.16 |
| ------------------ | -------- | --------- | ------ | -------- | ------ |
| Ver Blank Start    | = 7.872; | // (msec) | = 1344 | lines    |        |
| Ver Blank Time     | = 0.463; | // (msec) | =      | 79 lines |        |
| Ver Sync Start     | = 7.890; | // (msec) | = 1347 | lines    |        |
| // V Bottom Border | = 0.000; | // (msec) | =      | 0 lines  |        |
| // V Front Porch   | = 0.018; | // (msec) | =      | 3 lines  |        |
| Ver Sync Time      | = 0.023; | // (msec) | =      | 4 lines  |        |
| // V Back Porch    | = 0.422; | // (msec) | = 72   | lines    |        |
| // V Top Border    | = 0.000; | // (msec) | =      | 0 lines  |        |

Definition of Terms: Refer to section 3.2.

VESA MONITOR TIMING STANDARD
Adopted:  9/17/98
Resolution:  1856 x 1392 at 60 Hz (non-interlaced)
EDID ID:  DMT ID: 41h; Std. 2 Byte Code: (C9, 40)h; CVT 3 Byte Code: n/a
Method:  *** NOT CVT COMPLIANT ***

Detailed Timing Parameters
| Timing Name   | = 1856 x 1392 at 60Hz; |           |        |        |        |     |
| ------------- | ---------------------- | --------- | ------ | ------ | ------ | --- |
| Hor Pixels    | = 1856;                | // Pixels |        |        |        |     |
| Ver Pixels    | = 1392;                | // Lines  |        |        |        |     |
| Hor Frequency | = 86.333;              | // kHz    | = 11.6 | usec / | line   |     |
| Ver Frequency | = 59.995;              | // Hz     | = 16.7 | msec / | frame  |     |
| Pixel Clock   | = 218.250;             | // MHz    | = 4.6  | nsec   | ± 0.5% |     |
8;
| Character Width   | =                | // Pixels | = 36.7     | nsec       |             |     |
| ----------------- | ---------------- | --------- | ---------- | ---------- | ----------- | --- |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =          | 5.1 %       |     |
| Hor Sync Polarity | = NEGATIVE;      | // HBlank | = 26.6%    | of HTotal  |             |     |
| Ver Sync Polarity | = POSITIVE;      | // VBlank | = 3.3%     | of VTotal  |             |     |
| Hor Total Time    | = 11.583;        | // (usec) | = 316      | chars =    | 2528 Pixels |     |
| Hor Addr Time     | = 8.504;         | // (usec) | = 232      | chars =    | 1856 Pixels |     |
| Hor Blank Start   | = 8.504;         | // (usec) | = 232      | chars =    | 1856 Pixels |     |
| Hor Blank Time    | = 3.079;         | // (usec) | =          | 84 chars = | 672 Pixels  |     |
| Hor Sync Start    | = 8.944;         | // (usec) | = 244      | chars =    | 1952 Pixels |     |
| // H Right Border | = 0.000;         | // (usec) | =          | 0 chars =  | 0 Pixels    |     |
12
| // H Front Porch | = 0.440; | // (usec) | =   | chars =    | 96 Pixels  |     |
| ---------------- | -------- | --------- | --- | ---------- | ---------- | --- |
| Hor Sync Time    | = 1.026; | // (usec) | =   | 28 chars = | 224 Pixels |     |
| // H Back Porch  | = 1.613; | // (usec) | =   | 44 chars = | 352 Pixels |     |
| // H Left Border | = 0.000; | // (usec) | =   | 0 chars =  | 0 Pixels   |     |
Ver Total Time = 16.668; // (msec) = 1439 lines HT – (1.06xHA)
| Ver Addr Time      | = 16.124; | // (msec) | = 1392 | lines    | = 2.57 |     |
| ------------------ | --------- | --------- | ------ | -------- | ------ | --- |
| Ver Blank Start    | = 16.124; | // (msec) | = 1392 | lines    |        |     |
| Ver Blank Time     | = 0.544;  | // (msec) | =      | 47 lines |        |     |
| Ver Sync Start     | = 16.135; | // (msec) | = 1393 | lines    |        |     |
| // V Bottom Border | = 0.000;  | // (msec) | =      | 0 lines  |        |     |
1
| // V Front Porch | = 0.012; | // (msec) | =   | lines    |     |     |
| ---------------- | -------- | --------- | --- | -------- | --- | --- |
| Ver Sync Time    | = 0.035; | // (msec) | =   | 3 lines  |     |     |
| // V Back Porch  | = 0.498; | // (msec) | =   | 43 lines |     |     |
| // V Top Border  | = 0.000; | // (msec) | =   | 0 lines  |     |     |

Definition of Terms: Refer to section 3.4.

VESA MONITOR TIMING STANDARD
Adopted:  9/17/98
Resolution:  1856 x 1392 at 75 Hz (non-interlaced)
EDID ID:  DMT ID: 42h; Std. 2 Byte Code: (C9, 4F)h; CVT 3 Byte Code: n/a
Method:  *** NOT CVT COMPLIANT ***

Detailed Timing Parameters
| Timing Name | = 1856 x 1392 @ 75Hz; |           |     |     |     |
| ----------- | --------------------- | --------- | --- | --- | --- |
| Hor Pixels  | = 1856;               | // Pixels |     |     |     |
1392;
| Ver Pixels        | =                | // Lines  |            |           |        |
| ----------------- | ---------------- | --------- | ---------- | --------- | ------ |
| Hor Frequency     | = 112.500;       | // kHz    | = 8.9      | usec /    | line   |
| Ver Frequency     | = 75.000;        | // Hz     | = 13.3     | msec /    | frame  |
| Pixel Clock       | = 288.000;       | // MHz    | = 3.5      | nsec      | ± 0.5% |
| Character Width   | = 8;             | // Pixels | = 27.8     | nsec      |        |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =         | 4.4 %  |
| Hor Sync Polarity | = NEGATIVE;      | // HBlank | = 27.5%    | of HTotal |        |
POSITIVE;
| Ver Sync Polarity | =        | // VBlank | = 7.2% | of VTotal  |             |
| ----------------- | -------- | --------- | ------ | ---------- | ----------- |
| Hor Total Time    | = 8.889; | // (usec) | = 320  | chars =    | 2560 Pixels |
| Hor Addr Time     | = 6.444; | // (usec) | = 232  | chars =    | 1856 Pixels |
| Hor Blank Start   | = 6.444; | // (usec) | = 232  | chars =    | 1856 Pixels |
| Hor Blank Time    | = 2.444; | // (usec) | =      | 88 chars = | 704 Pixels  |
| Hor Sync Start    | = 6.889; | // (usec) | = 248  | chars =    | 1984 Pixels |
| // H Right Border | = 0.000; | // (usec) | =      | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.444; | // (usec) | =      | 16 chars = | 128 Pixels  |
| Hor Sync Time     | = 0.778; | // (usec) | =      | 28 chars = | 224 Pixels  |
| // H Back Porch   | = 1.222; | // (usec) | =      | 44 chars = | 352 Pixels  |
| // H Left Border  | = 0.000; | // (usec) | =      | 0 chars =  | 0 Pixels    |
Ver Total Time = 13.333; // (msec) = 1500 lines HT – (1.06xHA)
| Ver Addr Time      | = 12.373; | // (msec) | = 1392 | lines   | = 2.06 |
| ------------------ | --------- | --------- | ------ | ------- | ------ |
| Ver Blank Start    | = 12.373; | // (msec) | = 1392 | lines   |        |
| Ver Blank Time     | = 0.960;  | // (msec) | = 108  | lines   |        |
| Ver Sync Start     | = 12.382; | // (msec) | = 1393 | lines   |        |
| // V Bottom Border | = 0.000;  | // (msec) | =      | 0 lines |        |
| // V Front Porch   | = 0.009;  | // (msec) | =      | 1 lines |        |
| Ver Sync Time      | = 0.027;  | // (msec) | =      | 3 lines |        |
| // V Back Porch    | = 0.924;  | // (msec) | = 104  | lines   |        |
| // V Top Border    | = 0.000;  | // (msec) | =      | 0 lines |        |

Definition of Terms: Refer to section 3.4.

VESA MONITOR TIMING STANDARD
Adopted:  5/1/07
Resolution:  1856 x 1392 at 120 Hz (non-interlaced) REDUCED BLANKING
EDID ID:  DMT ID: 43h; Std. 2 Byte Code: n/a; CVT 3 Byte Code: n/a
Method:  Generated using CVT (Reduced Blanking) Formula

Detailed Timing Parameters
| Timing Name | = 1856 x 1392 @ 120Hz CVT (Reduced Blanking); |           |     |     |     |
| ----------- | --------------------------------------------- | --------- | --- | --- | --- |
| Hor Pixels  | = 1856;                                       | // Pixels |     |     |     |
1392;
| Ver Pixels    | =          | // Lines |       |        |        |
| ------------- | ---------- | -------- | ----- | ------ | ------ |
| Hor Frequency | = 176.835; | // kHz   | = 5.7 | usec / | line   |
| Ver Frequency | = 119.970; | // Hz    | = 8.3 | msec / | frame  |
| Pixel Clock   | = 356.500; | // MHz   | = 2.8 | nsec   | ± 0.5% |
8;
| Character Width   | =                | // Pixels | = 22.4     | nsec       |             |
| ----------------- | ---------------- | --------- | ---------- | ---------- | ----------- |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =          | 0.8 %       |
| Hor Sync Polarity | = POSITIVE;      | // HBlank | = 7.9%     | of HTotal  |             |
| Ver Sync Polarity | = NEGATIVE       | // VBlank | = 5.6%     | of VTotal  |             |
| Hor Total Time    | = 5.655;         | // (usec) | = 252      | chars =    | 2016 Pixels |
| Hor Addr Time     | = 5.206;         | // (usec) | = 232      | chars =    | 1856 Pixels |
| Hor Blank Start   | = 5.206;         | // (usec) | = 232      | chars =    | 1856 Pixels |
| Hor Blank Time    | = 0.449;         | // (usec) | =          | 20 chars = | 160 Pixels  |
| Hor Sync Start    | = 5.341;         | // (usec) | = 238      | chars =    | 1904 Pixels |
| // H Right Border | = 0.000;         | // (usec) | =          | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.135;         | // (usec) | =          | 6 chars =  | 48 Pixels   |
| Hor Sync Time     | = 0.090;         | // (usec) | =          | 4 chars =  | 32 Pixels   |
10
| // H Back Porch  | = 0.224; | // (usec) | =   | chars =   | 80 Pixels |
| ---------------- | -------- | --------- | --- | --------- | --------- |
| // H Left Border | = 0.000; | // (usec) | =   | 0 chars = | 0 Pixels  |
Ver Total Time = 8.335; // (msec) = 1474 lines HT – (1.06xHA)
| Ver Addr Time      | = 7.872; | // (msec) | = 1392 | lines    | = 0.14 |
| ------------------ | -------- | --------- | ------ | -------- | ------ |
| Ver Blank Start    | = 7.872; | // (msec) | = 1392 | lines    |        |
| Ver Blank Time     | = 0.464; | // (msec) | =      | 82 lines |        |
| Ver Sync Start     | = 7.889; | // (msec) | = 1395 | lines    |        |
| // V Bottom Border | = 0.000; | // (msec) | =      | 0 lines  |        |
| // V Front Porch   | = 0.017; | // (msec) | =      | 3 lines  |        |
| Ver Sync Time      | = 0.023; | // (msec) | =      | 4 lines  |        |
| // V Back Porch    | = 0.424; | // (msec) | = 75   | lines    |        |
| // V Top Border    | = 0.000; | // (msec) | =      | 0 lines  |        |

Definition of Terms: Refer to section 3.2.

VESA MONITOR TIMING STANDARD
| Adopted:     | 11/17/08                                |     |     |     |     |     |     |
| ------------ | --------------------------------------- | --- | --- | --- | --- | --- | --- |
| Resolution:  | 1920 x 1080 at 60 Hz (non-interlaced)   |     |     |     |     |     |     |
EDID ID:  DMT ID: 52h; Std. 2 Byte Code: (D1, C0)h; CVT 3 Byte Code: n/a
| Method:  | *** NOT CVT COMPLIANT ***                          |     |     |     |     |     |     |
| -------- | -------------------------------------------------- | --- | --- | --- | --- | --- | --- |
|          | Per CEA-861 --- 1080p (Code 16) Timing Definition  |     |     |     |     |     |     |

Detailed Timing Parameters
| Timing Name       |     | = 1920 x 1080 @ 60Hz; |           |            |           |         |        |
| ----------------- | --- | --------------------- | --------- | ---------- | --------- | ------- | ------ |
| Hor Pixels        |     | = 1920;               | // Pixels |            |           |         |        |
| Ver Pixels        |     | = 1080;               | // Lines  |            |           |         |        |
| Hor Frequency     |     | = 67.500;             | // kHz    | = 14.8     | usec      | / line  |        |
| Ver Frequency     |     | = 60.000;             | // Hz     | = 16.7     | msec      | / frame |        |
| Pixel Clock       |     | = 148.500;            | // MHz    | = 6.7      | nsec      | ± 0.5%  |        |
| Character Width   |     | = 4;                  | // Pixels | = 26.9     | nsec      |         |        |
| Scan Type         |     | = NONINTERLACED;      |           | // H Phase |           | =       | 1.4 %  |
| Hor Sync Polarity |     | = POSITIVE            | // HBlank | = 12.7%    | of HTotal |         |        |
| Ver Sync Polarity |     | = POSITIVE            | // VBlank | = 4.0%     | of VTotal |         |        |
| Hor Total Time    |     | = 14.815;             | // (usec) | = 550      | chars     | = 2200  | Pixels |
| Hor Addr Time     |     | = 12.929;             | // (usec) | = 480      | chars     | = 1920  | Pixels |
Hor Blank Start = 12.929; // (usec) = 480 chars = 1920 Pixels
| Hor Blank Time    |     | = 1.886;  | // (usec) | =     | 70 chars | = 280  | Pixels    |
| ----------------- | --- | --------- | --------- | ----- | -------- | ------ | --------- |
| Hor Sync Start    |     | = 13.522; | // (usec) | = 502 | chars    | = 2008 | Pixels    |
| // H Right Border |     | = 0.000;  | // (usec) | =     | 0 chars  | =      | 0 Pixels  |
| // H Front Porch  |     | = 0.593;  | // (usec) | =     | 22 chars | =      | 88 Pixels |
| Hor Sync Time     |     | = 0.296;  | // (usec) | =     | 11 chars | =      | 44 Pixels |
| // H Back Porch   |     | = 0.997;  | // (usec) | =     | 37 chars | = 148  | Pixels    |
| // H Left Border  |     | = 0.000;  | // (usec) | =     | 0 chars  | =      | 0 Pixels  |
Ver Total Time = 16.667; // (msec) = 1125 lines HT – (1.06xHA)
| Ver Addr Time   |     | = 16.000; | // (msec) | = 1080 | lines    |     | = 1.11 |
| --------------- | --- | --------- | --------- | ------ | -------- | --- | ------ |
| Ver Blank Start |     | = 16.000; | // (msec) | = 1080 | lines    |     |        |
| Ver Blank Time  |     | = 0.667;  | // (msec) | =      | 45 lines |     |        |
| Ver Sync Start  |     | = 16.059; | // (msec) | = 1084 | lines    |     |        |
0
| // V Bottom Border |     | = 0.000; | // (msec) | =   | lines    |     |     |
| ------------------ | --- | -------- | --------- | --- | -------- | --- | --- |
| // V Front Porch   |     | = 0.059; | // (msec) | =   | 4 lines  |     |     |
| Ver Sync Time      |     | = 0.074; | // (msec) | =   | 5 lines  |     |     |
| // V Back Porch    |     | = 0.533; | // (msec) | =   | 36 lines |     |     |
| // V Top Border    |     | = 0.000; | // (msec) | =   | 0 lines  |     |     |

Definition of Terms: Refer to section 3.1
VESA MONITOR TIMING STANDARD
Adopted:  8/21/03
Resolution:  1920 x 1200 at 60 Hz (non-interlaced) REDUCED BLANKING
EDID ID:  DMT ID: 44h; Std. 2 Byte Code: n/a; CVT 3 Byte Code: (57, 28, 21)h
Method:  CVT Reduced Blanking

Detailed Timing Parameters
| Timing Name | = 1920 x 1200 @ 60Hz CVT (Reduced Blanking); |           |     |     |     |
| ----------- | -------------------------------------------- | --------- | --- | --- | --- |
| Hor Pixels  | = 1920;                                      | // Pixels |     |     |     |
1200;
| Ver Pixels    | =          | // Lines |        |        |        |
| ------------- | ---------- | -------- | ------ | ------ | ------ |
| Hor Frequency | = 74.038;  | // kHz   | = 13.5 | usec / | line   |
| Ver Frequency | = 59.950;  | // Hz    | = 16.7 | msec / | frame  |
| Pixel Clock   | = 154.000; | // MHz   | = 6.5  | nsec   | ± 0.5% |
8;
| Character Width   | =                | // Pixels | = 51.9     | nsec      |             |
| ----------------- | ---------------- | --------- | ---------- | --------- | ----------- |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =         | 0.8 %       |
| Hor Sync Polarity | = POSITIVE;      | // HBlank | = 7.7%     | of HTotal |             |
| Ver Sync Polarity | = NEGATIVE       | // VBlank | = 2.8%     | of VTotal |             |
| Hor Total Time    | = 13.506;        | // (usec) | = 260      | chars =   | 2080 Pixels |
| Hor Addr Time     | = 12.468;        | // (usec) | = 240      | chars =   | 1920 Pixels |
Hor Blank Start = 12.468; // (usec) = 240 chars = 1920 Pixels
| Hor Blank Time    | = 1.039;  | // (usec) | =     | 20 chars = | 160 Pixels  |
| ----------------- | --------- | --------- | ----- | ---------- | ----------- |
| Hor Sync Start    | = 12.779; | // (usec) | = 246 | chars =    | 1968 Pixels |
| // H Right Border | = 0.000;  | // (usec) | =     | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.312;  | // (usec) | =     | 6 chars =  | 48 Pixels   |
| Hor Sync Time     | = 0.208;  | // (usec) | =     | 4 chars =  | 32 Pixels   |
10
| // H Back Porch  | = 0.519; | // (usec) | =   | chars =   | 80 Pixels |
| ---------------- | -------- | --------- | --- | --------- | --------- |
| // H Left Border | = 0.000; | // (usec) | =   | 0 chars = | 0 Pixels  |
Ver Total Time = 16.681; // (msec) = 1235 lines HT – (1.06xHA)
| Ver Addr Time      | = 16.208; | // (msec) | = 1200 | lines    | = 0.29 |
| ------------------ | --------- | --------- | ------ | -------- | ------ |
| Ver Blank Start    | = 16.208; | // (msec) | = 1200 | lines    |        |
| Ver Blank Time     | = 0.473;  | // (msec) | =      | 35 lines |        |
| Ver Sync Start     | = 16.248; | // (msec) | = 1203 | lines    |        |
| // V Bottom Border | = 0.000;  | // (msec) | =      | 0 lines  |        |
| // V Front Porch   | = 0.041;  | // (msec) | =      | 3 lines  |        |
| Ver Sync Time      | = 0.081;  | // (msec) | =      | 6 lines  |        |
| // V Back Porch    | = 0.351;  | // (msec) | = 26   | lines    |        |
| // V Top Border    | = 0.000;  | // (msec) | =      | 0 lines  |        |

Definition of Terms: Refer to section 3.2.

VESA MONITOR TIMING STANDARD
Adopted:  8/21/03
Resolution:  1920 x 1200 at 60 Hz (non-interlaced)
EDID ID:  DMT ID: 45h; Std. 2 Byte Code: (D1, 00)h; CVT 3 Byte Code: (57, 28, 28)h
CVT Compliant
Method:

Detailed Timing Parameters
| Timing Name | = 1920 x 1200 @ 60Hz; |           |     |     |     |
| ----------- | --------------------- | --------- | --- | --- | --- |
| Hor Pixels  | = 1920;               | // Pixels |     |     |     |
1200;
| Ver Pixels    | =          | // Lines |        |        |        |
| ------------- | ---------- | -------- | ------ | ------ | ------ |
| Hor Frequency | = 74.556;  | // kHz   | = 13.4 | usec / | line   |
| Ver Frequency | = 59.885;  | // Hz    | = 16.7 | msec / | frame  |
| Pixel Clock   | = 193.250; | // MHz   | = 5.2  | nsec   | ± 0.5% |
8;
| Character Width   | =                | // Pixels | = 41.4     | nsec       |             |
| ----------------- | ---------------- | --------- | ---------- | ---------- | ----------- |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =          | 3.9 %       |
| Hor Sync Polarity | = NEGATIVE       | // HBlank | = 25.9%    | of HTotal  |             |
| Ver Sync Polarity | = POSITIVE;      | // VBlank | = 3.6%     | of VTotal  |             |
| Hor Total Time    | = 13.413;        | // (usec) | = 324      | chars =    | 2592 Pixels |
| Hor Addr Time     | = 9.935;         | // (usec) | = 240      | chars =    | 1920 Pixels |
| Hor Blank Start   | = 9.935;         | // (usec) | = 240      | chars =    | 1920 Pixels |
| Hor Blank Time    | = 3.477;         | // (usec) | =          | 84 chars = | 672 Pixels  |
| Hor Sync Start    | = 10.639;        | // (usec) | = 257      | chars =    | 2056 Pixels |
| // H Right Border | = 0.000;         | // (usec) | =          | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.704;         | // (usec) | = 17       | chars =    | 136 Pixels  |
| Hor Sync Time     | = 1.035;         | // (usec) | = 25       | chars =    | 200 Pixels  |
42
| // H Back Porch  | = 1.739; | // (usec) | =   | chars =   | 336 Pixels |
| ---------------- | -------- | --------- | --- | --------- | ---------- |
| // H Left Border | = 0.000; | // (usec) | =   | 0 chars = | 0 Pixels   |
Ver Total Time = 16.699; // (msec) = 1245 lines HT – (1.06xHA)
| Ver Addr Time      | = 16.095; | // (msec) | = 1200 | lines    | = 2.88 |
| ------------------ | --------- | --------- | ------ | -------- | ------ |
| Ver Blank Start    | = 16.095; | // (msec) | = 1200 | lines    |        |
| Ver Blank Time     | = 0.604;  | // (msec) | =      | 45 lines |        |
| Ver Sync Start     | = 16.135; | // (msec) | = 1203 | lines    |        |
| // V Bottom Border | = 0.000;  | // (msec) | =      | 0 lines  |        |
| // V Front Porch   | = 0.040;  | // (msec) | =      | 3 lines  |        |
| Ver Sync Time      | = 0.080;  | // (msec) | =      | 6 lines  |        |
| // V Back Porch    | = 0.483;  | // (msec) | = 36   | lines    |        |
| // V Top Border    | = 0.000;  | // (msec) | =      | 0 lines  |        |

Definition of Terms: Refer to section 3.4.

VESA MONITOR TIMING STANDARD
Adopted:  8/21/03
Resolution:  1920 x 1200 at 75 Hz (non-interlaced)
EDID ID:  DMT ID: 46h; Std. 2 Byte Code: (D1, 0F)h; CVT 3 Byte Code: (57, 28, 44)h
Method:  CVT Compliant

Detailed Timing Parameters
| Timing Name | = 1920 x 1200 @ 75Hz; |           |     |     |     |
| ----------- | --------------------- | --------- | --- | --- | --- |
| Hor Pixels  | = 1920;               | // Pixels |     |     |     |
1200;
| Ver Pixels    | =          | // Lines |        |        |        |
| ------------- | ---------- | -------- | ------ | ------ | ------ |
| Hor Frequency | = 94.038;  | // kHz   | = 10.6 | usec / | line   |
| Ver Frequency | = 74.930;  | // Hz    | = 13.3 | msec / | frame  |
| Pixel Clock   | = 245.250; | // MHz   | = 4.1  | nsec   | ± 0.5% |
8;
| Character Width   | =                | // Pixels | = 32.6     | nsec       |             |
| ----------------- | ---------------- | --------- | ---------- | ---------- | ----------- |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =          | 4.0 %       |
| Hor Sync Polarity | = NEGATIVE       | // HBlank | = 26.4%    | of HTotal  |             |
| Ver Sync Polarity | = POSITIVE;      | // VBlank | = 4.4%     | of VTotal  |             |
| Hor Total Time    | = 10.634;        | // (usec) | = 326      | chars =    | 2608 Pixels |
| Hor Addr Time     | = 7.829;         | // (usec) | = 240      | chars =    | 1920 Pixels |
| Hor Blank Start   | = 7.829;         | // (usec) | = 240      | chars =    | 1920 Pixels |
| Hor Blank Time    | = 2.805;         | // (usec) | =          | 86 chars = | 688 Pixels  |
| Hor Sync Start    | = 8.383;         | // (usec) | = 257      | chars =    | 2056 Pixels |
| // H Right Border | = 0.000;         | // (usec) | =          | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.555;         | // (usec) | = 17       | chars =    | 136 Pixels  |
| Hor Sync Time     | = 0.848;         | // (usec) | = 26       | chars =    | 208 Pixels  |
43
| // H Back Porch  | = 1.403; | // (usec) | =   | chars =   | 344 Pixels |
| ---------------- | -------- | --------- | --- | --------- | ---------- |
| // H Left Border | = 0.000; | // (usec) | =   | 0 chars = | 0 Pixels   |
Ver Total Time = 13.346; // (msec) = 1255 lines HT – (1.06xHA)
| Ver Addr Time      | = 12.761; | // (msec) | = 1200 | lines    | = 2.34 |
| ------------------ | --------- | --------- | ------ | -------- | ------ |
| Ver Blank Start    | = 12.761; | // (msec) | = 1200 | lines    |        |
| Ver Blank Time     | = 0.585;  | // (msec) | =      | 55 lines |        |
| Ver Sync Start     | = 12.793; | // (msec) | = 1203 | lines    |        |
| // V Bottom Border | = 0.000;  | // (msec) | =      | 0 lines  |        |
| // V Front Porch   | = 0.032;  | // (msec) | =      | 3 lines  |        |
| Ver Sync Time      | = 0.064;  | // (msec) | =      | 6 lines  |        |
| // V Back Porch    | = 0.489;  | // (msec) | = 46   | lines    |        |
| // V Top Border    | = 0.000;  | // (msec) | =      | 0 lines  |        |

Definition of Terms: Refer to section 3.4.

VESA MONITOR TIMING STANDARD
Adopted:  8/21/03
Resolution:  1920 x 1200 at 85 Hz (non-interlaced)
EDID ID:  DMT ID: 47h; Std. 2 Byte Code: (D1, 19)h; CVT 3 Byte Code: (57, 28, 62)h
Method:  CVT Compliant

Detailed Timing Parameters
| Timing Name | = 1920 x 1200 @ 85Hz; |           |     |     |     |
| ----------- | --------------------- | --------- | --- | --- | --- |
| Hor Pixels  | = 1920;               | // Pixels |     |     |     |
1200;
| Ver Pixels    | =          | // Lines |        |        |        |
| ------------- | ---------- | -------- | ------ | ------ | ------ |
| Hor Frequency | = 107.184; | // kHz   | = 9.3  | usec / | line   |
| Ver Frequency | = 84.932;  | // Hz    | = 11.8 | msec / | frame  |
| Pixel Clock   | = 281.250; | // MHz   | = 3.6  | nsec   | ± 0.5% |
8;
| Character Width   | =                | // Pixels | = 28.4     | nsec       |             |
| ----------------- | ---------------- | --------- | ---------- | ---------- | ----------- |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =          | 4.0 %       |
| Hor Sync Polarity | = NEGATIVE       | // HBlank | = 26.8%    | of HTotal  |             |
| Ver Sync Polarity | = POSITIVE;      | // VBlank | = 4.9%     | of VTotal  |             |
| Hor Total Time    | = 9.330;         | // (usec) | = 328      | chars =    | 2624 Pixels |
| Hor Addr Time     | = 6.827;         | // (usec) | = 240      | chars =    | 1920 Pixels |
| Hor Blank Start   | = 6.827;         | // (usec) | = 240      | chars =    | 1920 Pixels |
| Hor Blank Time    | = 2.503;         | // (usec) | =          | 88 chars = | 704 Pixels  |
| Hor Sync Start    | = 7.339;         | // (usec) | = 258      | chars =    | 2064 Pixels |
| // H Right Border | = 0.000;         | // (usec) | =          | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.512;         | // (usec) | = 18       | chars =    | 144 Pixels  |
| Hor Sync Time     | = 0.740;         | // (usec) | = 26       | chars =    | 208 Pixels  |
44
| // H Back Porch  | = 1.252; | // (usec) | =   | chars =   | 352 Pixels |
| ---------------- | -------- | --------- | --- | --------- | ---------- |
| // H Left Border | = 0.000; | // (usec) | =   | 0 chars = | 0 Pixels   |
Ver Total Time = 11.774; // (msec) = 1262 lines HT – (1.06xHA)
| Ver Addr Time      | = 11.196; | // (msec) | = 1200 | lines    | = 2.09 |
| ------------------ | --------- | --------- | ------ | -------- | ------ |
| Ver Blank Start    | = 11.196; | // (msec) | = 1200 | lines    |        |
| Ver Blank Time     | = 0.578;  | // (msec) | =      | 62 lines |        |
| Ver Sync Start     | = 11.224; | // (msec) | = 1203 | lines    |        |
| // V Bottom Border | = 0.000;  | // (msec) | =      | 0 lines  |        |
| // V Front Porch   | = 0.028;  | // (msec) | =      | 3 lines  |        |
| Ver Sync Time      | = 0.056;  | // (msec) | =      | 6 lines  |        |
| // V Back Porch    | = 0.494;  | // (msec) | = 53   | lines    |        |
| // V Top Border    | = 0.000;  | // (msec) | =      | 0 lines  |        |

Definition of Terms: Refer to section 3.4.

VESA MONITOR TIMING STANDARD
Adopted:  5/1/07
Resolution:  1920 x 1200 at 120 Hz (non-interlaced) REDUCED BLANKING
EDID ID:  DMT ID: 48h; Std. 2 Byte Code: n/a; CVT 3 Byte Code: n/a
Method:  Generated using CVT (Reduced Blanking) Formula

Detailed Timing Parameters
| Timing Name | = 1920 x 1200 @ 120Hz CVT (Reduced Blanking); |           |     |     |     |
| ----------- | --------------------------------------------- | --------- | --- | --- | --- |
| Hor Pixels  | = 1920;                                       | // Pixels |     |     |     |
1200;
| Ver Pixels    | =          | // Lines |       |        |        |
| ------------- | ---------- | -------- | ----- | ------ | ------ |
| Hor Frequency | = 152.404; | // kHz   | = 6.6 | usec / | line   |
| Ver Frequency | = 119.909; | // Hz    | = 8.3 | msec / | frame  |
| Pixel Clock   | = 317.000; | // MHz   | = 3.2 | nsec   | ± 0.5% |
8;
| Character Width   | =                | // Pixels | = 25.2     | nsec       |             |
| ----------------- | ---------------- | --------- | ---------- | ---------- | ----------- |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =          | 0.8 %       |
| Hor Sync Polarity | = POSITIVE;      | // HBlank | = 7.7%     | of HTotal  |             |
| Ver Sync Polarity | = NEGATIVE       | // VBlank | = 5.6%     | of VTotal  |             |
| Hor Total Time    | = 6.562;         | // (usec) | = 260      | chars =    | 2080 Pixels |
| Hor Addr Time     | = 6.057;         | // (usec) | = 240      | chars =    | 1920 Pixels |
| Hor Blank Start   | = 6.057;         | // (usec) | = 240      | chars =    | 1920 Pixels |
| Hor Blank Time    | = 0.505;         | // (usec) | =          | 20 chars = | 160 Pixels  |
| Hor Sync Start    | = 6.208;         | // (usec) | = 246      | chars =    | 1968 Pixels |
| // H Right Border | = 0.000;         | // (usec) | =          | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.151;         | // (usec) | =          | 6 chars =  | 48 Pixels   |
| Hor Sync Time     | = 0.101;         | // (usec) | =          | 4 chars =  | 32 Pixels   |
10
| // H Back Porch  | = 0.252; | // (usec) | =   | chars =   | 80 Pixels |
| ---------------- | -------- | --------- | --- | --------- | --------- |
| // H Left Border | = 0.000; | // (usec) | =   | 0 chars = | 0 Pixels  |
Ver Total Time = 8.340; // (msec) = 1271 lines HT – (1.06xHA)
| Ver Addr Time      | = 7.874; | // (msec) | = 1200 | lines    | = 0.14 |
| ------------------ | -------- | --------- | ------ | -------- | ------ |
| Ver Blank Start    | = 7.874; | // (msec) | = 1200 | lines    |        |
| Ver Blank Time     | = 0.466; | // (msec) | =      | 71 lines |        |
| Ver Sync Start     | = 7.894; | // (msec) | = 1203 | lines    |        |
| // V Bottom Border | = 0.000; | // (msec) | =      | 0 lines  |        |
| // V Front Porch   | = 0.020; | // (msec) | =      | 3 lines  |        |
| Ver Sync Time      | = 0.039; | // (msec) | =      | 6 lines  |        |
| // V Back Porch    | = 0.407; | // (msec) | = 62   | lines    |        |
| // V Top Border    | = 0.000; | // (msec) | =      | 0 lines  |        |

Definition of Terms: Refer to section 3.2.

VESA MONITOR TIMING STANDARD
Adopted:  9/17/98
Resolution:  1920 x 1440 at 60 Hz (non-interlaced)
EDID ID:  DMT ID: 49h; Std. 2 Byte Code: (D1, 40)h; CVT 3 Byte Code: n/a
Method:  *** NOT CVT COMPLIANT ***

Detailed Timing Parameters
| Timing Name   | = 1920 x 1440 @ 60Hz; |           |        |        |        |     |
| ------------- | --------------------- | --------- | ------ | ------ | ------ | --- |
| Hor Pixels    | = 1920;               | // Pixels |        |        |        |     |
| Ver Pixels    | = 1440;               | // Lines  |        |        |        |     |
| Hor Frequency | = 90.000;             | // kHz    | = 11.1 | usec / | line   |     |
| Ver Frequency | = 60.000;             | // Hz     | = 16.7 | msec / | frame  |     |
| Pixel Clock   | = 234.000;            | // MHz    | = 4.3  | nsec   | ± 0.5% |     |
8;
| Character Width   | =                | // Pixels | = 34.2     | nsec       |             |     |
| ----------------- | ---------------- | --------- | ---------- | ---------- | ----------- | --- |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =          | 4.2 %       |     |
| Hor Sync Polarity | = NEGATIVE;      | // HBlank | = 26.2%    | of HTotal  |             |     |
| Ver Sync Polarity | = POSITIVE;      | // VBlank | = 4.0%     | of VTotal  |             |     |
| Hor Total Time    | = 11.111;        | // (usec) | = 325      | chars =    | 2600 Pixels |     |
| Hor Addr Time     | = 8.205;         | // (usec) | = 240      | chars =    | 1920 Pixels |     |
| Hor Blank Start   | = 8.205;         | // (usec) | = 240      | chars =    | 1920 Pixels |     |
| Hor Blank Time    | = 2.906;         | // (usec) | =          | 85 chars = | 680 Pixels  |     |
| Hor Sync Start    | = 8.752;         | // (usec) | = 256      | chars =    | 2048 Pixels |     |
| // H Right Border | = 0.000;         | // (usec) | =          | 0 chars =  | 0 Pixels    |     |
16
| // H Front Porch | = 0.547; | // (usec) | =   | chars =    | 128 Pixels |     |
| ---------------- | -------- | --------- | --- | ---------- | ---------- | --- |
| Hor Sync Time    | = 0.889; | // (usec) | =   | 26 chars = | 208 Pixels |     |
| // H Back Porch  | = 1.470; | // (usec) | =   | 43 chars = | 344 Pixels |     |
| // H Left Border | = 0.000; | // (usec) | =   | 0 chars =  | 0 Pixels   |     |
Ver Total Time = 16.667; // (msec) = 1500 lines HT – (1.06xHA)
| Ver Addr Time      | = 16.000; | // (msec) | = 1440 | lines    | = 2.41 |     |
| ------------------ | --------- | --------- | ------ | -------- | ------ | --- |
| Ver Blank Start    | = 16.000; | // (msec) | = 1440 | lines    |        |     |
| Ver Blank Time     | = 0.667;  | // (msec) | =      | 60 lines |        |     |
| Ver Sync Start     | = 16.011; | // (msec) | = 1441 | lines    |        |     |
| // V Bottom Border | = 0.000;  | // (msec) | =      | 0 lines  |        |     |
1
| // V Front Porch | = 0.011; | // (msec) | =   | lines    |     |     |
| ---------------- | -------- | --------- | --- | -------- | --- | --- |
| Ver Sync Time    | = 0.033; | // (msec) | =   | 3 lines  |     |     |
| // V Back Porch  | = 0.622; | // (msec) | =   | 56 lines |     |     |
| // V Top Border  | = 0.000; | // (msec) | =   | 0 lines  |     |     |

Definition of Terms: Refer to section 3.4.

VESA MONITOR TIMING STANDARD
Adopted:  9/17/98
Resolution:  1920 x 1440 at 75 Hz (non-interlaced)
EDID ID:  DMT ID: 4Ah; Std. 2 Byte Code: (D1, 4F)h; CVT 3 Byte Code: n/a
Method:  *** NOT CVT COMPLIANT ***

Detailed Timing Parameters
| Timing Name       | = 1920 x 1440 @ 75Hz; |           |                   |               |
| ----------------- | --------------------- | --------- | ----------------- | ------------- |
| Hor Pixels        | = 1920;               | // Pixels |                   |               |
| Ver Pixels        | = 1440;               | // Lines  |                   |               |
| Hor Frequency     | = 112.500;            | // kHz    | = 8.9 usec        | / line        |
| Ver Frequency     | = 75.000;             | // Hz     | = 13.3 msec       | / frame       |
| Pixel Clock       | = 297.000;            | // MHz    | = 3.4 nsec        | ± 0.5%        |
| Character Width   | = 8;                  | // Pixels | = 26.9 nsec       |               |
| Scan Type         | = NONINTERLACED;      |           | // H Phase        | = 3.9 %       |
| Hor Sync Polarity | = NEGATIVE            | // HBlank | = 27.3% of HTotal |               |
| Ver Sync Polarity | = POSITIVE;           | // VBlank | = 4.0% of VTotal  |               |
| Hor Total Time    | = 8.889;              | // (usec) | = 330 chars       | = 2640 Pixels |
| Hor Addr Time     | = 6.465;              | // (usec) | = 240 chars       | = 1920 Pixels |
| Hor Blank Start   | = 6.465;              | // (usec) | = 240 chars       | = 1920 Pixels |
| Hor Blank Time    | = 2.424;              | // (usec) | = 90 chars        | = 720 Pixels  |
| Hor Sync Start    | = 6.949;              | // (usec) | = 258 chars       | = 2064 Pixels |
| // H Right Border | = 0.000;              | // (usec) | = 0 chars         | = 0 Pixels    |
| // H Front Porch  | = 0.485;              | // (usec) | = 18 chars        | = 144 Pixels  |
| Hor Sync Time     | = 0.754;              | // (usec) | = 28 chars        | = 224 Pixels  |
| // H Back Porch   | = 1.185;              | // (usec) | = 44 chars        | = 352 Pixels  |
| // H Left Border  | = 0.000;              | // (usec) | = 0 chars         | = 0 Pixels    |
Ver Total Time = 13.333; // (msec) = 1500 lines HT – (1.06xHA)
| Ver Addr Time      | = 12.800; | // (msec) | = 1440 lines | = 2.04 |
| ------------------ | --------- | --------- | ------------ | ------ |
| Ver Blank Start    | = 12.800; | // (msec) | = 1440 lines |        |
| Ver Blank Time     | = 0.533;  | // (msec) | = 60 lines   |        |
| Ver Sync Start     | = 12.809; | // (msec) | = 1441 lines |        |
| // V Bottom Border | = 0.000;  | // (msec) | = 0 lines    |        |
| // V Front Porch   | = 0.009;  | // (msec) | = 1 lines    |        |
| Ver Sync Time      | = 0.027;  | // (msec) | = 3 lines    |        |
56
| // V Back Porch | = 0.498; | // (msec) | = lines   |     |
| --------------- | -------- | --------- | --------- | --- |
| // V Top Border | = 0.000; | // (msec) | = 0 lines |     |

Definition of Terms: Refer to section 3.4.

VESA MONITOR TIMING STANDARD
Adopted:  5/1/07
Resolution:  1920 x 1440 at 120 Hz (non-interlaced) REDUCED BLANKING
EDID ID:  DMT ID: 4Bh; Std. 2 Byte Code: n/a; CVT 3 Byte Code: n/a
Method:  Generated using CVT (Reduced Blanking) Formula

Detailed Timing Parameters
| Timing Name       | = 1920 x 1440 @ 120Hz CVT (Reduced Blanking); |           |            |            |             |
| ----------------- | --------------------------------------------- | --------- | ---------- | ---------- | ----------- |
| Hor Pixels        | = 1920;                                       | // Pixels |            |            |             |
| Ver Pixels        | = 1440;                                       | // Lines  |            |            |             |
| Hor Frequency     | = 182.933;                                    | // kHz    | = 5.5      | usec /     | line        |
| Ver Frequency     | = 119.956;                                    | // Hz     | = 8.3      | msec /     | frame       |
| Pixel Clock       | = 380.500;                                    | // MHz    | = 2.6      | nsec       | ± 0.5%      |
| Character Width   | = 8;                                          | // Pixels | = 21.0     | nsec       |             |
| Scan Type         | = NONINTERLACED;                              |           | // H Phase | =          | 0.8 %       |
| Hor Sync Polarity | = POSITIVE;                                   | // HBlank | = 7.7%     | of HTotal  |             |
| Ver Sync Polarity | = NEGATIVE                                    | // VBlank | = 5.6%     | of VTotal  |             |
| Hor Total Time    | = 5.466;                                      | // (usec) | = 260      | chars =    | 2080 Pixels |
| Hor Addr Time     | = 5.046;                                      | // (usec) | = 240      | chars =    | 1920 Pixels |
| Hor Blank Start   | = 5.046;                                      | // (usec) | = 240      | chars =    | 1920 Pixels |
| Hor Blank Time    | = 0.420;                                      | // (usec) | =          | 20 chars = | 160 Pixels  |
| Hor Sync Start    | = 5.172;                                      | // (usec) | = 246      | chars =    | 1968 Pixels |
| // H Right Border | = 0.000;                                      | // (usec) | =          | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.126;                                      | // (usec) | =          | 6 chars =  | 48 Pixels   |
| Hor Sync Time     | = 0.084;                                      | // (usec) | =          | 4 chars =  | 32 Pixels   |
| // H Back Porch   | = 0.210;                                      | // (usec) | = 10       | chars =    | 80 Pixels   |
| // H Left Border  | = 0.000;                                      | // (usec) | =          | 0 chars =  | 0 Pixels    |
Ver Total Time = 8.336; // (msec) = 1525 lines HT – (1.06xHA)
| Ver Addr Time      | = 7.872; | // (msec) | = 1440 | lines    | = 0.12 |
| ------------------ | -------- | --------- | ------ | -------- | ------ |
| Ver Blank Start    | = 7.872; | // (msec) | = 1440 | lines    |        |
| Ver Blank Time     | = 0.465; | // (msec) | =      | 85 lines |        |
| Ver Sync Start     | = 7.888; | // (msec) | = 1443 | lines    |        |
| // V Bottom Border | = 0.000; | // (msec) | =      | 0 lines  |        |
| // V Front Porch   | = 0.016; | // (msec) | =      | 3 lines  |        |
| Ver Sync Time      | = 0.022; | // (msec) | =      | 4 lines  |        |
| // V Back Porch    | = 0.426; | // (msec) | = 78   | lines    |        |
| // V Top Border    | = 0.000; | // (msec) | =      | 0 lines  |        |

Definition of Terms: Refer to section 3.2.

VESA MONITOR TIMING STANDARD
Adopted:  11/17/08
Resolution:  2048 x 1152 at 60 Hz (non-interlaced) REDUCED BLANKING
EDID ID:  DMT ID: 54h; Std. 2 Byte Code: E1h, C0h; CVT 3 Byte Code: n/a
Method:  *** NOT CVT COMPLIANT ***

Detailed Timing Parameters
| Timing Name     | = 2048 x 1152 @ 60Hz; |           |        |        |        |
| --------------- | --------------------- | --------- | ------ | ------ | ------ |
| Hor Pixels      | = 2048;               | // Pixels |        |        |        |
| Ver Pixels      | = 1152;               | // Lines  |        |        |        |
| Hor Frequency   | = 72.000;             | // KHz    | = 13.9 | usec / | line   |
| Ver Frequency   | = 60.000;             | // Hz     | = 16.7 | msec / | frame  |
| Pixel Clock     | = 162.000;            | // MHz    | = 6.2  | nsec   | ± 0.5% |
| Character Width | = 1;                  | // Pixels | = 6.2  | nsec   |        |
NONINTERLACED;
| Scan Type         | =           |           | // H Phase | =         | 1.6 % |
| ----------------- | ----------- | --------- | ---------- | --------- | ----- |
| Hor Sync Polarity | = POSITIVE; | // HBlank | = 9.0%     | of HTotal |       |
| Ver Sync Polarity | = POSITIVE; | // VBlank | = 4.0%     | of VTotal |       |
Hor Total Time = 13.889; // (usec) = 2250 chars = 2250 Pixels
| Hor Addr Time | = 12.642; | // (usec) | = 2048 | chars = | 2048 Pixels |
| ------------- | --------- | --------- | ------ | ------- | ----------- |
Hor Blank Start = 12.642; // (usec) = 2048 chars = 2048 Pixels
| Hor Blank Time | = 1.247; | // (usec) | = 202 | chars = | 202 Pixels |
| -------------- | -------- | --------- | ----- | ------- | ---------- |
Hor Sync Start = 12.802; // (usec) = 2074 chars = 2074 Pixels
| // H Right Border | = 0.000; | // (usec) | =   | 0 chars =  | 0 Pixels  |
| ----------------- | -------- | --------- | --- | ---------- | --------- |
| // H Front Porch  | = 0.160; | // (usec) | =   | 26 chars = | 26 Pixels |
| Hor Sync Time     | = 0.494; | // (usec) | =   | 80 chars = | 80 Pixels |
| // H Back Porch   | = 0.593; | // (usec) | =   | 96 chars = | 96 Pixels |
| // H Left Border  | = 0.000; | // (usec) | =   | 0 chars =  | 0 Pixels  |
Ver Total Time = 16.667; // (msec) = 1200 lines HT – (1.06xHA)
| Ver Addr Time      | = 16.000; | // (msec) | = 1152 | lines    | = 0.49 |
| ------------------ | --------- | --------- | ------ | -------- | ------ |
| Ver Blank Start    | = 16.000; | // (msec) | = 1152 | lines    |        |
| Ver Blank Time     | = 0.667;  | // (msec) | =      | 48 lines |        |
| Ver Sync Start     | = 16.014; | // (msec) | = 1153 | lines    |        |
| // V Bottom Border | = 0.000;  | // (msec) | =      | 0 lines  |        |
| // V Front Porch   | = 0.014;  | // (msec) | =      | 1 lines  |        |
| Ver Sync Time      | = 0.042;  | // (msec) | =      | 3 lines  |        |
| // V Back Porch    | = 0.611;  | // (msec) | =      | 44 lines |        |
| // V Top Border    | = 0.000;  | // (msec) | =      | 0 lines  |        |

Definition of Terms: Refer to Section 3.1

VESA MONITOR TIMING STANDARD
Adopted:  5/1/07
Resolution:  2560 x 1600 at 60 Hz (non-interlaced) REDUCED BLANKING
EDID ID:  DMT ID: 4Ch; Std. 2 Byte Code: n/a; CVT 3 Byte Code: (1F, 38, 21)h
Method:  CVT Compliant

Detailed Timing Parameters
| Timing Name   | = 2560 x 1600 @ 60Hz CVT (Reduced Blanking); |           |        |        |        |
| ------------- | -------------------------------------------- | --------- | ------ | ------ | ------ |
| Hor Pixels    | = 2560;                                      | // Pixels |        |        |        |
| Ver Pixels    | = 1600;                                      | // Lines  |        |        |        |
| Hor Frequency | = 98.713;                                    | // kHz    | = 10.1 | usec / | line   |
| Ver Frequency | = 59.972;                                    | // Hz     | = 16.7 | msec / | frame  |
| Pixel Clock   | = 268.500;                                   | // MHz    | = 3.7  | nsec   | ± 0.5% |
8;
| Character Width   | =                | // Pixels | = 29.8     | nsec       |             |
| ----------------- | ---------------- | --------- | ---------- | ---------- | ----------- |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =          | 0.6 %       |
| Hor Sync Polarity | = POSITIVE;      | // HBlank | = 5.9%     | of HTotal  |             |
| Ver Sync Polarity | = NEGATIVE;      | // VBlank | = 2.8%     | of VTotal  |             |
| Hor Total Time    | = 10.130;        | // (usec) | = 340      | chars =    | 2720 Pixels |
| Hor Addr Time     | = 9.534;         | // (usec) | = 320      | chars =    | 2560 Pixels |
| Hor Blank Start   | = 9.534;         | // (usec) | = 320      | chars =    | 2560 Pixels |
| Hor Blank Time    | = 0.596;         | // (usec) | =          | 20 chars = | 160 Pixels  |
| Hor Sync Start    | = 9.713;         | // (usec) | = 326      | chars =    | 2608 Pixels |
| // H Right Border | = 0.000;         | // (usec) | =          | 0 chars =  | 0 Pixels    |
6
| // H Front Porch | = 0.179; | // (usec) | =   | chars =    | 48 Pixels |
| ---------------- | -------- | --------- | --- | ---------- | --------- |
| Hor Sync Time    | = 0.119; | // (usec) | =   | 4 chars =  | 32 Pixels |
| // H Back Porch  | = 0.298; | // (usec) | =   | 10 chars = | 80 Pixels |
| // H Left Border | = 0.000; | // (usec) | =   | 0 chars =  | 0 Pixels  |
Ver Total Time = 16.675; // (msec) = 1646 lines HT – (1.06xHA)
| Ver Addr Time      | = 16.209; | // (msec) | = 1600 | lines    | = 0.02 |
| ------------------ | --------- | --------- | ------ | -------- | ------ |
| Ver Blank Start    | = 16.209; | // (msec) | = 1600 | lines    |        |
| Ver Blank Time     | = 0.466;  | // (msec) | =      | 46 lines |        |
| Ver Sync Start     | = 16.239; | // (msec) | = 1603 | lines    |        |
| // V Bottom Border | = 0.000;  | // (msec) | =      | 0 lines  |        |
3
| // V Front Porch | = 0.030; | // (msec) | =   | lines    |     |
| ---------------- | -------- | --------- | --- | -------- | --- |
| Ver Sync Time    | = 0.061; | // (msec) | =   | 6 lines  |     |
| // V Back Porch  | = 0.375; | // (msec) | =   | 37 lines |     |
| // V Top Border  | = 0.000; | // (msec) | =   | 0 lines  |     |

Definition of Terms: Refer to section 3.2.

VESA MONITOR TIMING STANDARD
Adopted:  5/1/07
Resolution:  2560 x 1600 at 60 Hz (non-interlaced)
EDID ID:  DMT ID: 4Dh; Std. 2 Byte Code: n/a; CVT 3 Byte Code: (1F, 38, 28)h
Method:  CVT Compliant

Detailed Timing Parameters
| Timing Name   | = 2560 x 1600 @ 60Hz; |           |        |        |        |
| ------------- | --------------------- | --------- | ------ | ------ | ------ |
| Hor Pixels    | = 2560;               | // Pixels |        |        |        |
| Ver Pixels    | = 1600;               | // Lines  |        |        |        |
| Hor Frequency | = 99.458;             | // kHz    | = 10.1 | usec / | line   |
| Ver Frequency | = 59.987;             | // Hz     | = 16.7 | msec / | frame  |
| Pixel Clock   | = 348.500;            | // MHz    | = 2.9  | nsec   | ± 0.5% |
8;
| Character Width   | =                | // Pixels | = 23.0     | nsec      |             |
| ----------------- | ---------------- | --------- | ---------- | --------- | ----------- |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =         | 4.0 %       |
| Hor Sync Polarity | = NEGATIVE       | // HBlank | = 26.9%    | of HTotal |             |
| Ver Sync Polarity | = POSITIVE;      | // VBlank | = 3.5%     | of VTotal |             |
| Hor Total Time    | = 10.055;        | // (usec) | = 438      | chars =   | 3504 Pixels |
| Hor Addr Time     | = 7.346;         | // (usec) | = 320      | chars =   | 2560 Pixels |
| Hor Blank Start   | = 7.346;         | // (usec) | = 320      | chars =   | 2560 Pixels |
| Hor Blank Time    | = 2.709;         | // (usec) | = 118      | chars =   | 944 Pixels  |
| Hor Sync Start    | = 7.897;         | // (usec) | = 344      | chars =   | 2752 Pixels |
| // H Right Border | = 0.000;         | // (usec) | =          | 0 chars = | 0 Pixels    |
24
| // H Front Porch | = 0.551; | // (usec) | =   | chars =    | 192 Pixels |
| ---------------- | -------- | --------- | --- | ---------- | ---------- |
| Hor Sync Time    | = 0.803; | // (usec) | =   | 35 chars = | 280 Pixels |
| // H Back Porch  | = 1.354; | // (usec) | =   | 59 chars = | 472 Pixels |
| // H Left Border | = 0.000; | // (usec) | =   | 0 chars =  | 0 Pixels   |
Ver Total Time = 16.670; // (msec) = 1658 lines HT – (1.06xHA)
| Ver Addr Time      | = 16.087; | // (msec) | = 1600 | lines    | = 2.27 |
| ------------------ | --------- | --------- | ------ | -------- | ------ |
| Ver Blank Start    | = 16.087; | // (msec) | = 1600 | lines    |        |
| Ver Blank Time     | = 0.583;  | // (msec) | =      | 58 lines |        |
| Ver Sync Start     | = 16.117; | // (msec) | = 1603 | lines    |        |
| // V Bottom Border | = 0.000;  | // (msec) | =      | 0 lines  |        |
3
| // V Front Porch | = 0.030; | // (msec) | =   | lines    |     |
| ---------------- | -------- | --------- | --- | -------- | --- |
| Ver Sync Time    | = 0.060; | // (msec) | =   | 6 lines  |     |
| // V Back Porch  | = 0.493; | // (msec) | =   | 49 lines |     |
| // V Top Border  | = 0.000; | // (msec) | =   | 0 lines  |     |

Definition of Terms: Refer to section 3.4.

VESA MONITOR TIMING STANDARD
Adopted:  5/1/07
Resolution:  2560 x 1600 at 75 Hz (non-interlaced)
EDID ID:  DMT ID: 4Eh; Std. 2 Byte Code: n/a; CVT 3 Byte Code: (1F, 38, 44)h
Method:  CVT Compliant

Detailed Timing Parameters
| Timing Name   | = 2560 x 1600 @ 75Hz; |           |        |        |        |
| ------------- | --------------------- | --------- | ------ | ------ | ------ |
| Hor Pixels    | = 2560;               | // Pixels |        |        |        |
| Ver Pixels    | = 1600;               | // Lines  |        |        |        |
| Hor Frequency | = 125.354;            | // kHz    | = 8.0  | usec / | line   |
| Ver Frequency | = 74.972;             | // Hz     | = 13.3 | msec / | frame  |
| Pixel Clock   | = 443.250;            | // MHz    | = 2.3  | nsec   | ± 0.5% |
8;
| Character Width   | =                | // Pixels | = 18.0     | nsec      |             |
| ----------------- | ---------------- | --------- | ---------- | --------- | ----------- |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =         | 4.0 %       |
| Hor Sync Polarity | = NEGATIVE       | // HBlank | = 27.6%    | of HTotal |             |
| Ver Sync Polarity | = POSITIVE;      | // VBlank | = 4.3%     | of VTotal |             |
| Hor Total Time    | = 7.977;         | // (usec) | = 442      | chars =   | 3536 Pixels |
| Hor Addr Time     | = 5.776;         | // (usec) | = 320      | chars =   | 2560 Pixels |
| Hor Blank Start   | = 5.776;         | // (usec) | = 320      | chars =   | 2560 Pixels |
| Hor Blank Time    | = 2.202;         | // (usec) | = 122      | chars =   | 976 Pixels  |
| Hor Sync Start    | = 6.245;         | // (usec) | = 346      | chars =   | 2768 Pixels |
| // H Right Border | = 0.000;         | // (usec) | =          | 0 chars = | 0 Pixels    |
26
| // H Front Porch | = 0.469; | // (usec) | =   | chars =    | 208 Pixels |
| ---------------- | -------- | --------- | --- | ---------- | ---------- |
| Hor Sync Time    | = 0.632; | // (usec) | =   | 35 chars = | 280 Pixels |
| // H Back Porch  | = 1.101; | // (usec) | =   | 61 chars = | 488 Pixels |
| // H Left Border | = 0.000; | // (usec) | =   | 0 chars =  | 0 Pixels   |
Ver Total Time = 13.338; // (msec) = 1672 lines HT – (1.06xHA)
| Ver Addr Time      | = 12.764; | // (msec) | = 1600 | lines    | = 1.86 |
| ------------------ | --------- | --------- | ------ | -------- | ------ |
| Ver Blank Start    | = 12.764; | // (msec) | = 1600 | lines    |        |
| Ver Blank Time     | = 0.574;  | // (msec) | =      | 72 lines |        |
| Ver Sync Start     | = 12.788; | // (msec) | = 1603 | lines    |        |
| // V Bottom Border | = 0.000;  | // (msec) | =      | 0 lines  |        |
3
| // V Front Porch | = 0.024; | // (msec) | =   | lines    |     |
| ---------------- | -------- | --------- | --- | -------- | --- |
| Ver Sync Time    | = 0.048; | // (msec) | =   | 6 lines  |     |
| // V Back Porch  | = 0.503; | // (msec) | =   | 63 lines |     |
| // V Top Border  | = 0.000; | // (msec) | =   | 0 lines  |     |

Definition of Terms: Refer to section 3.4.

VESA MONITOR TIMING STANDARD
Adopted:  5/1/07
Resolution:  2560 x 1600 at 85 Hz (non-interlaced)
EDID ID:  DMT ID: 4Fh; Std. 2 Byte Code: n/a; CVT 3 Byte Code: (1F, 38, 62)h
Method:  CVT Compliant

Detailed Timing Parameters
| Timing Name   | = 2560 x 1600 @ 85Hz; |           |        |        |        |     |
| ------------- | --------------------- | --------- | ------ | ------ | ------ | --- |
| Hor Pixels    | = 2560;               | // Pixels |        |        |        |     |
| Ver Pixels    | = 1600;               | // Lines  |        |        |        |     |
| Hor Frequency | = 142.887;            | // kHz    | = 7.0  | usec / | line   |     |
| Ver Frequency | = 84.951;             | // Hz     | = 11.8 | msec / | frame  |     |
| Pixel Clock   | = 505.250;            | // MHz    | = 2.0  | nsec   | ± 0.5% |     |
8;
| Character Width   | =                | // Pixels | = 15.8     | nsec      |             |     |
| ----------------- | ---------------- | --------- | ---------- | --------- | ----------- | --- |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =         | 4.0 %       |     |
| Hor Sync Polarity | = NEGATIVE       | // HBlank | = 27.6%    | of HTotal |             |     |
| Ver Sync Polarity | = POSITIVE;      | // VBlank | = 4.9%     | of VTotal |             |     |
| Hor Total Time    | = 6.999;         | // (usec) | = 442      | chars =   | 3536 Pixels |     |
| Hor Addr Time     | = 5.067;         | // (usec) | = 320      | chars =   | 2560 Pixels |     |
| Hor Blank Start   | = 5.067;         | // (usec) | = 320      | chars =   | 2560 Pixels |     |
| Hor Blank Time    | = 1.932;         | // (usec) | = 122      | chars =   | 976 Pixels  |     |
| Hor Sync Start    | = 5.478;         | // (usec) | = 346      | chars =   | 2768 Pixels |     |
| // H Right Border | = 0.000;         | // (usec) | =          | 0 chars = | 0 Pixels    |     |
26
| // H Front Porch | = 0.412; | // (usec) | =   | chars =    | 208 Pixels |     |
| ---------------- | -------- | --------- | --- | ---------- | ---------- | --- |
| Hor Sync Time    | = 0.554; | // (usec) | =   | 35 chars = | 280 Pixels |     |
| // H Back Porch  | = 0.966; | // (usec) | =   | 61 chars = | 488 Pixels |     |
| // H Left Border | = 0.000; | // (usec) | =   | 0 chars =  | 0 Pixels   |     |
Ver Total Time = 11.772; // (msec) = 1682 lines HT – (1.06xHA)
| Ver Addr Time      | = 11.198; | // (msec) | = 1600 | lines    | = 1.63 |     |
| ------------------ | --------- | --------- | ------ | -------- | ------ | --- |
| Ver Blank Start    | = 11.198; | // (msec) | = 1600 | lines    |        |     |
| Ver Blank Time     | = 0.574;  | // (msec) | =      | 82 lines |        |     |
| Ver Sync Start     | = 11.219; | // (msec) | = 1603 | lines    |        |     |
| // V Bottom Border | = 0.000;  | // (msec) | =      | 0 lines  |        |     |
3
| // V Front Porch | = 0.021; | // (msec) | =   | lines    |     |     |
| ---------------- | -------- | --------- | --- | -------- | --- | --- |
| Ver Sync Time    | = 0.042; | // (msec) | =   | 6 lines  |     |     |
| // V Back Porch  | = 0.511; | // (msec) | =   | 73 lines |     |     |
| // V Top Border  | = 0.000; | // (msec) | =   | 0 lines  |     |     |

Definition of Terms: Refer to section 3.4.

VESA MONITOR TIMING STANDARD
Adopted:  5/1/07
Resolution:  2560 x 1600 at 120 Hz (non-interlaced) REDUCED BLANKING
EDID ID:  DMT ID: 50h; Std. 2 Byte Code: n/a; CVT 3 Byte Code: n/a
Method:  Generated using CVT (Reduced Blanking) Formula

Detailed Timing Parameters
| Timing Name | = 2560 x 1600 @ 120Hz CVT (Reduced Blanking); |           |     |     |     |
| ----------- | --------------------------------------------- | --------- | --- | --- | --- |
| Hor Pixels  | = 2560;                                       | // Pixels |     |     |     |
1600;
| Ver Pixels    | =          | // Lines |       |        |        |
| ------------- | ---------- | -------- | ----- | ------ | ------ |
| Hor Frequency | = 203.217; | // kHz   | = 4.9 | usec / | line   |
| Ver Frequency | = 119.963; | // Hz    | = 8.3 | msec / | frame  |
| Pixel Clock   | = 552.750; | // MHz   | = 1.8 | nsec   | ± 0.5% |
8;
| Character Width   | =                | // Pixels | = 14.5     | nsec       |             |
| ----------------- | ---------------- | --------- | ---------- | ---------- | ----------- |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =          | 0.6 %       |
| Hor Sync Polarity | = POSITIVE;      | // HBlank | = 5.9%     | of HTotal  |             |
| Ver Sync Polarity | = NEGATIVE       | // VBlank | = 5.5%     | of VTotal  |             |
| Hor Total Time    | = 4.921;         | // (usec) | = 340      | chars =    | 2720 Pixels |
| Hor Addr Time     | = 4.631;         | // (usec) | = 320      | chars =    | 2560 Pixels |
| Hor Blank Start   | = 4.631;         | // (usec) | = 320      | chars =    | 2560 Pixels |
| Hor Blank Time    | = 0.289;         | // (usec) | =          | 20 chars = | 160 Pixels  |
| Hor Sync Start    | = 4.718;         | // (usec) | = 326      | chars =    | 2608 Pixels |
| // H Right Border | = 0.000;         | // (usec) | =          | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.087;         | // (usec) | =          | 6 chars =  | 48 Pixels   |
| Hor Sync Time     | = 0.058;         | // (usec) | =          | 4 chars =  | 32 Pixels   |
10
| // H Back Porch  | = 0.145; | // (usec) | =   | chars =   | 80 Pixels |
| ---------------- | -------- | --------- | --- | --------- | --------- |
| // H Left Border | = 0.000; | // (usec) | =   | 0 chars = | 0 Pixels  |
Ver Total Time = 8.336; // (msec) = 1694 lines HT – (1.06xHA)
| Ver Addr Time      | = 7.873; | // (msec) | = 1600 | lines    | = 0.01 |
| ------------------ | -------- | --------- | ------ | -------- | ------ |
| Ver Blank Start    | = 7.873; | // (msec) | = 1600 | lines    |        |
| Ver Blank Time     | = 0.463; | // (msec) | =      | 94 lines |        |
| Ver Sync Start     | = 7.888; | // (msec) | = 1603 | lines    |        |
| // V Bottom Border | = 0.000; | // (msec) | =      | 0 lines  |        |
| // V Front Porch   | = 0.015; | // (msec) | =      | 3 lines  |        |
| Ver Sync Time      | = 0.030; | // (msec) | =      | 6 lines  |        |
| // V Back Porch    | = 0.418; | // (msec) | = 85   | lines    |        |
| // V Top Border    | = 0.000; | // (msec) | =      | 0 lines  |        |

Definition of Terms: Refer to section 3.2.
VESA MONITOR TIMING STANDARD
Proposed:  2/8/13
Resolution:  4096 x 2160 at 60 Hz (non-interlaced) REDUCED BLANKING v2
EDID ID:  DMT ID: 57h; Std. 2 Byte Code: n/a; CVT 3 Byte Code: n/a
Method:  Generated using CVT (Reduced Blanking v2) Formula

Detailed Timing Parameters
| Timing Name       | = 4096 x 2160 @ 60Hz CVT (Reduced Blanking v2); |           |            |           |             |
| ----------------- | ----------------------------------------------- | --------- | ---------- | --------- | ----------- |
| Hor Pixels        | = 4096;                                         | // Pixels |            |           |             |
| Ver Pixels        | = 2160;                                         | // Lines  |            |           |             |
| Hor Frequency     | = 133.320;                                      | // kHz    | = 7.5      | usec /    | line        |
| Ver Frequency     | = 60.000;                                       | // Hz     | = 16.7     | msec /    | frame       |
| Pixel Clock       | = 556.744;                                      | // MHz    | = 1.8      | nsec      | ± 0.5%      |
| Character Width   | = 1;                                            | // Pixels | = 1.8      | nsec      |             |
| Scan Type         | = NONINTERLACED;                                |           | // H Phase | =         | 0.4 %       |
| Hor Sync Polarity | = POSITIVE;                                     | // HBlank | = 1.9%     | of HTotal |             |
| Ver Sync Polarity | = NEGATIVE                                      | // VBlank | = 2.8%     | of VTotal |             |
| Hor Total Time    | = 7.501;                                        | // (usec) | = 4176     | chars =   | 4176 Pixels |
| Hor Addr Time     | = 7.357;                                        | // (usec) | = 4096     | chars =   | 4096 Pixels |
Hor Blank Start = 7.357; // (usec) = 4096 chars = 4096 Pixels
| Hor Blank Time    | = 0.144; | // (usec) | =      | 80 chars = | 80 Pixels   |
| ----------------- | -------- | --------- | ------ | ---------- | ----------- |
| Hor Sync Start    | = 7.371; | // (usec) | = 4104 | chars =    | 4104 Pixels |
| // H Right Border | = 0.000; | // (usec) | =      | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.014; | // (usec) | =      | 8 chars =  | 8 Pixels    |
| Hor Sync Time     | = 0.057; | // (usec) | =      | 32 chars = | 32 Pixels   |
| // H Back Porch   | = 0.072; | // (usec) | =      | 40 chars = | 40 Pixels   |
| // H Left Border  | = 0.000; | // (usec) | =      | 0 chars =  | 0 Pixels    |
Ver Total Time = 16.667; // (msec) = 2222 lines HT – (1.06xHA)
| Ver Addr Time      | = 16.202; | // (msec) | = 2160 | lines    | = -0.3 |
| ------------------ | --------- | --------- | ------ | -------- | ------ |
| Ver Blank Start    | = 16.202; | // (msec) | = 2160 | lines    |        |
| Ver Blank Time     | = 0.465;  | // (msec) | =      | 62 lines |        |
| Ver Sync Start     | = 16.562; | // (msec) | = 2208 | lines    |        |
| // V Bottom Border | = 0.000;  | // (msec) | =      | 0 lines  |        |
| // V Front Porch   | = 0.360;  | // (msec) | =      | 48 lines |        |
| Ver Sync Time      | = 0.060;  | // (msec) | =      | 8 lines  |        |
| // V Back Porch    | = 0.045;  | // (msec) | =      | 6 lines  |        |
| // V Top Border    | = 0.000;  | // (msec) | =      | 0 lines  |        |

Definition of Terms: Refer to section 3.2.
VESA MONITOR TIMING STANDARD
Proposed:  2/8/13
Resolution:  4096 x 2160 at 59.94 Hz (non-interlaced) REDUCED BLANKING v2
EDID ID:  DMT ID: 58h; Std. 2 Byte Code: n/a; CVT 3 Byte Code: n/a
Method:  Generated using CVT (Reduced Blanking v2) Formula

Detailed Timing Parameters
Timing Name = 4096 x 2160 @ 59.94 Hz CVT (Reduced Blanking v2);
| Hor Pixels        | = 4096;          | // Pixels |            |           |             |
| ----------------- | ---------------- | --------- | ---------- | --------- | ----------- |
| Ver Pixels        | = 2160;          | // Lines  |            |           |             |
| Hor Frequency     | = 133.187;       | // kHz    | = 7.5      | usec /    | line        |
| Ver Frequency     | = 59.940;        | // Hz     | = 16.7     | msec /    | frame       |
| Pixel Clock       | = 556.188;       | // MHz    | = 1.8      | nsec      | ± 0.5%      |
| Character Width   | = 1;             | // Pixels | = 1.8      | nsec      |             |
| Scan Type         | = NONINTERLACED; |           | // H Phase | =         | 0.4 %       |
| Hor Sync Polarity | = POSITIVE;      | // HBlank | = 1.9%     | of HTotal |             |
| Ver Sync Polarity | = NEGATIVE       | // VBlank | = 2.8%     | of VTotal |             |
| Hor Total Time    | = 7.508;         | // (usec) | = 4176     | chars =   | 4176 Pixels |
| Hor Addr Time     | = 7.364;         | // (usec) | = 4096     | chars =   | 4096 Pixels |
Hor Blank Start = 7.364; // (usec) = 4096 chars = 4096 Pixels
| Hor Blank Time    | = 0.144; | // (usec) | =      | 80 chars = | 80 Pixels   |
| ----------------- | -------- | --------- | ------ | ---------- | ----------- |
| Hor Sync Start    | = 7.379; | // (usec) | = 4104 | chars =    | 4104 Pixels |
| // H Right Border | = 0.000; | // (usec) | =      | 0 chars =  | 0 Pixels    |
| // H Front Porch  | = 0.014; | // (usec) | =      | 8 chars =  | 8 Pixels    |
| Hor Sync Time     | = 0.058; | // (usec) | =      | 32 chars = | 32 Pixels   |
| // H Back Porch   | = 0.072; | // (usec) | =      | 40 chars = | 40 Pixels   |
| // H Left Border  | = 0.000; | // (usec) | =      | 0 chars =  | 0 Pixels    |
Ver Total Time = 16.683; // (msec) = 2222 lines HT – (1.06xHA)
| Ver Addr Time      | = 16.218; | // (msec) | = 2160 | lines    | = -0.3 |
| ------------------ | --------- | --------- | ------ | -------- | ------ |
| Ver Blank Start    | = 16.218; | // (msec) | = 2160 | lines    |        |
| Ver Blank Time     | = 0.466;  | // (msec) | =      | 62 lines |        |
| Ver Sync Start     | = 16.578; | // (msec) | = 2208 | lines    |        |
| // V Bottom Border | = 0.000;  | // (msec) | =      | 0 lines  |        |
| // V Front Porch   | = 0.360;  | // (msec) | =      | 48 lines |        |
| Ver Sync Time      | = 0.060;  | // (msec) | =      | 8 lines  |        |
| // V Back Porch    | = 0.045;  | // (msec) | =      | 6 lines  |        |
| // V Top Border    | = 0.000;  | // (msec) | =      | 0 lines  |        |

Definition of Terms: Refer to section 3.2.

