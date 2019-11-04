// A Bison parser, made by GNU Bison 3.3.2.

// Skeleton implementation for Bison LALR(1) parsers in C++

// Copyright (C) 2002-2015, 2018-2019 Free Software Foundation, Inc.

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

// As a special exception, you may create a larger work that contains
// part or all of the Bison parser skeleton and distribute that work
// under terms of your choice, so long as that work isn't itself a
// parser generator using the skeleton or a modified version thereof
// as a parser skeleton.  Alternatively, if you modify or redistribute
// the parser skeleton itself, you may (at your option) remove this
// special exception, which will cause the skeleton and the resulting
// Bison output files to be licensed under the GNU General Public
// License without this special exception.

// This special exception was added by the Free Software Foundation in
// version 2.2 of Bison.

// Undocumented macros, especially those whose name start with YY_,
// are private implementation details.  Do not rely on them.


// Take the name prefix into account.
#define yylex   GringoNonGroundGrammar_lex

// First part of user prologue.
#line 58 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:429


#include "gringo/input/nongroundparser.hh"
#include "gringo/input/programbuilder.hh"
#include <climits> 

#define BUILDER (lexer->builder())
#define LOGGER (lexer->logger())
#define YYLLOC_DEFAULT(Current, Rhs, N)                                \
    do {                                                               \
        if (N) {                                                       \
            (Current).beginFilename = YYRHSLOC (Rhs, 1).beginFilename; \
            (Current).beginLine     = YYRHSLOC (Rhs, 1).beginLine;     \
            (Current).beginColumn   = YYRHSLOC (Rhs, 1).beginColumn;   \
            (Current).endFilename   = YYRHSLOC (Rhs, N).endFilename;   \
            (Current).endLine       = YYRHSLOC (Rhs, N).endLine;       \
            (Current).endColumn     = YYRHSLOC (Rhs, N).endColumn;     \
        }                                                              \
        else {                                                         \
            (Current).beginFilename = YYRHSLOC (Rhs, 0).endFilename;   \
            (Current).beginLine     = YYRHSLOC (Rhs, 0).endLine;       \
            (Current).beginColumn   = YYRHSLOC (Rhs, 0).endColumn;     \
            (Current).endFilename   = YYRHSLOC (Rhs, 0).endFilename;   \
            (Current).endLine       = YYRHSLOC (Rhs, 0).endLine;       \
            (Current).endColumn     = YYRHSLOC (Rhs, 0).endColumn;     \
        }                                                              \
    }                                                                  \
    while (false)

using namespace Gringo;
using namespace Gringo::Input;

int GringoNonGroundGrammar_lex(void *value, Gringo::Location* loc, NonGroundParser *lexer) {
    return lexer->lex(value, *loc);
}


#line 80 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:429

#include "grammar.hh"


// Unqualified %code blocks.
#line 96 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:435


void NonGroundGrammar::parser::error(DefaultLocation const &l, std::string const &msg) {
    lexer->parseError(l, msg);
}


#line 94 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:435


#ifndef YY_
# if defined YYENABLE_NLS && YYENABLE_NLS
#  if ENABLE_NLS
#   include <libintl.h> // FIXME: INFRINGES ON USER NAME SPACE.
#   define YY_(msgid) dgettext ("bison-runtime", msgid)
#  endif
# endif
# ifndef YY_
#  define YY_(msgid) msgid
# endif
#endif

// Whether we are compiled with exception support.
#ifndef YY_EXCEPTIONS
# if defined __GNUC__ && !defined __EXCEPTIONS
#  define YY_EXCEPTIONS 0
# else
#  define YY_EXCEPTIONS 1
# endif
#endif

#define YYRHSLOC(Rhs, K) ((Rhs)[K].location)
/* YYLLOC_DEFAULT -- Set CURRENT to span from RHS[1] to RHS[N].
   If N is 0, then set CURRENT to the empty location which ends
   the previous symbol: RHS[0] (always defined).  */

# ifndef YYLLOC_DEFAULT
#  define YYLLOC_DEFAULT(Current, Rhs, N)                               \
    do                                                                  \
      if (N)                                                            \
        {                                                               \
          (Current).begin  = YYRHSLOC (Rhs, 1).begin;                   \
          (Current).end    = YYRHSLOC (Rhs, N).end;                     \
        }                                                               \
      else                                                              \
        {                                                               \
          (Current).begin = (Current).end = YYRHSLOC (Rhs, 0).end;      \
        }                                                               \
    while (false)
# endif


// Suppress unused-variable warnings by "using" E.
#define YYUSE(E) ((void) (E))

// Enable debugging if requested.
#if YYDEBUG

// A pseudo ostream that takes yydebug_ into account.
# define YYCDEBUG if (yydebug_) (*yycdebug_)

# define YY_SYMBOL_PRINT(Title, Symbol)         \
  do {                                          \
    if (yydebug_)                               \
    {                                           \
      *yycdebug_ << Title << ' ';               \
      yy_print_ (*yycdebug_, Symbol);           \
      *yycdebug_ << '\n';                       \
    }                                           \
  } while (false)

# define YY_REDUCE_PRINT(Rule)          \
  do {                                  \
    if (yydebug_)                       \
      yy_reduce_print_ (Rule);          \
  } while (false)

# define YY_STACK_PRINT()               \
  do {                                  \
    if (yydebug_)                       \
      yystack_print_ ();                \
  } while (false)

#else // !YYDEBUG

# define YYCDEBUG if (false) std::cerr
# define YY_SYMBOL_PRINT(Title, Symbol)  YYUSE (Symbol)
# define YY_REDUCE_PRINT(Rule)           static_cast<void> (0)
# define YY_STACK_PRINT()                static_cast<void> (0)

#endif // !YYDEBUG

#define yyerrok         (yyerrstatus_ = 0)
#define yyclearin       (yyla.clear ())

#define YYACCEPT        goto yyacceptlab
#define YYABORT         goto yyabortlab
#define YYERROR         goto yyerrorlab
#define YYRECOVERING()  (!!yyerrstatus_)

#line 28 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:510
namespace Gringo { namespace Input { namespace NonGroundGrammar {
#line 189 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:510

  /* Return YYSTR after stripping away unnecessary quotes and
     backslashes, so that it's suitable for yyerror.  The heuristic is
     that double-quoting is unnecessary unless the string contains an
     apostrophe, a comma, or backslash (other than backslash-backslash).
     YYSTR is taken from yytname.  */
  std::string
  parser::yytnamerr_ (const char *yystr)
  {
    if (*yystr == '"')
      {
        std::string yyr;
        char const *yyp = yystr;

        for (;;)
          switch (*++yyp)
            {
            case '\'':
            case ',':
              goto do_not_strip_quotes;

            case '\\':
              if (*++yyp != '\\')
                goto do_not_strip_quotes;
              else
                goto append;

            append:
            default:
              yyr += *yyp;
              break;

            case '"':
              return yyr;
            }
      do_not_strip_quotes: ;
      }

    return yystr;
  }


  /// Build a parser object.
  parser::parser (Gringo::Input::NonGroundParser *lexer_yyarg)
    :
#if YYDEBUG
      yydebug_ (false),
      yycdebug_ (&std::cerr),
#endif
      lexer (lexer_yyarg)
  {}

  parser::~parser ()
  {}

  parser::syntax_error::~syntax_error () YY_NOEXCEPT YY_NOTHROW
  {}

  /*---------------.
  | Symbol types.  |
  `---------------*/

  // basic_symbol.
#if 201103L <= YY_CPLUSPLUS
  template <typename Base>
  parser::basic_symbol<Base>::basic_symbol (basic_symbol&& that)
    : Base (std::move (that))
    , value (std::move (that.value))
    , location (std::move (that.location))
  {}
#endif

  template <typename Base>
  parser::basic_symbol<Base>::basic_symbol (const basic_symbol& that)
    : Base (that)
    , value (that.value)
    , location (that.location)
  {}


  /// Constructor for valueless symbols.
  template <typename Base>
  parser::basic_symbol<Base>::basic_symbol (typename Base::kind_type t, YY_MOVE_REF (location_type) l)
    : Base (t)
    , value ()
    , location (l)
  {}

  template <typename Base>
  parser::basic_symbol<Base>::basic_symbol (typename Base::kind_type t, YY_RVREF (semantic_type) v, YY_RVREF (location_type) l)
    : Base (t)
    , value (YY_MOVE (v))
    , location (YY_MOVE (l))
  {}

  template <typename Base>
  bool
  parser::basic_symbol<Base>::empty () const YY_NOEXCEPT
  {
    return Base::type_get () == empty_symbol;
  }

  template <typename Base>
  void
  parser::basic_symbol<Base>::move (basic_symbol& s)
  {
    super_type::move (s);
    value = YY_MOVE (s.value);
    location = YY_MOVE (s.location);
  }

  // by_type.
  parser::by_type::by_type ()
    : type (empty_symbol)
  {}

#if 201103L <= YY_CPLUSPLUS
  parser::by_type::by_type (by_type&& that)
    : type (that.type)
  {
    that.clear ();
  }
#endif

  parser::by_type::by_type (const by_type& that)
    : type (that.type)
  {}

  parser::by_type::by_type (token_type t)
    : type (yytranslate_ (t))
  {}

  void
  parser::by_type::clear ()
  {
    type = empty_symbol;
  }

  void
  parser::by_type::move (by_type& that)
  {
    type = that.type;
    that.clear ();
  }

  int
  parser::by_type::type_get () const YY_NOEXCEPT
  {
    return type;
  }


  // by_state.
  parser::by_state::by_state () YY_NOEXCEPT
    : state (empty_state)
  {}

  parser::by_state::by_state (const by_state& that) YY_NOEXCEPT
    : state (that.state)
  {}

  void
  parser::by_state::clear () YY_NOEXCEPT
  {
    state = empty_state;
  }

  void
  parser::by_state::move (by_state& that)
  {
    state = that.state;
    that.clear ();
  }

  parser::by_state::by_state (state_type s) YY_NOEXCEPT
    : state (s)
  {}

  parser::symbol_number_type
  parser::by_state::type_get () const YY_NOEXCEPT
  {
    if (state == empty_state)
      return empty_symbol;
    else
      return yystos_[state];
  }

  parser::stack_symbol_type::stack_symbol_type ()
  {}

  parser::stack_symbol_type::stack_symbol_type (YY_RVREF (stack_symbol_type) that)
    : super_type (YY_MOVE (that.state), YY_MOVE (that.value), YY_MOVE (that.location))
  {
#if 201103L <= YY_CPLUSPLUS
    // that is emptied.
    that.state = empty_state;
#endif
  }

  parser::stack_symbol_type::stack_symbol_type (state_type s, YY_MOVE_REF (symbol_type) that)
    : super_type (s, YY_MOVE (that.value), YY_MOVE (that.location))
  {
    // that is emptied.
    that.type = empty_symbol;
  }

#if YY_CPLUSPLUS < 201103L
  parser::stack_symbol_type&
  parser::stack_symbol_type::operator= (stack_symbol_type& that)
  {
    state = that.state;
    value = that.value;
    location = that.location;
    // that is emptied.
    that.state = empty_state;
    return *this;
  }
#endif

  template <typename Base>
  void
  parser::yy_destroy_ (const char* yymsg, basic_symbol<Base>& yysym) const
  {
    if (yymsg)
      YY_SYMBOL_PRINT (yymsg, yysym);

    // User destructor.
    YYUSE (yysym.type_get ());
  }

#if YYDEBUG
  template <typename Base>
  void
  parser::yy_print_ (std::ostream& yyo,
                                     const basic_symbol<Base>& yysym) const
  {
    std::ostream& yyoutput = yyo;
    YYUSE (yyoutput);
    symbol_number_type yytype = yysym.type_get ();
#if defined __GNUC__ && ! defined __clang__ && ! defined __ICC && __GNUC__ * 100 + __GNUC_MINOR__ <= 408
    // Avoid a (spurious) G++ 4.8 warning about "array subscript is
    // below array bounds".
    if (yysym.empty ())
      std::abort ();
#endif
    yyo << (yytype < yyntokens_ ? "token" : "nterm")
        << ' ' << yytname_[yytype] << " ("
        << yysym.location << ": ";
    YYUSE (yytype);
    yyo << ')';
  }
#endif

  void
  parser::yypush_ (const char* m, YY_MOVE_REF (stack_symbol_type) sym)
  {
    if (m)
      YY_SYMBOL_PRINT (m, sym);
    yystack_.push (YY_MOVE (sym));
  }

  void
  parser::yypush_ (const char* m, state_type s, YY_MOVE_REF (symbol_type) sym)
  {
#if 201103L <= YY_CPLUSPLUS
    yypush_ (m, stack_symbol_type (s, std::move (sym)));
#else
    stack_symbol_type ss (s, sym);
    yypush_ (m, ss);
#endif
  }

  void
  parser::yypop_ (int n)
  {
    yystack_.pop (n);
  }

#if YYDEBUG
  std::ostream&
  parser::debug_stream () const
  {
    return *yycdebug_;
  }

  void
  parser::set_debug_stream (std::ostream& o)
  {
    yycdebug_ = &o;
  }


  parser::debug_level_type
  parser::debug_level () const
  {
    return yydebug_;
  }

  void
  parser::set_debug_level (debug_level_type l)
  {
    yydebug_ = l;
  }
#endif // YYDEBUG

  parser::state_type
  parser::yy_lr_goto_state_ (state_type yystate, int yysym)
  {
    int yyr = yypgoto_[yysym - yyntokens_] + yystate;
    if (0 <= yyr && yyr <= yylast_ && yycheck_[yyr] == yystate)
      return yytable_[yyr];
    else
      return yydefgoto_[yysym - yyntokens_];
  }

  bool
  parser::yy_pact_value_is_default_ (int yyvalue)
  {
    return yyvalue == yypact_ninf_;
  }

  bool
  parser::yy_table_value_is_error_ (int yyvalue)
  {
    return yyvalue == yytable_ninf_;
  }

  int
  parser::operator() ()
  {
    return parse ();
  }

