use criterion::{criterion_group, criterion_main, Criterion};

use scalpel::layers::dns::DNS;
use scalpel::register_defaults;
use scalpel::Layer;
use scalpel::Packet;
use scalpel::ENCAP_TYPE_ETH;

pub fn new_dns_packet_from_u8(c: &mut Criterion) {
    // Benchmark Taken from the 'gopacket' Go package.
    let dns_query = vec![
	0xfe, 0x54, 0x00, 0x3e, 0x00, 0x96, 0x52, 0x54, /* .T.>..RT */
	0x00, 0xbd, 0x1c, 0x70, 0x08, 0x00, 0x45, 0x00, /* ...p..E. */
	0x00, 0x3c, 0x22, 0xe0, 0x00, 0x00, 0x40, 0x11, /* .<"...@. */
	0xe2, 0x38, 0xc0, 0xa8, 0x7a, 0x46, 0xc0, 0xa8, /* .8..zF.. */
	0x7a, 0x01, 0xc3, 0x35, 0x00, 0x35, 0x00, 0x28, /* z..5.5.( */
	0x75, 0xd2, 0x52, 0x41, 0x01, 0x00, 0x00, 0x01, /* u.RA.... */
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x77, /* .......w */
	0x77, 0x77, 0x06, 0x67, 0x6f, 0x6f, 0x67, 0x6c, /* ww.googl */
	0x65, 0x03, 0x63, 0x6f, 0x6d, 0x00, 0x00, 0x01, /* e.com... */
	0x00, 0x01, /* .. */];
    let _ = register_defaults();
    c.bench_function("Parse_DNS", |b| {
        b.iter(|| Packet::from_u8(&dns_query, ENCAP_TYPE_ETH))
    });
}

pub fn new_dns_aaaa_from_u8(c: &mut Criterion) {
    // Benchmark Taken from the 'gopacket' Go package.
    let test_dns_aaaa = vec![
	0x52, 0x54, 0x00, 0xbd, 0x1c, 0x70, 0xfe, 0x54, /* RT...p.T */
	0x00, 0x3e, 0x00, 0x96, 0x08, 0x00, 0x45, 0x00, /* .>....E. */
	0x00, 0xe0, 0x00, 0x00, 0x40, 0x00, 0x40, 0x11, /* ....@.@. */
	0xc4, 0x74, 0xc0, 0xa8, 0x7a, 0x01, 0xc0, 0xa8, /* .t..z... */
	0x7a, 0x46, 0x00, 0x35, 0xdb, 0x13, 0x00, 0xcc, /* zF.5.... */
	0x76, 0x76, 0xf3, 0x03, 0x81, 0x80, 0x00, 0x01, /* vv...... */
	0x00, 0x01, 0x00, 0x04, 0x00, 0x04, 0x03, 0x77, /* .......w */
	0x77, 0x77, 0x06, 0x67, 0x6f, 0x6f, 0x67, 0x6c, /* ww.googl */
	0x65, 0x03, 0x63, 0x6f, 0x6d, 0x00, 0x00, 0x1c, /* e.com... */
	0x00, 0x01, 0xc0, 0x0c, 0x00, 0x1c, 0x00, 0x01, /* ........ */
	0x00, 0x00, 0x01, 0x2c, 0x00, 0x10, 0x2a, 0x00, /* ...,..*. */
	0x14, 0x50, 0x40, 0x0c, 0x0c, 0x01, 0x00, 0x00, /* .P@..... */
	0x00, 0x00, 0x00, 0x00, 0x00, 0x69, 0xc0, 0x10, /* .....i.. */
	0x00, 0x02, 0x00, 0x01, 0x00, 0x02, 0xa3, 0x00, /* ........ */
	0x00, 0x06, 0x03, 0x6e, 0x73, 0x34, 0xc0, 0x10, /* ...ns4.. */
	0xc0, 0x10, 0x00, 0x02, 0x00, 0x01, 0x00, 0x02, /* ........ */
	0xa3, 0x00, 0x00, 0x06, 0x03, 0x6e, 0x73, 0x32, /* .....ns2 */
	0xc0, 0x10, 0xc0, 0x10, 0x00, 0x02, 0x00, 0x01, /* ........ */
	0x00, 0x02, 0xa3, 0x00, 0x00, 0x06, 0x03, 0x6e, /* .......n */
	0x73, 0x31, 0xc0, 0x10, 0xc0, 0x10, 0x00, 0x02, /* s1...... */
	0x00, 0x01, 0x00, 0x02, 0xa3, 0x00, 0x00, 0x06, /* ........ */
	0x03, 0x6e, 0x73, 0x33, 0xc0, 0x10, 0xc0, 0x6c, /* .ns3...l */
	0x00, 0x01, 0x00, 0x01, 0x00, 0x02, 0xa3, 0x00, /* ........ */
	0x00, 0x04, 0xd8, 0xef, 0x20, 0x0a, 0xc0, 0x5a, /* .... ..Z */
	0x00, 0x01, 0x00, 0x01, 0x00, 0x02, 0xa3, 0x00, /* ........ */
	0x00, 0x04, 0xd8, 0xef, 0x22, 0x0a, 0xc0, 0x7e, /* ...."..~ */
	0x00, 0x01, 0x00, 0x01, 0x00, 0x02, 0xa3, 0x00, /* ........ */
	0x00, 0x04, 0xd8, 0xef, 0x24, 0x0a, 0xc0, 0x48, /* ....$..H */
	0x00, 0x01, 0x00, 0x01, 0x00, 0x02, 0xa3, 0x00, /* ........ */
	0x00, 0x04, 0xd8, 0xef, 0x26, 0x0a, /* ....&. */
    ];
    let _ = register_defaults();
    c.bench_function("Parse_DNS_AAAA", |b| {
        let mut dns: Box<dyn Layer> = Box::new(DNS::default());
        b.iter(|| dns.from_u8(&test_dns_aaaa[42..]));
    });
}

