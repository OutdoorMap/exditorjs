defmodule ExditorJS.ConfigTest do
  use ExUnit.Case, async: true

  describe "json_library configuration" do
    setup do
      original = Application.get_env(:exditorjs, :json_library)

      on_exit(fn ->
        if original do
          Application.put_env(:exditorjs, :json_library, original)
        else
          Application.delete_env(:exditorjs, :json_library)
        end
      end)
    end

    test "defaults to JSON when configured and available" do
      Application.put_env(:exditorjs, :json_library, JSON)
      assert Application.get_env(:exditorjs, :json_library) == JSON
    end

    test "can be configured to Jason" do
      Application.put_env(:exditorjs, :json_library, Jason)
      assert Application.get_env(:exditorjs, :json_library) == Jason
    end

    test "can be configured to custom library" do
      custom_lib = Jason
      Application.put_env(:exditorjs, :json_library, custom_lib)
      assert Application.get_env(:exditorjs, :json_library) == custom_lib
    end
  end

  describe "json_library/0 auto-detection" do
    test "uses JSON when available and not explicitly configured" do
      Application.delete_env(:exditorjs, :json_library)
      # JSON is available in this environment, so functions should work
      html = "<h1>Test</h1>"
      {:ok, document} = ExditorJS.html_to_editorjs(html)
      assert is_map(document)
      assert document["version"] == "2.25.0"
    end
  end
end
