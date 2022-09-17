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
fn plugin_new_with_manifest(manifest_payload: String, wasi: bool) -> Result<isize, Error> {
    match serde_json::from_str(&manifest_payload) {
        Ok(manifest) => {
            match Plugin::new_with_manifest(&manifest, wasi) {
                Err(_e) => Err(Error::Term(Box::new("Could not load plugin"))),
                Ok(plugin) => Ok(plugin.as_isize())
            }
        },
        Err(_e) => Err(Error::Term(Box::new("Could not parse manifest")))
    }
}

 #[rustler::nif]
fn call_plugin(plugin_iz: isize, name: String, input: String) -> Result<String, Error> {
    let plugin = Plugin(plugin_iz);
    match plugin.call(name, input) {
        Err(_e) => Err(Error::Term(Box::new("Failed to call plugin"))),
        Ok(result) => {
            match str::from_utf8(&result) {
                Ok(output) => Ok(output.to_string()),
                Err(_e) => Err(Error::Term(Box::new("Could not read output from plugin")))
            }
        }
    }
}

rustler::init!("Elixir.Extism.Native", [
    plugin_new_with_manifest,
    call_plugin,
]);
//load = load);

