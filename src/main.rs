use candid::Encode;
use candid_parser::utils::CandidSource;
use print_candid::Init;
use std::rc::Rc;

fn unwrap_type(type_: candid::types::Type) -> candid::types::TypeInner {
    let type_: Rc<_> = match type_ {
        candid::types::Type(ok) => ok,
    };

    Rc::into_inner(type_).unwrap()
}

// TL;DR: You need to call candid::IDLArgs::from_bytes_with_types.
//
// In addition to the blob you are trying to interpret, this requires two
// supporting arguments (three arguments in total). These additional pieces of
// information are gleaned from the .did file. However, extracting those pieces
// of information requires jumping through some hoops. Navigating that labrynth
// is the main point of this PoC.
//
// Overview of what is going on in here:
//
//     0. "Receive" the blob.
//
//     1. Read foo.did (ofc). Parse it.
//
//     2. Extract the initialization type. This requires lots of tedious
//        wrapping. In fact, most of the code is dedicated to this (rather
//        boring) step.
//
//     3. Finally, intepret the blob by constructing an IDLArgs from the blob.
//        To do this, we need supporting data, primarily from step 2, but also
//        step 1.
fn main() {
    // Step 0:
    // This represents the blob that someone "sent" us.
    let encoded = Encode!(&Init { i : Some(42) }).unwrap();

    // 1:
    // As best as I can tell, type_env is basically a map from name to type. For
    // example, in foo.did, there is `type Hello = record { ... };`. As a
    // result, load will return a type_env with an entry that conceptually looks
    // like
    //
    //     "Hello" => Record { fields: [...] }
    //
    // Whereas, type_ represents the service. For example, in foo.did, we have
    // `service : (Hello) { ... }`. Therefore, load returns a type_ that looks
    // (again, conceptually!) something like
    //
    //     Service { init: Hello, methods: [ ... ] }
    //
    // Does this behavior for load "make sense"? 🤷
    //
    // (The reason that type_ has an underscore is to avoid collision with the
    // type keyword.)
    let (type_env, type_)  = CandidSource::Text(include_str!("../foo.did"))
        .load()
        .unwrap();

    // 2: Dig out the supporting data from within foo.did.

    // For reasons, we need to do tons of unwrapping to get to datum of
    // interesting (i.e. the initialization argument type) within type_.
    let type_ = type_.unwrap();
    let (mut a, _ignored): (Vec<candid::types::Type>, _) = match unwrap_type(type_) {
        // Why a service is represented using Class? 🤷
        candid::types::TypeInner::Class(a, b) => (a, b),
        _ => panic!(),
    };
    assert_eq!(a.len(), 1, "{:#?}", a);
    let init = unwrap_type(a.pop().unwrap());

    // So, buried deep within the original type_, all we have is the name of the
    // initialization type.
    let init = match init {
        candid::types::TypeInner::Var(ok) => ok,
        _ => panic!(),
    };

    // To convert the name of the initialization type, we must look it up in
    // type_env.
    let init = type_env.find_type(&init).unwrap();

    println!("Type: {:#?}", init);
    println!();

    // 3: Interpret the blob.
    let decoded = candid::IDLArgs::from_bytes_with_types(
        &encoded,
        &type_env,
        &[init.clone()],
    )
    .unwrap();
    // Finally, output the thing that we originally wanted: a comprehensible
    // explanation of what's in the blob (i.e. `encoded`).
    println!("Decoded: {}", decoded);
}
