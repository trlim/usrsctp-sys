#![allow(non_camel_case_types)]

extern crate libc;

use libc::{c_char, c_int, c_uint, c_void, size_t, ssize_t, uint8_t, uint16_t, uint32_t};
use libc::{socklen_t};
use libc::{sockaddr};

// to avoid improper_ctypes warning
pub enum socket {}

pub type sctp_assoc_t = uint32_t;

#[repr(C)]
pub struct sctp_rcvinfo {
	rcv_sid: uint16_t,
	rcv_ssn: uint16_t,
	rcv_flags: uint16_t,
	rcv_ppid: uint32_t,
	rcv_tsn: uint32_t,
	rcv_cumtsn: uint32_t,
	rcv_context: uint32_t,
	rcv_assoc_id: sctp_assoc_t
}

// union sctp_sockstore {
// #if defined(INET)
// 	struct sockaddr_in sin;
// #endif
// #if defined(INET6)
// 	struct sockaddr_in6 sin6;
// #endif
// 	struct sockaddr_conn sconn;
// 	struct sockaddr sa;
// };
pub type sctp_sockstore = sockaddr;

pub type usrsctp_receive_cb = extern "C" fn(
    sock: *mut socket,
    addr: sctp_sockstore,
    data: *mut c_void,
    datalen: size_t,
    rcvinfo: sctp_rcvinfo,
    flags: c_int) -> c_int;

pub type usrsctp_send_cb = extern "C" fn(sock: *mut socket, sb_free: uint32_t) -> c_int;

extern "C" {
    pub fn usrsctp_init(
        udp_port: uint16_t,
        conn_output:
            Option<extern "C" fn(
                addr: *mut c_void,
                buffer: *mut c_void,
                length: size_t,
                tos: uint8_t,
                set_df: uint8_t) -> c_int>,
        debug_printf: Option<extern "C" fn(format: *const c_char, ...)>
    );
    pub fn usrsctp_finish() -> c_int;

    pub fn usrsctp_socket(
        domain: c_int,
        typ: c_int,
        protocol: c_int,
        receive_cb: usrsctp_receive_cb,
        send_cb: usrsctp_send_cb,
        sb_threshold: uint32_t) -> *mut socket;

    pub fn usrsctp_close(sock: *mut socket);

    pub fn usrsctp_bind(sock: *mut socket, addr: *mut sockaddr, addrlen: socklen_t) -> c_int;
    pub fn usrsctp_listen(sock: *mut socket, backlog: c_int) -> c_int;
    pub fn usrsctp_accept(sock: *mut socket, addr: *mut sockaddr, addrlen: *mut socklen_t) -> *mut socket;
    pub fn usrsctp_connect(sock: *mut socket, name: *mut sockaddr, addrlen: socklen_t) -> c_int;
    pub fn usrsctp_shutdown(sock: *mut socket, how: c_int) -> c_int;

    pub fn usrsctp_sendv(
        so: *mut socket,
        data: *const c_void,
        len: size_t,
        addrs: *const sockaddr,
        addrcnt: c_int,
        info: *mut c_void,
        infolen: socklen_t,
        infotype: c_uint,
        flags: c_int) -> ssize_t;
    pub fn usrsctp_recvv(
        so: *mut socket,
        dbuf: *mut c_void,
        len: size_t,
        from: *mut sockaddr,
        fromlen: *mut socklen_t,
        info: *mut c_void,
        infolen: *mut socklen_t,
        infotype: *mut c_uint,
        msg_flags: *mut c_int) -> ssize_t;

    pub fn usrsctp_getsockopt(
        so: *mut socket,
        level: c_int,
        optname: c_int,
        optval: *mut c_void,
        optlen: *mut socklen_t) -> c_int;
    pub fn usrsctp_setsockopt(
        so: *mut socket,
        level: c_int,
        optname: c_int,
        optval: *const c_void,
        optlen: socklen_t) -> c_int;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        unsafe { usrsctp_init(4802, None, None) };
        assert_eq!(unsafe { usrsctp_finish() }, 0);
    }
}
