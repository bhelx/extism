use rustler::{Error};
use extism::{Plugin};
use std::str;
use serde_json;

mod atoms {
    rustler::atoms! {
        ok,
        error,
        eof,
        unknown // Other error
    }
}

// fn load(env: Env, _: Term) -> bool {
//     true
// }

 #[rustler::nif]
fn plugin_new_with_manifest(manifest_payload: String) -> Result<isize, Error> {
    match serde_json::from_str(&manifest_payload) {
        Ok(manifest) => {
            let plugin = Plugin::new_with_manifest(&manifest, false).unwrap();
            Ok(plugin.as_isize())
        },
        Err(_e) => Err(Error::Term(Box::new("Could not parse manifest")))
    }
}

 #[rustler::nif]
fn call_plugin(plugin_iz: isize, name: String, input: String) -> Result<String, Error> {
    let plugin = Plugin(plugin_iz);
    let result = plugin.call(name, input).unwrap();
    let output = str::from_utf8(&result).unwrap().to_string();
    Ok(output)
}

rustler::init!("Elixir.Extism.Native", [
    plugin_new_with_manifest,
    call_plugin,
]);
//load = load);