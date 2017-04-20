// A Bison parser, made by GNU Bison 3.0.4.

// Skeleton implementation for Bison LALR(1) parsers in C++

// Copyright (C) 2002-2015 Free Software Foundation, Inc.

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

// Take the name prefix into account.
#define yylex   GringoNonGroundGrammar_lex

// First part of user declarations.
#line 54 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:404


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


#line 76 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:404

# ifndef YY_NULLPTR
#  if defined __cplusplus && 201103L <= __cplusplus
#   define YY_NULLPTR nullptr
#  else
#   define YY_NULLPTR 0
#  endif
# endif

#include "grammar.hh"

// User implementation prologue.

#line 90 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:412
// Unqualified %code blocks.
#line 92 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:413


void NonGroundGrammar::parser::error(DefaultLocation const &l, std::string const &msg) {
    lexer->parseError(l, msg);
}


#line 100 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:413


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
    while (/*CONSTCOND*/ false)
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
      *yycdebug_ << std::endl;                  \
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
# define YY_SYMBOL_PRINT(Title, Symbol)  YYUSE(Symbol)
# define YY_REDUCE_PRINT(Rule)           static_cast<void>(0)
# define YY_STACK_PRINT()                static_cast<void>(0)

#endif // !YYDEBUG

#define yyerrok         (yyerrstatus_ = 0)
#define yyclearin       (yyla.clear ())

#define YYACCEPT        goto yyacceptlab
#define YYABORT         goto yyabortlab
#define YYERROR         goto yyerrorlab
#define YYRECOVERING()  (!!yyerrstatus_)

#line 24 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:479
namespace Gringo { namespace Input { namespace NonGroundGrammar {
#line 186 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:479

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
        std::string yyr = "";
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
              // Fall through.
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


  /*---------------.
  | Symbol types.  |
  `---------------*/

  inline
  parser::syntax_error::syntax_error (const location_type& l, const std::string& m)
    : std::runtime_error (m)
    , location (l)
  {}

  // basic_symbol.
  template <typename Base>
  inline
  parser::basic_symbol<Base>::basic_symbol ()
    : value ()
  {}

  template <typename Base>
  inline
  parser::basic_symbol<Base>::basic_symbol (const basic_symbol& other)
    : Base (other)
    , value ()
    , location (other.location)
  {
    value = other.value;
  }


  template <typename Base>
  inline
  parser::basic_symbol<Base>::basic_symbol (typename Base::kind_type t, const semantic_type& v, const location_type& l)
    : Base (t)
    , value (v)
    , location (l)
  {}


  /// Constructor for valueless symbols.
  template <typename Base>
  inline
  parser::basic_symbol<Base>::basic_symbol (typename Base::kind_type t, const location_type& l)
    : Base (t)
    , value ()
    , location (l)
  {}

  template <typename Base>
  inline
  parser::basic_symbol<Base>::~basic_symbol ()
  {
    clear ();
  }

  template <typename Base>
  inline
  void
  parser::basic_symbol<Base>::clear ()
  {
    Base::clear ();
  }

  template <typename Base>
  inline
  bool
  parser::basic_symbol<Base>::empty () const
  {
    return Base::type_get () == empty_symbol;
  }

  template <typename Base>
  inline
  void
  parser::basic_symbol<Base>::move (basic_symbol& s)
  {
    super_type::move(s);
    value = s.value;
    location = s.location;
  }

  // by_type.
  inline
  parser::by_type::by_type ()
    : type (empty_symbol)
  {}

  inline
  parser::by_type::by_type (const by_type& other)
    : type (other.type)
  {}

  inline
  parser::by_type::by_type (token_type t)
    : type (yytranslate_ (t))
  {}

  inline
  void
  parser::by_type::clear ()
  {
    type = empty_symbol;
  }

  inline
  void
  parser::by_type::move (by_type& that)
  {
    type = that.type;
    that.clear ();
  }

  inline
  int
  parser::by_type::type_get () const
  {
    return type;
  }


  // by_state.
  inline
  parser::by_state::by_state ()
    : state (empty_state)
  {}

  inline
  parser::by_state::by_state (const by_state& other)
    : state (other.state)
  {}

  inline
  void
  parser::by_state::clear ()
  {
    state = empty_state;
  }

  inline
  void
  parser::by_state::move (by_state& that)
  {
    state = that.state;
    that.clear ();
  }

  inline
  parser::by_state::by_state (state_type s)
    : state (s)
  {}

  inline
  parser::symbol_number_type
  parser::by_state::type_get () const
  {
    if (state == empty_state)
      return empty_symbol;
    else
      return yystos_[state];
  }

  inline
  parser::stack_symbol_type::stack_symbol_type ()
  {}


  inline
  parser::stack_symbol_type::stack_symbol_type (state_type s, symbol_type& that)
    : super_type (s, that.location)
  {
    value = that.value;
    // that is emptied.
    that.type = empty_symbol;
  }

  inline
  parser::stack_symbol_type&
  parser::stack_symbol_type::operator= (const stack_symbol_type& that)
  {
    state = that.state;
    value = that.value;
    location = that.location;
    return *this;
  }


  template <typename Base>
  inline
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
    // Avoid a (spurious) G++ 4.8 warning about "array subscript is
    // below array bounds".
    if (yysym.empty ())
      std::abort ();
    yyo << (yytype < yyntokens_ ? "token" : "nterm")
        << ' ' << yytname_[yytype] << " ("
        << yysym.location << ": ";
    YYUSE (yytype);
    yyo << ')';
  }
#endif

  inline
  void
  parser::yypush_ (const char* m, state_type s, symbol_type& sym)
  {
    stack_symbol_type t (s, sym);
    yypush_ (m, t);
  }

  inline
  void
  parser::yypush_ (const char* m, stack_symbol_type& s)
  {
    if (m)
      YY_SYMBOL_PRINT (m, s);
    yystack_.push (s);
  }

  inline
  void
  parser::yypop_ (unsigned int n)
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

  inline parser::state_type
  parser::yy_lr_goto_state_ (state_type yystate, int yysym)
  {
    int yyr = yypgoto_[yysym - yyntokens_] + yystate;
    if (0 <= yyr && yyr <= yylast_ && yycheck_[yyr] == yystate)
      return yytable_[yyr];
    else
      return yydefgoto_[yysym - yyntokens_];
  }

  inline bool
  parser::yy_pact_value_is_default_ (int yyvalue)
  {
    return yyvalue == yypact_ninf_;
  }

