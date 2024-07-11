
# Credits

TomHarte for [SingleStepTests](https://github.com/SingleStepTests).
krom(Peter Lemon) for the test roms found at [SNES](https://github.com/PeterLemon/SNES)

# Status

## Ricoh 5A22

- W65C816
	- [x] All instructions implemented
	- [ ] Cycle accurate
	- [x] Handling interrupts
- DMA unit
	- [x] DMA
	- [ ] HDMA
- Math unit
	- [x] Multiplication
	- [x] Division
	- [ ] Cycle accurate

## PPU

- Components
	- [x] Cgram
	- [x] Oam
	- [x] Vram
	- [x] H/V counters
- Graphics format
	- [x] 2BPP
	- [ ] 4BPP
	- [ ] 8BPP
- Modes
	- [x] Mode 0
	- [x] Mode 1
	- [x] Mode 2
	- [x] Mode 3
	- [x] Mode 4
	- [ ] Mode 5
	- [ ] Mode 6
	- [ ] Mode 7
- Rendering
	- [x] Background
	- [ ] Sprite
	- [x] TileMap 32X32
	- [ ] TileMap 32X64
	- [ ] TileMap 64X32
	- [ ] TileMap 64X64
- Advanced Modes
	- [ ] Direct Color
	- [ ] High-res
	- [ ] Interlace
	- [ ] Overscan
	- [ ] Offset-per-tile
- Graphics effects
	- [ ] Color math
	- [ ] Mosaic
	- [ ] Window

## APU

- DSP
- SPC-700

## Cartridge

- Mapper
	- [x] LoROM
	- [ ] HiROM
	- [ ] SA1ROM
	- [ ] SDD1ROM
	- [ ] ExHiROM
- Coprocessor
	- [ ] Todo
- [x] NTSC/PAL support
