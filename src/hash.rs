#![allow(non_snake_case)]
#![allow(dead_code)]
use std::convert::TryFrom;
use crate::e8::E8;
// typedef struct {
// 	int hashbitlen;	   	              /*the message digest size*/
// 	unsigned long long databitlen;    /*the message size in bits*/
// 	unsigned long long datasize_in_buffer;      /*the size of the message remained in buffer; assumed to be multiple of 8bits except for the last partial block at the end of the message*/
// 	DATA_ALIGN16(uint64 x[8][2]);     /*the 1024-bit state, ( x[i][0] || x[i][1] ) is the ith row of the state in the pseudocode*/
// 	unsigned char buffer[64];         /*the 512-bit message block to be hashed;*/
// } hashState;
const JH224_H0: [u8; 128]=[0x2d,0xfe,0xdd,0x62,0xf9,0x9a,0x98,0xac,0xae,0x7c,0xac,0xd6,0x19,0xd6,0x34,0xe7,0xa4,0x83,0x10,0x5,0xbc,0x30,0x12,0x16,0xb8,0x60,0x38,0xc6,0xc9,0x66,0x14,0x94,0x66,0xd9,0x89,0x9f,0x25,0x80,0x70,0x6f,0xce,0x9e,0xa3,0x1b,0x1d,0x9b,0x1a,0xdc,0x11,0xe8,0x32,0x5f,0x7b,0x36,0x6e,0x10,0xf9,0x94,0x85,0x7f,0x2,0xfa,0x6,0xc1,0x1b,0x4f,0x1b,0x5c,0xd8,0xc8,0x40,0xb3,0x97,0xf6,0xa1,0x7f,0x6e,0x73,0x80,0x99,0xdc,0xdf,0x93,0xa5,0xad,0xea,0xa3,0xd3,0xa4,0x31,0xe8,0xde,0xc9,0x53,0x9a,0x68,0x22,0xb4,0xa9,0x8a,0xec,0x86,0xa1,0xe4,0xd5,0x74,0xac,0x95,0x9c,0xe5,0x6c,0xf0,0x15,0x96,0xd,0xea,0xb5,0xab,0x2b,0xbf,0x96,0x11,0xdc,0xf0,0xdd,0x64,0xea,0x6e];
const JH256_H0: [u8; 128]=[0xeb,0x98,0xa3,0x41,0x2c,0x20,0xd3,0xeb,0x92,0xcd,0xbe,0x7b,0x9c,0xb2,0x45,0xc1,0x1c,0x93,0x51,0x91,0x60,0xd4,0xc7,0xfa,0x26,0x0,0x82,0xd6,0x7e,0x50,0x8a,0x3,0xa4,0x23,0x9e,0x26,0x77,0x26,0xb9,0x45,0xe0,0xfb,0x1a,0x48,0xd4,0x1a,0x94,0x77,0xcd,0xb5,0xab,0x26,0x2,0x6b,0x17,0x7a,0x56,0xf0,0x24,0x42,0xf,0xff,0x2f,0xa8,0x71,0xa3,0x96,0x89,0x7f,0x2e,0x4d,0x75,0x1d,0x14,0x49,0x8,0xf7,0x7d,0xe2,0x62,0x27,0x76,0x95,0xf7,0x76,0x24,0x8f,0x94,0x87,0xd5,0xb6,0x57,0x47,0x80,0x29,0x6c,0x5c,0x5e,0x27,0x2d,0xac,0x8e,0xd,0x6c,0x51,0x84,0x50,0xc6,0x57,0x5,0x7a,0xf,0x7b,0xe4,0xd3,0x67,0x70,0x24,0x12,0xea,0x89,0xe3,0xab,0x13,0xd3,0x1c,0xd7,0x69];
const JH384_H0: [u8; 128]=[0x48,0x1e,0x3b,0xc6,0xd8,0x13,0x39,0x8a,0x6d,0x3b,0x5e,0x89,0x4a,0xde,0x87,0x9b,0x63,0xfa,0xea,0x68,0xd4,0x80,0xad,0x2e,0x33,0x2c,0xcb,0x21,0x48,0xf,0x82,0x67,0x98,0xae,0xc8,0x4d,0x90,0x82,0xb9,0x28,0xd4,0x55,0xea,0x30,0x41,0x11,0x42,0x49,0x36,0xf5,0x55,0xb2,0x92,0x48,0x47,0xec,0xc7,0x25,0xa,0x93,0xba,0xf4,0x3c,0xe1,0x56,0x9b,0x7f,0x8a,0x27,0xdb,0x45,0x4c,0x9e,0xfc,0xbd,0x49,0x63,0x97,0xaf,0xe,0x58,0x9f,0xc2,0x7d,0x26,0xaa,0x80,0xcd,0x80,0xc0,0x8b,0x8c,0x9d,0xeb,0x2e,0xda,0x8a,0x79,0x81,0xe8,0xf8,0xd5,0x37,0x3a,0xf4,0x39,0x67,0xad,0xdd,0xd1,0x7a,0x71,0xa9,0xb4,0xd3,0xbd,0xa4,0x75,0xd3,0x94,0x97,0x6c,0x3f,0xba,0x98,0x42,0x73,0x7f];
const JH512_H0: [u8; 128]=[0x6f,0xd1,0x4b,0x96,0x3e,0x0,0xaa,0x17,0x63,0x6a,0x2e,0x5,0x7a,0x15,0xd5,0x43,0x8a,0x22,0x5e,0x8d,0xc,0x97,0xef,0xb,0xe9,0x34,0x12,0x59,0xf2,0xb3,0xc3,0x61,0x89,0x1d,0xa0,0xc1,0x53,0x6f,0x80,0x1e,0x2a,0xa9,0x5,0x6b,0xea,0x2b,0x6d,0x80,0x58,0x8e,0xcc,0xdb,0x20,0x75,0xba,0xa6,0xa9,0xf,0x3a,0x76,0xba,0xf8,0x3b,0xf7,0x1,0x69,0xe6,0x5,0x41,0xe3,0x4a,0x69,0x46,0xb5,0x8a,0x8e,0x2e,0x6f,0xe6,0x5a,0x10,0x47,0xa7,0xd0,0xc1,0x84,0x3c,0x24,0x3b,0x6e,0x71,0xb1,0x2d,0x5a,0xc1,0x99,0xcf,0x57,0xf6,0xec,0x9d,0xb1,0xf8,0x56,0xa7,0x6,0x88,0x7c,0x57,0x16,0xb1,0x56,0xe3,0xc2,0xfc,0xdf,0xe6,0x85,0x17,0xfb,0x54,0x5a,0x46,0x78,0xcc,0x8c,0xdd,0x4b];


