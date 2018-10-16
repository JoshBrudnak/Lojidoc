use grammar::grammar::Token;
use model::model::*;
use parse::parse::*;

#[test]
fn test_method_lex() {
    let j_method = "public final static void main(String[] args) {";

    let tokens = lex_contents(&j_method.to_string());

    assert_eq!(Token::LineNumber(String::from("1")), tokens[0]);
    assert_eq!(Token::Keyword(String::from("public")), tokens[1]);
    assert_eq!(Token::Keyword(String::from("final")), tokens[2]);
    assert_eq!(Token::Keyword(String::from("static")), tokens[3]);
    assert_eq!(Token::Symbol(String::from("void")), tokens[4]);
    assert_eq!(Token::Symbol(String::from("main")), tokens[5]);
    assert_eq!(Token::ParamStart, tokens[6]);
    assert_eq!(Token::Symbol(String::from("String[]")), tokens[7]);
    assert_eq!(Token::Symbol(String::from("args")), tokens[8]);
    assert_eq!(Token::ParamEnd, tokens[9]);
    assert_eq!(Token::ExpressionEnd(String::from("{")), tokens[10]);
}

#[test]
fn test_method_complex_lex() {
    let j_method = "public final Response requestData(String[] arg, Type<String> param, int anotherOne) throws IOException {";

    let tokens = lex_contents(&j_method.to_string());

    assert_eq!(Token::LineNumber(String::from("1")), tokens[0]);
    assert_eq!(Token::Keyword(String::from("public")), tokens[1]);
    assert_eq!(Token::Keyword(String::from("final")), tokens[2]);
    assert_eq!(Token::Symbol(String::from("Response")), tokens[3]);
    assert_eq!(Token::Symbol(String::from("requestData")), tokens[4]);
    assert_eq!(Token::ParamStart, tokens[5]);
    assert_eq!(Token::Symbol(String::from("String[]")), tokens[6]);
    assert_eq!(Token::Symbol(String::from("arg")), tokens[7]);
    assert_eq!(Token::Join, tokens[8]);
    assert_eq!(Token::Symbol(String::from("Type<String>")), tokens[9]);
    assert_eq!(Token::Symbol(String::from("param")), tokens[10]);
    assert_eq!(Token::Join, tokens[11]);
    assert_eq!(Token::Symbol(String::from("int")), tokens[12]);
    assert_eq!(Token::Symbol(String::from("anotherOne")), tokens[13]);
    assert_eq!(Token::ParamEnd, tokens[14]);
    assert_eq!(Token::Keyword(String::from("throws")), tokens[15]);
    assert_eq!(Token::Symbol(String::from("IOException")), tokens[16]);
    assert_eq!(Token::ExpressionEnd(String::from("{")), tokens[17]);
}

#[test]
fn test_inter_lex() {
    let j_inter = "public interface sample {";

    let tokens = lex_contents(&j_inter.to_string());

    assert_eq!(Token::LineNumber(String::from("1")), tokens[0]);
    assert_eq!(Token::Keyword(String::from("public")), tokens[1]);
    assert_eq!(Token::Keyword(String::from("interface")), tokens[2]);
    assert_eq!(Token::Symbol(String::from("sample")), tokens[3]);
    assert_eq!(Token::ExpressionEnd(String::from("{")), tokens[4]);
}

#[test]
fn test_class_lex() {
    let j_inter = "public static class sample extends Parent implements IEnumerable {";

    let tokens = lex_contents(&j_inter.to_string());

    assert_eq!(Token::LineNumber(String::from("1")), tokens[0]);
    assert_eq!(Token::Keyword(String::from("public")), tokens[1]);
    assert_eq!(Token::Keyword(String::from("static")), tokens[2]);
    assert_eq!(Token::Keyword(String::from("class")), tokens[3]);
    assert_eq!(Token::Symbol(String::from("sample")), tokens[4]);
    assert_eq!(Token::Keyword(String::from("extends")), tokens[5]);
    assert_eq!(Token::Symbol(String::from("Parent")), tokens[6]);
    assert_eq!(Token::Keyword(String::from("implements")), tokens[7]);
    assert_eq!(Token::Symbol(String::from("IEnumerable")), tokens[8]);
    assert_eq!(Token::ExpressionEnd(String::from("{")), tokens[9]);
}

