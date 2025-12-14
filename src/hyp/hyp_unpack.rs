//! HYP (Hyper) Archive Decompressor
//!
//! Clean-room reconstruction of the original UNPACK.ASM/COMM.ASM routines.
//! The Huffman structures operate on byte offsets exactly like the 16-bit code.

use crate::error::Result;

const STR_IND_BUF_LEN: usize = 8191;
const TAB_SIZE: usize = STR_IND_BUF_LEN + 200;
const HUFF_SIZE: usize = 3200;
const MAX_REC_FREQUENCY: usize = 4096;
const MAX_FREQ: usize = 2;
const MAXMCOUNT: usize = 1350;
// EXE initializes only 4 words at 0xC320 (freq offsets 0,2,4,6).
const NFREQ_TABLE_LEN: usize = 4;

const DEFAULT_MINDIFF: i16 = -4;
const DEFAULT_MAXDIFF: i16 = 8;
const DIFF_OFFSET: i16 = 1;
const LSEQUENCE_KEY: u16 = 0;
const BASE_LASTPOSITION: u16 = 2 * 256;
const MARKED: u16 = 1;

#[inline]
fn word_index(offset: u16) -> usize {
    (offset as usize) >> 1
}

const READ_BITS_MSB: bool = false;
const UPDATE_DYNA_HUFFMAN: bool = true;

struct BitReader<'a> {
    buffer: &'a [u8],
    byte_pos: usize,
    bit_cnt: u8,
    puffer_byte: u8,
}

impl<'a> BitReader<'a> {
    fn new(buffer: &'a [u8]) -> Self {
        Self {
            buffer,
            byte_pos: 0,
            bit_cnt: 8,
            puffer_byte: buffer.get(0).copied().unwrap_or(0),
        }
    }

    /// Read `count` bits (LSB-first) into the low bits of the return value.
    /// This matches UNPACK.ASM `rBits` usage (e.g. `CX=13` for `teststrings_index`).
    #[inline]
    fn read_bits(&mut self, count: u8) -> u16 {
        let mut value = 0u16;
        for bit_index in 0..count {
            if self.read_bit() {
                value |= 1u16 << bit_index;
            }
        }
        value
    }

    #[inline]
    fn read_bit(&mut self) -> bool {
        let bit = if READ_BITS_MSB {
            (self.puffer_byte & 0x80) != 0
        } else {
            (self.puffer_byte & 1) != 0
        };

        if READ_BITS_MSB {
            self.puffer_byte <<= 1;
        } else {
            self.puffer_byte >>= 1;
        }

        self.bit_cnt -= 1;
        if self.bit_cnt == 0 {
            self.byte_pos += 1;
            self.puffer_byte = self.buffer.get(self.byte_pos).copied().unwrap_or(0);
            self.bit_cnt = 8;
        }
        bit
    }

    fn is_exhausted(&self) -> bool {
        self.byte_pos >= self.buffer.len()
    }
}

struct HuffmanState {
    vater: Vec<u16>,
    sohn: Vec<u16>,
    the_freq: Vec<u16>,
    nindex: Vec<u16>,
    nvalue: Vec<u16>,
    frequencys: Vec<u16>,
    nfreq: [u16; NFREQ_TABLE_LEN],
    nfreqmax: Vec<u16>,
    str_ind_buf: Vec<u16>,
    new_index: Vec<u16>,
    teststrings_index: u16,
    low_tsi: u16,
    lastposition: u16,
    huff_max: u16,
    huff_max_index: u16,
    maxlocal: u16,
    maxlocal255: u16,
    local_offset: u16,
    pos_offset: u16,
    char_offset: u16,
    version: u8,
    mindiff: i16,
    maxdiff: i16,
}

