defmodule ExditorJS.MixProject do
  use Mix.Project

  @version "0.1.3"

  def project do
    [
      app: :exditorjs,
      version: @version,
      elixir: "~> 1.14",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      compilers: Mix.compilers()
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.35.1"},
      {:rustler_precompiled, "~> 0.8.3"},
      {:jason, "~> 1.4"}
    ]
  end
end