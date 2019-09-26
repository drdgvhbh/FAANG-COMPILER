pub static PRINTLN: &str = "FAANG_println";

use super::util;
use inkwell::{builder::Builder, context::Context, module::Module, AddressSpace};

pub enum Features {
    PRINTLN,
}

pub fn add(features: &[Features], context: &Context, module: &Module, builder: &Builder) {
    for feature in features {
        match feature {
            Features::PRINTLN => {
                add_println(context, module, builder);
            }
        }
    }
}

fn add_println(context: &Context, module: &Module, builder: &Builder) {
    let println_fcn = module.add_function(
        PRINTLN,
        context.void_type().fn_type(
            &[context.i8_type().ptr_type(AddressSpace::Generic).into()],
            false,
        ),
        None,
    );
    let block = context.append_basic_block(&println_fcn, "entry");
    builder.position_at_end(&block);

    let text = println_fcn.get_nth_param(0).unwrap();
    builder.build_call(
        module.get_function(super::external::stdio::PRINTF).unwrap(),
        &[text
            .as_pointer_value()
            .const_cast(context.i8_type().ptr_type(AddressSpace::Generic))
            .into()],
        super::external::stdio::PRINTF,
    );

    let new_line = util::alloc_string("\n", "format", &context, &builder);
    builder.build_call(
        module.get_function(super::external::stdio::PRINTF).unwrap(),
        &[new_line
            .const_cast(context.i8_type().ptr_type(AddressSpace::Generic))
            .into()],
        super::external::stdio::PRINTF,
    );
    builder.build_return(None);
}
