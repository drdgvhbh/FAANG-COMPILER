static NULL_TERMINATOR_SIZE: usize = 1;

use inkwell::{builder::Builder, context::Context, values::PointerValue, AddressSpace};

pub fn alloc_string(
    value: &str,
    variable_name: &str,
    context: &Context,
    builder: &Builder,
) -> PointerValue {
    cast_ptr_to_char_ptr(
        alloc_str_array(value, variable_name, &context, &builder),
        &context,
        &builder,
    )
}

fn alloc_str_array(
    string: &str,
    variable_name: &str,
    context: &Context,
    builder: &Builder,
) -> PointerValue {
    let array_size = string.len();

    let arr_ptr = builder.build_array_alloca(
        context
            .i8_type()
            .array_type((array_size + NULL_TERMINATOR_SIZE) as u32),
        context.i32_type().const_int(1, false),
        &variable_name,
    );
    builder.build_store(arr_ptr, context.const_string(&string, true));

    arr_ptr
}

fn cast_ptr_to_char_ptr(
    str_ptr: PointerValue,
    context: &Context,
    builder: &Builder,
) -> PointerValue {
    builder
        .build_bitcast(
            str_ptr,
            context.i8_type().ptr_type(AddressSpace::Generic),
            str_ptr.get_name().to_str().unwrap(),
        )
        .into_pointer_value()
}
