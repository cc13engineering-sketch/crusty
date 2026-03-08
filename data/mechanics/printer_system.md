# Pokemon Crystal -- GB Printer System Reference

Source: pokecrystal engine/printer/ (3 files, ~500 lines)

---

## Overview

Pokemon Crystal supports the Game Boy Printer accessory for printing Pokedex entries and PC box contents. Communication uses the serial port with a custom protocol.

---

## Printer Operations (printer.asm)

### SendScreenToPrinter
Main print loop:
1. Check joypad for cancel input
2. Run PrinterJumptableIteration (state machine for serial transfer)
3. CheckPrinterStatus (check for errors)
4. PlacePrinterStatusString (display status to user)
5. Repeat until JUMPTABLE_EXIT_F set or cancelled

Returns carry if cancelled, no carry if completed.

### PrintDexEntry
Prints a Pokedex entry in two pages:
1. Load inverted font tiles
2. Play printer music (MUSIC_PRINTER)
3. Enable serial interrupts (IE_SERIAL | IE_VBLANK)
4. **Page 1**: PrintPage1, margins 1/0, queue length 8 (16/2)
5. 12-frame delay between pages
6. **Page 2**: PrintPage2, margins 0/3, queue length 4 (8/2)
7. Clean up: restore VBlank handler, interrupts, exit printer, low volume delay

### PrintPCBox
Prints a PC box contents in four pages:
1. Similar setup to PrintDexEntry
2. **Page 1**: PrintPCBox_Page1, margins 1/0
3. **Page 2**: PrintPCBox_Page2, margins 0/0
4. **Page 3**: PrintPCBox_Page3, margins 0/0
5. **Page 4**: PrintPCBox_Page4, margins 0/3
6. 12-frame delays between each page
7. Any page can be cancelled (jumps to cleanup)

### Printer Margins
wPrinterMargins encodes top and bottom margins as high/low nybbles:
- High nybble: top margin (feed lines before print)
- Low nybble: bottom margin (feed lines after print)
- Typical values: $10 (top=1, bottom=0), $03 (top=0, bottom=3)

---

## Printer Serial Protocol (printer_serial.asm)

### Communication Protocol
The GB Printer protocol uses synchronous serial communication:
1. **Magic bytes**: $88, $33 (printer handshake)
2. **Command byte**: Tells printer what to do
3. **Data packets**: Tilemap data compressed for printer format
4. **Checksum**: Validates data integrity
5. **Status byte**: Printer reports its state

### Printer Commands
| Command | Description |
|---------|-------------|
| PRINTER_INIT | Initialize printer |
| PRINTER_PRINT | Start printing |
| PRINTER_DATA | Send tile data |
| PRINTER_NUL | No operation / status check |

### State Machine
PrinterJumptableIteration runs a state machine (wJumptableIndex):
- States handle: init handshake, send data packets, wait for print, check status
- Each state advances via serial interrupt handler

### Status Reporting
CheckPrinterStatus monitors printer responses:
- Paper jam, low battery, communication error
- PlacePrinterStatusString displays human-readable status

---

## Print Party (print_party.asm)

Prints the player's party Pokemon as a formatted list for the GB Printer. Separate from the box and Pokedex printing paths.

---

## Key Variables

| Variable | Description |
|----------|-------------|
| wJumptableIndex | Printer state machine state |
| wPrinterConnectionOpen | Whether serial connection is active |
| wPrinterOpcode | Current printer command |
| wPrinterMargins | Top/bottom margin values |
| wPrinterQueueLength | Number of data packets to send |
| wPrinterStatusFlags | Printer status byte |
| hPrinter | Printer mode flag in HRAM |
| wFinishedPrintingBox | Flag for box print completion |
| wAddrOfBoxToPrint | Address of box data |
| wBankOfBoxToPrint | Bank of box data |
| wWhichBoxToPrint | Box index |

---

## Audio

The printer system uses MUSIC_PRINTER (special music track $5B) during printing operations. Volume is lowered after printing completes (LowVolume called for 8 frames).

---

## VBlank Mode

During printing, VBlank handler is switched to VBLANK_SERIAL mode to prioritize serial communication timing over normal game VBlank processing.
