local lfdds = require("lfdds")
lfdds.setup()
lfdds.apply_highlights()
vim.api.nvim_create_user_command("LfddsApplyHighlights", lfdds.apply_highlights, {})
