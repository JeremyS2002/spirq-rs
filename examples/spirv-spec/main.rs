use std::collections::HashMap;
use spirq::SpirvBinary;
use log::info;
use std::path::Path;

fn main() {
    env_logger::init();

    let spvs = collect_spirv_binaries("assets/effects/spirv-spec");
    info!("collected spirvs: {:?}", spvs.iter().map(|x| x.0.as_ref()).collect::<Vec<&str>>());
    let frag = spvs["referential.frag"].reflect_vec().unwrap();
    info!("{:#?}", frag);
    let frag = &frag[0];
    let check = |sym :&str| {
        let desc_res = frag.resolve_desc(sym).unwrap();
        info!("{}: {:?}", sym, desc_res);
    };
    check("0.0");
    check("0.0.s");
    check("0.0.cond");
    check("0.0.s.b");
    check("0.0.s.v");
    check("0.0.s.v.0");
    check("0.0.s.v.1");
    check("0.0.s.v.2");
    check("0.0.s.v.3");
    check("0.0.s.v.4");
    check("0.0.s.i");
}


fn collect_spirv_binaries<P: AsRef<Path>>(path: P) -> HashMap<String, SpirvBinary> {
    use std::ffi::OsStr;
    use std::fs::{read_dir, File};
    use std::io::Read;
    use log::warn;

    read_dir(path).unwrap()
        .filter_map(|x| match x {
            Ok(rv) => Some(rv.path()),
            Err(err) => {
                warn!("cannot access to filesystem item: {}", err);
                None
            },
        })
        .filter_map(|x| {
            let mut buf = Vec::new();
            if !x.is_file() ||
                x.extension() != Some(OsStr::new("spv")) ||
                File::open(&x).and_then(|mut x| x.read_to_end(&mut buf)).is_err() ||
                buf.len() & 3 != 0 {
                return None;
            }
            let spv = buf.into();
            let name = x.file_stem()
                .and_then(OsStr::to_str)
                .map(ToOwned::to_owned)
                .unwrap();
            Some((name, spv))
        })
        .collect::<HashMap<_, _>>()
}
