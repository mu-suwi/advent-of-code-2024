// day 25 - code chronicle

// in this puzzle we're parsing keys and locks and seeing how many keys
// can fit in each lock by comparing their pin heights.
// sounds a little too easy..............

use mlua::Lua;

pub fn main(input: &str) {
    let lua = Lua::new();

    lua.globals().set("input", input).unwrap();

    lua.load("local locks = {} local keys = {} input = string.gsub(input,
      \"\\n\\n\", \"O\") for grid in string.gmatch(input, \"([^O]+)\") do grid =
      string.gsub(grid, \"\\n\", \"\") if string.sub(grid, 1, 5) == \"#####\" 
      then table.insert(locks, grid) else table.insert(keys, grid) end end local
      function fits(k, l) if k == nil or l == nil then return false end for
      i=1,#l do if string.sub(k, i, i) == \"#\" and string.sub(l, i, i) ==
      \"#\" then return false end end return true end local matches = 0 for
      i=1,#locks do local lock = locks[i] for j=1,#keys do local key = keys[j]
      if fits(key, lock) then matches = matches + 1 end end end
      print(\"matches: \"..matches)",)
    .exec()
    .unwrap();

    // part 2

    // shhhhh... ***** ***** ***** ***** ***** ***** ***** ***** ***** *****
}
