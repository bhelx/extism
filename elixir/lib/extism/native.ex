defmodule Extism.Native do
  use Rustler,
  otp_app: :extism,
  crate: :extism_nif

  def plugin_new_with_manifest(_manifest), do: error()
  def call_plugin(_plugin, _name, _input), do: error()

  defp error, do: :erlang.nif_error(:nif_not_loaded)
end
