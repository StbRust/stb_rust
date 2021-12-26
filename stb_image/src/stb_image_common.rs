// Generated by Hebron at 12/26/2021 12:56:14 PM

use std;
use c_runtime;
use crate::*;

pub const STBI__SCAN_header: i32 = 2;

pub const STBI__SCAN_load: i32 = 0;

pub const STBI__SCAN_type: i32 = 1;

pub const STBI_default: i32 = 0;

pub const STBI_grey: i32 = 1;

pub const STBI_grey_alpha: i32 = 2;

pub const STBI_ORDER_BGR: i32 = 1;

pub const STBI_ORDER_RGB: i32 = 0;

pub const STBI_rgb: i32 = 3;

pub const STBI_rgb_alpha: i32 = 4;

pub static stbi__h2l_gamma_i: f32 = 1.0f32 / 2.2f32;
pub static stbi__h2l_scale_i: f32 = 1.0f32;
pub static stbi__l2h_gamma: f32 = 2.2f32;
pub static stbi__l2h_scale: f32 = 1.0f32;
pub static stbi__vertically_flip_on_load_global: i32 = 0;
pub static stbi__vertically_flip_on_load_local: i32 = Default::default();
pub static stbi__vertically_flip_on_load_set: i32 = Default::default();
#[derive(Debug, Copy, Clone)]
pub struct stbi__context {
	pub img_x:u32,
	pub img_y:u32,
	pub img_n:i32,
	pub img_out_n:i32,
	pub io:stbi_io_callbacks,
	pub io_user_data:*mut u8,
	pub read_from_callbacks:i32,
	pub buflen:i32,
	pub buffer_start:[u8;128],
	pub callback_already_read:i32,
	pub img_buffer:*mut u8,
	pub img_buffer_end:*mut u8,
	pub img_buffer_original:*mut u8,
	pub img_buffer_original_end:*mut u8,
}

#[derive(Debug, Copy, Clone)]
pub struct stbi__result_info {
	pub bits_per_channel:i32,
	pub num_channels:i32,
	pub channel_order:i32,
}

#[derive(Debug, Copy, Clone)]
pub struct stbi_io_callbacks {
	pub read:*mut fn(arg0: *mut u8, arg1: *mut i8, arg2: i32) -> i32,
	pub skip:*mut fn(arg0: *mut u8, arg1: i32),
	pub eof:*mut fn(arg0: *mut u8) -> i32,
}

impl std::default::Default for stbi__context {
	fn default() -> Self {
		stbi__context {
			img_x: 0,
			img_y: 0,
			img_n: 0,
			img_out_n: 0,
			io: stbi_io_callbacks::default(),
			io_user_data: std::ptr::null_mut(),
			read_from_callbacks: 0,
			buflen: 0,
			buffer_start: [0;128],
			callback_already_read: 0,
			img_buffer: std::ptr::null_mut(),
			img_buffer_end: std::ptr::null_mut(),
			img_buffer_original: std::ptr::null_mut(),
			img_buffer_original_end: std::ptr::null_mut(),
		}
	}
}

impl std::default::Default for stbi__result_info {
	fn default() -> Self {
		stbi__result_info {
			bits_per_channel: 0,
			num_channels: 0,
			channel_order: 0,
		}
	}
}

impl std::default::Default for stbi_io_callbacks {
	fn default() -> Self {
		stbi_io_callbacks {
			read: std::ptr::null_mut(),
			skip: std::ptr::null_mut(),
			eof: std::ptr::null_mut(),
		}
	}
}

pub unsafe fn stbi__addsizes_valid(a: i32, b: i32) -> i32 {
	if (b) < (0) {return (0) as i32;}
	return ((a) <= (2147483647 - b)?1:0) as i32;
}

pub unsafe fn stbi__bitcount(a: u32) -> i32 {
	a = ((a & (0x55555555) as u32) + ((a >> 1) & (0x55555555) as u32)) as u32;
	a = ((a & (0x33333333) as u32) + ((a >> 2) & (0x33333333) as u32)) as u32;
	a = ((a + (a >> 4)) & (0x0f0f0f0f) as u32);
	a = (a + (a >> 8)) as u32;
	a = (a + (a >> 16)) as u32;
	return (a & (0xff) as u32) as i32;
}

