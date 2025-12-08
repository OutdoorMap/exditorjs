defmodule ExditorJS do
  @moduledoc """
  Native Elixir module for converting Markdown and HTML to EditorJS format
  using Rustler for performance.
  
  This module provides functions to convert HTML and Markdown content into
  EditorJS block format, which can be used with the Editor.js library.
  """

  version = Mix.Project.config()[:version]

  use RustlerPrecompiled,
    otp_app: :exditorjs,
    crate: "exditorjs_native",
    base_url: "https://github.com/OutdoorMap/exditorjs/releases/download/v#{version}",
    force_build: System.get_env("RUSTLER_EXDITORJS_FORCE_BUILD") in ["1", "true"],
    targets:
      Enum.uniq(["aarch64-unknown-linux-musl" | RustlerPrecompiled.Config.default_targets()]),
    version: version


  @doc """
  Converts HTML to EditorJS blocks format.
  
  Takes a string containing HTML and returns a list of EditorJS blocks
  that can be used with Editor.js.
  
  ## Examples
  
      iex> ExditorJS.html_to_editorjs("<h1>Hello</h1><p>World</p>")
      {:ok, [%{"type" => "heading", ...}, %{"type" => "paragraph", ...}]}
      
      iex> ExditorJS.html_to_editorjs("<ul><li>Item 1</li></ul>")
      {:ok, [%{"type" => "list", ...}]}
  """
  def html_to_editorjs(html) do
    case html_to_editorjs_nif(html) do
      {:ok, json} -> {:ok, Jason.decode!(json)}
      {:error, reason} -> {:error, reason}
    end
  end

  @doc """
  Converts Markdown to EditorJS blocks format.
  
  Takes a string containing Markdown and returns a list of EditorJS blocks
  that can be used with Editor.js.
  
  ## Examples
  
      iex> ExditorJS.markdown_to_editorjs("# Heading\\n\\nParagraph text")
      {:ok, [%{"type" => "heading", ...}, %{"type" => "paragraph", ...}]}
      
      iex> ExditorJS.markdown_to_editorjs("- Item 1\\n- Item 2")
      {:ok, [%{"type" => "list", ...}]}
  """
  def markdown_to_editorjs(markdown) do
    case markdown_to_editorjs_nif(markdown) do
      {:ok, json} -> {:ok, Jason.decode!(json)}
      {:error, reason} -> {:error, reason}
    end
  end

  # Private NIF functions
  defp html_to_editorjs_nif(_html) do
    :erlang.nif_error(:not_loaded)
  end

  defp markdown_to_editorjs_nif(_markdown) do
    :erlang.nif_error(:not_loaded)
  end
end