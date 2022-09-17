defmodule Extism.Plugin do
  defstruct [
    # The actual NIF Resource.
    resource: nil,
    # Normally the compiler will happily do stuff like inlining the
    # resource in attributes. This will convert the resource into an
    # empty binary with no warning. This will make that harder to
    # accidentaly do.
    # It also serves as a handy way to tell plugins apart.
    reference: nil
  ]

  def wrap_resource(resource) do
    %__MODULE__{
      resource: resource,
      reference: make_ref()
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
    case Extism.Native.call_plugin(plugin.resource, name, input) do
      {:error, err} -> {:error, err}
      res -> {:ok, res}
    end
  end

  def update(plugin, manifest, wasi) when is_map(manifest) do
    {:ok, manifest_payload} = JSON.encode(manifest)
    case Extism.Native.update_manifest(plugin.resource, manifest_payload, wasi) do
      {:error, err} -> {:error, err}
      res -> {:ok, res}
    end
  end

  def set_log_file(plugin, filename, log_level) do
    Extism.Native.set_log_file(plugin.resource, filename, log_level)
  end

  def has_function(plugin, function_name) do
    Extism.Native.plugin_has_function(plugin.resource, function_name)
  end
end

defimpl Inspect, for: Extim.Plugin do
  import Inspect.Algebra

  def inspect(dict, opts) do
    concat(["#Extism.Plugin<", to_doc(dict.reference, opts), ">"])
  end
end
