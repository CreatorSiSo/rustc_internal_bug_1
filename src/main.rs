use chumsky::input::SpannedInput;
use chumsky::prelude::*;

fn main() {
    type Span = std::ops::Range<usize>;

    #[derive(Clone, PartialEq, Eq)]
    enum Token {
        Ident,
    }
    struct State<'a>(&'a str);

    let ident_parser =
        just::<_, SpannedInput<Token, Span, &[(Token, Span)]>, extra::State<State>>(Token::Ident)
            .map_with_state(|_, span, state: State| state.0[span]);

    ident_parser.parse_with_state(&[(Token::Ident, 0..7)].spanned(7..7), State("testing"));
}
