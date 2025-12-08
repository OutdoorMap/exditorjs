# editorjs-converter

This Elixir library provides functionality to convert Markdown and HTML into Editor.js JSON format using a Rust backend for performance.

## Installation

To use the `exditorjs_native` library in your Elixir project, add it to your `mix.exs` dependencies:

```elixir
defp deps do
  [
    {:exditorjs_native, git: "https://github.com/yourusername/editorjs-converter.git"}
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
json_output = EditorjsConverter.convert_markdown(markdown_input)
```

### Converting HTML

```elixir
html_input = "<h1>Hello World</h1>"
json_output = EditorjsConverter.convert_html(html_input)
```

## Contributing

1. Fork the repository.
2. Create a new branch (`git checkout -b feature/your-feature`).
3. Make your changes and commit them (`git commit -m 'Add some feature'`).
4. Push to the branch (`git push origin feature/your-feature`).
5. Create a new Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.