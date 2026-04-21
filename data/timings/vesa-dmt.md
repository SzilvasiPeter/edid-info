# VESA and Industry Standards and Guidelines for Computer Display Monitor Timing (DMT)

Version 1.0, Rev. 13

February 8, 2013

This document includes all current VESA Monitor Timing Standards & Guidelines. Guidelines are subjected to the same VESA review and approval process as Standards, but are designated as Guidelines to ease concerns on the part of some members that VESA is 'endorsing' these timing standards. Guideline designations are typically used for lower resolutions or lower refresh rates that are in common industry use in lower-performance systems. For reference, this document also includes a number of industry-standard timings (de-facto standards) for the computer industry.

This document is the primary means of distribution for all VESA Monitor Timing Standards and Guidelines. The standards and guidelines covered by this document are outlined on the following page.

## Table of Contents

- Intellectual Property
- Trademarks
- Patents
- Support
- DMT Standards and Guidelines Summary
- DMT Standard Codes & IDs Summary
- DMT Timing Specifications


## Tables

- Table 1-1: Summary of Display Monitor Timings – Standards and Guidelines
- Table 2-1: Summary of DMT ID, Std. 2 Byte & CVT 3 Byte Codes

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

- Version 1.0 Revision 0.0 Sept. 12, 1994 Initial Release of the Standard
- Version 1.0 Revision 0.1 Oct. 10, 1994 Fixed sync polarity of 1024x768 @ 60 & 70 Hz. Removed page numbers so new timings could be added.
- Version 1.0 Revision 0.2 Nov. 4, 1994 Added notes & comments to clarify timing of interlaced modes.
- Version 1.0 Revision 0.3 Feb. 16, 1995 Fixed miscellaneous typos
- Version 1.0 Revision 0.4 May 4, 1995 Added EDID IDs for DDC, fixed 1024x768 interlace vertical times.
- Version 1.0 Revision 0.5 June 14, 1995 Added BIOS mode #s, fixed miscellaneous typos
- Version 1.0 Revision 0.6 April 10, 1996 Added new modes from VDMTPROP V1.0, R0.6 passed in March 1996 (85 Hz stds, 1152x864@75, 1280x960@60).
- Version 1.0 Revision 0.6a Sept. 8, 1996 Reformatted to Word 6 for electronic distribution
- Version 1.0 Revision 0.7 Dec. 18, 1996 Added new modes from VDMTREV V1.0, R0.8 passed in Dec. 1996 (1280x1024@60, 1600x1200@60, 65, 70, 75, 85)
- Version 1.0 Revision 0.8 July 22, 1998 Added 1792x1344, 1856X1392 & 1920x1440 all @60, 75 Hz. Corrected EDID code for 1600x1200@85 Hz.
- Version 1.0 Revision 0.9 Aug. 21, 2003 Added 848x480@60 Hz, CVT 1280x768 timings, 1360x768@60 Hz, CVT 1400x1050 timings, & CVT 1920x1200 timings based on US & Japan workgroup requests.
- Version 1.0 Revision 10 July 14, 2004 Added CVT 1.30MA (1440x900) & CVT 1.76MA (1680x1050) formats.
- Version 1.0 Revision 11 May 1, 2007 Added several DMT CVT Reduced Blanking Timings, 1280x800@60/75/85 Hz timings, 2560x1600@60/75/85 Hz and DMT IDs.
- Version 1.0 Revision 12 Nov. 17, 2008 Added timing definitions for 1280x720 @ 60Hz, 1366x768 @ 60 Hz (Normal & Reduced Blanking), 1600x900 @ 60 Hz (Reduced Blanking), 1920x1080 @ 60 Hz and 2048x1152 @60 Hz (Reduced Blanking). Updated Tables 1-1 and 2-1.
- Version 1.0 Revision 13 Feb. 8, 2013 Added timing definitions for 4096x2160 @ 60Hz (Reduced Blanking v2) and4096x2160 @ 59.94Hz (Reduced Blanking v2). Updated Tables 1-1 and 2-1.

# DMT Standards and Guidelines Summary
 
Table 1-1 contains a summary of display monitor timings (DMT) that are defined in this standard. All DMTs listed in Table 1-1 are non-interlaced video timing modes, unless otherwise specified using the symbol “(Int.)”. The symbol “(Int.)” means that this DMT is interlaced. All DMTs listed in Table 1-1 include normal video blanking, unless otherwise specified using the symbol “(RB)”. The symbol “(RB)” means that this DMT includes Reduced Blanking. Complete timing specifications for these DMTs are defined in Section 4.

*Table 1-1: Summary of Display Monitor Timings – Standards and Guidelines*

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

# DMT Standard Codes & ID Summary

Table 2-1 includes a list of DMT ID codes, Standard (Std.) Timing 2 byte codes and Coordinated Video Timing (CVT) 3 byte codes. A display may use these codes to indicate support for the associated DMT. Refer to the latest version of VESA’s Enhanced Extended Display Identification (E-EDID) Standard for an explanation of how to derive the Std. byte codes and the CVT 3 byte codes. The letters “n/a” (not applicable) indicates that a Std. 2 byte code and/or a CVT 3 byte code (DMT is not CVT-compliant) cannot be created.