impl HuffmanState {
    fn new() -> Self {
        Self {
            vater: vec![0; 2 * HUFF_SIZE + 2],
            sohn: vec![0; 2 * HUFF_SIZE + 2],
            the_freq: vec![0; 2 * HUFF_SIZE + 2],
            nindex: vec![0; STR_IND_BUF_LEN + 2],
            nvalue: vec![0; TAB_SIZE + 2],
            frequencys: vec![0; TAB_SIZE + 2],
            nfreq: [0; NFREQ_TABLE_LEN],
            nfreqmax: vec![0; MAX_REC_FREQUENCY + 1],
            str_ind_buf: vec![0; STR_IND_BUF_LEN + 1],
            new_index: vec![0; STR_IND_BUF_LEN + 1],
            teststrings_index: 255,
            low_tsi: 2 * 255,
            lastposition: BASE_LASTPOSITION,
            huff_max: 2,
            huff_max_index: 2,
            maxlocal: 0,
            maxlocal255: 0,
            local_offset: 0,
            pos_offset: 0,
            char_offset: 0,
            version: 0,
            mindiff: DEFAULT_MINDIFF,
            maxdiff: DEFAULT_MAXDIFF,
        }
    }

    fn read_str(&self, offset: u16) -> u16 {
        self.str_ind_buf
            .get(word_index(offset))
            .copied()
            .unwrap_or(0)
    }

    fn write_str(&mut self, offset: u16, value: u16) {
        if let Some(slot) = self.str_ind_buf.get_mut(word_index(offset)) {
            *slot = value;
        }
    }

    fn get_sohn(&self, offset: u16) -> u16 {
        self.sohn.get(word_index(offset)).copied().unwrap_or(0)
    }

    fn set_sohn(&mut self, offset: u16, value: u16) {
        if let Some(slot) = self.sohn.get_mut(word_index(offset)) {
            *slot = value;
        }
    }

    fn get_vater(&self, offset: u16) -> u16 {
        self.vater.get(word_index(offset)).copied().unwrap_or(0)
    }

    fn set_vater(&mut self, offset: u16, value: u16) {
        if let Some(slot) = self.vater.get_mut(word_index(offset)) {
            *slot = value;
        }
    }

    fn get_the_freq(&self, offset: u16) -> u16 {
        self.the_freq.get(word_index(offset)).copied().unwrap_or(0)
    }

    fn set_the_freq(&mut self, offset: u16, value: u16) {
        if let Some(slot) = self.the_freq.get_mut(word_index(offset)) {
            *slot = value;
        }
    }

    fn get_freq(&self, offset: u16) -> u16 {
        self.frequencys
            .get(word_index(offset))
            .copied()
            .unwrap_or(0)
    }

    fn set_freq(&mut self, offset: u16, value: u16) {
        if let Some(slot) = self.frequencys.get_mut(word_index(offset)) {
            *slot = value;
        }
    }

    fn get_nvalue(&self, offset: u16) -> u16 {
        self.nvalue.get(word_index(offset)).copied().unwrap_or(0)
    }

    fn set_nvalue(&mut self, offset: u16, value: u16) {
        if let Some(slot) = self.nvalue.get_mut(word_index(offset)) {
            *slot = value;
        }
    }

    fn set_nindex(&mut self, offset: u16, value: u16) {
        if let Some(slot) = self.nindex.get_mut(word_index(offset)) {
            *slot = value;
        }
    }

    fn get_nfreq(&self, freq_even: u16) -> u16 {
        let slot = (freq_even as usize) / 2;
        self.nfreq.get(slot).copied().unwrap_or(0)
    }

    fn set_nfreq(&mut self, freq_even: u16, value: u16) {
        let slot = (freq_even as usize) / 2;
        if let Some(slot_ref) = self.nfreq.get_mut(slot) {
            *slot_ref = value;
        }
    }

    fn get_nfreqmax(&self, freq_even: u16) -> u16 {
        if freq_even < 2 {
            return 0;
        }
        let slot = (freq_even as usize / 2) - 1;
        self.nfreqmax.get(slot).copied().unwrap_or(0)
    }

