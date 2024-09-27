-- set marks 'F ,'D in rpg files
local Marker = function(buf)
  local self = { buf = buf }

  function self.pad10(line_number)
    local line_text = vim.api.nvim_buf_get_lines(self.buf, line_number, line_number + 1, false)[1]
    if line_text ~= nil then
      local line_len = string.len(line_text)
      if line_len < 10 then
        local n = 10 - line_len
        local new_text = line_text .. string.rep(" ", n, "")
        vim.api.nvim_buf_set_text(
          self.buf,
          line_number, 0, line_number, string.len(line_text),
          { new_text }
        )
      end
    end
  end

  function self.get_formtype(line_number)
    local line_text = vim.api.nvim_buf_get_lines(self.buf, line_number, line_number + 1, false)[1]
    if line_text ~= nil then
      local line_len = string.len(line_text)
      if line_len > 8 then
        local ch = string.sub(line_text, 6, 6)
        local peek = string.sub(line_text, 7, 7)
        if ch == "D" and peek ~= "*" and peek ~= "/" then
          return "D"
        elseif ch == "F" and peek ~= "*" and peek ~= "/" then
          return "F"
        else
          return nil
        end
      end
    end
  end

  function self.set_marks()
    local pos = vim.api.nvim_win_get_cursor(0)
    local lines_in_file = vim.api.nvim_buf_line_count(self.buf)
    local state = { ["F"] = nil, ["D"] = nil }
    for line = 0, lines_in_file - 1 do
      local formtype = self.get_formtype(line)
      if formtype == "F" and state.F == nil then
        state.F = line + 1
      end
      if formtype == "D" and state.D == nil then
        state.D = line + 1
      end
    end
    if state.F ~= nil then
      vim.api.nvim_buf_set_mark(self.buf, "F", state.F, 0, {})
    end
    if state.D ~= nil then
      vim.api.nvim_buf_set_mark(self.buf, "D", state.D, 0, {})
    end
    vim.api.nvim_win_set_cursor(0, pos);
  end

  return self
end

-- Public API
local M = {}

function M.setup(_)
end

function M.set_marks()
  local current_buf = vim.api.nvim_get_current_buf()
  local marker = Marker(current_buf)
  marker.set_marks()
end

return M
