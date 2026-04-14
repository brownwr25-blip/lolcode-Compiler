grammar lc;

//Parser

lc
    : HAI (comment* head)? body KBYE 
    ;

body
    : bodyItem*
    ;

bodyItem
    : comment
    | paragraph
    | bold
    | italics
    | newline
    | link
    | varDef
    | varUse
    | text
    ;

head
    : MAEK_HEAD comment* GIMMEH_TITLE text OIC comment* MKAY
    ;

comment
    : OBTW WORD* TLDR
    ;

paragraph
    : MAEK_PARAGRAF varDef? paraItem* MKAY
    ;

paraItem
    : comment
    | bold
    | italics
    | list
    | newline
    | link
    | varUse
    | text
    ;

bold
    : GIMMEH_BOLD text OIC
    ;

italics
    : GIMMEH_ITALICS text OIC
    ;

list
    : MAEK_LIST listItem+ MKAY
    ;

listItem
    : GIMMEH_ITEM varDef? innerItemContent OIC
    ;

innerItemContent
    : (text | bold | italics | varUse)*
    ;

newline
    : NEWLINE_TAG
    ;

link
    : GIMMEH_LINX text OIC
    ;

varDef
    : IHAZ WORD ITIZ text MKAY
    ;

varUse
    : LEMMESEE WORD OIC
    ;

text
    : WORD+
    ;

//Lexer Rules

MAEK_HEAD       : '#' M A E K WS_CHARS H E A D ;
MAEK_PARAGRAF   : '#' M A E K WS_CHARS P A R A G R A F ;
MAEK_LIST       : '#' M A E K WS_CHARS L I S T ;
GIMMEH_TITLE    : '#' G I M M E H WS_CHARS T I T L E ;
GIMMEH_BOLD     : '#' G I M M E H WS_CHARS B O L D ;
GIMMEH_ITALICS  : '#' G I M M E H WS_CHARS I T A L I C S ;
GIMMEH_ITEM     : '#' G I M M E H WS_CHARS I T E M ;
GIMMEH_LINX     : '#' G I M M E H WS_CHARS L I N X ;

HAI         : '#' H A I ;
KBYE        : '#' K B Y E ;
OBTW        : '#' O B T W ;
TLDR        : '#' T L D R ;
MKAY        : '#' M K A Y ;
OIC         : '#' O I C ;
NEWLINE_TAG : '#' N E W L I N E ;
IHAZ        : '#' I H A Z ;
ITIZ        : '#' I T I Z ;
LEMMESEE    : '#' L E M M E S E E ;

//Valid Word Token
WORD : (LETTER | DIGIT | PUNCT)+ ;

//Whitespace
WS : (' ' | '\t' | '\r' | '\n')+ { $channel = HIDDEN; } ;

//Fragments

fragment WS_CHARS : (' ' | '\t')+ ;

//Case-insensitive letter fragments
fragment A : 'a'|'A';
fragment B : 'b'|'B';
fragment C : 'c'|'C';
fragment D : 'd'|'D';
fragment E : 'e'|'E';
fragment F : 'f'|'F';
fragment G : 'g'|'G';
fragment H : 'h'|'H';
fragment I : 'i'|'I';
fragment J : 'j'|'J';
fragment K : 'k'|'K';
fragment L : 'l'|'L';
fragment M : 'm'|'M';
fragment N : 'n'|'N';
fragment O : 'o'|'O';
fragment P : 'p'|'P';
fragment Q : 'q'|'Q';
fragment R : 'r'|'R';
fragment S : 's'|'S';
fragment T : 't'|'T';
fragment U : 'u'|'U';
fragment V : 'v'|'V';
fragment W : 'w'|'W';
fragment X : 'x'|'X';
fragment Y : 'y'|'Y';
fragment Z : 'z'|'Z';

fragment LETTER : 'a'..'z' | 'A'..'Z' ;
fragment DIGIT  : '0'..'9' ;
fragment PUNCT  : ',' | '.' | '"' | ':' | '?' | '!' | '%' | '/' ;