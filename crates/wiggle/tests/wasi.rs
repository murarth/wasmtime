use wiggle::{GuestBorrows, GuestError, GuestErrorType, GuestPtr};
use wiggle_test::WasiCtx;

// This test file exists to make sure that the entire `wasi.witx` file can be
// handled by wiggle, producing code that compiles correctly.
// The trait impls here are never executed, and just exist to validate that the
// witx is exposed with the type signatures that we expect.

wiggle::from_witx!({
    witx: ["tests/wasi.witx"],
    ctx: WasiCtx,
});

// The only test in this file is to verify that the witx document provided by the
// proc macro in the `metadata` module is equal to the document on the disk.
#[test]
fn document_equivelant() {
    let macro_doc = metadata::document();
    let disk_doc = witx::load(&["tests/wasi.witx"]).expect("load wasi.witx from disk");

    assert_eq!(macro_doc, disk_doc);
}

type Result<T> = std::result::Result<T, types::Errno>;

impl GuestErrorType for types::Errno {
    fn success() -> types::Errno {
        types::Errno::Success
    }
}

impl<'a> types::GuestErrorConversion for WasiCtx<'a> {
    fn into_errno(&self, e: GuestError) -> types::Errno {
        eprintln!("GuestError {:?}", e);
        types::Errno::Badf
    }
}

impl<'a> crate::wasi_snapshot_preview1::WasiSnapshotPreview1 for WasiCtx<'a> {
    fn args_get(&self, _argv: &GuestPtr<GuestPtr<u8>>, _argv_buf: &GuestPtr<u8>) -> Result<()> {
        unimplemented!("args_get")
    }

    fn args_sizes_get(&self) -> Result<(types::Size, types::Size)> {
        unimplemented!("args_sizes_get")
    }

    fn environ_get(
        &self,
        _environ: &GuestPtr<GuestPtr<u8>>,
        _environ_buf: &GuestPtr<u8>,
    ) -> Result<()> {
        unimplemented!("environ_get")
    }

    fn environ_sizes_get(&self) -> Result<(types::Size, types::Size)> {
        unimplemented!("environ_sizes_get")
    }

    fn clock_res_get(&self, _id: types::Clockid) -> Result<types::Timestamp> {
        unimplemented!("clock_res_get")
    }

    fn clock_time_get(
        &self,
        _id: types::Clockid,
        _precision: types::Timestamp,
    ) -> Result<types::Timestamp> {
        unimplemented!("clock_time_get")
    }

    fn fd_advise(
        &self,
        _fd: types::Fd,
        _offset: types::Filesize,
        _len: types::Filesize,
        _advice: types::Advice,
    ) -> Result<()> {
        unimplemented!("fd_advise")
    }

    fn fd_allocate(
        &self,
        _fd: types::Fd,
        _offset: types::Filesize,
        _len: types::Filesize,
    ) -> Result<()> {
        unimplemented!("fd_allocate")
    }

    fn fd_close(&self, _fd: types::Fd) -> Result<()> {
        unimplemented!("fd_close")
    }

    fn fd_datasync(&self, _fd: types::Fd) -> Result<()> {
        unimplemented!("fd_datasync")
    }

    fn fd_fdstat_get(&self, _fd: types::Fd) -> Result<types::Fdstat> {
        unimplemented!("fd_fdstat_get")
    }

    fn fd_fdstat_set_flags(&self, _fd: types::Fd, _flags: types::Fdflags) -> Result<()> {
        unimplemented!("fd_fdstat_set_flags")
    }

    fn fd_fdstat_set_rights(
        &self,
        _fd: types::Fd,
        _fs_rights_base: types::Rights,
        _fs_rights_inherting: types::Rights,
    ) -> Result<()> {
        unimplemented!("fd_fdstat_set_rights")
    }

    fn fd_filestat_get(&self, _fd: types::Fd) -> Result<types::Filestat> {
        unimplemented!("fd_filestat_get")
    }