    fn set_nfreqmax(&mut self, freq_even: u16, value: u16) {
        if freq_even < 2 {
            return;
        }
        let slot = (freq_even as usize / 2) - 1;
        if let Some(slot_ref) = self.nfreqmax.get_mut(slot) {
            *slot_ref = value;
        }
    }

    fn markiere(&mut self, di: u16, counter: &mut usize) {
        let idx = word_index(di);
        if idx >= self.new_index.len() || self.new_index[idx] != 0 {
            return;
        }
        let si = self.read_str(di);
        if si >= 512 {
            if si >= 2 {
                self.markiere(si.wrapping_sub(2), counter);
            }
            if si != di {
                self.markiere(si, counter);
            }
        }
        self.new_index[idx] = MARKED;
        *counter += 1;
    }

    fn clear_when_full(&mut self) {
        if self.teststrings_index != STR_IND_BUF_LEN as u16 {
            return;
        }

        self.new_index.fill(0);
        let mut count = 0usize;
        let mut di = (2 * STR_IND_BUF_LEN) as u16;
        while di > 0 {
            di = di.wrapping_sub(2);
            self.markiere(di, &mut count);
            if count >= MAXMCOUNT || di == 0 {
                break;
            }
        }

        let low = ((count + 255) * 2).min(u16::MAX as usize) as u16;
        self.low_tsi = low;

        let mut bx = 2 * 255u16;
        let mut di_iter = 2 * 254u16;
        while (bx as usize) < self.low_tsi as usize {
            di_iter = di_iter.wrapping_add(2);
            let idx = word_index(di_iter);
            if idx >= self.new_index.len() {
                break;
            }
            if self.new_index[idx] == 0 {
                continue;
            }

            self.new_index[idx] = bx;
            let mut si = self.read_str(di_iter);
            if si >= 512 {
                let map_idx = word_index(si);
                if map_idx < self.new_index.len() {
                    si = self.new_index[map_idx];
                } else {
                    si = 0;
                }
            }

            self.write_str(bx, si);
            bx = bx.wrapping_add(2);
        }
    }

    fn set_vars(&mut self) {
        // Port of HYPER.EXE `fcn.00004696` (see `hyper_exe/disasm/4696_set_vars.txt`).
        // The EXE selects between two modes (3/4). We approximate that selection
        // via the header BCD major version nibble.
        let major = self.version >> 4;

        if major == 3 {
            // mode 3
            self.mindiff = -8;
            self.maxdiff = 8;
            self.maxlocal = 0x0096;
        } else {
            // mode 4
            self.mindiff = -4;
            self.maxdiff = 8;

            // ax = (((0x009c * (teststrings_index - 0x00ff)) / 0x1f00) & 0xfffe) + 4
            // 16-bit semantics: subtraction wraps, multiply/divide are signed.
            let bx_u16 = self.teststrings_index.wrapping_sub(0x00ff);
            let bx_i32 = (bx_u16 as i16) as i32;
            let prod = 0x009c_i32 * bx_i32;
            let quot = prod / 0x1f00_i32;

            let mut ax_u16 = ((quot as i16) as u16) & 0xfffe;
            ax_u16 = ax_u16.wrapping_add(4);
            self.maxlocal = ax_u16;
        }

        self.maxlocal255 = self.maxlocal.wrapping_add(0x01fe);
        self.local_offset = (4i16 + self.maxdiff - self.mindiff) as u16;
        self.pos_offset = self
            .local_offset
            .wrapping_add(self.maxlocal)
            .wrapping_add(2);
        self.char_offset = self.pos_offset.wrapping_add(2 * (MAX_FREQ as u16 + 1));
    }

