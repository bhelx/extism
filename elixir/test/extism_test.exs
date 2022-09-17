defmodule ExtismTest do
  use ExUnit.Case
  doctest Extism

  test "counts vowels" do
    path = Path.join([__DIR__, "../../wasm/code.wasm"])
    IO.puts path
    manifest = %{wasm: [%{path: path}]}
    result =
       with {:ok, plugin} <- Extism.Plugin.new(manifest),
            {:ok, output} <- Extism.Plugin.call(plugin, "count_vowels", "this is a test"),
         do: JSON.decode(output)

    assert result == {:ok, %{"count" => 4}}
  end
end