  int
  parser::parse ()
  {
    // State.
    int yyn;
    /// Length of the RHS of the rule being reduced.
    int yylen = 0;

    // Error handling.
    int yynerrs_ = 0;
    int yyerrstatus_ = 0;

    /// The lookahead symbol.
    symbol_type yyla;

    /// The locations where the error started and ended.
    stack_symbol_type yyerror_range[3];

    /// The return value of parse ().
    int yyresult;

#if YY_EXCEPTIONS
    try
#endif // YY_EXCEPTIONS
      {
    YYCDEBUG << "Starting parse\n";


    /* Initialize the stack.  The initial state will be set in
       yynewstate, since the latter expects the semantical and the
       location values to have been already stored, initialize these
       stacks with a primary value.  */
    yystack_.clear ();
    yypush_ (YY_NULLPTR, 0, YY_MOVE (yyla));

  /*-----------------------------------------------.
  | yynewstate -- push a new symbol on the stack.  |
  `-----------------------------------------------*/
  yynewstate:
    YYCDEBUG << "Entering state " << yystack_[0].state << '\n';

    // Accept?
    if (yystack_[0].state == yyfinal_)
      YYACCEPT;

    goto yybackup;


  /*-----------.
  | yybackup.  |
  `-----------*/
  yybackup:
    // Try to take a decision without lookahead.
    yyn = yypact_[yystack_[0].state];
    if (yy_pact_value_is_default_ (yyn))
      goto yydefault;

    // Read a lookahead token.
    if (yyla.empty ())
      {
        YYCDEBUG << "Reading a token: ";
#if YY_EXCEPTIONS
        try
#endif // YY_EXCEPTIONS
          {
            yyla.type = yytranslate_ (yylex (&yyla.value, &yyla.location, lexer));
          }
#if YY_EXCEPTIONS
        catch (const syntax_error& yyexc)
          {
            YYCDEBUG << "Caught exception: " << yyexc.what() << '\n';
            error (yyexc);
            goto yyerrlab1;
          }
#endif // YY_EXCEPTIONS
      }
    YY_SYMBOL_PRINT ("Next token is", yyla);

    /* If the proper action on seeing token YYLA.TYPE is to reduce or
       to detect an error, take that action.  */
    yyn += yyla.type_get ();
    if (yyn < 0 || yylast_ < yyn || yycheck_[yyn] != yyla.type_get ())
      goto yydefault;

    // Reduce or error.
    yyn = yytable_[yyn];
    if (yyn <= 0)
      {
        if (yy_table_value_is_error_ (yyn))
          goto yyerrlab;
        yyn = -yyn;
        goto yyreduce;
      }

    // Count tokens shifted since error; after three, turn off error status.
    if (yyerrstatus_)
      --yyerrstatus_;

    // Shift the lookahead token.
    yypush_ ("Shifting", yyn, YY_MOVE (yyla));
    goto yynewstate;


  /*-----------------------------------------------------------.
  | yydefault -- do the default action for the current state.  |
  `-----------------------------------------------------------*/
  yydefault:
    yyn = yydefact_[yystack_[0].state];
    if (yyn == 0)
      goto yyerrlab;
    goto yyreduce;


  /*-----------------------------.
  | yyreduce -- do a reduction.  |
  `-----------------------------*/
  yyreduce:
    yylen = yyr2_[yyn];
    {
      stack_symbol_type yylhs;
      yylhs.state = yy_lr_goto_state_ (yystack_[yylen].state, yyr1_[yyn]);
      /* If YYLEN is nonzero, implement the default value of the
         action: '$$ = $1'.  Otherwise, use the top of the stack.

         Otherwise, the following line sets YYLHS.VALUE to garbage.
         This behavior is undocumented and Bison users should not rely
         upon it.  */
      if (yylen)
        yylhs.value = yystack_[yylen - 1].value;
      else
        yylhs.value = yystack_[0].value;

      // Default location.
      {
        stack_type::slice range (yystack_, yylen);
        YYLLOC_DEFAULT (yylhs.location, range, yylen);
        yyerror_range[1].location = yylhs.location;
      }

      // Perform the reduction.
      YY_REDUCE_PRINT (yyn);
#if YY_EXCEPTIONS
      try
#endif // YY_EXCEPTIONS
        {
          switch (yyn)
            {
  case 7:
#line 353 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { lexer->parseError(yylhs.location, "syntax error, unexpected ."); }
#line 673 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 10:
#line 359 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.str) = (yystack_[0].value.str); }
#line 679 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 11:
#line 360 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.str) = (yystack_[0].value.str); }
#line 685 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 12:
#line 361 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.str) = (yystack_[0].value.str); }
#line 691 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 13:
#line 368 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BinOp::XOR, (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 697 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 14:
#line 369 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BinOp::OR, (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 703 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 15:
#line 370 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BinOp::AND, (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 709 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 16:
#line 371 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BinOp::ADD, (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 715 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 17:
#line 372 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BinOp::SUB, (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 721 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 18:
#line 373 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BinOp::MUL, (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 727 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 19:
#line 374 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BinOp::DIV, (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 733 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 20:
#line 375 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BinOp::MOD, (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 739 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 21:
#line 376 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BinOp::POW, (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 745 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 22:
#line 377 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, UnOp::NEG, (yystack_[0].value.term)); }
#line 751 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 23:
#line 378 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, UnOp::NOT, (yystack_[0].value.term)); }
#line 757 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 24:
#line 379 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BUILDER.termvec(), false); }
#line 763 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 25:
#line 380 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BUILDER.termvec(), true); }
#line 769 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 26:
#line 381 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, (yystack_[1].value.termvec), false); }
#line 775 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 27:
#line 382 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, (yystack_[2].value.termvec), true); }
#line 781 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 28:
#line 383 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, String::fromRep((yystack_[3].value.str)), (yystack_[1].value.termvecvec), false); }
#line 787 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 29:
#line 384 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, String::fromRep((yystack_[3].value.str)), (yystack_[1].value.termvecvec), true); }
#line 793 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 30:
#line 385 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, UnOp::ABS, (yystack_[1].value.term)); }
#line 799 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 31:
#line 386 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, Symbol::createId(String::fromRep((yystack_[0].value.str)))); }
#line 805 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 32:
#line 387 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, String::fromRep((yystack_[0].value.str)), BUILDER.termvecvec(BUILDER.termvecvec(), BUILDER.termvec()), true); }
#line 811 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 33:
#line 388 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, Symbol::createNum((yystack_[0].value.num))); }
#line 817 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 34:
#line 389 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, Symbol::createStr(String::fromRep((yystack_[0].value.str)))); }
#line 823 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 35:
#line 390 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, Symbol::createInf()); }
#line 829 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 36:
#line 391 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, Symbol::createSup()); }
#line 835 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 37:
#line 397 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.termvec) = BUILDER.termvec(BUILDER.termvec(), (yystack_[0].value.term));  }
#line 841 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 38:
#line 398 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.termvec) = BUILDER.termvec((yystack_[2].value.termvec), (yystack_[0].value.term));  }
#line 847 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 39:
#line 402 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.termvecvec) = BUILDER.termvecvec(BUILDER.termvecvec(), (yystack_[0].value.termvec));  }
#line 853 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 40:
#line 403 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.termvecvec) = BUILDER.termvecvec(BUILDER.termvecvec(), BUILDER.termvec());  }
#line 859 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 41:
#line 409 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 865 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 42:
#line 410 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BinOp::XOR, (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 871 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 43:
#line 411 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BinOp::OR, (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 877 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 44:
#line 412 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BinOp::AND, (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 883 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 45:
#line 413 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BinOp::ADD, (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 889 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 46:
#line 414 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BinOp::SUB, (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 895 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 47:
#line 415 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BinOp::MUL, (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 901 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 48:
#line 416 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BinOp::DIV, (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 907 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 49:
#line 417 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BinOp::MOD, (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 913 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 50:
#line 418 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BinOp::POW, (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 919 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 51:
#line 419 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, UnOp::NEG, (yystack_[0].value.term)); }
#line 925 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 52:
#line 420 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, UnOp::NOT, (yystack_[0].value.term)); }
#line 931 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 53:
#line 421 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.pool(yylhs.location, (yystack_[1].value.termvec)); }
#line 937 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 54:
#line 422 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, String::fromRep((yystack_[3].value.str)), (yystack_[1].value.termvecvec), false); }
#line 943 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 55:
#line 423 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, String::fromRep((yystack_[3].value.str)), (yystack_[1].value.termvecvec), true); }
#line 949 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 56:
#line 424 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, UnOp::ABS, (yystack_[1].value.termvec)); }
#line 955 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 57:
#line 425 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, Symbol::createId(String::fromRep((yystack_[0].value.str)))); }
#line 961 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 58:
#line 426 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, String::fromRep((yystack_[0].value.str)), BUILDER.termvecvec(BUILDER.termvecvec(), BUILDER.termvec()), true); }
#line 967 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 59:
#line 427 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, Symbol::createNum((yystack_[0].value.num))); }
#line 973 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 60:
#line 428 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, Symbol::createStr(String::fromRep((yystack_[0].value.str)))); }
#line 979 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 61:
#line 429 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, Symbol::createInf()); }
#line 985 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 62:
#line 430 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, Symbol::createSup()); }
#line 991 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 63:
#line 431 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, String::fromRep((yystack_[0].value.str))); }
#line 997 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 64:
#line 432 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, String("_")); }
#line 1003 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 65:
#line 438 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.termvec) = BUILDER.termvec(BUILDER.termvec(), (yystack_[0].value.term)); }
#line 1009 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 66:
#line 439 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.termvec) = BUILDER.termvec((yystack_[2].value.termvec), (yystack_[0].value.term)); }
#line 1015 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 67:
#line 445 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.termvec) = BUILDER.termvec(BUILDER.termvec(), (yystack_[0].value.term)); }
#line 1021 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 68:
#line 446 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.termvec) = BUILDER.termvec((yystack_[2].value.termvec), (yystack_[0].value.term)); }
#line 1027 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 69:
#line 450 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.termvec) = (yystack_[0].value.termvec); }
#line 1033 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 70:
#line 451 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.termvec) = BUILDER.termvec(); }
#line 1039 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 71:
#line 455 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, (yystack_[1].value.termvec), true); }
#line 1045 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 72:
#line 456 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, (yystack_[0].value.termvec), false); }
#line 1051 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 73:
#line 457 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BUILDER.termvec(), true); }
#line 1057 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 74:
#line 458 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BUILDER.termvec(), false); }
#line 1063 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 75:
#line 461 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.termvec) = BUILDER.termvec(BUILDER.termvec(), (yystack_[1].value.term)); }
#line 1069 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 76:
#line 462 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.termvec) = BUILDER.termvec((yystack_[2].value.termvec), (yystack_[1].value.term)); }
#line 1075 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 77:
#line 465 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.termvec) = BUILDER.termvec(BUILDER.termvec(), (yystack_[0].value.term)); }
#line 1081 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 78:
#line 466 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.termvec) = BUILDER.termvec((yystack_[1].value.termvec), (yystack_[0].value.term)); }
#line 1087 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 79:
#line 469 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.termvecvec) = BUILDER.termvecvec(BUILDER.termvecvec(), (yystack_[0].value.termvec)); }
#line 1093 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 80:
#line 470 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.termvecvec) = BUILDER.termvecvec((yystack_[2].value.termvecvec), (yystack_[0].value.termvec)); }
#line 1099 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 81:
#line 474 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.termvecvec) = BUILDER.termvecvec(BUILDER.termvecvec(), BUILDER.termvec(BUILDER.termvec(BUILDER.termvec(), (yystack_[2].value.term)), (yystack_[0].value.term))); }
#line 1105 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 82:
#line 475 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.termvecvec) = BUILDER.termvecvec((yystack_[4].value.termvecvec), BUILDER.termvec(BUILDER.termvec(BUILDER.termvec(), (yystack_[2].value.term)), (yystack_[0].value.term))); }
#line 1111 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 83:
#line 485 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.rel) = Relation::GT; }
#line 1117 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 84:
#line 486 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.rel) = Relation::LT; }
#line 1123 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 85:
#line 487 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.rel) = Relation::GEQ; }
#line 1129 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 86:
#line 488 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.rel) = Relation::LEQ; }
#line 1135 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 87:
#line 489 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.rel) = Relation::EQ; }
#line 1141 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 88:
#line 490 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.rel) = Relation::NEQ; }
#line 1147 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 89:
#line 494 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.predRep(yylhs.location, false, String::fromRep((yystack_[0].value.str)), BUILDER.termvecvec(BUILDER.termvecvec(), BUILDER.termvec())); }
#line 1153 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 90:
#line 495 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.predRep(yylhs.location, false, String::fromRep((yystack_[3].value.str)), (yystack_[1].value.termvecvec)); }
#line 1159 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 91:
#line 496 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.predRep(yylhs.location, true, String::fromRep((yystack_[0].value.str)), BUILDER.termvecvec(BUILDER.termvecvec(), BUILDER.termvec())); }
#line 1165 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 92:
#line 497 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.predRep(yylhs.location, true, String::fromRep((yystack_[3].value.str)), (yystack_[1].value.termvecvec)); }
#line 1171 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 93:
#line 501 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.lit) = BUILDER.boollit(yylhs.location, true); }
#line 1177 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 94:
#line 502 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.lit) = BUILDER.boollit(yylhs.location, false); }
#line 1183 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 95:
#line 503 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.lit) = BUILDER.boollit(yylhs.location, true); }
#line 1189 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 96:
#line 504 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.lit) = BUILDER.boollit(yylhs.location, false); }
#line 1195 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 97:
#line 505 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.lit) = BUILDER.boollit(yylhs.location, true); }
#line 1201 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 98:
#line 506 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.lit) = BUILDER.boollit(yylhs.location, false); }
#line 1207 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 99:
#line 507 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.lit) = BUILDER.predlit(yylhs.location, NAF::POS, (yystack_[0].value.term)); }
#line 1213 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 100:
#line 508 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.lit) = BUILDER.predlit(yylhs.location, NAF::NOT, (yystack_[0].value.term)); }
#line 1219 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 101:
#line 509 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.lit) = BUILDER.predlit(yylhs.location, NAF::NOTNOT, (yystack_[0].value.term)); }
#line 1225 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 102:
#line 510 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.lit) = BUILDER.rellit(yylhs.location, (yystack_[1].value.rel), (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 1231 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 103:
#line 511 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.lit) = BUILDER.rellit(yylhs.location, neg((yystack_[1].value.rel)), (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 1237 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 104:
#line 512 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.lit) = BUILDER.rellit(yylhs.location, (yystack_[1].value.rel), (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 1243 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 105:
#line 513 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.lit) = BUILDER.csplit((yystack_[0].value.csplit)); }
#line 1249 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 106:
#line 517 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.cspmulterm) = BUILDER.cspmulterm(yylhs.location, (yystack_[0].value.term),                     (yystack_[2].value.term)); }
#line 1255 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 107:
#line 518 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.cspmulterm) = BUILDER.cspmulterm(yylhs.location, (yystack_[3].value.term),                     (yystack_[0].value.term)); }
#line 1261 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 108:
#line 519 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.cspmulterm) = BUILDER.cspmulterm(yylhs.location, BUILDER.term(yylhs.location, Symbol::createNum(1)), (yystack_[0].value.term)); }
#line 1267 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 109:
#line 520 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.cspmulterm) = BUILDER.cspmulterm(yylhs.location, (yystack_[0].value.term)); }
#line 1273 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 110:
#line 524 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.cspaddterm) = BUILDER.cspaddterm(yylhs.location, (yystack_[2].value.cspaddterm), (yystack_[0].value.cspmulterm), true); }
#line 1279 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 111:
#line 525 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.cspaddterm) = BUILDER.cspaddterm(yylhs.location, (yystack_[2].value.cspaddterm), (yystack_[0].value.cspmulterm), false); }
#line 1285 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 112:
#line 526 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.cspaddterm) = BUILDER.cspaddterm(yylhs.location, (yystack_[0].value.cspmulterm)); }
#line 1291 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 113:
#line 530 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.rel) = Relation::GT; }
#line 1297 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 114:
#line 531 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.rel) = Relation::LT; }
#line 1303 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 115:
#line 532 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.rel) = Relation::GEQ; }
#line 1309 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 116:
#line 533 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.rel) = Relation::LEQ; }
#line 1315 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 117:
#line 534 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.rel) = Relation::EQ; }
#line 1321 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 118:
#line 535 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.rel) = Relation::NEQ; }
#line 1327 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 119:
#line 539 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.csplit) = BUILDER.csplit(yylhs.location, (yystack_[2].value.csplit), (yystack_[1].value.rel), (yystack_[0].value.cspaddterm)); }
#line 1333 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 120:
#line 540 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.csplit) = BUILDER.csplit(yylhs.location, (yystack_[2].value.cspaddterm),   (yystack_[1].value.rel), (yystack_[0].value.cspaddterm)); }
#line 1339 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 121:
#line 548 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.litvec) = BUILDER.litvec(BUILDER.litvec(), (yystack_[0].value.lit)); }
#line 1345 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 122:
#line 549 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.litvec) = BUILDER.litvec((yystack_[2].value.litvec), (yystack_[0].value.lit)); }
#line 1351 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 123:
#line 553 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.litvec) = (yystack_[0].value.litvec); }
#line 1357 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 124:
#line 554 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.litvec) = BUILDER.litvec(); }
#line 1363 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 125:
#line 558 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.litvec) = (yystack_[0].value.litvec); }
#line 1369 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 126:
#line 559 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.litvec) = BUILDER.litvec(); }
#line 1375 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 127:
#line 563 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.litvec) = (yystack_[0].value.litvec); }
#line 1381 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 128:
#line 564 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.litvec) = BUILDER.litvec(); }
#line 1387 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 129:
#line 568 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.fun) = AggregateFunction::SUM; }
#line 1393 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 130:
#line 569 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.fun) = AggregateFunction::SUMP; }
#line 1399 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 131:
#line 570 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.fun) = AggregateFunction::MIN; }
#line 1405 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 132:
#line 571 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.fun) = AggregateFunction::MAX; }
#line 1411 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 133:
#line 572 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.fun) = AggregateFunction::COUNT; }
#line 1417 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 134:
#line 578 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.bodyaggrelem) = { BUILDER.termvec(), (yystack_[0].value.litvec) }; }
#line 1423 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 135:
#line 579 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.bodyaggrelem) = { (yystack_[1].value.termvec), (yystack_[0].value.litvec) }; }
#line 1429 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 136:
#line 583 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.bodyaggrelemvec) = BUILDER.bodyaggrelemvec(BUILDER.bodyaggrelemvec(), (yystack_[0].value.bodyaggrelem).first, (yystack_[0].value.bodyaggrelem).second); }
#line 1435 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 137:
#line 584 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.bodyaggrelemvec) = BUILDER.bodyaggrelemvec((yystack_[2].value.bodyaggrelemvec), (yystack_[0].value.bodyaggrelem).first, (yystack_[0].value.bodyaggrelem).second); }
#line 1441 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 138:
#line 590 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.lbodyaggrelem) = { (yystack_[1].value.lit), (yystack_[0].value.litvec) }; }
#line 1447 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 139:
#line 594 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.condlitlist) = BUILDER.condlitvec(BUILDER.condlitvec(), (yystack_[0].value.lbodyaggrelem).first, (yystack_[0].value.lbodyaggrelem).second); }
#line 1453 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 140:
#line 595 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.condlitlist) = BUILDER.condlitvec((yystack_[2].value.condlitlist), (yystack_[0].value.lbodyaggrelem).first, (yystack_[0].value.lbodyaggrelem).second); }
#line 1459 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 141:
#line 601 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.aggr) = { AggregateFunction::COUNT, true, BUILDER.condlitvec() }; }
#line 1465 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 142:
#line 602 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.aggr) = { AggregateFunction::COUNT, true, (yystack_[1].value.condlitlist) }; }
#line 1471 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 143:
#line 603 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.aggr) = { (yystack_[2].value.fun), false, BUILDER.bodyaggrelemvec() }; }
#line 1477 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 144:
#line 604 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.aggr) = { (yystack_[3].value.fun), false, (yystack_[1].value.bodyaggrelemvec) }; }
#line 1483 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 145:
#line 608 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.bound) = { Relation::LEQ, (yystack_[0].value.term) }; }
#line 1489 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 146:
#line 609 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.bound) = { (yystack_[1].value.rel), (yystack_[0].value.term) }; }
#line 1495 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 147:
#line 610 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.bound) = { Relation::LEQ, TermUid(-1) }; }
#line 1501 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 148:
#line 614 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.uid) = lexer->aggregate((yystack_[1].value.aggr).fun, (yystack_[1].value.aggr).choice, (yystack_[1].value.aggr).elems, lexer->boundvec(Relation::LEQ, (yystack_[2].value.term), (yystack_[0].value.bound).rel, (yystack_[0].value.bound).term)); }
#line 1507 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 149:
#line 615 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.uid) = lexer->aggregate((yystack_[1].value.aggr).fun, (yystack_[1].value.aggr).choice, (yystack_[1].value.aggr).elems, lexer->boundvec((yystack_[2].value.rel), (yystack_[3].value.term), (yystack_[0].value.bound).rel, (yystack_[0].value.bound).term)); }
#line 1513 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 150:
#line 616 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.uid) = lexer->aggregate((yystack_[1].value.aggr).fun, (yystack_[1].value.aggr).choice, (yystack_[1].value.aggr).elems, lexer->boundvec(Relation::LEQ, TermUid(-1), (yystack_[0].value.bound).rel, (yystack_[0].value.bound).term)); }
#line 1519 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 151:
#line 617 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.uid) = lexer->aggregate((yystack_[0].value.theoryAtom)); }
#line 1525 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 152:
#line 623 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.headaggrelemvec) = BUILDER.headaggrelemvec((yystack_[5].value.headaggrelemvec), (yystack_[3].value.termvec), (yystack_[1].value.lit), (yystack_[0].value.litvec)); }
#line 1531 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 153:
#line 624 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.headaggrelemvec) = BUILDER.headaggrelemvec(BUILDER.headaggrelemvec(), (yystack_[3].value.termvec), (yystack_[1].value.lit), (yystack_[0].value.litvec)); }
#line 1537 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 154:
#line 628 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.condlitlist) = BUILDER.condlitvec(BUILDER.condlitvec(), (yystack_[1].value.lit), (yystack_[0].value.litvec)); }
#line 1543 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 155:
#line 629 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.condlitlist) = BUILDER.condlitvec((yystack_[3].value.condlitlist), (yystack_[1].value.lit), (yystack_[0].value.litvec)); }
#line 1549 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 156:
#line 635 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.aggr) = { (yystack_[2].value.fun), false, BUILDER.headaggrelemvec() }; }
#line 1555 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 157:
#line 636 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.aggr) = { (yystack_[3].value.fun), false, (yystack_[1].value.headaggrelemvec) }; }
#line 1561 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 158:
#line 637 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.aggr) = { AggregateFunction::COUNT, true, BUILDER.condlitvec()}; }
#line 1567 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 159:
#line 638 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.aggr) = { AggregateFunction::COUNT, true, (yystack_[1].value.condlitlist)}; }
#line 1573 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 160:
#line 642 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.uid) = lexer->aggregate((yystack_[1].value.aggr).fun, (yystack_[1].value.aggr).choice, (yystack_[1].value.aggr).elems, lexer->boundvec(Relation::LEQ, (yystack_[2].value.term), (yystack_[0].value.bound).rel, (yystack_[0].value.bound).term)); }
#line 1579 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 161:
#line 643 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.uid) = lexer->aggregate((yystack_[1].value.aggr).fun, (yystack_[1].value.aggr).choice, (yystack_[1].value.aggr).elems, lexer->boundvec((yystack_[2].value.rel), (yystack_[3].value.term), (yystack_[0].value.bound).rel, (yystack_[0].value.bound).term)); }
#line 1585 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 162:
#line 644 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.uid) = lexer->aggregate((yystack_[1].value.aggr).fun, (yystack_[1].value.aggr).choice, (yystack_[1].value.aggr).elems, lexer->boundvec(Relation::LEQ, TermUid(-1), (yystack_[0].value.bound).rel, (yystack_[0].value.bound).term)); }
#line 1591 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 163:
#line 645 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.uid) = lexer->aggregate((yystack_[0].value.theoryAtom)); }
#line 1597 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 164:
#line 651 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.cspelemvec) = BUILDER.cspelemvec(BUILDER.cspelemvec(), yylhs.location, (yystack_[3].value.termvec), (yystack_[1].value.cspaddterm), (yystack_[0].value.litvec)); }
#line 1603 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 165:
#line 652 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.cspelemvec) = BUILDER.cspelemvec((yystack_[5].value.cspelemvec), yylhs.location, (yystack_[3].value.termvec), (yystack_[1].value.cspaddterm), (yystack_[0].value.litvec)); }
#line 1609 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 166:
#line 656 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.cspelemvec) = (yystack_[0].value.cspelemvec); }
#line 1615 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 167:
#line 657 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.cspelemvec) = BUILDER.cspelemvec(); }
#line 1621 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 168:
#line 661 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.disjoint) = { NAF::POS, (yystack_[1].value.cspelemvec) }; }
#line 1627 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 169:
#line 662 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.disjoint) = { NAF::NOT, (yystack_[1].value.cspelemvec) }; }
#line 1633 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 170:
#line 663 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.disjoint) = { NAF::NOTNOT, (yystack_[1].value.cspelemvec) }; }
#line 1639 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 171:
#line 670 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.lbodyaggrelem) = { (yystack_[2].value.lit), (yystack_[0].value.litvec) }; }
#line 1645 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 174:
#line 682 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.condlitlist) = BUILDER.condlitvec((yystack_[2].value.condlitlist), (yystack_[1].value.lit), BUILDER.litvec()); }
#line 1651 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 175:
#line 683 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.condlitlist) = BUILDER.condlitvec((yystack_[3].value.condlitlist), (yystack_[2].value.lit), (yystack_[1].value.litvec)); }
#line 1657 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 176:
#line 684 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.condlitlist) = BUILDER.condlitvec(); }
#line 1663 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 177:
#line 689 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.condlitlist) = BUILDER.condlitvec(BUILDER.condlitvec((yystack_[2].value.condlitlist), (yystack_[1].value.lit), (yystack_[0].value.litvec)), (yystack_[4].value.lit), BUILDER.litvec()); }
#line 1669 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 178:
#line 690 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.condlitlist) = BUILDER.condlitvec(BUILDER.condlitvec((yystack_[2].value.condlitlist), (yystack_[1].value.lit), (yystack_[0].value.litvec)), (yystack_[4].value.lit), BUILDER.litvec()); }
#line 1675 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 179:
#line 691 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.condlitlist) = BUILDER.condlitvec(BUILDER.condlitvec((yystack_[2].value.condlitlist), (yystack_[1].value.lit), (yystack_[0].value.litvec)), (yystack_[6].value.lit), (yystack_[4].value.litvec)); }
#line 1681 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 180:
#line 692 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.condlitlist) = BUILDER.condlitvec(BUILDER.condlitvec(), (yystack_[2].value.lit), (yystack_[0].value.litvec)); }
#line 1687 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 181:
#line 699 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.body) = BUILDER.bodylit((yystack_[2].value.body), (yystack_[1].value.lit)); }
#line 1693 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 182:
#line 700 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.body) = BUILDER.bodylit((yystack_[2].value.body), (yystack_[1].value.lit)); }
#line 1699 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 183:
#line 701 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.body) = lexer->bodyaggregate((yystack_[2].value.body), yystack_[1].location, NAF::POS, (yystack_[1].value.uid)); }
#line 1705 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 184:
#line 702 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.body) = lexer->bodyaggregate((yystack_[2].value.body), yystack_[1].location, NAF::POS, (yystack_[1].value.uid)); }
#line 1711 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 185:
#line 703 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.body) = lexer->bodyaggregate((yystack_[3].value.body), yystack_[1].location + yystack_[2].location, NAF::NOT, (yystack_[1].value.uid)); }
#line 1717 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 186:
#line 704 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.body) = lexer->bodyaggregate((yystack_[3].value.body), yystack_[1].location + yystack_[2].location, NAF::NOT, (yystack_[1].value.uid)); }
#line 1723 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 187:
#line 705 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.body) = lexer->bodyaggregate((yystack_[4].value.body), yystack_[1].location + yystack_[3].location, NAF::NOTNOT, (yystack_[1].value.uid)); }
#line 1729 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 188:
#line 706 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.body) = lexer->bodyaggregate((yystack_[4].value.body), yystack_[1].location + yystack_[3].location, NAF::NOTNOT, (yystack_[1].value.uid)); }
#line 1735 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 189:
#line 707 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.body) = BUILDER.conjunction((yystack_[2].value.body), yystack_[1].location, (yystack_[1].value.lbodyaggrelem).first, (yystack_[1].value.lbodyaggrelem).second); }
#line 1741 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 190:
#line 708 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.body) = BUILDER.disjoint((yystack_[2].value.body), yystack_[1].location, (yystack_[1].value.disjoint).first, (yystack_[1].value.disjoint).second); }
#line 1747 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 191:
#line 709 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.body) = BUILDER.body(); }
#line 1753 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 192:
#line 713 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.body) = BUILDER.bodylit((yystack_[2].value.body), (yystack_[1].value.lit)); }
#line 1759 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 193:
#line 714 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.body) = lexer->bodyaggregate((yystack_[2].value.body), yystack_[1].location, NAF::POS, (yystack_[1].value.uid)); }
#line 1765 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 194:
#line 715 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.body) = lexer->bodyaggregate((yystack_[3].value.body), yystack_[1].location + yystack_[2].location, NAF::NOT, (yystack_[1].value.uid)); }
#line 1771 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 195:
#line 716 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.body) = lexer->bodyaggregate((yystack_[4].value.body), yystack_[1].location + yystack_[3].location, NAF::NOTNOT, (yystack_[1].value.uid)); }
#line 1777 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 196:
#line 717 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.body) = BUILDER.conjunction((yystack_[2].value.body), yystack_[1].location, (yystack_[1].value.lbodyaggrelem).first, (yystack_[1].value.lbodyaggrelem).second); }
#line 1783 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 197:
#line 718 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.body) = BUILDER.disjoint((yystack_[2].value.body), yystack_[1].location, (yystack_[1].value.disjoint).first, (yystack_[1].value.disjoint).second); }
#line 1789 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 198:
#line 722 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.body) = BUILDER.body(); }
#line 1795 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 199:
#line 723 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.body) = BUILDER.body(); }
#line 1801 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 200:
#line 724 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.body) = (yystack_[0].value.body); }
#line 1807 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 201:
#line 727 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.head) = BUILDER.headlit((yystack_[0].value.lit)); }
#line 1813 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 202:
#line 728 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.head) = BUILDER.disjunction(yylhs.location, (yystack_[0].value.condlitlist)); }
#line 1819 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 203:
#line 729 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.head) = lexer->headaggregate(yylhs.location, (yystack_[0].value.uid)); }
#line 1825 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 204:
#line 733 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.rule(yylhs.location, (yystack_[1].value.head)); }
#line 1831 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 205:
#line 734 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.rule(yylhs.location, (yystack_[2].value.head)); }
#line 1837 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 206:
#line 735 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.rule(yylhs.location, (yystack_[2].value.head), (yystack_[0].value.body)); }
#line 1843 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 207:
#line 736 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.rule(yylhs.location, BUILDER.headlit(BUILDER.boollit(yylhs.location, false)), (yystack_[0].value.body)); }
#line 1849 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 208:
#line 737 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.rule(yylhs.location, BUILDER.headlit(BUILDER.boollit(yylhs.location, false)), BUILDER.body()); }
#line 1855 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 209:
#line 743 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.rule(yylhs.location, BUILDER.headlit(BUILDER.boollit(yystack_[2].location, false)), BUILDER.disjoint((yystack_[0].value.body), yystack_[2].location, inv((yystack_[2].value.disjoint).first), (yystack_[2].value.disjoint).second)); }
#line 1861 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 210:
#line 744 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.rule(yylhs.location, BUILDER.headlit(BUILDER.boollit(yystack_[2].location, false)), BUILDER.disjoint(BUILDER.body(), yystack_[2].location, inv((yystack_[2].value.disjoint).first), (yystack_[2].value.disjoint).second)); }
#line 1867 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 211:
#line 745 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.rule(yylhs.location, BUILDER.headlit(BUILDER.boollit(yystack_[1].location, false)), BUILDER.disjoint(BUILDER.body(), yystack_[1].location, inv((yystack_[1].value.disjoint).first), (yystack_[1].value.disjoint).second)); }
#line 1873 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 212:
#line 751 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.termvec) = (yystack_[0].value.termvec); }
#line 1879 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 213:
#line 752 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.termvec) = BUILDER.termvec(); }
#line 1885 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 214:
#line 756 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.termpair) = {(yystack_[2].value.term), (yystack_[0].value.term)}; }
#line 1891 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 215:
#line 757 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.termpair) = {(yystack_[0].value.term), BUILDER.term(yylhs.location, Symbol::createNum(0))}; }
#line 1897 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 216:
#line 761 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.body) = BUILDER.bodylit(BUILDER.body(), (yystack_[0].value.lit)); }
#line 1903 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 217:
#line 762 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.body) = BUILDER.bodylit((yystack_[2].value.body), (yystack_[0].value.lit)); }
#line 1909 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 218:
#line 766 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.body) = (yystack_[0].value.body); }
#line 1915 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 219:
#line 767 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.body) = BUILDER.body(); }
#line 1921 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 220:
#line 768 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.body) = BUILDER.body(); }
#line 1927 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 221:
#line 772 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.optimize(yylhs.location, (yystack_[2].value.termpair).first, (yystack_[2].value.termpair).second, (yystack_[1].value.termvec), (yystack_[4].value.body)); }
#line 1933 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 222:
#line 773 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.optimize(yylhs.location, (yystack_[2].value.termpair).first, (yystack_[2].value.termpair).second, (yystack_[1].value.termvec), BUILDER.body()); }
#line 1939 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 223:
#line 777 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.optimize(yylhs.location, BUILDER.term(yystack_[2].location, UnOp::NEG, (yystack_[2].value.termpair).first), (yystack_[2].value.termpair).second, (yystack_[1].value.termvec), (yystack_[0].value.body)); }
#line 1945 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 224:
#line 778 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.optimize(yylhs.location, BUILDER.term(yystack_[2].location, UnOp::NEG, (yystack_[2].value.termpair).first), (yystack_[2].value.termpair).second, (yystack_[1].value.termvec), (yystack_[0].value.body)); }
#line 1951 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 225:
#line 782 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.optimize(yylhs.location, (yystack_[2].value.termpair).first, (yystack_[2].value.termpair).second, (yystack_[1].value.termvec), (yystack_[0].value.body)); }
#line 1957 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 226:
#line 783 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.optimize(yylhs.location, (yystack_[2].value.termpair).first, (yystack_[2].value.termpair).second, (yystack_[1].value.termvec), (yystack_[0].value.body)); }
#line 1963 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 231:
#line 796 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.showsig(yylhs.location, Sig(String::fromRep((yystack_[3].value.str)), (yystack_[1].value.num), false), false); }
#line 1969 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 232:
#line 797 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.showsig(yylhs.location, Sig(String::fromRep((yystack_[3].value.str)), (yystack_[1].value.num), true), false); }
#line 1975 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 233:
#line 798 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.showsig(yylhs.location, Sig("", 0, false), false); }
#line 1981 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 234:
#line 799 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.show(yylhs.location, (yystack_[2].value.term), (yystack_[0].value.body), false); }
#line 1987 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 235:
#line 800 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.show(yylhs.location, (yystack_[1].value.term), BUILDER.body(), false); }
#line 1993 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 236:
#line 801 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.showsig(yylhs.location, Sig(String::fromRep((yystack_[3].value.str)), (yystack_[1].value.num), false), true); }
#line 1999 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 237:
#line 802 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.show(yylhs.location, (yystack_[2].value.term), (yystack_[0].value.body), true); }
#line 2005 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 238:
#line 803 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.show(yylhs.location, (yystack_[1].value.term), BUILDER.body(), true); }
#line 2011 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 239:
#line 809 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.defined(yylhs.location, Sig(String::fromRep((yystack_[3].value.str)), (yystack_[1].value.num), false)); }
#line 2017 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 240:
#line 810 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.defined(yylhs.location, Sig(String::fromRep((yystack_[3].value.str)), (yystack_[1].value.num), true)); }
#line 2023 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 241:
#line 815 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.edge(yylhs.location, (yystack_[2].value.termvecvec), (yystack_[0].value.body)); }
#line 2029 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 242:
#line 821 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.heuristic(yylhs.location, (yystack_[8].value.term), (yystack_[7].value.body), (yystack_[5].value.term), (yystack_[3].value.term), (yystack_[1].value.term)); }
#line 2035 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 243:
#line 822 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.heuristic(yylhs.location, (yystack_[6].value.term), (yystack_[5].value.body), (yystack_[3].value.term), BUILDER.term(yylhs.location, Symbol::createNum(0)), (yystack_[1].value.term)); }
#line 2041 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 244:
#line 828 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.project(yylhs.location, Sig(String::fromRep((yystack_[3].value.str)), (yystack_[1].value.num), false)); }
#line 2047 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 245:
#line 829 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.project(yylhs.location, Sig(String::fromRep((yystack_[3].value.str)), (yystack_[1].value.num), true)); }
#line 2053 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 246:
#line 830 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.project(yylhs.location, (yystack_[1].value.term), (yystack_[0].value.body)); }
#line 2059 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 247:
#line 836 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    {  BUILDER.define(yylhs.location, String::fromRep((yystack_[2].value.str)), (yystack_[0].value.term), false, LOGGER); }
#line 2065 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 248:
#line 840 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.define(yylhs.location, String::fromRep((yystack_[3].value.str)), (yystack_[1].value.term), true, LOGGER); }
#line 2071 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 249:
#line 841 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.define(yylhs.location, String::fromRep((yystack_[6].value.str)), (yystack_[4].value.term), true, LOGGER); }
#line 2077 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 250:
#line 842 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.define(yylhs.location, String::fromRep((yystack_[6].value.str)), (yystack_[4].value.term), false, LOGGER); }
#line 2083 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 251:
#line 848 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.python(yylhs.location, String::fromRep((yystack_[1].value.str))); }
#line 2089 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 252:
#line 849 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.lua(yylhs.location, String::fromRep((yystack_[1].value.str))); }
#line 2095 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 253:
#line 855 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { lexer->include(String::fromRep((yystack_[1].value.str)), yylhs.location, false, LOGGER); }
#line 2101 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 254:
#line 856 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { lexer->include(String::fromRep((yystack_[2].value.str)), yylhs.location, true, LOGGER); }
#line 2107 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 255:
#line 862 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.idlist) = BUILDER.idvec((yystack_[2].value.idlist), yystack_[0].location, String::fromRep((yystack_[0].value.str))); }
#line 2113 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 256:
#line 863 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.idlist) = BUILDER.idvec(BUILDER.idvec(), yystack_[0].location, String::fromRep((yystack_[0].value.str))); }
#line 2119 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 257:
#line 867 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.idlist) = BUILDER.idvec(); }
#line 2125 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 258:
#line 868 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.idlist) = (yystack_[0].value.idlist); }
#line 2131 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 259:
#line 872 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.block(yylhs.location, String::fromRep((yystack_[4].value.str)), (yystack_[2].value.idlist)); }
#line 2137 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 260:
#line 873 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.block(yylhs.location, String::fromRep((yystack_[1].value.str)), BUILDER.idvec()); }
#line 2143 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 261:
#line 879 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.external(yylhs.location, (yystack_[2].value.term), (yystack_[0].value.body), BUILDER.term(yylhs.location, Symbol::createId("false"))); }
#line 2149 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 262:
#line 880 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.external(yylhs.location, (yystack_[2].value.term), BUILDER.body(), BUILDER.term(yylhs.location, Symbol::createId("false"))); }
#line 2155 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 263:
#line 881 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.external(yylhs.location, (yystack_[1].value.term), BUILDER.body(), BUILDER.term(yylhs.location, Symbol::createId("false"))); }
#line 2161 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 264:
#line 882 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.external(yylhs.location, (yystack_[5].value.term), (yystack_[3].value.body), (yystack_[1].value.term)); }
#line 2167 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 265:
#line 883 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.external(yylhs.location, (yystack_[5].value.term), BUILDER.body(), (yystack_[1].value.term)); }
#line 2173 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 266:
#line 884 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.external(yylhs.location, (yystack_[4].value.term), BUILDER.body(), (yystack_[1].value.term)); }
#line 2179 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 267:
#line 892 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryOps) = BUILDER.theoryops((yystack_[1].value.theoryOps), String::fromRep((yystack_[0].value.str))); }
#line 2185 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 268:
#line 893 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryOps) = BUILDER.theoryops(BUILDER.theoryops(), String::fromRep((yystack_[0].value.str))); }
#line 2191 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 269:
#line 897 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryTerm) = BUILDER.theorytermset(yylhs.location, (yystack_[1].value.theoryOpterms)); }
#line 2197 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 270:
#line 898 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryTerm) = BUILDER.theoryoptermlist(yylhs.location, (yystack_[1].value.theoryOpterms)); }
#line 2203 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 271:
#line 899 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryTerm) = BUILDER.theorytermtuple(yylhs.location, BUILDER.theoryopterms()); }
#line 2209 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 272:
#line 900 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryTerm) = BUILDER.theorytermopterm(yylhs.location, (yystack_[1].value.theoryOpterm)); }
#line 2215 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 273:
#line 901 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryTerm) = BUILDER.theorytermtuple(yylhs.location, BUILDER.theoryopterms(BUILDER.theoryopterms(), yystack_[2].location, (yystack_[2].value.theoryOpterm))); }
#line 2221 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 274:
#line 902 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryTerm) = BUILDER.theorytermtuple(yylhs.location, BUILDER.theoryopterms(yystack_[3].location, (yystack_[3].value.theoryOpterm), (yystack_[1].value.theoryOpterms))); }
#line 2227 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 275:
#line 903 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryTerm) = BUILDER.theorytermfun(yylhs.location, String::fromRep((yystack_[3].value.str)), (yystack_[1].value.theoryOpterms)); }
#line 2233 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 276:
#line 904 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryTerm) = BUILDER.theorytermvalue(yylhs.location, Symbol::createId(String::fromRep((yystack_[0].value.str)))); }
#line 2239 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 277:
#line 905 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryTerm) = BUILDER.theorytermvalue(yylhs.location, Symbol::createNum((yystack_[0].value.num))); }
#line 2245 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 278:
#line 906 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryTerm) = BUILDER.theorytermvalue(yylhs.location, Symbol::createStr(String::fromRep((yystack_[0].value.str)))); }
#line 2251 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 279:
#line 907 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryTerm) = BUILDER.theorytermvalue(yylhs.location, Symbol::createInf()); }
#line 2257 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 280:
#line 908 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryTerm) = BUILDER.theorytermvalue(yylhs.location, Symbol::createSup()); }
#line 2263 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 281:
#line 909 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryTerm) = BUILDER.theorytermvar(yylhs.location, String::fromRep((yystack_[0].value.str))); }
#line 2269 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 282:
#line 913 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryOpterm) = BUILDER.theoryopterm((yystack_[2].value.theoryOpterm), (yystack_[1].value.theoryOps), (yystack_[0].value.theoryTerm)); }
#line 2275 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 283:
#line 914 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryOpterm) = BUILDER.theoryopterm((yystack_[1].value.theoryOps), (yystack_[0].value.theoryTerm)); }
#line 2281 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 284:
#line 915 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryOpterm) = BUILDER.theoryopterm(BUILDER.theoryops(), (yystack_[0].value.theoryTerm)); }
#line 2287 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 285:
#line 919 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryOpterms) = BUILDER.theoryopterms((yystack_[2].value.theoryOpterms), yystack_[0].location, (yystack_[0].value.theoryOpterm)); }
#line 2293 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 286:
#line 920 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryOpterms) = BUILDER.theoryopterms(BUILDER.theoryopterms(), yystack_[0].location, (yystack_[0].value.theoryOpterm)); }
#line 2299 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 287:
#line 924 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryOpterms) = (yystack_[0].value.theoryOpterms); }
#line 2305 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 288:
#line 925 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryOpterms) = BUILDER.theoryopterms(); }
#line 2311 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 289:
#line 929 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryElem) = { (yystack_[2].value.theoryOpterms), (yystack_[0].value.litvec) }; }
#line 2317 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 290:
#line 930 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryElem) = { BUILDER.theoryopterms(), (yystack_[0].value.litvec) }; }
#line 2323 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 291:
#line 934 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryElems) = BUILDER.theoryelems((yystack_[3].value.theoryElems), (yystack_[0].value.theoryElem).first, (yystack_[0].value.theoryElem).second); }
#line 2329 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 292:
#line 935 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryElems) = BUILDER.theoryelems(BUILDER.theoryelems(), (yystack_[0].value.theoryElem).first, (yystack_[0].value.theoryElem).second); }
#line 2335 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 293:
#line 939 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryElems) = (yystack_[0].value.theoryElems); }
#line 2341 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 294:
#line 940 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryElems) = BUILDER.theoryelems(); }
#line 2347 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 295:
#line 944 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, String::fromRep((yystack_[0].value.str)), BUILDER.termvecvec(BUILDER.termvecvec(), BUILDER.termvec()), false); }
#line 2353 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 296:
#line 945 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.term) = BUILDER.term(yylhs.location, String::fromRep((yystack_[3].value.str)), (yystack_[1].value.termvecvec), false); }
#line 2359 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 297:
#line 948 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryAtom) = BUILDER.theoryatom((yystack_[0].value.term), BUILDER.theoryelems()); }
#line 2365 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 298:
#line 949 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryAtom) = BUILDER.theoryatom((yystack_[6].value.term), (yystack_[3].value.theoryElems)); }
#line 2371 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 299:
#line 950 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryAtom) = BUILDER.theoryatom((yystack_[8].value.term), (yystack_[5].value.theoryElems), String::fromRep((yystack_[2].value.str)), yystack_[1].location, (yystack_[1].value.theoryOpterm)); }
#line 2377 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 300:
#line 956 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryOps) = BUILDER.theoryops(BUILDER.theoryops(), String::fromRep((yystack_[0].value.str))); }
#line 2383 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 301:
#line 957 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryOps) = BUILDER.theoryops((yystack_[2].value.theoryOps), String::fromRep((yystack_[0].value.str))); }
#line 2389 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 302:
#line 961 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryOps) = (yystack_[0].value.theoryOps); }
#line 2395 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 303:
#line 962 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryOps) = BUILDER.theoryops(); }
#line 2401 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 304:
#line 966 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryOpDef) = BUILDER.theoryopdef(yylhs.location, String::fromRep((yystack_[5].value.str)), (yystack_[2].value.num), TheoryOperatorType::Unary); }
#line 2407 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 305:
#line 967 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryOpDef) = BUILDER.theoryopdef(yylhs.location, String::fromRep((yystack_[7].value.str)), (yystack_[4].value.num), TheoryOperatorType::BinaryLeft); }
#line 2413 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 306:
#line 968 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryOpDef) = BUILDER.theoryopdef(yylhs.location, String::fromRep((yystack_[7].value.str)), (yystack_[4].value.num), TheoryOperatorType::BinaryRight); }
#line 2419 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 307:
#line 972 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryOpDefs) = BUILDER.theoryopdefs(BUILDER.theoryopdefs(), (yystack_[0].value.theoryOpDef)); }
#line 2425 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 308:
#line 973 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryOpDefs) = BUILDER.theoryopdefs((yystack_[3].value.theoryOpDefs), (yystack_[0].value.theoryOpDef)); }
#line 2431 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 309:
#line 977 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryOpDefs) = (yystack_[0].value.theoryOpDefs); }
#line 2437 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 310:
#line 978 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryOpDefs) = BUILDER.theoryopdefs(); }
#line 2443 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 311:
#line 982 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.str) = (yystack_[0].value.str); }
#line 2449 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 312:
#line 983 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.str) = String::toRep("left"); }
#line 2455 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 313:
#line 984 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.str) = String::toRep("right"); }
#line 2461 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 314:
#line 985 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.str) = String::toRep("unary"); }
#line 2467 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 315:
#line 986 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.str) = String::toRep("binary"); }
#line 2473 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 316:
#line 987 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.str) = String::toRep("head"); }
#line 2479 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 317:
#line 988 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.str) = String::toRep("body"); }
#line 2485 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 318:
#line 989 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.str) = String::toRep("any"); }
#line 2491 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 319:
#line 990 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.str) = String::toRep("directive"); }
#line 2497 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 320:
#line 994 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryTermDef) = BUILDER.theorytermdef(yylhs.location, String::fromRep((yystack_[5].value.str)), (yystack_[2].value.theoryOpDefs), LOGGER); }
#line 2503 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 321:
#line 998 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryAtomType) = TheoryAtomType::Head; }
#line 2509 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 322:
#line 999 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryAtomType) = TheoryAtomType::Body; }
#line 2515 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 323:
#line 1000 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryAtomType) = TheoryAtomType::Any; }
#line 2521 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 324:
#line 1001 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryAtomType) = TheoryAtomType::Directive; }
#line 2527 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 325:
#line 1006 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryAtomDef) = BUILDER.theoryatomdef(yylhs.location, String::fromRep((yystack_[14].value.str)), (yystack_[12].value.num), String::fromRep((yystack_[10].value.str)), (yystack_[0].value.theoryAtomType), (yystack_[6].value.theoryOps), String::fromRep((yystack_[2].value.str))); }
#line 2533 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 326:
#line 1007 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryAtomDef) = BUILDER.theoryatomdef(yylhs.location, String::fromRep((yystack_[6].value.str)), (yystack_[4].value.num), String::fromRep((yystack_[2].value.str)), (yystack_[0].value.theoryAtomType)); }
#line 2539 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 327:
#line 1011 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryDefs) = BUILDER.theorydefs((yystack_[2].value.theoryDefs), (yystack_[0].value.theoryAtomDef)); }
#line 2545 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 328:
#line 1012 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryDefs) = BUILDER.theorydefs((yystack_[2].value.theoryDefs), (yystack_[0].value.theoryTermDef)); }
#line 2551 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 329:
#line 1013 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryDefs) = BUILDER.theorydefs(BUILDER.theorydefs(), (yystack_[0].value.theoryAtomDef)); }
#line 2557 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 330:
#line 1014 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryDefs) = BUILDER.theorydefs(BUILDER.theorydefs(), (yystack_[0].value.theoryTermDef)); }
#line 2563 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 331:
#line 1018 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryDefs) = (yystack_[0].value.theoryDefs); }
#line 2569 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 332:
#line 1019 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { (yylhs.value.theoryDefs) = BUILDER.theorydefs(); }
#line 2575 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 333:
#line 1023 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { BUILDER.theorydef(yylhs.location, String::fromRep((yystack_[6].value.str)), (yystack_[3].value.theoryDefs), LOGGER); }
#line 2581 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 334:
#line 1029 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { lexer->theoryLexing(TheoryLexing::Theory); }
#line 2587 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 335:
#line 1033 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { lexer->theoryLexing(TheoryLexing::Definition); }
#line 2593 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;

  case 336:
#line 1037 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:919
    { lexer->theoryLexing(TheoryLexing::Disabled); }
#line 2599 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
    break;


#line 2603 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:919
            default:
              break;
            }
        }
#if YY_EXCEPTIONS
      catch (const syntax_error& yyexc)
        {
          YYCDEBUG << "Caught exception: " << yyexc.what() << '\n';
          error (yyexc);
          YYERROR;
        }
#endif // YY_EXCEPTIONS
      YY_SYMBOL_PRINT ("-> $$ =", yylhs);
      yypop_ (yylen);
      yylen = 0;
      YY_STACK_PRINT ();

      // Shift the result of the reduction.
      yypush_ (YY_NULLPTR, YY_MOVE (yylhs));
    }
    goto yynewstate;


  /*--------------------------------------.
  | yyerrlab -- here on detecting error.  |
  `--------------------------------------*/
  yyerrlab:
    // If not already recovering from an error, report this error.
    if (!yyerrstatus_)
      {
        ++yynerrs_;
        error (yyla.location, yysyntax_error_ (yystack_[0].state, yyla));
      }


    yyerror_range[1].location = yyla.location;
    if (yyerrstatus_ == 3)
      {
        /* If just tried and failed to reuse lookahead token after an
           error, discard it.  */

        // Return failure if at end of input.
        if (yyla.type_get () == yyeof_)
          YYABORT;
        else if (!yyla.empty ())
          {
            yy_destroy_ ("Error: discarding", yyla);
            yyla.clear ();
          }
      }

    // Else will try to reuse lookahead token after shifting the error token.
    goto yyerrlab1;


  /*---------------------------------------------------.
  | yyerrorlab -- error raised explicitly by YYERROR.  |
  `---------------------------------------------------*/
  yyerrorlab:
    /* Pacify compilers when the user code never invokes YYERROR and
       the label yyerrorlab therefore never appears in user code.  */
    if (false)
      YYERROR;

    /* Do not reclaim the symbols of the rule whose action triggered
       this YYERROR.  */
    yypop_ (yylen);
    yylen = 0;
    goto yyerrlab1;


  /*-------------------------------------------------------------.
  | yyerrlab1 -- common code for both syntax error and YYERROR.  |
  `-------------------------------------------------------------*/
  yyerrlab1:
    yyerrstatus_ = 3;   // Each real token shifted decrements this.
    {
      stack_symbol_type error_token;
      for (;;)
        {
          yyn = yypact_[yystack_[0].state];
          if (!yy_pact_value_is_default_ (yyn))
            {
              yyn += yyterror_;
              if (0 <= yyn && yyn <= yylast_ && yycheck_[yyn] == yyterror_)
                {
                  yyn = yytable_[yyn];
                  if (0 < yyn)
                    break;
                }
            }

          // Pop the current state because it cannot handle the error token.
          if (yystack_.size () == 1)
            YYABORT;

          yyerror_range[1].location = yystack_[0].location;
          yy_destroy_ ("Error: popping", yystack_[0]);
          yypop_ ();
          YY_STACK_PRINT ();
        }

      yyerror_range[2].location = yyla.location;
      YYLLOC_DEFAULT (error_token.location, yyerror_range, 2);

      // Shift the error token.
      error_token.state = yyn;
      yypush_ ("Shifting", YY_MOVE (error_token));
    }
    goto yynewstate;


  /*-------------------------------------.
  | yyacceptlab -- YYACCEPT comes here.  |
  `-------------------------------------*/
  yyacceptlab:
    yyresult = 0;
    goto yyreturn;


  /*-----------------------------------.
  | yyabortlab -- YYABORT comes here.  |
  `-----------------------------------*/
  yyabortlab:
    yyresult = 1;
    goto yyreturn;


  /*-----------------------------------------------------.
  | yyreturn -- parsing is finished, return the result.  |
  `-----------------------------------------------------*/
  yyreturn:
    if (!yyla.empty ())
      yy_destroy_ ("Cleanup: discarding lookahead", yyla);

    /* Do not reclaim the symbols of the rule whose action triggered
       this YYABORT or YYACCEPT.  */
    yypop_ (yylen);
    while (1 < yystack_.size ())
      {
        yy_destroy_ ("Cleanup: popping", yystack_[0]);
        yypop_ ();
      }

    return yyresult;
  }
#if YY_EXCEPTIONS
    catch (...)
      {
        YYCDEBUG << "Exception caught: cleaning lookahead and stack\n";
        // Do not try to display the values of the reclaimed symbols,
        // as their printers might throw an exception.
        if (!yyla.empty ())
          yy_destroy_ (YY_NULLPTR, yyla);

        while (1 < yystack_.size ())
          {
            yy_destroy_ (YY_NULLPTR, yystack_[0]);
            yypop_ ();
          }
        throw;
      }
#endif // YY_EXCEPTIONS
  }