    fn init_huff_tables(&mut self) {
        self.nfreqmax.fill(0);

        let mut di = self.char_offset;
        let mut si = 0u16;
        for _ in 0..STR_IND_BUF_LEN {
            let index = word_index(si);
            if index >= self.nindex.len() {
                break;
            }
            self.nindex[index] = di;

            let dest = word_index(di);
            if dest < self.nvalue.len() {
                self.nvalue[dest] = si;
                self.frequencys[dest] = 0;
            }

            di = di.wrapping_add(2);
            si = si.wrapping_add(2);
        }

        for slot in self.nfreq.iter_mut() {
            *slot = self.char_offset;
        }

        // EXE sets only the root/anchor values here (no full memset of the tables).
        // The EXE explicitly sets nfreq[3] = 2 after the char_offset fill.
        // This corresponds to the `mov word es:[0xc326], 2` instruction at 0x4847.
        self.nfreq[3] = 2;
        self.sohn[0] = 1;
        self.the_freq[0] = 2;
        self.vater[0] = 0;
        self.huff_max = 2;
        self.huff_max_index = 2;
        self.frequencys[0] = 0;

        let mut count = (self.char_offset / 2).saturating_sub(1);
        while count > 0 {
            self.ninsert();
            count -= 1;
        }
    }

    fn ninsert(&mut self) {
        let di = self.huff_max;
        let parent = di.wrapping_sub(2);

        self.set_vater(di, parent);
        self.set_vater(di.wrapping_add(2), parent);
        self.set_the_freq(di.wrapping_add(2), 2);

        let leaf_marker = self.nfreq[3].wrapping_add(1);
        self.set_sohn(di.wrapping_add(2), leaf_marker);

        let old_child = self.get_sohn(parent);
        self.set_sohn(parent, di);
        self.set_sohn(di, old_child);

        if old_child != 0 {
            self.set_freq(old_child.wrapping_sub(1), di);
        }

        let parent_freq = self.get_the_freq(parent);
        self.set_the_freq(di, parent_freq);

        let leaf_slot = di.wrapping_add(2);
        self.set_freq(self.nfreq[3], leaf_slot);

        self.nfreq[3] = self.nfreq[3].wrapping_add(2);
        self.huff_max = leaf_slot.wrapping_add(2);

        // COMM.ASM `ninsert` ends with `jmp ientry` (internal path) using di=HuffMax-3,
        // which is the current `parent` here.
        self.inc_frequency_ientry(parent);
    }

    fn inc_frequency_ientry(&mut self, mut di: u16) {
        // Internal entry of COMM.ASM `inc_frequency` (label `ientry`).
        let limit = (2 * MAX_REC_FREQUENCY) as u16;

        loop {
            let freq = self.get_the_freq(di);
            if freq >= limit {
                return;
            }

            let si = self.get_nfreqmax(freq);
            self.set_nfreqmax(freq, si.wrapping_add(2));

            if di != si {
                let bx_child = self.get_sohn(di);
                self.set_vater(bx_child, si);
                self.set_vater(bx_child.wrapping_add(2), si);
                self.inc_frequency_entry_swap(di, si, bx_child);
            }

            self.set_the_freq(si, self.get_the_freq(si).wrapping_add(2));
            di = self.get_vater(si);
            if si == 0 {
                return;
            }
        }
    }

    fn inc_posfreq(&mut self, si: u16) {
        // Literal port of COMM.ASM `inc_posfreq`.
        let mut bx = self.get_freq(si); // frequencys[si]
        let mut di = self.get_nfreq(bx.wrapping_add(2)); // nfreq[freq+2]

        let mut item = self.get_nvalue(si);
        self.set_nindex(item, di);

        // xchg item, nvalue[di]
        let swapped = self.get_nvalue(di);
        self.set_nvalue(di, item);
        item = swapped;

        self.set_nindex(item, si);
        self.set_nvalue(si, item);

        bx = self.get_freq(di);
        if bx == (2 * MAX_FREQ as u16) {
            self.ninsert();
            let di_next = self.huff_max.wrapping_sub(2);
            self.inc_frequency(di_next);
            return;
        }

        bx = bx.wrapping_add(2);
        self.set_freq(di, bx);
        di = di.wrapping_add(2);
        self.set_nfreq(bx, di);
    }