pub struct HashState {
    hash_bit_len: HashBitLen,
    data_bit_len: usize,
    datasize_in_buffer: usize,
    pub state: [[u64; 2]; 8],
    buffer: [u8; 64],
}

impl HashState {
    /// before hashing a message, initialize the hash state as H0
    pub fn new(hash_bit_len: HashBitLen) -> Self {
        let state = match hash_bit_len {
            HashBitLen::L224 => JH224_H0,
            HashBitLen::L256 => JH256_H0,
            HashBitLen::L384 => JH384_H0,
            HashBitLen::L512 => JH512_H0,
        };
        unsafe {
            let state: [[u64; 2]; 8] = std::mem::transmute(state);
            HashState {
                hash_bit_len,
                data_bit_len: 0,
                datasize_in_buffer: 0,
                state,
                buffer: [0; 64],
            }
        }
    }

    /// hash each 512-bit message block, except the last partial block
    pub fn update(&mut self, data: &[u8]) {
        // the starting address of the data to be compressed
        let mut index = 0;
        self.data_bit_len += data.len()*8;
        let mut data_bit_len = data.len()*8;
        /*if there is remaining data in the buffer, fill it to a full message block first*/
	    /*we assume that the size of the data in the buffer is the multiple of 8 bits if it is not at the end of a message*/

	    /*There is data in the buffer, but the incoming data is insufficient for a full block*/
        if (self.datasize_in_buffer > 0 ) && (( self.datasize_in_buffer + data_bit_len) < 512) {
            if (data_bit_len & 7) == 0 {
                self.buffer[self.datasize_in_buffer >> 3..(64-(self.datasize_in_buffer >> 3))].clone_from_slice(&data[..]);
                // memcpy(self.buffer + (self.datasize_in_buffer >> 3), data, 64-(self.datasize_in_buffer >> 3)) ;
            }else{
                // memcpy(self.buffer + (self.datasize_in_buffer >> 3), data, 64-(self.datasize_in_buffer >> 3)+1)
                self.buffer[self.datasize_in_buffer >> 3..(64-(self.datasize_in_buffer >> 3)+1)].clone_from_slice(&data[..]);
            } ;
            self.datasize_in_buffer += data_bit_len;
            data_bit_len = 0; //TODO ?
        }

        /*There is data in the buffer, and the incoming data is sufficient for a full block*/
        if (self.datasize_in_buffer > 0 ) && (( self.datasize_in_buffer + data_bit_len) >= 512) {
            // memcpy( self.buffer + (self.datasize_in_buffer >> 3), data, 64-(self.datasize_in_buffer >> 3) ) ;
            self.buffer[self.datasize_in_buffer >> 3..(64-(self.datasize_in_buffer >> 3))].clone_from_slice(&data[..]);
            index = 64-(self.datasize_in_buffer >> 3);
            data_bit_len = data_bit_len - (512 - self.datasize_in_buffer);
            F8(self);
            self.datasize_in_buffer = 0;
        }

        /*hash the remaining full message blocks*/
        while data_bit_len >= 512 {
            // memcpy(state->buffer, data+index, 64);
            self.buffer[..].clone_from_slice(&data[index..index+64]);
            F8(self);
            index = index+64;
            data_bit_len = data_bit_len - 512;
        }

        /*store the partial block into buffer, assume that -- if part of the last byte is not part of the message, then that part consists of 0 bits*/
        if data_bit_len > 0 {
            if (data_bit_len & 7) == 0 {
                // memcpy(state->buffer, data+index, (data_bit_len & 0x1ff) >> 3);
                let data_ref = &data[index..index+((data_bit_len & 0x1ff) >> 3)];
                self.buffer[..data_ref.len()].clone_from_slice(data_ref);
            }else{
                // memcpy(state->buffer, data+index, ((data_bit_len & 0x1ff) >> 3)+1);
                let data_ref = &data[index..index+((data_bit_len & 0x1ff) >> 3)+1];
                self.buffer[..data_ref.len()].clone_from_slice(data_ref);
            }
            self.datasize_in_buffer = data_bit_len;
        }
    }

