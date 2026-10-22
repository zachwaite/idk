-- local KEYWORDS = require("lfdds.tokens").DDS_KEYWORDS

local M = {}
M.namespace_id = 0

local Highlighter = function(buf)
	local self = { buf = buf }
	function self.highlight(row, col, end_row, end_col, group)
		local current_buf = self.buf
		local current_marks = vim.api.nvim_buf_get_extmarks(
			current_buf,
			M.namespace_id,
			{ row, col - 1 },
			{ end_row, end_col - 1 },
			{}
		)
		if #current_marks > 0 then
			for _, m in ipairs(current_marks) do
				vim.api.nvim_buf_del_extmark(current_buf, M.namespace_id, m[1])
			end
		end
		vim.api.nvim_buf_set_extmark(
			current_buf,
			M.namespace_id,
			row,
			col - 1,
			{ end_row = end_row, end_col = end_col, hl_group = group }
		)
	end

	return self
end

function M.setup(_)
	vim.api.nvim_command(
		"highlight default LfddsRecordType guifg=#ff007c gui=bold ctermfg=198 cterm=bold ctermbg=darkgreen"
	)
	vim.api.nvim_command("highlight default LfddsField guifg=#9ece6a gui=bold ctermfg=198 cterm=bold ctermbg=darkgreen")
	vim.api.nvim_command(
		"highlight default LfddsRefField guifg=#1abc9c gui=bold ctermfg=198 cterm=bold ctermbg=darkgreen"
	)
	vim.api.nvim_command("highlight default LfddsParam guifg=#915d03 gui=bold ctermfg=198 cterm=bold ctermbg=darkgreen")
	M.namespace_id = vim.api.nvim_create_namespace("LFDDSNamespace")
end

function M.apply_naive_highlights()
	local current_buf = vim.api.nvim_get_current_buf()
	local lines_in_file = vim.api.nvim_buf_line_count(current_buf)
	local highlighter = Highlighter(current_buf)
	vim.api.nvim_buf_clear_namespace(current_buf, M.namespace_id, 0, -1)

	for i = 0, lines_in_file - 1 do
		local current_line = vim.api.nvim_buf_get_lines(current_buf, i, i + 1, false)[1]
		local mappings = {
			{
				["name"] = "sequence number",
				["start"] = 1,
				["end"] = 5,
				["highlight_group"] = "Comment",
				["predicate"] = function(line)
					return string.sub(line, 1, 1) ~= " "
				end,
			},
			{
				["name"] = "form_type",
				["start"] = 6,
				["end"] = 6,
				["highlight_group"] = "Comment",
				["predicate"] = function(line)
					return string.sub(line, 6, 6) == "A"
				end,
			},
			{
				["name"] = "comment",
				["start"] = 7,
				["end"] = nil,
				["highlight_group"] = "Comment",
				["predicate"] = function(line)
					return string.sub(line, 7, 7) == "*"
				end,
			},
			{
				["name"] = "condition",
				["start"] = 7,
				["end"] = 16,
				["highlight_group"] = "ErrorMsg",
				["predicate"] = function(line)
					return string.sub(line, 7, 7) ~= "*"
				end,
			},
			{
				["name"] = "record_type_and_name",
				["start"] = 17,
				["end"] = 28,
				["highlight_group"] = "LfddsRecordType",
				["predicate"] = function(line)
					return string.sub(line, 17, 17) == "R"
				end,
			},
			{
				["name"] = "field_name",
				["start"] = 19,
				["end"] = 28,
				["highlight_group"] = "LfddsField",
				["predicate"] = function(line)
					return string.sub(line, 7, 7) ~= "*"
						and string.sub(line, 17, 17) ~= "R"
						and string.sub(line, 29, 29) ~= "R"
				end,
			},
			{
				["name"] = "ref_field_indicator",
				["start"] = 29,
				["end"] = 29,
				["highlight_group"] = "LfddsRefField",
				["predicate"] = function(line)
					return string.sub(line, 7, 7) ~= "*"
						and string.sub(line, 17, 17) ~= "R"
						and string.sub(line, 29, 29) == "R"
				end,
			},
			{
				["name"] = "length_shift_and_decimals",
				["start"] = 30,
				["end"] = 37,
				["highlight_group"] = "ErrorMsg",
				["predicate"] = function(line)
					return string.sub(line, 7, 7) ~= "*" and string.sub(line, 17, 17) ~= "R"
				end,
			},
			{
				["name"] = "io_usage",
				["start"] = 38,
				["end"] = 38,
				["highlight_group"] = "LfddsIOUsage",
				["predicate"] = function(line)
					local is_valid_type = false
					local valid_types = {
						[" "] = true, -- output only
						["O"] = true, -- output only
						["I"] = true, -- input only
						["B"] = true, -- both input and output
						["H"] = true, -- hidden / invisible input and output field
						["M"] = true, -- message output field (has specific keykword restrictions)
					}
					if valid_types[string.sub(line, 38, 38)] ~= nil then
						is_valid_type = true
					end
					return string.sub(line, 7, 7) ~= "*" and string.sub(line, 17, 17) ~= "R" and is_valid_type
				end,
			},
			{
				["name"] = "keyword_entries",
				["start"] = 45,
				["end"] = 80,
				["highlight_group"] = "keyword",
				["predicate"] = function(line)
					return string.sub(line, 7, 7) ~= "*"
				end,
			},
			{
				["name"] = "constant_field",
				["start"] = 45,
				["end"] = 80,
				["highlight_group"] = "string",
				["predicate"] = function(line)
					local entries = string.sub(line, 45, 80)
					local first_char = string.sub(entries, 1, 1)
					local reverse_entries = string.reverse(entries)
					local last_char = ""
					for idx = 1, #reverse_entries do
						last_char = string.sub(reverse_entries, idx, idx)
						if last_char ~= "" and last_char ~= " " then
							break
						end
					end
					return string.sub(line, 7, 7) ~= "*"
						and string.sub(line, 17, 17) ~= "R"
						and string.sub(line, 17, 38) == "                      "
						and first_char == "'"
						and last_char == "'"
				end,
			},
		}

		for _, v in pairs(mappings) do
			local e = string.len(current_line)
			if e == 0 then
				break
			end
			if v["end"] ~= nil then
				e = math.min(v["end"], e)
			end

			if v["predicate"] ~= nil then
				if v["predicate"](current_line) then
					highlighter.highlight(i, v["start"], i, e, v["highlight_group"])
				end
			else
				highlighter.highlight(i, v["start"], i, e, v["highlight_group"])
			end
		end
	end
end

function M.apply_semantic_highlights() end

function M.apply_highlights()
	M.apply_naive_highlights()
	M.apply_semantic_highlights()
end

return M
