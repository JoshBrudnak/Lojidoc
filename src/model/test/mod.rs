use model::model::*;

#[test]
fn test_class_impl() {
    let mut class = Class::new();
    class.ch_class_name("sample".to_string());
    class.ch_package_name("com.stuff.package".to_string());
    class.ch_access("private".to_string());
    class.ch_author("jim smith".to_string());
    let class2 = class.clone();
    class.ch_package_name("new.package".to_string());

    assert_eq!("sample", class.class_name.as_str());
    assert_eq!("private", class2.access.as_str());
    assert_eq!("new.package", class.package_name.as_str());
    assert_eq!("com.stuff.package", class2.package_name.as_str());
}

#[test]
fn test_method_impl() {
    let mut method = Method::new();
    method.ch_method_name("sample".to_string());
    method.ch_privacy("private".to_string());
    let mut method2 = method.clone();
    method2.ch_method_name("sample2".to_string());

    assert_eq!("sample", method.name.as_str());
    assert_eq!("sample2", method2.name.as_str());
    assert_eq!("private", method2.privacy.as_str());
}
