defmodule Extism.Plugin do
  defstruct [
    # The actual NIF Resource. PluginIndex in this case
    plugin_index: nil,
  ]

  def wrap_resource(plugin_index) do
    %__MODULE__{
      plugin_index: plugin_index
    }
  end

  def new(manifest, wasi \\ false) when is_map(manifest) do
    {:ok, manifest_payload} = JSON.encode(manifest)
    case Extism.Native.plugin_new_with_manifest(manifest_payload, wasi) do
      {:error, err} -> {:error, err}
      res -> {:ok, Extism.Plugin.wrap_resource(res)}
    end
  end

  def call(plugin, name, input) do
    case Extism.Native.call_plugin(plugin.plugin_index, name, input) do
      {:error, err} -> {:error, err}
      res -> {:ok, res}
    end
  end

  def update(plugin, manifest, wasi) when is_map(manifest) do
    {:ok, manifest_payload} = JSON.encode(manifest)
    case Extism.Native.update_manifest(plugin.plugin_index, manifest_payload, wasi) do
      {:error, err} -> {:error, err}
      res -> {:ok, res}
    end
  end

  def set_log_file(plugin, filename, log_level) do
    Extism.Native.set_log_file(plugin.plugin_index, filename, log_level)
  end

  def has_function(plugin, function_name) do
    Extism.Native.plugin_has_function(plugin.plugin_index, function_name)
  end
end

defimpl Inspect, for: Extim.Plugin do
  import Inspect.Algebra

  def inspect(dict, opts) do
    concat(["#Extism.Plugin<", to_doc(dict.plugin_index, opts), ">"])
  end
end
