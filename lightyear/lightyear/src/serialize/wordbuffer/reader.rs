use std::num::NonZeroUsize;

use anyhow::Context;
use bitcode::buffer::BufferTrait;
use bitcode::encoding::{Encoding, Fixed};
use bitcode::read::Read;
use bitcode::word::Word;
use bitcode::word_buffer::{WordBuffer, WordContext, WordReader};
use bitcode::Decode;
use self_cell::self_cell;
use serde::de::DeserializeOwned;

use crate::serialize::reader::{BitRead, ReadBuffer};

#[derive(Default)]
pub struct Reader<'a>(Option<(WordReader<'a>, WordContext)>);

#[derive(Decode)]
// #[bitcode_hint(gamma)]
struct OnlyGammaDecode<T: DeserializeOwned>(#[bitcode(with_serde)] T);

// We use self_cell because the reader contains a reference to the WordBuffer
// (it will take ownership of the buffer's contents to write into)
self_cell!(
    pub struct ReadWordBuffer {
        owner: WordBuffer,

        #[covariant]
        // reader contains a reference to the buffer
        dependent: Reader,
    }
);

impl ReadBuffer for ReadWordBuffer {
    fn capacity(&self) -> usize {
        self.borrow_owner().capacity()
    }

    // fn deserialize<T: DeserializeOwned>(&mut self) -> anyhow::Result<T> {
    //     self.with_dependent_mut(|buffer, reader| {
    //         let reader = reader
    //             .0
    //             .as_mut()
    //             .map_or_else(|| panic!("no reader"), |(reader, _)| reader);
    //         deserialize_compat(Fixed, reader).context("error deserializing")
    //     })
    // }

    fn deserialize<T: DeserializeOwned>(&mut self) -> anyhow::Result<T> {
        self.with_dependent_mut(|_buffer, reader| {
            let reader = reader
                .0
                .as_mut()
                .map_or_else(|| panic!("no reader"), |(reader, _)| reader);
            let with_gamma =
                OnlyGammaDecode::<T>::decode(Fixed, reader).context("error deserializing")?;
            Ok(with_gamma.0)
        })
    }

    fn decode<T: Decode>(&mut self, encoding: impl Encoding) -> anyhow::Result<T> {
        self.with_dependent_mut(|_buffer, reader| {
            let reader = reader
                .0
                .as_mut()
                .map_or_else(|| panic!("no reader"), |(reader, _)| reader);
            T::decode(encoding, reader).context("error decoding")
        })
    }

    fn start_read(bytes: &[u8]) -> Self {
        ReadWordBuffer::new(WordBuffer::with_capacity(bytes.len()), |buffer| {
            // safety: we just created the buffer and nothing else had access to it
            // we need to get a mutable reference to the buffer to take ownership of it
            let mut_buffer: &mut WordBuffer;
            unsafe {
                let const_ptr = buffer as *const WordBuffer;
                let mut_ptr = const_ptr.cast_mut();
                mut_buffer = &mut *mut_ptr;
            }
            let (reader, context) = mut_buffer.start_read(bytes);
            Reader(Some((reader, context)))
        })
    }

    fn finish_read(&mut self) -> anyhow::Result<()> {
        self.with_dependent_mut(|_buffer, reader| {
            let (reader, context) = std::mem::take(reader).0.context("no reader")?;
            WordBuffer::finish_read(reader, context).context("error finishing read")
        })
    }
}

impl BitRead for ReadWordBuffer {
    fn advance(&mut self, _bits: usize) {
        todo!()
    }

    fn peek_bits(&mut self) -> anyhow::Result<Word> {
        todo!()
    }

    fn read_bit(&mut self) -> anyhow::Result<bool> {
        todo!()
    }

    fn read_bits(&mut self, _bits: usize) -> anyhow::Result<Word> {
        todo!()
    }

    fn read_bytes(&mut self, len: NonZeroUsize) -> anyhow::Result<&[u8]> {
        self.with_dependent_mut(|_buffer, reader| {
            let reader = reader
                .0
                .as_mut()
                .map_or_else(|| panic!("no reader"), |(reader, _)| reader);
            reader.read_bytes(len).context("error reading bytes")
        })
    }

    fn reserve_bits(&self, _bits: usize) -> anyhow::Result<()> {
        todo!()
    }
}