pub unsafe fn stbi__compute_y(r: i32, g: i32, b: i32) -> u8 {
	return (((r * 77) + (g * 150) + (29 * b)) >> 8) as u8;
}

pub unsafe fn stbi__convert_16_to_8(orig: *mut u16, w: i32, h: i32, channels: i32) -> *mut u8 {
	let mut i: i32 = Default::default();
	let mut img_len: i32 = w * h * channels;
	let mut reduced: *mut u8 = std::ptr::null_mut();
	reduced = stbi__malloc((img_len) as u64);
	if (reduced) == (null) {return ((((stbi__err("outofmem")) != 0?(0):(0)) as u64) as *mut u8);}
	for (i = (0) as i32;(i) < (img_len);i += 1) {reduced[i] = ((((orig[i]) as i32 >> 8) & 0xFF) as u8);}
	CRuntime.free(orig);
	return reduced;
}

pub unsafe fn stbi__convert_8_to_16(orig: *mut u8, w: i32, h: i32, channels: i32) -> *mut u16 {
	let mut i: i32 = Default::default();
	let mut img_len: i32 = w * h * channels;
	let mut enlarged: *mut u16 = std::ptr::null_mut();
	enlarged = (stbi__malloc((img_len * 2) as u64)) as *mut u16;
	if (enlarged) == (null) {return ((((stbi__err("outofmem")) != 0?(0):(0)) as u64) as *mut u8) as *mut u16;}
	for (i = (0) as i32;(i) < (img_len);i += 1) {enlarged[i] = ((((orig[i]) as i32 << 8) + (orig[i]) as i32) as u16);}
	CRuntime.free(orig);
	return enlarged;
}

pub unsafe fn stbi__convert_format(data: *mut u8, img_n: i32, req_comp: i32, x: u32, y: u32) -> *mut u8 {
	let mut i: i32 = Default::default();let mut j: i32 = Default::default();
	let mut good: *mut u8 = std::ptr::null_mut();
	if (req_comp) == (img_n) {return data;}
	
	good = stbi__malloc_mad3((req_comp) as i32, (x) as i32, (y) as i32, (0) as i32);
	if (good) == (null) {
CRuntime.free(data);return ((((stbi__err("outofmem")) != 0?(0):(0)) as u64) as *mut u8);}
	for (j = (0) as i32;(j) < ((y) as i32);j += 1) {
let mut src: *mut u8 = (data).offset(((j) as u32 * x * (img_n) as u32) as isize);let mut dest: *mut u8 = (good).offset(((j) as u32 * x * (req_comp) as u32) as isize);switch (((img_n) * 8 + (req_comp))){
case ((1) * 8 + (2)):for (i = ((x - (1) as u32) as i32);(i) >= (0);i -= 1 , src = src.offset((1) as isize) , dest = dest.offset((2) as isize)) {
dest[0] = (src[0]) as u8;dest[1] = ((255) as u8);}break;case ((1) * 8 + (3)):for (i = ((x - (1) as u32) as i32);(i) >= (0);i -= 1 , src = src.offset((1) as isize) , dest = dest.offset((3) as isize)) {
dest[0] = (dest[1] = (dest[2] = (src[0]) as u8)) as u8;}break;case ((1) * 8 + (4)):for (i = ((x - (1) as u32) as i32);(i) >= (0);i -= 1 , src = src.offset((1) as isize) , dest = dest.offset((4) as isize)) {
dest[0] = (dest[1] = (dest[2] = (src[0]) as u8)) as u8;dest[3] = ((255) as u8);}break;case ((2) * 8 + (1)):for (i = ((x - (1) as u32) as i32);(i) >= (0);i -= 1 , src = src.offset((2) as isize) , dest = dest.offset((1) as isize)) {
dest[0] = (src[0]) as u8;}break;case ((2) * 8 + (3)):for (i = ((x - (1) as u32) as i32);(i) >= (0);i -= 1 , src = src.offset((2) as isize) , dest = dest.offset((3) as isize)) {
dest[0] = (dest[1] = (dest[2] = (src[0]) as u8)) as u8;}break;case ((2) * 8 + (4)):for (i = ((x - (1) as u32) as i32);(i) >= (0);i -= 1 , src = src.offset((2) as isize) , dest = dest.offset((4) as isize)) {
dest[0] = (dest[1] = (dest[2] = (src[0]) as u8)) as u8;dest[3] = (src[1]) as u8;}break;case ((3) * 8 + (4)):for (i = ((x - (1) as u32) as i32);(i) >= (0);i -= 1 , src = src.offset((3) as isize) , dest = dest.offset((4) as isize)) {
dest[0] = (src[0]) as u8;dest[1] = (src[1]) as u8;dest[2] = (src[2]) as u8;dest[3] = ((255) as u8);}break;case ((3) * 8 + (1)):for (i = ((x - (1) as u32) as i32);(i) >= (0);i -= 1 , src = src.offset((3) as isize) , dest = dest.offset((1) as isize)) {
dest[0] = (stbi__compute_y((src[0]) as i32, (src[1]) as i32, (src[2]) as i32)) as u8;}break;case ((3) * 8 + (2)):for (i = ((x - (1) as u32) as i32);(i) >= (0);i -= 1 , src = src.offset((3) as isize) , dest = dest.offset((2) as isize)) {
dest[0] = (stbi__compute_y((src[0]) as i32, (src[1]) as i32, (src[2]) as i32)) as u8;dest[1] = ((255) as u8);}break;case ((4) * 8 + (1)):for (i = ((x - (1) as u32) as i32);(i) >= (0);i -= 1 , src = src.offset((4) as isize) , dest = dest.offset((1) as isize)) {
dest[0] = (stbi__compute_y((src[0]) as i32, (src[1]) as i32, (src[2]) as i32)) as u8;}break;case ((4) * 8 + (2)):for (i = ((x - (1) as u32) as i32);(i) >= (0);i -= 1 , src = src.offset((4) as isize) , dest = dest.offset((2) as isize)) {
dest[0] = (stbi__compute_y((src[0]) as i32, (src[1]) as i32, (src[2]) as i32)) as u8;dest[1] = (src[3]) as u8;}break;case ((4) * 8 + (3)):for (i = ((x - (1) as u32) as i32);(i) >= (0);i -= 1 , src = src.offset((4) as isize) , dest = dest.offset((3) as isize)) {
dest[0] = (src[0]) as u8;dest[1] = (src[1]) as u8;dest[2] = (src[2]) as u8;}break;default: ;CRuntime.free(data);CRuntime.free(good);return ((((stbi__err("unsupported")) != 0?(0):(0)) as u64) as *mut u8);}
}
	CRuntime.free(data);
	return good;
}

