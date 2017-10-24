use proc_macro::TokenStream;
use syn;

pub(crate) fn vtable(input: TokenStream) -> TokenStream {
  let input = syn::parse_token_trees(&input.to_string()).unwrap();
  let mut input = input.into_iter();
  let mut attributes = Vec::new();
  let mut thread_id = Vec::new();
  let mut thread_name = Vec::new();
  let mut thread_number = Vec::new();
  let mut thread_attributes = Vec::new();
  let mut thread_count = 0usize;
  'outer: loop {
    let mut inner_attributes = Vec::new();
    loop {
      match input.next() {
        Some(syn::TokenTree::Token(syn::Token::DocComment(string))) => {
          if string.starts_with("//!") {
            let string = string.trim_left_matches("//!");
            attributes.push(quote!(#[doc = #string]));
          } else {
            let string = string.trim_left_matches("///");
            inner_attributes.push(quote!(#[doc = #string]));
          }
        }
        Some(syn::TokenTree::Token(syn::Token::Pound)) => match input.next() {
          Some(syn::TokenTree::Token(syn::Token::Not)) => match input.next() {
            Some(syn::TokenTree::Delimited(delimited)) => {
              attributes.push(quote!(# #delimited))
            }
            token => panic!("Invalid tokens after `#!`: {:?}", token),
          },
          Some(syn::TokenTree::Delimited(delimited)) => {
            inner_attributes.push(quote!(# #delimited))
          }
          token => panic!("Invalid tokens after `#`: {:?}", token),
        },
        Some(syn::TokenTree::Token(syn::Token::Ident(name))) => {
          match input.next() {
            Some(syn::TokenTree::Token(syn::Token::Semi)) => (),
            token => panic!("Invalid token after `{}`: {:?}", name, token),
          }
          thread_id.push(thread_count);
          thread_name.push(name);
          thread_number.push(None);
          thread_attributes.push(inner_attributes);
          thread_count += 1;
          break;
        }
        Some(
          syn::TokenTree::Token(
            syn::Token::Literal(syn::Lit::Int(number, syn::IntTy::Unsuffixed)),
          ),
        ) => {
          match input.next() {
            Some(syn::TokenTree::Token(syn::Token::Colon)) => (),
            token => panic!("Invalid token after `{}`: {:?}", number, token),
          }
          let name = match input.next() {
            Some(syn::TokenTree::Token(syn::Token::Ident(name))) => name,
            token => panic!("Invalid token after `{}:`: {:?}", number, token),
          };
          match input.next() {
            Some(syn::TokenTree::Token(syn::Token::Semi)) => (),
            token => panic!("Invalid token after `{}`: {:?}", name, token),
          }
          thread_id.push(thread_count);
          thread_name.push(name);
          thread_number.push(Some(number));
          thread_attributes.push(inner_attributes);
          thread_count += 1;
          break;
        }
        None => break 'outer,
        token => panic!("Invalid token: {:?}", token),
      }
    }
  }
  let irq_number = thread_number
    .iter()
    .cloned()
    .filter_map(|x| x)
    .max()
    .map(|x| x + 1)
    .unwrap_or(0);
  let mut irq_name = (0..irq_number)
    .map(|n| syn::Ident::new(format!("_irq{}", n)))
    .collect::<Vec<_>>();
  thread_number
    .iter()
    .zip(thread_name.iter())
    .filter_map(|(number, name)| {
      number.map(|number| (number as usize, name))
    })
    .for_each(|(number, name)| {
      irq_name[number] = name.clone();
    });
  let thread_handler = thread_name
    .iter()
    .map(|name| syn::Ident::new(format!("__{}_handler", name)))
    .collect::<Vec<_>>();
  let thread_id2 = thread_id.clone();
  let thread_id3 = thread_id.clone();
  let thread_name2 = thread_name.clone();
  let thread_handler2 = thread_handler.clone();
  let irq_name2 = irq_name.clone();

  let output = quote! {
    use drone_cortex_m::vtable::{Handler, ResetHandler, Reserved};

    #(#attributes)*
    #[allow(dead_code)]
    pub struct VectorTable {
      reset: ResetHandler,
      nmi: Option<Handler>,
      hard_fault: Option<Handler>,
      mem_manage: Option<Handler>,
      bus_fault: Option<Handler>,
      usage_fault: Option<Handler>,
      _reserved0: [Reserved; 4],
      sv_call: Option<Handler>,
      debug: Option<Handler>,
      _reserved1: [Reserved; 1],
      pend_sv: Option<Handler>,
      sys_tick: Option<Handler>,
      #(
        #irq_name: Option<Handler>,
      )*
    }

    impl VectorTable {
      /// Creates an empty `VectorTable` with `reset` handler.
      pub const fn new(reset: ResetHandler) -> VectorTable {
        VectorTable {
          #(
            #thread_name: Some(#thread_handler),
          )*
          ..VectorTable {
            reset,
            nmi: None,
            hard_fault: None,
            mem_manage: None,
            bus_fault: None,
            usage_fault: None,
            _reserved0: [Reserved::Vector; 4],
            sv_call: None,
            debug: None,
            _reserved1: [Reserved::Vector; 1],
            pend_sv: None,
            sys_tick: None,
            #(
              #irq_name2: None,
            )*
          }
        }
      }
    }

    #[allow(dead_code)]
    static mut THREADS: [ThreadLocal; #thread_count] = [
      #(
        ThreadLocal::new(#thread_id),
      )*
    ];

    #(
      #[doc(hidden)]
      pub unsafe extern "C" fn #thread_handler2() {
        const THREAD_ID: usize = #thread_id2;
        THREADS.get_unchecked_mut(THREAD_ID).run(THREAD_ID);
      }

      #(#thread_attributes)*
      #[inline]
      pub fn #thread_name2() -> &'static ThreadLocal {
        unsafe { ThreadLocal::get_unchecked(#thread_id3) }
      }
    )*
  };
  output.parse().unwrap()
}