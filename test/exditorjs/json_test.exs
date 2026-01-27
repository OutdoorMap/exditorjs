defmodule ExditorJS.JSONTest do
  use ExUnit.Case, async: true

  describe "encode/2" do
    test "encodes map to JSON string with Jason" do
      data = %{"key" => "value", "nested" => %{"a" => 1}}
      assert {:ok, json_string} = ExditorJS.JSON.encode(data, Jason)
      assert String.contains?(json_string, ~s|"key":"value"|)
      assert String.contains?(json_string, ~s|"nested":|)
    end

    test "encodes list to JSON string with Jason" do
      data = [1, 2, "three", %{"four" => 4}]
      assert {:ok, json_string} = ExditorJS.JSON.encode(data, Jason)
      assert String.contains?(json_string, "1")
      assert String.contains?(json_string, "three")
    end

    test "returns error for invalid data with Jason" do
      data = make_ref()
      result = ExditorJS.JSON.encode(data, Jason)
      assert {:error, _} = result
    end

    test "encodes with JSON module when available" do
      if Code.ensure_loaded?(JSON) do
        data = %{"test" => "value"}
        assert {:ok, json_string} = ExditorJS.JSON.encode(data, JSON)
        assert String.contains?(json_string, ~s|"test":"value"|)
      end
    end

    test "encodes with custom library" do
      defmodule CustomJSON do
        def encode(_), do: {:ok, "{\"custom\": true}"}
        def decode(_, _), do: {:ok, %{"custom" => true}}
      end

      assert {:ok, json_string} = ExditorJS.JSON.encode(%{}, CustomJSON)
      assert json_string == "{\"custom\": true}"
    end
  end

  describe "decode/2" do
    test "decodes JSON string to map with Jason" do
      json_string = ~s|{"key":"value","nested":{"a":1}}|
      assert {:ok, data} = ExditorJS.JSON.decode(json_string, Jason)
      assert data["key"] == "value"
      assert data["nested"]["a"] == 1
    end

    test "decodes array JSON string to list with Jason" do
      json_string = ~s|[1,2,"three"]|
      assert {:ok, data} = ExditorJS.JSON.decode(json_string, Jason)
      assert data == [1, 2, "three"]
    end

    test "returns error for invalid JSON with Jason" do
      invalid_json = "{invalid json"
      result = ExditorJS.JSON.decode(invalid_json, Jason)
      assert {:error, _} = result
    end

    test "decodes with JSON module when available" do
      if Code.ensure_loaded?(JSON) do
        json_string = ~s|{"test":"value"}|
        assert {:ok, data} = ExditorJS.JSON.decode(json_string, JSON)
        assert data["test"] == "value"
      end
    end

    test "decodes with custom library" do
      defmodule CustomJSONDecoder do
        def decode(_), do: {:ok, %{"custom" => true}}
      end

      assert {:ok, data} = ExditorJS.JSON.decode("some json", CustomJSONDecoder)
      assert data == %{"custom" => true}
    end
  end

  describe "UTF-8 support" do
    test "encodes UTF-8 characters with Jason" do
      data = %{"swedish" => "Upptäck Dalsland", "japanese" => "ダルスランド"}
      assert {:ok, json_string} = ExditorJS.JSON.encode(data, Jason)
      assert String.contains?(json_string, "Upptäck Dalsland")
      assert String.contains?(json_string, "ダルスランド")
    end

    test "decodes UTF-8 characters with Jason" do
      json_string = ~s|{"swedish":"Upptäck Dalsland","japanese":"ダルスランド"}|
      assert {:ok, data} = ExditorJS.JSON.decode(json_string, Jason)
      assert data["swedish"] == "Upptäck Dalsland"
      assert data["japanese"] == "ダルスランド"
    end
  end
end
