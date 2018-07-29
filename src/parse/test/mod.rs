use model::model::*;
use parse::parse::*;

#[test]
fn test_method_line_type() {
    let mut state = ParseState::new();
    state.ch_class(true);

    let j_method = "public static void main(String[] args) {";

    let line_type = determine_line_type(&j_method.to_string(), &state);
    let method = LineType::IsMethod;
    assert_eq!(method, line_type);
}

#[test]
fn test_interface_line_type() {
    let mut state = ParseState::new();
    state.ch_class(true);

    let j_method = "public interface sample {";

    let line_type = determine_line_type(&j_method.to_string(), &state);
    assert_eq!(LineType::IsInterface, line_type);
}

#[test]
fn test_class_line_type() {
    let mut state = ParseState::new();
    state.ch_class(true);

    let j_method = "public class sample {";

    let line_type = determine_line_type(&j_method.to_string(), &state);
    assert_eq!(LineType::IsClass, line_type);
}

#[test]
fn test_jdoc_line_type() {
    let mut state = ParseState::new();
    state.ch_class(true);

    let j_start_doc = "/**";

    let start_line_type = determine_line_type(&j_start_doc.to_string(), &state);
    state.ch_doc(true);
    let j_desc = "Service class that does stuff";
    let jdoc_code = "public class sample {";
    let j_end_doc = "*/";

    let desc_line_type = determine_line_type(&j_desc.to_string(), &state);
    let code_line_type = determine_line_type(&jdoc_code.to_string(), &state);
    let end_line_type = determine_line_type(&j_end_doc.to_string(), &state);

    assert_eq!(LineType::IsStartdoc, start_line_type);
    assert_eq!(LineType::IsOther, desc_line_type);
    assert_eq!(LineType::IsOther, code_line_type);
    assert_eq!(LineType::IsEnddoc, end_line_type);
}

#[test]
fn test_package_line_type() {
    let mut state = ParseState::new();

    let j_line = "package main.java.service;";

    let line_type = determine_line_type(&j_line.to_string(), &state);
    assert_eq!(LineType::IsPackage, line_type);
}

#[test]
fn test_import_line_type() {
    let mut state = ParseState::new();

    let j_line = "import java.utils.List;";

    let line_type = determine_line_type(&j_line.to_string(), &state);
    assert_eq!(LineType::IsImport, line_type);
}

#[test]
fn test_variable_line_type() {
    let mut state = ParseState::new();
    state.ch_class(true);
    let j_var = "public String dataSource;";
    let j_var_init = "public final Bean dataSource = new DataSource(\"data\");";

    let reg_line_type = determine_line_type(&j_var.to_string(), &state);
    let init_line_type = determine_line_type(&j_var_init.to_string(), &state);
    assert_eq!(LineType::IsVariable, reg_line_type);
    assert_eq!(LineType::IsVariable, init_line_type);
}

#[test]
fn test_comment_line_type() {
    let mut state = ParseState::new();
    let j_line = "// public String dataSource;";

    let line_type = determine_line_type(&j_line.to_string(), &state);
    assert_eq!(LineType::IsComment, line_type);
}

#[test]
fn test_other_line_type() {
    let mut state = ParseState::new();
    state.ch_class(true);
    state.ch_method(true);
    let j_line = "for(int i = 0; i < 5; i++)";

    let line_type = determine_line_type(&j_line.to_string(), &state);
    assert_eq!(LineType::IsOther, line_type);
}

#[test]
fn test_handle_class() {
    let mut class = Class::new();
    let mut class2 = Class::new();
    let j_class = "public class sample extends parentSample {";
    let j_line_impl = "protected final class Sample2 implements C,D,Z {";

    let new_reg_class = handle_class(class, &j_class.to_string());
    let new_impl_class = handle_class(class2, &j_line_impl.to_string());

    assert_eq!("sample", new_reg_class.class_name);
    assert_eq!("public", new_reg_class.access);
    assert_eq!("parentSample", new_reg_class.parent);
    assert_eq!("Sample2", new_impl_class.class_name);
    assert_eq!("protected", new_impl_class.access);
}

#[test]
fn test_handle_method() {
    let j_method1 = "public void publicTest(String[] stuff) {".to_string();
    let j_method2 = "private final String voidMethod() {".to_string();
    let j_method3 = "abstract final String abstractfinalMethod (){".to_string();
    let j_method4 = "protected final List<String> A() throws Exception {".to_string();

    let new_method1 = handle_method(&j_method1, 50).unwrap();
    let new_method2 = handle_method(&j_method2, 69).unwrap();
    let new_method3 = handle_method(&j_method3, 98).unwrap();
    let new_method4 = handle_method(&j_method4, 182).unwrap();

    assert_eq!("publicTest", new_method1.name);
    assert_eq!("public", new_method1.privacy);
    assert_eq!("voidMethod", new_method2.name);
    assert_eq!("abstractfinalMethod", new_method3.name);
    assert_eq!("A", new_method4.name);
}
