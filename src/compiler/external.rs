pub mod stdio {
    use inkwell::{
        context::Context, module::Linkage, module::Module, values::FunctionValue, AddressSpace,
    };

    pub static PRINTF: &str = "printf";
    pub static FPRINTF: &str = "fprintf";
    pub static FFLUSH: &str = "fflush";
    pub static FOPEN: &str = "fopen";

    pub enum Features {
        PRINTF,
        FPRINTF,
        FFLUSH,
        FOPEN,
    }

    pub fn add(features: &[Features], context: &Context, module: &Module) {
        for feature in features {
            match feature {
                Features::PRINTF => {
                    add_printf(context, module);
                }
                Features::FPRINTF => {
                    add_fprintf(context, module);
                }
                Features::FFLUSH => {
                    add_fflush(context, module);
                }
                Features::FOPEN => {
                    add_fopen(context, module);
                }
            }
        }
    }

    fn add_printf(context: &Context, module: &Module) -> FunctionValue {
        let const_char_ptr = context.i8_type().ptr_type(AddressSpace::Generic);
        module.add_function(
            PRINTF,
            context.i32_type().fn_type(&[const_char_ptr.into()], true),
            Some(Linkage::External),
        )
    }

    fn add_fprintf(context: &Context, module: &Module) -> FunctionValue {
        let file_descriptor_ptr = context.i8_type().ptr_type(AddressSpace::Generic);
        let const_char_ptr = context.i8_type().ptr_type(AddressSpace::Generic);
        module.add_function(
            FPRINTF,
            context
                .i32_type()
                .fn_type(&[file_descriptor_ptr.into(), const_char_ptr.into()], true),
            Some(Linkage::External),
        )
    }

    fn add_fopen(context: &Context, module: &Module) -> FunctionValue {
        let file_name_ptr = context.i8_type().ptr_type(AddressSpace::Generic);
        let mode_ptr = context.i8_type().ptr_type(AddressSpace::Generic);
        module.add_function(
            FOPEN,
            context
                .i8_type()
                .ptr_type(AddressSpace::Generic)
                .fn_type(&[file_name_ptr.into(), mode_ptr.into()], false),
            Some(Linkage::External),
        )
    }

    fn add_fflush(context: &Context, module: &Module) -> FunctionValue {
        let file_ptr = context.i8_type().ptr_type(AddressSpace::Generic);
        module.add_function(
            FFLUSH,
            context.i32_type().fn_type(&[file_ptr.into()], false),
            Some(Linkage::External),
        )
    }

    pub mod mock {
        use super::super::super::util;
        use super::*;
        use inkwell::builder::Builder;

        /// Adds a mock implementation of printf, which is just an adapter for fprintf.
        #[allow(dead_code)]
        pub fn add_printf(
            file_name: &str,
            context: &Context,
            module: &Module,
            builder: &Builder,
        ) -> FunctionValue {
            let printf = module.add_function(
                PRINTF,
                context.i32_type().fn_type(
                    &[context.i8_type().ptr_type(AddressSpace::Generic).into()],
                    true,
                ),
                None,
            );
            let block = context.append_basic_block(&printf, "entry");
            builder.position_at_end(&block);
            let file_name = util::alloc_string(file_name, "file_name", &context, &builder);
            let mode = util::alloc_string("a", "mode", &context, &builder);
            let file = builder
                .build_call(
                    module.get_function(FOPEN).unwrap(),
                    &[file_name.into(), mode.into()],
                    "file",
                )
                .try_as_basic_value()
                .left()
                .unwrap();

            let mut fprintf_args = vec![file.into()];
            fprintf_args.extend(printf.get_params());
            builder.build_call(
                module.get_function(FPRINTF).unwrap(),
                &fprintf_args,
                FPRINTF,
            );
            builder.build_call(module.get_function(FFLUSH).unwrap(), &[file.into()], FFLUSH);
            builder.build_return(Some(&context.i32_type().const_int(0, false)));
            printf
        }
    }
}