    fn fd_filestat_set_size(&self, _fd: types::Fd, _size: types::Filesize) -> Result<()> {
        unimplemented!("fd_filestat_set_size")
    }

    fn fd_filestat_set_times(
        &self,
        _fd: types::Fd,
        _atim: types::Timestamp,
        _mtim: types::Timestamp,
        _fst_flags: types::Fstflags,
    ) -> Result<()> {
        unimplemented!("fd_filestat_set_times")
    }

    fn fd_pread(
        &self,
        _fd: types::Fd,
        iovs: &types::IovecArray<'_>,
        _offset: types::Filesize,
    ) -> Result<types::Size> {
        // This is not functional code, but the type annotations demonstrate
        // that we can use the wiggle API to create the datastructures we want
        // for efficient implementation of this function elsewhere.

        let mut bc = GuestBorrows::new();
        let mut slices: Vec<&'_ mut [u8]> = Vec::new();
        // Mark the iov elements as borrowed, to ensure that they does not
        // overlap with any of the as_raw regions.
        bc.borrow_slice(&iovs).expect("borrow iovec array");
        for iov_ptr in iovs.iter() {
            let iov_ptr = iov_ptr.expect("iovec element pointer is valid");

            let iov: types::Iovec = iov_ptr.read().expect("read iovec element");
            let base: GuestPtr<u8> = iov.buf;
            let len: u32 = iov.buf_len;
            let buf: GuestPtr<[u8]> = base.as_array(len);
            let slice = buf.as_raw(&mut bc).expect("borrow slice from iovec");
            slices.push(unsafe { &mut *slice });
        }
        println!("iovec slices: {:?}", slices);
        unimplemented!("fd_pread")
    }

    fn fd_prestat_get(&self, _fd: types::Fd) -> Result<types::Prestat> {
        unimplemented!("fd_prestat_get")
    }

    fn fd_prestat_dir_name(
        &self,
        _fd: types::Fd,
        _path: &GuestPtr<u8>,
        _path_len: types::Size,
    ) -> Result<()> {
        unimplemented!("fd_prestat_dir_name")
    }

    fn fd_pwrite(
        &self,
        _fd: types::Fd,
        _ciovs: &types::CiovecArray<'_>,
        _offset: types::Filesize,
    ) -> Result<types::Size> {
        unimplemented!("fd_pwrite")
    }

    fn fd_read(&self, _fd: types::Fd, _iovs: &types::IovecArray<'_>) -> Result<types::Size> {
        unimplemented!("fd_read")
    }

    fn fd_readdir(
        &self,
        _fd: types::Fd,
        _buf: &GuestPtr<u8>,
        _buf_len: types::Size,
        _cookie: types::Dircookie,
    ) -> Result<types::Size> {
        unimplemented!("fd_readdir")
    }

    fn fd_renumber(&self, _fd: types::Fd, _to: types::Fd) -> Result<()> {
        unimplemented!("fd_renumber")
    }

    fn fd_seek(
        &self,
        _fd: types::Fd,
        _offset: types::Filedelta,
        _whence: types::Whence,
    ) -> Result<types::Filesize> {
        unimplemented!("fd_seek")
    }

    fn fd_sync(&self, _fd: types::Fd) -> Result<()> {
        unimplemented!("fd_sync")
    }

    fn fd_tell(&self, _fd: types::Fd) -> Result<types::Filesize> {
        unimplemented!("fd_tell")
    }

    fn fd_write(&self, _fd: types::Fd, _ciovs: &types::CiovecArray<'_>) -> Result<types::Size> {
        unimplemented!("fd_write")
    }

    fn path_create_directory(&self, _fd: types::Fd, _path: &GuestPtr<'_, str>) -> Result<()> {
        unimplemented!("path_create_directory")
    }

