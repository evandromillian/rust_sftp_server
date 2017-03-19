/**
From RFC 959 (https://tools.ietf.org/html/rfc959)

FTP commands:

    USER <SP> <username> <CRLF>
    PASS <SP> <password> <CRLF>
    ACCT <SP> <account-information> <CRLF>
    CWD  <SP> <pathname> <CRLF>
    CDUP <CRLF>
    SMNT <SP> <pathname> <CRLF>
    QUIT <CRLF>
    REIN <CRLF>
    PORT <SP> <host-port> <CRLF>
    PASV <CRLF>
    TYPE <SP> <type-code> <CRLF>
    STRU <SP> <structure-code> <CRLF>
    MODE <SP> <mode-code> <CRLF>
    RETR <SP> <pathname> <CRLF>
    STOR <SP> <pathname> <CRLF>
    STOU <CRLF>
    APPE <SP> <pathname> <CRLF>
    ALLO <SP> <decimal-integer>
        [<SP> R <SP> <decimal-integer>] <CRLF>
    REST <SP> <marker> <CRLF>
    RNFR <SP> <pathname> <CRLF>
    RNTO <SP> <pathname> <CRLF>
    ABOR <CRLF>
    DELE <SP> <pathname> <CRLF>
    RMD  <SP> <pathname> <CRLF>
    MKD  <SP> <pathname> <CRLF>
    PWD  <CRLF>
    LIST [<SP> <pathname>] <CRLF>
    NLST [<SP> <pathname>] <CRLF>
    SITE <SP> <string> <CRLF>
    SYST <CRLF>
    STAT [<SP> <pathname>] <CRLF>
    HELP [<SP> <string>] <CRLF>
    NOOP <CRLF>

The syntax of the above argument fields (using BNF notation where applicable) is:

    <username> ::= <string>
    <password> ::= <string>
    <account-information> ::= <string>
    <string> ::= <char> | <char><string>
    <char> ::= any of the 128 ASCII characters except <CR> and
    <LF>
    <marker> ::= <pr-string>
    <pr-string> ::= <pr-char> | <pr-char><pr-string>
    <pr-char> ::= printable characters, any
                    ASCII code 33 through 126
    <byte-size> ::= <number>
    <host-port> ::= <host-number>,<port-number>
    <host-number> ::= <number>,<number>,<number>,<number>
    <port-number> ::= <number>,<number>
    <number> ::= any decimal integer 1 through 255
    <form-code> ::= N | T | C
    <type-code> ::= A [<sp> <form-code>]
                    | E [<sp> <form-code>]
                    | I
                    | L <sp> <byte-size>
    <structure-code> ::= F | R | P
    <mode-code> ::= S | B | C
    <pathname> ::= <string>
    <decimal-integer> ::= any decimal integer


*/