    /// pad the message, process the padded block(s), truncate the hash value H to obtain the message digest
    pub fn finalize(&mut self, hashval: &mut [u8]) {
        // unsigned int i;

        if  (self.data_bit_len & 0x1ff) == 0  {
                /*pad the message when databitlen is multiple of 512 bits, then process the padded block*/
                for b in self.buffer[0..64].iter_mut() {
                    *b = 0;
                }
                self.buffer[0]  = 0x80;
                self.buffer[63] = ((self.data_bit_len as u64      ) & 0xff) as u8;
                self.buffer[62] = ((self.data_bit_len as u64 >> 8 ) & 0xff) as u8;
                self.buffer[61] = ((self.data_bit_len as u64 >> 16) & 0xff) as u8;
                self.buffer[60] = ((self.data_bit_len as u64 >> 24) & 0xff) as u8;
                self.buffer[59] = ((self.data_bit_len as u64 >> 32) & 0xff) as u8;
                self.buffer[58] = ((self.data_bit_len as u64 >> 40) & 0xff) as u8;
                self.buffer[57] = ((self.data_bit_len as u64 >> 48) & 0xff) as u8;
                self.buffer[56] = ((self.data_bit_len as u64 >> 56) & 0xff) as u8;
                F8(self);
        } else {
                /*set the rest of the bytes in the buffer to 0*/
                if (self.datasize_in_buffer & 7) == 0 {
                    for i in ((self.data_bit_len & 0x1ff) >> 3)..64  {
                        self.buffer[i] = 0;
                    }
                }else{
                    for i in (((self.data_bit_len & 0x1ff) >> 3)+1)..64 { 
                        self.buffer[i] = 0;
                    }
                }

                /*pad and process the partial block when data_bit_len is not multiple of 512 bits, then hash the padded blocks*/
                self.buffer[((self.data_bit_len & 0x1ff) >> 3)] |= 1 << (7- (self.data_bit_len & 7));

                F8(self);
                for b in self.buffer[0..64].iter_mut() {
                    *b = 0;
                }
                self.buffer[63] = ((self.data_bit_len as u64      ) & 0xff) as u8;
                self.buffer[62] = ((self.data_bit_len as u64 >> 8 ) & 0xff) as u8;
                self.buffer[61] = ((self.data_bit_len as u64 >> 16) & 0xff) as u8;
                self.buffer[60] = ((self.data_bit_len as u64 >> 24) & 0xff) as u8;
                self.buffer[59] = ((self.data_bit_len as u64 >> 32) & 0xff) as u8;
                self.buffer[58] = ((self.data_bit_len as u64 >> 40) & 0xff) as u8;
                self.buffer[57] = ((self.data_bit_len as u64 >> 48) & 0xff) as u8;
                self.buffer[56] = ((self.data_bit_len as u64 >> 56) & 0xff) as u8;
                F8(self);
        }

        // /*truncating the final hash value to generate the message digest*/
        // switch(state->hashbitlen) {
        //         case 224: memcpy(hashval,(unsigned char*)state->x+64+36,28);  break;
        //         case 256: memcpy(hashval,(unsigned char*)state->x+64+32,32);  break;
        //         case 384: memcpy(hashval,(unsigned char*)state->x+64+16,48);  break;
        //         case 512: memcpy(hashval,(unsigned char*)state->x+64,64);     break;
        // }
        unsafe {
            let x: &[u8; 128] = std::mem::transmute(&self.state);
            match self.hash_bit_len {
                HashBitLen::L224 => hashval[..].clone_from_slice( &x[64+36..64+36] ),
                HashBitLen::L256 => hashval[..].clone_from_slice( &x[64+32..64+32+32] ),
                HashBitLen::L384 => hashval[..].clone_from_slice( &x[64+16..64+16+48] ),
                HashBitLen::L512 => hashval[..].clone_from_slice( &x[64..64+64] ),
            }
        }

    }
}

