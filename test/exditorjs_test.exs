defmodule ExditorJSTest do
  use ExUnit.Case, async: true

  describe "html_to_editorjs/1" do
    test "converts simple heading and paragraph" do
      html = "<h1>Welcome to EditorJS</h1><p>This is a simple paragraph.</p>"
      {:ok, document} = ExditorJS.html_to_editorjs(html)
      
      assert is_map(document)
      assert document["version"] == "2.25.0"
      assert is_integer(document["time"])
      assert document["time"] > 0
      assert is_list(document["blocks"])
      assert length(document["blocks"]) >= 2
      
      block_types = Enum.map(document["blocks"], & &1["type"])
      assert "heading" in block_types
      assert "paragraph" in block_types
      
      heading_block = Enum.find(document["blocks"], fn block -> block["type"] == "heading" end)
      assert heading_block != nil
      assert heading_block["data"]["text"] == "Welcome to EditorJS"
      assert heading_block["data"]["level"] == 1
      
      paragraph_block = Enum.find(document["blocks"], fn block -> block["type"] == "paragraph" end)
      assert paragraph_block != nil
      assert paragraph_block["data"]["text"] == "This is a simple paragraph."
    end

    test "converts unordered lists" do
      html = "<ul><li>First item</li><li>Second item</li><li>Third item</li></ul>"
      {:ok, document} = ExditorJS.html_to_editorjs(html)
      
      assert is_list(document["blocks"])
      assert Enum.any?(document["blocks"], fn block -> block["type"] == "list" end)
      
      list_block = Enum.find(document["blocks"], fn block -> block["type"] == "list" end)
      assert list_block != nil
      assert is_map(list_block["data"])
      assert is_list(list_block["data"]["items"])
      assert length(list_block["data"]["items"]) == 3
      assert Enum.at(list_block["data"]["items"], 0)["content"] == "First item"
      assert Enum.at(list_block["data"]["items"], 1)["content"] == "Second item"
      assert Enum.at(list_block["data"]["items"], 2)["content"] == "Third item"
    end

    test "converts blockquotes" do
      html = "<blockquote>This is a blockquote with some wisdom.</blockquote>"
      {:ok, document} = ExditorJS.html_to_editorjs(html)
      
      assert is_list(document["blocks"])
      assert document["version"] == "2.25.0"
      
      block_types = Enum.map(document["blocks"], & &1["type"])
      assert "quote" in block_types
      
      quote_block = Enum.find(document["blocks"], fn block -> block["type"] == "quote" end)
      assert quote_block != nil
      assert quote_block["data"]["text"] == "This is a blockquote with some wisdom."
    end

    test "handles empty HTML" do
      {:ok, document} = ExditorJS.html_to_editorjs("")
      
      assert is_list(document["blocks"])
      assert document["version"] == "2.25.0"
    end

    test "converts images" do
      html = "<img src=\"https://example.com/image.jpg\" alt=\"Example Image\">"
      {:ok, document} = ExditorJS.html_to_editorjs(html)
      
      assert is_list(document["blocks"])
      assert document["version"] == "2.25.0"
      
      block_types = Enum.map(document["blocks"], & &1["type"])
      assert "image" in block_types
      
      image_block = Enum.find(document["blocks"], fn block -> block["type"] == "image" end)
      assert image_block != nil
      assert image_block["data"]["url"] == "https://example.com/image.jpg"
      assert image_block["data"]["caption"] == "Example Image"
    end

    test "converts code blocks" do
      html = "<code>let result = convert(input);</code>"
      {:ok, document} = ExditorJS.html_to_editorjs(html)
      
      assert is_list(document["blocks"])
      assert document["version"] == "2.25.0"
      
      block_types = Enum.map(document["blocks"], & &1["type"])
      assert "code" in block_types
      
      code_block = Enum.find(document["blocks"], fn block -> block["type"] == "code" end)
      assert code_block != nil
      assert code_block["data"]["code"] == "let result = convert(input);"
    end
  end

  describe "markdown_to_editorjs/1" do
    test "converts headings and paragraphs" do
      markdown = "# Getting Started\n\nThis is a **markdown** document."
      {:ok, document} = ExditorJS.markdown_to_editorjs(markdown)
      
      assert is_map(document)
      assert document["version"] == "2.25.0"
      assert is_integer(document["time"])
      assert document["time"] > 0
      assert is_list(document["blocks"])
      assert length(document["blocks"]) >= 2
      
      block_types = Enum.map(document["blocks"], & &1["type"])
      assert "heading" in block_types
      assert "paragraph" in block_types
      
      heading_block = Enum.find(document["blocks"], fn block -> block["type"] == "heading" end)
      assert heading_block != nil
      assert heading_block["data"]["text"] == "Getting Started"
      assert heading_block["data"]["level"] == 1
      
      paragraph_block = Enum.find(document["blocks"], fn block -> block["type"] == "paragraph" end)
      assert paragraph_block != nil
      assert String.contains?(paragraph_block["data"]["text"], "markdown")
    end

    test "converts unordered lists" do
      markdown = "- Easy to use\n- Powerful conversion\n- Well documented"
      {:ok, document} = ExditorJS.markdown_to_editorjs(markdown)
      
      assert is_list(document["blocks"])
      assert Enum.any?(document["blocks"], fn block -> block["type"] == "list" end)
      
      list_block = Enum.find(document["blocks"], fn block -> block["type"] == "list" end)
      assert list_block != nil
      assert is_map(list_block["data"])
      assert is_list(list_block["data"]["items"])
      assert length(list_block["data"]["items"]) == 3
      assert Enum.at(list_block["data"]["items"], 0)["content"] == "Easy to use"
      assert Enum.at(list_block["data"]["items"], 1)["content"] == "Powerful conversion"
      assert Enum.at(list_block["data"]["items"], 2)["content"] == "Well documented"
    end

    test "converts code blocks" do
      markdown = "```rust\nfn main() {\n    println!(\"Hello, world!\");\n}\n```"
      {:ok, document} = ExditorJS.markdown_to_editorjs(markdown)
      
      assert is_list(document["blocks"])
      assert document["version"] == "2.25.0"
      
      block_types = Enum.map(document["blocks"], & &1["type"])
      assert "code" in block_types
      
      code_block = Enum.find(document["blocks"], fn block -> block["type"] == "code" end)
      assert code_block != nil
      assert code_block["data"]["language"] == "rust"
      assert String.contains?(code_block["data"]["code"], "fn main()")
      assert String.contains?(code_block["data"]["code"], "println!")
    end

    test "converts blockquotes" do
      markdown = "> This is a blockquote with some wisdom."
      {:ok, document} = ExditorJS.markdown_to_editorjs(markdown)
      
      assert is_list(document["blocks"])
      assert document["version"] == "2.25.0"
      
      block_types = Enum.map(document["blocks"], & &1["type"])
      assert "quote" in block_types
      
      quote_block = Enum.find(document["blocks"], fn block -> block["type"] == "quote" end)
      assert quote_block != nil
      assert quote_block["data"]["text"] == "This is a blockquote with some wisdom."
    end

    test "handles complex markdown document" do
      markdown = """
      # Main Title
      
      Some introductory text.
      
      ## Section
      
      - Item 1
      - Item 2
      - Item 3
      """
      {:ok, document} = ExditorJS.markdown_to_editorjs(markdown)

      assert is_map(document)
      assert document["version"] == "2.25.0"
      assert is_integer(document["time"])
      assert is_list(document["blocks"])
      assert length(document["blocks"]) > 0
      
      block_types = Enum.map(document["blocks"], & &1["type"])
      assert "heading" in block_types
      assert "paragraph" in block_types
      assert "list" in block_types
      
      h1_block = Enum.find(document["blocks"], fn block -> 
        block["type"] == "heading" && block["data"]["level"] == 1 
      end)
      assert h1_block != nil
      assert h1_block["data"]["text"] == "Main Title"
      
      h2_block = Enum.find(document["blocks"], fn block -> 
        block["type"] == "heading" && block["data"]["level"] == 2 
      end)
      assert h2_block != nil
      assert h2_block["data"]["text"] == "Section"
    end

    test "handles empty markdown" do
      {:ok, document} = ExditorJS.markdown_to_editorjs("")
      
      assert is_list(document["blocks"])
      assert document["version"] == "2.25.0"
    end
  end
end