    fn inc_frequency_entry_swap(&mut self, di: u16, si: u16, mut bx: u16) {
        // @@entry: xchg bx, sohn[si]
        let old = self.get_sohn(si);
        self.set_sohn(si, bx);
        bx = old;

        // Update back-links for the swapped-out child.
        if (bx & 1) != 0 {
            // leaf: frequencys[leaf-1] = di
            self.set_freq(bx.wrapping_sub(1), di);
        } else {
            // internal: vater[child] = di; vater[child+2] = di
            self.set_vater(bx, di);
            self.set_vater(bx.wrapping_add(2), di);
        }

        self.set_sohn(di, bx);
    }

    fn inc_frequency(&mut self, mut di: u16) {
        // Port of HYPER.EXE / COMM.ASM `inc_frequency` with the EXE's effective
        // entry selection: leaf vs internal handling depends on `sohn[di] & 1`.
        let limit = (2 * MAX_REC_FREQUENCY) as u16;

        loop {
            let freq = self.get_the_freq(di);
            if freq >= limit {
                return;
            }

            let si = self.get_nfreqmax(freq);
            self.set_nfreqmax(freq, si.wrapping_add(2));

            if di != si {
                let bx_child = self.get_sohn(di);
                if (bx_child & 1) != 0 {
                    // leaf child: update leaf->parent index.
                    self.set_freq(bx_child.wrapping_sub(1), si);
                } else {
                    // internal child: update vater links.
                    self.set_vater(bx_child, si);
                    self.set_vater(bx_child.wrapping_add(2), si);
                }
                self.inc_frequency_entry_swap(di, si, bx_child);
            }

            self.set_the_freq(si, self.get_the_freq(si).wrapping_add(2));
            di = self.get_vater(si);
            if si == 0 {
                return;
            }
        }
    }

    fn decode_huff_entry(&mut self, reader: &mut BitReader) -> Option<u16> {
        let mut si = 0u16;

        loop {
            let child = self.get_sohn(si);
            if (child & 1) != 0 {
                let leaf = child.wrapping_sub(1);
                let freq_ptr = self.get_freq(leaf);
                if UPDATE_DYNA_HUFFMAN {
                    self.inc_frequency(freq_ptr);
                }
                return Some(leaf);
            }

            let bit = reader.read_bit();
            si = if bit { child.wrapping_add(2) } else { child };
        }
    }

    fn tab_decode(&mut self, reader: &mut BitReader, freq: u16) -> u16 {
        // Literal port of UNPACK.ASM TabDecode macro.
        let base = freq.wrapping_sub(self.pos_offset);

        // AX = nfreq[base] - nfreq[base+2]; AX >>= 1; DX = AX
        let mut ax = self.get_nfreq(base);
        let mut si = self.get_nfreq(base.wrapping_add(2));
        ax = ax.wrapping_sub(si);
        ax >>= 1;
        let dx = ax;

        // CX = 1; AX = 1
        let mut cx = 1u16;
        ax = 1;

        loop {
            // rBit: carry set => bit=1
            if !reader.read_bit() {
                ax ^= cx;
            }
            cx <<= 1;
            ax |= cx;
            if ax <= dx {
                continue;
            }
            ax ^= cx;
            break;
        }

        ax <<= 1;
        si = si.wrapping_add(ax);
        let value = self.get_nvalue(si);
        if UPDATE_DYNA_HUFFMAN {
            self.inc_posfreq(si);
        }
        value
    }
}

