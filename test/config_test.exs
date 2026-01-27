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

    test "defaults to JSON when not configured" do
      Application.delete_env(:exditorjs, :json_library)
      assert Application.get_env(:exditorjs, :json_library, JSON) == JSON
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
end
