local rpgle_formatter = require("luarpgle.formatter")
local rpgle_marker = require("luarpgle.marker")
local libidk = require("libidk")
local utils = require("utils")

local function tfunc(pattern, _)
  local tagitem = libidk.getdef(pattern)
  if tagitem then
    local name = tagitem.name
    local range = {
      ['start'] = { ['line'] = tagitem.start_line, ['character'] = tagitem.start_char },
      ['end'] = { ['line'] = tagitem.end_line, ['character'] = tagitem.end_char }
    }
    local uri = "file://" .. vim.api.nvim_buf_get_name(0)
    if tagitem.uri then
      uri = tagitem.uri
    end
    local offset_encoding = 'utf-8'
    local item = utils.mk_tag_item(name, range, uri, offset_encoding)
    item.kind = 'Unknown'
    return { item }
  else
    return vim.NIL
  end
end

local config = {}

local M = {}

M.config = config
M.setup = function(_)
end

M.setup_rpgle = function(args)
  M.config = vim.tbl_deep_extend("force", M.config, args or {})
  rpgle_formatter.setup()
  vim.bo[0].tagfunc = "v:lua.require'idk'.tfunc"
  vim.opt.iskeyword:append("$")
end

M.tfunc = tfunc

-- rpgle specific
M.highlight_rpgle = function()
  libidk.highlight_rpgle()
end

M.format_rpgle = function()
  rpgle_formatter.format_all()
end

M.mark_rpgle = function()
  rpgle_marker.set_marks()
end

M.run_rpgle_tools = function()
  rpgle_formatter.format_all()
  libidk.highlight_rpgle()
end

M.json_dump_current_buffer = function(path)
  libidk.json_dump_current_buffer(path)
end

M.dot_dump_current_buffer = function(path)
  libidk.dot_dump_current_buffer(path)
end

-- pfdds specific
M.highlight_pfdds = function()
  libidk.highlight_pfdds()
end

return M