pub fn bench_dns_gopacket_regression(c: &mut Criterion) {
    // Benchmark Taken from the 'gopacket' Go package.
    //
    // testPacketDNSRegression is the packet:
    //   11:08:05.708342 IP 109.194.160.4.57766 > 95.211.92.14.53: 63000% [1au] A? picslife.ru. (40)
    //      0x0000:  0022 19b6 7e22 000f 35bb 0b40 0800 4500  ."..~"..5..@..E.
    //      0x0010:  0044 89c4 0000 3811 2f3d 6dc2 a004 5fd3  .D....8./=m..._.
    //      0x0020:  5c0e e1a6 0035 0030 a597 f618 0010 0001  \....5.0........
    //      0x0030:  0000 0000 0001 0870 6963 736c 6966 6502  .......picslife.
    //      0x0040:  7275 0000 0100 0100 0029 1000 0000 8000  ru.......)......
    //      0x0050:  0000                                     ..
    let test_packet_dns_regression = vec![
        0x00, 0x22, 0x19, 0xb6, 0x7e, 0x22, 0x00, 0x0f, 0x35, 0xbb, 0x0b, 0x40, 0x08, 0x00, 0x45,
        0x00, 0x00, 0x44, 0x89, 0xc4, 0x00, 0x00, 0x38, 0x11, 0x2f, 0x3d, 0x6d, 0xc2, 0xa0, 0x04,
        0x5f, 0xd3, 0x5c, 0x0e, 0xe1, 0xa6, 0x00, 0x35, 0x00, 0x30, 0xa5, 0x97, 0xf6, 0x18, 0x00,
        0x10, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x08, 0x70, 0x69, 0x63, 0x73, 0x6c,
        0x69, 0x66, 0x65, 0x02, 0x72, 0x75, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x29, 0x10,
        0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00,
    ];
    let _ = register_defaults();
    c.bench_function("Parse_DNS_Regression_Packet", |b| {
        b.iter(|| Packet::from_u8(&test_packet_dns_regression, ENCAP_TYPE_ETH))
    });
}

