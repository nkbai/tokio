#![allow(unreachable_pub)] // https://github.com/rust-lang/rust/issues/57411

#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
mod async_buf_read_ext;

#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
pub use async_buf_read_ext::AsyncBufReadExt;
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
mod async_read_ext;
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
pub use async_read_ext::AsyncReadExt;
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
mod async_seek_ext;
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
pub use async_seek_ext::AsyncSeekExt;
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
mod async_write_ext;
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
pub use async_write_ext::AsyncWriteExt;
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
mod buf_reader;
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
pub use buf_reader::BufReader;
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
mod buf_stream;
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
pub use buf_stream::BufStream;
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
mod buf_writer;
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
pub use buf_writer::BufWriter;
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
mod chain;
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
mod copy;
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
pub use copy::{copy, Copy};
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
mod empty;
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
pub use empty::{empty, Empty};
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
mod flush;
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
mod lines;
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
pub use lines::Lines;
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
mod read;
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
mod read_buf;
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
mod read_exact;
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
mod read_int;
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
mod read_line;
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
mod read_to_end;
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
#[cfg(feature = "process")]
#[cfg_attr(docsrs, doc(cfg(feature = "process")))]
#[cfg(not(loom))]
pub(crate) use read_to_end::read_to_end;

#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
mod read_to_string;
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
mod read_until;
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
mod repeat;
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
pub use repeat::{repeat, Repeat};
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
mod shutdown;
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
mod sink;
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
pub use sink::{sink, Sink};
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
mod split;
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
pub use split::Split;
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
mod take;
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
pub use take::Take;
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
mod write;
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
mod write_all;
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
mod write_buf;
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
mod write_int;
// used by `BufReader` and `BufWriter`
// https://github.com/rust-lang/rust/blob/master/src/libstd/sys_common/io.rs#L1
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
const DEFAULT_BUF_SIZE: usize = 8 * 1024;

//cfg_not_io_util! {
//    cfg_process! {
//        mod read_to_end;
//        // Used by process
//        pub(crate) use read_to_end::read_to_end;
//    }
//}