  void
  parser::error (const syntax_error& yyexc)
  {
    error (yyexc.location, yyexc.what ());
  }

  // Generate an error message.
  std::string
  parser::yysyntax_error_ (state_type yystate, const symbol_type& yyla) const
  {
    // Number of reported tokens (one for the "unexpected", one per
    // "expected").
    size_t yycount = 0;
    // Its maximum.
    enum { YYERROR_VERBOSE_ARGS_MAXIMUM = 5 };
    // Arguments of yyformat.
    char const *yyarg[YYERROR_VERBOSE_ARGS_MAXIMUM];

    /* There are many possibilities here to consider:
       - If this state is a consistent state with a default action, then
         the only way this function was invoked is if the default action
         is an error action.  In that case, don't check for expected
         tokens because there are none.
       - The only way there can be no lookahead present (in yyla) is
         if this state is a consistent state with a default action.
         Thus, detecting the absence of a lookahead is sufficient to
         determine that there is no unexpected or expected token to
         report.  In that case, just report a simple "syntax error".
       - Don't assume there isn't a lookahead just because this state is
         a consistent state with a default action.  There might have
         been a previous inconsistent state, consistent state with a
         non-default action, or user semantic action that manipulated
         yyla.  (However, yyla is currently not documented for users.)
       - Of course, the expected token list depends on states to have
         correct lookahead information, and it depends on the parser not
         to perform extra reductions after fetching a lookahead from the
         scanner and before detecting a syntax error.  Thus, state
         merging (from LALR or IELR) and default reductions corrupt the
         expected token list.  However, the list is correct for
         canonical LR with one exception: it will still contain any
         token that will not be accepted due to an error action in a
         later state.
    */
    if (!yyla.empty ())
      {
        int yytoken = yyla.type_get ();
        yyarg[yycount++] = yytname_[yytoken];
        int yyn = yypact_[yystate];
        if (!yy_pact_value_is_default_ (yyn))
          {
            /* Start YYX at -YYN if negative to avoid negative indexes in
               YYCHECK.  In other words, skip the first -YYN actions for
               this state because they are default actions.  */
            int yyxbegin = yyn < 0 ? -yyn : 0;
            // Stay within bounds of both yycheck and yytname.
            int yychecklim = yylast_ - yyn + 1;
            int yyxend = yychecklim < yyntokens_ ? yychecklim : yyntokens_;
            for (int yyx = yyxbegin; yyx < yyxend; ++yyx)
              if (yycheck_[yyx + yyn] == yyx && yyx != yyterror_
                  && !yy_table_value_is_error_ (yytable_[yyx + yyn]))
                {
                  if (yycount == YYERROR_VERBOSE_ARGS_MAXIMUM)
                    {
                      yycount = 1;
                      break;
                    }
                  else
                    yyarg[yycount++] = yytname_[yyx];
                }
          }
      }

    char const* yyformat = YY_NULLPTR;
    switch (yycount)
      {
#define YYCASE_(N, S)                         \
        case N:                               \
          yyformat = S;                       \
        break
      default: // Avoid compiler warnings.
        YYCASE_ (0, YY_("syntax error"));
        YYCASE_ (1, YY_("syntax error, unexpected %s"));
        YYCASE_ (2, YY_("syntax error, unexpected %s, expecting %s"));
        YYCASE_ (3, YY_("syntax error, unexpected %s, expecting %s or %s"));
        YYCASE_ (4, YY_("syntax error, unexpected %s, expecting %s or %s or %s"));
        YYCASE_ (5, YY_("syntax error, unexpected %s, expecting %s or %s or %s or %s"));
#undef YYCASE_
      }

    std::string yyres;
    // Argument number.
    size_t yyi = 0;
    for (char const* yyp = yyformat; *yyp; ++yyp)
      if (yyp[0] == '%' && yyp[1] == 's' && yyi < yycount)
        {
          yyres += yytnamerr_ (yyarg[yyi++]);
          ++yyp;
        }
      else
        yyres += *yyp;
    return yyres;
  }


