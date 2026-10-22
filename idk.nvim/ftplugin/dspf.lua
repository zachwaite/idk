local dspf = require("dspf")
dspf.setup()
dspf.apply_highlights()
vim.api.nvim_create_user_command("DspfApplyHighlights", dspf.apply_highlights, {})
