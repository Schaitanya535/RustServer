# LSP Setup

This is the LSP server implemented using tower_lsp.

To test this server, you need a lsp client. One simplest way is to use Neovim.

This code need to be present (or referenced) in the neovim config file. (~/.config/nvim/init.vim)

```lua

local client = vim.lsp.start_client {
  cmd = { 'Path/to/this/binary' },
  name = 'trying_lsp',
}

if not client then
  vim.notify "hey you didn't do the client well"
  return
end

vim.api.nvim_create_autocmd('FileType', {
  pattern = 'markdown',
  callback = function()
    vim.lsp.buf_attach_client(0, client)
    vim.notify 'Client started'
  end,
})

return {}
```

This code says NeoVim to start up a language server ( which is at the specified location) on opening of a file of type markdown.

So, once we start the neovim, and open a markdown file, the language server will start and we can see the logs in the log file we are writing to.