  inline bool
  parser::yy_table_value_is_error_ (int yyvalue)
  {
    return yyvalue == yytable_ninf_;
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

    // FIXME: This shoud be completely indented.  It is not yet to
    // avoid gratuitous conflicts when merging into the master branch.
    try
      {
    YYCDEBUG << "Starting parse" << std::endl;


    /* Initialize the stack.  The initial state will be set in
       yynewstate, since the latter expects the semantical and the
       location values to have been already stored, initialize these
       stacks with a primary value.  */
    yystack_.clear ();
    yypush_ (YY_NULLPTR, 0, yyla);

    // A new symbol was pushed on the stack.
  yynewstate:
    YYCDEBUG << "Entering state " << yystack_[0].state << std::endl;

    // Accept?
    if (yystack_[0].state == yyfinal_)
      goto yyacceptlab;

    goto yybackup;

    // Backup.
  yybackup:

    // Try to take a decision without lookahead.
    yyn = yypact_[yystack_[0].state];
    if (yy_pact_value_is_default_ (yyn))
      goto yydefault;

    // Read a lookahead token.
    if (yyla.empty ())
      {
        YYCDEBUG << "Reading a token: ";
        try
          {
            yyla.type = yytranslate_ (yylex (&yyla.value, &yyla.location, lexer));
          }
        catch (const syntax_error& yyexc)
          {
            error (yyexc);
            goto yyerrlab1;
          }
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
    yypush_ ("Shifting", yyn, yyla);
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
  | yyreduce -- Do a reduction.  |
  `-----------------------------*/
  yyreduce:
    yylen = yyr2_[yyn];
    {
      stack_symbol_type yylhs;
      yylhs.state = yy_lr_goto_state_(yystack_[yylen].state, yyr1_[yyn]);
      /* If YYLEN is nonzero, implement the default value of the
         action: '$$ = $1'.  Otherwise, use the top of the stack.

         Otherwise, the following line sets YYLHS.VALUE to garbage.
         This behavior is undocumented and Bison users should not rely
         upon it.  */
      if (yylen)
        yylhs.value = yystack_[yylen - 1].value;
      else
        yylhs.value = yystack_[0].value;

      // Compute the default @$.
      {
        slice<stack_symbol_type, stack_type> slice (yystack_, yylen);
        YYLLOC_DEFAULT (yylhs.location, slice, yylen);
      }

      // Perform the reduction.
      YY_REDUCE_PRINT (yyn);
      try
        {
          switch (yyn)
            {
  case 7:
#line 348 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.str) = (yystack_[0].value.str); }
#line 666 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 8:
#line 355 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BinOp::XOR, (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 672 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 9:
#line 356 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BinOp::OR, (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 678 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 10:
#line 357 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BinOp::AND, (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 684 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 11:
#line 358 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BinOp::ADD, (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 690 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 12:
#line 359 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BinOp::SUB, (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 696 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 13:
#line 360 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BinOp::MUL, (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 702 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 14:
#line 361 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BinOp::DIV, (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 708 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 15:
#line 362 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BinOp::MOD, (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 714 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 16:
#line 363 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BinOp::POW, (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 720 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 17:
#line 364 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, UnOp::NEG, (yystack_[0].value.term)); }
#line 726 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 18:
#line 365 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, UnOp::NOT, (yystack_[0].value.term)); }
#line 732 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 19:
#line 366 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BUILDER.termvec(), false); }
#line 738 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 20:
#line 367 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BUILDER.termvec(), true); }
#line 744 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 21:
#line 368 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, (yystack_[1].value.termvec), false); }
#line 750 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 22:
#line 369 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, (yystack_[2].value.termvec), true); }
#line 756 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 23:
#line 370 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, String::fromRep((yystack_[3].value.str)), (yystack_[1].value.termvecvec), false); }
#line 762 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 24:
#line 371 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, String::fromRep((yystack_[3].value.str)), (yystack_[1].value.termvecvec), true); }
#line 768 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 25:
#line 372 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, UnOp::ABS, (yystack_[1].value.term)); }
#line 774 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 26:
#line 373 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, Symbol::createId(String::fromRep((yystack_[0].value.str)))); }
#line 780 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 27:
#line 374 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, Symbol::createNum((yystack_[0].value.num))); }
#line 786 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 28:
#line 375 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, Symbol::createStr(String::fromRep((yystack_[0].value.str)))); }
#line 792 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 29:
#line 376 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, Symbol::createInf()); }
#line 798 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 30:
#line 377 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, Symbol::createSup()); }
#line 804 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 31:
#line 383 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.termvec) = BUILDER.termvec(BUILDER.termvec(), (yystack_[0].value.term));  }
#line 810 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 32:
#line 384 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.termvec) = BUILDER.termvec((yystack_[2].value.termvec), (yystack_[0].value.term));  }
#line 816 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 33:
#line 388 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.termvecvec) = BUILDER.termvecvec(BUILDER.termvecvec(), (yystack_[0].value.termvec));  }
#line 822 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 34:
#line 389 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.termvecvec) = BUILDER.termvecvec();  }
#line 828 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 35:
#line 395 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 834 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 36:
#line 396 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BinOp::XOR, (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 840 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 37:
#line 397 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BinOp::OR, (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 846 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 38:
#line 398 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BinOp::AND, (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 852 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 39:
#line 399 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BinOp::ADD, (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 858 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 40:
#line 400 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BinOp::SUB, (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 864 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 41:
#line 401 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BinOp::MUL, (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 870 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 42:
#line 402 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BinOp::DIV, (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 876 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 43:
#line 403 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BinOp::MOD, (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 882 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 44:
#line 404 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BinOp::POW, (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 888 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 45:
#line 405 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, UnOp::NEG, (yystack_[0].value.term)); }
#line 894 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 46:
#line 406 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, UnOp::NOT, (yystack_[0].value.term)); }
#line 900 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 47:
#line 407 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.pool(yylhs.location, (yystack_[1].value.termvec)); }
#line 906 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 48:
#line 408 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, String::fromRep((yystack_[3].value.str)), (yystack_[1].value.termvecvec), false); }
#line 912 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 49:
#line 409 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, String::fromRep((yystack_[3].value.str)), (yystack_[1].value.termvecvec), true); }
#line 918 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 50:
#line 410 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, UnOp::ABS, (yystack_[1].value.termvec)); }
#line 924 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 51:
#line 411 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, Symbol::createId(String::fromRep((yystack_[0].value.str)))); }
#line 930 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 52:
#line 412 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, Symbol::createNum((yystack_[0].value.num))); }
#line 936 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 53:
#line 413 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, Symbol::createStr(String::fromRep((yystack_[0].value.str)))); }
#line 942 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 54:
#line 414 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, Symbol::createInf()); }
#line 948 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 55:
#line 415 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, Symbol::createSup()); }
#line 954 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 56:
#line 416 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, String::fromRep((yystack_[0].value.str))); }
#line 960 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 57:
#line 417 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, String("_")); }
#line 966 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 58:
#line 423 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.termvec) = BUILDER.termvec(BUILDER.termvec(), (yystack_[0].value.term)); }
#line 972 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 59:
#line 424 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.termvec) = BUILDER.termvec((yystack_[2].value.termvec), (yystack_[0].value.term)); }
#line 978 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 60:
#line 430 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.termvec) = BUILDER.termvec(BUILDER.termvec(), (yystack_[0].value.term)); }
#line 984 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 61:
#line 431 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.termvec) = BUILDER.termvec((yystack_[2].value.termvec), (yystack_[0].value.term)); }
#line 990 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 62:
#line 435 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.termvec) = (yystack_[0].value.termvec); }
#line 996 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 63:
#line 436 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.termvec) = BUILDER.termvec(); }
#line 1002 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 64:
#line 440 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, (yystack_[1].value.termvec), true); }
#line 1008 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 65:
#line 441 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, (yystack_[0].value.termvec), false); }
#line 1014 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 66:
#line 442 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BUILDER.termvec(), true); }
#line 1020 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 67:
#line 443 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, BUILDER.termvec(), false); }
#line 1026 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 68:
#line 446 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.termvec) = BUILDER.termvec(BUILDER.termvec(), (yystack_[1].value.term)); }
#line 1032 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 69:
#line 447 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.termvec) = BUILDER.termvec((yystack_[2].value.termvec), (yystack_[1].value.term)); }
#line 1038 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 70:
#line 450 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.termvec) = BUILDER.termvec(BUILDER.termvec(), (yystack_[0].value.term)); }
#line 1044 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 71:
#line 451 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.termvec) = BUILDER.termvec((yystack_[1].value.termvec), (yystack_[0].value.term)); }
#line 1050 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 72:
#line 454 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.termvecvec) = BUILDER.termvecvec(BUILDER.termvecvec(), (yystack_[0].value.termvec)); }
#line 1056 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 73:
#line 455 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.termvecvec) = BUILDER.termvecvec((yystack_[2].value.termvecvec), (yystack_[0].value.termvec)); }
#line 1062 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 74:
#line 459 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.termvecvec) = BUILDER.termvecvec(BUILDER.termvecvec(), BUILDER.termvec(BUILDER.termvec(BUILDER.termvec(), (yystack_[2].value.term)), (yystack_[0].value.term))); }
#line 1068 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 75:
#line 460 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.termvecvec) = BUILDER.termvecvec((yystack_[4].value.termvecvec), BUILDER.termvec(BUILDER.termvec(BUILDER.termvec(), (yystack_[2].value.term)), (yystack_[0].value.term))); }
#line 1074 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 76:
#line 470 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.rel) = Relation::GT; }
#line 1080 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 77:
#line 471 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.rel) = Relation::LT; }
#line 1086 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 78:
#line 472 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.rel) = Relation::GEQ; }
#line 1092 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 79:
#line 473 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.rel) = Relation::LEQ; }
#line 1098 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 80:
#line 474 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.rel) = Relation::EQ; }
#line 1104 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 81:
#line 475 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.rel) = Relation::NEQ; }
#line 1110 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 82:
#line 479 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.predRep(yylhs.location, false, String::fromRep((yystack_[0].value.str)), BUILDER.termvecvec(BUILDER.termvecvec(), BUILDER.termvec())); }
#line 1116 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 83:
#line 480 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.predRep(yylhs.location, false, String::fromRep((yystack_[3].value.str)), (yystack_[1].value.termvecvec)); }
#line 1122 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 84:
#line 481 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.predRep(yylhs.location, true, String::fromRep((yystack_[0].value.str)), BUILDER.termvecvec(BUILDER.termvecvec(), BUILDER.termvec())); }
#line 1128 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 85:
#line 482 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.predRep(yylhs.location, true, String::fromRep((yystack_[3].value.str)), (yystack_[1].value.termvecvec)); }
#line 1134 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 86:
#line 486 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.lit) = BUILDER.boollit(yylhs.location, true); }
#line 1140 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 87:
#line 487 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.lit) = BUILDER.boollit(yylhs.location, false); }
#line 1146 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 88:
#line 488 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.lit) = BUILDER.boollit(yylhs.location, true); }
#line 1152 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 89:
#line 489 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.lit) = BUILDER.boollit(yylhs.location, false); }
#line 1158 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 90:
#line 490 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.lit) = BUILDER.boollit(yylhs.location, true); }
#line 1164 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 91:
#line 491 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.lit) = BUILDER.boollit(yylhs.location, false); }
#line 1170 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 92:
#line 492 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.lit) = BUILDER.predlit(yylhs.location, NAF::POS, (yystack_[0].value.term)); }
#line 1176 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 93:
#line 493 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.lit) = BUILDER.predlit(yylhs.location, NAF::NOT, (yystack_[0].value.term)); }
#line 1182 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 94:
#line 494 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.lit) = BUILDER.predlit(yylhs.location, NAF::NOTNOT, (yystack_[0].value.term)); }
#line 1188 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 95:
#line 495 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.lit) = BUILDER.rellit(yylhs.location, (yystack_[1].value.rel), (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 1194 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 96:
#line 496 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.lit) = BUILDER.rellit(yylhs.location, neg((yystack_[1].value.rel)), (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 1200 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 97:
#line 497 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.lit) = BUILDER.rellit(yylhs.location, (yystack_[1].value.rel), (yystack_[2].value.term), (yystack_[0].value.term)); }
#line 1206 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 98:
#line 498 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.lit) = BUILDER.csplit((yystack_[0].value.csplit)); }
#line 1212 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 99:
#line 502 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.cspmulterm) = BUILDER.cspmulterm(yylhs.location, (yystack_[0].value.term),                     (yystack_[2].value.term)); }
#line 1218 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 100:
#line 503 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.cspmulterm) = BUILDER.cspmulterm(yylhs.location, (yystack_[3].value.term),                     (yystack_[0].value.term)); }
#line 1224 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 101:
#line 504 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.cspmulterm) = BUILDER.cspmulterm(yylhs.location, BUILDER.term(yylhs.location, Symbol::createNum(1)), (yystack_[0].value.term)); }
#line 1230 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 102:
#line 505 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.cspmulterm) = BUILDER.cspmulterm(yylhs.location, (yystack_[0].value.term)); }
#line 1236 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 103:
#line 509 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.cspaddterm) = BUILDER.cspaddterm(yylhs.location, (yystack_[2].value.cspaddterm), (yystack_[0].value.cspmulterm), true); }
#line 1242 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 104:
#line 510 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.cspaddterm) = BUILDER.cspaddterm(yylhs.location, (yystack_[2].value.cspaddterm), (yystack_[0].value.cspmulterm), false); }
#line 1248 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 105:
#line 511 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.cspaddterm) = BUILDER.cspaddterm(yylhs.location, (yystack_[0].value.cspmulterm)); }
#line 1254 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 106:
#line 515 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.rel) = Relation::GT; }
#line 1260 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 107:
#line 516 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.rel) = Relation::LT; }
#line 1266 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 108:
#line 517 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.rel) = Relation::GEQ; }
#line 1272 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 109:
#line 518 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.rel) = Relation::LEQ; }
#line 1278 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 110:
#line 519 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.rel) = Relation::EQ; }
#line 1284 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 111:
#line 520 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.rel) = Relation::NEQ; }
#line 1290 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 112:
#line 524 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.csplit) = BUILDER.csplit(yylhs.location, (yystack_[2].value.csplit), (yystack_[1].value.rel), (yystack_[0].value.cspaddterm)); }
#line 1296 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 113:
#line 525 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.csplit) = BUILDER.csplit(yylhs.location, (yystack_[2].value.cspaddterm),   (yystack_[1].value.rel), (yystack_[0].value.cspaddterm)); }
#line 1302 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 114:
#line 533 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.litvec) = BUILDER.litvec(BUILDER.litvec(), (yystack_[0].value.lit)); }
#line 1308 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 115:
#line 534 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.litvec) = BUILDER.litvec((yystack_[2].value.litvec), (yystack_[0].value.lit)); }
#line 1314 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 116:
#line 538 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.litvec) = (yystack_[0].value.litvec); }
#line 1320 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 117:
#line 539 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.litvec) = BUILDER.litvec(); }
#line 1326 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 118:
#line 543 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.litvec) = (yystack_[0].value.litvec); }
#line 1332 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 119:
#line 544 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.litvec) = BUILDER.litvec(); }
#line 1338 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 120:
#line 548 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.litvec) = (yystack_[0].value.litvec); }
#line 1344 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 121:
#line 549 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.litvec) = BUILDER.litvec(); }
#line 1350 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 122:
#line 553 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.fun) = AggregateFunction::SUM; }
#line 1356 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 123:
#line 554 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.fun) = AggregateFunction::SUMP; }
#line 1362 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 124:
#line 555 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.fun) = AggregateFunction::MIN; }
#line 1368 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 125:
#line 556 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.fun) = AggregateFunction::MAX; }
#line 1374 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 126:
#line 557 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.fun) = AggregateFunction::COUNT; }
#line 1380 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 127:
#line 563 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.bodyaggrelem) = { BUILDER.termvec(), (yystack_[0].value.litvec) }; }
#line 1386 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 128:
#line 564 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.bodyaggrelem) = { (yystack_[1].value.termvec), (yystack_[0].value.litvec) }; }
#line 1392 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 129:
#line 568 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.bodyaggrelemvec) = BUILDER.bodyaggrelemvec(BUILDER.bodyaggrelemvec(), (yystack_[0].value.bodyaggrelem).first, (yystack_[0].value.bodyaggrelem).second); }
#line 1398 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 130:
#line 569 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.bodyaggrelemvec) = BUILDER.bodyaggrelemvec((yystack_[2].value.bodyaggrelemvec), (yystack_[0].value.bodyaggrelem).first, (yystack_[0].value.bodyaggrelem).second); }
#line 1404 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 131:
#line 575 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.lbodyaggrelem) = { (yystack_[1].value.lit), (yystack_[0].value.litvec) }; }
#line 1410 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 132:
#line 579 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.condlitlist) = BUILDER.condlitvec(BUILDER.condlitvec(), (yystack_[0].value.lbodyaggrelem).first, (yystack_[0].value.lbodyaggrelem).second); }
#line 1416 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 133:
#line 580 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.condlitlist) = BUILDER.condlitvec((yystack_[2].value.condlitlist), (yystack_[0].value.lbodyaggrelem).first, (yystack_[0].value.lbodyaggrelem).second); }
#line 1422 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 134:
#line 586 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.aggr) = { AggregateFunction::COUNT, true, BUILDER.condlitvec() }; }
#line 1428 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 135:
#line 587 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.aggr) = { AggregateFunction::COUNT, true, (yystack_[1].value.condlitlist) }; }
#line 1434 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 136:
#line 588 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.aggr) = { (yystack_[2].value.fun), false, BUILDER.bodyaggrelemvec() }; }
#line 1440 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 137:
#line 589 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.aggr) = { (yystack_[3].value.fun), false, (yystack_[1].value.bodyaggrelemvec) }; }
#line 1446 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 138:
#line 593 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.bound) = { Relation::LEQ, (yystack_[0].value.term) }; }
#line 1452 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 139:
#line 594 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.bound) = { (yystack_[1].value.rel), (yystack_[0].value.term) }; }
#line 1458 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 140:
#line 595 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.bound) = { Relation::LEQ, TermUid(-1) }; }
#line 1464 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 141:
#line 599 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.uid) = lexer->aggregate((yystack_[1].value.aggr).fun, (yystack_[1].value.aggr).choice, (yystack_[1].value.aggr).elems, lexer->boundvec(Relation::LEQ, (yystack_[2].value.term), (yystack_[0].value.bound).rel, (yystack_[0].value.bound).term)); }
#line 1470 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 142:
#line 600 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.uid) = lexer->aggregate((yystack_[1].value.aggr).fun, (yystack_[1].value.aggr).choice, (yystack_[1].value.aggr).elems, lexer->boundvec((yystack_[2].value.rel), (yystack_[3].value.term), (yystack_[0].value.bound).rel, (yystack_[0].value.bound).term)); }
#line 1476 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 143:
#line 601 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.uid) = lexer->aggregate((yystack_[1].value.aggr).fun, (yystack_[1].value.aggr).choice, (yystack_[1].value.aggr).elems, lexer->boundvec(Relation::LEQ, TermUid(-1), (yystack_[0].value.bound).rel, (yystack_[0].value.bound).term)); }
#line 1482 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 144:
#line 602 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.uid) = lexer->aggregate((yystack_[0].value.theoryAtom)); }
#line 1488 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 145:
#line 608 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.headaggrelemvec) = BUILDER.headaggrelemvec((yystack_[5].value.headaggrelemvec), (yystack_[3].value.termvec), (yystack_[1].value.lit), (yystack_[0].value.litvec)); }
#line 1494 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 146:
#line 609 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.headaggrelemvec) = BUILDER.headaggrelemvec(BUILDER.headaggrelemvec(), (yystack_[3].value.termvec), (yystack_[1].value.lit), (yystack_[0].value.litvec)); }
#line 1500 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 147:
#line 613 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.condlitlist) = BUILDER.condlitvec(BUILDER.condlitvec(), (yystack_[1].value.lit), (yystack_[0].value.litvec)); }
#line 1506 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 148:
#line 614 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.condlitlist) = BUILDER.condlitvec((yystack_[3].value.condlitlist), (yystack_[1].value.lit), (yystack_[0].value.litvec)); }
#line 1512 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 149:
#line 620 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.aggr) = { (yystack_[2].value.fun), false, BUILDER.headaggrelemvec() }; }
#line 1518 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 150:
#line 621 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.aggr) = { (yystack_[3].value.fun), false, (yystack_[1].value.headaggrelemvec) }; }
#line 1524 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 151:
#line 622 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.aggr) = { AggregateFunction::COUNT, true, BUILDER.condlitvec()}; }
#line 1530 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 152:
#line 623 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.aggr) = { AggregateFunction::COUNT, true, (yystack_[1].value.condlitlist)}; }
#line 1536 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 153:
#line 627 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.uid) = lexer->aggregate((yystack_[1].value.aggr).fun, (yystack_[1].value.aggr).choice, (yystack_[1].value.aggr).elems, lexer->boundvec(Relation::LEQ, (yystack_[2].value.term), (yystack_[0].value.bound).rel, (yystack_[0].value.bound).term)); }
#line 1542 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 154:
#line 628 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.uid) = lexer->aggregate((yystack_[1].value.aggr).fun, (yystack_[1].value.aggr).choice, (yystack_[1].value.aggr).elems, lexer->boundvec((yystack_[2].value.rel), (yystack_[3].value.term), (yystack_[0].value.bound).rel, (yystack_[0].value.bound).term)); }
#line 1548 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 155:
#line 629 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.uid) = lexer->aggregate((yystack_[1].value.aggr).fun, (yystack_[1].value.aggr).choice, (yystack_[1].value.aggr).elems, lexer->boundvec(Relation::LEQ, TermUid(-1), (yystack_[0].value.bound).rel, (yystack_[0].value.bound).term)); }
#line 1554 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 156:
#line 630 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.uid) = lexer->aggregate((yystack_[0].value.theoryAtom)); }
#line 1560 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 157:
#line 636 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.cspelemvec) = BUILDER.cspelemvec(BUILDER.cspelemvec(), yylhs.location, (yystack_[3].value.termvec), (yystack_[1].value.cspaddterm), (yystack_[0].value.litvec)); }
#line 1566 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 158:
#line 637 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.cspelemvec) = BUILDER.cspelemvec((yystack_[5].value.cspelemvec), yylhs.location, (yystack_[3].value.termvec), (yystack_[1].value.cspaddterm), (yystack_[0].value.litvec)); }
#line 1572 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 159:
#line 641 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.cspelemvec) = (yystack_[0].value.cspelemvec); }
#line 1578 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 160:
#line 642 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.cspelemvec) = BUILDER.cspelemvec(); }
#line 1584 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 161:
#line 646 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.disjoint) = { NAF::POS, (yystack_[1].value.cspelemvec) }; }
#line 1590 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 162:
#line 647 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.disjoint) = { NAF::NOT, (yystack_[1].value.cspelemvec) }; }
#line 1596 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 163:
#line 648 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.disjoint) = { NAF::NOTNOT, (yystack_[1].value.cspelemvec) }; }
#line 1602 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 164:
#line 655 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.lbodyaggrelem) = { (yystack_[2].value.lit), (yystack_[0].value.litvec) }; }
#line 1608 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 167:
#line 667 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.condlitlist) = BUILDER.condlitvec((yystack_[2].value.condlitlist), (yystack_[1].value.lit), BUILDER.litvec()); }
#line 1614 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 168:
#line 668 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.condlitlist) = BUILDER.condlitvec((yystack_[3].value.condlitlist), (yystack_[2].value.lit), (yystack_[1].value.litvec)); }
#line 1620 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 169:
#line 669 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.condlitlist) = BUILDER.condlitvec(); }
#line 1626 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 170:
#line 674 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.condlitlist) = BUILDER.condlitvec(BUILDER.condlitvec((yystack_[2].value.condlitlist), (yystack_[1].value.lit), (yystack_[0].value.litvec)), (yystack_[4].value.lit), BUILDER.litvec()); }
#line 1632 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 171:
#line 675 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.condlitlist) = BUILDER.condlitvec(BUILDER.condlitvec((yystack_[2].value.condlitlist), (yystack_[1].value.lit), (yystack_[0].value.litvec)), (yystack_[4].value.lit), BUILDER.litvec()); }
#line 1638 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 172:
#line 676 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.condlitlist) = BUILDER.condlitvec(BUILDER.condlitvec((yystack_[2].value.condlitlist), (yystack_[1].value.lit), (yystack_[0].value.litvec)), (yystack_[6].value.lit), (yystack_[4].value.litvec)); }
#line 1644 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 173:
#line 677 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.condlitlist) = BUILDER.condlitvec(BUILDER.condlitvec(), (yystack_[2].value.lit), (yystack_[0].value.litvec)); }
#line 1650 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 174:
#line 684 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.body) = BUILDER.bodylit((yystack_[2].value.body), (yystack_[1].value.lit)); }
#line 1656 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 175:
#line 685 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.body) = BUILDER.bodylit((yystack_[2].value.body), (yystack_[1].value.lit)); }
#line 1662 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 176:
#line 686 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.body) = lexer->bodyaggregate((yystack_[2].value.body), yystack_[1].location, NAF::POS, (yystack_[1].value.uid)); }
#line 1668 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 177:
#line 687 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.body) = lexer->bodyaggregate((yystack_[2].value.body), yystack_[1].location, NAF::POS, (yystack_[1].value.uid)); }
#line 1674 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 178:
#line 688 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.body) = lexer->bodyaggregate((yystack_[3].value.body), yystack_[1].location + yystack_[2].location, NAF::NOT, (yystack_[1].value.uid)); }
#line 1680 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 179:
#line 689 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.body) = lexer->bodyaggregate((yystack_[3].value.body), yystack_[1].location + yystack_[2].location, NAF::NOT, (yystack_[1].value.uid)); }
#line 1686 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 180:
#line 690 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.body) = lexer->bodyaggregate((yystack_[4].value.body), yystack_[1].location + yystack_[3].location, NAF::NOTNOT, (yystack_[1].value.uid)); }
#line 1692 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 181:
#line 691 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.body) = lexer->bodyaggregate((yystack_[4].value.body), yystack_[1].location + yystack_[3].location, NAF::NOTNOT, (yystack_[1].value.uid)); }
#line 1698 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 182:
#line 692 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.body) = BUILDER.conjunction((yystack_[2].value.body), yystack_[1].location, (yystack_[1].value.lbodyaggrelem).first, (yystack_[1].value.lbodyaggrelem).second); }
#line 1704 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 183:
#line 693 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.body) = BUILDER.disjoint((yystack_[2].value.body), yystack_[1].location, (yystack_[1].value.disjoint).first, (yystack_[1].value.disjoint).second); }
#line 1710 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 184:
#line 694 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.body) = BUILDER.body(); }
#line 1716 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 185:
#line 698 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.body) = BUILDER.bodylit((yystack_[2].value.body), (yystack_[1].value.lit)); }
#line 1722 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 186:
#line 699 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.body) = lexer->bodyaggregate((yystack_[2].value.body), yystack_[1].location, NAF::POS, (yystack_[1].value.uid)); }
#line 1728 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 187:
#line 700 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.body) = lexer->bodyaggregate((yystack_[3].value.body), yystack_[1].location + yystack_[2].location, NAF::NOT, (yystack_[1].value.uid)); }
#line 1734 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 188:
#line 701 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.body) = lexer->bodyaggregate((yystack_[4].value.body), yystack_[1].location + yystack_[3].location, NAF::NOTNOT, (yystack_[1].value.uid)); }
#line 1740 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 189:
#line 702 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.body) = BUILDER.conjunction((yystack_[2].value.body), yystack_[1].location, (yystack_[1].value.lbodyaggrelem).first, (yystack_[1].value.lbodyaggrelem).second); }
#line 1746 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 190:
#line 703 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.body) = BUILDER.disjoint((yystack_[2].value.body), yystack_[1].location, (yystack_[1].value.disjoint).first, (yystack_[1].value.disjoint).second); }
#line 1752 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 191:
#line 707 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.body) = BUILDER.body(); }
#line 1758 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 192:
#line 708 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.body) = BUILDER.body(); }
#line 1764 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 193:
#line 709 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.body) = (yystack_[0].value.body); }
#line 1770 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 194:
#line 712 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.head) = BUILDER.headlit((yystack_[0].value.lit)); }
#line 1776 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 195:
#line 713 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.head) = BUILDER.disjunction(yylhs.location, (yystack_[0].value.condlitlist)); }
#line 1782 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 196:
#line 714 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.head) = lexer->headaggregate(yylhs.location, (yystack_[0].value.uid)); }
#line 1788 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 197:
#line 718 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { BUILDER.rule(yylhs.location, (yystack_[1].value.head)); }
#line 1794 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 198:
#line 719 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { BUILDER.rule(yylhs.location, (yystack_[2].value.head)); }
#line 1800 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 199:
#line 720 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { BUILDER.rule(yylhs.location, (yystack_[2].value.head), (yystack_[0].value.body)); }
#line 1806 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 200:
#line 721 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { BUILDER.rule(yylhs.location, BUILDER.headlit(BUILDER.boollit(yylhs.location, false)), (yystack_[0].value.body)); }
#line 1812 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 201:
#line 722 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { BUILDER.rule(yylhs.location, BUILDER.headlit(BUILDER.boollit(yylhs.location, false)), BUILDER.body()); }
#line 1818 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 202:
#line 728 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { BUILDER.rule(yylhs.location, BUILDER.headlit(BUILDER.boollit(yystack_[2].location, false)), BUILDER.disjoint((yystack_[0].value.body), yystack_[2].location, inv((yystack_[2].value.disjoint).first), (yystack_[2].value.disjoint).second)); }
#line 1824 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 203:
#line 729 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { BUILDER.rule(yylhs.location, BUILDER.headlit(BUILDER.boollit(yystack_[2].location, false)), BUILDER.disjoint(BUILDER.body(), yystack_[2].location, inv((yystack_[2].value.disjoint).first), (yystack_[2].value.disjoint).second)); }
#line 1830 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 204:
#line 730 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { BUILDER.rule(yylhs.location, BUILDER.headlit(BUILDER.boollit(yystack_[1].location, false)), BUILDER.disjoint(BUILDER.body(), yystack_[1].location, inv((yystack_[1].value.disjoint).first), (yystack_[1].value.disjoint).second)); }
#line 1836 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 205:
#line 736 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.termvec) = (yystack_[0].value.termvec); }
#line 1842 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 206:
#line 737 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.termvec) = BUILDER.termvec(); }
#line 1848 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 207:
#line 741 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.termpair) = {(yystack_[2].value.term), (yystack_[0].value.term)}; }
#line 1854 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 208:
#line 742 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.termpair) = {(yystack_[0].value.term), BUILDER.term(yylhs.location, Symbol::createNum(0))}; }
#line 1860 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 209:
#line 746 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.body) = BUILDER.bodylit(BUILDER.body(), (yystack_[0].value.lit)); }
#line 1866 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 210:
#line 747 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.body) = BUILDER.bodylit((yystack_[2].value.body), (yystack_[0].value.lit)); }
#line 1872 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 211:
#line 751 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.body) = (yystack_[0].value.body); }
#line 1878 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 212:
#line 752 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.body) = BUILDER.body(); }
#line 1884 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 213:
#line 753 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.body) = BUILDER.body(); }
#line 1890 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 214:
#line 757 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { BUILDER.optimize(yylhs.location, (yystack_[2].value.termpair).first, (yystack_[2].value.termpair).second, (yystack_[1].value.termvec), (yystack_[4].value.body)); }
#line 1896 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 215:
#line 758 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { BUILDER.optimize(yylhs.location, (yystack_[2].value.termpair).first, (yystack_[2].value.termpair).second, (yystack_[1].value.termvec), BUILDER.body()); }
#line 1902 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 216:
#line 762 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { BUILDER.optimize(yylhs.location, BUILDER.term(yystack_[2].location, UnOp::NEG, (yystack_[2].value.termpair).first), (yystack_[2].value.termpair).second, (yystack_[1].value.termvec), (yystack_[0].value.body)); }
#line 1908 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 217:
#line 763 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { BUILDER.optimize(yylhs.location, BUILDER.term(yystack_[2].location, UnOp::NEG, (yystack_[2].value.termpair).first), (yystack_[2].value.termpair).second, (yystack_[1].value.termvec), (yystack_[0].value.body)); }
#line 1914 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 218:
#line 767 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { BUILDER.optimize(yylhs.location, (yystack_[2].value.termpair).first, (yystack_[2].value.termpair).second, (yystack_[1].value.termvec), (yystack_[0].value.body)); }
#line 1920 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 219:
#line 768 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { BUILDER.optimize(yylhs.location, (yystack_[2].value.termpair).first, (yystack_[2].value.termpair).second, (yystack_[1].value.termvec), (yystack_[0].value.body)); }
#line 1926 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 224:
#line 781 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { BUILDER.showsig(yylhs.location, Sig(String::fromRep((yystack_[3].value.str)), (yystack_[1].value.num), false), false); }
#line 1932 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 225:
#line 782 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { BUILDER.showsig(yylhs.location, Sig(String::fromRep((yystack_[3].value.str)), (yystack_[1].value.num), true), false); }
#line 1938 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 226:
#line 783 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { BUILDER.showsig(yylhs.location, Sig("", 0, false), false); }
#line 1944 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 227:
#line 784 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { BUILDER.show(yylhs.location, (yystack_[2].value.term), (yystack_[0].value.body), false); }
#line 1950 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 228:
#line 785 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { BUILDER.show(yylhs.location, (yystack_[1].value.term), BUILDER.body(), false); }
#line 1956 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 229:
#line 786 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { BUILDER.showsig(yylhs.location, Sig(String::fromRep((yystack_[3].value.str)), (yystack_[1].value.num), false), true); }
#line 1962 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 230:
#line 787 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { BUILDER.show(yylhs.location, (yystack_[2].value.term), (yystack_[0].value.body), true); }
#line 1968 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 231:
#line 788 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { BUILDER.show(yylhs.location, (yystack_[1].value.term), BUILDER.body(), true); }
#line 1974 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 232:
#line 794 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { BUILDER.edge(yylhs.location, (yystack_[2].value.termvecvec), (yystack_[0].value.body)); }
#line 1980 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 233:
#line 800 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { BUILDER.heuristic(yylhs.location, (yystack_[8].value.term), (yystack_[7].value.body), (yystack_[5].value.term), (yystack_[3].value.term), (yystack_[1].value.term)); }
#line 1986 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 234:
#line 801 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { BUILDER.heuristic(yylhs.location, (yystack_[6].value.term), (yystack_[5].value.body), (yystack_[3].value.term), BUILDER.term(yylhs.location, Symbol::createNum(0)), (yystack_[1].value.term)); }
#line 1992 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 235:
#line 807 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { BUILDER.project(yylhs.location, Sig(String::fromRep((yystack_[3].value.str)), (yystack_[1].value.num), false)); }
#line 1998 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 236:
#line 808 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { BUILDER.project(yylhs.location, Sig(String::fromRep((yystack_[3].value.str)), (yystack_[1].value.num), true)); }
#line 2004 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 237:
#line 809 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { BUILDER.project(yylhs.location, (yystack_[1].value.term), (yystack_[0].value.body)); }
#line 2010 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 238:
#line 815 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    {  BUILDER.define(yylhs.location, String::fromRep((yystack_[2].value.str)), (yystack_[0].value.term), false, LOGGER); }
#line 2016 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 239:
#line 819 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    {  BUILDER.define(yylhs.location, String::fromRep((yystack_[3].value.str)), (yystack_[1].value.term), true, LOGGER); }
#line 2022 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 240:
#line 825 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { BUILDER.python(yylhs.location, String::fromRep((yystack_[1].value.str))); }
#line 2028 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 241:
#line 826 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { BUILDER.lua(yylhs.location, String::fromRep((yystack_[1].value.str))); }
#line 2034 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 242:
#line 832 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { lexer->include(String::fromRep((yystack_[1].value.str)), yylhs.location, false, LOGGER); }
#line 2040 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 243:
#line 833 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { lexer->include(String::fromRep((yystack_[2].value.str)), yylhs.location, true, LOGGER); }
#line 2046 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 244:
#line 839 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.idlist) = BUILDER.idvec((yystack_[2].value.idlist), yystack_[0].location, String::fromRep((yystack_[0].value.str))); }
#line 2052 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 245:
#line 840 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.idlist) = BUILDER.idvec(BUILDER.idvec(), yystack_[0].location, String::fromRep((yystack_[0].value.str))); }
#line 2058 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 246:
#line 844 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.idlist) = BUILDER.idvec(); }
#line 2064 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 247:
#line 845 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.idlist) = (yystack_[0].value.idlist); }
#line 2070 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 248:
#line 849 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { BUILDER.block(yylhs.location, String::fromRep((yystack_[4].value.str)), (yystack_[2].value.idlist)); }
#line 2076 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 249:
#line 850 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { BUILDER.block(yylhs.location, String::fromRep((yystack_[1].value.str)), BUILDER.idvec()); }
#line 2082 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 250:
#line 856 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { BUILDER.external(yylhs.location, (yystack_[2].value.term), (yystack_[0].value.body)); }
#line 2088 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 251:
#line 857 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { BUILDER.external(yylhs.location, (yystack_[2].value.term), BUILDER.body()); }
#line 2094 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 252:
#line 858 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { BUILDER.external(yylhs.location, (yystack_[1].value.term), BUILDER.body()); }
#line 2100 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 253:
#line 866 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryOps) = BUILDER.theoryops((yystack_[1].value.theoryOps), String::fromRep((yystack_[0].value.str))); }
#line 2106 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 254:
#line 867 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryOps) = BUILDER.theoryops(BUILDER.theoryops(), String::fromRep((yystack_[0].value.str))); }
#line 2112 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 255:
#line 871 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryTerm) = BUILDER.theorytermset(yylhs.location, (yystack_[1].value.theoryOpterms)); }
#line 2118 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 256:
#line 872 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryTerm) = BUILDER.theoryoptermlist(yylhs.location, (yystack_[1].value.theoryOpterms)); }
#line 2124 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 257:
#line 873 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryTerm) = BUILDER.theorytermtuple(yylhs.location, BUILDER.theoryopterms()); }
#line 2130 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 258:
#line 874 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryTerm) = BUILDER.theorytermopterm(yylhs.location, (yystack_[1].value.theoryOpterm)); }
#line 2136 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 259:
#line 875 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryTerm) = BUILDER.theorytermtuple(yylhs.location, BUILDER.theoryopterms(BUILDER.theoryopterms(), yystack_[2].location, (yystack_[2].value.theoryOpterm))); }
#line 2142 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 260:
#line 876 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryTerm) = BUILDER.theorytermtuple(yylhs.location, BUILDER.theoryopterms(yystack_[3].location, (yystack_[3].value.theoryOpterm), (yystack_[1].value.theoryOpterms))); }
#line 2148 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 261:
#line 877 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryTerm) = BUILDER.theorytermfun(yylhs.location, String::fromRep((yystack_[3].value.str)), (yystack_[1].value.theoryOpterms)); }
#line 2154 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 262:
#line 878 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryTerm) = BUILDER.theorytermvalue(yylhs.location, Symbol::createId(String::fromRep((yystack_[0].value.str)))); }
#line 2160 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 263:
#line 879 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryTerm) = BUILDER.theorytermvalue(yylhs.location, Symbol::createNum((yystack_[0].value.num))); }
#line 2166 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 264:
#line 880 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryTerm) = BUILDER.theorytermvalue(yylhs.location, Symbol::createStr(String::fromRep((yystack_[0].value.str)))); }
#line 2172 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 265:
#line 881 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryTerm) = BUILDER.theorytermvalue(yylhs.location, Symbol::createInf()); }
#line 2178 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 266:
#line 882 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryTerm) = BUILDER.theorytermvalue(yylhs.location, Symbol::createSup()); }
#line 2184 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 267:
#line 883 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryTerm) = BUILDER.theorytermvar(yylhs.location, String::fromRep((yystack_[0].value.str))); }
#line 2190 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 268:
#line 887 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryOpterm) = BUILDER.theoryopterm((yystack_[2].value.theoryOpterm), (yystack_[1].value.theoryOps), (yystack_[0].value.theoryTerm)); }
#line 2196 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 269:
#line 888 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryOpterm) = BUILDER.theoryopterm((yystack_[1].value.theoryOps), (yystack_[0].value.theoryTerm)); }
#line 2202 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 270:
#line 889 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryOpterm) = BUILDER.theoryopterm(BUILDER.theoryops(), (yystack_[0].value.theoryTerm)); }
#line 2208 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 271:
#line 893 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryOpterms) = BUILDER.theoryopterms((yystack_[2].value.theoryOpterms), yystack_[0].location, (yystack_[0].value.theoryOpterm)); }
#line 2214 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 272:
#line 894 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryOpterms) = BUILDER.theoryopterms(BUILDER.theoryopterms(), yystack_[0].location, (yystack_[0].value.theoryOpterm)); }
#line 2220 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 273:
#line 898 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryOpterms) = (yystack_[0].value.theoryOpterms); }
#line 2226 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 274:
#line 899 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryOpterms) = BUILDER.theoryopterms(); }
#line 2232 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 275:
#line 903 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryElem) = { (yystack_[2].value.theoryOpterms), (yystack_[0].value.litvec) }; }
#line 2238 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 276:
#line 904 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryElem) = { BUILDER.theoryopterms(), (yystack_[0].value.litvec) }; }
#line 2244 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 277:
#line 908 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryElems) = BUILDER.theoryelems((yystack_[3].value.theoryElems), (yystack_[0].value.theoryElem).first, (yystack_[0].value.theoryElem).second); }
#line 2250 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 278:
#line 909 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryElems) = BUILDER.theoryelems(BUILDER.theoryelems(), (yystack_[0].value.theoryElem).first, (yystack_[0].value.theoryElem).second); }
#line 2256 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 279:
#line 913 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryElems) = (yystack_[0].value.theoryElems); }
#line 2262 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 280:
#line 914 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryElems) = BUILDER.theoryelems(); }
#line 2268 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 281:
#line 918 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, String::fromRep((yystack_[0].value.str)), BUILDER.termvecvec(BUILDER.termvecvec(), BUILDER.termvec()), false); }
#line 2274 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 282:
#line 919 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.term) = BUILDER.term(yylhs.location, String::fromRep((yystack_[3].value.str)), (yystack_[1].value.termvecvec), false); }
#line 2280 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 283:
#line 922 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryAtom) = BUILDER.theoryatom((yystack_[6].value.term), (yystack_[3].value.theoryElems)); }
#line 2286 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 284:
#line 923 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryAtom) = BUILDER.theoryatom((yystack_[8].value.term), (yystack_[5].value.theoryElems), String::fromRep((yystack_[2].value.str)), yystack_[1].location, (yystack_[1].value.theoryOpterm)); }
#line 2292 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 285:
#line 929 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryOps) = BUILDER.theoryops(BUILDER.theoryops(), String::fromRep((yystack_[0].value.str))); }
#line 2298 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 286:
#line 930 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryOps) = BUILDER.theoryops((yystack_[2].value.theoryOps), String::fromRep((yystack_[0].value.str))); }
#line 2304 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 287:
#line 934 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryOps) = (yystack_[0].value.theoryOps); }
#line 2310 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 288:
#line 935 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryOps) = BUILDER.theoryops(); }
#line 2316 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 289:
#line 939 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryOpDef) = BUILDER.theoryopdef(yylhs.location, String::fromRep((yystack_[5].value.str)), (yystack_[2].value.num), TheoryOperatorType::Unary); }
#line 2322 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 290:
#line 940 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryOpDef) = BUILDER.theoryopdef(yylhs.location, String::fromRep((yystack_[7].value.str)), (yystack_[4].value.num), TheoryOperatorType::BinaryLeft); }
#line 2328 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 291:
#line 941 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryOpDef) = BUILDER.theoryopdef(yylhs.location, String::fromRep((yystack_[7].value.str)), (yystack_[4].value.num), TheoryOperatorType::BinaryRight); }
#line 2334 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 292:
#line 945 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryOpDefs) = BUILDER.theoryopdefs(BUILDER.theoryopdefs(), (yystack_[0].value.theoryOpDef)); }
#line 2340 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 293:
#line 946 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryOpDefs) = BUILDER.theoryopdefs((yystack_[3].value.theoryOpDefs), (yystack_[0].value.theoryOpDef)); }
#line 2346 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 294:
#line 950 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryOpDefs) = (yystack_[0].value.theoryOpDefs); }
#line 2352 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 295:
#line 951 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryOpDefs) = BUILDER.theoryopdefs(); }
#line 2358 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 296:
#line 955 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.str) = (yystack_[0].value.str); }
#line 2364 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 297:
#line 956 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.str) = String::toRep("left"); }
#line 2370 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 298:
#line 957 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.str) = String::toRep("right"); }
#line 2376 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 299:
#line 958 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.str) = String::toRep("unary"); }
#line 2382 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 300:
#line 959 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.str) = String::toRep("binary"); }
#line 2388 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 301:
#line 960 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.str) = String::toRep("head"); }
#line 2394 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 302:
#line 961 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.str) = String::toRep("body"); }
#line 2400 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 303:
#line 962 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.str) = String::toRep("any"); }
#line 2406 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 304:
#line 963 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.str) = String::toRep("directive"); }
#line 2412 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 305:
#line 967 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryTermDef) = BUILDER.theorytermdef(yylhs.location, String::fromRep((yystack_[5].value.str)), (yystack_[2].value.theoryOpDefs), LOGGER); }
#line 2418 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 306:
#line 971 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryAtomType) = TheoryAtomType::Head; }
#line 2424 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 307:
#line 972 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryAtomType) = TheoryAtomType::Body; }
#line 2430 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 308:
#line 973 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryAtomType) = TheoryAtomType::Any; }
#line 2436 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 309:
#line 974 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryAtomType) = TheoryAtomType::Directive; }
#line 2442 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 310:
#line 979 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryAtomDef) = BUILDER.theoryatomdef(yylhs.location, String::fromRep((yystack_[14].value.str)), (yystack_[12].value.num), String::fromRep((yystack_[10].value.str)), (yystack_[0].value.theoryAtomType), (yystack_[6].value.theoryOps), String::fromRep((yystack_[2].value.str))); }
#line 2448 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 311:
#line 980 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryAtomDef) = BUILDER.theoryatomdef(yylhs.location, String::fromRep((yystack_[6].value.str)), (yystack_[4].value.num), String::fromRep((yystack_[2].value.str)), (yystack_[0].value.theoryAtomType)); }
#line 2454 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 312:
#line 984 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryDefs) = BUILDER.theorydefs((yystack_[2].value.theoryDefs), (yystack_[0].value.theoryAtomDef)); }
#line 2460 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 313:
#line 985 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryDefs) = BUILDER.theorydefs((yystack_[2].value.theoryDefs), (yystack_[0].value.theoryTermDef)); }
#line 2466 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 314:
#line 986 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryDefs) = BUILDER.theorydefs(BUILDER.theorydefs(), (yystack_[0].value.theoryAtomDef)); }
#line 2472 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 315:
#line 987 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryDefs) = BUILDER.theorydefs(BUILDER.theorydefs(), (yystack_[0].value.theoryTermDef)); }
#line 2478 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 316:
#line 991 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryDefs) = (yystack_[0].value.theoryDefs); }
#line 2484 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 317:
#line 992 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { (yylhs.value.theoryDefs) = BUILDER.theorydefs(); }
#line 2490 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 318:
#line 996 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { BUILDER.theorydef(yylhs.location, String::fromRep((yystack_[6].value.str)), (yystack_[3].value.theoryDefs), LOGGER); }
#line 2496 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 319:
#line 1002 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { lexer->theoryLexing(TheoryLexing::Theory); }
#line 2502 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 320:
#line 1006 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { lexer->theoryLexing(TheoryLexing::Definition); }
#line 2508 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;

  case 321:
#line 1010 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:859
    { lexer->theoryLexing(TheoryLexing::Disabled); }
#line 2514 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
    break;


#line 2518 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:859
            default:
              break;
            }
        }
      catch (const syntax_error& yyexc)
        {
          error (yyexc);
          YYERROR;
        }
      YY_SYMBOL_PRINT ("-> $$ =", yylhs);
      yypop_ (yylen);
      yylen = 0;
      YY_STACK_PRINT ();

      // Shift the result of the reduction.
      yypush_ (YY_NULLPTR, yylhs);
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

    /* Pacify compilers like GCC when the user code never invokes
       YYERROR and the label yyerrorlab therefore never appears in user
       code.  */
    if (false)
      goto yyerrorlab;
    yyerror_range[1].location = yystack_[yylen - 1].location;
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
      yypush_ ("Shifting", error_token);
    }
    goto yynewstate;

    // Accept.
  yyacceptlab:
    yyresult = 0;
    goto yyreturn;

    // Abort.
  yyabortlab:
    yyresult = 1;
    goto yyreturn;

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
    catch (...)
      {
        YYCDEBUG << "Exception caught: cleaning lookahead and stack"
                 << std::endl;
        // Do not try to display the values of the reclaimed symbols,
        // as their printer might throw an exception.
        if (!yyla.empty ())
          yy_destroy_ (YY_NULLPTR, yyla);

        while (1 < yystack_.size ())
          {
            yy_destroy_ (YY_NULLPTR, yystack_[0]);
            yypop_ ();
          }
        throw;
      }
  }

  void
  parser::error (const syntax_error& yyexc)
  {
    error (yyexc.location, yyexc.what());
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
        YYCASE_(0, YY_("syntax error"));
        YYCASE_(1, YY_("syntax error, unexpected %s"));
        YYCASE_(2, YY_("syntax error, unexpected %s, expecting %s"));
        YYCASE_(3, YY_("syntax error, unexpected %s, expecting %s or %s"));
        YYCASE_(4, YY_("syntax error, unexpected %s, expecting %s or %s or %s"));
        YYCASE_(5, YY_("syntax error, unexpected %s, expecting %s or %s or %s or %s"));
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


  const short int parser::yypact_ninf_ = -514;

  const short int parser::yytable_ninf_ = -322;

  const short int
  parser::yypact_[] =
  {
     152,  -514,   -13,    85,   914,  -514,    94,  -514,  -514,  -514,
     -13,   -13,  1354,   -13,  -514,  1354,    69,    70,  -514,   108,
      24,  -514,   376,  1223,  -514,   142,  -514,   146,   560,   149,
      91,    70,    30,  1354,  -514,  -514,  -514,  -514,   -13,  1354,
     174,   -13,  -514,  -514,   176,   178,  -514,  -514,   983,  -514,
    1154,  1448,  -514,    48,  -514,  1015,   612,   187,  1047,  -514,
     160,  -514,   186,  -514,  1388,   214,   219,  -514,   225,  1354,
     231,  -514,   245,  1704,   662,   -13,   240,    67,  -514,   336,
    -514,   -13,   248,  -514,  1020,  1594,   271,    73,  -514,  1815,
     278,   249,  1223,   256,  1254,  1263,  1354,  -514,  1640,  1354,
     -13,    98,   151,   151,   -13,   -13,   247,  1883,  -514,   141,
    1815,    14,   273,   283,  -514,  -514,  -514,   288,  -514,  -514,
    1153,  1624,  -514,  1354,  1354,  1354,  -514,   318,  1354,  -514,
    -514,  -514,  -514,  1354,  1354,  -514,  1354,  1354,  1354,  1354,
    1354,  1056,  1047,   802,  -514,  -514,  -514,  -514,  1281,  1281,
    -514,  -514,  -514,  -514,  -514,  -514,  1281,  1281,  1290,  1815,
    1354,  -514,  -514,   308,  -514,   322,   -13,  1388,  -514,   829,
    1388,  -514,  1388,  -514,  -514,   317,   309,  -514,  1354,   323,
    1354,  1354,  1388,  1354,   342,   354,  -514,   188,   337,  1354,
     348,  -514,   760,   651,  1498,   117,   345,  1047,    68,    35,
      42,  -514,   346,  -514,  1160,  1354,   802,  -514,  -514,   802,
    1354,  -514,   339,  -514,   358,  1721,   392,   212,   380,   392,
     250,  1670,  -514,  -514,  1750,   235,   124,   327,   386,  -514,
    -514,   377,   373,   388,   368,  1354,  -514,   -13,  1354,  -514,
    1354,  1354,   420,   662,   421,  -514,  -514,  1624,  -514,  1354,
    -514,   286,   207,   499,  1354,  1866,   410,   410,   410,   532,
     410,   207,    65,  1815,  1047,  -514,  -514,    39,   802,   802,
    1755,  -514,  -514,   331,   331,  -514,   452,   254,  1815,  -514,
    -514,  -514,  -514,   424,  -514,   414,  -514,   309,    45,  -514,
    1839,  1388,  1388,  1388,  1388,  1388,  1388,  1388,  1388,  1388,
    1388,   299,   443,   310,   314,   464,  1815,  1354,  1281,  -514,
    1354,  1354,   335,  -514,  -514,  -514,   271,  -514,   262,   719,
    1548,   134,  1113,  1047,   802,  -514,  -514,  -514,  1196,  -514,
    -514,  -514,  -514,  -514,  -514,  -514,  -514,   445,   461,  -514,
     271,  1815,  -514,  -514,  1354,  1354,   465,   450,  1354,  -514,
     465,   451,  1354,  -514,  -514,  -514,  1354,   151,  1354,   400,
     459,  -514,  -514,  1354,   403,   404,   462,   349,  -514,   478,
     440,  1815,   392,   392,   154,   266,   662,  1354,  1815,  1552,
    1354,  1815,  -514,   802,  -514,   407,   407,   802,  -514,  1354,
    1388,  -514,  1381,  -514,  -514,   482,   444,   227,   643,   457,
     457,   457,   636,   457,   227,   389,  -514,  -514,   898,   898,
    1882,  -514,  -514,  -514,  -514,  -514,   460,  1893,  -514,   408,
     489,  -514,   454,  -514,   508,  -514,  -514,  -514,   267,   511,
     370,  -514,  -514,  -514,   802,  1548,   153,  1113,  -514,  -514,
    -514,  1047,  -514,  -514,   802,  -514,   418,  -514,   272,  -514,
    -514,  1815,   342,   802,  -514,  -514,   392,  -514,  -514,   392,
    -514,  1815,  -514,  1779,   496,  -514,  1687,   503,   504,  -514,
    1894,   -13,   513,   474,   483,   797,  -514,  -514,  -514,  -514,
    -514,  -514,  -514,  -514,  -514,  -514,  -514,  -514,  -514,   276,
    -514,   280,  1815,  -514,  -514,   802,   802,  -514,    23,    23,
     271,   530,   490,  -514,   309,  1388,  -514,   489,   491,   495,
    -514,    46,   898,  -514,  -514,  1893,   898,   271,   498,   502,
     802,  -514,  1281,  -514,  -514,  1113,  -514,  -514,  -514,  -514,
    -514,  -514,  -514,  1347,  -514,   538,   465,   465,  1354,  -514,
    1354,  1354,  -514,  -514,  -514,  -514,  -514,  -514,   514,   518,
    -514,   154,  -514,   407,   461,  -514,  -514,   802,  -514,  -514,
    -514,  1903,  -514,   505,  -514,   408,  -514,   898,   480,  -514,
     267,  -514,   802,  -514,  -514,  1815,  1807,  1206,   472,   485,
     546,  -514,  -514,    23,   271,  -514,    54,  -514,  -514,   898,
    -514,  -514,  -514,  1354,  -514,   566,  -514,  -514,   454,  -514,
    -514,  -514,  -514,   408,  1831,   797,   569,   528,   533,  -514,
    -514,   571,   501,   485,  -514,   136,   574,  -514,  -514,  -514,
    -514,  -514,  -514,   551,   355,   500,  -514,   587,  -514,   589,
    -514,   364,   523,   564,  -514,  -514,  -514,   590,   797,   604,
     136,  -514
  };

  const unsigned short int
  parser::yydefact_[] =
  {
       0,     5,     0,     0,     0,     7,     0,     3,     1,   321,
       0,     0,     0,     0,   126,     0,     0,     0,    89,   184,
       0,    54,     0,    67,   125,     0,   124,     0,     0,     0,
       0,     0,     0,     0,   122,   123,    55,    86,     0,     0,
     184,     0,    52,    57,     0,     0,    53,    56,     0,     4,
      51,   102,    92,   194,   105,     0,    98,     0,   140,   196,
       0,   195,     0,   156,     0,     0,   281,   319,     0,     0,
      51,    46,     0,   101,   160,     0,    82,     0,   201,     0,
     200,     0,     0,   151,     0,   102,   119,     0,    66,    60,
      65,    70,    67,     0,     0,     0,     0,   226,     0,     0,
       0,    82,     0,     0,     0,     0,     0,    51,    45,     0,
      58,     0,     0,     0,   320,   240,   241,     0,    90,    87,
       0,     0,    93,    63,     0,     0,    80,     0,     0,    78,
      76,    79,    77,     0,     0,    81,     0,     0,     0,     0,
       0,     0,   140,     0,   169,   165,   166,   169,     0,     0,
     109,   107,   106,   108,   110,   111,     0,     0,    63,   138,
       0,   155,   204,   184,   197,   184,     0,     0,    29,     0,
       0,    30,     0,    27,    28,    26,   238,     6,    63,     0,
      63,    63,     0,     0,    62,     0,   159,     0,    84,    63,
     184,   252,     0,     0,   102,     0,     0,   140,     0,     0,
       0,   144,     0,   242,     0,     0,   117,   147,   152,     0,
      64,    68,    71,    47,     0,   208,   206,     0,     0,   206,
       0,     0,   184,   228,     0,     0,    84,     0,   184,   191,
     237,     0,     0,     0,     0,    63,   249,   246,     0,    50,
       0,     0,     0,   160,     0,    91,    88,     0,    94,     0,
      72,     0,    39,    38,     0,    35,    43,    41,    44,    37,
      42,    40,    36,    95,   140,   153,   114,   173,     0,     0,
     102,   103,   104,   113,   112,   149,     0,     0,   139,   203,
     202,   198,   199,     0,    18,     0,    19,    31,     0,    17,
       0,    34,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,   280,     0,     0,     0,    99,     0,     0,   161,
      63,    63,     0,   251,   250,   134,   119,   132,     0,     0,
       0,     0,     0,   140,   117,   174,   185,   175,     0,   143,
     176,   186,   177,   190,   183,   189,   182,     0,   116,   118,
     119,    61,    69,   221,     0,     0,   213,     0,     0,   220,
     213,     0,     0,   184,   231,   227,     0,     0,     0,     0,
       0,   192,   193,     0,     0,     0,     0,     0,   245,   247,
       0,    59,   206,   206,   317,     0,   160,     0,    96,    48,
      63,   100,   154,     0,   169,   121,   121,     0,   150,    63,
      34,    20,     0,    21,    25,    33,     0,    11,    10,    15,
      13,    16,     9,    14,    12,     8,   282,   265,   274,   274,
       0,   266,   263,   264,   267,   254,   262,     0,   270,   272,
     321,   278,   279,   319,     0,    49,    48,   239,   119,     0,
       0,    83,   131,   135,     0,     0,     0,     0,   178,   187,
     179,   140,   141,   164,   117,   136,   119,   129,     0,   243,
     148,   207,   205,   212,   216,   223,   206,   218,   222,   206,
     230,    74,   232,     0,     0,   235,     0,     0,     0,   224,
      48,     0,     0,     0,     0,     0,   303,   299,   300,   297,
     298,   301,   302,   304,   296,   319,   315,   314,   316,     0,
     162,     0,    97,    73,   115,     0,     0,   167,   170,   171,
     119,     0,     0,    22,    32,     0,    23,   273,     0,     0,
     257,     0,   274,   253,   269,     0,     0,   119,     0,     0,
     117,   157,     0,    85,   133,     0,   180,   188,   181,   142,
     127,   128,   137,     0,   209,   211,   213,   213,     0,   236,
       0,     0,   229,   225,   244,   248,   215,   214,     0,     0,
     321,     0,   163,   121,   120,   168,   146,     0,    24,   255,
     256,     0,   258,     0,   268,   271,   275,   321,   321,   276,
     119,   130,     0,   217,   219,    75,     0,     0,     0,   295,
       0,   313,   312,   172,   119,   259,     0,   261,   277,     0,
     283,   158,   210,     0,   234,     0,   320,   292,   294,   320,
     318,   145,   260,   321,     0,     0,     0,     0,     0,   284,
     233,     0,     0,     0,   305,   319,     0,   293,   308,   306,
     307,   309,   311,     0,     0,   288,   289,     0,   285,   287,
     320,     0,     0,     0,   290,   291,   286,     0,     0,     0,
       0,   310
  };

  const short int
  parser::yypgoto_[] =
  {
    -514,  -514,  -514,  -514,    -2,   -51,   446,   228,   468,  -514,
     -18,   -50,   529,  -514,  -514,  -128,  -514,   -41,    17,    66,
     296,  -153,   570,  -514,  -137,  -301,  -300,  -359,    33,    87,
    -514,   201,  -514,  -182,  -125,  -158,  -514,  -514,     8,  -514,
    -514,  -205,   558,  -514,   -31,  -122,  -514,  -514,   -33,   -89,
    -514,  -200,   -44,  -514,  -324,  -514,  -514,  -514,  -514,  -514,
    -377,  -363,  -369,  -284,  -364,    74,  -514,  -514,  -514,   641,
    -514,  -514,    36,  -514,  -514,  -454,    99,    12,   102,  -514,
    -514,  -390,  -513,    -8
  };

  const short int
  parser::yydefgoto_[] =
  {
      -1,     3,     4,    49,    70,   287,   395,   396,    89,   111,
     184,   250,    91,    92,    93,   251,   225,   160,    52,   266,
      54,    55,   156,    56,   338,   339,   207,   498,   196,   447,
     448,   317,   318,   197,   161,   198,   277,    87,    58,    59,
     186,   187,    60,   200,   555,   268,    61,    79,    80,   230,
      62,   346,   216,   535,   454,   217,   220,     7,   369,   370,
     417,   418,   419,   507,   508,   421,   422,   423,    67,   201,
     629,   630,   597,   598,   599,   485,   486,   622,   487,   488,
     489,   179,   242,   424
  };

  const short int
  parser::yytable_[] =
  {
       6,    65,    50,   273,   274,    90,   267,   113,    66,    68,
     141,    72,   323,   176,   231,    76,   432,   265,   420,   350,
      50,   548,   147,   443,   185,   269,   457,   499,   101,    76,
     106,   107,   518,   519,    77,   321,   109,    57,   375,   114,
     450,   511,   515,   104,   205,   509,    50,   102,   103,   383,
     301,   219,   303,   304,   514,   392,   561,   143,   144,   142,
     333,   312,   175,    81,   516,   122,   238,   335,   124,   125,
      53,     5,   329,   188,    90,   145,   190,    50,   330,   202,
     249,   239,    50,   606,    57,     8,   608,   334,    86,   105,
     146,   145,   191,   331,   336,   549,   393,   562,   226,    64,
     145,   122,   232,   233,    74,   602,   146,   367,   276,   133,
     134,    82,   136,   137,     5,   146,   284,   633,    50,   289,
     332,   290,   208,   138,   139,   209,   324,   325,   521,    75,
     280,   305,   282,    78,   515,   415,   189,   248,   323,   382,
     441,    50,   326,   530,   438,   195,   531,   565,   563,   264,
     100,   611,   564,   322,     5,   428,   227,   314,   475,   439,
     228,   436,   311,   526,   283,   175,   236,   175,   175,   327,
     175,   491,   473,   474,    57,     5,   229,    94,   527,   237,
     175,    95,   359,   430,   639,   162,   440,    99,   515,   355,
      50,    50,   163,   185,   583,   362,   372,   373,   442,   112,
     556,   115,    50,   116,    50,   528,   377,    50,   607,   618,
     122,   164,   573,   574,   619,   620,   621,   566,   165,   569,
     603,   248,   158,     1,     2,   623,   515,   476,   477,   478,
     479,   480,   481,   482,   483,   368,   384,   309,     5,   177,
     310,   397,   398,   399,   400,   401,   402,   403,   404,   405,
     182,   133,   134,   323,   136,   441,   536,   178,   316,   537,
     429,   347,   495,   180,   348,   138,    50,    50,   462,   181,
     591,   294,   295,   203,   296,   340,   206,   586,   189,   437,
     206,   148,   149,   420,   601,   298,   357,   358,   210,   175,
     175,   175,   175,   175,   175,   175,   175,   175,   175,   351,
     416,   211,   352,   388,   456,   234,   389,   213,   459,   240,
     446,   433,   292,   293,   434,   490,   529,    50,   310,   241,
     460,   532,    50,   243,   533,   550,   185,   452,   551,   552,
     493,   254,   310,   279,   385,   386,   248,   379,   380,   501,
      10,   504,    11,   441,    12,   148,   149,   281,    14,    15,
     406,   380,   307,   294,   295,   291,   296,   297,   302,   554,
      16,   425,   380,   308,    18,   426,   380,   298,   299,   570,
      21,   192,   484,   313,    23,   311,    24,   337,    26,   300,
     328,    50,    11,   343,    12,    50,   431,   380,   175,    15,
     175,   342,   292,   293,   525,    33,    34,    35,    36,    37,
     470,   380,   345,    39,    18,   349,   416,   416,   416,   360,
      21,   361,   517,   363,    23,   416,   496,   497,    42,    43,
       5,   523,   380,    46,    47,    83,   193,   206,   307,   626,
     627,   364,    50,   294,   295,    33,   296,   297,    36,    37,
     634,   635,    50,    39,   271,   272,   365,   298,   299,   494,
     366,    50,  -321,   500,   504,   374,   376,   136,    42,    43,
       5,   387,   390,    46,    47,   391,    84,   292,   293,   544,
     449,   383,    51,   484,   453,   455,   458,   407,   408,   409,
      71,   410,   464,    73,   465,   467,   468,   469,   471,   427,
      85,   472,   505,    50,    50,   506,    98,   415,   512,   516,
     316,   108,   124,   175,   296,   411,  -319,   110,   294,   295,
     416,   296,   297,   416,   416,   446,   121,   520,    50,   534,
     522,   539,   298,   299,   546,   412,   159,     5,   542,   543,
     413,   414,   415,   547,   300,   124,   125,   108,   545,   557,
     559,   558,   580,   133,   134,   560,   136,   194,   572,   484,
     567,   568,   121,   579,   595,    50,   587,   138,   139,   416,
     590,   553,   215,   215,   221,   416,    11,   224,    12,   589,
      50,   600,   578,    96,   596,   605,   133,   134,   612,   136,
     613,   615,   614,   616,   624,    97,   625,   416,   247,   628,
     138,   139,   252,   253,    21,   609,   255,   631,    23,   632,
     638,   256,   257,   484,   258,   259,   260,   261,   262,   263,
     159,    85,   636,   637,   640,   288,   270,   270,   502,    69,
     571,   212,    36,   584,   270,   270,   157,    39,   278,   150,
     151,   152,   153,   154,   155,   524,   484,   199,   592,   292,
     293,   588,    42,    43,     5,    63,   292,    46,    47,   617,
     581,   306,   641,   582,     0,    10,     0,    11,     0,    12,
      85,   320,     0,    14,     0,   159,     0,     0,    11,     0,
      12,   -63,   247,   263,    85,   117,     0,    85,   341,   118,
     294,   295,     0,   296,     0,    21,   192,   294,   295,    23,
     296,    24,     0,    26,   298,   299,    21,     0,     0,     0,
      23,   298,   299,     0,     0,     0,   371,     0,   215,   215,
      33,    34,    35,    36,   119,     0,     0,   378,    39,     0,
       0,    69,   381,    10,    36,    11,     0,    12,     0,    39,
       0,    14,   159,    42,    43,     5,    85,    85,    46,    47,
       0,   319,     0,   244,    42,    43,     5,   245,     0,    46,
      47,     0,     0,    21,   192,     0,     0,    23,     0,    24,
       0,    26,     0,     0,     0,     0,    11,     0,    12,     0,
       0,     0,     0,    15,     0,   341,   270,     0,    33,    34,
      35,    36,   246,     0,     0,     0,    39,   435,    18,     0,
     263,   159,    85,     0,    21,     0,     0,     0,    23,     0,
       0,    42,    43,     5,     0,     0,    46,    47,    11,   315,
      12,     0,   451,     0,     0,    15,   215,     0,     0,    33,
     215,     0,    36,    37,   461,     0,   463,    39,     0,     0,
      18,   466,     0,     0,     0,   166,    21,   167,     0,   285,
      23,     0,    42,    43,     5,   492,     0,    46,    47,     0,
      84,    85,     0,     0,     0,    85,     0,     0,     0,     0,
       0,    33,     0,   168,    36,    37,     0,   169,     0,    39,
     476,   477,   478,   479,   480,   481,   482,   483,     0,     0,
     286,     5,     0,     0,    42,    43,     5,     0,   170,    46,
      47,   171,    84,     0,     0,     0,   172,     0,     0,     0,
       0,     0,    85,     0,     0,   378,     0,     0,     0,   159,
       0,   173,    85,     5,    -2,     9,   174,     0,    10,     0,
      11,    85,    12,     0,     0,    13,    14,    15,     0,     0,
       0,     0,   407,   408,   409,     0,   410,     0,    16,     0,
       0,    17,    18,     0,     0,     0,    19,    20,    21,    22,
       0,     0,    23,     0,    24,    25,    26,    27,     0,     0,
     411,     0,     0,    85,    85,     0,     0,    28,    29,    30,
      31,    32,     0,    33,    34,    35,    36,    37,    38,     0,
     412,    39,     5,    40,     0,   413,   414,   415,    85,    11,
     270,    12,     0,   492,     0,    41,    42,    43,     5,    44,
      45,    46,    47,     0,    48,     0,   575,   117,   576,   577,
       0,   118,     0,     0,     0,     0,     0,    21,     0,     0,
       0,    23,     0,     0,     0,    85,    11,     0,    12,   148,
     149,     0,   150,   151,   152,   153,   154,   155,     0,     0,
      85,     0,    33,     0,     0,    36,   119,     0,   118,     0,
      39,     0,   126,    11,    21,    12,     0,     0,    23,     0,
       0,   604,    11,     0,    12,    42,    43,     5,    14,     0,
      46,    47,     0,   120,     0,     0,     0,   129,   130,    33,
       0,    21,    36,   119,   131,    23,   132,    39,     0,     0,
      21,    22,     0,   135,    23,     0,    24,     0,    26,     0,
       0,     0,    42,    43,     5,     0,    69,    46,    47,    36,
     204,     0,     0,     0,    39,    69,    34,    35,    36,    11,
       0,    12,     0,    39,     0,    14,     0,     0,     0,    42,
      43,     5,     0,     0,    46,    47,     0,     0,    42,    43,
       5,     0,     0,    46,    47,     0,     0,    21,   192,     0,
       0,    23,     0,    24,     0,    26,     0,     0,     0,    11,
       0,    12,     0,   -82,   -82,     0,    11,     0,    12,     0,
       0,     0,    69,    34,    35,    36,     0,   244,     0,   -82,
      39,   245,     0,     0,     0,     0,   -82,    21,   245,     0,
       0,    23,   123,     0,    21,    42,    43,     5,    23,     0,
      46,    47,    11,   -82,    12,   444,   -82,     0,     0,   124,
     125,     0,    33,     0,     0,    36,   246,     0,     0,    33,
      39,   -82,    36,   246,     0,     0,     0,    39,     0,    11,
      21,    12,   128,    88,    23,    42,    43,     5,     0,     0,
      46,    47,    42,    43,     5,   445,     0,    46,    47,     0,
     133,   134,     0,   136,   137,    69,   594,    21,    36,     0,
      11,    23,    12,    39,   138,   139,     0,     0,     0,    11,
       0,    12,     0,     0,     0,     0,   140,     0,    42,    43,
       5,     0,    69,    46,    47,    36,     0,    11,    21,    12,
      39,     0,    23,     0,    15,     0,    11,    21,    12,     0,
       0,    23,     0,   214,     0,    42,    43,     5,     0,     0,
      46,    47,   218,    69,     0,    21,    36,     0,     0,    23,
       0,    39,    69,     0,    21,    36,     0,     0,    23,     0,
      39,     0,     0,     0,     0,     0,    42,    43,     5,   275,
      69,    46,    47,    36,     0,    42,    43,     5,    39,    69,
      46,    47,    36,    11,     0,    12,   444,    39,     0,     0,
      11,     0,    12,    42,    43,     5,     0,     0,    46,    47,
       0,     0,    42,    43,     5,     0,     0,    46,    47,     0,
       0,    21,     0,     0,     0,    23,     0,   166,    21,   167,
       0,     0,    23,     0,   166,     0,   167,     0,     0,     0,
       0,     0,     0,     0,     0,     0,    69,     0,     0,    36,
       0,     0,     0,    69,    39,   168,    36,     0,     0,   169,
       0,    39,   168,     0,     0,     0,   169,     0,     0,    42,
      43,     5,   503,     0,    46,    47,    42,    43,     5,     0,
     170,    46,    47,   171,     0,     0,     0,   170,   172,     0,
     171,   124,   125,   126,     0,   172,     0,     0,     0,     0,
      14,     0,     0,   173,   127,     5,     0,     0,   174,     0,
     173,     0,     5,     0,   128,   174,     0,     0,   129,   130,
       0,     0,     0,    22,     0,   131,     0,   132,    24,     0,
      26,     0,   133,   134,   135,   136,   137,     0,     0,     0,
       0,   124,   125,   126,     0,     0,   138,   139,    34,    35,
      14,     0,     0,     0,   127,     0,     0,     0,   140,     0,
       0,     0,     0,     0,   128,     0,     0,     0,   129,   130,
       0,     0,     0,   192,     0,   131,     0,   132,    24,     0,
      26,     0,   133,   134,   135,   136,   137,     0,     0,     0,
       0,   124,   125,   126,     0,     0,   138,   139,    34,    35,
      14,   -83,   -83,     0,     0,     0,     0,     0,   140,     0,
       0,     0,     0,     0,   128,     0,     0,   -83,   129,   130,
       0,     0,     0,   192,   -83,   131,     0,   132,    24,     0,
      26,     0,   133,   134,   135,   136,   137,   124,   125,   126,
       0,   -83,     0,     0,   -83,     0,   138,   139,    34,    35,
     127,     0,     0,     0,     0,     0,     0,     0,   140,   -83,
     128,     0,     0,     0,   129,   130,     0,   124,   125,   126,
       0,   131,     0,   132,     0,     0,     0,     0,   133,   134,
     135,   136,   137,   124,   125,     0,     0,     0,     0,   222,
     128,     0,   138,   139,   129,   130,     0,     0,     0,     0,
       0,   131,     0,   132,   140,   223,   128,     0,   133,   134,
     135,   136,   137,   124,   125,     0,     0,     0,     0,   353,
       0,     0,   138,   139,   133,   134,     0,   136,   137,     0,
     124,   125,     0,   540,   140,   354,   128,   541,   138,   139,
       0,     0,     0,     0,     0,     0,     0,   124,   125,     0,
     140,     0,     0,   128,   133,   134,     0,   136,   137,     0,
     183,     0,     0,     0,   124,   125,     0,   344,   138,   139,
     128,   133,   134,     0,   136,   137,     0,     0,     0,     0,
     140,     0,     0,     0,     0,   138,   139,   128,   133,   134,
       0,   136,   137,   124,   125,     0,     0,   140,   124,   125,
     356,     0,   138,   139,     0,   133,   134,     0,   136,   137,
       0,   127,     0,     0,   140,     0,   128,     0,     0,   138,
     139,   128,   124,   125,     0,     0,     0,     0,     0,   538,
       0,   140,     0,     0,   133,   134,     0,   136,   137,   133,
     134,     0,   136,   137,     0,   128,     0,     0,   138,   139,
     124,   125,     0,   138,   139,     0,     0,   593,   124,   125,
     140,     0,     0,   133,   134,   140,   136,   137,     0,     0,
       0,     0,     0,   128,   124,   125,     0,   138,   139,     0,
       0,   128,   292,   293,     0,     0,     0,     0,     0,   140,
       0,   133,   134,     0,   136,   137,     0,   128,     0,   133,
     134,     0,   136,   137,     0,   138,   139,     0,     0,   124,
     125,     0,     0,   138,   139,   133,   134,   140,   136,   137,
       0,   610,     0,   294,   295,   140,   296,   297,     0,   138,
     139,     0,   -84,   -84,     0,     0,     0,   298,   299,     0,
       0,   140,     0,   -85,   -85,     0,   394,     0,   -84,   300,
     133,   134,     0,   136,   137,   -84,   407,   408,   409,   -85,
     410,   235,     0,     0,   138,   139,   -85,   407,   408,   409,
       0,   410,   -84,   510,     0,   -84,   140,   407,   408,   409,
       0,   410,     0,   -85,   411,     0,   -85,     0,     0,     0,
     -84,     0,     0,     0,   585,   411,     0,     0,     0,     0,
       0,   -85,     0,     0,   412,   411,     5,     0,     0,   413,
     414,   415,     0,     0,     0,   412,     0,     5,     0,     0,
     413,   414,   513,     0,     0,   412,     0,     5,     0,     0,
     413,   414,   415
  };

  const short int
  parser::yycheck_[] =
  {
       2,     9,     4,   156,   157,    23,   143,    40,    10,    11,
      51,    13,   194,    64,   103,    17,   316,   142,   302,   219,
      22,   475,    53,   324,    74,   147,   350,   386,    30,    31,
      32,    33,   422,   423,    17,   193,    38,     4,   243,    41,
     340,   410,   419,    13,    85,   409,    48,    30,    31,    10,
     178,    95,   180,   181,   417,    10,    10,     9,    10,    51,
      25,   189,    64,    39,    10,    48,    52,    25,     3,     4,
       4,    84,   197,    75,    92,    52,     9,    79,    10,    81,
     121,    67,    84,   596,    51,     0,   599,    52,    22,    59,
      67,    52,    25,    25,    52,   485,    51,    51,   100,     5,
      52,    84,   104,   105,    35,    51,    67,   235,   158,    44,
      45,    87,    47,    48,    84,    67,   167,   630,   120,   170,
      52,   172,    49,    58,    59,    52,     9,    10,   428,    59,
     163,   182,   165,    25,   511,    89,    38,   120,   320,   264,
     322,   143,    25,   444,    10,    79,   446,   516,   512,   141,
      59,   605,   515,   194,    84,   308,    58,   190,     4,    25,
       9,   319,    38,    10,   166,   167,    25,   169,   170,    52,
     172,   376,   372,   373,   141,    84,    25,    35,    25,    38,
     182,    35,    58,   311,   638,    25,    52,    38,   565,   222,
     192,   193,    32,   243,   553,   228,   240,   241,   323,    25,
     500,    25,   204,    25,   206,    52,   247,   209,   598,    73,
     193,    25,   536,   537,    78,    79,    80,   517,    32,   520,
     589,   204,    35,    71,    72,   615,   603,    73,    74,    75,
      76,    77,    78,    79,    80,   237,   267,    49,    84,    25,
      52,   292,   293,   294,   295,   296,   297,   298,   299,   300,
       5,    44,    45,   435,    47,   437,   456,    38,   192,   459,
     310,    49,   384,    38,    52,    58,   268,   269,   357,    38,
     570,    44,    45,    25,    47,   209,     9,   561,    38,   320,
       9,    14,    15,   567,   584,    58,    51,    52,    10,   291,
     292,   293,   294,   295,   296,   297,   298,   299,   300,    49,
     302,    52,    52,    49,   348,    58,    52,    51,   352,    36,
     328,    49,     3,     4,    52,    49,   441,   319,    52,    36,
     353,    49,   324,    35,    52,    49,   376,   345,    52,    49,
     380,    13,    52,    25,   268,   269,   319,    51,    52,   389,
       4,   392,     6,   525,     8,    14,    15,    25,    12,    13,
      51,    52,    10,    44,    45,    38,    47,    48,    35,   496,
      24,    51,    52,     9,    28,    51,    52,    58,    59,   522,
      34,    35,   374,    25,    38,    38,    40,    31,    42,    70,
      35,   383,     6,    25,     8,   387,    51,    52,   390,    13,
     392,    52,     3,     4,   435,    59,    60,    61,    62,    63,
      51,    52,    10,    67,    28,    25,   408,   409,   410,    82,
      34,    25,   420,    36,    38,   417,     9,    10,    82,    83,
      84,    51,    52,    87,    88,    49,    90,     9,    10,    74,
      75,    58,   434,    44,    45,    59,    47,    48,    62,    63,
      76,    77,   444,    67,   148,   149,    58,    58,    59,   383,
      82,   453,     9,   387,   505,    35,    35,    47,    82,    83,
      84,     9,    38,    87,    88,    51,    90,     3,     4,   471,
      25,    10,     4,   475,     9,    25,    25,    34,    35,    36,
      12,    38,    82,    15,    25,    82,    82,    25,    10,    25,
      22,    51,    10,   495,   496,    51,    28,    89,    38,    10,
     434,    33,     3,   505,    47,    62,    52,    39,    44,    45,
     512,    47,    48,   515,   516,   533,    48,     9,   520,   453,
       9,    25,    58,    59,    50,    82,    58,    84,    25,    25,
      87,    88,    89,    50,    70,     3,     4,    69,    25,     9,
      49,    51,   550,    44,    45,    50,    47,    79,    10,   551,
      52,    49,    84,    35,    82,   557,    51,    58,    59,   561,
     568,   495,    94,    95,    96,   567,     6,    99,     8,    89,
     572,    25,    58,    13,    89,     9,    44,    45,     9,    47,
      52,    10,    49,    82,    10,    25,    35,   589,   120,    89,
      58,    59,   124,   125,    34,   603,   128,    10,    38,    10,
      10,   133,   134,   605,   136,   137,   138,   139,   140,   141,
     142,   143,    89,    49,    10,   169,   148,   149,   390,    59,
     533,    92,    62,   557,   156,   157,    56,    67,   160,    17,
      18,    19,    20,    21,    22,   434,   638,    79,   572,     3,
       4,   567,    82,    83,    84,     4,     3,    87,    88,   613,
     551,   183,   640,   551,    -1,     4,    -1,     6,    -1,     8,
     192,   193,    -1,    12,    -1,   197,    -1,    -1,     6,    -1,
       8,     9,   204,   205,   206,    24,    -1,   209,   210,    28,
      44,    45,    -1,    47,    -1,    34,    35,    44,    45,    38,
      47,    40,    -1,    42,    58,    59,    34,    -1,    -1,    -1,
      38,    58,    59,    -1,    -1,    -1,   238,    -1,   240,   241,
      59,    60,    61,    62,    63,    -1,    -1,   249,    67,    -1,
      -1,    59,   254,     4,    62,     6,    -1,     8,    -1,    67,
      -1,    12,   264,    82,    83,    84,   268,   269,    87,    88,
      -1,    90,    -1,    24,    82,    83,    84,    28,    -1,    87,
      88,    -1,    -1,    34,    35,    -1,    -1,    38,    -1,    40,
      -1,    42,    -1,    -1,    -1,    -1,     6,    -1,     8,    -1,
      -1,    -1,    -1,    13,    -1,   307,   308,    -1,    59,    60,
      61,    62,    63,    -1,    -1,    -1,    67,   319,    28,    -1,
     322,   323,   324,    -1,    34,    -1,    -1,    -1,    38,    -1,
      -1,    82,    83,    84,    -1,    -1,    87,    88,     6,    49,
       8,    -1,   344,    -1,    -1,    13,   348,    -1,    -1,    59,
     352,    -1,    62,    63,   356,    -1,   358,    67,    -1,    -1,
      28,   363,    -1,    -1,    -1,     6,    34,     8,    -1,    10,
      38,    -1,    82,    83,    84,   377,    -1,    87,    88,    -1,
      90,   383,    -1,    -1,    -1,   387,    -1,    -1,    -1,    -1,
      -1,    59,    -1,    34,    62,    63,    -1,    38,    -1,    67,
      73,    74,    75,    76,    77,    78,    79,    80,    -1,    -1,
      51,    84,    -1,    -1,    82,    83,    84,    -1,    59,    87,
      88,    62,    90,    -1,    -1,    -1,    67,    -1,    -1,    -1,
      -1,    -1,   434,    -1,    -1,   437,    -1,    -1,    -1,   441,
      -1,    82,   444,    84,     0,     1,    87,    -1,     4,    -1,
       6,   453,     8,    -1,    -1,    11,    12,    13,    -1,    -1,
      -1,    -1,    34,    35,    36,    -1,    38,    -1,    24,    -1,
      -1,    27,    28,    -1,    -1,    -1,    32,    33,    34,    35,
      -1,    -1,    38,    -1,    40,    41,    42,    43,    -1,    -1,
      62,    -1,    -1,   495,   496,    -1,    -1,    53,    54,    55,
      56,    57,    -1,    59,    60,    61,    62,    63,    64,    -1,
      82,    67,    84,    69,    -1,    87,    88,    89,   520,     6,
     522,     8,    -1,   525,    -1,    81,    82,    83,    84,    85,
      86,    87,    88,    -1,    90,    -1,   538,    24,   540,   541,
      -1,    28,    -1,    -1,    -1,    -1,    -1,    34,    -1,    -1,
      -1,    38,    -1,    -1,    -1,   557,     6,    -1,     8,    14,
      15,    -1,    17,    18,    19,    20,    21,    22,    -1,    -1,
     572,    -1,    59,    -1,    -1,    62,    63,    -1,    28,    -1,
      67,    -1,     5,     6,    34,     8,    -1,    -1,    38,    -1,
      -1,   593,     6,    -1,     8,    82,    83,    84,    12,    -1,
      87,    88,    -1,    90,    -1,    -1,    -1,    30,    31,    59,
      -1,    34,    62,    63,    37,    38,    39,    67,    -1,    -1,
      34,    35,    -1,    46,    38,    -1,    40,    -1,    42,    -1,
      -1,    -1,    82,    83,    84,    -1,    59,    87,    88,    62,
      90,    -1,    -1,    -1,    67,    59,    60,    61,    62,     6,
      -1,     8,    -1,    67,    -1,    12,    -1,    -1,    -1,    82,
      83,    84,    -1,    -1,    87,    88,    -1,    -1,    82,    83,
      84,    -1,    -1,    87,    88,    -1,    -1,    34,    35,    -1,
      -1,    38,    -1,    40,    -1,    42,    -1,    -1,    -1,     6,
      -1,     8,    -1,     9,    10,    -1,     6,    -1,     8,    -1,
      -1,    -1,    59,    60,    61,    62,    -1,    24,    -1,    25,
      67,    28,    -1,    -1,    -1,    -1,    32,    34,    28,    -1,
      -1,    38,    38,    -1,    34,    82,    83,    84,    38,    -1,
      87,    88,     6,    49,     8,     9,    52,    -1,    -1,     3,
       4,    -1,    59,    -1,    -1,    62,    63,    -1,    -1,    59,
      67,    67,    62,    63,    -1,    -1,    -1,    67,    -1,     6,
      34,     8,    26,    10,    38,    82,    83,    84,    -1,    -1,
      87,    88,    82,    83,    84,    49,    -1,    87,    88,    -1,
      44,    45,    -1,    47,    48,    59,    50,    34,    62,    -1,
       6,    38,     8,    67,    58,    59,    -1,    -1,    -1,     6,
      -1,     8,    -1,    -1,    -1,    -1,    70,    -1,    82,    83,
      84,    -1,    59,    87,    88,    62,    -1,     6,    34,     8,
      67,    -1,    38,    -1,    13,    -1,     6,    34,     8,    -1,
      -1,    38,    -1,    49,    -1,    82,    83,    84,    -1,    -1,
      87,    88,    49,    59,    -1,    34,    62,    -1,    -1,    38,
      -1,    67,    59,    -1,    34,    62,    -1,    -1,    38,    -1,
      67,    -1,    -1,    -1,    -1,    -1,    82,    83,    84,    49,
      59,    87,    88,    62,    -1,    82,    83,    84,    67,    59,
      87,    88,    62,     6,    -1,     8,     9,    67,    -1,    -1,
       6,    -1,     8,    82,    83,    84,    -1,    -1,    87,    88,
      -1,    -1,    82,    83,    84,    -1,    -1,    87,    88,    -1,
      -1,    34,    -1,    -1,    -1,    38,    -1,     6,    34,     8,
      -1,    -1,    38,    -1,     6,    -1,     8,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    59,    -1,    -1,    62,
      -1,    -1,    -1,    59,    67,    34,    62,    -1,    -1,    38,
      -1,    67,    34,    -1,    -1,    -1,    38,    -1,    -1,    82,
      83,    84,    51,    -1,    87,    88,    82,    83,    84,    -1,
      59,    87,    88,    62,    -1,    -1,    -1,    59,    67,    -1,
      62,     3,     4,     5,    -1,    67,    -1,    -1,    -1,    -1,
      12,    -1,    -1,    82,    16,    84,    -1,    -1,    87,    -1,
      82,    -1,    84,    -1,    26,    87,    -1,    -1,    30,    31,
      -1,    -1,    -1,    35,    -1,    37,    -1,    39,    40,    -1,
      42,    -1,    44,    45,    46,    47,    48,    -1,    -1,    -1,
      -1,     3,     4,     5,    -1,    -1,    58,    59,    60,    61,
      12,    -1,    -1,    -1,    16,    -1,    -1,    -1,    70,    -1,
      -1,    -1,    -1,    -1,    26,    -1,    -1,    -1,    30,    31,
      -1,    -1,    -1,    35,    -1,    37,    -1,    39,    40,    -1,
      42,    -1,    44,    45,    46,    47,    48,    -1,    -1,    -1,
      -1,     3,     4,     5,    -1,    -1,    58,    59,    60,    61,
      12,     9,    10,    -1,    -1,    -1,    -1,    -1,    70,    -1,
      -1,    -1,    -1,    -1,    26,    -1,    -1,    25,    30,    31,
      -1,    -1,    -1,    35,    32,    37,    -1,    39,    40,    -1,
      42,    -1,    44,    45,    46,    47,    48,     3,     4,     5,
      -1,    49,    -1,    -1,    52,    -1,    58,    59,    60,    61,
      16,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    70,    67,
      26,    -1,    -1,    -1,    30,    31,    -1,     3,     4,     5,
      -1,    37,    -1,    39,    -1,    -1,    -1,    -1,    44,    45,
      46,    47,    48,     3,     4,    -1,    -1,    -1,    -1,     9,
      26,    -1,    58,    59,    30,    31,    -1,    -1,    -1,    -1,
      -1,    37,    -1,    39,    70,    25,    26,    -1,    44,    45,
      46,    47,    48,     3,     4,    -1,    -1,    -1,    -1,     9,
      -1,    -1,    58,    59,    44,    45,    -1,    47,    48,    -1,
       3,     4,    -1,     6,    70,    25,    26,    10,    58,    59,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,     3,     4,    -1,
      70,    -1,    -1,    26,    44,    45,    -1,    47,    48,    -1,
      16,    -1,    -1,    -1,     3,     4,    -1,     6,    58,    59,
      26,    44,    45,    -1,    47,    48,    -1,    -1,    -1,    -1,
      70,    -1,    -1,    -1,    -1,    58,    59,    26,    44,    45,
      -1,    47,    48,     3,     4,    -1,    -1,    70,     3,     4,
      10,    -1,    58,    59,    -1,    44,    45,    -1,    47,    48,
      -1,    16,    -1,    -1,    70,    -1,    26,    -1,    -1,    58,
      59,    26,     3,     4,    -1,    -1,    -1,    -1,    -1,    10,
      -1,    70,    -1,    -1,    44,    45,    -1,    47,    48,    44,
      45,    -1,    47,    48,    -1,    26,    -1,    -1,    58,    59,
       3,     4,    -1,    58,    59,    -1,    -1,    10,     3,     4,
      70,    -1,    -1,    44,    45,    70,    47,    48,    -1,    -1,
      -1,    -1,    -1,    26,     3,     4,    -1,    58,    59,    -1,
      -1,    26,     3,     4,    -1,    -1,    -1,    -1,    -1,    70,
      -1,    44,    45,    -1,    47,    48,    -1,    26,    -1,    44,
      45,    -1,    47,    48,    -1,    58,    59,    -1,    -1,     3,
       4,    -1,    -1,    58,    59,    44,    45,    70,    47,    48,
      -1,    50,    -1,    44,    45,    70,    47,    48,    -1,    58,
      59,    -1,     9,    10,    -1,    -1,    -1,    58,    59,    -1,
      -1,    70,    -1,     9,    10,    -1,    67,    -1,    25,    70,
      44,    45,    -1,    47,    48,    32,    34,    35,    36,    25,
      38,    38,    -1,    -1,    58,    59,    32,    34,    35,    36,
      -1,    38,    49,    51,    -1,    52,    70,    34,    35,    36,
      -1,    38,    -1,    49,    62,    -1,    52,    -1,    -1,    -1,
      67,    -1,    -1,    -1,    51,    62,    -1,    -1,    -1,    -1,
      -1,    67,    -1,    -1,    82,    62,    84,    -1,    -1,    87,
      88,    89,    -1,    -1,    -1,    82,    -1,    84,    -1,    -1,
      87,    88,    89,    -1,    -1,    82,    -1,    84,    -1,    -1,
      87,    88,    89
  };

  const unsigned char
  parser::yystos_[] =
  {
       0,    71,    72,    92,    93,    84,    95,   148,     0,     1,
       4,     6,     8,    11,    12,    13,    24,    27,    28,    32,
      33,    34,    35,    38,    40,    41,    42,    43,    53,    54,
      55,    56,    57,    59,    60,    61,    62,    63,    64,    67,
      69,    81,    82,    83,    85,    86,    87,    88,    90,    94,
      95,    99,   109,   110,   111,   112,   114,   119,   129,   130,
     133,   137,   141,   160,     5,   174,    95,   159,    95,    59,
      95,    99,    95,    99,    35,    59,    95,   109,    25,   138,
     139,    39,    87,    49,    90,    99,   110,   128,    10,    99,
     101,   103,   104,   105,    35,    35,    13,    25,    99,    38,
      59,    95,   109,   109,    13,    59,    95,    95,    99,    95,
      99,   100,    25,   139,    95,    25,    25,    24,    28,    63,
      90,    99,   109,    38,     3,     4,     5,    16,    26,    30,
      31,    37,    39,    44,    45,    46,    47,    48,    58,    59,
      70,   108,   129,     9,    10,    52,    67,   135,    14,    15,
      17,    18,    19,    20,    21,    22,   113,   113,    35,    99,
     108,   125,    25,    32,    25,    32,     6,     8,    34,    38,
      59,    62,    67,    82,    87,    95,    96,    25,    38,   172,
      38,    38,     5,    16,   101,   102,   131,   132,    95,    38,
       9,    25,    35,    90,    99,   110,   119,   124,   126,   133,
     134,   160,    95,    25,    90,   108,     9,   117,    49,    52,
      10,    52,   103,    51,    49,    99,   143,   146,    49,   143,
     147,    99,     9,    25,    99,   107,    95,    58,     9,    25,
     140,   140,    95,    95,    58,    38,    25,    38,    52,    67,
      36,    36,   173,    35,    24,    28,    63,    99,   109,   108,
     102,   106,    99,    99,    13,    99,    99,    99,    99,    99,
      99,    99,    99,    99,   129,   125,   110,   115,   136,   136,
      99,   111,   111,   112,   112,    49,   102,   127,    99,    25,
     139,    25,   139,    95,    96,    10,    51,    96,    97,    96,
      96,    38,     3,     4,    44,    45,    47,    48,    58,    59,
      70,   106,    35,   106,   106,    96,    99,    10,     9,    49,
      52,    38,   106,    25,   139,    49,   110,   122,   123,    90,
      99,   126,   108,   124,     9,    10,    25,    52,    35,   125,
      10,    25,    52,    25,    52,    25,    52,    31,   115,   116,
     110,    99,    52,    25,     6,    10,   142,    49,    52,    25,
     142,    49,    52,     9,    25,   139,    10,    51,    52,    58,
      82,    25,   139,    36,    58,    58,    82,   106,    95,   149,
     150,    99,   143,   143,    35,   132,    35,   108,    99,    51,
      52,    99,   125,    10,   135,   110,   110,     9,    49,    52,
      38,    51,    10,    51,    67,    97,    98,    96,    96,    96,
      96,    96,    96,    96,    96,    96,    51,    34,    35,    36,
      38,    62,    82,    87,    88,    89,    95,   151,   152,   153,
     154,   156,   157,   158,   174,    51,    51,    25,   112,   102,
     106,    51,   117,    49,    52,    99,   126,   108,    10,    25,
      52,   124,   125,   116,     9,    49,   101,   120,   121,    25,
     117,    99,   101,     9,   145,    25,   143,   145,    25,   143,
     139,    99,   140,    99,    82,    25,    99,    82,    82,    25,
      51,    10,    51,   142,   142,     4,    73,    74,    75,    76,
      77,    78,    79,    80,    95,   166,   167,   169,   170,   171,
      49,   132,    99,   102,   110,   136,     9,    10,   118,   118,
     110,   102,    98,    51,    96,    10,    51,   154,   155,   155,
      51,   153,    38,    89,   152,   151,    10,   174,   172,   172,
       9,   117,     9,    51,   122,   108,    10,    25,    52,   125,
     116,   117,    49,    52,   110,   144,   142,   142,    10,    25,
       6,    10,    25,    25,    95,    25,    50,    50,   166,   172,
      49,    52,    49,   110,   115,   135,   117,     9,    51,    49,
      50,    10,    51,   155,   152,   153,   117,    52,    49,   116,
     112,   120,    10,   145,   145,    99,    99,    99,    58,    35,
     174,   167,   169,   118,   110,    51,   154,    51,   156,    89,
     174,   117,   110,    10,    50,    82,    89,   163,   164,   165,
      25,   117,    51,   153,    99,     9,   173,   172,   173,   174,
      50,   166,     9,    52,    49,    10,    82,   163,    73,    78,
      79,    80,   168,   172,    10,    35,    74,    75,    89,   161,
     162,    10,    10,   173,    76,    77,    89,    49,    10,   166,
      10,   168
  };

  const unsigned char
  parser::yyr1_[] =
  {
       0,    91,    92,    92,    93,    93,    94,    95,    96,    96,
      96,    96,    96,    96,    96,    96,    96,    96,    96,    96,
      96,    96,    96,    96,    96,    96,    96,    96,    96,    96,
      96,    97,    97,    98,    98,    99,    99,    99,    99,    99,
      99,    99,    99,    99,    99,    99,    99,    99,    99,    99,
      99,    99,    99,    99,    99,    99,    99,    99,   100,   100,
     101,   101,   102,   102,   103,   103,   103,   103,   104,   104,
     105,   105,   106,   106,   107,   107,   108,   108,   108,   108,
     108,   108,   109,   109,   109,   109,   110,   110,   110,   110,
     110,   110,   110,   110,   110,   110,   110,   110,   110,   111,
     111,   111,   111,   112,   112,   112,   113,   113,   113,   113,
     113,   113,   114,   114,   115,   115,   116,   116,   117,   117,
     118,   118,   119,   119,   119,   119,   119,   120,   120,   121,
     121,   122,   123,   123,   124,   124,   124,   124,   125,   125,
     125,   126,   126,   126,   126,   127,   127,   128,   128,   129,
     129,   129,   129,   130,   130,   130,   130,   131,   131,   132,
     132,   133,   133,   133,   134,   135,   135,   136,   136,   136,
     137,   137,   137,   137,   138,   138,   138,   138,   138,   138,
     138,   138,   138,   138,   138,   139,   139,   139,   139,   139,
     139,   140,   140,   140,   141,   141,   141,    94,    94,    94,
      94,    94,    94,    94,    94,   142,   142,   143,   143,   144,
     144,   145,   145,   145,    94,    94,   146,   146,   147,   147,
      94,    94,    94,    94,    94,    94,    94,    94,    94,    94,
      94,    94,    94,    94,    94,    94,    94,    94,   148,    94,
      94,    94,    94,    94,   149,   149,   150,   150,    94,    94,
      94,    94,    94,   151,   151,   152,   152,   152,   152,   152,
     152,   152,   152,   152,   152,   152,   152,   152,   153,   153,
     153,   154,   154,   155,   155,   156,   156,   157,   157,   158,
     158,   159,   159,   160,   160,   161,   161,   162,   162,   163,
     163,   163,   164,   164,   165,   165,   166,   166,   166,   166,
     166,   166,   166,   166,   166,   167,   168,   168,   168,   168,
     169,   169,   170,   170,   170,   170,   171,   171,    94,   172,
     173,   174
  };

  const unsigned char
  parser::yyr2_[] =
  {
       0,     2,     2,     2,     2,     0,     3,     1,     3,     3,
       3,     3,     3,     3,     3,     3,     3,     2,     2,     2,
       3,     3,     4,     4,     5,     3,     1,     1,     1,     1,
       1,     1,     3,     1,     0,     3,     3,     3,     3,     3,
       3,     3,     3,     3,     3,     2,     2,     3,     4,     5,
       3,     1,     1,     1,     1,     1,     1,     1,     1,     3,
       1,     3,     1,     0,     2,     1,     1,     0,     2,     3,
       1,     2,     1,     3,     3,     5,     1,     1,     1,     1,
       1,     1,     1,     4,     2,     5,     1,     2,     3,     1,
       2,     3,     1,     2,     3,     3,     4,     5,     1,     4,
       4,     2,     1,     3,     3,     1,     1,     1,     1,     1,
       1,     1,     3,     3,     1,     3,     1,     0,     2,     0,
       2,     0,     1,     1,     1,     1,     1,     2,     2,     1,
       3,     2,     1,     3,     2,     3,     3,     4,     1,     2,
       0,     3,     4,     2,     1,     6,     4,     2,     4,     3,
       4,     2,     3,     3,     4,     2,     1,     4,     6,     1,
       0,     4,     5,     6,     3,     1,     1,     3,     4,     0,
       5,     5,     7,     3,     3,     3,     3,     3,     4,     4,
       5,     5,     3,     3,     0,     3,     3,     4,     5,     3,
       3,     1,     2,     2,     1,     1,     1,     2,     3,     3,
       2,     2,     3,     3,     2,     2,     0,     3,     1,     1,
       3,     2,     1,     0,     6,     6,     3,     5,     3,     5,
       4,     4,     5,     5,     5,     6,     2,     4,     3,     6,
       5,     4,     5,    10,     8,     5,     6,     3,     3,     5,
       2,     2,     3,     5,     3,     1,     0,     1,     6,     3,
       4,     4,     3,     2,     1,     3,     3,     2,     3,     4,
       5,     4,     1,     1,     1,     1,     1,     1,     3,     2,
       1,     3,     1,     1,     0,     3,     3,     4,     1,     1,
       0,     1,     4,     8,    10,     1,     3,     1,     0,     6,
       8,     8,     1,     4,     1,     0,     1,     1,     1,     1,
       1,     1,     1,     1,     1,     6,     1,     1,     1,     1,
      16,     8,     3,     3,     1,     1,     1,     0,     8,     0,
       0,     0
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
  "\".\"", "\"..\"", "\"#external\"", "\"#false\"", "\"#forget\"",
  "\">=\"", "\">\"", "\":-\"", "\"#include\"", "\"#inf\"", "\"{\"",
  "\"[\"", "\"<=\"", "\"(\"", "\"<\"", "\"#max\"", "\"#maximize\"",
  "\"#min\"", "\"#minimize\"", "\"\\\\\"", "\"*\"", "\"!=\"", "\"**\"",
  "\"?\"", "\"}\"", "\"]\"", "\")\"", "\";\"", "\"#show\"", "\"#edge\"",
  "\"#project\"", "\"#heuristic\"", "\"#showsig\"", "\"/\"", "\"-\"",
  "\"#sum\"", "\"#sum+\"", "\"#sup\"", "\"#true\"", "\"#program\"",
  "UBNOT", "UMINUS", "\"|\"", "\"#volatile\"", "\":~\"", "\"^\"",
  "\"<program>\"", "\"<define>\"", "\"any\"", "\"unary\"", "\"binary\"",
  "\"left\"", "\"right\"", "\"head\"", "\"body\"", "\"directive\"",
  "\"#theory\"", "\"<NUMBER>\"", "\"<ANONYMOUS>\"", "\"<IDENTIFIER>\"",
  "\"<PYTHON>\"", "\"<LUA>\"", "\"<STRING>\"", "\"<VARIABLE>\"",
  "\"<THEORYOP>\"", "\"not\"", "$accept", "start", "program", "statement",
  "identifier", "constterm", "consttermvec", "constargvec", "term",
  "unaryargvec", "ntermvec", "termvec", "tuple", "tuplevec_sem",
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
  const unsigned short int
  parser::yyrline_[] =
  {
       0,   332,   332,   333,   337,   338,   344,   348,   355,   356,
     357,   358,   359,   360,   361,   362,   363,   364,   365,   366,
     367,   368,   369,   370,   371,   372,   373,   374,   375,   376,
     377,   383,   384,   388,   389,   395,   396,   397,   398,   399,
     400,   401,   402,   403,   404,   405,   406,   407,   408,   409,
     410,   411,   412,   413,   414,   415,   416,   417,   423,   424,
     430,   431,   435,   436,   440,   441,   442,   443,   446,   447,
     450,   451,   454,   455,   459,   460,   470,   471,   472,   473,
     474,   475,   479,   480,   481,   482,   486,   487,   488,   489,
     490,   491,   492,   493,   494,   495,   496,   497,   498,   502,
     503,   504,   505,   509,   510,   511,   515,   516,   517,   518,
     519,   520,   524,   525,   533,   534,   538,   539,   543,   544,
     548,   549,   553,   554,   555,   556,   557,   563,   564,   568,
     569,   575,   579,   580,   586,   587,   588,   589,   593,   594,
     595,   599,   600,   601,   602,   608,   609,   613,   614,   620,
     621,   622,   623,   627,   628,   629,   630,   636,   637,   641,
     642,   646,   647,   648,   655,   662,   663,   667,   668,   669,
     674,   675,   676,   677,   684,   685,   686,   687,   688,   689,
     690,   691,   692,   693,   694,   698,   699,   700,   701,   702,
     703,   707,   708,   709,   712,   713,   714,   718,   719,   720,
     721,   722,   728,   729,   730,   736,   737,   741,   742,   746,
     747,   751,   752,   753,   757,   758,   762,   763,   767,   768,
     772,   773,   774,   775,   781,   782,   783,   784,   785,   786,
     787,   788,   794,   800,   801,   807,   808,   809,   815,   819,
     825,   826,   832,   833,   839,   840,   844,   845,   849,   850,
     856,   857,   858,   866,   867,   871,   872,   873,   874,   875,
     876,   877,   878,   879,   880,   881,   882,   883,   887,   888,
     889,   893,   894,   898,   899,   903,   904,   908,   909,   913,
     914,   918,   919,   922,   923,   929,   930,   934,   935,   939,
     940,   941,   945,   946,   950,   951,   955,   956,   957,   958,
     959,   960,   961,   962,   963,   967,   971,   972,   973,   974,
     978,   980,   984,   985,   986,   987,   991,   992,   996,  1002,
    1006,  1010
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
    *yycdebug_ << std::endl;
  }

  // Report on the debug stream that the rule \a yyrule is going to be reduced.
  void
  parser::yy_reduce_print_ (int yyrule)
  {
    unsigned int yylno = yyrline_[yyrule];
    int yynrhs = yyr2_[yyrule];
    // Print the symbols being reduced, and their result.
    *yycdebug_ << "Reducing stack by rule " << yyrule - 1
               << " (line " << yylno << "):" << std::endl;
    // The symbols being reduced.
    for (int yyi = 0; yyi < yynrhs; yyi++)
      YY_SYMBOL_PRINT ("   $" << yyi + 1 << " =",
                       yystack_[(yynrhs) - (yyi + 1)]);
  }
#endif // YYDEBUG

  // Symbol number corresponding to token number t.
  inline
  parser::token_number_type
  parser::yytranslate_ (int t)
  {
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
      85,    86,    87,    88,    89,    90
    };
    const unsigned int user_token_number_max_ = 345;
    const token_number_type undef_token_ = 2;

    if (static_cast<int>(t) <= yyeof_)
      return yyeof_;
    else if (static_cast<unsigned int> (t) <= user_token_number_max_)
      return translate_table[t];
    else
      return undef_token_;
  }

#line 24 "libgringo/src/input/nongroundgrammar.yy" // lalr1.cc:1167
} } } // Gringo::Input::NonGroundGrammar
#line 3675 "build/release/libgringo/src/input/nongroundgrammar/grammar.cc" // lalr1.cc:1167
