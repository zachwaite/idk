local idk = require("idk")
idk.setup_rpgle()
vim.api.nvim_create_user_command("RpgleRunTools", idk.run_rpgle_tools, {})
vim.api.nvim_create_user_command("RpgleHighlight", idk.highlight_rpgle, {})
idk.highlight_rpgle()

-- searching
vim.cmd([[set ignorecase]])

-- Use shift + k to grep the word under the cursor and open the quickfix list with results
-- use the quickfix list to inspect the matches
vim.cmd([[nnoremap K :w<CR>mk:vimgrep! /<C-R><C-W>\c/ %<CR>:cw<CR><ENTER>]])

-- close the quickfix list with ctl + x
-- redundant with db2.nvim
vim.cmd([[nnoremap <C-x> :ccl<CR><ENTER>'k]])

--
-- vim.cmd([[vnoremap <C-x> mk<CR>:lua require('rpgle').remove_assignment_middleman()<CR>'k]])
