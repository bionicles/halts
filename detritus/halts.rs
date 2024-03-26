/// An enablement for hashing and equality checking on syn::Path
#[derive(Clone)]
struct PathWrapper(syn::Path);

impl PartialEq for PathWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.segments.iter().eq(other.0.segments.iter())
    }
}

impl Eq for PathWrapper {}

impl std::hash::Hash for PathWrapper {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for segment in &self.0.segments {
            segment.ident.hash(state);
            match &segment.arguments {
                syn::PathArguments::None => (),
                syn::PathArguments::AngleBracketed(args) => {
                    let args: Vec<_> = args.args.iter().map(|arg| arg.to_token_stream()).collect();
                    args.hash(state);
                }
                syn::PathArguments::Parenthesized(args) => {
                    let args: Vec<_> = args
                        .inputs
                        .iter()
                        .map(|input| input.to_token_stream())
                        .collect();
                    args.hash(state);
                }
            }
        }
    }
}

impl PathWrapper {
    fn new(path: syn::Path) -> Self {
        PathWrapper(path)
    }

    fn segments(&self) -> &syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep> {
        &self.0.segments
    }

    fn identifiers(&self) -> Vec<String> {
        self.segments()
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect()
    }
}

#[test]
/// test the path_wrapper to enable hashing of syn::Path
fn test_path_wrapper() {
    let path = syn::parse_str::<syn::Path>("foo::bar::baz::qux").unwrap();
    let wrapper = PathWrapper::new(path);
    let identifiers = wrapper.identifiers();
    assert_eq!(
        identifiers,
        vec![
            "foo".to_string(),
            "bar".to_string(),
            "baz".to_string(),
            "qux".to_string()
        ]
    );

    let mut set = collections::HashSet::new();
    set.insert(wrapper.clone());
    assert!(set.contains(&wrapper));
}
