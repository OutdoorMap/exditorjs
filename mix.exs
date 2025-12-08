defmodule ExditorJS.MixProject do
  use Mix.Project

  def project do
    [
      app: :exditorjs,
      version: "0.1.0",
      elixir: "~> 1.14",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      compilers: Mix.compilers(),
      rustler_crates: rustler_crates()
    ]
  end

  defp rustler_crates do
    [
      exditorjs_native: []
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.37.1"},
      {:jason, "~> 1.4"}
    ]
  end
end