pub unsafe fn stbi__get16le(s: *mut stbi__context) -> i32 {
	let mut z: i32 = (stbi__get8(s)) as i32;
	return (z + ((stbi__get8(s)) as i32 << 8)) as i32;
}

pub unsafe fn stbi__get32le(s: *mut stbi__context) -> u32 {
	let mut z: u32 = (stbi__get16le(s)) as u32;
	z += ((stbi__get16le(s)) as u32 << 16) as u32;
	return (z) as u32;
}

pub unsafe fn stbi__get8(s: *mut stbi__context) -> u8 {
	if (s.img_buffer) < (s.img_buffer_end) {return (*(s.img_buffer = s.img_buffer.offset(1))) as u8;}
	if (s.read_from_callbacks) != 0 {
stbi__refill_buffer(s);return (*(s.img_buffer = s.img_buffer.offset(1))) as u8;}
	return (0) as u8;
}

pub unsafe fn stbi__high_bit(z: u32) -> i32 {
	let mut n: i32 = 0;
	if (z) == ((0) as u32) {return (-1) as i32;}
	if (z) >= ((0x10000) as u32) {
n += (16) as i32;z >>= 16;}
	if (z) >= ((0x00100) as u32) {
n += (8) as i32;z >>= 8;}
	if (z) >= ((0x00010) as u32) {
n += (4) as i32;z >>= 4;}
	if (z) >= ((0x00004) as u32) {
n += (2) as i32;z >>= 2;}
	if (z) >= ((0x00002) as u32) {
n += (1) as i32;}
	return (n) as i32;
}

pub unsafe fn stbi__info_main(s: *mut stbi__context, x: *mut i32, y: *mut i32, comp: *mut i32) -> i32 {
	if (stbi__bmp_info(s, x, y, comp)) != 0 {return (1) as i32;}
	return (stbi__err("unknown image type")) as i32;
}

