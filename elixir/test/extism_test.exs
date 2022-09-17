defmodule ExtismTest do
  use ExUnit.Case
  doctest Extism

  defp new_plugin() do
    path = Path.join([__DIR__, "../../wasm/code.wasm"])
    manifest = %{wasm: [%{path: path}]}
    {:ok, plugin} = Extism.Plugin.new(manifest, false)
    plugin
  end

  test "counts vowels" do
    plugin = new_plugin()
    {:ok, output} = Extism.Plugin.call(plugin, "count_vowels", "this is a test")
    result = JSON.decode(output)
    assert result == {:ok, %{"count" => 4}}
  end

  test "can update manifest" do
    plugin = new_plugin()
    path = Path.join([__DIR__, "../../wasm/code.wasm"])
    manifest = %{wasm: [%{path: path}]}
    assert Extism.Plugin.update(plugin, manifest, true) == {:ok, true}
  end

  test "errors on bad manifest" do
    {:error, _msg} = Extism.Plugin.new(%{"wasm" => 123})
  end

  test "errors on unknown function" do
    {:error, _msg} = Extism.Plugin.call(new_plugin(), "unknown", "this is a test")
  end

  test "set_log_file" do
    assert Extism.Plugin.set_log_file(new_plugin(), "/tmp/logfile.log", "debug")
  end

  test "has_function" do
    assert Extism.Plugin.has_function(new_plugin(), "count_vowels")
    assert !Extism.Plugin.has_function(new_plugin(), "unknown")
  end
end