  const short parser::yypact_ninf_ = -449;

  const short parser::yytable_ninf_ = -337;

  const short
  parser::yypact_[] =
  {
     156,  -449,   165,    68,  1067,  -449,  -449,  -449,    65,   -11,
    -449,  -449,   165,   165,   585,   165,  -449,   585,    71,  -449,
      67,   104,  -449,   143,     7,  -449,  1133,   518,  -449,   170,
    -449,   172,   481,   173,   106,    67,    58,   585,  -449,  -449,
    -449,  -449,   165,   585,   222,   165,  -449,  -449,  -449,   237,
     287,  -449,  -449,  1208,  -449,    66,  1719,  -449,    55,  -449,
    1031,   905,   218,   716,  -449,    48,  -449,   134,  -449,  1644,
    -449,    38,   278,   220,   283,   585,   285,  -449,   323,   915,
    1080,   165,   296,   185,   165,   284,  -449,   753,  -449,   165,
     329,  -449,  1281,  1868,   353,    88,  -449,  2122,   359,   311,
     518,   314,  1414,  1446,   585,  -449,   227,   585,   165,    44,
     194,   194,   165,   165,   313,   305,  -449,   196,  2122,     9,
     341,   346,  -449,  -449,  -449,   361,  -449,  -449,  1318,  1898,
    -449,   585,   585,   585,  -449,   376,   585,  -449,  -449,  -449,
    -449,   585,   585,  -449,   585,   585,   585,   585,   585,   909,
     716,  1245,  -449,  -449,  -449,  -449,  1478,  1478,  -449,  -449,
    -449,  -449,  -449,  -449,  1478,  1478,  1510,  2122,   585,  -449,
    -449,   368,  -449,   378,   165,  1644,  -449,  1577,  1644,  -449,
    1644,  -449,  -449,   374,  2138,  -449,  -449,   585,   401,   585,
     585,  1644,   585,   428,   437,  -449,   112,   411,   585,   427,
     423,   396,   379,  1171,   355,  1770,    79,   426,   716,    49,
     129,   179,  -449,   432,  -449,  1350,   585,  1245,  -449,  -449,
    1245,   585,  -449,   412,  -449,   444,  1974,   470,   216,   456,
     470,   217,  1916,  -449,  -449,  1969,   281,    89,   398,   459,
    -449,  -449,   449,   434,   436,   414,   585,  -449,   165,   585,
    -449,   585,   585,   460,  1080,   465,  -449,  -449,  1898,  -449,
     585,  -449,   299,   215,   502,   585,  2156,   454,   454,   454,
     659,   454,   215,   643,  2122,   716,  -449,  -449,    45,  1245,
    1245,  2005,  -449,  -449,   356,   356,  -449,   494,   229,  2122,
    -449,  -449,  -449,  -449,   469,  -449,   452,  -449,  2138,    84,
    -449,   829,  1644,  1644,  1644,  1644,  1644,  1644,  1644,  1644,
    1644,  1644,   322,  1650,   324,   328,  1789,  2122,   585,  1478,
    -449,   585,   585,   335,   472,   474,   585,   430,   492,  -449,
     353,  -449,   231,   844,  1821,    76,   976,   716,  1245,  -449,
    -449,  -449,  1382,  -449,  -449,  -449,  -449,  -449,  -449,  -449,
    -449,   496,   508,  -449,   353,  2122,  -449,  -449,   585,   585,
     522,   509,   585,  -449,   522,   513,   585,  -449,  -449,  -449,
     585,   194,   585,   455,   517,  -449,  -449,   585,   461,   467,
     527,   354,  -449,   544,   503,  2122,   470,   470,   397,   263,
    1080,   585,  2122,   332,   585,  2122,  -449,  1245,  -449,   399,
     399,  1245,  -449,   585,  1644,  -449,  1612,  -449,  -449,   549,
     511,   301,   850,   512,   512,   512,   690,   512,   301,   680,
    -449,  -449,  2195,  2195,  2155,  -449,  -449,  -449,  -449,  -449,
     530,  2217,  -449,   485,   562,  -449,   524,  -449,   575,  -449,
    -449,   550,   282,   579,   373,  -449,   585,   585,   408,   565,
    -449,  -449,  -449,  1245,  1821,   105,   976,  -449,  -449,  -449,
     716,  -449,  -449,  1245,  -449,   418,  -449,   276,  -449,  -449,
    2122,   428,  1245,  -449,  -449,   470,  -449,  -449,   470,  -449,
    2122,  -449,  2022,   567,  -449,  1945,   574,   576,  -449,   490,
     165,   580,   555,   558,   947,  -449,  -449,  -449,  -449,  -449,
    -449,  -449,  -449,  -449,  -449,  -449,  -449,  -449,   295,  -449,
     303,  2122,  -449,  -449,  1245,  1245,  -449,   102,   102,   353,
     601,   564,  -449,  2138,  1644,  -449,   562,   568,   563,  -449,
      40,  2195,  -449,  -449,  2217,  2195,   353,   566,   572,  1245,
     337,  -449,  1478,  -449,  1998,  2052,  -449,  -449,  -449,   976,
    -449,  -449,  -449,  -449,  -449,  -449,  -449,  1542,  -449,   607,
     522,   522,   585,  -449,   585,   585,  -449,  -449,  -449,  -449,
    -449,  -449,   569,   591,  -449,   397,  -449,   399,   508,  -449,
    -449,  1245,  -449,  -449,  -449,  2184,  -449,   578,  -449,   485,
    -449,  2195,   540,  -449,   581,   600,   282,  -449,  -449,  -449,
    1245,  -449,  -449,  2122,  2069,  2087,   552,   561,   629,  -449,
    -449,   102,   353,  -449,    87,  -449,  -449,  2195,  -449,  -449,
    -449,  -449,  -449,   585,  -449,   646,  -449,  -449,   524,  -449,
    -449,  -449,  -449,   485,  2104,   947,   647,   606,   610,  -449,
    -449,   654,   583,   561,  -449,   209,   655,  -449,  -449,  -449,
    -449,  -449,  -449,   632,   357,   582,  -449,   662,  -449,   666,
    -449,   358,   586,   630,  -449,  -449,  -449,   671,   947,   672,
     209,  -449
  };

