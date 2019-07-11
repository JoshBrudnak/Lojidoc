#[derive(Debug)]
pub enum RequestMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH
}

#[derive(Debug)]
pub struct SpringClass {
    pub bean_type: String,
    pub mapping: RequestMap,
}

#[derive(Debug)]
pub struct SpringMember {
    pub autowire: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug)]
pub struct SpringMethod {
    pub mapping: RequestMap,
    pub request_params: Vec<String>,
    pub request_body: Vec<String>,
}

#[derive(Debug)]
pub struct RequestMap {
    pub url: Vec<String>,
    pub method: RequestMethod,
    pub headers: Vec<String>,
    pub produces: String,
}

impl SpringClass {
    pub fn new() -> SpringClass {
        SpringClass {
            bean_type: String::new(),
            mapping: RequestMap {
                url: Vec::new(),
                method: RequestMethod::GET,
                headers: Vec::new(),
                produces: String::new()
            },
        }
    }
    pub fn clone(&mut self) -> SpringClass {
        SpringClass {
            bean_type: self.bean_type.clone(),
            mapping: self.mapping.clone()
        }
    }
}

impl SpringMethod {
    pub fn new() -> SpringMethod {
        SpringMethod {
            mapping: RequestMap {
                url: Vec::new(),
                method: RequestMethod::GET,
                headers: Vec::new(),
                produces: String::new()
            },
            request_params: Vec::new(),
            request_body: Vec::new()
        }
    }
    pub fn clone(&mut self) -> SpringMethod {
        SpringMethod {
            mapping: self.mapping.clone(),
            request_params: self.request_params.clone(),
            request_body: self.request_body.clone()
        }
    }
}

impl SpringMember {
    pub fn new() -> SpringMember {
        SpringMember {
            autowire: Option::None,
            value: Option::None
        }
    }
    pub fn clone(&mut self) -> SpringMember {
        SpringMember {
            autowire: self.autowire.clone(),
            value: self.value.clone()
        }
    }
}

impl RequestMap {
    pub fn clone(&mut self) -> RequestMap {
        RequestMap {
            url: self.url.clone(),
            method: self.method,
            headers: self.headers.clone(),
            produces: self.produces.clone(),
        }
    }
}
