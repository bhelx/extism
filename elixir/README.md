# Extism

Extism Host SDK for Elixir and Erlang

## Installation

```elixir
def deps do
  [
    {:extism, "~> 0.1.0"}
  ]
end
```

## Usage

```elixir
# point to some wasm code, this is the count_vowels example that ships with extism
manifest = %{ wasm: [ %{ path: "/Users/ben/code/extism/wasm/code.wasm" } ]}
{:ok, plugin} = Extism.Plugin.new(manifest)
# {:ok,
# %Extism.Plugin{
#   resource: 0,
#   reference: #Reference<0.520418104.1263009793.80956>
# }}
{:ok, output} = Extism.Plugin.call(plugin, "count_vowels", "this is a test")
# {:ok, "{\"count\": 4}"}
{:ok, result} = JSON.decode(output)
# {:ok, %{"count" => 4}}
```
