defmodule ExditorJS.MixProject do
  use Mix.Project

  def project do
    [
      app: :exditorjs,
      version: "0.1.0",
      elixir: "~> 1.14",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      compilers: Mix.compilers()
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.37.1"},
      {:rustler_precompiled, "~> 0.8.3"},
      {:jason, "~> 1.4"}
    ]
  end
end