fn read_smart(state: &mut HuffmanState, reader: &mut BitReader) -> Result<()> {
    let tsi = reader.read_bits(13) & 0x1fff;
    state.teststrings_index = tsi;
    state.set_vars();
    state.init_huff_tables();

    state.teststrings_index <<= 1;
    let mut di = state.low_tsi.wrapping_sub(2);
    state.lastposition = BASE_LASTPOSITION;
    state.nfreq[0] = state.char_offset + 2 * 255;

    loop {
        di = di.wrapping_add(2);
        if di >= state.teststrings_index {
            break;
        }

        if di > state.maxlocal255 {
            let value = di
                .wrapping_add(state.char_offset)
                .wrapping_sub(state.maxlocal);
            state.nfreq[0] = value;
        }

        let Some(symbol) = state.decode_huff_entry(reader) else {
            break;
        };

        if symbol == 2 * LSEQUENCE_KEY {
            let mut pos = state.lastposition.wrapping_add(4);
            state.write_str(di, pos);

            di = di.wrapping_add(2);
            loop {
                pos = pos.wrapping_add(4);
                state.write_str(di, pos);
                di = di.wrapping_add(2);

                if reader.read_bit() {
                    continue;
                }

                di = di.wrapping_sub(2);
                break;
            }

            state.lastposition = pos;
        } else if symbol < state.local_offset {
            let adjust = (2 * DIFF_OFFSET - state.mindiff) as u16;
            let new_pos = state.lastposition.wrapping_add(symbol).wrapping_sub(adjust);
            state.write_str(di, new_pos);
            state.lastposition = new_pos;
        } else if symbol < state.pos_offset {
            let offset = symbol.wrapping_sub(state.local_offset);
            let new_pos = di.wrapping_sub(offset);
            state.write_str(di, new_pos);
            state.lastposition = new_pos;
        } else if symbol < state.char_offset {
            let value = state.tab_decode(reader, symbol);
            if value >= 512 {
                state.write_str(di, value);
                state.lastposition = value;
            } else {
                state.write_str(di, value >> 1);
            }
        } else {
            let value = state.get_nvalue(symbol);
            if value >= 512 {
                state.write_str(di, value);
                state.lastposition = value;
            } else {
                state.write_str(di, value >> 1);
            }
        }
    }

    state.teststrings_index >>= 1;

    Ok(())
}

fn decode_data(state: &mut HuffmanState, output: &mut Vec<u8>, target_len: usize) -> Result<()> {
    let mut si = state.low_tsi;
    let tsi_bytes = state.teststrings_index << 1;
    let mut stack: Vec<u16> = Vec::with_capacity(64);

    while si < tsi_bytes && output.len() < target_len {
        let mut ax = state.read_str(si);
        si = si.wrapping_add(2);

        loop {
            if (ax & 0xFF00) == 0 {
                output.push((ax & 0x00FF) as u8);
                if output.len() >= target_len {
                    return Ok(());
                }

                if let Some(next) = stack.pop() {
                    ax = next;
                    continue;
                }
                break;
            } else {
                let bx = ax;

                let mut tail = state.read_str(bx);
                if tail == bx {
                    tail = state.read_str(bx.wrapping_sub(2));
                }
                stack.push(tail);
                ax = state.read_str(bx.wrapping_sub(2));
            }
        }
    }

    Ok(())
}

pub(crate) fn unpack_hyp(
    compressed_buffer: &[u8],
    original_size: usize,
    version: u8,
) -> Result<Vec<u8>> {
    let mut reader = BitReader::new(compressed_buffer);
    let mut state = HuffmanState::new();
    state.version = version;
    let mut output = Vec::with_capacity(original_size.max(1));

    loop {
        // Per the original EXE (fcn.00004f26), reset low_tsi to 0x1fe at the
        // START of each block, THEN run clear_when_full which may adjust it.
        state.low_tsi = 2 * 255; // 0x1fe
        state.clear_when_full();

        read_smart(&mut state, &mut reader)?;

        decode_data(&mut state, &mut output, original_size)?;

        // Exit when teststrings_index signals end (255) or we have enough output
        if state.teststrings_index == 255 || output.len() >= original_size {
            break;
        }

        // Also exit if we've consumed all input
        if reader.is_exhausted() {
            break;
        }
    }

    // Truncate to expected size if we overproduced
    output.truncate(original_size);
    Ok(output)
}
