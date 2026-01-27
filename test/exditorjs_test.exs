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

      paragraph_block =
        Enum.find(document["blocks"], fn block -> block["type"] == "paragraph" end)

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

      paragraph_block =
        Enum.find(document["blocks"], fn block -> block["type"] == "paragraph" end)

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

      h1_block =
        Enum.find(document["blocks"], fn block ->
          block["type"] == "heading" && block["data"]["level"] == 1
        end)

      assert h1_block != nil
      assert h1_block["data"]["text"] == "Main Title"

      h2_block =
        Enum.find(document["blocks"], fn block ->
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

    test "markdown with UTF-8 in headings" do
      markdown = "# Upptäck Dalsland från cykelsadeln"
      {:ok, document} = ExditorJS.markdown_to_editorjs(markdown)

      assert is_list(document["blocks"])
      heading_blocks = Enum.filter(document["blocks"], fn block -> block["type"] == "heading" end)
      assert length(heading_blocks) > 0

      heading = Enum.at(heading_blocks, 0)
      assert heading["data"]["level"] == 1
      assert heading["data"]["text"] == "Upptäck Dalsland från cykelsadeln"
    end

    test "markdown with UTF-8 in paragraphs" do
      markdown = "Cykeln är det perfekta redskapet för att upptäcka Dalsland."
      {:ok, document} = ExditorJS.markdown_to_editorjs(markdown)

      assert is_list(document["blocks"])

      paragraph_blocks =
        Enum.filter(document["blocks"], fn block -> block["type"] == "paragraph" end)

      assert length(paragraph_blocks) > 0

      paragraph = Enum.at(paragraph_blocks, 0)
      assert String.contains?(paragraph["data"]["text"], "Dalsland")
    end

    test "markdown with UTF-8 in unordered lists" do
      markdown = """
      - Cykelpaket för äventyrare
      - Guidat cykeltur i Dalsland
      - Sevärdheter och aktiviteter
      """

      {:ok, document} = ExditorJS.markdown_to_editorjs(markdown)

      list_blocks = Enum.filter(document["blocks"], fn block -> block["type"] == "list" end)
      assert length(list_blocks) > 0

      list = Enum.at(list_blocks, 0)
      assert list["data"]["style"] == "unordered"
      assert length(list["data"]["items"]) == 3
      assert Enum.at(list["data"]["items"], 0)["content"] == "Cykelpaket för äventyrare"
      assert Enum.at(list["data"]["items"], 1)["content"] == "Guidat cykeltur i Dalsland"
    end

    test "markdown with UTF-8 in ordered lists" do
      markdown = """
      1. Cykelpaket för äventyrare
      2. Guidat cykeltur i Dalsland
      """

      {:ok, document} = ExditorJS.markdown_to_editorjs(markdown)

      list_blocks = Enum.filter(document["blocks"], fn block -> block["type"] == "list" end)
      assert length(list_blocks) > 0

      list = Enum.at(list_blocks, 0)
      assert list["data"]["style"] == "ordered"
      assert length(list["data"]["items"]) == 2
    end

    test "markdown with UTF-8 in blockquotes" do
      markdown = """
      > Cykeln är det perfekta redskapet för att upptäcka Dalsland
      > och älska naturen omkring dig.
      """

      {:ok, document} = ExditorJS.markdown_to_editorjs(markdown)

      quote_blocks = Enum.filter(document["blocks"], fn block -> block["type"] == "quote" end)
      assert length(quote_blocks) > 0

      quote = Enum.at(quote_blocks, 0)
      assert String.contains?(quote["data"]["text"], "perfekta")
      assert String.contains?(quote["data"]["text"], "Dalsland")
    end

    test "markdown with UTF-8 in code blocks" do
      markdown = """
      ```rust
      // Här är en kommentar
      fn test() {}
      ```
      """

      {:ok, document} = ExditorJS.markdown_to_editorjs(markdown)

      code_blocks = Enum.filter(document["blocks"], fn block -> block["type"] == "code" end)
      assert length(code_blocks) > 0

      code = Enum.at(code_blocks, 0)
      assert code["data"]["language"] == "rust"
      assert String.contains?(code["data"]["code"], "kommentar")
    end

    test "markdown with Japanese in headings" do
      markdown = "# ダルスランドを自転車で探検"
      {:ok, document} = ExditorJS.markdown_to_editorjs(markdown)

      assert is_list(document["blocks"])
      heading_blocks = Enum.filter(document["blocks"], fn block -> block["type"] == "heading" end)
      assert length(heading_blocks) > 0

      heading = Enum.at(heading_blocks, 0)
      assert heading["data"]["level"] == 1
      assert heading["data"]["text"] == "ダルスランドを自転車で探検"
    end

    test "markdown with Japanese in paragraphs" do
      markdown = "自転車はダルスランドを探検するのに最適なツールです。"
      {:ok, document} = ExditorJS.markdown_to_editorjs(markdown)

      assert is_list(document["blocks"])

      paragraph_blocks =
        Enum.filter(document["blocks"], fn block -> block["type"] == "paragraph" end)

      assert length(paragraph_blocks) > 0

      paragraph = Enum.at(paragraph_blocks, 0)
      assert String.contains?(paragraph["data"]["text"], "ダルスランド")
    end

    test "markdown with Japanese in unordered lists" do
      markdown = """
      - サイクリングパッケージ
      - ガイド付きツアー
      - レンタル自転車
      """

      {:ok, document} = ExditorJS.markdown_to_editorjs(markdown)

      list_blocks = Enum.filter(document["blocks"], fn block -> block["type"] == "list" end)
      assert length(list_blocks) > 0

      list = Enum.at(list_blocks, 0)
      assert list["data"]["style"] == "unordered"
      assert length(list["data"]["items"]) == 3
      assert Enum.at(list["data"]["items"], 0)["content"] == "サイクリングパッケージ"
      assert Enum.at(list["data"]["items"], 1)["content"] == "ガイド付きツアー"
      assert Enum.at(list["data"]["items"], 2)["content"] == "レンタル自転車"
    end

    test "markdown with Japanese in ordered lists" do
      markdown = """
      1. 高品質なレンタル自転車
      2. 宿泊施設の手配
      3. ガイドサービス
      """

      {:ok, document} = ExditorJS.markdown_to_editorjs(markdown)

      list_blocks = Enum.filter(document["blocks"], fn block -> block["type"] == "list" end)
      assert length(list_blocks) > 0

      list = Enum.at(list_blocks, 0)
      assert list["data"]["style"] == "ordered"
      assert length(list["data"]["items"]) == 3
      assert Enum.at(list["data"]["items"], 0)["content"] == "高品質なレンタル自転車"
    end

    test "markdown with Japanese in blockquotes" do
      markdown = """
      > 自転車はダルスランドを探検するのに最適なツールです。
      > より深く、より楽しい体験ができます。
      """

      {:ok, document} = ExditorJS.markdown_to_editorjs(markdown)

      quote_blocks = Enum.filter(document["blocks"], fn block -> block["type"] == "quote" end)
      assert length(quote_blocks) > 0

      quote = Enum.at(quote_blocks, 0)
      assert String.contains?(quote["data"]["text"], "最適な")
      assert String.contains?(quote["data"]["text"], "ツール")
    end

    test "markdown with Japanese in code blocks" do
      markdown = """
      ```javascript
      // これはコメントです
      function test() {}
      ```
      """

      {:ok, document} = ExditorJS.markdown_to_editorjs(markdown)

      code_blocks = Enum.filter(document["blocks"], fn block -> block["type"] == "code" end)
      assert length(code_blocks) > 0

      code = Enum.at(code_blocks, 0)
      assert code["data"]["language"] == "javascript"
      assert String.contains?(code["data"]["code"], "コメント")
    end

    test "markdown with mixed Japanese and ASCII" do
      markdown = """
      # ダルスランド探検ガイド 2024

      このガイドはDalsland Experience提供です。

      - 初心者向けコース
      - 中級者向けコース (25km)
      - 上級者向けコース
      """

      {:ok, document} = ExditorJS.markdown_to_editorjs(markdown)

      block_types = Enum.map(document["blocks"], & &1["type"])
      assert "heading" in block_types
      assert "paragraph" in block_types
      assert "list" in block_types

      # Verify heading
      heading =
        Enum.find(document["blocks"], fn block ->
          block["type"] == "heading" && String.contains?(block["data"]["text"], "ダルスランド")
        end)

      assert heading != nil
      assert String.contains?(heading["data"]["text"], "2024")
    end
  end

  describe "embed support" do
    test "converts HTML iframe to embed block" do
      html =
        ~s|<iframe src="https://www.youtube.com/embed/dQw4w9WgXcQ" width="560" height="315"></iframe>|

      {:ok, document} = ExditorJS.html_to_editorjs(html)

      assert is_list(document["blocks"])
      assert Enum.any?(document["blocks"], fn block -> block["type"] == "embed" end)

      embed_block = Enum.find(document["blocks"], fn block -> block["type"] == "embed" end)
      assert embed_block != nil
      assert embed_block["data"]["service"] == "youtube"
      assert embed_block["data"]["width"] == 560
      assert embed_block["data"]["height"] == 315
      assert String.contains?(embed_block["data"]["embed"], "dQw4w9WgXcQ")
    end

    test "converts markdown URL to embed block for YouTube" do
      markdown = "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
      {:ok, document} = ExditorJS.markdown_to_editorjs(markdown)

      assert is_list(document["blocks"])
      embed_block = Enum.find(document["blocks"], fn block -> block["type"] == "embed" end)
      assert embed_block != nil
      assert embed_block["data"]["service"] == "youtube"
    end

    test "converts markdown short URL to embed block for YouTube" do
      markdown = "https://youtu.be/dQw4w9WgXcQ"
      {:ok, document} = ExditorJS.markdown_to_editorjs(markdown)

      assert is_list(document["blocks"])
      embed_block = Enum.find(document["blocks"], fn block -> block["type"] == "embed" end)
      assert embed_block != nil
      assert embed_block["data"]["service"] == "youtube"
    end

    test "converts markdown link with caption to embed block for Vimeo" do
      markdown = "[Watch this](https://vimeo.com/123456789)"
      {:ok, document} = ExditorJS.markdown_to_editorjs(markdown)

      assert is_list(document["blocks"])
      embed_block = Enum.find(document["blocks"], fn block -> block["type"] == "embed" end)
      assert embed_block != nil
      assert embed_block["data"]["service"] == "vimeo"
      assert embed_block["data"]["caption"] == "Watch this"
    end

    test "converts Coub URL to embed block" do
      markdown = "https://coub.com/view/1czcdf"
      {:ok, document} = ExditorJS.markdown_to_editorjs(markdown)

      assert is_list(document["blocks"])
      embed_block = Enum.find(document["blocks"], fn block -> block["type"] == "embed" end)
      assert embed_block != nil
      assert embed_block["data"]["service"] == "coub"
    end

    test "converts Instagram URL to embed block" do
      markdown = "https://www.instagram.com/p/ABC123XYZ/"
      {:ok, document} = ExditorJS.markdown_to_editorjs(markdown)

      assert is_list(document["blocks"])
      embed_block = Enum.find(document["blocks"], fn block -> block["type"] == "embed" end)
      assert embed_block != nil
      assert embed_block["data"]["service"] == "instagram"
    end

    test "converts Twitter URL to embed block" do
      markdown = "https://twitter.com/user/status/1234567890"
      {:ok, document} = ExditorJS.markdown_to_editorjs(markdown)

      assert is_list(document["blocks"])
      embed_block = Enum.find(document["blocks"], fn block -> block["type"] == "embed" end)
      assert embed_block != nil
      assert embed_block["data"]["service"] == "twitter"
    end

    test "converts Twitch video URL to embed block" do
      markdown = "https://twitch.tv/videos/123456789"
      {:ok, document} = ExditorJS.markdown_to_editorjs(markdown)

      assert is_list(document["blocks"])
      embed_block = Enum.find(document["blocks"], fn block -> block["type"] == "embed" end)
      assert embed_block != nil
      assert embed_block["data"]["service"] == "twitch-video"
    end

    test "converts Twitch channel URL to embed block" do
      markdown = "https://twitch.tv/channel_name"
      {:ok, document} = ExditorJS.markdown_to_editorjs(markdown)

      assert is_list(document["blocks"])
      embed_block = Enum.find(document["blocks"], fn block -> block["type"] == "embed" end)
      assert embed_block != nil
      assert embed_block["data"]["service"] == "twitch-channel"
    end

    test "ignores non-embed URLs" do
      markdown = "https://example.com/some/page"
      {:ok, document} = ExditorJS.markdown_to_editorjs(markdown)

      # Non-embed URLs should be treated as paragraphs
      assert is_list(document["blocks"])
      assert !Enum.any?(document["blocks"], fn block -> block["type"] == "embed" end)
    end
  end

  describe "complex HTML parsing" do
    test "parses Swedish cycling content with links and attributes" do
      html = """
      <p>Upptäck Dalsland från cykelsadeln</p>
      <h3>Färdiga cykelpaket och guidade turer</h3>
      <p>Cykeln är det perfekta redskapet för att upptäcka Dalsland. Tillsammans med en guide från <a href="https://www.thedalslandexperience.com/" rel="nofollow noopener" target="_blank">The Dalsland Experience</a> blir upplevelsen både starkare och roligare.</p>
      <p>Du kan välja på färdiga cykelpaket med boende, guide och hyrcykel eller guidade cykelupplevelser om du tar med egen cykel. Under sommaren erbjuds både korta turer som passar alla, men också lite längre turer för cyklister med mer trampvana. Upptäck de stigar, vägar och platser du inte visste fanns och få ut mer av din cykelupplevelse i Dalsland. The Dalsland Experience är baserade i Bengtsfors men arbetar över nästan hela Dalsland. Även i södra Värmland, såsom Åmål, Dals Långed, Mellerud, Dals-Ed och Säffle.</p>
      <p> </p>
      <h3>Hyrcyklar med kvalitet</h3>
      <p>Alla hyrcyklar hos The Dalsland Experience är byggda för att fungera bra på blandat underlag - inte minst på Dalslands fina grusvägar. Samtliga cyklar har hydrauliska bromsar av hög kvalitet och högklassiga växelgrupper. Hjälm och pedaler ingår alltid i hyran, men det är såklart valfritt att använda sina egna. De erbjuder också hämtning/lämning av cyklar.</p>
      <h3>Mer än bara cykel</h3>
      <p>The Dalsland Experience kan hjälpa dig att skräddarsy upplevelser och aktiviteter runt om i Dalsland. Proffsiga samarbetspartners kan erbjuda spännande utflykter, sevärdheter och aktiviteter som sätter guldkant på er cykelresa.<br>Inget boende? Teamet hjälper dig även med boendealternativ anpassat efter dina förutsättningar och önskemål. Det ska vara lätt och roligt att uppleva The Dalsland Experience!</p>
      <p> </p>
      """

      {:ok, document} = ExditorJS.html_to_editorjs(html)

      assert is_map(document)
      assert document["version"] == "2.25.0"
      assert is_integer(document["time"])
      assert document["time"] > 0
      assert is_list(document["blocks"])
      assert length(document["blocks"]) > 0

      # Verify we have heading blocks
      block_types = Enum.map(document["blocks"], & &1["type"])
      assert "heading" in block_types
      assert "paragraph" in block_types

      # Check specific headings
      heading_blocks = Enum.filter(document["blocks"], fn block -> block["type"] == "heading" end)
      heading_texts = Enum.map(heading_blocks, & &1["data"]["text"])

      assert Enum.any?(heading_texts, fn text -> String.contains?(text, "Färdiga") end)
      assert Enum.any?(heading_texts, fn text -> String.contains?(text, "Hyrcyklar") end)
      assert Enum.any?(heading_texts, fn text -> String.contains?(text, "cykel") end)

      # Check that link text is preserved (link should be stripped but text remains)
      paragraph_blocks =
        Enum.filter(document["blocks"], fn block -> block["type"] == "paragraph" end)

      paragraph_texts = Enum.map(paragraph_blocks, & &1["data"]["text"])

      assert Enum.any?(paragraph_texts, fn text ->
               String.contains?(text, "The Dalsland Experience")
             end)

      assert Enum.any?(paragraph_texts, fn text -> String.contains?(text, "Dalsland") end)
    end

    test "handles complex attributes on links" do
      html =
        ~s|<p>Visit <a href="https://example.com" rel="nofollow noopener" target="_blank" class="external-link" data-custom="value">our website</a> for more info.</p>|

      {:ok, document} = ExditorJS.html_to_editorjs(html)

      assert is_list(document["blocks"])
      assert Enum.any?(document["blocks"], fn block -> block["type"] == "paragraph" end)

      paragraph = Enum.find(document["blocks"], fn block -> block["type"] == "paragraph" end)
      assert paragraph != nil
      # Link text should be extracted but link itself removed
      assert String.contains?(paragraph["data"]["text"], "our website")
      assert String.contains?(paragraph["data"]["text"], "more info")
    end

    test "parses HTML with multiple paragraphs and whitespace" do
      html = """
      <p>First paragraph</p>
      <p> </p>
      <p>Second paragraph</p>
      <p></p>
      <p>Third paragraph</p>
      """

      {:ok, document} = ExditorJS.html_to_editorjs(html)

      assert is_map(document)
      assert is_list(document["blocks"])
      assert length(document["blocks"]) > 0

      # Should have paragraph blocks (empty paragraphs might be filtered or kept)
      block_types = Enum.map(document["blocks"], & &1["type"])
      assert "paragraph" in block_types
    end

    test "handles mixed content with headings and paragraphs with links" do
      html = """
      <h2>Welcome</h2>
      <p>This is a <strong>bold</strong> paragraph with a <a href="https://example.com">link</a>.</p>
      <h3>Section</h3>
      <p>Another paragraph with <em>emphasis</em> and <a href="https://other.com" target="_blank">another link</a>.</p>
      """

      {:ok, document} = ExditorJS.html_to_editorjs(html)

      assert is_map(document)
      assert document["version"] == "2.25.0"
      assert is_list(document["blocks"])

      block_types = Enum.map(document["blocks"], & &1["type"])
      assert "heading" in block_types
      assert "paragraph" in block_types

      # Verify h2 exists
      h2_blocks =
        Enum.filter(document["blocks"], fn block ->
          block["type"] == "heading" && block["data"]["level"] == 2
        end)

      assert length(h2_blocks) > 0
      assert Enum.at(h2_blocks, 0)["data"]["text"] == "Welcome"

      # Verify h3 exists
      h3_blocks =
        Enum.filter(document["blocks"], fn block ->
          block["type"] == "heading" && block["data"]["level"] == 3
        end)

      assert length(h3_blocks) > 0
      assert Enum.at(h3_blocks, 0)["data"]["text"] == "Section"
    end

    test "parses Japanese content with links and attributes" do
      html = """
      <p>ダルスランドを自転車で探検</p>
      <h3>パッケージ化されたサイクリングパッケージとガイド付きツアー</h3>
      <p>自転車はダルスランドを探検するのに最適なツールです。<a href="https://www.thedalslandexperience.com/" rel="nofollow noopener" target="_blank">ダルスランド エクスペリエンス</a>のガイドと一緒にサイクリングすれば、より深く、より楽しい体験ができます。</p>
      <p>宿泊、ガイド、レンタル自転車がセットになったパッケージ化されたサイクリングパッケージ、またはご自身の自転車をお持ち込みの場合はガイド付きサイクリング体験からお選びいただけます。</p>
      <p> </p>
      <h3>高品質なレンタル自転車</h3>
      <p>Dalsland Experience のレンタル自転車はすべて、Dalsland の美しい砂利道をはじめ、さまざまな路面状況に対応できるよう設計されています。</p>
      <h3>自転車以上の価値</h3>
      <p>Dalsland Experience では、Dalsland 周辺での体験やアクティビティをお客様に合わせてカスタマイズできます。<br>ご宿泊先がお決まりですか？お客様のご都合やご希望に合わせた宿泊施設のご案内もいたします。</p>
      """

      {:ok, document} = ExditorJS.html_to_editorjs(html)

      assert is_map(document)
      assert document["version"] == "2.25.0"
      assert is_integer(document["time"])
      assert document["time"] > 0
      assert is_list(document["blocks"])
      assert length(document["blocks"]) > 0

      # Verify we have heading blocks
      block_types = Enum.map(document["blocks"], & &1["type"])
      assert "heading" in block_types
      assert "paragraph" in block_types

      # Check specific headings with Japanese text
      heading_blocks = Enum.filter(document["blocks"], fn block -> block["type"] == "heading" end)
      heading_texts = Enum.map(heading_blocks, & &1["data"]["text"])

      assert Enum.any?(heading_texts, fn text -> String.contains?(text, "パッケージ") end)
      assert Enum.any?(heading_texts, fn text -> String.contains?(text, "高品質") end)
      assert Enum.any?(heading_texts, fn text -> String.contains?(text, "自転車") end)

      # Check that Japanese text is preserved
      paragraph_blocks =
        Enum.filter(document["blocks"], fn block -> block["type"] == "paragraph" end)

      paragraph_texts = Enum.map(paragraph_blocks, & &1["data"]["text"])

      assert Enum.any?(paragraph_texts, fn text -> String.contains?(text, "ダルスランド") end)
      assert Enum.any?(paragraph_texts, fn text -> String.contains?(text, "エクスペリエンス") end)
    end

    test "handles Japanese with complex attributes on links" do
      html =
        ~s|<p>詳細は<a href="https://example.jp" rel="nofollow noopener" target="_blank" class="external-link" data-custom="value">こちら</a>をご覧ください。</p>|

      {:ok, document} = ExditorJS.html_to_editorjs(html)

      assert is_list(document["blocks"])
      assert Enum.any?(document["blocks"], fn block -> block["type"] == "paragraph" end)

      paragraph = Enum.find(document["blocks"], fn block -> block["type"] == "paragraph" end)
      assert paragraph != nil
      # Link text should be extracted but link itself removed
      assert String.contains?(paragraph["data"]["text"], "こちら")
      assert String.contains?(paragraph["data"]["text"], "ご覧ください")
    end

    test "parses Japanese characters in headings with proper levels" do
      html = """
      <h1>第一見出し</h1>
      <h2>第二見出し</h2>
      <h3>第三見出し</h3>
      <h4>第四見出し</h4>
      """

      {:ok, document} = ExditorJS.html_to_editorjs(html)

      assert is_map(document)
      assert is_list(document["blocks"])

      # Verify all heading levels
      h1_blocks =
        Enum.filter(document["blocks"], fn block ->
          block["type"] == "heading" && block["data"]["level"] == 1
        end)

      assert length(h1_blocks) > 0
      assert h1_blocks |> Enum.at(0) |> Map.get("data") |> Map.get("text") == "第一見出し"

      h2_blocks =
        Enum.filter(document["blocks"], fn block ->
          block["type"] == "heading" && block["data"]["level"] == 2
        end)

      assert length(h2_blocks) > 0
      assert h2_blocks |> Enum.at(0) |> Map.get("data") |> Map.get("text") == "第二見出し"

      h3_blocks =
        Enum.filter(document["blocks"], fn block ->
          block["type"] == "heading" && block["data"]["level"] == 3
        end)

      assert length(h3_blocks) > 0
      assert h3_blocks |> Enum.at(0) |> Map.get("data") |> Map.get("text") == "第三見出し"
    end
  end

  describe "json_library configuration" do
    setup do
      original = Application.get_env(:exditorjs, :json_library)

      on_exit(fn ->
        if original do
          Application.put_env(:exditorjs, :json_library, original, persistent: true)
        else
          Application.delete_env(:exditorjs, :json_library)
        end
      end)
    end

    test "html_to_editorjs uses configured json_library" do
      Application.put_env(:exditorjs, :json_library, JSON, persistent: true)
      html = "<h1>Test</h1>"
      {:ok, document} = ExditorJS.html_to_editorjs(html)

      assert is_map(document)
      assert document["version"] == "2.25.0"
      assert is_list(document["blocks"])
    end

    test "markdown_to_editorjs uses configured json_library" do
      Application.put_env(:exditorjs, :json_library, JSON, persistent: true)
      markdown = "# Test Heading"
      {:ok, document} = ExditorJS.markdown_to_editorjs(markdown)

      assert is_map(document)
      assert document["version"] == "2.25.0"
      assert is_list(document["blocks"])
    end

    test "functions work with Jason when configured" do
      Application.put_env(:exditorjs, :json_library, Jason, persistent: true)
      html = "<p>Test paragraph</p>"
      {:ok, document} = ExditorJS.html_to_editorjs(html)

      assert document["version"] == "2.25.0"
      assert Enum.any?(document["blocks"], fn b -> b["type"] == "paragraph" end)
    end

    test "functions work with JSON when configured" do
      Application.put_env(:exditorjs, :json_library, JSON, persistent: true)
      markdown = "- Item 1\n- Item 2"
      {:ok, document} = ExditorJS.markdown_to_editorjs(markdown)

      assert document["version"] == "2.25.0"
      assert Enum.any?(document["blocks"], fn b -> b["type"] == "list" end)
    end
  end
end
