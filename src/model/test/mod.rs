use model::model::*;

#[test]
fn test_class_impl() {
    let mut object = Object::new();
    object.ch_name(String::from("sample"));
    object.ch_package_name("com.stuff.package".to_string());
    object.ch_access("private".to_string());
    object.ch_author("jim smith".to_string());
    let class = object.to_class();

    assert_eq!("sample", object.name.as_str());
    assert_eq!("private", object.access.as_str());
    assert_eq!("com.stuff.package", object.package_name.as_str());
    assert_eq!("com.stuff.package", class.package_name.as_str());
    assert_eq!("sample", class.name.as_str());
    assert_eq!("private", class.access.as_str());
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
