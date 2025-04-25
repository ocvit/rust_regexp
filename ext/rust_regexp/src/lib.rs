use magnus::{
    class,
    define_class,
    encoding::RbEncoding,
    exception,
    function,
    method,
    prelude::*,
    scan_args::scan_args,
    Value,
    Error,
    RString,
    RArray,
};
use regex::bytes::{Regex, RegexSet, Match};

#[magnus::wrap(class = "RustRegexp", free_immediately, size)]
pub struct RustRegexp(Regex);

impl RustRegexp {
    pub fn new(args: &[Value]) -> Result<Self, Error> {
        let args = scan_args::<(String,), (), (), (), (), ()>(args)?;
        let pattern = args.required.0;

        let regex = Regex::new(&pattern).map_err(|e| Error::new(exception::arg_error(), e.to_string()))?;

        Ok(Self(regex))
    }

    pub fn find(&self, haystack: RString) -> RArray {
        let result = RArray::new();

        let regex = &self.0;
        let haystack = unsafe { haystack.as_slice() };

        // no capture groups defined except the default one
        if regex.captures_len() == 1 {
            // speed optimization, `.find` is faster than `.captures`
            if let Some(capture) = regex.find(haystack) {
                result
                    .push(Self::capture_to_ruby_string(&capture))
                    .expect("Non-frozen array");
            }
        } else {
            if let Some(captures) = regex.captures(haystack) {
                for capture in captures.iter().skip(1) {
                    if let Some(capture) = capture {
                        result
                            .push(Self::capture_to_ruby_string(&capture))
                            .expect("Non-frozen array");
                    } else {
                        result
                            .push(()) // push `nil`
                            .expect("Non-frozen array");
                    }
                }
            }
        }

        result
    }

    pub fn scan(&self, haystack: RString) -> RArray {
        let result = RArray::new();

        let regex = &self.0;
        let haystack = unsafe { haystack.as_slice() };

        // no capture groups defined except the default one
        if regex.captures_len() == 1 {
            // speed optimization, `.find_iter` is faster than `.captures_iter`
            for capture in regex.find_iter(haystack) {
                result
                    .push(Self::capture_to_ruby_string(&capture))
                    .expect("Non-frozen array");
            }
        } else {
            for captures in regex.captures_iter(haystack) {
                let group = RArray::with_capacity(regex.captures_len());

                for capture in captures.iter().skip(1) {
                    if let Some(capture) = capture {
                        group
                            .push(Self::capture_to_ruby_string(&capture))
                            .expect("Non-frozen array");
                    } else {
                        group
                            .push(()) // push `nil`
                            .expect("Non-frozen array");
                    }
                }

                result
                    .push(group)
                    .expect("Non-frozen array");
            }
        }

        result
    }

    pub fn is_match(&self, haystack: RString) -> bool {
        let regex = &self.0;
        let haystack = unsafe { haystack.as_slice() };

        regex.is_match(haystack)
    }

    pub fn pattern(&self) -> &str {
        let regex = &self.0;

        regex.as_str()
    }

    fn capture_to_ruby_string(capture: &Match) -> RString {
        RString::enc_new(
            capture.as_bytes(),
            RbEncoding::utf8()
        )
    }
}

#[magnus::wrap(class = "RustRegexp::Set", free_immediately, size)]
pub struct RustRegexpSet(RegexSet);

impl RustRegexpSet {
    pub fn new(args: &[Value]) -> Result<Self, Error> {
        let args = scan_args::<(Vec<String>,), (), (), (), (), ()>(args)?;
        let patterns = args.required.0;

        let set = RegexSet::new(patterns).map_err(|e| Error::new(exception::arg_error(), e.to_string()))?;

        Ok(Self(set))
    }

    pub fn matches(&self, haystack: RString) -> Vec<usize> {
        let set = &self.0;
        let haystack = unsafe { haystack.as_slice() };

        set.matches(haystack).into_iter().collect()
    }

    pub fn is_match(&self, haystack: RString) -> bool {
        let set = &self.0;
        let haystack = unsafe { haystack.as_slice() };

        set.is_match(haystack)
    }

    pub fn patterns(&self) -> Vec<String> {
        let set = &self.0;

        set.patterns().into()
    }
}

#[magnus::init]
pub fn init() -> Result<(), Error> {
    let regexp_class = define_class("RustRegexp", class::object())?;

    regexp_class.define_singleton_method("new", function!(RustRegexp::new, -1))?;
    regexp_class.define_method("match", method!(RustRegexp::find, 1))?;
    regexp_class.define_method("match?", method!(RustRegexp::is_match, 1))?;
    regexp_class.define_method("scan", method!(RustRegexp::scan, 1))?;
    regexp_class.define_method("pattern", method!(RustRegexp::pattern, 0))?;

    let regexp_set_class = regexp_class.define_class("Set", class::object())?;

    regexp_set_class.define_singleton_method("new", function!(RustRegexpSet::new, -1))?;
    regexp_set_class.define_method("match", method!(RustRegexpSet::matches, 1))?;
    regexp_set_class.define_method("match?", method!(RustRegexpSet::is_match, 1))?;
    regexp_set_class.define_method("patterns", method!(RustRegexpSet::patterns, 0))?;

    Ok(())
}