#[test]
fn test_doc_lex() {
    let j_inter = "/**
                    * The sample description
                    *
                    * @param args Arguments
                    * @param str A string
                    * @return The value
                    *
                    */";

    let tokens = lex_contents(&j_inter.to_string());

    assert_eq!(Token::LineNumber(String::from("1")), tokens[0]);
    assert_eq!(Token::Symbol(String::from("/**")), tokens[1]);

    assert_eq!(Token::LineNumber(String::from("2")), tokens[2]);
    assert_eq!(Token::Symbol(String::from("*")), tokens[3]);
    assert_eq!(Token::Symbol(String::from("The")), tokens[4]);
    assert_eq!(Token::Symbol(String::from("sample")), tokens[5]);
    assert_eq!(Token::Symbol(String::from("description")), tokens[6]);

    assert_eq!(Token::LineNumber(String::from("3")), tokens[7]);
    assert_eq!(Token::Symbol(String::from("*")), tokens[8]);

    assert_eq!(Token::LineNumber(String::from("4")), tokens[9]);
    assert_eq!(Token::Symbol(String::from("*")), tokens[10]);
    assert_eq!(Token::Keyword(String::from("@param")), tokens[11]);
    assert_eq!(Token::Symbol(String::from("args")), tokens[12]);
    assert_eq!(Token::Symbol(String::from("Arguments")), tokens[13]);

    assert_eq!(Token::LineNumber(String::from("5")), tokens[14]);
    assert_eq!(Token::Symbol(String::from("*")), tokens[15]);
    assert_eq!(Token::Keyword(String::from("@param")), tokens[16]);
    assert_eq!(Token::Symbol(String::from("str")), tokens[17]);
    assert_eq!(Token::Symbol(String::from("A")), tokens[18]);
    assert_eq!(Token::Symbol(String::from("string")), tokens[19]);

    assert_eq!(Token::LineNumber(String::from("6")), tokens[20]);
    assert_eq!(Token::Symbol(String::from("*")), tokens[21]);
    assert_eq!(Token::Keyword(String::from("@return")), tokens[22]);
    assert_eq!(Token::Symbol(String::from("The")), tokens[23]);
    assert_eq!(Token::Symbol(String::from("value")), tokens[24]);
}

#[test]
fn test_param_match() {
    let mut method = Method::new();
    let mut params: Vec<Param> = Vec::new();

    method.add_param(Param {
        desc: String::new(),
        name: String::from("testParam1"),
        var_type: String::from("String"),
    });
    method.add_param(Param {
        desc: String::new(),
        name: String::from("mapOfLists"),
        var_type: String::from("Map<String, List<String>>"),
    });
    method.add_param(Param {
        desc: String::new(),
        name: String::from("ParamEdgeCase1_IHOPEThisWorks"),
        var_type: String::from("Map<List<Object>, Map<String, List<String>>>"),
    });
    params.push(Param {
        desc: String::from("A map of lists"),
        name: String::from("mapOfLists"),
        var_type: String::new(),
    });
    params.push(Param {
        desc: String::from("A sample string parameter"),
        name: String::from("testParam1"),
        var_type: String::new(),
    });
    params.push(Param {
        desc: String::from("An edge case parameter :)"),
        name: String::from("ParamEdgeCase1_IHOPEThisWorks"),
        var_type: String::new(),
    });

    let res = match_params(&method, &params);

    assert_eq!(res[0].name, String::from("testParam1"));
    assert_eq!(res[0].desc, String::from("A sample string parameter"));
    assert_eq!(res[0].var_type, String::from("String"));

    assert_eq!(res[1].name, String::from("mapOfLists"));
    assert_eq!(res[1].desc, String::from("A map of lists"));
    assert_eq!(res[1].var_type, String::from("Map<String, List<String>>"));

    assert_eq!(res[2].name, String::from("ParamEdgeCase1_IHOPEThisWorks"));
    assert_eq!(res[2].desc, String::from("An edge case parameter :)"));
    assert_eq!(
        res[2].var_type,
        String::from("Map<List<Object>, Map<String, List<String>>>")
    );
}