pub unsafe fn stbi__is_16_main(s: *mut stbi__context) -> i32 {
	return (0) as i32;
}

pub unsafe fn stbi__ldr_to_hdr(data: *mut u8, x: i32, y: i32, comp: i32) -> *mut f32 {
	let mut i: i32 = Default::default();let mut k: i32 = Default::default();let mut n: i32 = Default::default();
	let mut output: *mut f32 = std::ptr::null_mut();
	if data== null {return null;}
	output = (stbi__malloc_mad4((x) as i32, (y) as i32, (comp) as i32, (sizeof(float)) as i32, (0) as i32)) as *mut f32;
	if (output) == (null) {
CRuntime.free(data);return ((((stbi__err("outofmem")) != 0?(0):(0)) as u64) as *mut f32);}
	if (comp & 1) != 0 {n = (comp) as i32;} else {n = (comp - 1) as i32;}
	for (i = (0) as i32;(i) < (x * y);i += 1) {
for (k = (0) as i32;(k) < (n);k += 1) {
output[i * comp + k] = ((CRuntime.pow((((data[i * comp + k]) as i32) as f32 / 255.0f32) as f64, (stbi__l2h_gamma) as f64) * (stbi__l2h_scale) as f64) as f32);}}
	if (n) < (comp) {
for (i = (0) as i32;(i) < (x * y);i += 1) {
output[i * comp + n] = (((data[i * comp + n]) as i32) as f32 / 255.0f32) as f32;}}
	CRuntime.free(data);
	return output;
}

pub unsafe fn stbi__load_and_postprocess_16bit(s: *mut stbi__context, x: *mut i32, y: *mut i32, comp: *mut i32, req_comp: i32) -> *mut u16 {
	let mut ri: stbi__result_info = Default::default();
	let mut result: *mut u8 = stbi__load_main(s, x, y, comp, (req_comp) as i32, &mut ri, (16) as i32);
	if (result) == std::ptr::null_mut() {return null;}
	
	if ri.bits_per_channel != 16 {
result = stbi__convert_8_to_16(result, (*x) as i32, (*y) as i32, ((req_comp) == (0)?*comp:req_comp) as i32);ri.bits_per_channel = (16) as i32;}
	if ((stbi__vertically_flip_on_load_set) != 0?stbi__vertically_flip_on_load_local:stbi__vertically_flip_on_load_global) != 0 {
let mut channels: i32 = (req_comp) != 0?req_comp:*comp;stbi__vertical_flip(result, (*x) as i32, (*y) as i32, ((channels) as u64 * std::mem::size_of::<u16>()) as i32);}
	return (result) as *mut u16;
}

pub unsafe fn stbi__load_and_postprocess_8bit(s: *mut stbi__context, x: *mut i32, y: *mut i32, comp: *mut i32, req_comp: i32) -> *mut u8 {
	let mut ri: stbi__result_info = Default::default();
	let mut result: *mut u8 = stbi__load_main(s, x, y, comp, (req_comp) as i32, &mut ri, (8) as i32);
	if (result) == std::ptr::null_mut() {return null;}
	
	if ri.bits_per_channel != 8 {
result = stbi__convert_16_to_8((result) as *mut u16, (*x) as i32, (*y) as i32, ((req_comp) == (0)?*comp:req_comp) as i32);ri.bits_per_channel = (8) as i32;}
	if ((stbi__vertically_flip_on_load_set) != 0?stbi__vertically_flip_on_load_local:stbi__vertically_flip_on_load_global) != 0 {
let mut channels: i32 = (req_comp) != 0?req_comp:*comp;stbi__vertical_flip(result, (*x) as i32, (*y) as i32, ((channels) as u64 * std::mem::size_of::<u8>()) as i32);}
	return result;
}

pub unsafe fn stbi__load_main(s: *mut stbi__context, x: *mut i32, y: *mut i32, comp: *mut i32, req_comp: i32, ri: *mut stbi__result_info, bpc: i32) -> *mut u8 {
	CRuntime.memset(ri, (0) as i32, (std::mem::size_of(stbi__result_info)) as u64);
	ri.bits_per_channel = (8) as i32;
	ri.channel_order = (STBI_ORDER_RGB) as i32;
	ri.num_channels = (0) as i32;
	if (stbi__bmp_test(s)) != 0 {return stbi__bmp_load(s, x, y, comp, (req_comp) as i32, ri);}
	
	return ((((stbi__err("unknown image type")) != 0?(0):(0)) as u64) as *mut u8);
}

