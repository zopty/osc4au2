use aviutl2::{AnyResult, lprintln, module::ScriptModuleFunctions};
use rosc::{OscPacket, OscType, decoder};
use std::{
    collections::HashMap,
    net::UdpSocket,
    sync::{Arc, OnceLock, RwLock},
    thread,
};

type SharedMap = Arc<RwLock<HashMap<String, Vec<OscType>>>>;

static OSC_THREAD_INIT: OnceLock<()> = OnceLock::new();

fn start_osc_receiver(shared: SharedMap) {
    OSC_THREAD_INIT.get_or_init(|| {
        let shared = shared.clone();

        thread::spawn(move || {
            let socket = UdpSocket::bind("0.0.0.0:9000").expect("bind failed");

            lprintln!("OSC Listening on 9000");

            let mut buf = [0u8; rosc::decoder::MTU];

            loop {
                let (size, _) = match socket.recv_from(&mut buf) {
                    Ok(v) => v,
                    Err(e) => {
                        let _ = aviutl2::logger::write_error_log(&format!("recv error: {e}"));
                        continue;
                    }
                };

                let packet = match decoder::decode_udp(&buf[..size]) {
                    Ok((_rem, packet)) => packet,
                    Err(e) => {
                        let _ = aviutl2::logger::write_error_log(&format!("decode error: {e}"));
                        continue;
                    }
                };

                handle_packet(packet, &shared);
            }
        });
    });
}

fn handle_packet(packet: OscPacket, shared: &SharedMap) {
    match packet {
        OscPacket::Message(msg) => {
            let mut map = shared.write().unwrap();

            map.insert(msg.addr, msg.args);
        }

        OscPacket::Bundle(bundle) => {
            for p in bundle.content {
                handle_packet(p, shared);
            }
        }
    }
}

#[aviutl2::plugin(ScriptModule)]
struct OscReceiverModule {
    shared: SharedMap,
}

impl aviutl2::module::ScriptModule for OscReceiverModule {
    fn new(_info: aviutl2::common::AviUtl2Info) -> AnyResult<Self> {
        let shared = SharedMap::default();

        start_osc_receiver(shared.clone());

        Ok(Self { shared })
    }

    fn plugin_info(&self) -> aviutl2::module::ScriptModuleTable {
        aviutl2::module::ScriptModuleTable {
            information: format!(
                "OSC Receiver for AviUtl2 | v{version}",
                version = env!("CARGO_PKG_VERSION")
            ),
            functions: Self::functions(),
        }
    }
}

#[aviutl2::module::functions]
impl OscReceiverModule {
    #[direct]
    fn get(&self, params: &mut aviutl2::module::ScriptModuleCallHandle) {
        let addr = params.get_param_str(0).unwrap_or_default();
        if addr.is_empty() {
            return;
        }
        if let Ok(map) = self.shared.read() {
            if let Some(values) = map.get(&addr).cloned() {
                let parsed_values: Vec<f64> = values
                    .iter()
                    .filter_map(|v| match v {
                        OscType::Float(f) => Some((*f).into()),
                        _ => None,
                    })
                    .collect();
                params
                    .push_result_array_float(&parsed_values)
                    .expect("Unable to push values");
            }
        }
    }
}

aviutl2::register_script_module!(OscReceiverModule);
