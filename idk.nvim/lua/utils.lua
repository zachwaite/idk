local function get_visual_selection()
  local s_start = vim.fn.getpos("'<")
  local s_end = vim.fn.getpos("'>")
  local n_lines = math.abs(s_end[2] - s_start[2]) + 1
  local lines = vim.api.nvim_buf_get_lines(0, s_start[2] - 1, s_end[2], false)
  lines[1] = string.sub(lines[1], s_start[3], -1)
  if n_lines == 1 then
    lines[n_lines] = string.sub(lines[n_lines], 1, s_end[3] - s_start[3] + 1)
  else
    lines[n_lines] = string.sub(lines[n_lines], 1, s_end[3])
  end
  return table.concat(lines, '\n')
end

local function get_cursor_word()
  return vim.fn.escape(vim.fn.expand('<cword>'), [[\/]])
end

local function mk_tag_item(name, range, uri, offset_encoding)
  local bufnr = vim.uri_to_bufnr(uri)
  -- This is get_line_byte_from_position is 0-indexed, call cursor expects a 1-indexed position
  local byte = vim.lsp.util._get_line_byte_from_position(bufnr, range.start, offset_encoding) + 1
  return {
    name = name,
    filename = vim.uri_to_fname(uri),
    cmd = string.format([[/\%%%dl\%%%dc/]], range.start.line + 1, byte),
  }
end

local M = {}
M.get_visual_selection = get_visual_selection
M.get_cursor_word = get_cursor_word
M.mk_tag_item = mk_tag_item
return M