*Table 2-1:  Summary of DMT ID, Std. 2 Byte & CVT 3 Byte Codes*

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

# DMT Timing Specifications

Section 4 includes a list of detailed timing parameters for all DMTs defined in this standard.

| Timing Name | DMT ID | Std 2-Byte Code | CVT 3-Byte Code | RB | Method | PCLK (MHz) | H-Freq (kHz) | V-Freq (Hz) | Scan | H-Pol | V-Pol | H-Total (px) | H-Addr (px) | H-FP (px) | H-Sync (px) | H-BP (px) | V-Total (ln) | V-Addr (ln) | V-FP (ln) | V-Sync (ln) | V-BP (ln) | H-Left-Border (px) | H-Right-Border (px) | V-Top-Border (ln) | V-Bottom-Border (ln) |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| 640 x 350 @ 85Hz | 01h | n/a | n/a | false | *** NOT CVT COMPLIANT *** | 31.500 | 37.861 | 85.080 | NONINTERLACED | POSITIVE | NEGATIVE | 832 | 640 | 32 | 64 | 96 | 445 | 350 | 32 | 3 | 60 | 0 | 0 | 0 | 0 |
| 640 x 400 @ 85Hz | 02h | (31, 19)h | n/a | false | *** NOT CVT COMPLIANT *** | 31.500 | 37.861 | 85.080 | NONINTERLACED | NEGATIVE | POSITIVE | 832 | 640 | 32 | 64 | 96 | 445 | 400 | 1 | 3 | 41 | 0 | 0 | 0 | 0 |
| 720 x 400 @ 85Hz | 03h | n/a | n/a | false | *** NOT CVT COMPLIANT *** | 35.500 | 37.927 | 85.039 | NONINTERLACED | NEGATIVE | POSITIVE | 936 | 720 | 36 | 72 | 108 | 446 | 400 | 1 | 3 | 42 | 0 | 0 | 0 | 0 |
| 640 x 480 @ 60Hz | 04h | (31, 40)h | n/a | false | *** NOT CVT COMPLIANT *** | 25.175 | 31.469 | 59.940 | NONINTERLACED | NEGATIVE | NEGATIVE | 800 | 640 | 8 | 96 | 40 | 525 | 480 | 2 | 2 | 25 | 8 | 8 | 8 | 8 |
| 640 x 480 @ 72Hz | 05h | (31, 4C)h | n/a | false | *** NOT CVT COMPLIANT *** | 31.500 | 37.861 | 72.809 | NONINTERLACED | NEGATIVE | NEGATIVE | 832 | 640 | 16 | 40 | 120 | 520 | 480 | 1 | 3 | 20 | 8 | 8 | 8 | 8 |
| 640 x 480 @ 75Hz | 06h | (31, 4F)h | n/a | false | *** NOT CVT COMPLIANT *** | 31.500 | 37.500 | 75.000 | NONINTERLACED | NEGATIVE | NEGATIVE | 840 | 640 | 16 | 64 | 120 | 500 | 480 | 1 | 3 | 16 | 0 | 0 | 0 | 0 |
| 640 x 480 @ 85Hz | 07h | (31, 59)h | n/a | false | *** NOT CVT COMPLIANT *** | 36.000 | 43.269 | 85.008 | NONINTERLACED | NEGATIVE | NEGATIVE | 832 | 640 | 56 | 56 | 80 | 509 | 480 | 1 | 3 | 25 | 0 | 0 | 0 | 0 |
| 800 x 600 @ 56Hz | 08h | n/a | n/a | false | *** NOT CVT COMPLIANT *** | 36.000 | 35.156 | 56.250 | NONINTERLACED | POSITIVE | POSITIVE | 1024 | 800 | 24 | 72 | 128 | 625 | 600 | 1 | 2 | 22 | 0 | 0 | 0 | 0 |
| 800 x 600 @ 60Hz | 09h | (45, 40)h | n/a | false | *** NOT CVT COMPLIANT *** | 40.000 | 37.879 | 60.317 | NONINTERLACED | POSITIVE | POSITIVE | 1056 | 800 | 40 | 128 | 88 | 628 | 600 | 1 | 4 | 23 | 0 | 0 | 0 | 0 |
| 800 x 600 @ 72Hz | 0Ah | (45, 4C)h | n/a | false | *** NOT CVT COMPLIANT *** | 50.000 | 48.077 | 72.188 | NONINTERLACED | POSITIVE | POSITIVE | 1040 | 800 | 56 | 120 | 64 | 666 | 600 | 37 | 6 | 23 | 0 | 0 | 0 | 0 |
| 800 x 600 @ 75Hz | 0Bh | (45, 4F)h | n/a | false | *** NOT CVT COMPLIANT *** | 49.500 | 46.875 | 75.000 | NONINTERLACED | POSITIVE | POSITIVE | 1056 | 800 | 16 | 80 | 160 | 625 | 600 | 1 | 3 | 21 | 0 | 0 | 0 | 0 |
| 800 x 600 @ 85Hz | 0Ch | (45, 59)h | n/a | false | *** NOT CVT COMPLIANT *** | 56.250 | 53.674 | 85.061 | NONINTERLACED | POSITIVE | POSITIVE | 1048 | 800 | 32 | 64 | 152 | 631 | 600 | 1 | 3 | 27 | 0 | 0 | 0 | 0 |
| 800 x 600 @ 120Hz CVT (Reduced Blanking) | 0Dh | n/a | n/a | true | Generated using CVT (Reduced Blanking) Formula | 73.250 | 76.302 | 119.972 | NONINTERLACED | POSITIVE | NEGATIVE | 960 | 800 | 48 | 32 | 80 | 636 | 600 | 3 | 4 | 29 | 0 | 0 | 0 | 0 |
| 848 x 480 @ 60Hz | 0Eh | n/a | n/a | false | *** NOT CVT COMPLIANT *** | 33.750 | 31.020 | 60.000 | NONINTERLACED | POSITIVE | POSITIVE | 1088 | 848 | 16 | 112 | 112 | 517 | 480 | 6 | 8 | 23 | 0 | 0 | 0 | 0 |
| 1024 x 768 @ 43Hz (Interlaced) | 0Fh | n/a | n/a | false | *** NOT CVT COMPLIANT *** | 44.900 | 35.522 | 86.957 | INTERLACED | POSITIVE | POSITIVE | 1264 | 1024 | 8 | 176 | 56 | 817 | 768 | 0 | 4 | 20 | 0 | 0 | 0 | 0 |
| 1024 x 768 @ 60Hz | 10h | (61, 40)h | n/a | false | *** NOT CVT COMPLIANT *** | 65.000 | 48.363 | 60.004 | NONINTERLACED | NEGATIVE | NEGATIVE | 1344 | 1024 | 24 | 136 | 160 | 806 | 768 | 3 | 6 | 29 | 0 | 0 | 0 | 0 |
| 1024 x 768 @ 70Hz | 11h | (61, 4A)h | n/a | false | *** NOT CVT COMPLIANT *** | 75.000 | 56.476 | 70.069 | NONINTERLACED | NEGATIVE | NEGATIVE | 1328 | 1024 | 24 | 136 | 144 | 806 | 768 | 3 | 6 | 29 | 0 | 0 | 0 | 0 |
| 1024 x 768 @ 75Hz | 12h | (61, 4F)h | n/a | false | *** NOT CVT COMPLIANT *** | 78.750 | 60.023 | 75.029 | NONINTERLACED | POSITIVE | POSITIVE | 1312 | 1024 | 16 | 96 | 176 | 800 | 768 | 1 | 3 | 28 | 0 | 0 | 0 | 0 |
| 1024 x 768 @ 85Hz | 13h | (61, 59)h | n/a | false | *** NOT CVT COMPLIANT *** | 94.500 | 68.677 | 84.997 | NONINTERLACED | POSITIVE | POSITIVE | 1376 | 1024 | 48 | 96 | 208 | 808 | 768 | 1 | 3 | 36 | 0 | 0 | 0 | 0 |
| 1024 x 768 @ 120Hz CVT (Reduced Blanking) | 14h | n/a | n/a | true | Generated using CVT (Reduced Blanking) Formula | 115.500 | 97.551 | 119.989 | NONINTERLACED | POSITIVE | NEGATIVE | 1184 | 1024 | 48 | 32 | 80 | 813 | 768 | 3 | 4 | 38 | 0 | 0 | 0 | 0 |
| 1152 x 864 @ 75Hz | 15h | (71, 4F)h | n/a | false | *** NOT CVT COMPLIANT *** | 108.000 | 67.500 | 75.000 | NONINTERLACED | POSITIVE | POSITIVE | 1600 | 1152 | 64 | 128 | 256 | 900 | 864 | 1 | 3 | 32 | 0 | 0 | 0 | 0 |
| 1280 x 720 @ 60Hz | 55h | 81h, C0h | n/a | false | *** NOT CVT COMPLIANT *** | 74.250 | 45.000 | 60.000 | NONINTERLACED | POSITIVE | POSITIVE | 1650 | 1280 | 110 | 40 | 220 | 750 | 720 | 5 | 5 | 20 | 0 | 0 | 0 | 0 |
| 1280 x 768 @ 60Hz CVT (Reduced Blanking) | 16h | n/a | (7F, 1C, 21)h | true | CVT Reduced Blanking | 68.250 | 47.396 | 59.995 | NONINTERLACED | POSITIVE | NEGATIVE | 1440 | 1280 | 48 | 32 | 80 | 790 | 768 | 3 | 7 | 12 | 0 | 0 | 0 | 0 |
| 1280 x 768 @ 60Hz | 17h | n/a | (7F, 1C, 28)h | false | CVT Compliant | 79.500 | 47.776 | 59.870 | NONINTERLACED | NEGATIVE | POSITIVE | 1664 | 1280 | 64 | 128 | 192 | 798 | 768 | 3 | 7 | 20 | 0 | 0 | 0 | 0 |
| 1280 x 768 @ 75Hz | 18h | n/a | (7F, 1C, 44)h | false | CVT Compliant | 102.250 | 60.289 | 74.893 | NONINTERLACED | NEGATIVE | POSITIVE | 1696 | 1280 | 80 | 128 | 208 | 805 | 768 | 3 | 7 | 27 | 0 | 0 | 0 | 0 |
| 1280 x 768 @ 85Hz | 19h | n/a | (7F, 1C, 62)h | false | CVT Compliant | 117.500 | 68.633 | 84.837 | NONINTERLACED | NEGATIVE | POSITIVE | 1712 | 1280 | 80 | 136 | 216 | 809 | 768 | 3 | 7 | 31 | 0 | 0 | 0 | 0 |
| 1280 x 768 @ 120Hz CVT (Reduced Blanking) | 1Ah | n/a | n/a | true | Generated using CVT (Reduced Blanking) Formula | 140.250 | 97.396 | 119.798 | NONINTERLACED | POSITIVE | NEGATIVE | 1440 | 1280 | 48 | 32 | 80 | 813 | 768 | 3 | 7 | 35 | 0 | 0 | 0 | 0 |
| 1280 x 800 @ 60Hz CVT (Reduced Blanking) | 1Bh | n/a | (8F, 18, 21)h | true | CVT Reduced Blanking | 71.000 | 49.306 | 59.910 | NONINTERLACED | POSITIVE | NEGATIVE | 1440 | 1280 | 48 | 32 | 80 | 823 | 800 | 3 | 6 | 14 | 0 | 0 | 0 | 0 |
| 1280 x 800 @ 60Hz | 1Ch | (81, 00)h | (8F, 18, 28)h | false | CVT Compliant | 83.500 | 49.702 | 59.810 | NONINTERLACED | NEGATIVE | POSITIVE | 1680 | 1280 | 72 | 128 | 200 | 831 | 800 | 3 | 6 | 22 | 0 | 0 | 0 | 0 |
| 1280 x 800 @ 75Hz | 1Dh | (81, 0F)h | (8F, 18, 44)h | false | CVT Compliant | 106.500 | 62.795 | 74.934 | NONINTERLACED | NEGATIVE | POSITIVE | 1696 | 1280 | 80 | 128 | 208 | 838 | 800 | 3 | 6 | 29 | 0 | 0 | 0 | 0 |
| 1280 x 800 @ 85Hz | 1Eh | (81, 19)h | (8F, 18, 62)h | false | CVT Compliant | 122.500 | 71.554 | 84.880 | NONINTERLACED | NEGATIVE | POSITIVE | 1712 | 1280 | 80 | 136 | 216 | 843 | 800 | 3 | 6 | 34 | 0 | 0 | 0 | 0 |
| 1280 x 800 @ 120Hz CVT (Reduced Blanking) | 1Fh | n/a | n/a | true | Generated using CVT (Reduced Blanking) Formula | 146.250 | 101.563 | 119.909 | NONINTERLACED | POSITIVE | NEGATIVE | 1440 | 1280 | 48 | 32 | 80 | 847 | 800 | 3 | 6 | 38 | 0 | 0 | 0 | 0 |
| 1280 x 960 @ 60Hz | 20h | (81, 40)h | n/a | false | *** NOT CVT COMPLIANT *** | 108.000 | 60.000 | 60.000 | NONINTERLACED | POSITIVE | POSITIVE | 1800 | 1280 | 96 | 112 | 312 | 1000 | 960 | 1 | 3 | 36 | 0 | 0 | 0 | 0 |
| 1280 x 960 @ 85Hz | 21h | (81, 59)h | n/a | false | *** NOT CVT COMPLIANT *** | 148.500 | 85.938 | 85.002 | NONINTERLACED | POSITIVE | POSITIVE | 1728 | 1280 | 64 | 160 | 224 | 1011 | 960 | 1 | 3 | 47 | 0 | 0 | 0 | 0 |
| 1280 x 960 @ 120Hz CVT (Reduced Blanking) | 22h | n/a | n/a | true | Generated using CVT (Reduced Blanking) Formula | 175.500 | 121.875 | 119.838 | NONINTERLACED | POSITIVE | NEGATIVE | 1440 | 1280 | 48 | 32 | 80 | 1017 | 960 | 3 | 4 | 50 | 0 | 0 | 0 | 0 |
| 1280 x 1024 @ 60Hz | 23h | (81, 80)h | n/a | false | *** NOT CVT COMPLIANT *** | 108.000 | 63.981 | 60.020 | NONINTERLACED | POSITIVE | POSITIVE | 1688 | 1280 | 48 | 112 | 248 | 1066 | 1024 | 1 | 3 | 38 | 0 | 0 | 0 | 0 |
| 1280 x 1024 @ 75Hz | 24h | (81, 8F)h | n/a | false | *** NOT CVT COMPLIANT *** | 135.000 | 79.976 | 75.025 | NONINTERLACED | POSITIVE | POSITIVE | 1688 | 1280 | 16 | 144 | 248 | 1066 | 1024 | 1 | 3 | 38 | 0 | 0 | 0 | 0 |
| 1280 x 1024 @ 85Hz | 25h | (81, 99)h | n/a | false | *** NOT CVT COMPLIANT *** | 157.500 | 91.146 | 85.024 | NONINTERLACED | POSITIVE | POSITIVE | 1728 | 1280 | 64 | 160 | 224 | 1072 | 1024 | 1 | 3 | 44 | 0 | 0 | 0 | 0 |
| 1280 x 1024 @ 120Hz CVT (Reduced Blanking) | 26h | n/a | n/a | true | Generated using CVT (Reduced Blanking) Formula | 187.250 | 130.035 | 119.958 | NONINTERLACED | POSITIVE | NEGATIVE | 1440 | 1280 | 48 | 32 | 80 | 1084 | 1024 | 3 | 7 | 50 | 0 | 0 | 0 | 0 |
| 1360 x 768 @ 60Hz | 27h | n/a | n/a | false | *** NOT CVT COMPLIANT *** | 85.500 | 47.712 | 60.015 | NONINTERLACED | POSITIVE | POSITIVE | 1792 | 1360 | 64 | 112 | 256 | 795 | 768 | 3 | 6 | 18 | 0 | 0 | 0 | 0 |
| 1360 x 768 @ 120Hz CVT (Reduced Blanking) | 28h | n/a | n/a | true | Generated using CVT (Reduced Blanking) Formula | 148.250 | 97.533 | 119.967 | NONINTERLACED | POSITIVE | NEGATIVE | 1520 | 1360 | 48 | 32 | 80 | 813 | 768 | 3 | 5 | 37 | 0 | 0 | 0 | 0 |
| 1366 x 768 @ 60Hz | 51h | n/a | n/a | false | *** NOT CVT COMPLIANT *** | 85.500 | 47.712 | 59.790 | NONINTERLACED | POSITIVE | POSITIVE | 1792 | 1366 | 70 | 143 | 213 | 798 | 768 | 3 | 3 | 24 | 0 | 0 | 0 | 0 |
| 1366 x 768 @ 60Hz | 56h | n/a | n/a | true | *** NOT CVT COMPLIANT *** | 72.000 | 48.000 | 60.000 | NONINTERLACED | POSITIVE | POSITIVE | 1500 | 1366 | 14 | 56 | 64 | 800 | 768 | 1 | 3 | 28 | 0 | 0 | 0 | 0 |
| 1400 x 1050 @ 60Hz CVT (Reduced Blanking) | 29h | n/a | (0C, 20, 21)h | true | CVT Reduced Blanking | 101.000 | 64.744 | 59.948 | NONINTERLACED | POSITIVE | NEGATIVE | 1560 | 1400 | 48 | 32 | 80 | 1080 | 1050 | 3 | 4 | 23 | 0 | 0 | 0 | 0 |
| 1400 x 1050 @ 60Hz | 2Ah | (90, 40)h | (0C, 20, 28)h | false | CVT Compliant | 121.750 | 65.317 | 59.978 | NONINTERLACED | NEGATIVE | POSITIVE | 1864 | 1400 | 88 | 144 | 232 | 1089 | 1050 | 3 | 4 | 32 | 0 | 0 | 0 | 0 |
| 1400 x 1050 @ 75Hz | 2Bh | (90, 4F)h | (0C, 20, 44)h | false | CVT Compliant | 156.000 | 82.278 | 74.867 | NONINTERLACED | NEGATIVE | POSITIVE | 1896 | 1400 | 104 | 144 | 248 | 1099 | 1050 | 3 | 4 | 42 | 0 | 0 | 0 | 0 |
| 1400 x 1050 @ 85Hz | 2Ch | (90, 59)h | (0C, 20, 62)h | false | CVT Compliant | 179.500 | 93.881 | 84.960 | NONINTERLACED | NEGATIVE | POSITIVE | 1912 | 1400 | 104 | 152 | 256 | 1105 | 1050 | 3 | 4 | 48 | 0 | 0 | 0 | 0 |
| 1400 x 1050 @ 120Hz CVT (Reduced Blanking) | 2Dh | n/a | n/a | true | Generated using CVT (Reduced Blanking) Formula | 208.000 | 133.333 | 119.904 | NONINTERLACED | POSITIVE | NEGATIVE | 1560 | 1400 | 48 | 32 | 80 | 1112 | 1050 | 3 | 4 | 55 | 0 | 0 | 0 | 0 |
| 1440 x 900 @ 60Hz CVT (Reduced Blanking) | 2Eh | n/a | (C1, 18, 21)h | true | CVT Reduced Blanking | 88.750 | 55.469 | 59.901 | NONINTERLACED | POSITIVE | NEGATIVE | 1600 | 1440 | 48 | 32 | 80 | 926 | 900 | 3 | 6 | 17 | 0 | 0 | 0 | 0 |
| 1440 x 900 @ 60Hz | 2Fh | (95, 00)h | (C1, 18, 28)h | false | CVT Compliant | 106.500 | 55.935 | 59.887 | NONINTERLACED | NEGATIVE | POSITIVE | 1904 | 1440 | 80 | 152 | 232 | 934 | 900 | 3 | 6 | 25 | 0 | 0 | 0 | 0 |
| 1440 x 900 @ 75Hz | 30h | (95, 0F)h | (C1, 18, 44)h | false | CVT Compliant | 136.750 | 70.635 | 74.984 | NONINTERLACED | NEGATIVE | POSITIVE | 1936 | 1440 | 96 | 152 | 248 | 942 | 900 | 3 | 6 | 33 | 0 | 0 | 0 | 0 |
| 1440 x 900 @ 85Hz | 31h | (95, 19)h | (C1, 18, 68)h | false | CVT Compliant | 157.000 | 80.430 | 84.842 | NONINTERLACED | NEGATIVE | POSITIVE | 1952 | 1440 | 104 | 152 | 256 | 948 | 900 | 3 | 6 | 39 | 0 | 0 | 0 | 0 |
| 1440 x 900 @ 120Hz CVT (Reduced Blanking) | 32h | n/a | n/a | true | Generated using CVT (Reduced Blanking) Formula | 182.750 | 114.219 | 119.852 | NONINTERLACED | POSITIVE | NEGATIVE | 1600 | 1440 | 48 | 32 | 80 | 953 | 900 | 3 | 6 | 44 | 0 | 0 | 0 | 0 |
| 1600 x 900 @ 60Hz | 53h | A9h, C0h | n/a | true | *** NOT CVT COMPLIANT *** | 108.000 | 60.000 | 60.000 | NONINTERLACED | POSITIVE | POSITIVE | 1800 | 1600 | 24 | 80 | 96 | 1000 | 900 | 1 | 3 | 96 | 0 | 0 | 0 | 0 |
| 1600 x 1200 @ 60Hz | 33h | (A9, 40)h | n/a | false | *** NOT CVT COMPLIANT *** | 162.000 | 75.000 | 60.000 | NONINTERLACED | POSITIVE | POSITIVE | 2160 | 1600 | 64 | 192 | 304 | 1250 | 1200 | 1 | 3 | 46 | 0 | 0 | 0 | 0 |
| 1600 x 1200 @ 65Hz | 34h | (A9, 45)h | n/a | false | *** NOT CVT COMPLIANT *** | 175.500 | 81.250 | 65.000 | NONINTERLACED | POSITIVE | POSITIVE | 2160 | 1600 | 64 | 192 | 304 | 1250 | 1200 | 1 | 3 | 46 | 0 | 0 | 0 | 0 |
| 1600 x 1200 @ 70Hz | 35h | (A9, 4A)h | n/a | false | *** NOT CVT COMPLIANT *** | 189.000 | 87.500 | 70.000 | NONINTERLACED | POSITIVE | POSITIVE | 2160 | 1600 | 64 | 192 | 304 | 1250 | 1200 | 1 | 3 | 46 | 0 | 0 | 0 | 0 |
| 1600 x 1200 @ 75Hz | 36h | (A9, 4F)h | n/a | false | *** NOT CVT COMPLIANT *** | 202.500 | 93.750 | 75.000 | NONINTERLACED | POSITIVE | POSITIVE | 2160 | 1600 | 64 | 192 | 304 | 1250 | 1200 | 1 | 3 | 46 | 0 | 0 | 0 | 0 |
| 1600 x 1200 @ 85Hz | 37h | (A9, 59)h | n/a | false | *** NOT CVT COMPLIANT *** | 229.500 | 106.250 | 85.000 | NONINTERLACED | POSITIVE | POSITIVE | 2160 | 1600 | 64 | 192 | 304 | 1250 | 1200 | 1 | 3 | 46 | 0 | 0 | 0 | 0 |
| 1600 x 1200 @ 120Hz CVT (Reduced Blanking) | 38h | n/a | n/a | true | Generated using CVT (Reduced Blanking) Formula | 268.250 | 152.415 | 119.917 | NONINTERLACED | POSITIVE | NEGATIVE | 1760 | 1600 | 48 | 32 | 80 | 1271 | 1200 | 3 | 4 | 64 | 0 | 0 | 0 | 0 |
| 1680 x 1050 @ 60Hz CVT (Reduced Blanking) | 39h | n/a | (0C, 28, 21)h | true | CVT Reduced Blanking | 119.000 | 64.674 | 59.883 | NONINTERLACED | POSITIVE | NEGATIVE | 1840 | 1680 | 48 | 32 | 80 | 1080 | 1050 | 3 | 6 | 21 | 0 | 0 | 0 | 0 |
| 1680 x 1050 @ 60Hz | 3Ah | (B3, 00)h | (0C, 28, 28)h | false | CVT Compliant | 146.250 | 65.290 | 59.954 | NONINTERLACED | NEGATIVE | POSITIVE | 2240 | 1680 | 104 | 176 | 280 | 1089 | 1050 | 3 | 6 | 30 | 0 | 0 | 0 | 0 |
| 1680 x 1050 @ 75Hz | 3Bh | (B3, 0F)h | (0C, 28, 44)h | false | CVT Compliant | 187.000 | 82.306 | 74.892 | NONINTERLACED | NEGATIVE | POSITIVE | 2272 | 1680 | 120 | 176 | 296 | 1099 | 1050 | 3 | 6 | 40 | 0 | 0 | 0 | 0 |
| 1680 x 1050 @ 85Hz | 3Ch | (B3, 19)h | (0C, 28, 68)h | false | CVT Compliant | 214.750 | 93.859 | 84.941 | NONINTERLACED | NEGATIVE | POSITIVE | 2288 | 1680 | 128 | 176 | 304 | 1105 | 1050 | 3 | 6 | 46 | 0 | 0 | 0 | 0 |
| 1680 x 1050 @ 120Hz CVT (Reduced Blanking) | 3Dh | n/a | n/a | true | Generated using CVT (Reduced Blanking) Formula | 245.500 | 133.424 | 119.986 | NONINTERLACED | POSITIVE | NEGATIVE | 1840 | 1680 | 48 | 32 | 80 | 1112 | 1050 | 3 | 6 | 53 | 0 | 0 | 0 | 0 |
| 1792 x 1344 @ 60 Hz | 3Eh | (C1, 40)h | n/a | false | *** NOT CVT COMPLIANT *** | 204.750 | 83.640 | 60.000 | NONINTERLACED | NEGATIVE | POSITIVE | 2448 | 1792 | 128 | 200 | 328 | 1394 | 1344 | 1 | 3 | 46 | 0 | 0 | 0 | 0 |
| 1792 x 1344 @ 75Hz | 3Fh | (C1, 4F)h | n/a | false | *** NOT CVT COMPLIANT *** | 261.000 | 106.270 | 74.997 | NONINTERLACED | NEGATIVE | POSITIVE | 2456 | 1792 | 96 | 216 | 352 | 1417 | 1344 | 1 | 3 | 69 | 0 | 0 | 0 | 0 |
| 1792 x 1344 @ 120Hz CVT (Reduced Blanking) | 40h | n/a | n/a | true | Generated using CVT (Reduced Blanking) Formula | 333.250 | 170.722 | 119.974 | NONINTERLACED | POSITIVE | NEGATIVE | 1952 | 1792 | 48 | 32 | 80 | 1423 | 1344 | 3 | 4 | 72 | 0 | 0 | 0 | 0 |
| 1856 x 1392 at 60Hz | 41h | (C9, 40)h | n/a | false | *** NOT CVT COMPLIANT *** | 218.250 | 86.333 | 59.995 | NONINTERLACED | NEGATIVE | POSITIVE | 2528 | 1856 | 96 | 224 | 352 | 1439 | 1392 | 1 | 3 | 43 | 0 | 0 | 0 | 0 |
| 1856 x 1392 @ 75Hz | 42h | (C9, 4F)h | n/a | false | *** NOT CVT COMPLIANT *** | 288.000 | 112.500 | 75.000 | NONINTERLACED | NEGATIVE | POSITIVE | 2560 | 1856 | 128 | 224 | 352 | 1500 | 1392 | 1 | 3 | 104 | 0 | 0 | 0 | 0 |
| 1856 x 1392 @ 120Hz CVT (Reduced Blanking) | 43h | n/a | n/a | true | Generated using CVT (Reduced Blanking) Formula | 356.500 | 176.835 | 119.970 | NONINTERLACED | POSITIVE | NEGATIVE | 2016 | 1856 | 48 | 32 | 80 | 1474 | 1392 | 3 | 4 | 75 | 0 | 0 | 0 | 0 |
| 1920 x 1080 @ 60Hz | 52h | (D1, C0)h | n/a | false | *** NOT CVT COMPLIANT *** | 148.500 | 67.500 | 60.000 | NONINTERLACED | POSITIVE | POSITIVE | 2200 | 1920 | 88 | 44 | 148 | 1125 | 1080 | 4 | 5 | 36 | 0 | 0 | 0 | 0 |
| 1920 x 1200 @ 60Hz CVT (Reduced Blanking) | 44h | n/a | (57, 28, 21)h | true | CVT Reduced Blanking | 154.000 | 74.038 | 59.950 | NONINTERLACED | POSITIVE | NEGATIVE | 2080 | 1920 | 48 | 32 | 80 | 1235 | 1200 | 3 | 6 | 26 | 0 | 0 | 0 | 0 |
| 1920 x 1200 @ 60Hz | 45h | (D1, 00)h | (57, 28, 28)h | false | CVT Compliant | 193.250 | 74.556 | 59.885 | NONINTERLACED | NEGATIVE | POSITIVE | 2592 | 1920 | 136 | 200 | 336 | 1245 | 1200 | 3 | 6 | 36 | 0 | 0 | 0 | 0 |
| 1920 x 1200 @ 75Hz | 46h | (D1, 0F)h | (57, 28, 44)h | false | CVT Compliant | 245.250 | 94.038 | 74.930 | NONINTERLACED | NEGATIVE | POSITIVE | 2608 | 1920 | 136 | 208 | 344 | 1255 | 1200 | 3 | 6 | 46 | 0 | 0 | 0 | 0 |
| 1920 x 1200 @ 85Hz | 47h | (D1, 19)h | (57, 28, 62)h | false | CVT Compliant | 281.250 | 107.184 | 84.932 | NONINTERLACED | NEGATIVE | POSITIVE | 2624 | 1920 | 144 | 208 | 352 | 1262 | 1200 | 3 | 6 | 53 | 0 | 0 | 0 | 0 |
| 1920 x 1200 @ 120Hz CVT (Reduced Blanking) | 48h | n/a | n/a | true | Generated using CVT (Reduced Blanking) Formula | 317.000 | 152.404 | 119.909 | NONINTERLACED | POSITIVE | NEGATIVE | 2080 | 1920 | 48 | 32 | 80 | 1271 | 1200 | 3 | 6 | 62 | 0 | 0 | 0 | 0 |
| 1920 x 1440 @ 60Hz | 49h | (D1, 40)h | n/a | false | *** NOT CVT COMPLIANT *** | 234.000 | 90.000 | 60.000 | NONINTERLACED | NEGATIVE | POSITIVE | 2600 | 1920 | 128 | 208 | 344 | 1500 | 1440 | 1 | 3 | 56 | 0 | 0 | 0 | 0 |
| 1920 x 1440 @ 75Hz | 4Ah | (D1, 4F)h | n/a | false | *** NOT CVT COMPLIANT *** | 297.000 | 112.500 | 75.000 | NONINTERLACED | NEGATIVE | POSITIVE | 2640 | 1920 | 144 | 224 | 352 | 1500 | 1440 | 1 | 3 | 56 | 0 | 0 | 0 | 0 |
| 1920 x 1440 @ 120Hz CVT (Reduced Blanking) | 4Bh | n/a | n/a | true | Generated using CVT (Reduced Blanking) Formula | 380.500 | 182.933 | 119.956 | NONINTERLACED | POSITIVE | NEGATIVE | 2080 | 1920 | 48 | 32 | 80 | 1525 | 1440 | 3 | 4 | 78 | 0 | 0 | 0 | 0 |
| 2048 x 1152 @ 60Hz | 54h | E1h, C0h | n/a | true | *** NOT CVT COMPLIANT *** | 162.000 | 72.000 | 60.000 | NONINTERLACED | POSITIVE | POSITIVE | 2250 | 2048 | 26 | 80 | 96 | 1200 | 1152 | 1 | 3 | 44 | 0 | 0 | 0 | 0 |
| 2560 x 1600 @ 60Hz CVT (Reduced Blanking) | 4Ch | n/a | (1F, 38, 21)h | true | CVT Compliant | 268.500 | 98.713 | 59.972 | NONINTERLACED | POSITIVE | NEGATIVE | 2720 | 2560 | 48 | 32 | 80 | 1646 | 1600 | 3 | 6 | 37 | 0 | 0 | 0 | 0 |
| 2560 x 1600 @ 60Hz | 4Dh | n/a | (1F, 38, 28)h | false | CVT Compliant | 348.500 | 99.458 | 59.987 | NONINTERLACED | NEGATIVE | POSITIVE | 3504 | 2560 | 192 | 280 | 472 | 1658 | 1600 | 3 | 6 | 49 | 0 | 0 | 0 | 0 |
| 2560 x 1600 @ 75Hz | 4Eh | n/a | (1F, 38, 44)h | false | CVT Compliant | 443.250 | 125.354 | 74.972 | NONINTERLACED | NEGATIVE | POSITIVE | 3536 | 2560 | 208 | 280 | 488 | 1672 | 1600 | 3 | 6 | 63 | 0 | 0 | 0 | 0 |
| 2560 x 1600 @ 85Hz | 4Fh | n/a | (1F, 38, 62)h | false | CVT Compliant | 505.250 | 142.887 | 84.951 | NONINTERLACED | NEGATIVE | POSITIVE | 3536 | 2560 | 208 | 280 | 488 | 1682 | 1600 | 3 | 6 | 73 | 0 | 0 | 0 | 0 |
| 2560 x 1600 @ 120Hz CVT (Reduced Blanking) | 50h | n/a | n/a | true | Generated using CVT (Reduced Blanking) Formula | 552.750 | 203.217 | 119.963 | NONINTERLACED | POSITIVE | NEGATIVE | 2720 | 2560 | 48 | 32 | 80 | 1694 | 1600 | 3 | 6 | 85 | 0 | 0 | 0 | 0 |
| 4096 x 2160 @ 60Hz CVT (Reduced Blanking v2) | 57h | n/a | n/a | true | Generated using CVT (Reduced Blanking v2) Formula | 556.744 | 133.320 | 60.000 | NONINTERLACED | POSITIVE | NEGATIVE | 4176 | 4096 | 8 | 32 | 40 | 2222 | 2160 | 48 | 8 | 6 | 0 | 0 | 0 | 0 |
| 4096 x 2160 @ 59.94 Hz CVT (Reduced Blanking v2) | 58h | n/a | n/a | true | Generated using CVT (Reduced Blanking v2) Formula | 556.188 | 133.187 | 59.940 | NONINTERLACED | POSITIVE | NEGATIVE | 4176 | 4096 | 8 | 32 | 40 | 2222 | 2160 | 48 | 8 | 6 | 0 | 0 | 0 | 0 |