pub unsafe fn stbi__loadf_main(s: *mut stbi__context, x: *mut i32, y: *mut i32, comp: *mut i32, req_comp: i32) -> *mut f32 {
	let mut data: *mut u8 = std::ptr::null_mut();
	data = stbi__load_and_postprocess_8bit(s, x, y, comp, (req_comp) as i32);
	if (data) != null {return stbi__ldr_to_hdr(data, (*x) as i32, (*y) as i32, ((req_comp) != 0?req_comp:*comp) as i32);}
	return ((((stbi__err("unknown image type")) != 0?(0):(0)) as u64) as *mut f32);
}

pub unsafe fn stbi__mad3sizes_valid(a: i32, b: i32, c: i32, add: i32) -> i32 {
	return ((((stbi__mul2sizes_valid((a) as i32, (b) as i32)) != 0) && ((stbi__mul2sizes_valid((a * b) as i32, (c) as i32)) != 0)) && ((stbi__addsizes_valid((a * b * c) as i32, (add) as i32)) != 0)?1:0) as i32;
}

pub unsafe fn stbi__mad4sizes_valid(a: i32, b: i32, c: i32, d: i32, add: i32) -> i32 {
	return (((((stbi__mul2sizes_valid((a) as i32, (b) as i32)) != 0) && ((stbi__mul2sizes_valid((a * b) as i32, (c) as i32)) != 0)) && ((stbi__mul2sizes_valid((a * b * c) as i32, (d) as i32)) != 0)) && ((stbi__addsizes_valid((a * b * c * d) as i32, (add) as i32)) != 0)?1:0) as i32;
}

pub unsafe fn stbi__malloc(size: u64) -> *mut u8 {
	return CRuntime.malloc((size) as u64);
}

pub unsafe fn stbi__malloc_mad3(a: i32, b: i32, c: i32, add: i32) -> *mut u8 {
	if stbi__mad3sizes_valid((a) as i32, (b) as i32, (c) as i32, (add) as i32)== 0 {return std::ptr::null_mut();}
	return stbi__malloc((a * b * c + add) as u64);
}

pub unsafe fn stbi__malloc_mad4(a: i32, b: i32, c: i32, d: i32, add: i32) -> *mut u8 {
	if stbi__mad4sizes_valid((a) as i32, (b) as i32, (c) as i32, (d) as i32, (add) as i32)== 0 {return std::ptr::null_mut();}
	return stbi__malloc((a * b * c * d + add) as u64);
}

pub unsafe fn stbi__mul2sizes_valid(a: i32, b: i32) -> i32 {
	if ((a) < (0)) || ((b) < (0)) {return (0) as i32;}
	if (b) == (0) {return (1) as i32;}
	return ((a) <= (2147483647 / b)?1:0) as i32;
}

pub unsafe fn stbi__refill_buffer(s: *mut stbi__context) {
	let mut n: i32 = s.io.read(s.io_user_data, (s.buffer_start) as *mut i8, (s.buflen) as i32);
	s.callback_already_read += (((s.img_buffer).offset(-((s.img_buffer_original) as isize))) as i32);
	if (n) == (0) {
s.read_from_callbacks = (0) as i32;s.img_buffer = s.buffer_start;s.img_buffer_end = (s.buffer_start).offset((1) as isize);*s.img_buffer = ((0) as u8);} else {
s.img_buffer = s.buffer_start;s.img_buffer_end = (s.buffer_start).offset((n) as isize);}
}

pub unsafe fn stbi__rewind(s: *mut stbi__context) {
	s.img_buffer = s.img_buffer_original;
	s.img_buffer_end = s.img_buffer_original_end;
}

pub unsafe fn stbi__shiftsigned(v: u32, shift: i32, bits: i32) -> i32 {
	
	
	if (shift) < (0) {v <<= -shift;} else {v >>= shift;}
	
	v >>= (8 - bits);
	
	return ((v * stbi__shiftsigned_mul_table[bits]) as i32 >> stbi__shiftsigned_shift_table[bits]) as i32;
}

