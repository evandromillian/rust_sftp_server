/**
From RFC 959 (https://tools.ietf.org/html/rfc959)

Possible return codes (page 40):

    110 Restart marker reply.
        In this case, the text is exact and not left to the
        particular implementation; it must read:
            MARK yyyy = mmmm
        Where yyyy is User-process data stream marker, and mmmm
        server's equivalent marker (note the spaces between markers
        and "=").
    120 Service ready in nnn minutes.
    125 Data connection already open; transfer starting.
    150 File status okay; about to open data connection.
    200 Command okay.
    202 Command not implemented, superfluous at this site.
    211 System status, or system help reply.
    212 Directory status.
    213 File status.
    214 Help message.
        On how to use the server or the meaning of a particular
        non-standard command.  This reply is useful only to the
        human user.
    215 NAME system type.
        Where NAME is an official system name from the list in the
        Assigned Numbers document.
    220 Service ready for new user.
    221 Service closing control connection.
        Logged out if appropriate.
    225 Data connection open; no transfer in progress.
    226 Closing data connection.
        Requested file action successful (for example, file
        transfer or file abort).
    227 Entering Passive Mode (h1,h2,h3,h4,p1,p2).
    230 User logged in, proceed.
    250 Requested file action okay, completed.
    257 "PATHNAME" created.

    331 User name okay, need password.
    332 Need account for login.
    350 Requested file action pending further information.

    421 Service not available, closing control connection.
        This may be a reply to any command if the service knows it
        must shut down.
    425 Can't open data connection.
    426 Connection closed; transfer aborted.
    450 Requested file action not taken.
        File unavailable (e.g., file busy).
    451 Requested action aborted: local error in processing.
    452 Requested action not taken.
        Insufficient storage space in system.
    500 Syntax error, command unrecognized.
        This may include errors such as command line too long.
    501 Syntax error in parameters or arguments.
    502 Command not implemented.
    503 Bad sequence of commands.
    504 Command not implemented for that parameter.
    530 Not logged in.
    532 Need account for storing files.
    550 Requested action not taken.
        File unavailable (e.g., file not found, no access).
    551 Requested action aborted: page type unknown.
    552 Requested file action aborted.
        Exceeded storage allocation (for current directory or
        dataset).
    553 Requested action not taken.
        File name not allowed.

Return codes per command (page 49):

    Connection Establishment
        120
            220
        220
        421
    Login
        USER
            230
            530
            500, 501, 421
            331, 332
        PASS
            230
            202
            530
            500, 501, 503, 421
            332
        ACCT
            230
            202
            530
            500, 501, 503, 421
        CWD
            250
            500, 501, 502, 421, 530, 550
        CDUP
            200
            500, 501, 502, 421, 530, 550
        SMNT
            202, 250
            500, 501, 502, 421, 530, 550
    Logout
        REIN
            120
                220
            220
            421
            500, 502
        QUIT
            221
            500
    Transfer parameters
        PORT
            200
            500, 501, 421, 530
        PASV
            227
            500, 501, 502, 421, 530
        MODE
            200
            500, 501, 504, 421, 530
        TYPE
            200
            500, 501, 504, 421, 530
        STRU
            200
            500, 501, 504, 421, 530
        File action commands
            ALLO
                200
                202
                500, 501, 504, 421, 530
            REST
                500, 501, 502, 421, 530
                350
            STOR
                125, 150
                    (110)
                    226, 250
                    425, 426, 451, 551, 552
                532, 450, 452, 553
                500, 501, 421, 530
            STOU
                125, 150
                    (110)
                    226, 250
                    425, 426, 451, 551, 552
                532, 450, 452, 553
                500, 501, 421, 530
            RETR
                125, 150
                    (110)
                    226, 250
                    425, 426, 451
                450, 550
                500, 501, 421, 530
        LIST
            125, 150
                226, 250
                425, 426, 451
            450
            500, 501, 502, 421, 530
        NLST
            125, 150
                226, 250
                425, 426, 451
            450
            500, 501, 502, 421, 530
        APPE
            125, 150
                (110)
                226, 250
                425, 426, 451, 551, 552
            532, 450, 550, 452, 553
            500, 501, 502, 421, 530
        RNFR
            450, 550
            500, 501, 502, 421, 530
            350
        RNTO
            250
            532, 553
            500, 501, 502, 503, 421, 530
        DELE
            250
            450, 550
            500, 501, 502, 421, 530
        RMD
            250
            500, 501, 502, 421, 530, 550
        MKD
            257
            500, 501, 502, 421, 530, 550
        PWD
            257
            500, 501, 502, 421, 550
        ABOR
            225, 226
            500, 501, 502, 421
    Informational commands
        SYST
            215
            500, 501, 502, 421
        STAT
            211, 212, 213
            450
            500, 501, 502, 421, 530
        HELP
            211, 214
            500, 501, 502, 421
    Miscellaneous commands
        SITE
            200
            202
            500, 501, 530
        NOOP
            200
            500 421


*/