local Formatter = function(buf)
  local self = { buf = buf }

  function self.pad100(line_number)
    local line_text = vim.api.nvim_buf_get_lines(self.buf, line_number, line_number + 1, false)[1]
    if line_text ~= nil then
      local line_len = string.len(line_text)
      if line_len < 100 then
        local n = 100 - line_len
        local new_text = line_text .. string.rep(" ", n, "")
        vim.api.nvim_buf_set_text(
          self.buf,
          line_number, 0, line_number, string.len(line_text),
          { new_text }
        )
      end
    end
  end

  function self.trim100(line_number)
    local line_text = vim.api.nvim_buf_get_lines(self.buf, line_number, line_number + 1, false)[1]
    if line_text ~= nil then
      local line_len = string.len(line_text)
      if line_len > 100 then
        local n = line_len - 100
        local whitespace = string.rep(" ", n - 1)
        local tail = vim.api.nvim_buf_get_text(self.buf, line_number, 101, line_number, line_len, {})[1]
        if tail == whitespace then
          local new_text = string.sub(line_text, 1, 100)
          vim.api.nvim_buf_set_text(
            self.buf,
            line_number, 0, line_number, string.len(line_text),
            { new_text }
          )
        end
      end
    end
  end

  function self.format_line(line_number)
    self.pad100(line_number)
    self.trim100(line_number)
  end

  function self.remove_nonprintable()
    vim.cmd(":silent! %s/[^[:print:]]/ /g")
  end

  function self.format_all()
    local pos = vim.api.nvim_win_get_cursor(0)
    self.remove_nonprintable()
    local lines_in_file = vim.api.nvim_buf_line_count(self.buf)
    for line = 0, lines_in_file - 1 do
      self.format_line(line)
    end
    vim.api.nvim_win_set_cursor(0, pos);
  end

  return self
end

-- Public API
local M = {}

function M.setup(_)
end

function M.format_all()
  local current_buf = vim.api.nvim_get_current_buf()
  local formatter = Formatter(current_buf)
  formatter.format_all()
end

return M
