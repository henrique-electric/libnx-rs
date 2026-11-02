//! `netinet/tcp.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const TH_FIN: u32 = 0x1;
pub const TH_SYN: u32 = 0x2;
pub const TH_RST: u32 = 0x4;
pub const TH_PUSH: u32 = 0x8;
pub const TH_ACK: u32 = 0x10;
pub const TH_URG: u32 = 0x20;
pub const TH_ECE: u32 = 0x40;
pub const TH_CWR: u32 = 0x80;
pub const TH_FLAGS: u32 = 255;
pub const PRINT_TH_FLAGS: &CStr = c"\x10\x01FIN\x02SYN\x03RST\x04PUSH\x05ACK\x06URG\x07ECE\x08CWR";
pub const TCPOPT_EOL: u32 = 0;
pub const TCPOLEN_EOL: u32 = 1;
pub const TCPOPT_PAD: u32 = 0;
pub const TCPOLEN_PAD: u32 = 1;
pub const TCPOPT_NOP: u32 = 1;
pub const TCPOLEN_NOP: u32 = 1;
pub const TCPOPT_MAXSEG: u32 = 2;
pub const TCPOLEN_MAXSEG: u32 = 4;
pub const TCPOPT_WINDOW: u32 = 3;
pub const TCPOLEN_WINDOW: u32 = 3;
pub const TCPOPT_SACK_PERMITTED: u32 = 4;
pub const TCPOLEN_SACK_PERMITTED: u32 = 2;
pub const TCPOPT_SACK: u32 = 5;
pub const TCPOLEN_SACKHDR: u32 = 2;
pub const TCPOLEN_SACK: u32 = 8;
pub const TCPOPT_TIMESTAMP: u32 = 8;
pub const TCPOLEN_TIMESTAMP: u32 = 10;
pub const TCPOLEN_TSTAMP_APPA: u32 = 12;
pub const TCPOPT_SIGNATURE: u32 = 19;
pub const TCPOLEN_SIGNATURE: u32 = 18;
pub const TCPOPT_FAST_OPEN: u32 = 34;
pub const TCPOLEN_FAST_OPEN_EMPTY: u32 = 2;
pub const TCPOLEN_FAST_OPEN_MIN: u32 = 6;
pub const TCPOLEN_FAST_OPEN_MAX: u32 = 18;
pub const MAX_SACK_BLKS: u32 = 6;
pub const TCP_MAX_SACK: u32 = 4;
pub const TCP_MSS: u32 = 536;
pub const TCP6_MSS: u32 = 1220;
pub const TCP_MINMSS: u32 = 216;
pub const TCP_MAXWIN: u32 = 65535;
pub const TTCP_CLIENT_SND_WND: u32 = 4096;
pub const TCP_MAX_WINSHIFT: u32 = 14;
pub const TCP_MAXBURST: u32 = 4;
pub const TCP_MAXHLEN: u32 = 60;
pub const TCP_MAXOLEN: u64 = 40;
pub const TCP_NODELAY: u32 = 1;
pub const TCP_MAXSEG: u32 = 2;
pub const TCP_NOPUSH: u32 = 4;
pub const TCP_NOOPT: u32 = 8;
pub const TCP_MD5SIG: u32 = 16;
pub const TCP_INFO: u32 = 32;
pub const TCP_CONGESTION: u32 = 64;
pub const TCP_CCALGOOPT: u32 = 65;
pub const TCP_KEEPINIT: u32 = 128;
pub const TCP_KEEPIDLE: u32 = 256;
pub const TCP_KEEPINTVL: u32 = 512;
pub const TCP_KEEPCNT: u32 = 1024;
pub const TCP_FASTOPEN: u32 = 1025;
pub const TCP_PCAP_OUT: u32 = 2048;
pub const TCP_PCAP_IN: u32 = 4096;
pub const TCP_FUNCTION_BLK: u32 = 8192;
pub const TCP_VENDOR: u32 = 0x80000000;
pub const TCP_CA_NAME_MAX: u32 = 16;
pub const TCPI_OPT_TIMESTAMPS: u32 = 0x1;
pub const TCPI_OPT_SACK: u32 = 0x2;
pub const TCPI_OPT_WSCALE: u32 = 0x4;
pub const TCPI_OPT_ECN: u32 = 0x8;
pub const TCPI_OPT_TOE: u32 = 0x10;
pub const TCP_FUNCTION_NAME_LEN_MAX: u32 = 32;

pub type tcp_seq = u_int32_t;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct tcphdr {
    pub th_sport: u_short,
    pub th_dport: u_short,
    pub th_seq: tcp_seq,
    pub th_ack: tcp_seq,
    pub _bitfield_0: [u8; 1],
    pub th_flags: u_char,
    pub th_win: u_short,
    pub th_sum: u_short,
    pub th_urp: u_short,
}

impl tcphdr {
    bitfields!(_bitfield_0 {
        th_x2 / set_th_x2: 0, 4 => u_char;
        th_off / set_th_off: 4, 4 => u_char;
    });
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct tcp_info {
    pub tcpi_state: u_int8_t,
    pub __tcpi_ca_state: u_int8_t,
    pub __tcpi_retransmits: u_int8_t,
    pub __tcpi_probes: u_int8_t,
    pub __tcpi_backoff: u_int8_t,
    pub tcpi_options: u_int8_t,
    pub _bitfield_0: [u8; 1],
    pub tcpi_rto: u_int32_t,
    pub __tcpi_ato: u_int32_t,
    pub tcpi_snd_mss: u_int32_t,
    pub tcpi_rcv_mss: u_int32_t,
    pub __tcpi_unacked: u_int32_t,
    pub __tcpi_sacked: u_int32_t,
    pub __tcpi_lost: u_int32_t,
    pub __tcpi_retrans: u_int32_t,
    pub __tcpi_fackets: u_int32_t,
    pub __tcpi_last_data_sent: u_int32_t,
    pub __tcpi_last_ack_sent: u_int32_t,
    pub tcpi_last_data_recv: u_int32_t,
    pub __tcpi_last_ack_recv: u_int32_t,
    pub __tcpi_pmtu: u_int32_t,
    pub __tcpi_rcv_ssthresh: u_int32_t,
    pub tcpi_rtt: u_int32_t,
    pub tcpi_rttvar: u_int32_t,
    pub tcpi_snd_ssthresh: u_int32_t,
    pub tcpi_snd_cwnd: u_int32_t,
    pub __tcpi_advmss: u_int32_t,
    pub __tcpi_reordering: u_int32_t,
    pub __tcpi_rcv_rtt: u_int32_t,
    pub tcpi_rcv_space: u_int32_t,
    pub tcpi_snd_wnd: u_int32_t,
    pub tcpi_snd_bwnd: u_int32_t,
    pub tcpi_snd_nxt: u_int32_t,
    pub tcpi_rcv_nxt: u_int32_t,
    pub tcpi_toe_tid: u_int32_t,
    pub tcpi_snd_rexmitpack: u_int32_t,
    pub tcpi_rcv_ooopack: u_int32_t,
    pub tcpi_snd_zerowin: u_int32_t,
    pub __tcpi_pad: [u_int32_t; 26],
}

impl tcp_info {
    bitfields!(_bitfield_0 {
        tcpi_snd_wscale / set_tcpi_snd_wscale: 0, 4 => u_int8_t;
        tcpi_rcv_wscale / set_tcpi_rcv_wscale: 4, 4 => u_int8_t;
    });
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct tcp_function_set {
    pub function_set_name: [c_char; 32],
    pub pcbcnt: u32,
}

impl_zeroed_default!(tcphdr, tcp_info, tcp_function_set);