  const unsigned short
  parser::yydefact_[] =
  {
       0,     5,     0,     0,     0,    10,    11,    12,     0,     0,
       1,   336,     0,     0,     0,     0,   133,     0,     0,     7,
       0,     0,    96,   191,     0,    61,     0,    74,   132,     0,
     131,     0,     0,     0,     0,     0,     0,     0,   129,   130,
      62,    93,     0,     0,   191,     0,     6,    59,    64,     0,
       0,    60,    63,     0,     4,    57,   109,    99,   201,   112,
       0,   105,     0,   147,   203,     0,   202,     0,   163,     0,
       3,     0,   295,   297,    58,     0,    57,    52,     0,   108,
     167,     0,    89,     0,     0,     0,   208,     0,   207,     0,
       0,   158,     0,   109,   126,     0,    73,    67,    72,    77,
      74,     0,     0,     0,     0,   233,     0,     0,     0,    89,
       0,     0,     0,     0,     0,    57,    51,     0,    65,     0,
       0,     0,   335,   251,   252,     0,    97,    94,     0,     0,
     100,    70,     0,     0,    87,     0,     0,    85,    83,    86,
      84,     0,     0,    88,     0,     0,     0,     0,     0,     0,
     147,     0,   176,   172,   173,   176,     0,     0,   116,   114,
     113,   115,   117,   118,     0,     0,    70,   145,     0,   162,
     211,   191,   204,   191,     0,     0,    35,     0,     0,    36,
       0,    33,    34,    31,   247,     8,     9,    70,     0,    70,
      70,     0,     0,    69,     0,   166,     0,    91,    70,   191,
     263,     0,     0,     0,     0,   109,     0,     0,   147,     0,
       0,     0,   151,     0,   253,     0,     0,   124,   154,   159,
       0,    71,    75,    78,    53,     0,   215,   213,     0,     0,
     213,     0,     0,   191,   235,     0,     0,    91,     0,   191,
     198,   246,     0,     0,     0,     0,    70,   260,   257,     0,
      56,     0,     0,     0,   167,     0,    98,    95,     0,   101,
       0,    79,     0,    45,    44,     0,    41,    49,    47,    50,
      43,    48,    46,    42,   102,   147,   160,   121,   180,     0,
       0,   109,   110,   111,   120,   119,   156,     0,     0,   146,
     210,   209,   205,   206,    32,    23,     0,    24,    37,     0,
      22,     0,    40,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,   294,     0,     0,     0,   106,     0,     0,
     168,    70,    70,     0,   262,   261,     0,     0,     0,   141,
     126,   139,     0,     0,     0,     0,     0,   147,   124,   181,
     192,   182,     0,   150,   183,   193,   184,   197,   190,   196,
     189,     0,   123,   125,   126,    68,    76,   228,     0,     0,
     220,     0,     0,   227,   220,     0,     0,   191,   238,   234,
       0,     0,     0,     0,     0,   199,   200,     0,     0,     0,
       0,     0,   256,   258,     0,    66,   213,   213,   332,     0,
     167,     0,   103,    54,    70,   107,   161,     0,   176,   128,
     128,     0,   157,    70,    40,    25,     0,    26,    30,    39,
       0,    16,    15,    20,    18,    21,    14,    19,    17,    13,
     296,   279,   288,   288,     0,   280,   277,   278,   281,   268,
     276,     0,   284,   286,   336,   292,   293,   334,     0,    55,
      54,   248,   126,     0,     0,    90,     0,     0,     0,     0,
     239,   138,   142,     0,     0,     0,     0,   185,   194,   186,
     147,   148,   171,   124,   143,   126,   136,     0,   254,   155,
     214,   212,   219,   223,   230,   213,   225,   229,   213,   237,
      81,   241,     0,     0,   244,     0,     0,     0,   231,    54,
       0,     0,     0,     0,     0,   318,   314,   315,   312,   313,
     316,   317,   319,   311,   334,   330,   329,   331,     0,   169,
       0,   104,    80,   122,     0,     0,   174,   177,   178,   126,
       0,     0,    27,    38,     0,    28,   287,     0,     0,   271,
       0,   288,   267,   283,     0,     0,   126,     0,     0,   124,
       0,   164,     0,    92,     0,     0,   266,   240,   140,     0,
     187,   195,   188,   149,   134,   135,   144,     0,   216,   218,
     220,   220,     0,   245,     0,     0,   236,   232,   255,   259,
     222,   221,     0,     0,   336,     0,   170,   128,   127,   175,
     153,     0,    29,   269,   270,     0,   272,     0,   282,   285,
     289,   336,   336,   290,     0,     0,   126,   265,   264,   137,
       0,   224,   226,    82,     0,     0,     0,   310,     0,   328,
     327,   179,   126,   273,     0,   275,   291,     0,   298,   249,
     250,   165,   217,     0,   243,     0,   335,   307,   309,   335,
     333,   152,   274,   336,     0,     0,     0,     0,     0,   299,
     242,     0,     0,     0,   320,   334,     0,   308,   323,   321,
     322,   324,   326,     0,     0,   303,   304,     0,   300,   302,
     335,     0,     0,     0,   305,   306,   301,     0,     0,     0,
       0,   325
  };

  const short
  parser::yypgoto_[] =
  {
    -449,  -449,  -449,  -449,    -2,   -66,   510,   291,   493,  -449,
     -22,   -71,   590,  -449,  -449,  -129,  -449,   -49,    -8,    13,
     286,  -150,   625,  -449,  -147,  -316,  -302,  -363,    37,   141,
    -449,   246,  -449,  -199,  -142,  -156,  -449,  -449,    -3,  -449,
    -449,  -216,   613,  -449,   -29,  -134,  -449,  -449,   -28,   -91,
    -449,  -207,   -67,  -449,  -334,  -449,  -449,  -449,  -449,  -449,
    -408,  -377,  -393,  -264,  -381,   115,  -449,  -449,  -449,   707,
    -449,  -449,    69,  -449,  -449,  -448,   140,    46,   142,  -449,
    -449,  -380,  -438,   -10
  };

  const short
  parser::yydefgoto_[] =
  {
      -1,     3,     4,    54,    76,   298,   409,   410,    97,   119,
     193,   261,    99,   100,   101,   262,   236,   168,    57,   277,
      59,    60,   164,    61,   352,   353,   218,   517,   207,   466,
     467,   331,   332,   208,   169,   209,   288,    95,    63,    64,
     195,   196,    65,   211,   579,   279,    66,    87,    88,   241,
      67,   360,   227,   559,   473,   228,   231,     9,   383,   384,
     431,   432,   433,   526,   527,   435,   436,   437,    73,   212,
     659,   660,   627,   628,   629,   504,   505,   652,   506,   507,
     508,   188,   253,   438
  };

