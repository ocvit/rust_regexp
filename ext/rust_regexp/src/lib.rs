use magnus::{
    class,
    define_class,
    encoding::RbEncoding,
    exception,
    function,
    method,
    prelude::*,
    scan_args::scan_args,
    Error,
    RArray,
    RString,
    Value,
};
use regex::bytes::{Regex, RegexBuilder};

#[magnus::wrap(class = "", free_immediately, size)]
pub struct RustRegexp(Regex);

impl RustRegexp {
    pub fn new(args: &[Value]) -> Result<Self, Error> {
        let args = scan_args::<(String,), (Option<String>,), (), (), (), ()>(args)?;
        let pattern = args.required.0;
        let options = args.optional.0.unwrap_or_default();

        let mut re = RegexBuilder::new(&pattern);
        re.unicode(true)
            .case_insensitive(options.contains('i'))
            .multi_line(options.contains('m'))
            .dot_matches_new_line(options.contains('s'))
            .crlf(options.contains('R'))
            .swap_greed(options.contains('U'))
            .ignore_whitespace(options.contains('x'));

        let re = re
            .build()
            .map_err(|e| Error::new(exception::arg_error(), e.to_string()))?;

        Ok(Self(re))
    }

    pub fn is_match(&self, haystack: RString) -> bool {
        let haystack = unsafe { haystack.as_slice() };

        self.0.is_match(haystack)
    }

    pub fn find(&self, haystack: RString) -> RArray {
        let haystack = unsafe { haystack.as_slice() };

        let mut result = RArray::new();
        if self.0.captures_len() == 1 {
            if let Some(capture) = self.0.find(haystack) {
                result
                    .push(RString::enc_new(capture.as_bytes(), RbEncoding::utf8()))
                    .expect("Non-frozen array");
            }
        } else {
            if let Some(captures) = self.0.captures(haystack) {
                let mut group = RArray::with_capacity(self.0.captures_len());
                for capture in captures.iter() {
                    if let Some(capture) = capture {
                        group
                            .push(RString::enc_new(capture.as_bytes(), RbEncoding::utf8()))
                            .expect("Non-frozen array");
                    } else {
                        group.push(()).expect("Non-frozen array");
                    }
                }
                result.push(group).expect("Non-frozen array");
            }
        }

        result
    }

    pub fn scan(&self, haystack: RString) -> RArray {
        let haystack = unsafe { haystack.as_slice() };

        let mut result = RArray::new();
        if self.0.captures_len() == 1 {
            for capture in self.0.find_iter(haystack) {
                result
                    .push(RString::enc_new(capture.as_bytes(), RbEncoding::utf8()))
                    .expect("Non-frozen array");
            }
        } else {
            for captures in self.0.captures_iter(haystack) {
                let mut group = RArray::with_capacity(self.0.captures_len());
                for capture in captures.iter().skip(1) {
                    if let Some(capture) = capture {
                        group
                            .push(RString::enc_new(capture.as_bytes(), RbEncoding::utf8()))
                            .expect("Non-frozen array");
                    } else {
                        group.push(()).expect("Non-frozen array");
                    }
                }
                result.push(group).expect("Non-frozen array");
            }
        }

        result
    }

    pub fn from_ruby_regexp(regexp: Value) -> Result<Self, Error> {
        if !regexp.is_kind_of(class::regexp()) {
            return Err(magnus::Error::new(
                magnus::exception::type_error(),
                "expected Regexp",
            ));
        }

        let pattern = regexp.to_r_string()?;
        Self::new(&[pattern.as_value()])
    }
}

fn ruby_regexp_to_rust_regexp(rb_self: Value) -> Result<RustRegexp, Error> {
    if !rb_self.is_kind_of(class::regexp()) {
        return Err(magnus::Error::new(
            magnus::exception::type_error(),
            "expected Regexp",
        ));
    }

    let pattern = rb_self.to_r_string()?;
    RustRegexp::new(&[pattern.as_value()])
}

#[magnus::init]
pub fn init() -> Result<(), Error> {
    let rust_regexp_class = define_class("RustRegexp", class::object())?;

    rust_regexp_class.define_singleton_method("new", function!(RustRegexp::new, -1))?;
    rust_regexp_class.define_singleton_method("from_ruby_regexp", function!(RustRegexp::from_ruby_regexp, 1))?;

    rust_regexp_class.define_method("match?", method!(RustRegexp::is_match, 1))?;
    rust_regexp_class.define_method("scan", method!(RustRegexp::scan, 1))?;
    rust_regexp_class.define_method("find", method!(RustRegexp::find, 1))?;

    let ruby_regexp_class = define_class("Regexp", class::object())?;
    ruby_regexp_class.define_method("to_rust_regexp", method!(ruby_regexp_to_rust_regexp, 0))?;

    Ok(())
}
