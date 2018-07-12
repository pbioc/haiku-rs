//
// Copyright 2018, Niels Sascha Reedijk <niels.reedijk@gmail.com>
// All rights reserved. Distributed under the terms of the MIT License.
//

#![allow(non_camel_case_types)]

extern crate libc;
use libc::{c_int, c_char, DIR, dirent, off_t, size_t, ssize_t};

macro_rules! haiku_constant {
	($a:tt, $b:tt, $c:tt, $d:tt) => ((($a as u32) << 24) + (($b as u32) << 16) + (($c as u32) << 8) + ($d as u32));
}

pub mod message;

// OS.h
pub type area_id = i32;
pub type port_id = i32;
pub type sem_id = i32;
pub type team_id = i32;
pub type thread_id = i32;

pub type status_t = i32;
pub type bigtime_t = i64;

// fs_attr.h
#[repr(C)]
pub struct attr_info {
	pub attr_type: u32,
	pub size: off_t,
}

extern {
	pub fn fs_read_attr(fd: c_int, attribute: *const c_char, typeCode: u32,
						pos: off_t, buffer: *mut u8, readBytes: size_t) -> ssize_t;
	pub fn fs_write_attr(fd: c_int, attribute: *const c_char, typeCode: u32,
						pos: off_t, buffer: *const u8, readBytes: size_t) -> ssize_t;
	pub fn fs_remove_attr(fd: c_int, attribute: *const c_char) -> c_int;
	pub fn fs_stat_attr(fd: c_int, attribute: *const c_char, attrInfo: *mut attr_info) -> c_int;
	
	pub fn fs_open_attr(path: *const c_char, attribute: *const c_char,
						typeCode: u32, openMode: c_int) -> c_int;
	pub fn fs_fopen_attr(fd: c_int, attribute: *const c_char, typeCode: u32, 
						openMode: c_int) -> c_int;
	pub fn fs_close_attr(fd: c_int) -> c_int;
	
	pub fn fs_open_attr_dir(path: *const c_char) -> *mut DIR;
	pub fn fs_lopen_attr_dir(path: *const c_char) -> *mut DIR;
	pub fn fs_fopen_attr_dir(fd: c_int) -> *mut DIR;
	pub fn fs_close_attr_dir(dir: *mut DIR) -> c_int;
	pub fn fs_read_attr_dir(dir: *mut DIR) -> *mut dirent;
	pub fn fs_rewind_attr_dir(dir: *mut DIR);
}

// support/TypeConstants.h
pub const B_AFFINE_TRANSFORM_TYPE: u32 = haiku_constant!('A','M','T','X');
pub const B_ALIGNMENT_TYPE: u32 = haiku_constant!('A','L','G','N');
pub const B_ANY_TYPE: u32 = haiku_constant!('A','N','Y','T');
pub const B_ATOM_TYPE: u32 = haiku_constant!('A','T','O','M');
pub const B_ATOM_REF_TYPE: u32 = haiku_constant!('A','T','M','R');
pub const B_BOOL_TYPE: u32 = haiku_constant!('B','O','O','L');
pub const B_CHAR_TYPE: u32 = haiku_constant!('C','H','A','R');
pub const B_COLOR_8_BIT_TYPE: u32 = haiku_constant!('C','L','R','B');
pub const B_DOUBLE_TYPE: u32 = haiku_constant!('D','B','L','E');
pub const B_FLOAT_TYPE: u32 = haiku_constant!('F','L','O','T');
pub const B_GRAYSCALE_8_BIT_TYPE: u32 = haiku_constant!('G','R','Y','B');
pub const B_INT16_TYPE: u32 = haiku_constant!('S','H','R','T');
pub const B_INT32_TYPE: u32 = haiku_constant!('L','O','N','G');
pub const B_INT64_TYPE: u32 = haiku_constant!('L','L','N','G');
pub const B_INT8_TYPE: u32 = haiku_constant!('B','Y','T','E'); 
pub const B_LARGE_ICON_TYPE: u32 = haiku_constant!('I','C','O','N');
pub const B_MEDIA_PARAMETER_GROUP_TYPE: u32 = haiku_constant!('B','M','C','G');
pub const B_MEDIA_PARAMETER_TYPE: u32 = haiku_constant!('B','M','C','T');
pub const B_MEDIA_PARAMETER_WEB_TYPE: u32 = haiku_constant!('B','M','C','W');
pub const B_MESSAGE_TYPE: u32 = haiku_constant!('M','S','G','G');
pub const B_MESSENGER_TYPE: u32 = haiku_constant!('M','S','N','G');
pub const B_MIME_TYPE: u32 = haiku_constant!('M','I','M','E');
pub const B_MINI_ICON_TYPE: u32 = haiku_constant!('M','I','C','N');
pub const B_MONOCHROME_1_BIT_TYPE: u32 = haiku_constant!('M','N','O','B');
pub const B_OBJECT_TYPE: u32 = haiku_constant!('O','P','T','R');
pub const B_OFF_T_TYPE: u32 = haiku_constant!('O','F','F','T');
pub const B_PATTERN_TYPE: u32 = haiku_constant!('P','A','T','N');
pub const B_POINTER_TYPE: u32 = haiku_constant!('P','N','T','R');
pub const B_POINT_TYPE: u32 = haiku_constant!('B','P','N','T');
pub const B_PROPERTY_INFO_TYPE: u32 = haiku_constant!('S','C','T','D');
pub const B_RAW_TYPE: u32 = haiku_constant!('R','A','W','T');
pub const B_RECT_TYPE: u32 = haiku_constant!('R','E','C','T');
pub const B_REF_TYPE: u32 = haiku_constant!('R','R','E','F');
pub const B_RGB_32_BIT_TYPE: u32 = haiku_constant!('R','G','B','B');
pub const B_RGB_COLOR_TYPE: u32 = haiku_constant!('R','G','B','C');
pub const B_SIZE_TYPE: u32 = haiku_constant!('S','I','Z','E');
pub const B_SIZE_T_TYPE: u32 = haiku_constant!('S','I','Z','T');
pub const B_SSIZE_T_TYPE: u32 = haiku_constant!('S','S','Z','T');
pub const B_STRING_TYPE: u32 = haiku_constant!('C','S','T','R');
pub const B_STRING_LIST_TYPE: u32 = haiku_constant!('S','T','R','L');
pub const B_TIME_TYPE: u32 = haiku_constant!('T','I','M','E');
pub const B_UINT16_TYPE: u32 = haiku_constant!('U','S','H','T');
pub const B_UINT32_TYPE: u32 = haiku_constant!('U','L','N','G');
pub const B_UINT64_TYPE: u32 = haiku_constant!('U','L','L','G');
pub const B_UINT8_TYPE: u32 = haiku_constant!('U','B','Y','T');
pub const B_VECTOR_ICON_TYPE: u32 = haiku_constant!('V','I','C','N');
pub const B_XATTR_TYPE: u32 = haiku_constant!('X','A','T','R');
pub const B_NETWORK_ADDRESS_TYPE: u32 = haiku_constant!('N','W','A','D');
pub const B_MIME_STRING_TYPE: u32 = haiku_constant!('M','I','M','S');

// SupportDefs.h
pub type type_code = u32;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}