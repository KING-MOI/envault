# envault

Paste a key once. Approve each directory. Keep secrets out of status output.

A local CLI + MCP server for coding agents: an agent runs `envault need OPENAI_API_KEY`; a stash hit is written to the project's env file once you have approved that directory, a miss files a task you answer once in a localhost inbox. `need` returns an exit code and a status line without echoing the value. The value goes from your paste to the OS keychain to the env file, which any process in that directory (the agent included) can read.

```bash
brew install pekkaiscool/envault/envault   # or: npm install -g envault · uv tool install envault
cargo install --locked --git https://github.com/pekkaiscool/envault envault
envault init
```

Docs, the provider registry and the changelog: <https://github.com/pekkaiscool/envault>.