/// The compression function F8
fn F8(state: &mut HashState) {

    /*xor the 512-bit message with the fist half of the 1024-bit hash state*/
    // for (i = 0; i < 8; i++)  state->x[i >> 1][i & 1] ^= ((uint64*)state->buffer)[i];
    unsafe {
        for i in 0..8 {
            let buffer_ref: &[u64; 8] = std::mem::transmute(&state.buffer);
            state.state[i >> 1][i & 1] ^= buffer_ref[i];
        }
    // }


        /*the bijective function E8 */
        E8(state);

    /*xor the 512-bit message with the second half of the 1024-bit hash state*/
    // for (i = 0; i < 8; i++)  state->x[(8+i) >> 1][(8+i) & 1] ^= ((uint64*)state->buffer)[i];
    // unsafe {
        for i in 0..8 {
            let buffer_ref: &[u64; 8] = std::mem::transmute(&state.buffer);
            state.state[(8+i) >> 1][(8+i) & 1] ^= buffer_ref[i];
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum JhError {
    /// Generic failure state
    Fail,
    /// `hashbitlen` or `hash()` incorrect
    BadHashbitlen,
}

#[repr(u16)]
pub enum HashBitLen {
    // switch(state->hashbitlen) {
	// 		case 224: memcpy(hashval,(unsigned char*)state->x+64+36,28);  break;
	// 		case 256: memcpy(hashval,(unsigned char*)state->x+64+32,32);  break;
	// 		case 384: memcpy(hashval,(unsigned char*)state->x+64+16,48);  break;
	// 		case 512: memcpy(hashval,(unsigned char*)state->x+64,64);     break;
    //   }
    L224 = 224,
    L256 = 256,
    L384 = 384,
    L512 = 512,
}

impl TryFrom<u16> for HashBitLen {
    type Error = JhError;

    fn try_from(value: u16) -> Result<HashBitLen, Self::Error> {
        match value {
            224 => Ok(HashBitLen::L224),
            256 => Ok(HashBitLen::L256),
            384 => Ok(HashBitLen::L384),
            512 => Ok(HashBitLen::L512),
            _ => Err(JhError::BadHashbitlen),
        }
    }

}