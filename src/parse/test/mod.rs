use grammar::grammar::Token;
use model::model::*;
use parse::parse::*;

#[test]
fn test_method_lex() {
    let j_method = "public static void main(String[] args) {";

    let tokens = lex_contents(&j_method.to_string());

    assert_eq!(Token::Keyword(String::from("public")), tokens[0]);
    assert_eq!(Token::Keyword(String::from("static")), tokens[1]);
    assert_eq!(Token::Symbol(String::from("void")), tokens[2]);
    assert_eq!(Token::Symbol(String::from("main")), tokens[3]);
    assert_eq!(Token::ParamStart, tokens[4]);
    assert_eq!(Token::Symbol(String::from("String[]")), tokens[5]);
    assert_eq!(Token::Symbol(String::from("args")), tokens[6]);
    assert_eq!(Token::ParamEnd, tokens[7]);
    assert_eq!(Token::ExpressionEnd(String::from("{")), tokens[8]);
}

#[test]
fn test_method_complex_lex() {
    let j_method = "public final Response requestData(String[] arg, Type<String> param, int anotherOne) throws IOException {";

    let tokens = lex_contents(&j_method.to_string());

    assert_eq!(Token::Keyword(String::from("public")), tokens[0]);
    assert_eq!(Token::Keyword(String::from("final")), tokens[1]);
    assert_eq!(Token::Symbol(String::from("Response")), tokens[2]);
    assert_eq!(Token::Symbol(String::from("requestData")), tokens[3]);
    assert_eq!(Token::ParamStart, tokens[4]);
    assert_eq!(Token::Symbol(String::from("String[]")), tokens[5]);
    assert_eq!(Token::Symbol(String::from("arg")), tokens[6]);
    assert_eq!(Token::Join, tokens[7]);
    assert_eq!(Token::Symbol(String::from("Type<String>")), tokens[8]);
    assert_eq!(Token::Symbol(String::from("param")), tokens[9]);
    assert_eq!(Token::Join, tokens[10]);
    assert_eq!(Token::Symbol(String::from("int")), tokens[11]);
    assert_eq!(Token::Symbol(String::from("anotherOne")), tokens[12]);
    assert_eq!(Token::ParamEnd, tokens[13]);
    assert_eq!(Token::Keyword(String::from("throws")), tokens[14]);
    assert_eq!(Token::Symbol(String::from("IOException")), tokens[15]);
    assert_eq!(Token::ExpressionEnd(String::from("{")), tokens[16]);
}

#[test]
fn test_inter_lex() {
    let j_inter = "public interface sample {";

    let tokens = lex_contents(&j_inter.to_string());

    assert_eq!(Token::Keyword(String::from("public")), tokens[0]);
    assert_eq!(Token::Keyword(String::from("interface")), tokens[1]);
    assert_eq!(Token::Symbol(String::from("sample")), tokens[2]);
    assert_eq!(Token::ExpressionEnd(String::from("{")), tokens[3]);
}

#[test]
fn test_class_lex() {
    let j_inter = "public static class sample extends Parent implements IEnumerable {";

    let tokens = lex_contents(&j_inter.to_string());

    assert_eq!(Token::Keyword(String::from("public")), tokens[0]);
    assert_eq!(Token::Keyword(String::from("static")), tokens[1]);
    assert_eq!(Token::Keyword(String::from("class")), tokens[2]);
    assert_eq!(Token::Symbol(String::from("sample")), tokens[3]);
    assert_eq!(Token::Keyword(String::from("extends")), tokens[4]);
    assert_eq!(Token::Symbol(String::from("Parent")), tokens[5]);
    assert_eq!(Token::Keyword(String::from("implements")), tokens[6]);
    assert_eq!(Token::Symbol(String::from("IEnumerable")), tokens[7]);
    assert_eq!(Token::ExpressionEnd(String::from("{")), tokens[8]);
}

#[test]
fn test_doc_lex() {
    let j_inter = "/**\n
                    * The sample description\n
                    *\n
                    * @param args Arguments\n
                    * @param str A string\n
                    * @return The value\n
                    *\n
                    */\n";

    let tokens = lex_contents(&j_inter.to_string());

    assert_eq!(Token::Symbol(String::from("/**")), tokens[0]);
    assert_eq!(Token::Symbol(String::from("*")), tokens[1]);
    assert_eq!(Token::Symbol(String::from("The")), tokens[2]);
    assert_eq!(Token::Symbol(String::from("sample")), tokens[3]);
    assert_eq!(Token::Symbol(String::from("description")), tokens[4]);
    assert_eq!(Token::Symbol(String::from("*")), tokens[5]);
    assert_eq!(Token::Symbol(String::from("*")), tokens[6]);
    assert_eq!(Token::Keyword(String::from("@param")), tokens[7]);
    assert_eq!(Token::Symbol(String::from("args")), tokens[8]);
    assert_eq!(Token::Symbol(String::from("Arguments")), tokens[9]);
    assert_eq!(Token::Symbol(String::from("*")), tokens[10]);
    assert_eq!(Token::Keyword(String::from("@param")), tokens[11]);
    assert_eq!(Token::Symbol(String::from("str")), tokens[12]);
    assert_eq!(Token::Symbol(String::from("A")), tokens[13]);
    assert_eq!(Token::Symbol(String::from("string")), tokens[14]);
    assert_eq!(Token::Symbol(String::from("*")), tokens[15]);
    assert_eq!(Token::Keyword(String::from("@return")), tokens[16]);
    assert_eq!(Token::Symbol(String::from("The")), tokens[17]);
    assert_eq!(Token::Symbol(String::from("value")), tokens[18]);
}
