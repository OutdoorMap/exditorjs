# exditorjs

This Elixir library provides functionality to convert Markdown and HTML into Editor.js JSON format using a Rust backend for performance.

## Installation

To use the `exditorjs` library in your Elixir project, add it to your `mix.exs` dependencies:

```elixir
defp deps do
  [
    {:exditorjs, git: "https://github.com/OutdoorMap/exditorjs.git"}
  ]
end
```

Then, run the following command to fetch the dependency:

```bash
mix deps.get
```

## Usage

After installing the library, you can use it to convert Markdown or HTML to Editor.js JSON format.

### Converting Markdown

```elixir
markdown_input = "# Hello World"
{:ok, json_output} = ExditorJS.markdown_to_editorjs(markdown_input)
```

Returns

```
{:ok,
 %{
   "blocks" => [
     %{
       "data" => %{"level" => 1, "text" => "Hello World"},
       "id" => "jFAGy00fr2",
       "type" => "heading"
     }
   ],
   "time" => 1765198639892,
   "version" => "2.25.0"
 }}
```

### Converting HTML

```elixir
html_input = "<h1>Hello World</h1>"
{:ok, json_output} = ExditorJS.html_to_editorjs(html_input)
```

Returns

```
{:ok,
 %{
   "blocks" => [
     %{
       "data" => %{"level" => 1, "text" => "Hello World"},
       "id" => "MpNdrP5nOK",
       "type" => "heading"
     }
   ],
   "time" => 1765198666516,
   "version" => "2.25.0"
 }}
```

## Supported Data Structure

### Image Block

Image blocks support the following fields:

- `url` (required): The image URL
- `caption` (optional): Image caption text
- `with_border` (optional, boolean): Display image with border (formerly `withBorder`)
- `with_background` (optional, boolean): Display image with background (formerly `withBackground`)
- `stretched` (optional, boolean): Stretch image to full width

### Embed Block

The Embed tool supports embedding content from various services. It automatically detects and converts URLs from supported services into embed blocks.

**Supported Services:**
- YouTube (youtube.com, youtu.be)
- Vimeo (vimeo.com)
- Coub (coub.com)
- Instagram (instagram.com)
- Twitter/X (twitter.com, x.com)
- Twitch (twitch.tv) - both videos and channels
- CodePen (codepen.io)
- GitHub Gist (gist.github.com)
- Figma (figma.com)
- Miro (miro.com)
- Imgur (imgur.com)
- Pinterest (pinterest.com)

**Embed Block Fields:**
- `service` (string): Service identifier (e.g., "youtube", "vimeo")
- `source` (string): Original URL
- `embed` (string): Embed/iframe URL
- `width` (number): Embed width in pixels
- `height` (number): Embed height in pixels
- `caption` (optional): Caption text for the embed

**Usage Examples:**

#### HTML with iframe:
```elixir
html_input = ~s|<iframe src="https://www.youtube.com/embed/dQw4w9WgXcQ" width="560" height="315"></iframe>|
{:ok, json_output} = ExditorJS.html_to_editorjs(html_input)
```

Returns:
```
{:ok,
 %{
   "blocks" => [
     %{
       "data" => %{
         "service" => "youtube",
         "source" => "https://www.youtube.com/embed/dQw4w9WgXcQ",
         "embed" => "https://www.youtube.com/embed/dQw4w9WgXcQ",
         "width" => 560,
         "height" => 315,
         "caption" => null
       },
       "id" => "aBcDefGhIj",
       "type" => "embed"
     }
   ],
   "time" => 1765198639892,
   "version" => "2.25.0"
 }}
```

#### Markdown with URL:
```elixir
markdown_input = "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
{:ok, json_output} = ExditorJS.markdown_to_editorjs(markdown_input)
```

#### Markdown with link:
```elixir
markdown_input = "[Check this out](https://vimeo.com/123456789)"
{:ok, json_output} = ExditorJS.markdown_to_editorjs(markdown_input)
```

### List Block

List blocks support the following fields:

- `style` (required): List style - `"ordered"` or `"unordered"`
- `items` (required): Array of list items
- `meta` (optional): List metadata
  - `start` (optional): Starting number for ordered lists
  - `counter_type` (optional): Counter type for ordered lists (formerly `counterType`)

## Migration Notes

As of recent updates, the following attribute names have been changed to follow Rust naming conventions (snake_case):

| Old Name | New Name |
|----------|----------|
| `withBorder` | `with_border` |
| `withBackground` | `with_background` |
| `counterType` | `counter_type` |

For backward compatibility with existing JSON data, the library supports deserialization using both old and new field names via serde aliases. This means existing code that uses the old names will continue to work when parsing JSON responses.

## Contributing

1. Fork the repository.
2. Create a new branch (`git checkout -b feature/your-feature`).
3. Make your changes and commit them (`git commit -m 'Add some feature'`).
4. Push to the branch (`git push origin feature/your-feature`).
5. Create a new Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.