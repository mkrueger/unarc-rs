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
const NFREQ_TABLE_LEN: usize = 8;

const MINDIFF: i16 = -4;
const MAXDIFF: i16 = 8;
const DIFF_OFFSET: i16 = 1;
const LSEQUENCE_KEY: u16 = 0;
const BASE_LASTPOSITION: u16 = 2 * 256;
const MARKED: u16 = 1;

const MAXLOCAL_40: usize = 2;
const MAXLOCAL_41: usize = 80;

#[inline]
fn word_index(offset: u16) -> usize {
    (offset as usize) >> 1
}

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

    #[inline]
    fn read_bit(&mut self) -> bool {
        let bit = (self.puffer_byte & 1) != 0;
        self.puffer_byte >>= 1;
        self.bit_cnt -= 1;
        if self.bit_cnt == 0 {
            self.byte_pos += 1;
            self.puffer_byte = self.buffer.get(self.byte_pos).copied().unwrap_or(0);
            self.bit_cnt = 8;
        }
        bit
    }

    fn read_bits(&mut self, count: usize) -> u16 {
        let mut value = 0u16;
        let mut mask = 1u16;
        for _ in 0..count {
            if self.read_bit() {
                value |= mask;
            }
            mask <<= 1;
        }
        value
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
}

impl HuffmanState {
    fn new() -> Self {
        let mut state = Self {
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
        };

        for i in 0..=255 {
            if let Some(slot) = state.str_ind_buf.get_mut(i) {
                *slot = i as u16;
            }
        }

        state
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

    fn get_new_index(&self, offset: u16) -> u16 {
        self.new_index.get(word_index(offset)).copied().unwrap_or(0)
    }

    fn set_new_index(&mut self, offset: u16, value: u16) {
        if let Some(slot) = self.new_index.get_mut(word_index(offset)) {
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
        self.low_tsi = 2 * 255;
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
        let numerator = 2 * (MAXLOCAL_41 - MAXLOCAL_40) as i32;
        let mut cx = self.teststrings_index as i32 - 255;
        if cx < 0 {
            cx = 0;
        }

        let divisor = (STR_IND_BUF_LEN as i32 - 255).max(1);
        let mut ax = (numerator * cx) / divisor;
        ax &= !1;
        ax += 2 * MAXLOCAL_40 as i32;
        let maxlocal = ax as u16;

        self.maxlocal = maxlocal;
        self.maxlocal255 = maxlocal + 2 * 255;
        self.local_offset = (2 * (DIFF_OFFSET + 1) + (MAXDIFF - MINDIFF)) as u16;
        self.pos_offset = self.local_offset + maxlocal + 2;
        self.char_offset = self.pos_offset + 2 * (MAX_FREQ as u16 + 1);
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

        self.sohn.fill(0);
        self.the_freq.fill(0);
        self.vater.fill(0);

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

        let leaf_marker = self.huff_max_index.wrapping_add(1);
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
        self.set_freq(self.huff_max_index, leaf_slot);

        self.huff_max_index = self.huff_max_index.wrapping_add(2);
        self.huff_max = leaf_slot.wrapping_add(2);

        self.inc_frequency_internal(parent);
    }

    fn inc_frequency(&mut self, di: u16) {
        let freq = self.get_the_freq(di);
        if freq == 0 || freq >= (2 * MAX_REC_FREQUENCY) as u16 {
            return;
        }

        let mut si = self.get_nfreqmax(freq);
        self.set_nfreqmax(freq, si.wrapping_add(2));

        if di != si {
            let sohn_di = self.get_sohn(di);
            if sohn_di != 0 {
                if (sohn_di & 1) != 0 {
                    self.set_freq(sohn_di.wrapping_sub(1), si);
                } else {
                    self.set_vater(sohn_di, si);
                    self.set_vater(sohn_di.wrapping_add(2), si);
                }
            }

            let sohn_si = self.get_sohn(si);
            self.set_sohn(si, sohn_di);

            if sohn_si != 0 {
                if (sohn_si & 1) != 0 {
                    self.set_freq(sohn_si.wrapping_sub(1), di);
                } else {
                    self.set_vater(sohn_si, di);
                    self.set_vater(sohn_si.wrapping_add(2), di);
                }
            }

            self.set_sohn(di, sohn_si);
        }

        let new_freq = self.get_the_freq(si).wrapping_add(2);
        self.set_the_freq(si, new_freq);

        if si != 0 {
            let parent = self.get_vater(si);
            if parent != 0 {
                self.inc_frequency_internal(parent);
            }
        }
    }

    fn inc_frequency_internal(&mut self, mut di: u16) {
        if di == 0 {
            return;
        }

        loop {
            let freq = self.get_the_freq(di);
            if freq == 0 || freq >= (2 * MAX_REC_FREQUENCY) as u16 {
                return;
            }

            let mut si = self.get_nfreqmax(freq);
            self.set_nfreqmax(freq, si.wrapping_add(2));

            if di != si {
                let sohn_di = self.get_sohn(di);
                if sohn_di != 0 {
                    self.set_vater(sohn_di, si);
                    self.set_vater(sohn_di.wrapping_add(2), si);
                }

                let sohn_si = self.get_sohn(si);
                self.set_sohn(si, sohn_di);

                if sohn_si != 0 {
                    if (sohn_si & 1) != 0 {
                        self.set_freq(sohn_si.wrapping_sub(1), di);
                    } else {
                        self.set_vater(sohn_si, di);
                        self.set_vater(sohn_si.wrapping_add(2), di);
                    }
                }

                self.set_sohn(di, sohn_si);
            }

            let new_freq = self.get_the_freq(si).wrapping_add(2);
            self.set_the_freq(si, new_freq);

            if si == 0 {
                return;
            }

            di = self.get_vater(si);
            if di == 0 {
                return;
            }
        }
    }

    fn inc_posfreq(&mut self, si: u16) {
        let freq = self.get_freq(si);
        let di = self.get_nfreq(freq.wrapping_add(2));
        let item = self.get_nvalue(si);

        self.set_nindex(item, di);

        let swapped = self.get_nvalue(di);
        self.set_nvalue(di, item);
        self.set_nindex(swapped, si);
        self.set_nvalue(si, swapped);

        let current = self.get_freq(di);
        if current == (2 * MAX_FREQ as u16) {
            self.ninsert();
            let parent = self.huff_max.wrapping_sub(2);
            self.inc_frequency(parent);
        } else {
            let updated = current.wrapping_add(2);
            self.set_freq(di, updated);
            self.set_nfreq(updated, di.wrapping_add(2));
        }
    }

    fn decode_huff_entry(&mut self, reader: &mut BitReader) -> u16 {
        let mut si = 0u16;

        loop {
            let child = self.get_sohn(si);
            if (child & 1) != 0 {
                let leaf = child.wrapping_sub(1);
                let freq_ptr = self.get_freq(leaf);
                if freq_ptr != 0 {
                    self.inc_frequency(freq_ptr);
                }
                return leaf;
            }

            let next = if reader.read_bit() {
                child.wrapping_add(2)
            } else {
                child
            };
            si = next;
        }
    }

    fn tab_decode(&mut self, reader: &mut BitReader, freq: u16) -> u16 {
        let base = freq.wrapping_sub(self.pos_offset);
        let start = self.get_nfreq(base);
        let end = self.get_nfreq(base.wrapping_add(2));
        let mut span = start.wrapping_sub(end) >> 1;

        let mut mask = 1u16;
        let mut code = 1u16;

        while {
            if !reader.read_bit() {
                code ^= mask;
            }
            mask <<= 1;
            code |= mask;
            code <= span
        } {}

        code ^= mask;
        let offset = end.wrapping_add(code << 1);
        let value = self.get_nvalue(offset);
        self.inc_posfreq(offset);
        value
    }
}

fn read_smart(state: &mut HuffmanState, reader: &mut BitReader) -> Result<()> {
    let tsi = reader.read_bits(13);
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

        let symbol = state.decode_huff_entry(reader);

        if symbol == 2 * LSEQUENCE_KEY {
            let mut pos = state.lastposition.wrapping_add(4);
            state.write_str(di, pos);

            loop {
                di = di.wrapping_add(2);
                if reader.read_bit() {
                    pos = pos.wrapping_add(4);
                    state.write_str(di, pos);
                } else {
                    di = di.wrapping_sub(2);
                    break;
                }
            }

            state.lastposition = pos;
        } else if symbol < state.local_offset {
            let adjust = (2 * DIFF_OFFSET - MINDIFF) as i32;
            let new_pos = (state.lastposition as i32 + symbol as i32 - adjust)
                .max(0)
                .min(u16::MAX as i32) as u16;
            state.write_str(di, new_pos);
            state.lastposition = new_pos;
        } else if symbol < state.pos_offset {
            let offset = symbol.wrapping_sub(state.local_offset) as i32;
            let base = (di as i32 - offset)
                .max(0)
                .min(u16::MAX as i32) as u16;
            let new_pos = base.wrapping_add(2);
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

fn decode_data(state: &mut HuffmanState, output: &mut Vec<u8>) {
    let mut si = state.low_tsi;
    let tsi_bytes = state.teststrings_index << 1;
    let start_len = output.len();
    while si < tsi_bytes {
        let value = state.read_str(si);
        si = si.wrapping_add(2);
        let mut visited = Vec::with_capacity(32);
        if !emit_value(state, value, output, &mut visited) {
            break;
        }
    }

    if cfg!(debug_assertions) {
        eprintln!("[decode] produced {} new bytes", output.len() - start_len);
    }
}

fn emit_value(
    state: &HuffmanState,
    value: u16,
    output: &mut Vec<u8>,
    visited: &mut Vec<u16>,
) -> bool {
    if value < 256 {
        output.push(value as u8);
        return true;
    }

    if visited.contains(&value) {
        return false;
    }
    visited.push(value);

    let idx = word_index(value);
    if idx == 0 || idx >= state.str_ind_buf.len() {
        visited.pop();
        return false;
    }

    let predecessor = state
        .str_ind_buf
        .get(idx.saturating_sub(1))
        .copied()
        .unwrap_or_default();

    let mut tail = state
        .str_ind_buf
        .get(idx)
        .copied()
        .unwrap_or(predecessor);

    if tail == value {
        tail = predecessor;
    }

    let ok = emit_value(state, predecessor, output, visited)
        && emit_value(state, tail, output, visited);

    visited.pop();
    ok
}

pub(crate) fn unpack_hyp(compressed_buffer: &[u8], original_size: usize) -> Result<Vec<u8>> {
    let mut reader = BitReader::new(compressed_buffer);
    let mut state = HuffmanState::new();
    let mut output = Vec::with_capacity(original_size.max(1));

    loop {
        state.clear_when_full();
        read_smart(&mut state, &mut reader)?;
        decode_data(&mut state, &mut output);
        if state.teststrings_index == 255 {
            break;
        }
    }

    Ok(output)
}
