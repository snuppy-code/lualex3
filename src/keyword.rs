#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Keyword {
//  and       break     do        else      elseif    end
//  false     for       function  goto      if        in
//  local     nil       not       or        repeat    return
//  then      true      until     while
    And,       Break,     Do,        Else,      Elseif,    End,
    False,     For,       Function,  Goto,      If,        In,
    Local,     Nil,       Not,       Or,        Repeat,    Return,
    Then,      True,      Until,     While,
}