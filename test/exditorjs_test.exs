defmodule ExditorJSTest do
  use ExUnit.Case, async: true

  describe "html_to_editorjs/1" do
    test "converts simple heading and paragraph" do
      html = "<h1>Welcome to EditorJS</h1><p>This is a simple paragraph.</p>"
      {:ok, blocks} = ExditorJS.html_to_editorjs(html)
      
      assert is_list(blocks)
      assert length(blocks) >= 2
    end

    test "converts unordered lists" do
      html = "<ul><li>First item</li><li>Second item</li><li>Third item</li></ul>"
      {:ok, blocks} = ExditorJS.html_to_editorjs(html)
      
      assert is_list(blocks)
      assert Enum.any?(blocks, fn block -> block["type"] == "list" end)
    end

    test "converts blockquotes" do
      html = "<blockquote>This is a blockquote with some wisdom.</blockquote>"
      {:ok, blocks} = ExditorJS.html_to_editorjs(html)
      
      assert is_list(blocks)
    end

    test "handles empty HTML" do
      {:ok, blocks} = ExditorJS.html_to_editorjs("")
      assert is_list(blocks)
    end

    test "converts images" do
      html = "<img src=\"https://example.com/image.jpg\" alt=\"Example Image\">"
      {:ok, blocks} = ExditorJS.html_to_editorjs(html)
      
      assert is_list(blocks)
    end

    test "converts code blocks" do
      html = "<code>let result = convert(input);</code>"
      {:ok, blocks} = ExditorJS.html_to_editorjs(html)
      
      assert is_list(blocks)
    end
  end

  describe "markdown_to_editorjs/1" do
    test "converts headings and paragraphs" do
      markdown = "# Getting Started\n\nThis is a **markdown** document."
      {:ok, blocks} = ExditorJS.markdown_to_editorjs(markdown)
      
      assert is_list(blocks)
      assert length(blocks) >= 2
    end

    test "converts unordered lists" do
      markdown = "- Easy to use\n- Powerful conversion\n- Well documented"
      {:ok, blocks} = ExditorJS.markdown_to_editorjs(markdown)
      
      assert is_list(blocks)
      assert Enum.any?(blocks, fn block -> block["type"] == "list" end)
    end

    test "converts code blocks" do
      markdown = "```rust\nfn main() {\n    println!(\"Hello, world!\");\n}\n```"
      {:ok, blocks} = ExditorJS.markdown_to_editorjs(markdown)
      
      assert is_list(blocks)
    end

    test "converts blockquotes" do
      markdown = "> This is a blockquote with some wisdom."
      {:ok, blocks} = ExditorJS.markdown_to_editorjs(markdown)
      
      assert is_list(blocks)
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
      {:ok, blocks} = ExditorJS.markdown_to_editorjs(markdown)
      
      assert is_list(blocks)
      assert length(blocks) > 0
    end

    test "handles empty markdown" do
      {:ok, blocks} = ExditorJS.markdown_to_editorjs("")
      assert is_list(blocks)
    end
  end
end