  const short
  parser::yytable_[] =
  {
       8,    71,    55,   184,   278,    98,   337,   149,   276,   194,
      72,    74,    83,    78,   284,   285,   121,    58,    82,    85,
     242,   280,   462,   364,    55,   534,   110,   111,   451,   155,
     476,   530,   109,    82,   114,   115,   230,   518,   389,    94,
     117,    62,   528,   122,   216,   130,   572,    89,   335,   434,
     585,    55,   469,   150,   533,   397,   537,   538,   312,   344,
     314,   315,   249,   185,   151,   152,   343,   183,    10,   323,
      69,   112,    70,   170,   345,   -89,   -89,   250,    98,   197,
     260,   171,   201,   198,   130,    55,   457,   213,   338,   339,
      55,   -89,   586,    62,   406,   287,    90,   535,   153,   -89,
     206,   458,   346,   238,   340,   131,   237,    80,   153,   295,
     243,   244,   300,   154,   301,   550,   -89,   381,   113,   -89,
     259,   186,   534,   154,   573,   316,    55,    81,   322,   459,
     551,   429,   341,   396,   -89,   337,   407,   460,   219,   632,
     541,   220,   589,   291,     5,   293,   275,   554,   373,    55,
     587,     6,     7,     5,   347,   153,   336,   588,   552,   172,
       6,     7,   320,   555,    84,   321,   108,   173,    86,   442,
     154,   325,   294,   183,   510,   183,   183,   455,   183,   492,
     493,   534,   348,   194,   386,   387,    62,   641,   636,   183,
       5,   638,     5,   444,   199,   461,   130,     6,     7,     6,
       7,    55,    55,   239,   349,   369,   102,   259,   103,   391,
     200,   376,   107,    55,   611,    55,   330,   580,    55,   240,
     669,   247,   663,   593,   633,   534,   601,   602,     1,     2,
     132,   133,   350,   354,   590,   248,   233,   411,   412,   413,
     414,   415,   416,   417,   418,   419,   382,   120,   637,   398,
     443,     5,   234,   136,   166,   337,  -334,   460,     6,     7,
     141,   142,   123,   144,   514,   653,   361,   365,   560,   362,
     366,   561,   141,   142,   146,   144,   145,    55,    55,   402,
     481,   452,   403,   648,   453,   456,   146,   147,   649,   650,
     651,   217,   399,   400,   621,   475,   156,   157,   148,   478,
     183,   183,   183,   183,   183,   183,   183,   183,   183,   183,
     631,   430,   124,   509,   -91,   -91,   321,   187,   553,   194,
     465,   614,   189,   512,   190,   259,   556,   434,   191,   557,
     -91,    55,   520,   371,   372,   198,    55,   471,   -91,   479,
     523,   -90,   -90,   202,   246,   574,   305,   306,   575,   307,
     460,   393,   394,   576,   214,   -91,   321,   -90,   -91,    12,
     309,    13,   217,    14,   222,   -90,   224,    16,   578,   221,
     156,   157,   245,   -91,   420,   394,   439,   394,   251,   125,
     440,   394,   -90,   252,   126,   -90,   503,   445,   394,   265,
      25,   203,   596,   290,    27,    55,    28,   254,    30,    55,
     -90,   494,   183,   292,   183,   549,   489,   394,   515,   516,
     513,   132,   133,   302,   519,    37,    38,    39,    40,   127,
     430,   430,   430,    43,   536,   543,   394,   217,   318,   430,
     594,   595,   656,   657,   136,   664,   665,   313,   318,    47,
      48,     5,   282,   283,    51,    52,   319,   333,     6,     7,
     322,    55,   324,   141,   142,   327,   144,   145,   523,   546,
     326,    55,   342,   328,   351,   356,   330,   146,   147,   357,
      55,   495,   496,   497,   498,   499,   500,   501,   502,   148,
     359,   363,   374,     5,   375,   558,   377,    13,   568,    14,
       6,     7,   503,   378,   104,   379,   388,    56,   380,   -92,
     -92,   390,   144,   401,   405,   132,   105,    77,   404,   446,
      79,   447,    55,    55,   449,   -92,    25,   450,   397,    93,
      27,   468,   183,   -92,    13,   106,    14,   577,    96,   430,
     116,   472,   430,   430,   474,   465,   118,    55,   477,   483,
     -92,    75,   484,   -92,    40,   486,   129,   141,   142,    43,
     144,   487,   488,    25,   490,   491,   167,    27,   -92,   524,
     307,   146,   147,   525,   608,    47,    48,     5,   116,   531,
      51,    52,   535,   503,     6,     7,   429,  -334,    75,    55,
     205,    40,   618,   430,   539,   129,    43,   540,   542,   430,
     547,    13,   563,    14,   612,   226,   226,   232,    55,   566,
     235,   567,    47,    48,     5,   569,   570,    51,    52,   571,
     581,     6,     7,   622,   584,   430,   582,   600,   583,   591,
      25,   258,   592,   639,    27,   263,   264,   607,   606,   266,
     615,   617,   619,   503,   267,   268,   625,   269,   270,   271,
     272,   273,   274,   167,    93,    75,   132,   133,    40,   281,
     281,   620,   626,    43,   630,   635,   642,   281,   281,   643,
     644,   289,   132,   133,   645,   654,   503,   646,   655,    47,
      48,     5,   661,   658,    51,    52,   662,   666,     6,     7,
     667,   668,   670,   303,   304,   317,   165,   299,   141,   142,
     223,   144,   145,   303,   304,   521,    93,   334,   599,   548,
     210,   167,   146,   147,   141,   142,   616,   144,   258,   274,
      93,    68,   647,    93,   355,   609,   671,   610,   146,   147,
       0,   134,    13,     0,    14,   305,   306,     0,   307,   308,
       0,     0,     0,     0,     0,   305,   306,     0,   307,   309,
     310,     0,   385,     0,   226,   226,     0,   137,   138,   309,
     310,    25,     0,   392,   139,    27,   140,    12,   395,    13,
       0,    14,     0,   143,     0,    16,    17,     0,   167,     0,
       0,     0,    93,    93,     0,     0,    75,    18,     0,    40,
       0,     0,    22,     0,    43,     0,     0,     0,    25,   203,
       0,     0,    27,     0,    28,     0,    30,     0,     0,     0,
      47,    48,     5,     0,     0,    51,    52,     0,     0,     6,
       7,   355,   281,    37,    38,    39,    40,    41,     0,   448,
       0,    43,     0,     0,     0,     0,   454,     0,     0,   274,
     167,    93,   303,   304,     0,     0,     0,    47,    48,     5,
       0,     0,    51,    52,     0,   204,     6,     7,    12,     0,
      13,   470,    14,   303,     0,   226,    16,     0,     0,   226,
       0,     0,     0,   480,     0,   482,     0,     0,   255,     0,
     485,     0,     0,   256,   305,   306,     0,   307,   308,    25,
     203,     0,     0,    27,   511,    28,     0,    30,   309,   310,
      93,     0,     0,     0,    93,   305,   306,   408,   307,     0,
     311,     0,     0,     0,    37,    38,    39,    40,   257,   309,
     310,     0,    43,     0,     0,    13,     0,    14,   132,   133,
       0,    16,   158,   159,   160,   161,   162,   163,    47,    48,
       5,   192,     0,    51,    52,     0,     0,     6,     7,   544,
     545,   136,     0,     0,    25,    26,    93,     0,    27,   392,
      28,     0,    30,   167,     0,     0,    93,     0,     0,     0,
     141,   142,     0,   144,   145,    93,     0,     0,     0,    75,
      38,    39,    40,     0,   146,   147,     0,    43,     0,     0,
       0,     0,    13,     0,    14,     0,   148,     0,    16,     0,
       0,     0,     0,    47,    48,     5,     0,     0,    51,    52,
       0,     0,     6,     7,     0,     0,     0,    93,    93,     0,
       0,    25,   203,     0,     0,    27,     0,    28,     0,    30,
       0,   495,   496,   497,   498,   499,   500,   501,   502,     0,
       0,     0,    93,     5,     0,   281,    75,    38,    39,    40,
       6,     7,   511,     0,    43,   156,   157,     0,   158,   159,
     160,   161,   162,   163,     0,   603,     0,   604,   605,     0,
      47,    48,     5,     0,     0,    51,    52,    -2,    11,     6,
       7,    12,     0,    13,    93,    14,     0,     0,    15,    16,
      17,     0,     0,     0,     0,     0,    13,     0,    14,   -70,
       0,    18,    19,    93,    20,    21,    22,     0,     0,     0,
      23,    24,    25,    26,     0,     0,    27,     0,    28,    29,
      30,    31,     0,     0,     0,    25,   634,     0,     0,    27,
       0,    32,    33,    34,    35,    36,     0,    37,    38,    39,
      40,    41,    42,     0,     0,    43,     0,    44,     0,    13,
      75,    14,     0,    40,     0,     0,    17,     0,    43,    45,
      46,    47,    48,     5,    49,    50,    51,    52,     0,    53,
       6,     7,    22,     0,    47,    48,     5,     0,    25,    51,
      52,     0,    27,     6,     7,     0,     0,    13,     0,    14,
       0,     0,     0,    91,    17,     0,     0,     0,     0,     0,
       0,     0,     0,    37,     0,     0,    40,    41,     0,     0,
      22,    43,     0,     0,     0,     0,    25,     0,     0,     0,
      27,     0,     0,     0,    13,     0,    14,    47,    48,     5,
       0,   329,    51,    52,     0,    92,     6,     7,     0,     0,
       0,    37,   125,     0,    40,    41,     0,   126,     0,    43,
       0,     0,     0,    25,     0,     0,     0,    27,     0,     0,
       0,    13,     0,    14,     0,    47,    48,     5,    17,     0,
      51,    52,     0,    92,     6,     7,     0,     0,    37,     0,
       0,    40,   127,     0,    22,     0,    43,     0,     0,     0,
      25,     0,     0,     0,    27,     0,     0,    13,     0,    14,
       0,     0,    47,    48,     5,     0,     0,    51,    52,     0,
     128,     6,     7,     0,     0,    37,     0,     0,    40,    41,
     126,     0,     0,    43,     0,     0,    25,     0,     0,     0,
      27,     0,     0,     0,    13,     0,    14,     0,     0,    47,
      48,     5,     0,     0,    51,    52,     0,    92,     6,     7,
       0,    37,   255,     0,    40,   127,     0,   256,     0,    43,
       0,     0,     0,    25,     0,     0,    13,    27,    14,     0,
       0,     0,     0,     0,     0,    47,    48,     5,     0,     0,
      51,    52,     0,   215,     6,     7,     0,     0,    37,   256,
       0,    40,   257,     0,     0,    25,    43,     0,    13,    27,
      14,   463,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,    47,    48,     5,     0,     0,    51,    52,     0,
      37,     6,     7,    40,   257,     0,     0,    25,    43,     0,
      13,    27,    14,     0,     0,     0,     0,     0,     0,     0,
       0,     0,   464,     0,    47,    48,     5,     0,     0,    51,
      52,     0,    75,     6,     7,    40,     0,     0,     0,    25,
      43,     0,    13,    27,    14,     0,     0,     0,     0,     0,
       0,     0,     0,     0,   225,     0,    47,    48,     5,     0,
       0,    51,    52,     0,    75,     6,     7,    40,     0,     0,
       0,    25,    43,     0,    13,    27,    14,     0,     0,     0,
       0,    17,     0,     0,     0,     0,   229,     0,    47,    48,
       5,     0,     0,    51,    52,     0,    75,     6,     7,    40,
       0,     0,     0,    25,    43,     0,    13,    27,    14,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
      47,    48,     5,     0,     0,    51,    52,     0,    75,     6,
       7,    40,     0,     0,     0,    25,    43,     0,    13,    27,
      14,   463,     0,     0,     0,     0,     0,     0,     0,     0,
     286,     0,    47,    48,     5,     0,     0,    51,    52,     0,
      75,     6,     7,    40,     0,     0,     0,    25,    43,     0,
       0,    27,     0,   174,     0,   175,     0,   296,     0,     0,
       0,     0,     0,     0,    47,    48,     5,     0,     0,    51,
      52,     0,    75,     6,     7,    40,     0,     0,     0,     0,
      43,     0,   176,     0,     0,     0,   177,     0,   174,     0,
     175,     0,     0,     0,     0,     0,    47,    48,     5,   297,
       0,    51,    52,     0,     0,     6,     7,   178,     0,     0,
     179,     0,     0,     0,     0,   180,     0,   176,     0,     0,
     174,   177,   175,     0,     0,     0,     0,     0,     0,  -336,
       0,   181,     0,     5,   522,     0,   182,     0,     0,     0,
       6,     7,   178,     0,     0,   179,     0,     0,     0,   176,
     180,     0,     0,   177,     0,   421,   422,   423,     0,   424,
       0,     0,     0,     0,     0,     0,   181,     0,     5,     0,
       0,   182,     0,     0,   178,     6,     7,   179,     0,     0,
       0,     0,   180,   425,     0,     0,     0,     0,     0,     0,
       0,     0,   132,   133,   134,     0,     0,     0,   181,     0,
       5,    16,     0,   182,   426,   135,     5,     6,     7,   427,
     428,   429,     0,     6,     7,   136,     0,     0,     0,     0,
     137,   138,     0,     0,     0,    26,     0,   139,     0,   140,
      28,     0,    30,     0,   141,   142,   143,   144,   145,     0,
       0,     0,     0,   132,   133,   134,     0,     0,   146,   147,
      38,    39,    16,     0,     0,     0,   135,     0,     0,     0,
     148,     0,   303,   304,     0,     0,   136,     0,     0,     0,
       0,   137,   138,     0,     0,     0,   203,     0,   139,     0,
     140,    28,     0,    30,   441,   141,   142,   143,   144,   145,
       0,     0,     0,     0,   132,   133,   134,     0,     0,   146,
     147,    38,    39,    16,   305,   306,     0,   307,   308,     0,
       0,   148,     0,     0,     0,     0,     0,   136,   309,   310,
       0,     0,   137,   138,     0,     0,     0,   203,     0,   139,
     311,   140,    28,     0,    30,     0,   141,   142,   143,   144,
     145,   132,   133,   134,     0,     0,     0,     0,     0,     0,
     146,   147,    38,    39,   135,     0,     0,     0,     0,     0,
       0,     0,   148,     0,   136,     0,     0,     0,     0,   137,
     138,   132,   133,   134,     0,     0,   139,     0,   140,     0,
       0,     0,     0,   141,   142,   143,   144,   145,     0,   132,
     133,     0,     0,     0,   136,   367,     0,   146,   147,   137,
     138,     0,     0,     0,     0,     0,   139,     0,   140,   148,
       0,   368,   136,   141,   142,   143,   144,   145,   132,   133,
       0,   564,     0,     0,     0,   565,     0,   146,   147,     0,
       0,   141,   142,     0,   144,   145,     0,     0,     0,   148,
       0,   136,   132,   133,     0,   146,   147,   132,   133,   370,
     358,     0,     0,     0,     0,     0,     0,   148,     0,     0,
     141,   142,     0,   144,   145,   136,     0,     0,     0,     0,
     136,   132,   133,     0,   146,   147,     0,     0,   132,   133,
       0,     0,     0,     0,   141,   142,   148,   144,   145,   141,
     142,   135,   144,   145,   136,   132,   133,     0,   146,   147,
       0,   136,   562,   146,   147,     0,     0,     0,     0,     0,
     148,     0,     0,   141,   142,   148,   144,   145,   136,   597,
     141,   142,     0,   144,   145,   132,   133,   146,   147,     0,
       0,     0,     0,     0,   146,   147,     0,   141,   142,   148,
     144,   145,   132,   133,     0,     0,   148,     0,   136,   623,
       0,   146,   147,     0,     0,     0,     0,     0,     0,     0,
     132,   133,     0,   148,     0,   136,     0,   141,   142,     0,
     144,   145,     0,   598,     0,     0,     0,   132,   133,     0,
       0,   146,   147,   136,   141,   142,     0,   144,   145,     0,
       0,     0,     0,   148,     0,   132,   133,     0,   146,   147,
     136,     0,   141,   142,     0,   144,   145,     0,   624,     0,
     148,   303,   304,     0,     0,     0,   146,   147,   136,   141,
     142,     0,   144,   145,     0,   640,     0,     0,   148,   132,
     133,     0,     0,   146,   147,     0,     0,   141,   142,     0,
     144,   145,     0,     0,     0,   148,     0,     0,     0,     0,
       0,   146,   147,   305,   306,     0,   307,   308,     0,     0,
     421,   422,   423,   148,   424,     0,     0,   309,   310,     0,
       0,   141,   142,     0,   144,   145,     0,   529,     0,   311,
       0,     0,     0,     0,     0,   146,   147,     0,   425,   421,
     422,   423,     0,   424,     0,     0,     0,   148,     0,     0,
     421,   422,   423,     0,   424,     0,   613,     0,     0,   426,
       0,     5,     0,     0,   427,   428,   429,   425,     6,     7,
       0,     0,   421,   422,   423,     0,   424,     0,   425,     0,
       0,     0,     0,     0,     0,     0,     0,     0,   426,     0,
       5,     0,     0,   427,   428,   429,     0,     6,     7,   426,
     425,     5,     0,     0,   427,   428,   429,     0,     6,     7,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,   426,     0,     5,     0,     0,   427,   428,   532,     0,
       6,     7
  };

