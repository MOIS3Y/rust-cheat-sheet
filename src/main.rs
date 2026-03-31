//! Rust Tutorial — Interactive Cheat Sheet.
//!
//! This project demonstrates Rust methods and patterns,
//! organized by topic into modular examples.

mod advanced;
mod basics;
mod collections;
mod error_handling;
mod ownership;
mod std_lib;
mod types;

fn main() {
    println!("=== RUST TUTORIAL ===\n");

    // Basics: Option
    println!("--- Option ---");
    basics::options::unwrap_some();
    basics::options::is_some_check();
    basics::options::map_transform();
    basics::options::and_then_chain();
    basics::options::unwrap_or_default();
    basics::options::ok_or_conversion();
    basics::options::filter_by_condition();
    basics::options::take_value();
    basics::options::replace_value();
    basics::options::zip_combine();
    basics::options::flatten_nested();
    println!("🦀 Option examples passed\n");

    // Basics: Result
    println!("--- Result ---");
    basics::results::unwrap_ok();
    basics::results::is_ok_check();
    basics::results::map_ok();
    basics::results::map_err();
    basics::results::and_then_chain();
    basics::results::or_fallback();
    basics::results::ok_to_option();
    basics::results::err_extract();
    basics::results::as_ref_access();
    basics::results::transpose_swap();
    println!("🦀 Result examples passed\n");

    // Basics: String
    println!("--- String ---");
    basics::strings::push_char();
    basics::strings::push_str();
    basics::strings::len_bytes();
    basics::strings::is_empty_check();
    basics::strings::contains_substring();
    basics::strings::starts_ends_with();
    basics::strings::split_delimiter();
    basics::strings::split_whitespace();
    basics::strings::trim_whitespace();
    basics::strings::chars_unicode();
    basics::strings::as_bytes();
    basics::strings::replace_substring();
    basics::strings::convert_case();
    basics::strings::find_substring();
    basics::strings::get_slice();
    println!("🦀 String examples passed\n");

    // Basics: Numbers
    println!("--- Numbers ---");
    basics::numbers::saturating_add();
    basics::numbers::checked_arithmetic();
    basics::numbers::wrapping_overflow();
    basics::numbers::overflowing_flag();
    basics::numbers::power_operation();
    basics::numbers::abs_signum();
    basics::numbers::min_max();
    basics::numbers::clamp_range();
    basics::numbers::is_power_of_two();
    basics::numbers::next_power_of_two();
    println!("🦀 Numbers examples passed\n");

    // Basics: Arrays & Slices
    println!("--- Arrays & Slices ---");
    basics::arrays_slices::slice_len();
    basics::arrays_slices::is_empty_check();
    basics::arrays_slices::get_element();
    basics::arrays_slices::first_last();
    basics::arrays_slices::split_at();
    basics::arrays_slices::binary_search();
    basics::arrays_slices::contains_element();
    basics::arrays_slices::repeat_elements();
    basics::arrays_slices::concat_slices();
    basics::arrays_slices::join_elements();
    println!("🦀 Arrays & Slices examples passed\n");

    // Basics: Documentation
    println!("--- Documentation ---");
    basics::documentation::doc_unwrap_with_panics();
    basics::documentation::doc_with_errors();
    basics::documentation::safe_divide(10, 2);
    println!("🦀 Documentation examples passed\n");

    // Basics: Functions
    println!("--- Functions ---");
    basics::functions::function_no_params();
    basics::functions::function_single_param();
    basics::functions::function_multiple_params();
    basics::functions::function_return_type();
    basics::functions::function_return_expression();
    basics::functions::function_early_return();
    basics::functions::function_return_tuple();
    basics::functions::function_mutable_param();
    basics::functions::function_reference_param();
    basics::functions::function_mutable_ref_param();
    basics::functions::function_const_param();
    basics::functions::function_recursive();
    basics::functions::closure_no_capture();
    basics::functions::closure_capture_ref();
    basics::functions::closure_capture_mut();
    basics::functions::closure_move();
    basics::functions::function_as_parameter();
    basics::functions::function_return_closure();
    basics::functions::function_diverging();
    println!("🦀 Functions examples passed\n");

    // Basics: Conditionals
    println!("--- Conditionals ---");
    basics::conditionals::if_simple();
    basics::conditionals::if_else_value();
    basics::conditionals::else_if_chain();
    basics::conditionals::if_multiple_conditions();
    basics::conditionals::if_let_option();
    basics::conditionals::if_let_with_else();
    basics::conditionals::match_basic();
    basics::conditionals::match_multiple_patterns();
    basics::conditionals::match_with_guards();
    basics::conditionals::match_destructure_tuple();
    basics::conditionals::matches_macro();
    basics::conditionals::matches_with_guard();
    println!("🦀 Conditionals examples passed\n");

    // Basics: Loops
    println!("--- Loops ---");
    basics::loops::loop_with_break();
    basics::loops::loop_return_value();
    basics::loops::while_loop();
    basics::loops::while_countdown();
    basics::loops::for_over_range();
    basics::loops::for_over_array();
    basics::loops::for_with_reference();
    basics::loops::for_with_enumerate();
    basics::loops::continue_keyword();
    basics::loops::nested_loops();
    basics::loops::loop_labels();
    basics::loops::for_over_chars();
    println!("🦀 Loops examples passed\n");

    // Error Handling: Question Mark
    println!("--- Error Handling ---");
    error_handling::question_mark::question_mark_env();
    error_handling::question_mark::question_mark_parse();
    error_handling::question_mark::question_mark_chain();
    error_handling::question_mark::question_mark_option();
    error_handling::question_mark::question_mark_in_main();
    error_handling::question_mark::question_mark_map_err();
    error_handling::question_mark::question_mark_vs_match();
    println!("🦀 Question Mark examples passed\n");

    // Error Handling: From Trait
    println!("--- From Trait ---");
    error_handling::from_trait::from_parse_int();
    error_handling::from_trait::from_multiple_sources();
    error_handling::from_trait::from_custom_conversion();
    error_handling::from_trait::from_infallible();
    error_handling::from_trait::from_vs_map_err();
    println!("🦀 From Trait examples passed\n");

    // Error Handling: Custom Error
    println!("--- Custom Error ---");
    error_handling::custom_error::custom_error_with_context();
    error_handling::custom_error::custom_error_validation();
    error_handling::custom_error::custom_error_service();
    error_handling::custom_error::custom_error_display();
    error_handling::custom_error::custom_error_trait();
    error_handling::custom_error::custom_error_matching();
    println!("🦀 Custom Error examples passed\n");

    // Error Handling: Box dyn Error
    println!("--- Box<dyn Error> ---");
    error_handling::box_dyn_error::box_dyn_error_basic();
    error_handling::box_dyn_error::box_dyn_error_multiple();
    error_handling::box_dyn_error::box_dyn_error_source();
    error_handling::box_dyn_error::box_dyn_error_send_sync();
    error_handling::box_dyn_error::box_dyn_error_tradeoffs();
    error_handling::box_dyn_error::box_dyn_error_conversion();
    println!("🦀 Box<dyn Error> examples passed\n");

    // Collections: Vec
    println!("--- Vec ---");
    collections::vecs::create_empty();
    collections::vecs::create_macro();
    collections::vecs::push_element();
    collections::vecs::pop_element();
    collections::vecs::index_access();
    collections::vecs::get_safe();
    collections::vecs::remove_at();
    collections::vecs::drain_range();
    collections::vecs::clear_all();
    collections::vecs::extend_iter();
    println!("🦀 Vec examples passed\n");

    // Collections: HashMap
    println!("--- HashMap ---");
    collections::hashmaps::insert_pair();
    collections::hashmaps::get_value();
    collections::hashmaps::contains_key();
    collections::hashmaps::remove_key();
    collections::hashmaps::entry_insert();
    collections::hashmaps::or_insert_with();
    collections::hashmaps::or_default();
    collections::hashmaps::and_modify();
    collections::hashmaps::iter_pairs();
    collections::hashmaps::keys_values();
    println!("🦀 HashMap examples passed\n");

    // Collections: Iterators
    println!("--- Iterators ---");
    collections::iterators::collect_vec();
    collections::iterators::map_transform();
    collections::iterators::filter_even();
    collections::iterators::filter_map_parse();
    collections::iterators::fold_sum();
    collections::iterators::reduce_product();
    collections::iterators::any_even();
    collections::iterators::all_positive();
    collections::iterators::find_first();
    collections::iterators::enumerate();
    collections::iterators::take_skip();
    collections::iterators::zip_iterators();
    collections::iterators::chain_iterators();
    collections::iterators::partition();
    println!("🦀 Iterators examples passed\n");

    // Advanced: Smart Pointers
    println!("--- Smart Pointers ---");
    advanced::smart_pointers::box_heap();
    advanced::smart_pointers::rc_clone();
    advanced::smart_pointers::rc_refcell();
    advanced::smart_pointers::arc_thread_safe();
    advanced::smart_pointers::weak_reference();
    println!("🦀 Smart Pointers examples passed\n");

    // Advanced: Traits
    println!("--- Traits ---");
    advanced::traits::clone_copy();
    advanced::traits::default_value();
    advanced::traits::from_into();
    advanced::traits::try_from_into();
    advanced::traits::as_ref_generic();
    println!("🦀 Traits examples passed\n");

    // Std Lib: Time
    println!("--- Time ---");
    std_lib::time::duration_new();
    std_lib::time::duration_from_millis();
    std_lib::time::instant_elapsed();
    std_lib::time::system_time_epoch();
    std_lib::time::checked_add_time();
    println!("🦀 Time examples passed\n");

    // Std Lib: Filesystem
    println!("--- Filesystem ---");
    std_lib::fs::write_file();
    std_lib::fs::read_to_string_file();
    std_lib::fs::read_to_bytes();
    std_lib::fs::file_metadata();
    std_lib::fs::path_exists();
    std_lib::fs::create_dirs();
    std_lib::fs::read_directory();
    std_lib::fs::copy_file();
    std_lib::fs::remove_file_op();
    std_lib::fs::remove_dir_all_op();
    println!("🦀 Filesystem examples passed\n");

    // Ownership: Move Semantics
    println!("--- Move Semantics ---");
    ownership::move_semantics::move_ownership();
    ownership::move_semantics::move_into_function();
    ownership::move_semantics::move_out_of_function();
    ownership::move_semantics::move_and_return();
    ownership::move_semantics::copy_types();
    ownership::move_semantics::clone_instead_of_move();
    ownership::move_semantics::move_struct();
    ownership::move_semantics::move_enum();
    ownership::move_semantics::partial_move();
    ownership::move_semantics::move_nested();
    println!("🦀 Move Semantics examples passed\n");

    // Ownership: Borrowing
    println!("--- Borrowing ---");
    ownership::borrowing::immutable_borrow();
    ownership::borrowing::mutable_borrow();
    ownership::borrowing::borrow_rules();
    ownership::borrowing::deref_coercion();
    ownership::borrowing::borrow_in_function();
    ownership::borrowing::mutable_borrow_in_function();
    ownership::borrowing::slice_borrow();
    ownership::borrowing::borrow_with_methods();
    println!("🦀 Borrowing examples passed\n");

    // Ownership: Lifetimes
    println!("--- Lifetimes ---");
    ownership::lifetimes::lifetime_elision();
    ownership::lifetimes::explicit_lifetime();
    ownership::lifetimes::lifetime_with_struct();
    ownership::lifetimes::lifetime_with_impl();
    ownership::lifetimes::multiple_lifetimes();
    ownership::lifetimes::static_lifetime();
    ownership::lifetimes::lifetime_bounds();
    println!("🦀 Lifetimes examples passed\n");

    // Types: Structs
    println!("--- Structs ---");
    types::structs::classic_struct();
    types::structs::tuple_struct();
    types::structs::unit_struct();
    types::structs::struct_update();
    types::structs::struct_destructure();
    types::structs::mutable_struct();
    types::structs::struct_methods();
    types::structs::associated_function();
    types::structs::struct_with_traits();
    types::structs::struct_with_display();
    types::structs::multiple_impl_blocks();
    types::structs::struct_init_shorthand();
    types::structs::newtype_pattern();
    println!("🦀 Structs examples passed\n");

    // Types: Enums
    println!("--- Enums ---");
    types::enums::simple_enum();
    types::enums::enum_with_data();
    types::enums::option_enum();
    types::enums::result_enum();
    types::enums::enum_methods();
    types::enums::enum_next();
    types::enums::nullable_pattern();
    types::enums::enum_with_traits();
    types::enums::enum_with_display();
    println!("🦀 Enums examples passed\n");

    // Types: Pattern Matching
    println!("--- Pattern Matching ---");
    types::pattern_matching::basic_match();
    types::pattern_matching::match_with_enum();
    types::pattern_matching::match_guards();
    types::pattern_matching::at_patterns();
    types::pattern_matching::if_let();
    types::pattern_matching::while_let();
    types::pattern_matching::destructure_tuple();
    types::pattern_matching::destructure_struct();
    types::pattern_matching::nested_destructure();
    types::pattern_matching::ignore_with_underscore();
    types::pattern_matching::match_ranges();
    println!("🦀 Pattern Matching examples passed\n");

    // Types: Custom Traits
    println!("--- Custom Traits ---");
    types::custom_traits::simple_trait();
    types::custom_traits::trait_with_default();
    types::custom_traits::trait_bounds();
    types::custom_traits::multiple_trait_bounds();
    types::custom_traits::where_clause();
    types::custom_traits::trait_objects();
    types::custom_traits::associated_types();
    types::custom_traits::trait_with_lifetime();
    types::custom_traits::trait_inheritance();
    println!("🦀 Custom Traits examples passed\n");

    println!("🦀 ALL EXAMPLES PASSED 🦀");
}