pub unsafe fn stbi__skip(s: *mut stbi__context, n: i32) {
	if (n) == (0) {return;}
	if (n) < (0) {
s.img_buffer = s.img_buffer_end;return;}
	if (s.io.read) != null {
let mut blen: i32 = ((s.img_buffer_end).offset(-((s.img_buffer) as isize))) as i32;if (blen) < (n) {
s.img_buffer = s.img_buffer_end;s.io.skip(s.io_user_data, (n - blen) as i32);return;}}
	s.img_buffer = s.img_buffer.offset((n) as isize);
}

pub unsafe fn stbi__start_callbacks(s: *mut stbi__context, c: *mut stbi_io_callbacks, user: *mut u8) {
	s.io = (*c) as stbi_io_callbacks;
	s.io_user_data = user;
	s.buflen = ((128 * std::mem::size_of(u8)) as i32);
	s.read_from_callbacks = (1) as i32;
	s.callback_already_read = (0) as i32;
	s.img_buffer = s.img_buffer_original = s.buffer_start;
	stbi__refill_buffer(s);
	s.img_buffer_original_end = s.img_buffer_end;
}

pub unsafe fn stbi__start_mem(s: *mut stbi__context, buffer: *mut u8, len: i32) {
	s.io.read = null;
	s.read_from_callbacks = (0) as i32;
	s.callback_already_read = (0) as i32;
	s.img_buffer = s.img_buffer_original = buffer;
	s.img_buffer_end = s.img_buffer_original_end = (buffer).offset((len) as isize);
}

pub unsafe fn stbi__vertical_flip(image: *mut u8, w: i32, h: i32, bytes_per_pixel: i32) {
	let mut row: i32 = Default::default();
	let mut bytes_per_row: u64 = (w) as u64 * (bytes_per_pixel) as u64;
	let mut temp: [u8;2048] = 2048;
	let mut bytes: *mut u8 = image;
	for (row = (0) as i32;(row) < (h >> 1);row += 1) {
let mut row0: *mut u8 = (bytes).offset(((row) as u64 * bytes_per_row) as isize);let mut row1: *mut u8 = (bytes).offset(((h - row - 1) as u64 * bytes_per_row) as isize);let mut bytes_left: u64 = bytes_per_row;while ((bytes_left) != 0) {
let mut bytes_copy: u64 = ((bytes_left) < (2048 * std::mem::size_of(u8)))?bytes_left:2048 * std::mem::size_of(u8);CRuntime.memcpy(temp, row0, (bytes_copy) as u64);CRuntime.memcpy(row0, row1, (bytes_copy) as u64);CRuntime.memcpy(row1, temp, (bytes_copy) as u64);row0 = row0.offset((bytes_copy) as isize);row1 = row1.offset((bytes_copy) as isize);bytes_left -= (bytes_copy) as u64;}}
}

pub unsafe fn stbi_hdr_to_ldr_gamma(gamma: f32) {
	stbi__h2l_gamma_i = ((1) as f32 / gamma) as f32;
}

pub unsafe fn stbi_hdr_to_ldr_scale(scale: f32) {
	stbi__h2l_scale_i = ((1) as f32 / scale) as f32;
}

pub unsafe fn stbi_image_free(retval_from_stbi_load: *mut u8) {
	CRuntime.free(retval_from_stbi_load);
}

pub unsafe fn stbi_info_from_callbacks(c: *mut stbi_io_callbacks, user: *mut u8, x: *mut i32, y: *mut i32, comp: *mut i32) -> i32 {
	let mut s: stbi__context = Default::default();
	stbi__start_callbacks(&mut s, c, user);
	return (stbi__info_main(&mut s, x, y, comp)) as i32;
}

pub unsafe fn stbi_info_from_memory(buffer: *mut u8, len: i32, x: *mut i32, y: *mut i32, comp: *mut i32) -> i32 {
	let mut s: stbi__context = Default::default();
	stbi__start_mem(&mut s, buffer, (len) as i32);
	return (stbi__info_main(&mut s, x, y, comp)) as i32;
}

