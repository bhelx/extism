use rustler::{Atom};
use extism::{Plugin};
use std::str;
use std::path::Path;
use std::str::FromStr;

mod atoms {
    rustler::atoms! {
        ok,
        error,
        unknown // Other error
    }
}

// fn load(env: Env, _: Term) -> bool {
//     true
// }
fn to_rustler_error(extism_error: extism::Error) -> rustler::Error {
    match extism_error {
        extism::Error::UnableToLoadPlugin => rustler::Error::Term(Box::new("Unable to load plugin for unknown reason")),
        extism::Error::Message(msg) => rustler::Error::Term(Box::new(msg)),
        extism::Error::Json(json_err) => rustler::Error::Term(Box::new(json_err.to_string()))
    }
}

 #[rustler::nif]
fn plugin_new_with_manifest(manifest_payload: String, wasi: bool) -> Result<isize, rustler::Error> {
    match Plugin::new(manifest_payload, wasi) {
        Err(e) => Err(to_rustler_error(e)),
        Ok(plugin) => Ok(plugin.as_isize())
    }
}

 #[rustler::nif]
fn call_plugin(plugin_iz: isize, name: String, input: String) -> Result<String, rustler::Error> {
    let plugin = Plugin(plugin_iz);
    match plugin.call(name, input) {
        Err(e) => Err(to_rustler_error(e)),
        Ok(result) => {
            match str::from_utf8(&result) {
                Ok(output) => Ok(output.to_string()),
                Err(_e) => Err(rustler::Error::Term(Box::new("Could not read output from plugin")))
            }
        }
    }
}

 #[rustler::nif]
fn update_manifest(plugin_iz: isize, manifest_payload: String, wasi: bool) -> Result<bool, rustler::Error> {
    let plugin = &mut Plugin(plugin_iz);
    let result = plugin.update(manifest_payload, wasi);
    Ok(result)
}

#[rustler::nif]
fn set_log_file(plugin_iz: isize, filename: String, log_level: String) -> Result<Atom, rustler::Error> {
    let plugin = &Plugin(plugin_iz);
    let path = Path::new(&filename);
    match log::LevelFilter::from_str(&log_level) {
        Err(_e) => Err(rustler::Error::Term(Box::new(format!("{} not a valid log level", log_level)))),
        Ok(level) => {
            plugin.set_log_file(path, Some(level));
            Ok(atoms::ok())
        }
    }
}

#[rustler::nif]
fn plugin_has_function(plugin_iz: isize, function_name: String) -> Result<bool, rustler::Error> {
    let plugin = &Plugin(plugin_iz);
    Ok(plugin.has_function(function_name))
}

rustler::init!("Elixir.Extism.Native", [
    plugin_new_with_manifest,
    call_plugin,
    update_manifest,
    set_log_file,
    plugin_has_function,
]);
//load = load);

