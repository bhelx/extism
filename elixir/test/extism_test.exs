defmodule ExtismTest do
  use ExUnit.Case
  doctest Extism

  test "greets the world" do
    assert Extism.hello() == :world
  end
end
