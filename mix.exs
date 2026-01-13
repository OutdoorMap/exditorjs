defmodule ExditorJS.MixProject do
  use Mix.Project

  @version "0.1.7"

  def project do
    [
      app: :exditorjs,
      version: @version,
      elixir: "~> 1.14",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      compilers: Mix.compilers(),
      description: description(),
      package: package()
    ]
  end

  defp description do
    "Elixir/Rust library for parsing and validating EditorJS content"
  end

  defp package do
    [
      licenses: ["MIT"],
      links: %{
        "GitHub" => "https://github.com/OutdoorMap/exditorjs"
      }
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.37.1", optional: true, runtime: false},
      {:rustler_precompiled, "~> 0.8.3"},
      {:jason, "~> 1.4"}
    ]
  end
end