pub unsafe fn stbi_is_16_bit_from_callbacks(c: *mut stbi_io_callbacks, user: *mut u8) -> i32 {
	let mut s: stbi__context = Default::default();
	stbi__start_callbacks(&mut s, c, user);
	return (stbi__is_16_main(&mut s)) as i32;
}

pub unsafe fn stbi_is_16_bit_from_memory(buffer: *mut u8, len: i32) -> i32 {
	let mut s: stbi__context = Default::default();
	stbi__start_mem(&mut s, buffer, (len) as i32);
	return (stbi__is_16_main(&mut s)) as i32;
}

pub unsafe fn stbi_is_hdr_from_callbacks(clbk: *mut stbi_io_callbacks, user: *mut u8) -> i32 {
	
	
	return (0) as i32;
}

pub unsafe fn stbi_is_hdr_from_memory(buffer: *mut u8, len: i32) -> i32 {
	
	
	return (0) as i32;
}

pub unsafe fn stbi_ldr_to_hdr_gamma(gamma: f32) {
	stbi__l2h_gamma = (gamma) as f32;
}

pub unsafe fn stbi_ldr_to_hdr_scale(scale: f32) {
	stbi__l2h_scale = (scale) as f32;
}

pub unsafe fn stbi_load_16_from_callbacks(clbk: *mut stbi_io_callbacks, user: *mut u8, x: *mut i32, y: *mut i32, channels_in_file: *mut i32, desired_channels: i32) -> *mut u16 {
	let mut s: stbi__context = Default::default();
	stbi__start_callbacks(&mut s, clbk, user);
	return stbi__load_and_postprocess_16bit(&mut s, x, y, channels_in_file, (desired_channels) as i32);
}

pub unsafe fn stbi_load_16_from_memory(buffer: *mut u8, len: i32, x: *mut i32, y: *mut i32, channels_in_file: *mut i32, desired_channels: i32) -> *mut u16 {
	let mut s: stbi__context = Default::default();
	stbi__start_mem(&mut s, buffer, (len) as i32);
	return stbi__load_and_postprocess_16bit(&mut s, x, y, channels_in_file, (desired_channels) as i32);
}

pub unsafe fn stbi_load_from_callbacks(clbk: *mut stbi_io_callbacks, user: *mut u8, x: *mut i32, y: *mut i32, comp: *mut i32, req_comp: i32) -> *mut u8 {
	let mut s: stbi__context = Default::default();
	stbi__start_callbacks(&mut s, clbk, user);
	return stbi__load_and_postprocess_8bit(&mut s, x, y, comp, (req_comp) as i32);
}

pub unsafe fn stbi_load_from_memory(buffer: *mut u8, len: i32, x: *mut i32, y: *mut i32, comp: *mut i32, req_comp: i32) -> *mut u8 {
	let mut s: stbi__context = Default::default();
	stbi__start_mem(&mut s, buffer, (len) as i32);
	return stbi__load_and_postprocess_8bit(&mut s, x, y, comp, (req_comp) as i32);
}

pub unsafe fn stbi_loadf_from_callbacks(clbk: *mut stbi_io_callbacks, user: *mut u8, x: *mut i32, y: *mut i32, comp: *mut i32, req_comp: i32) -> *mut f32 {
	let mut s: stbi__context = Default::default();
	stbi__start_callbacks(&mut s, clbk, user);
	return stbi__loadf_main(&mut s, x, y, comp, (req_comp) as i32);
}

pub unsafe fn stbi_loadf_from_memory(buffer: *mut u8, len: i32, x: *mut i32, y: *mut i32, comp: *mut i32, req_comp: i32) -> *mut f32 {
	let mut s: stbi__context = Default::default();
	stbi__start_mem(&mut s, buffer, (len) as i32);
	return stbi__loadf_main(&mut s, x, y, comp, (req_comp) as i32);
}

pub unsafe fn stbi_set_flip_vertically_on_load(flag_true_if_should_flip: i32) {
	stbi__vertically_flip_on_load_global = (flag_true_if_should_flip) as i32;
}

pub unsafe fn stbi_set_flip_vertically_on_load_thread(flag_true_if_should_flip: i32) {
	stbi__vertically_flip_on_load_local = (flag_true_if_should_flip) as i32;
	stbi__vertically_flip_on_load_set = (1) as i32;
}