    fn path_filestat_get(
        &self,
        _fd: types::Fd,
        _flags: types::Lookupflags,
        _path: &GuestPtr<'_, str>,
    ) -> Result<types::Filestat> {
        unimplemented!("path_filestat_get")
    }

    fn path_filestat_set_times(
        &self,
        _fd: types::Fd,
        _flags: types::Lookupflags,
        _path: &GuestPtr<'_, str>,
        _atim: types::Timestamp,
        _mtim: types::Timestamp,
        _fst_flags: types::Fstflags,
    ) -> Result<()> {
        unimplemented!("path_filestat_set_times")
    }

    fn path_link(
        &self,
        _old_fd: types::Fd,
        _old_flags: types::Lookupflags,
        _old_path: &GuestPtr<'_, str>,
        _new_fd: types::Fd,
        _new_path: &GuestPtr<'_, str>,
    ) -> Result<()> {
        unimplemented!("path_link")
    }

    fn path_open(
        &self,
        _fd: types::Fd,
        _dirflags: types::Lookupflags,
        _path: &GuestPtr<'_, str>,
        _oflags: types::Oflags,
        _fs_rights_base: types::Rights,
        _fs_rights_inherting: types::Rights,
        _fdflags: types::Fdflags,
    ) -> Result<types::Fd> {
        unimplemented!("path_open")
    }

    fn path_readlink(
        &self,
        _fd: types::Fd,
        _path: &GuestPtr<'_, str>,
        _buf: &GuestPtr<u8>,
        _buf_len: types::Size,
    ) -> Result<types::Size> {
        unimplemented!("path_readlink")
    }

    fn path_remove_directory(&self, _fd: types::Fd, _path: &GuestPtr<'_, str>) -> Result<()> {
        unimplemented!("path_remove_directory")
    }

    fn path_rename(
        &self,
        _fd: types::Fd,
        _old_path: &GuestPtr<'_, str>,
        _new_fd: types::Fd,
        _new_path: &GuestPtr<'_, str>,
    ) -> Result<()> {
        unimplemented!("path_rename")
    }

    fn path_symlink(
        &self,
        _old_path: &GuestPtr<'_, str>,
        _fd: types::Fd,
        _new_path: &GuestPtr<'_, str>,
    ) -> Result<()> {
        unimplemented!("path_symlink")
    }

    fn path_unlink_file(&self, _fd: types::Fd, _path: &GuestPtr<'_, str>) -> Result<()> {
        unimplemented!("path_unlink_file")
    }

    fn poll_oneoff(
        &self,
        _in_: &GuestPtr<types::Subscription>,
        _out: &GuestPtr<types::Event>,
        _nsubscriptions: types::Size,
    ) -> Result<types::Size> {
        unimplemented!("poll_oneoff")
    }

    fn proc_exit(&self, _rval: types::Exitcode) -> std::result::Result<(), ()> {
        unimplemented!("proc_exit")
    }

    fn proc_raise(&self, _sig: types::Signal) -> Result<()> {
        unimplemented!("proc_raise")
    }

    fn sched_yield(&self) -> Result<()> {
        unimplemented!("sched_yield")
    }

    fn random_get(&self, _buf: &GuestPtr<u8>, _buf_len: types::Size) -> Result<()> {
        unimplemented!("random_get")
    }

    fn sock_recv(
        &self,
        _fd: types::Fd,
        _ri_data: &types::IovecArray<'_>,
        _ri_flags: types::Riflags,
    ) -> Result<(types::Size, types::Roflags)> {
        unimplemented!("sock_recv")
    }

    fn sock_send(
        &self,
        _fd: types::Fd,
        _si_data: &types::CiovecArray<'_>,
        _si_flags: types::Siflags,
    ) -> Result<types::Size> {
        unimplemented!("sock_send")
    }

    fn sock_shutdown(&self, _fd: types::Fd, _how: types::Sdflags) -> Result<()> {
        unimplemented!("sock_shutdown")
    }
}