pub fn bench_dns_serde_json(c: &mut Criterion) {
    // Benchmark Packet taken from 'gopacket'
    let test_dns_aaaa = vec![
	0x52, 0x54, 0x00, 0xbd, 0x1c, 0x70, 0xfe, 0x54, /* RT...p.T */
	0x00, 0x3e, 0x00, 0x96, 0x08, 0x00, 0x45, 0x00, /* .>....E. */
	0x00, 0xe0, 0x00, 0x00, 0x40, 0x00, 0x40, 0x11, /* ....@.@. */
	0xc4, 0x74, 0xc0, 0xa8, 0x7a, 0x01, 0xc0, 0xa8, /* .t..z... */
	0x7a, 0x46, 0x00, 0x35, 0xdb, 0x13, 0x00, 0xcc, /* zF.5.... */
	0x76, 0x76, 0xf3, 0x03, 0x81, 0x80, 0x00, 0x01, /* vv...... */
	0x00, 0x01, 0x00, 0x04, 0x00, 0x04, 0x03, 0x77, /* .......w */
	0x77, 0x77, 0x06, 0x67, 0x6f, 0x6f, 0x67, 0x6c, /* ww.googl */
	0x65, 0x03, 0x63, 0x6f, 0x6d, 0x00, 0x00, 0x1c, /* e.com... */
	0x00, 0x01, 0xc0, 0x0c, 0x00, 0x1c, 0x00, 0x01, /* ........ */
	0x00, 0x00, 0x01, 0x2c, 0x00, 0x10, 0x2a, 0x00, /* ...,..*. */
	0x14, 0x50, 0x40, 0x0c, 0x0c, 0x01, 0x00, 0x00, /* .P@..... */
	0x00, 0x00, 0x00, 0x00, 0x00, 0x69, 0xc0, 0x10, /* .....i.. */
	0x00, 0x02, 0x00, 0x01, 0x00, 0x02, 0xa3, 0x00, /* ........ */
	0x00, 0x06, 0x03, 0x6e, 0x73, 0x34, 0xc0, 0x10, /* ...ns4.. */
	0xc0, 0x10, 0x00, 0x02, 0x00, 0x01, 0x00, 0x02, /* ........ */
	0xa3, 0x00, 0x00, 0x06, 0x03, 0x6e, 0x73, 0x32, /* .....ns2 */
	0xc0, 0x10, 0xc0, 0x10, 0x00, 0x02, 0x00, 0x01, /* ........ */
	0x00, 0x02, 0xa3, 0x00, 0x00, 0x06, 0x03, 0x6e, /* .......n */
	0x73, 0x31, 0xc0, 0x10, 0xc0, 0x10, 0x00, 0x02, /* s1...... */
	0x00, 0x01, 0x00, 0x02, 0xa3, 0x00, 0x00, 0x06, /* ........ */
	0x03, 0x6e, 0x73, 0x33, 0xc0, 0x10, 0xc0, 0x6c, /* .ns3...l */
	0x00, 0x01, 0x00, 0x01, 0x00, 0x02, 0xa3, 0x00, /* ........ */
	0x00, 0x04, 0xd8, 0xef, 0x20, 0x0a, 0xc0, 0x5a, /* .... ..Z */
	0x00, 0x01, 0x00, 0x01, 0x00, 0x02, 0xa3, 0x00, /* ........ */
	0x00, 0x04, 0xd8, 0xef, 0x22, 0x0a, 0xc0, 0x7e, /* ...."..~ */
	0x00, 0x01, 0x00, 0x01, 0x00, 0x02, 0xa3, 0x00, /* ........ */
	0x00, 0x04, 0xd8, 0xef, 0x24, 0x0a, 0xc0, 0x48, /* ....$..H */
	0x00, 0x01, 0x00, 0x01, 0x00, 0x02, 0xa3, 0x00, /* ........ */
	0x00, 0x04, 0xd8, 0xef, 0x26, 0x0a, /* ....&. */
    ];
    let _ = register_defaults();
    c.bench_function("Json_Serialize_DNS_AAAA", |b| {
        b.iter(|| {
            let p = Packet::from_u8(&test_dns_aaaa, ENCAP_TYPE_ETH).unwrap();
            let _ = serde_json::to_string(&p).unwrap();
        })
    });
}

criterion_group!(
    dns,
    // Note: Following benchmark runs slower then corresponding 'gopacket' benchmark. However, the
    // benchmarks are not apples to apples comparison. In the 'gopacket' benchmark version the DNS
    // struct is (and internal vectors are) allocated only once per benchmark. That is not the
    // case. That makes the 'gopacket' benchmark look faster. But if we run the 'full' packet
    // decode benchmark our benchmark is fast as seen with other benchmarks.
    new_dns_aaaa_from_u8,
    new_dns_packet_from_u8,
    bench_dns_gopacket_regression,
    bench_dns_serde_json
);
criterion_main!(dns);
