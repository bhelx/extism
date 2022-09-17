defmodule Extism.Native do
  use Rustler,
  otp_app: :extism,
  crate: :extism_nif

  def plugin_new_with_manifest(_manifest, _wasi), do: error()
  def call_plugin(_plugin, _name, _input), do: error()
  def update_manifest(_plugin, _manifest, _wasi), do: error()
  def set_log_file(_plugin, _filename, _level), do: error()

  defp error, do: :erlang.nif_error(:nif_not_loaded)
end