  const short
  parser::yycheck_[] =
  {
       2,    11,     4,    69,   151,    27,   205,    56,   150,    80,
      12,    13,    20,    15,   164,   165,    44,     4,    20,    21,
     111,   155,   338,   230,    26,   433,    34,    35,   330,    58,
     364,   424,    34,    35,    36,    37,   103,   400,   254,    26,
      42,     4,   423,    45,    93,    53,   494,    40,   204,   313,
      10,    53,   354,    56,   431,    10,   436,   437,   187,    10,
     189,   190,    53,    25,     9,    10,   208,    69,     0,   198,
       5,    13,    83,    25,    25,     9,    10,    68,   100,    81,
     129,    33,    84,    39,    92,    87,    10,    89,     9,    10,
      92,    25,    52,    56,    10,   166,    89,    10,    53,    33,
      87,    25,    53,    59,    25,    39,   108,    36,    53,   175,
     112,   113,   178,    68,   180,    10,    50,   246,    60,    53,
     128,    83,   530,    68,   504,   191,   128,    60,    39,    53,
      25,    91,    53,   275,    68,   334,    52,   336,    50,    52,
     442,    53,   535,   171,    86,   173,   149,   463,    59,   151,
     531,    93,    94,    86,    25,    53,   205,   534,    53,    25,
      93,    94,    50,   465,    60,    53,    60,    33,    25,   319,
      68,   199,   174,   175,   390,   177,   178,   333,   180,   386,
     387,   589,    53,   254,   251,   252,   149,   635,   626,   191,
      86,   629,    86,   322,     9,   337,   204,    93,    94,    93,
      94,   203,   204,     9,    25,   233,    36,   215,    36,   258,
      25,   239,    39,   215,   577,   217,   203,   519,   220,    25,
     668,    25,   660,   539,   617,   633,   560,   561,    72,    73,
       3,     4,    53,   220,   536,    39,     9,   303,   304,   305,
     306,   307,   308,   309,   310,   311,   248,    25,   628,   278,
     321,    86,    25,    26,    36,   454,    36,   456,    93,    94,
      45,    46,    25,    48,   398,   645,    50,    50,   475,    53,
      53,   478,    45,    46,    59,    48,    49,   279,   280,    50,
     371,    50,    53,    74,    53,   334,    59,    60,    79,    80,
      81,     9,   279,   280,   596,   362,    14,    15,    71,   366,
     302,   303,   304,   305,   306,   307,   308,   309,   310,   311,
     612,   313,    25,    50,     9,    10,    53,    39,   460,   390,
     342,   585,    39,   394,    39,   333,    50,   591,     5,    53,
      25,   333,   403,    52,    53,    39,   338,   359,    33,   367,
     406,     9,    10,    59,    39,    50,    45,    46,    53,    48,
     549,    52,    53,    50,    25,    50,    53,    25,    53,     4,
      59,     6,     9,     8,    53,    33,    52,    12,   515,    10,
      14,    15,    59,    68,    52,    53,    52,    53,    37,    24,
      52,    53,    50,    37,    29,    53,   388,    52,    53,    13,
      35,    36,   542,    25,    39,   397,    41,    36,    43,   401,
      68,     4,   404,    25,   406,   454,    52,    53,     9,    10,
     397,     3,     4,    39,   401,    60,    61,    62,    63,    64,
     422,   423,   424,    68,   434,    52,    53,     9,    10,   431,
      93,    94,    75,    76,    26,    77,    78,    36,    10,    84,
      85,    86,   156,   157,    89,    90,     9,    92,    93,    94,
      39,   453,    25,    45,    46,    59,    48,    49,   524,    51,
      37,   463,    36,    84,    32,    53,   453,    59,    60,    25,
     472,    74,    75,    76,    77,    78,    79,    80,    81,    71,
      10,    25,    84,    86,    25,   472,    37,     6,   490,     8,
      93,    94,   494,    59,    13,    59,    36,     4,    84,     9,
      10,    36,    48,     9,    52,     3,    25,    14,    39,    37,
      17,    37,   514,   515,    84,    25,    35,    25,    10,    26,
      39,    25,   524,    33,     6,    32,     8,   514,    10,   531,
      37,     9,   534,   535,    25,   557,    43,   539,    25,    84,
      50,    60,    25,    53,    63,    84,    53,    45,    46,    68,
      48,    84,    25,    35,    10,    52,    63,    39,    68,    10,
      48,    59,    60,    52,   574,    84,    85,    86,    75,    39,
      89,    90,    10,   575,    93,    94,    91,    53,    60,   581,
      87,    63,   592,   585,     9,    92,    68,    37,     9,   591,
      25,     6,    25,     8,   581,   102,   103,   104,   600,    25,
     107,    25,    84,    85,    86,    25,    51,    89,    90,    51,
       9,    93,    94,   600,    51,   617,    52,    10,    50,    53,
      35,   128,    50,   633,    39,   132,   133,    36,    59,   136,
      52,    91,    51,   635,   141,   142,    84,   144,   145,   146,
     147,   148,   149,   150,   151,    60,     3,     4,    63,   156,
     157,    51,    91,    68,    25,     9,     9,   164,   165,    53,
      50,   168,     3,     4,    10,    10,   668,    84,    36,    84,
      85,    86,    10,    91,    89,    90,    10,    91,    93,    94,
      50,    10,    10,     3,     4,   192,    61,   177,    45,    46,
     100,    48,    49,     3,     4,   404,   203,   204,   557,   453,
      87,   208,    59,    60,    45,    46,   591,    48,   215,   216,
     217,     4,   643,   220,   221,   575,   670,   575,    59,    60,
      -1,     5,     6,    -1,     8,    45,    46,    -1,    48,    49,
      -1,    -1,    -1,    -1,    -1,    45,    46,    -1,    48,    59,
      60,    -1,   249,    -1,   251,   252,    -1,    31,    32,    59,
      60,    35,    -1,   260,    38,    39,    40,     4,   265,     6,
      -1,     8,    -1,    47,    -1,    12,    13,    -1,   275,    -1,
      -1,    -1,   279,   280,    -1,    -1,    60,    24,    -1,    63,
      -1,    -1,    29,    -1,    68,    -1,    -1,    -1,    35,    36,
      -1,    -1,    39,    -1,    41,    -1,    43,    -1,    -1,    -1,
      84,    85,    86,    -1,    -1,    89,    90,    -1,    -1,    93,
      94,   318,   319,    60,    61,    62,    63,    64,    -1,   326,
      -1,    68,    -1,    -1,    -1,    -1,   333,    -1,    -1,   336,
     337,   338,     3,     4,    -1,    -1,    -1,    84,    85,    86,
      -1,    -1,    89,    90,    -1,    92,    93,    94,     4,    -1,
       6,   358,     8,     3,    -1,   362,    12,    -1,    -1,   366,
      -1,    -1,    -1,   370,    -1,   372,    -1,    -1,    24,    -1,
     377,    -1,    -1,    29,    45,    46,    -1,    48,    49,    35,
      36,    -1,    -1,    39,   391,    41,    -1,    43,    59,    60,
     397,    -1,    -1,    -1,   401,    45,    46,    68,    48,    -1,
      71,    -1,    -1,    -1,    60,    61,    62,    63,    64,    59,
      60,    -1,    68,    -1,    -1,     6,    -1,     8,     3,     4,
      -1,    12,    17,    18,    19,    20,    21,    22,    84,    85,
      86,    16,    -1,    89,    90,    -1,    -1,    93,    94,   446,
     447,    26,    -1,    -1,    35,    36,   453,    -1,    39,   456,
      41,    -1,    43,   460,    -1,    -1,   463,    -1,    -1,    -1,
      45,    46,    -1,    48,    49,   472,    -1,    -1,    -1,    60,
      61,    62,    63,    -1,    59,    60,    -1,    68,    -1,    -1,
      -1,    -1,     6,    -1,     8,    -1,    71,    -1,    12,    -1,
      -1,    -1,    -1,    84,    85,    86,    -1,    -1,    89,    90,
      -1,    -1,    93,    94,    -1,    -1,    -1,   514,   515,    -1,
      -1,    35,    36,    -1,    -1,    39,    -1,    41,    -1,    43,
      -1,    74,    75,    76,    77,    78,    79,    80,    81,    -1,
      -1,    -1,   539,    86,    -1,   542,    60,    61,    62,    63,
      93,    94,   549,    -1,    68,    14,    15,    -1,    17,    18,
      19,    20,    21,    22,    -1,   562,    -1,   564,   565,    -1,
      84,    85,    86,    -1,    -1,    89,    90,     0,     1,    93,
      94,     4,    -1,     6,   581,     8,    -1,    -1,    11,    12,
      13,    -1,    -1,    -1,    -1,    -1,     6,    -1,     8,     9,
      -1,    24,    25,   600,    27,    28,    29,    -1,    -1,    -1,
      33,    34,    35,    36,    -1,    -1,    39,    -1,    41,    42,
      43,    44,    -1,    -1,    -1,    35,   623,    -1,    -1,    39,
      -1,    54,    55,    56,    57,    58,    -1,    60,    61,    62,
      63,    64,    65,    -1,    -1,    68,    -1,    70,    -1,     6,
      60,     8,    -1,    63,    -1,    -1,    13,    -1,    68,    82,
      83,    84,    85,    86,    87,    88,    89,    90,    -1,    92,
      93,    94,    29,    -1,    84,    85,    86,    -1,    35,    89,
      90,    -1,    39,    93,    94,    -1,    -1,     6,    -1,     8,
      -1,    -1,    -1,    50,    13,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    60,    -1,    -1,    63,    64,    -1,    -1,
      29,    68,    -1,    -1,    -1,    -1,    35,    -1,    -1,    -1,
      39,    -1,    -1,    -1,     6,    -1,     8,    84,    85,    86,
      -1,    50,    89,    90,    -1,    92,    93,    94,    -1,    -1,
      -1,    60,    24,    -1,    63,    64,    -1,    29,    -1,    68,
      -1,    -1,    -1,    35,    -1,    -1,    -1,    39,    -1,    -1,
      -1,     6,    -1,     8,    -1,    84,    85,    86,    13,    -1,
      89,    90,    -1,    92,    93,    94,    -1,    -1,    60,    -1,
      -1,    63,    64,    -1,    29,    -1,    68,    -1,    -1,    -1,
      35,    -1,    -1,    -1,    39,    -1,    -1,     6,    -1,     8,
      -1,    -1,    84,    85,    86,    -1,    -1,    89,    90,    -1,
      92,    93,    94,    -1,    -1,    60,    -1,    -1,    63,    64,
      29,    -1,    -1,    68,    -1,    -1,    35,    -1,    -1,    -1,
      39,    -1,    -1,    -1,     6,    -1,     8,    -1,    -1,    84,
      85,    86,    -1,    -1,    89,    90,    -1,    92,    93,    94,
      -1,    60,    24,    -1,    63,    64,    -1,    29,    -1,    68,
      -1,    -1,    -1,    35,    -1,    -1,     6,    39,     8,    -1,
      -1,    -1,    -1,    -1,    -1,    84,    85,    86,    -1,    -1,
      89,    90,    -1,    92,    93,    94,    -1,    -1,    60,    29,
      -1,    63,    64,    -1,    -1,    35,    68,    -1,     6,    39,
       8,     9,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    84,    85,    86,    -1,    -1,    89,    90,    -1,
      60,    93,    94,    63,    64,    -1,    -1,    35,    68,    -1,
       6,    39,     8,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    50,    -1,    84,    85,    86,    -1,    -1,    89,
      90,    -1,    60,    93,    94,    63,    -1,    -1,    -1,    35,
      68,    -1,     6,    39,     8,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    50,    -1,    84,    85,    86,    -1,
      -1,    89,    90,    -1,    60,    93,    94,    63,    -1,    -1,
      -1,    35,    68,    -1,     6,    39,     8,    -1,    -1,    -1,
      -1,    13,    -1,    -1,    -1,    -1,    50,    -1,    84,    85,
      86,    -1,    -1,    89,    90,    -1,    60,    93,    94,    63,
      -1,    -1,    -1,    35,    68,    -1,     6,    39,     8,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      84,    85,    86,    -1,    -1,    89,    90,    -1,    60,    93,
      94,    63,    -1,    -1,    -1,    35,    68,    -1,     6,    39,
       8,     9,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      50,    -1,    84,    85,    86,    -1,    -1,    89,    90,    -1,
      60,    93,    94,    63,    -1,    -1,    -1,    35,    68,    -1,
      -1,    39,    -1,     6,    -1,     8,    -1,    10,    -1,    -1,
      -1,    -1,    -1,    -1,    84,    85,    86,    -1,    -1,    89,
      90,    -1,    60,    93,    94,    63,    -1,    -1,    -1,    -1,
      68,    -1,    35,    -1,    -1,    -1,    39,    -1,     6,    -1,
       8,    -1,    -1,    -1,    -1,    -1,    84,    85,    86,    52,
      -1,    89,    90,    -1,    -1,    93,    94,    60,    -1,    -1,
      63,    -1,    -1,    -1,    -1,    68,    -1,    35,    -1,    -1,
       6,    39,     8,    -1,    -1,    -1,    -1,    -1,    -1,     9,
      -1,    84,    -1,    86,    52,    -1,    89,    -1,    -1,    -1,
      93,    94,    60,    -1,    -1,    63,    -1,    -1,    -1,    35,
      68,    -1,    -1,    39,    -1,    35,    36,    37,    -1,    39,
      -1,    -1,    -1,    -1,    -1,    -1,    84,    -1,    86,    -1,
      -1,    89,    -1,    -1,    60,    93,    94,    63,    -1,    -1,
      -1,    -1,    68,    63,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,     3,     4,     5,    -1,    -1,    -1,    84,    -1,
      86,    12,    -1,    89,    84,    16,    86,    93,    94,    89,
      90,    91,    -1,    93,    94,    26,    -1,    -1,    -1,    -1,
      31,    32,    -1,    -1,    -1,    36,    -1,    38,    -1,    40,
      41,    -1,    43,    -1,    45,    46,    47,    48,    49,    -1,
      -1,    -1,    -1,     3,     4,     5,    -1,    -1,    59,    60,
      61,    62,    12,    -1,    -1,    -1,    16,    -1,    -1,    -1,
      71,    -1,     3,     4,    -1,    -1,    26,    -1,    -1,    -1,
      -1,    31,    32,    -1,    -1,    -1,    36,    -1,    38,    -1,
      40,    41,    -1,    43,    25,    45,    46,    47,    48,    49,
      -1,    -1,    -1,    -1,     3,     4,     5,    -1,    -1,    59,
      60,    61,    62,    12,    45,    46,    -1,    48,    49,    -1,
      -1,    71,    -1,    -1,    -1,    -1,    -1,    26,    59,    60,
      -1,    -1,    31,    32,    -1,    -1,    -1,    36,    -1,    38,
      71,    40,    41,    -1,    43,    -1,    45,    46,    47,    48,
      49,     3,     4,     5,    -1,    -1,    -1,    -1,    -1,    -1,
      59,    60,    61,    62,    16,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    71,    -1,    26,    -1,    -1,    -1,    -1,    31,
      32,     3,     4,     5,    -1,    -1,    38,    -1,    40,    -1,
      -1,    -1,    -1,    45,    46,    47,    48,    49,    -1,     3,
       4,    -1,    -1,    -1,    26,     9,    -1,    59,    60,    31,
      32,    -1,    -1,    -1,    -1,    -1,    38,    -1,    40,    71,
      -1,    25,    26,    45,    46,    47,    48,    49,     3,     4,
      -1,     6,    -1,    -1,    -1,    10,    -1,    59,    60,    -1,
      -1,    45,    46,    -1,    48,    49,    -1,    -1,    -1,    71,
      -1,    26,     3,     4,    -1,    59,    60,     3,     4,    10,
       6,    -1,    -1,    -1,    -1,    -1,    -1,    71,    -1,    -1,
      45,    46,    -1,    48,    49,    26,    -1,    -1,    -1,    -1,
      26,     3,     4,    -1,    59,    60,    -1,    -1,     3,     4,
      -1,    -1,    -1,    -1,    45,    46,    71,    48,    49,    45,
      46,    16,    48,    49,    26,     3,     4,    -1,    59,    60,
      -1,    26,    10,    59,    60,    -1,    -1,    -1,    -1,    -1,
      71,    -1,    -1,    45,    46,    71,    48,    49,    26,    51,
      45,    46,    -1,    48,    49,     3,     4,    59,    60,    -1,
      -1,    -1,    -1,    -1,    59,    60,    -1,    45,    46,    71,
      48,    49,     3,     4,    -1,    -1,    71,    -1,    26,    10,
      -1,    59,    60,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
       3,     4,    -1,    71,    -1,    26,    -1,    45,    46,    -1,
      48,    49,    -1,    51,    -1,    -1,    -1,     3,     4,    -1,
      -1,    59,    60,    26,    45,    46,    -1,    48,    49,    -1,
      -1,    -1,    -1,    71,    -1,     3,     4,    -1,    59,    60,
      26,    -1,    45,    46,    -1,    48,    49,    -1,    51,    -1,
      71,     3,     4,    -1,    -1,    -1,    59,    60,    26,    45,
      46,    -1,    48,    49,    -1,    51,    -1,    -1,    71,     3,
       4,    -1,    -1,    59,    60,    -1,    -1,    45,    46,    -1,
      48,    49,    -1,    -1,    -1,    71,    -1,    -1,    -1,    -1,
      -1,    59,    60,    45,    46,    -1,    48,    49,    -1,    -1,
      35,    36,    37,    71,    39,    -1,    -1,    59,    60,    -1,
      -1,    45,    46,    -1,    48,    49,    -1,    52,    -1,    71,
      -1,    -1,    -1,    -1,    -1,    59,    60,    -1,    63,    35,
      36,    37,    -1,    39,    -1,    -1,    -1,    71,    -1,    -1,
      35,    36,    37,    -1,    39,    -1,    52,    -1,    -1,    84,
      -1,    86,    -1,    -1,    89,    90,    91,    63,    93,    94,
      -1,    -1,    35,    36,    37,    -1,    39,    -1,    63,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    84,    -1,
      86,    -1,    -1,    89,    90,    91,    -1,    93,    94,    84,
      63,    86,    -1,    -1,    89,    90,    91,    -1,    93,    94,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    84,    -1,    86,    -1,    -1,    89,    90,    91,    -1,
      93,    94
  };

