//! The language and the errors of the parser: for each input, the status, the messages and
//! the text written back, as the reader gives them.
use super::support::read_text;
use martensite_tkxsbase::interface::InterfaceModelEntities;

/// One case: its name, its input and the expected stream.
const CASES: &[(&str, &[u8], &[u8])] = &[
    (
        "bad_after_endstep",
        b"ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1=A(1);\nENDSEC;\nENDSTEP;\nmore;\n",
        b"status 1\nentities 0\nfails 0\nwarnings 0\n",
    ),
    (
        "bad_empty",
        b"",
        b"status 1\nentities 0\nfails 0\nwarnings 0\n",
    ),
    (
        "bad_empty_header",
        b"ISO-10303-21;\nHEADER;\nENDSEC;\nDATA;\n#1=A(1);\n@\nENDSEC;\nEND-ISO-10303-21;\n",
        b"status 0\nentities 1\nfails 1\nUndefined Parsing: Line 7: Incorrect syntax: unexpected QUID, expecting ENDSEC or ENTITY\nwarnings 0\nISO-10303-21;\nHEADER;\nENDSEC;\nDATA;\n  /*  GLOBAL FAIL MESSAGES,  recorded at Read time :Undefined Parsing: L\nine 7: Incorrect syntax: unexpected QUID, expecting ENDSEC or ENTITY  */\n#1 = A(1);\nENDSEC;\nEND-ISO-10303-21;\n",
    ),
    (
        "bad_extra_close",
        b"ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1=A(1));\n#2=B(2);\nENDSEC;\nEND-ISO-10303-21;\n",
        b"status 0\nentities 2\nfails 1\nUndefined Parsing: Line 9: Incorrect syntax: unexpected ')', expecting ';'\nwarnings 0\nISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n  /*  GLOBAL FAIL MESSAGES,  recorded at Read time :Undefined Parsing: L\nine 9: Incorrect syntax: unexpected ')', expecting ';'  */\n#1 = A(1);\n#2 = B(2);\nENDSEC;\nEND-ISO-10303-21;\n",
    ),
    (
        "bad_garbage_end",
        b"ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1=A(1);\nENDSEC;\nEND-ISO-10303-21;\ngarbage",
        b"status 0\nentities 1\nfails 0\nwarnings 0\nISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1 = A(1);\nENDSEC;\nEND-ISO-10303-21;\n",
    ),
    (
        "bad_header_entity",
        b"ISO-10303-21;\nHEADER;\nFILE_SCHEMA(('s')\n;\nFOO(1);\nENDSEC;\nDATA;\n#1=A(1);\nENDSEC;\nEND-ISO-10303-21;\n",
        b"status 0\nentities 1\nfails 2\nUndefined Parsing: Line 5: Incorrect syntax: unexpected ';', expecting ')' or ','\nCount of Parameters is not 1 for file_schema has not 1 parameter(s)\nwarnings 0\nISO-10303-21;\nHEADER;\nFILE_SCHEMA(());\nENDSEC;\nDATA;\n  /*  GLOBAL FAIL MESSAGES,  recorded at Read time :Undefined Parsing: L\nine 5: Incorrect syntax: unexpected ';', expecting ')' or ','\nCount of Parameters is not 1 for file_schema has not 1 parameter(s)  */\n#1 = A(1);\nENDSEC;\nEND-ISO-10303-21;\n",
    ),
    (
        "bad_many_expected",
        b"ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1=;\n#2=A(;\n#3=(;\n#4=(A);\nENDSEC;\nEND-ISO-10303-21;\n",
        b"status 0\nentities 1\nfails 2\nUndefined Parsing: Line 9: Incorrect syntax: unexpected ';', expecting SCOPE or TYPE or '('\nUndefined Parsing: Line 10: Incorrect syntax: unexpected ';'\nwarnings 0\nISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n  /*  GLOBAL FAIL MESSAGES,  recorded at Read time :Undefined Parsing: L\nine 9: Incorrect syntax: unexpected ';', expecting SCOPE or TYPE or '('\nUndefined Parsing: Line 10: Incorrect syntax: unexpected ';'  */\n#1 = A(A#3#4A);\nENDSEC;\nEND-ISO-10303-21;\n",
    ),
    (
        "bad_missing_equal",
        b"ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1 A(1);\n#2=B(2);\nENDSEC;\nEND-ISO-10303-21;\n",
        b"status 0\nentities 1\nfails 1\nUndefined Parsing: Line 9: Incorrect syntax: unexpected IDENT, expecting ENTITY\nwarnings 0\nISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n  /*  GLOBAL FAIL MESSAGES,  recorded at Read time :Undefined Parsing: L\nine 9: Incorrect syntax: unexpected IDENT, expecting ENTITY  */\n#1 = B(2);\nENDSEC;\nEND-ISO-10303-21;\n",
    ),
    (
        "bad_missing_semicolon",
        b"ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1=A(1)\n#2=B(2);\nENDSEC;\nEND-ISO-10303-21;\n",
        b"status 0\nentities 2\nfails 1\nUndefined Parsing: Line 10: Incorrect syntax: unexpected ENTITY, expecting ';'\nwarnings 0\nISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n  /*  GLOBAL FAIL MESSAGES,  recorded at Read time :Undefined Parsing: L\nine 10: Incorrect syntax: unexpected ENTITY, expecting ';'  */\n#1 = A(1);\n#2 = B(2);\nENDSEC;\nEND-ISO-10303-21;\n",
    ),
    (
        "bad_no_header",
        b"ISO-10303-21;\nDATA;\n",
        b"status 1\nentities 0\nfails 0\nwarnings 0\n",
    ),
    (
        "bad_no_iso",
        b"HEADER;\n",
        b"status 1\nentities 0\nfails 0\nwarnings 0\n",
    ),
    (
        "bad_no_section_end",
        b"ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1=A(1);\n",
        b"status 1\nentities 0\nfails 0\nwarnings 0\n",
    ),
    (
        "bad_stray_token",
        b"ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1=A(1);\n= ;\n#2=B(2);\nENDSEC;\nEND-ISO-10303-21;\n",
        b"status 0\nentities 2\nfails 1\nUndefined Parsing: Line 10: Incorrect syntax: unexpected '=', expecting ENDSEC or ENTITY\nwarnings 0\nISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n  /*  GLOBAL FAIL MESSAGES,  recorded at Read time :Undefined Parsing: L\nine 10: Incorrect syntax: unexpected '=', expecting ENDSEC or ENTITY  */\n#1 = A(1);\n#2 = B(2);\nENDSEC;\nEND-ISO-10303-21;\n",
    ),
    (
        "bad_type_in_arg",
        b"ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1=A(1E5,1.E,\"1g\");\nENDSEC;\nEND-ISO-10303-21;\n",
        b"status 0\nentities 1\nfails 1\nUndefined Parsing: Line 9: Incorrect syntax: unexpected ',', expecting '('\nwarnings 0\nISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n  /*  GLOBAL FAIL MESSAGES,  recorded at Read time :Undefined Parsing: L\nine 9: Incorrect syntax: unexpected ',', expecting '('  */\n#1 = A(1E5,1E51.E,\"1g\");\nENDSEC;\nEND-ISO-10303-21;\n",
    ),
    (
        "bad_unclosed_list",
        b"ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1=A(1,(2,3);\n#2=B(2);\nENDSEC;\nEND-ISO-10303-21;\n",
        b"status 0\nentities 1\nfails 1\nUndefined Parsing: Line 9: Incorrect syntax: unexpected ';', expecting ')' or ','\nwarnings 0\nISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n  /*  GLOBAL FAIL MESSAGES,  recorded at Read time :Undefined Parsing: L\nine 9: Incorrect syntax: unexpected ';', expecting ')' or ','  */\n#1 = A(1,/* (SUB) */(2,3),3#2B2);\nENDSEC;\nEND-ISO-10303-21;\n",
    ),
    (
        "bad_unterminated_comment",
        b"ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1=A(1);\n/* never closed\nENDSEC;\nEND-ISO-10303-21;\n",
        b"status 1\nentities 0\nfails 0\nwarnings 0\n",
    ),
    (
        "bad_unterminated_text",
        b"ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1=A('never closed);\nENDSEC;\nEND-ISO-10303-21;\n",
        b"status 1\nentities 0\nfails 0\nwarnings 0\n",
    ),
    (
        "complex_order",
        b"ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1=(B(1)A(2));\nENDSEC;\nEND-ISO-10303-21;\n",
        b"status 0\nentities 1\nfails 0\nwarnings 1\nComplex Type incorrect : B / A ... \nISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1 = ( B(1) A(2) );\nENDSEC;\nEND-ISO-10303-21;\n",
    ),
    (
        "duplicate_ident",
        b"ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1=A(1);\n#1=B(2);\n#2=C(#1);\nENDSEC;\nEND-ISO-10303-21;\n",
        b"status 0\nentities 3\nfails 1\nIdent defined SEVERAL TIMES : #1\nwarnings 0\nISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n  /*  GLOBAL FAIL MESSAGES,  recorded at Read time :\nIdent defined SEVERAL TIMES : #1  */\n#1 = A(1);\n#2 = B(2);\n#3 = C(#2);\nENDSEC;\nEND-ISO-10303-21;\n",
    ),
    (
        "ok_bang_nulls",
        b"ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1=!USER(1);\n\x00\x00#2=A(1);\r\nENDSEC;\nEND-ISO-10303-21;\n",
        b"status 0\nentities 2\nfails 0\nwarnings 0\nISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1 = !USER(1);\n#2 = A(1);\nENDSEC;\nEND-ISO-10303-21;\n",
    ),
    (
        "ok_comments",
        b"ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n/* one */ #1 /* two\nlines ** */ = A( /* *** */ 1);\nENDSEC;\nEND-ISO-10303-21;\n",
        b"status 0\nentities 0\nfails 1\nUndefined Parsing: Line 9: Incorrect syntax: unexpected IDENT, expecting ENTITY\nwarnings 0\nISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n  /*  GLOBAL FAIL MESSAGES,  recorded at Read time :Undefined Parsing: L\nine 9: Incorrect syntax: unexpected IDENT, expecting ENTITY  */\nENDSEC;\nEND-ISO-10303-21;\n",
    ),
    (
        "ok_complex",
        b"ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1=(A(1)B(2)C(3));\n#2=(Z(1)A(2));\nENDSEC;\nEND-ISO-10303-21;\n",
        b"status 0\nentities 2\nfails 0\nwarnings 1\nComplex Type incorrect : Z / A ... \nISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1 = ( A(1) B(2) C(3) );\n#2 = ( Z(1) A(2) );\nENDSEC;\nEND-ISO-10303-21;\n",
    ),
    (
        "ok_directives",
        b"ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1=A('\\X2\\03C0\\X0\\','\\X\\E9','\\S\\D','\\PB\\\\S\\D','\\X4\\0001F600\\X0\\','a\\\\b','\\N\\x\\T\\y','\\X2\\03C\\X0\\','\\PZ\\');\nENDSEC;\nEND-ISO-10303-21;\n",
        b"status 0\nentities 1\nfails 0\nwarnings 0\nISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1 = A('\\X2\\03C0\\X0\\','\\X\\E9','\\S\\D','\\PB\\\\S\\D','\\X4\\0001F600\\X0\\',\n  'a\\\\b','\\N\\x\\T\\y','\\X2\\03C\\X0\\','\\PZ\\');\nENDSEC;\nEND-ISO-10303-21;\n",
    ),
    (
        "ok_export",
        b"ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1=A(#2) &SCOPE\n#2=B(1);\nENDSCOPE /#2/ C(#2);\nENDSEC;\nEND-ISO-10303-21;\n",
        b"status 0\nentities 2\nfails 2\nUndefined Parsing: Line 9: Incorrect syntax: unexpected SCOPE, expecting ';'\nUndefined Parsing: Line 11: Incorrect syntax: unexpected ENDSCOPE, expecting ENDSEC or ENTITY\nwarnings 0\nISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n  /*  GLOBAL FAIL MESSAGES,  recorded at Read time :Undefined Parsing: L\nine 9: Incorrect syntax: unexpected SCOPE, expecting ';'Undefined Parsin\ng: Line 11: Incorrect syntax: unexpected ENDSCOPE, expecting ENDSEC or E\nNTITY  */\n#1 = A(#2);\n#2 = B(1);\nENDSEC;\nEND-ISO-10303-21;\n",
    ),
    (
        "ok_lowercase",
        b"iso-10303-21;\nheader;\nfile_schema(('s'));\nendsec;\ndata;\n#1=a(1.e3,1.5e-3,.t.);\nendsec;\nend-iso-10303-21;\n",
        b"status 0\nentities 1\nfails 0\nwarnings 1\nHeader Entity not Recognized, StepType: file_schema\nISO-10303-21;\nHEADER;\nfile_schema(/* (SUB) */('s'));\nENDSEC;\nDATA;\n#1 = a(1.e3,1.5e-3,.t.);\nENDSEC;\nEND-ISO-10303-21;\n",
    ),
    (
        "ok_minimal",
        b"ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1=A(1,2.5,'x',.T.,$,*,#2,(1,2),B(3));\n#2=C();\nENDSEC;\nEND-ISO-10303-21;\n",
        b"status 0\nentities 2\nfails 0\nwarnings 0\nISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1 = A(1,2.5,'x',.T.,$,*,#2,/* (SUB) */(1,2),B(3));\n#2 = C();\nENDSEC;\nEND-ISO-10303-21;\n",
    ),
    (
        "ok_misc_arg",
        b"ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1=A(@,%x%,1 2);\n#2=A(1,@ @,2);\nENDSEC;\nEND-ISO-10303-21;\n",
        b"status 0\nentities 2\nfails 2\nUndefined Parsing: Line 9: Incorrect syntax: unexpected TYPE, expecting ')' or ','\nUndefined Parsing: Line 10: Incorrect syntax: unexpected QUID, expecting ')' or ','\nwarnings 0\nISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n  /*  GLOBAL FAIL MESSAGES,  recorded at Read time :Undefined Parsing: L\nine 9: Incorrect syntax: unexpected TYPE, expecting ')' or ','Undefined \nParsing: Line 10: Incorrect syntax: unexpected QUID, expecting ')' or ',\n'  */\n#1 = A(@,%x%,1,2);\n#2 = A(1,@@,2);\nENDSEC;\nEND-ISO-10303-21;\n",
    ),
    (
        "ok_numbers",
        b"ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1=A(+1,-1,+1.5,-.5,1.,.5,1..2,1E5,12E3,1.E,\"AB\",\"1g\",-,+,.,..A.,A.,_1.);\nENDSEC;\nEND-ISO-10303-21;\n",
        b"status 0\nentities 1\nfails 3\nUndefined Parsing: Line 9: Incorrect syntax: unexpected ',', expecting '('\nUndefined Parsing: Line 9: Incorrect syntax: unexpected TYPE, expecting ')' or ','\nUndefined Parsing: Line 9: Incorrect syntax: unexpected TYPE, expecting ')' or ','\nwarnings 0\nISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n  /*  GLOBAL FAIL MESSAGES,  recorded at Read time :Undefined Parsing: L\nine 9: Incorrect syntax: unexpected ',', expecting '('Undefined Parsing:\n Line 9: Incorrect syntax: unexpected TYPE, expecting ')' or ','Undefine\nd Parsing: Line 9: Incorrect syntax: unexpected TYPE, expecting ')' or '\n,'  */\n#1 = A(+1,-1,+1.5,-.5,1.,.5,1..2,1E5,1E512E3,1.,E,\"AB\",\"1g\",-,+,.,..A.,\n  A.,_1.);\nENDSEC;\nEND-ISO-10303-21;\n",
    ),
    (
        "ok_numbers_plain",
        b"ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1=A(+1,-1,+1.5,-.5,1.,.5,1..2,12E3,\"AB\",-,+,.,..A.,A.,_1.);\nENDSEC;\nEND-ISO-10303-21;\n",
        b"status 0\nentities 1\nfails 0\nwarnings 0\nISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1 = A(+1,-1,+1.5,-.5,1.,.5,1..2,12E3,\"AB\",-,+,.,..A.,A.,_1.);\nENDSEC;\nEND-ISO-10303-21;\n",
    ),
    (
        "ok_scope",
        b"ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1=A(#2) &SCOPE\n#2=B(1);\nENDSCOPE C(#2);\n#3=D(#1);\nENDSEC;\nEND-ISO-10303-21;\n",
        b"status 0\nentities 3\nfails 2\nUndefined Parsing: Line 9: Incorrect syntax: unexpected SCOPE, expecting ';'\nUndefined Parsing: Line 11: Incorrect syntax: unexpected ENDSCOPE, expecting ENDSEC or ENTITY\nwarnings 0\nISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n  /*  GLOBAL FAIL MESSAGES,  recorded at Read time :Undefined Parsing: L\nine 9: Incorrect syntax: unexpected SCOPE, expecting ';'Undefined Parsin\ng: Line 11: Incorrect syntax: unexpected ENDSCOPE, expecting ENDSEC or E\nNTITY  */\n#1 = A(#2);\n#2 = B(1);\n#3 = D(#1);\nENDSEC;\nEND-ISO-10303-21;\n",
    ),
    (
        "ok_scope_dup",
        b"ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1=A(#5) &SCOPE\n#5=B(1);\nENDSCOPE C(#5);\n#2=A(#5) &SCOPE\n#5=B(2);\nENDSCOPE C(#5);\n#3=D(#1,#2);\nENDSEC;\nEND-ISO-10303-21;\n",
        b"status 0\nentities 5\nfails 5\nUndefined Parsing: Line 9: Incorrect syntax: unexpected SCOPE, expecting ';'\nUndefined Parsing: Line 11: Incorrect syntax: unexpected ENDSCOPE, expecting ENDSEC or ENTITY\nUndefined Parsing: Line 12: Incorrect syntax: unexpected SCOPE, expecting ';'\nUndefined Parsing: Line 14: Incorrect syntax: unexpected ENDSCOPE, expecting ENDSEC or ENTITY\nIdent defined SEVERAL TIMES : #5\nwarnings 0\nISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n  /*  GLOBAL FAIL MESSAGES,  recorded at Read time :Undefined Parsing: L\nine 9: Incorrect syntax: unexpected SCOPE, expecting ';'Undefined Parsin\ng: Line 11: Incorrect syntax: unexpected ENDSCOPE, expecting ENDSEC or E\nNTITYUndefined Parsing: Line 12: Incorrect syntax: unexpected SCOPE, exp\necting ';'Undefined Parsing: Line 14: Incorrect syntax: unexpected ENDSC\nOPE, expecting ENDSEC or ENTITYIdent defined SEVERAL TIMES : #5  */\n#1 = A(#2);\n#2 = B(1);\n#3 = A(#2);\n#4 = B(2);\n#5 = D(#1,#3);\nENDSEC;\nEND-ISO-10303-21;\n",
    ),
    (
        "ok_scope_dup_form",
        b"ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1= &SCOPE\n#5=B(1);\nENDSCOPE A(#5);\n#2= &SCOPE\n#5=B(2);\nENDSCOPE A(#5);\n#3=D(#1,#2);\nENDSEC;\nEND-ISO-10303-21;\n",
        b"status 0\nentities 5\nfails 0\nwarnings 0\nISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1 = B(1);\n#2 = A(#1);\n#3 = B(2);\n#4 = A(#3);\n#5 = D(#2,#4);\nENDSEC;\nEND-ISO-10303-21;\n",
    ),
    (
        "ok_scope_empty",
        b"ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1= &SCOPE\nENDSCOPE A(1);\n#3=D(#1);\nENDSEC;\nEND-ISO-10303-21;\n",
        b"status 0\nentities 2\nfails 0\nwarnings 0\nISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1 = A(1);\n#2 = D(#1);\nENDSEC;\nEND-ISO-10303-21;\n",
    ),
    (
        "ok_scope_export",
        b"ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1= &SCOPE\n#2=B(1);\nENDSCOPE /#2/ A(#2);\n#3=D(#1,#2);\nENDSEC;\nEND-ISO-10303-21;\n",
        b"status 0\nentities 3\nfails 0\nwarnings 0\nISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1 = B(1);\n#2 = A(#1);\n#3 = D(#2,#1);\nENDSEC;\nEND-ISO-10303-21;\n",
    ),
    (
        "ok_scope_form",
        b"ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1= &SCOPE\n#2=B(1);\nENDSCOPE A(#2);\n#3=D(#1);\nENDSEC;\nEND-ISO-10303-21;\n",
        b"status 0\nentities 3\nfails 0\nwarnings 0\nISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1 = B(1);\n#2 = A(#1);\n#3 = D(#2);\nENDSEC;\nEND-ISO-10303-21;\n",
    ),
    (
        "ok_scope_nested",
        b"ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1= &SCOPE\n#2= &SCOPE\n#5=B(1);\nENDSCOPE C(#5);\nENDSCOPE A(#2);\n#3=D(#1);\nENDSEC;\nEND-ISO-10303-21;\n",
        b"status 0\nentities 4\nfails 0\nwarnings 0\nISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1 = B(1);\n#2 = C(#1);\n#3 = A(#2);\n#4 = D(#3);\nENDSEC;\nEND-ISO-10303-21;\n",
    ),
    (
        "ok_step_keywords",
        b"STEP;\nHEADER;\nFILE_SCHEMA(('s'));\nENDSEC;\nDATA;\n#1=A(1);\nENDSEC;\nENDSTEP; trailing words here\n",
        b"status 0\nentities 1\nfails 0\nwarnings 0\nISO-10303-21;\nHEADER;\nFILE_SCHEMA(('s'));\nENDSEC;\nDATA;\n#1 = A(1);\nENDSEC;\nEND-ISO-10303-21;\n",
    ),
    (
        "ok_text_quotes",
        b"ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1=A('it''s',' ) , ','a'b',\n'multi\nline','',' ''');\nENDSEC;\nEND-ISO-10303-21;\n",
        b"status 0\nentities 1\nfails 0\nwarnings 0\nISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1 = A('it''s',' ) , ','a'b','multiline','',' ''');\nENDSEC;\nEND-ISO-10303-21;\n",
    ),
    (
        "unresolved",
        b"ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1=A(#99);\n#2=B(#1,#98);\nENDSEC;\nEND-ISO-10303-21;\n",
        b"status 0\nentities 2\nfails 2\nUnresolved Reference, Ent.Id.#1 Param.n0 1 (Id.#99)\nUnresolved Reference, Ent.Id.#2 Param.n0 2 (Id.#98)\nwarnings 0\nISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\nFILE_NAME('n','t',('a'),('o'),'p','s','u');\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n  /*  GLOBAL FAIL MESSAGES,  recorded at Read time :\nUnresolved Reference, Ent.Id.#1 Param.n0 1 (Id.#99)\nUnresolved Reference, Ent.Id.#2 Param.n0 2 (Id.#98)  */\n#1 = A(#99);\n  /*  On Entity above, Fail Messages recorded at Read time :\nA reference to another entity is unresolved\nA reference to another entity is unresolved  */\n#2 = B(#1,#98);\n  /*  On Entity above, Fail Messages recorded at Read time :\nA reference to another entity is unresolved\nA reference to another entity is unresolved  */\nENDSEC;\nEND-ISO-10303-21;\n",
    ),
];

