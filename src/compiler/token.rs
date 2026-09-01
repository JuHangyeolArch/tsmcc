use super::SpanInfo;
use super::SnapShotPhase;
use super::ProducedBy;


use std::ops::{Index,Range};
use std::slice::SliceIndex;

#[derive(Debug, Clone, PartialEq)]
pub struct TokenList {
    tokens: Vec<Token>,
}

impl TokenList {

    pub fn new() -> Self{
        TokenList {
            tokens: Vec::new(),
        }
    }
    
    pub fn push(&mut self,token: Token) {
        self.tokens.push(token);
    }
    
    pub fn as_iter(&self) -> TokenIteration<'_> {
        TokenIteration::new( self)
    }

    pub fn get(&self, token_pos: usize) -> Option<Token> {
        Some(self.tokens.get(token_pos)?.clone())
    }

    pub fn match_pair<P,Q>(
        tokens:
        &[Token],
        car: &mut P,
        cdr: &mut Q,
    ) -> Option<Range<usize>>
where
P: FnMut(&Token) -> bool,
Q: FnMut(&Token) -> bool,
    {

        let mut it = tokens.iter();
        
        let match_start = it
            .position(&mut *car)?;
        
        let mut depth: i32 = 0;
        let match_end = it
            .position(                
                |t| {
                    match t {
                        t if car(t) => {depth +=1;},
                        t if cdr(t) => {depth -=1;},
                        _ => {},
                    }
                    depth > 0
                }
            )?;
        Some(match_start..match_end)
    }

    pub fn position_in<P>(&self,
        range: Range<usize>,
        predicate: P
    ) -> Option<usize>
where
P: FnMut(&Token) -> bool
    {
        Some(self[range.clone()]
            .iter()
            .position(predicate)? + range.start
        )
    }

    pub fn position_last_in<P>(&self,
        range: Range<usize>,
        predicate: &mut P
    ) -> Option<usize>
where
P: FnMut(&Token) -> bool
    {
        let mut last_index: Option<usize> = None;
        let mut cur_range = range;
        
        while let Some(i) = self.position_in(cur_range.clone(), &mut *predicate) {
            last_index = Some(i);
            cur_range = last_index.unwrap()..cur_range.end;
        }
        
        last_index
    }



    
    pub fn match_pair_in<P,Q>(
        &self,
        range: Range<usize>,
        car: &mut P,
        cdr: &mut Q,
    ) -> Option<Range<usize>>
where
P: FnMut(&Token) -> bool,
Q: FnMut(&Token) -> bool,
    {

        let matched_range = TokenList::match_pair(
            &self[range.clone()],
            car,
            cdr,
        )?;

        Some((matched_range.start + range.start)..(matched_range.end + range.start))
        
    }

    pub fn pair_start_from<P,Q>(
        &self,
        range: Range<usize>,
        car: &mut P,
        cdr: &mut Q,
    ) -> Option<Range<usize>>
where
P: FnMut(&Token) -> bool,
Q: FnMut(&Token) -> bool,
    {
        if car(&self.get(range.start).unwrap()) {
                return None;
        }
        
        self.match_pair_in(range.clone(),car,cdr)
        
    }


    

    
    pub fn split_by<P>(&self,
        range: Range<usize>,
        predicate: P
    ) -> Range<usize>
where
P: FnMut(&Token) -> bool
    {
        if let Some(p) = self.position_in(range.clone(), predicate) {
            range.start..p
        } else {
            range
        } 
    }

    pub fn is_single(range: Range<usize>) -> bool {
        range.start + 1 == range.end
    }
    
}

impl<I> Index<I> for TokenList
where
I: SliceIndex<[Token]>
{
    type Output = I::Output;
    
    fn index(&self, index: I) -> &Self::Output {
        &self.tokens[index]
    }

}


pub struct TokenIteration<'a> {
    token_list: &'a TokenList,
    pos: usize,
}

impl<'a> TokenIteration<'a> {

    fn new(token_list: &'a TokenList) -> Self {
        TokenIteration{
            token_list: token_list,
            pos: 0,
        }
    }
}

impl<'a> Iterator for TokenIteration<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.token_list.get(self.pos) {
            Some(token) => Some(token.clone()),
            None => None,
        }

    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    token_type: TokenType,
    start_pos: usize,
    end_pos: usize,
    span_info: Option<SpanInfo>,
}

impl Token {
    pub fn new(token_type: TokenType, start_pos: usize, end_pos: usize) -> Self {
        Token {
            token_type: token_type,
            start_pos: start_pos,
            end_pos: end_pos,
            span_info: Some(
                SpanInfo::new(SnapShotPhase::Lexer, Some(start_pos),Some( end_pos))
            ),
        }
    }

    pub fn kind(&self) -> TokenType {
        self.token_type.clone()
    }
}

impl ProducedBy for Token {
    fn produced_by(&self) ->  Option<SpanInfo> {
        self.span_info.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {

    Eof,
    
    //Identifiers
    Id(String),


    //Numbers
    Number(i32),

    // Keywords
    Int,
    If,
    Else,
    While,
    Return,

    //Operators
    Plus,
    Minus,
    Star,
    Slash,
    Assign,
    Equal,
    Amper,

    //Delimiters
    LParenth,
    RParenth,
    LBrace,
    RBrace,
    Semic,
    Comma,

}