  const unsigned char
  parser::yystos_[] =
  {
       0,    72,    73,    96,    97,    86,    93,    94,    99,   152,
       0,     1,     4,     6,     8,    11,    12,    13,    24,    25,
      27,    28,    29,    33,    34,    35,    36,    39,    41,    42,
      43,    44,    54,    55,    56,    57,    58,    60,    61,    62,
      63,    64,    65,    68,    70,    82,    83,    84,    85,    87,
      88,    89,    90,    92,    98,    99,   103,   113,   114,   115,
     116,   118,   123,   133,   134,   137,   141,   145,   164,     5,
      83,   178,    99,   163,    99,    60,    99,   103,    99,   103,
      36,    60,    99,   113,    60,    99,    25,   142,   143,    40,
      89,    50,    92,   103,   114,   132,    10,   103,   105,   107,
     108,   109,    36,    36,    13,    25,   103,    39,    60,    99,
     113,   113,    13,    60,    99,    99,   103,    99,   103,   104,
      25,   143,    99,    25,    25,    24,    29,    64,    92,   103,
     113,    39,     3,     4,     5,    16,    26,    31,    32,    38,
      40,    45,    46,    47,    48,    49,    59,    60,    71,   112,
     133,     9,    10,    53,    68,   139,    14,    15,    17,    18,
      19,    20,    21,    22,   117,   117,    36,   103,   112,   129,
      25,    33,    25,    33,     6,     8,    35,    39,    60,    63,
      68,    84,    89,    99,   100,    25,    83,    39,   176,    39,
      39,     5,    16,   105,   106,   135,   136,    99,    39,     9,
      25,    99,    59,    36,    92,   103,   114,   123,   128,   130,
     137,   138,   164,    99,    25,    92,   112,     9,   121,    50,
      53,    10,    53,   107,    52,    50,   103,   147,   150,    50,
     147,   151,   103,     9,    25,   103,   111,    99,    59,     9,
      25,   144,   144,    99,    99,    59,    39,    25,    39,    53,
      68,    37,    37,   177,    36,    24,    29,    64,   103,   113,
     112,   106,   110,   103,   103,    13,   103,   103,   103,   103,
     103,   103,   103,   103,   103,   133,   129,   114,   119,   140,
     140,   103,   115,   115,   116,   116,    50,   106,   131,   103,
      25,   143,    25,   143,    99,   100,    10,    52,   100,   101,
     100,   100,    39,     3,     4,    45,    46,    48,    49,    59,
      60,    71,   110,    36,   110,   110,   100,   103,    10,     9,
      50,    53,    39,   110,    25,   143,    37,    59,    84,    50,
     114,   126,   127,    92,   103,   130,   112,   128,     9,    10,
      25,    53,    36,   129,    10,    25,    53,    25,    53,    25,
      53,    32,   119,   120,   114,   103,    53,    25,     6,    10,
     146,    50,    53,    25,   146,    50,    53,     9,    25,   143,
      10,    52,    53,    59,    84,    25,   143,    37,    59,    59,
      84,   110,    99,   153,   154,   103,   147,   147,    36,   136,
      36,   112,   103,    52,    53,   103,   129,    10,   139,   114,
     114,     9,    50,    53,    39,    52,    10,    52,    68,   101,
     102,   100,   100,   100,   100,   100,   100,   100,   100,   100,
      52,    35,    36,    37,    39,    63,    84,    89,    90,    91,
      99,   155,   156,   157,   158,   160,   161,   162,   178,    52,
      52,    25,   116,   106,   110,    52,    37,    37,   103,    84,
      25,   121,    50,    53,   103,   130,   112,    10,    25,    53,
     128,   129,   120,     9,    50,   105,   124,   125,    25,   121,
     103,   105,     9,   149,    25,   147,   149,    25,   147,   143,
     103,   144,   103,    84,    25,   103,    84,    84,    25,    52,
      10,    52,   146,   146,     4,    74,    75,    76,    77,    78,
      79,    80,    81,    99,   170,   171,   173,   174,   175,    50,
     136,   103,   106,   114,   140,     9,    10,   122,   122,   114,
     106,   102,    52,   100,    10,    52,   158,   159,   159,    52,
     157,    39,    91,   156,   155,    10,   178,   176,   176,     9,
      37,   121,     9,    52,   103,   103,    51,    25,   126,   112,
      10,    25,    53,   129,   120,   121,    50,    53,   114,   148,
     146,   146,    10,    25,     6,    10,    25,    25,    99,    25,
      51,    51,   170,   176,    50,    53,    50,   114,   119,   139,
     121,     9,    52,    50,    51,    10,    52,   159,   156,   157,
     121,    53,    50,   120,    93,    94,   116,    51,    51,   124,
      10,   149,   149,   103,   103,   103,    59,    36,   178,   171,
     173,   122,   114,    52,   158,    52,   160,    91,   178,    51,
      51,   121,   114,    10,    51,    84,    91,   167,   168,   169,
      25,   121,    52,   157,   103,     9,   177,   176,   177,   178,
      51,   170,     9,    53,    50,    10,    84,   167,    74,    79,
      80,    81,   172,   176,    10,    36,    75,    76,    91,   165,
     166,    10,    10,   177,    77,    78,    91,    50,    10,   170,
      10,   172
  };

  const unsigned char
  parser::yyr1_[] =
  {
       0,    95,    96,    96,    97,    97,    98,    98,    98,    98,
      99,    99,    99,   100,   100,   100,   100,   100,   100,   100,
     100,   100,   100,   100,   100,   100,   100,   100,   100,   100,
     100,   100,   100,   100,   100,   100,   100,   101,   101,   102,
     102,   103,   103,   103,   103,   103,   103,   103,   103,   103,
     103,   103,   103,   103,   103,   103,   103,   103,   103,   103,
     103,   103,   103,   103,   103,   104,   104,   105,   105,   106,
     106,   107,   107,   107,   107,   108,   108,   109,   109,   110,
     110,   111,   111,   112,   112,   112,   112,   112,   112,   113,
     113,   113,   113,   114,   114,   114,   114,   114,   114,   114,
     114,   114,   114,   114,   114,   114,   115,   115,   115,   115,
     116,   116,   116,   117,   117,   117,   117,   117,   117,   118,
     118,   119,   119,   120,   120,   121,   121,   122,   122,   123,
     123,   123,   123,   123,   124,   124,   125,   125,   126,   127,
     127,   128,   128,   128,   128,   129,   129,   129,   130,   130,
     130,   130,   131,   131,   132,   132,   133,   133,   133,   133,
     134,   134,   134,   134,   135,   135,   136,   136,   137,   137,
     137,   138,   139,   139,   140,   140,   140,   141,   141,   141,
     141,   142,   142,   142,   142,   142,   142,   142,   142,   142,
     142,   142,   143,   143,   143,   143,   143,   143,   144,   144,
     144,   145,   145,   145,    98,    98,    98,    98,    98,    98,
      98,    98,   146,   146,   147,   147,   148,   148,   149,   149,
     149,    98,    98,   150,   150,   151,   151,    98,    98,    98,
      98,    98,    98,    98,    98,    98,    98,    98,    98,    98,
      98,    98,    98,    98,    98,    98,    98,   152,    98,    98,
      98,    98,    98,    98,    98,   153,   153,   154,   154,    98,
      98,    98,    98,    98,    98,    98,    98,   155,   155,   156,
     156,   156,   156,   156,   156,   156,   156,   156,   156,   156,
     156,   156,   157,   157,   157,   158,   158,   159,   159,   160,
     160,   161,   161,   162,   162,   163,   163,   164,   164,   164,
     165,   165,   166,   166,   167,   167,   167,   168,   168,   169,
     169,   170,   170,   170,   170,   170,   170,   170,   170,   170,
     171,   172,   172,   172,   172,   173,   173,   174,   174,   174,
     174,   175,   175,    98,   176,   177,   178
  };

  const unsigned char
  parser::yyr2_[] =
  {
       0,     2,     2,     3,     2,     0,     1,     1,     3,     3,
       1,     1,     1,     3,     3,     3,     3,     3,     3,     3,
       3,     3,     2,     2,     2,     3,     3,     4,     4,     5,
       3,     1,     2,     1,     1,     1,     1,     1,     3,     1,
       0,     3,     3,     3,     3,     3,     3,     3,     3,     3,
       3,     2,     2,     3,     4,     5,     3,     1,     2,     1,
       1,     1,     1,     1,     1,     1,     3,     1,     3,     1,
       0,     2,     1,     1,     0,     2,     3,     1,     2,     1,
       3,     3,     5,     1,     1,     1,     1,     1,     1,     1,
       4,     2,     5,     1,     2,     3,     1,     2,     3,     1,
       2,     3,     3,     4,     5,     1,     4,     4,     2,     1,
       3,     3,     1,     1,     1,     1,     1,     1,     1,     3,
       3,     1,     3,     1,     0,     2,     0,     2,     0,     1,
       1,     1,     1,     1,     2,     2,     1,     3,     2,     1,
       3,     2,     3,     3,     4,     1,     2,     0,     3,     4,
       2,     1,     6,     4,     2,     4,     3,     4,     2,     3,
       3,     4,     2,     1,     4,     6,     1,     0,     4,     5,
       6,     3,     1,     1,     3,     4,     0,     5,     5,     7,
       3,     3,     3,     3,     3,     4,     4,     5,     5,     3,
       3,     0,     3,     3,     4,     5,     3,     3,     1,     2,
       2,     1,     1,     1,     2,     3,     3,     2,     2,     3,
       3,     2,     2,     0,     3,     1,     1,     3,     2,     1,
       0,     6,     6,     3,     5,     3,     5,     4,     4,     5,
       5,     5,     6,     2,     4,     3,     6,     5,     4,     5,
       6,     5,    10,     8,     5,     6,     3,     3,     5,     8,
       8,     2,     2,     3,     5,     3,     1,     0,     1,     6,
       3,     4,     4,     3,     7,     7,     6,     2,     1,     3,
       3,     2,     3,     4,     5,     4,     1,     1,     1,     1,
       1,     1,     3,     2,     1,     3,     1,     1,     0,     3,
       3,     4,     1,     1,     0,     1,     4,     2,     8,    10,
       1,     3,     1,     0,     6,     8,     8,     1,     4,     1,
       0,     1,     1,     1,     1,     1,     1,     1,     1,     1,
       6,     1,     1,     1,     1,    16,     8,     3,     3,     1,
       1,     1,     0,     8,     0,     0,     0
  };



  // YYTNAME[SYMBOL-NUM] -- String name of the symbol SYMBOL-NUM.
  // First, the terminals, then, starting at \a yyntokens_, nonterminals.
  const char*
  const parser::yytname_[] =
  {
  "\"<EOF>\"", "error", "$undefined", "\"+\"", "\"&\"", "\"=\"", "\"@\"",
  "\"#base\"", "\"~\"", "\":\"", "\",\"", "\"#const\"", "\"#count\"",
  "\"$\"", "\"$+\"", "\"$-\"", "\"$*\"", "\"$<=\"", "\"$<\"", "\"$>\"",
  "\"$>=\"", "\"$=\"", "\"$!=\"", "\"#cumulative\"", "\"#disjoint\"",
  "\".\"", "\"..\"", "\"#external\"", "\"#defined\"", "\"#false\"",
  "\"#forget\"", "\">=\"", "\">\"", "\":-\"", "\"#include\"", "\"#inf\"",
  "\"{\"", "\"[\"", "\"<=\"", "\"(\"", "\"<\"", "\"#max\"",
  "\"#maximize\"", "\"#min\"", "\"#minimize\"", "\"\\\\\"", "\"*\"",
  "\"!=\"", "\"**\"", "\"?\"", "\"}\"", "\"]\"", "\")\"", "\";\"",
  "\"#show\"", "\"#edge\"", "\"#project\"", "\"#heuristic\"",
  "\"#showsig\"", "\"/\"", "\"-\"", "\"#sum\"", "\"#sum+\"", "\"#sup\"",
  "\"#true\"", "\"#program\"", "UBNOT", "UMINUS", "\"|\"", "\"#volatile\"",
  "\":~\"", "\"^\"", "\"<program>\"", "\"<define>\"", "\"any\"",
  "\"unary\"", "\"binary\"", "\"left\"", "\"right\"", "\"head\"",
  "\"body\"", "\"directive\"", "\"#theory\"", "\"EOF\"", "\"<NUMBER>\"",
  "\"<ANONYMOUS>\"", "\"<IDENTIFIER>\"", "\"<PYTHON>\"", "\"<LUA>\"",
  "\"<STRING>\"", "\"<VARIABLE>\"", "\"<THEORYOP>\"", "\"not\"",
  "\"default\"", "\"override\"", "$accept", "start", "program",
  "statement", "identifier", "constterm", "consttermvec", "constargvec",
  "term", "unaryargvec", "ntermvec", "termvec", "tuple", "tuplevec_sem",
  "tuplevec", "argvec", "binaryargvec", "cmp", "atom", "literal",
  "csp_mul_term", "csp_add_term", "csp_rel", "csp_literal", "nlitvec",
  "litvec", "optcondition", "noptcondition", "aggregatefunction",
  "bodyaggrelem", "bodyaggrelemvec", "altbodyaggrelem",
  "altbodyaggrelemvec", "bodyaggregate", "upper", "lubodyaggregate",
  "headaggrelemvec", "altheadaggrelemvec", "headaggregate",
  "luheadaggregate", "ncspelemvec", "cspelemvec", "disjoint",
  "conjunction", "dsym", "disjunctionsep", "disjunction", "bodycomma",
  "bodydot", "bodyconddot", "head", "optimizetuple", "optimizeweight",
  "optimizelitvec", "optimizecond", "maxelemlist", "minelemlist", "define",
  "nidlist", "idlist", "theory_op_list", "theory_term", "theory_opterm",
  "theory_opterm_nlist", "theory_opterm_list", "theory_atom_element",
  "theory_atom_element_nlist", "theory_atom_element_list",
  "theory_atom_name", "theory_atom", "theory_operator_nlist",
  "theory_operator_list", "theory_operator_definition",
  "theory_operator_definition_nlist", "theory_operator_definiton_list",
  "theory_definition_identifier", "theory_term_definition",
  "theory_atom_type", "theory_atom_definition", "theory_definition_nlist",
  "theory_definition_list", "enable_theory_lexing",
  "enable_theory_definition_lexing", "disable_theory_lexing", YY_NULLPTR
  };

#if YYDEBUG
  const unsigned short
  parser::yyrline_[] =
  {
       0,   340,   340,   341,   345,   346,   352,   353,   354,   355,
     359,   360,   361,   368,   369,   370,   371,   372,   373,   374,
     375,   376,   377,   378,   379,   380,   381,   382,   383,   384,
     385,   386,   387,   388,   389,   390,   391,   397,   398,   402,
     403,   409,   410,   411,   412,   413,   414,   415,   416,   417,
     418,   419,   420,   421,   422,   423,   424,   425,   426,   427,
     428,   429,   430,   431,   432,   438,   439,   445,   446,   450,
     451,   455,   456,   457,   458,   461,   462,   465,   466,   469,
     470,   474,   475,   485,   486,   487,   488,   489,   490,   494,
     495,   496,   497,   501,   502,   503,   504,   505,   506,   507,
     508,   509,   510,   511,   512,   513,   517,   518,   519,   520,
     524,   525,   526,   530,   531,   532,   533,   534,   535,   539,
     540,   548,   549,   553,   554,   558,   559,   563,   564,   568,
     569,   570,   571,   572,   578,   579,   583,   584,   590,   594,
     595,   601,   602,   603,   604,   608,   609,   610,   614,   615,
     616,   617,   623,   624,   628,   629,   635,   636,   637,   638,
     642,   643,   644,   645,   651,   652,   656,   657,   661,   662,
     663,   670,   677,   678,   682,   683,   684,   689,   690,   691,
     692,   699,   700,   701,   702,   703,   704,   705,   706,   707,
     708,   709,   713,   714,   715,   716,   717,   718,   722,   723,
     724,   727,   728,   729,   733,   734,   735,   736,   737,   743,
     744,   745,   751,   752,   756,   757,   761,   762,   766,   767,
     768,   772,   773,   777,   778,   782,   783,   787,   788,   789,
     790,   796,   797,   798,   799,   800,   801,   802,   803,   809,
     810,   815,   821,   822,   828,   829,   830,   836,   840,   841,
     842,   848,   849,   855,   856,   862,   863,   867,   868,   872,
     873,   879,   880,   881,   882,   883,   884,   892,   893,   897,
     898,   899,   900,   901,   902,   903,   904,   905,   906,   907,
     908,   909,   913,   914,   915,   919,   920,   924,   925,   929,
     930,   934,   935,   939,   940,   944,   945,   948,   949,   950,
     956,   957,   961,   962,   966,   967,   968,   972,   973,   977,
     978,   982,   983,   984,   985,   986,   987,   988,   989,   990,
     994,   998,   999,  1000,  1001,  1005,  1007,  1011,  1012,  1013,
    1014,  1018,  1019,  1023,  1029,  1033,  1037
  };

  // Print the state stack on the debug stream.
  void
  parser::yystack_print_ ()
  {
    *yycdebug_ << "Stack now";
    for (stack_type::const_iterator
           i = yystack_.begin (),
           i_end = yystack_.end ();
         i != i_end; ++i)
      *yycdebug_ << ' ' << i->state;
    *yycdebug_ << '\n';
  }

  // Report on the debug stream that the rule \a yyrule is going to be reduced.
  void
  parser::yy_reduce_print_ (int yyrule)
  {
    unsigned yylno = yyrline_[yyrule];
    int yynrhs = yyr2_[yyrule];
    // Print the symbols being reduced, and their result.
    *yycdebug_ << "Reducing stack by rule " << yyrule - 1
               << " (line " << yylno << "):\n";
    // The symbols being reduced.
    for (int yyi = 0; yyi < yynrhs; yyi++)
      YY_SYMBOL_PRINT ("   $" << yyi + 1 << " =",
                       yystack_[(yynrhs) - (yyi + 1)]);
  }
#endif // YYDEBUG

  parser::token_number_type
  parser::yytranslate_ (int t)
  {
    // YYTRANSLATE[TOKEN-NUM] -- Symbol number corresponding to
    // TOKEN-NUM as returned by yylex.
    static
    const token_number_type
    translate_table[] =
    {
       0,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     1,     2,     3,     4,
       5,     6,     7,     8,     9,    10,    11,    12,    13,    14,
      15,    16,    17,    18,    19,    20,    21,    22,    23,    24,
      25,    26,    27,    28,    29,    30,    31,    32,    33,    34,
      35,    36,    37,    38,    39,    40,    41,    42,    43,    44,
      45,    46,    47,    48,    49,    50,    51,    52,    53,    54,
      55,    56,    57,    58,    59,    60,    61,    62,    63,    64,
      65,    66,    67,    68,    69,    70,    71,    72,    73,    74,
      75,    76,    77,    78,    79,    80,    81,    82,    83,    84,
      85,    86,    87,    88,    89,    90,    91,    92,    93,    94
    };
    const unsigned user_token_number_max_ = 349;
    const token_number_type undef_token_ = 2;

    if (static_cast<int> (t) <= yyeof_)
      return yyeof_;
    else if (static_cast<unsigned> (t) <= user_token_number_max_)
      return translate_table[t];
    else
      return undef_token_;
  }

#line 28 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:1242
} } } // Gringo::Input::NonGroundGrammar
#line 3852 "/home/sthiele/Projects/clingo/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:1242