#[test]
fn inputs_give_the_expected_streams() {
    for (name, input, expected) in CASES {
        let read = read_text(input);
        assert_eq!(
            String::from_utf8_lossy(&read.stream()),
            String::from_utf8_lossy(expected),
            "{name}"
        );
    }
}

/// A file with one string of `length` characters.
fn long_text(length: usize) -> Vec<u8> {
    let mut text = b"ISO-10303-21;\nHEADER;\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n#1=A('".to_vec();
    text.extend(std::iter::repeat_n(b'x', length));
    text.extend_from_slice(b"');\nENDSEC;\nEND-ISO-10303-21;\n");
    text
}

/// A token that fills the 16K scanner buffer fails the whole read. Here it reads: a
/// deliberate difference.
#[test]
fn a_string_above_16k_reads_and_writes_back() {
    assert_eq!(read_text(&long_text(8000)).status, 0);
    let read = read_text(&long_text(20000));
    assert_eq!(read.status, 0);
    assert_eq!(read.fails(), Vec::<String>::new());
    assert_eq!(read.model.get().expect("a model").nb_entities(), 1);
    let written = read.written();
    let again = read_text(&written);
    assert_eq!(again.status, 0);
    assert_eq!(again.fails(), Vec::<String>::new());
    let kept = |bytes: &[u8]| bytes.iter().filter(|byte| **byte == b'x').count();
    assert_eq!(kept(&written), 20000);
    assert_eq!(kept(&again.written()), 20000);
}

/// As the string above, for one line of a comment.
#[test]
fn a_comment_line_above_16k_is_skipped() {
    let mut text = b"ISO-10303-21;\nHEADER;\nFILE_SCHEMA(('S'));\nENDSEC;\nDATA;\n/*".to_vec();
    text.extend(std::iter::repeat_n(b'c', 20000));
    text.extend_from_slice(b"*/\n#1=A('a');\nENDSEC;\nEND-ISO-10303-21;\n");
    let read = read_text(&text);
    assert_eq!(read.status, 0);
    assert_eq!(read.fails(), Vec::<String>::new());
    assert_eq!(read.model.get().expect("a model").nb_entities(), 1);
}
