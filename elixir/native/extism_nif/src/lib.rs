use extism::Plugin;
use rustler::{Atom, Error};
use serde_json;
use std::path::Path;
use std::str;
use std::str::FromStr;

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
        Ok(manifest) => match Plugin::new_with_manifest(&manifest, wasi) {
            Err(_e) => Err(Error::Term(Box::new("Could not load plugin"))),
            Ok(plugin) => Ok(plugin.into()),
        },
        Err(_e) => Err(Error::Term(Box::new("Could not parse manifest"))),
    }
}

#[rustler::nif]
fn call_plugin(plugin_iz: isize, name: String, input: String) -> Result<String, Error> {
    let plugin = Plugin::from(plugin_iz);
    match plugin.call(name, input) {
        Err(_e) => Err(Error::Term(Box::new("Failed to call plugin"))),
        Ok(result) => match str::from_utf8(&result) {
            Ok(output) => Ok(output.to_string()),
            Err(_e) => Err(Error::Term(Box::new("Could not read output from plugin"))),
        },
    }
}

#[rustler::nif]
fn update_manifest(plugin_iz: isize, manifest_payload: String, wasi: bool) -> Result<bool, Error> {
    let plugin = &mut Plugin::from(plugin_iz);
    match serde_json::from_str(&manifest_payload) {
        Ok(manifest) => match plugin.update_manifest(&manifest, wasi) {
            Err(_e) => Err(Error::Term(Box::new("Could not update manifest"))),
            Ok(updated) => Ok(updated),
        },
        Err(_e) => Err(Error::Term(Box::new("Could not parse manifest"))),
    }
}

#[rustler::nif]
fn set_log_file(plugin_iz: isize, filename: String, log_level: String) -> Result<Atom, Error> {
    let plugin = &Plugin::from(plugin_iz);
    let path = Path::new(&filename);
    match log::LevelFilter::from_str(&log_level) {
        Err(_e) => Err(Error::Term(Box::new(format!(
            "Invalid log level {}",
            log_level
        )))),
        Ok(level) => {
            plugin.set_log_file(path, Some(level));
            Ok(atoms::ok())
        }
    }
}

#[rustler::nif]
fn plugin_has_function(plugin_iz: isize, function_name: String) -> Result<bool, Error> {
    let plugin = &Plugin::from(plugin_iz);
    Ok(plugin.has_function(function_name))
}

rustler::init!(
    "Elixir.Extism.Native",
    [
        plugin_new_with_manifest,
        call_plugin,
        update_manifest,
        set_log_file,
        plugin_has_function,
    ]
);
//load = load);
